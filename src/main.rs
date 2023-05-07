use std::thread;

mod server;
mod client;

fn main() {

    thread::spawn(move || server::start_server());
    client::connect_to("127.0.0.1:3000");
}
