
use std::net::{TcpListener, TcpStream};


pub fn start_server(){
    let server_addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(server_addr);
    if listener.is_ok() {
        println!("Server listening at {}", server_addr);
        for stream in listener.unwrap().incoming() {
            if stream.is_ok() {
                handle_client(stream.unwrap());
            }
        }
    } else {
        println!("Server couldnt start at {} because {}", server_addr, listener.err().unwrap());
        return;
    }

}

fn handle_client(stream: TcpStream) {
    println!("Hello client {}", stream.peer_addr().unwrap());
}