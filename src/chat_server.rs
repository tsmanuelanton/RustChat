use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;

use futures::stream::SplitStream;
use futures::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use tokio::sync::{mpsc, mpsc::UnboundedReceiver, mpsc::UnboundedSender};
use tokio_tungstenite::WebSocketStream;
use tungstenite::{Error, Message};
use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    id: String,
    nickname: String,
    addr: SocketAddr,
    sender: UnboundedSender<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Msg {
    pub(crate) client_id: String,
    pub(crate) nickname: String,
    pub(crate) text: String,
    pub(crate) created_at: SystemTime,
}

#[derive(Default)]
pub struct ServerState {
    pub(crate) clients: Vec<Client>,
    pub(crate) messages: Vec<Msg>,
}

pub type ServerStateSafe = Arc<RwLock<ServerState>>;

/*
Adds a new client to the clients list. Reads the stream and send a broadcast to other clients.
 */
pub async fn handle_new_connection(
    ws: WebSocketStream<Upgraded>,
    addr: SocketAddr,
    server_state: &ServerStateSafe,
) {
    println!("New connection from ip {addr}");

    // create async channel for communicating between tasks
    let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        mpsc::unbounded_channel();

    // split socket for simultaneous reading and writing
    let (mut sink, mut read) = ws.split();
    let client = handshake(&mut sink, &mut read, server_state, addr, tx).await;
    let client_id_cloned = client.id.clone();
    let state = server_state.clone();
    // task for sending the received msg from channel to the ws
    tokio::spawn(async move {
        let state = state.clone();
        while let Some(msg) = rx.recv().await {
            match sink.send(msg).await {
                Ok(_) => (),
                Err(e) => match e.into() {
                    Error::ConnectionClosed => {
                        println!("Client {client_id_cloned} disconnected");
                        let mut server_state = state.write().await;
                        if let Some(index) = server_state
                            .clients
                            .iter()
                            .position(|value| value.id == client_id_cloned)
                        {
                            server_state.clients.swap_remove(index);
                        }
                        break;
                    }
                    _ => {}
                },
            }
        }
        // close the socket
        if let Err(e) = sink.close().await {
            match e.into() {
                Error::ConnectionClosed => {}
                e => {
                    panic!("Error closing the socket: {}", e)
                }
            }
        }
    });

    // received msg from client through ws broadcast to the other clients
    while let Some(Ok(result)) = read.next().await {
        let msg = Msg {
            client_id: client.id.clone(),
            nickname: client.nickname.clone(),
            text: result.to_string(),
            created_at: SystemTime::now(),
        };
        let mut state = server_state.write().await;
        state.messages.push(msg.clone());
        drop(state);
        broadcast_msg(msg, server_state).await;
    }
}

/*
When a new connection, asks the nickname to the client and adds it to the clients list.
 */
async fn handshake(
    sink: &mut futures::stream::SplitSink<WebSocketStream<Upgraded>, Message>,
    read: &mut SplitStream<WebSocketStream<Upgraded>>,
    server_state: &Arc<RwLock<ServerState>>,
    addr: SocketAddr,
    tx: UnboundedSender<Message>,
) -> Client {
    let ask_nickname = Message::Text(
        serde_json::to_string(&json!({
            "handshake": {
                "nickname": "Tell me your nickname",
            }
        }))
        .unwrap(),
    );
    sink.send(ask_nickname).await.unwrap();
    let result = read.next().await.unwrap().unwrap();
    let nickname = result.to_string();

    let client_id = Uuid::new_v4().to_string();
    // Adds a new client to the clients list
    let client = Client {
        id: client_id.clone(),
        addr,
        nickname: nickname.clone(),
        sender: tx,
    };

    let mut state = server_state.write().await;
    state.clients.push(client.clone());
    drop(state);

    let welcome_client: Message = Message::Text(
        serde_json::to_string(&json!( {
                "handshake": {
                    "client_id": client_id,
                    "welcome": format!("Welcome to the chat {}!", nickname.trim()),
                }
            }
        ))
        .unwrap(),
    );

    sink.send(welcome_client).await.unwrap();
    client
}

/*
Given a message, sends the content to all the clients except to the author.
 */
async fn broadcast_msg(msg: Msg, server_state: &ServerStateSafe) {
    let state = server_state.read().await;

    for client in &state.clients {
        let msg_json = Message::Text(
            serde_json::to_string(&json!({
                "message": {
                    "client_id": msg.client_id,
                    "nickname": msg.nickname,
                    "text": msg.text,
                    "created_at": msg.created_at,
                }
            }))
            .unwrap(),
        );
        client.sender.send(msg_json).unwrap();
    }
}
