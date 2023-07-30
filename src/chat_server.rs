use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;

use futures::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Clone)]
pub struct Client {
    id: String,
    addr: SocketAddr,
    sender: UnboundedSender<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Msg {
    pub(crate) client_id: String,
    pub(crate) text: String,
    pub(crate) created_at: SystemTime,
}

#[derive(Default)]
pub struct ServerState {
    pub(crate) clients: Vec<Client>,
    pub(crate) messages: Vec<Msg>,
}

pub type ServerStateSafe = Arc<Mutex<ServerState>>;

/*
Adds a new client to the clients list. Reads the stream and send a broadcast to other clients.
 */
pub async fn handle_new_connection(
    ws: WebSocketStream<Upgraded>,
    addr: SocketAddr,
    server_state: &ServerStateSafe,
) {
    let mut state = server_state.lock().await;
    let client_id = state.clients.len().to_string();
    println!("New client {client_id} from ip {addr}");

    // create async channel for communicating between tasks
    let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        mpsc::unbounded_channel();

    // Adds a new client to the clients list
    let client = Client {
        id: client_id.clone(),
        addr,
        sender: tx,
    };

    state.clients.push(client);
    drop(state);

    // split socket for simultaneous reading and writing
    let (mut sink, mut read) = ws.split();

    let client_id_cloned = client_id.clone();
    let state = server_state.clone();
    // task for sending the received msg from channel to the ws
    tokio::spawn(async move {
        let state = state.clone();
        while let Some(msg) = rx.recv().await {
            match sink.send(msg).await {
                Ok(_) => (),
                Err(e) => match e.into() {
                    tokio_tungstenite::tungstenite::Error::ConnectionClosed => {
                        println!("Client {client_id_cloned} disconnected");
                        let mut server_state = state.lock().await;
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
        sink.close().await.unwrap();
    });

    // received msg from client through ws broadcast to the other clients
    while let Some(Ok(result)) = read.next().await {
        let msg = Msg {
            client_id: client_id.clone(),
            text: result.to_string(),
            created_at: SystemTime::now(),
        };
        let mut state = server_state.lock().await;
        state.messages.push(msg.clone());
        drop(state);
        broadcast_msg(msg, server_state).await;
    }
}

/*
Given a message, sends the content to all the clients except to the author.
 */
async fn broadcast_msg(msg: Msg, server_state: &ServerStateSafe) {
    let state = server_state.lock().await;

    for client in &state.clients {
        let msg_json = Message::Text(serde_json::to_string(&msg).unwrap());
        client.sender.send(msg_json).unwrap();
    }
}
