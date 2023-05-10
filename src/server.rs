use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::UPDATE_TIME;

static mut CLIENTS: Vec<TcpStream> = Vec::new();

pub fn start_server(addr: SocketAddr) {
    match TcpListener::bind(addr) {
        Ok(listener) => unsafe {
            println!("Server listening at {addr}");
            std::thread::spawn(move || wait_input());

            for stream in listener.incoming() {
                match stream {
                    Ok(stream)=> {
                        let peer_addr = stream.peer_addr().expect("Getting peer address");
                        println!("New client {peer_addr}");
                        CLIENTS.push(stream.try_clone().unwrap());
                        let len = CLIENTS.len();
                        std::thread::spawn(move || handle_client(&stream, len));
                    }
                    Err(e) => panic!("{e}")
                }
            }
        }
        Err(error) => {
            panic!("Server couldnt start at {addr}: {error}");
        }
    };
}

unsafe fn handle_client(mut stream: &TcpStream, client_id: usize) {
    let msg = format!("Hello client {client_id}\n");
    stream.write(msg.trim().as_bytes()).expect(
        format!("Failed trying to send {msg} to client {client_id}").as_str());

    // let stream1 = stream.try_clone().expect("");
    let stream2 = stream.try_clone().expect("");

    // let join_handle1 = std::thread::spawn(move || write_stream(stream1, client_id));
    let join_handle2 = std::thread::spawn(move || read_stream(&stream2, &client_id));

    // let _ = join_handle1.join();
    let _ = join_handle2.join();
}

fn read_stream(mut stream: &TcpStream, client_id: &usize) {
    let _ = stream.set_read_timeout(UPDATE_TIME);

    if let Ok(addr) = stream.peer_addr() {
        let mut in_msg = String::new();

        loop {
            let _ = stream.read_to_string(&mut in_msg);
            if !in_msg.is_empty() {
                println!("client-{client_id}@{addr}: {in_msg}.");
                send_msg_clients(&in_msg);
                in_msg.clear();
            }
        }
    }
}

fn wait_input() {
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str)
            .expect("TODO: panic message");

        send_msg_clients(&str)
    }
}

fn send_msg_clients(str : &String){
    unsafe {
        for mut client in &CLIENTS {
            client.write(str.trim().as_bytes()).expect("d");
        }
    }
}

