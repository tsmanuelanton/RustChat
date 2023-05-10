use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use crate::UPDATE_TIME;

pub fn connect_to(addr: SocketAddr) {
    let stream = TcpStream::connect(&addr);

    match stream {
        Ok(stream) => {
            println!("Successful connection");

            // stream.set_read_timeout(Option::from(Duration::from_millis(500)))
            //     .expect("TODO: panic message");

            let stream1 = stream.try_clone().expect("");
            let stream2 = stream.try_clone().expect("");
            stream2.set_read_timeout(UPDATE_TIME);

            let join_handle1 = std::thread::spawn(move || write_stream(&stream1));
            let join_handle2 = std::thread::spawn(move || read_stream(&stream2));

            join_handle1.join();
            join_handle2.join();

            println!("Closing connection...");
        }
        Err(error) => {
            println!("Something went wrong while attempting to connect to {addr}: {error}");
        }
    }
}

fn read_stream(mut stream: &TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        let mut in_msg = String::new();

        loop {
            let _ = stream.read_to_string(&mut in_msg);
            if !in_msg.is_empty() {
                println!("Server@{addr}: {in_msg}.");
                in_msg.clear();
            }
        }
    }
}

fn write_stream(mut stream: &TcpStream) {
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).expect("TODO: panic message");
        stream.write(str.trim().as_bytes()).expect(format!("Failed trying to send {str} to server").as_str());
    }
}