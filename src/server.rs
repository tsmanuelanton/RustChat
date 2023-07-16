use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::UPDATE_TIME;

struct Client {
    id: String,
    stream: TcpStream,
}

struct Message {
    client_id: String,
    text: String,
}

type ClientsList = Vec<Client>;

pub struct ServerState {
    addr: SocketAddr,
    clients: ClientsList,
}

pub type ServerStateSafe = Arc<Mutex<ServerState>>;

pub fn start_server(addr: SocketAddr) {
    let state = ServerState {
        addr,
        clients: Default::default(),
    };

    let safe_state = Arc::new(Mutex::new(state));

    match TcpListener::bind(addr) {
        Ok(listener) => {
            println!("Server listening at {}", safe_state.lock().unwrap().addr);

            // waits to new connections
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {


                        // new thread to handle the connection
                        let cloned_state = Arc::clone(&safe_state);
                        thread::spawn(move || { handle_new_connection(stream, &cloned_state) });
                    }
                    Err(error) => println!("{error}")
                }
            }
        }
        Err(error) => {
            panic!("Server couldnt start at {}: {}", safe_state.lock().unwrap().addr, error);
        }
    };
}

/*
Adds a new client to the clients list. Reads the stream and send a broadcast to other clients.
 */
fn handle_new_connection(mut stream: TcpStream, server_state: &ServerStateSafe) {
    let mut state = server_state.lock().unwrap();

    let peer_addr = stream.peer_addr().expect("Getting peer address");
    let client_id = state.clients.len().to_string();

    println!("New client {client_id} from ip {peer_addr}");

    // Adds a new client to the clients list
    let client = Client {
        id: client_id.clone(),
        stream: stream.try_clone().unwrap(),
    };
    state.clients.push(client);

    // unlock de mutex
    drop(state);

    // let msg = format!("Hello client {}\n", client_id);

    stream.set_read_timeout(UPDATE_TIME).unwrap();

    loop {
        // reads the stream
        if let Some(input) = read_stream(&mut stream) {
            let msg = Message {
                client_id: client_id.clone(),
                text: input,
            };

            // send msg back to other clients
            broadcast_msg(msg, server_state);
        }
    }
}

/*
 Given a stream, reads it and returns the input.
 */
fn read_stream(stream: &mut TcpStream) -> Option<String> {
    let mut string_buffer = String::new();
    let _ = stream.read_to_string(&mut string_buffer);

    if !string_buffer.is_empty() {
        return Some(string_buffer)
    }
    None
}

/*
Given a message, sends the content to all the clients except to the author.
 */
fn broadcast_msg(msg: Message, server_state: &ServerStateSafe) {
    let state = server_state.lock().unwrap();
    for client in &state.clients {
        if msg.client_id != client.id {
            let mut stream = &client.stream;
            stream.write(msg.text.trim().as_bytes()).unwrap();
        }
    }
    drop(state);
}
