use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Server, server::conn::AddrStream};
use hyper::service::{make_service_fn, service_fn};

use crate::chat_server::ServerStateSafe;

mod chat_server;
mod routes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //hyper server boilerplate code from https://hyper.rs/guides/server/hello-world/

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // new thread to handle the connection
    println!("Listening on {addr} for http or websocket connections.");

    let state = ServerStateSafe::default();

    // A `Service` is needed for every connection, so this
    // creates one from our `handle_request` function.
    let make_svc = make_service_fn(move |socket: &AddrStream| {
        let addr = socket.remote_addr();
        let state = state.clone();
        async move {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(move |req| routes::handle_request(req, addr, Arc::clone(&state))
            ))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
