use std::net::TcpStream;

pub fn connect_to(addr: &str){
    let stream = TcpStream::connect(addr);
    if stream.is_ok() {
        println!("Successful connection to {}", addr);
    }else {
        println!("Something went wrong while attempting to connect to {}: {}", addr, stream.err().unwrap());

    }

}