use std::io::Write;
use std::net::{TcpListener, TcpStream};


pub fn start_server(){
    let mut clients: Vec<TcpStream> = Vec::new();

    let server_addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(server_addr);
    if listener.is_ok() {
        println!("Server listening at {}", server_addr);
        for stream in listener.unwrap().incoming() {
            if stream.is_ok() {
                let stream = stream.unwrap();
                println!("New client {}", stream.peer_addr().unwrap().to_string());
                clients.push(stream.try_clone().unwrap());
                let len = clients.len();
                std::thread::spawn(move||  handle_client(stream,len ) );
            }
        }
    } else {
        println!("Server couldnt start at {} because {}", server_addr, listener.err().unwrap());
    }

}

fn handle_client(mut stream: TcpStream, client_id: usize) {
    let msg =  "Hello client ".to_owned() + &client_id.to_string();

    match stream.write( msg.as_bytes()) {
        Ok(_) => { }
        Err(error) => { println!("{}", error) }
    }

}