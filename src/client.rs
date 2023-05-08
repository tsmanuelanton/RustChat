use std::io::Read;
use std::net::TcpStream;

pub fn connect_to(addr: &str){
    let stream = TcpStream::connect(addr);

    match stream {
        Ok(mut stream) => {
            println!("Successful connection");
            let mut in_msg = [0;100] ;
            while let Ok(len) = stream.read(&mut in_msg) {
                println!("{}", std::str::from_utf8(&in_msg[..len]).unwrap());
            }

            println!("Closing connection...");
        }
        Err(error) => {
            println!("Something went wrong while attempting to connect to {}: {}", addr, error);
        }
    }
}