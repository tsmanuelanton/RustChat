use std::{env, thread};
mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host_type = args.get(1);
    if host_type != None {

        if matches!(host_type.unwrap().trim(), "server"){
            println!("Creating a new server");
            server::start_server();
        }else {
            println!("Attempting to a connect to 127.0.0.1:3000");
            client::connect_to("127.0.0.1:3000");
        }

    }else {
        println!("Missing host type argument. Add server o client argument.");
    }
}
