use std::{env, thread};
use std::time::Duration;

mod server;
mod client;

const UPDATE_TIME: Option<Duration> = Some(Duration::from_millis(500));

fn main() {
    let args: Vec<String> = env::args().collect();
    let host_type = args.get(1);
    let addr = std::net::SocketAddr::new("127.0.0.1".parse().unwrap(), 3000);

    if host_type != None {
        if matches!(host_type.unwrap().trim(), "server") {
            println!("Creating a new server...");
            server::start_server(addr);
        } else {
            println!("Attempting to a connect to server at {} ...", addr);
            client::connect_to(addr);
        }

    }else {
        println!("Missing host type argument. Add server o client argument.");
    }
}
