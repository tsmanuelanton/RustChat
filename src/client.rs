use std::io::{Read, stdin, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::UPDATE_TIME;

pub fn connect_to(addr: SocketAddr) {
    let stream = TcpStream::connect(&addr);

    match stream {
        Ok(stream) => {
            println!("Successful connection");

            let buffer = String::new();
            let user_input = Arc::new(Mutex::new(buffer));

            stream.set_read_timeout(UPDATE_TIME).unwrap();

            let input_temp = Arc::clone(&user_input);
            // thread reading content from server
            std::thread::spawn(move || handle_connection(stream, input_temp));

            loop {
                let mut buffer = String::new();

                stdin().read_line(&mut buffer).unwrap();
                if !buffer.is_empty() {
                    let mut input_buffer = user_input.lock().unwrap();
                    *input_buffer = buffer;
                    drop(input_buffer);
                }
            }

            // println!("Closing connection...");
        }
        Err(error) => {
            println!("Something went wrong while attempting to connect to {addr}: {error}");
        }
    }
}

fn handle_connection(mut stream: TcpStream, user_input: Arc<Mutex<String>>) {
    loop {
        // reads the stream
        if let Some(input) = read_stream(&mut stream) {
            println!("{}", input);
        }
        let mut user_input = user_input.lock().unwrap();

        if !user_input.is_empty() {
            // send input to server
            stream.write(user_input.as_ref()).unwrap();

            user_input.clear();
        }

        // unlock mutex
        drop(user_input);

        std::thread::sleep(Duration::from_millis(500))
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
