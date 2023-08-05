use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, header, Request, Response, StatusCode, upgrade};
use tokio_tungstenite::WebSocketStream;
use tungstenite::handshake;

use crate::chat_server;
use crate::chat_server::ServerStateSafe;

pub async fn handle_request(request: Request<Body>, addr: SocketAddr, state: ServerStateSafe) -> Result<Response<Body>, Infallible> {
    match (request.uri().path(), request.headers().contains_key(header::UPGRADE)) {
        //if the request is ws_echo and the request headers contains an Upgrade key
        ("/connect-chat", true) => {
            connect_chat(request, addr, state)
        }
        ("/connect-chat", false) => {
            //handle the case where the url is /ws_echo, but does not have an Upgrade field
            Ok(Response::new(Body::from("Getting even warmer, \
                                                try connecting to this url \
                                                using a websocket client.\n")))
        }
        (url, false) => {
            // handle any other url without an Upgrade header field

            match url {
                "/chat" => {
                    let state = state.read().await;
                    let res = serde_json::to_string(&state.messages).unwrap();
                    Ok(Response::new(Body::from(res)))
                }
                _ => Ok(Response::new(Body::from(format!("This {url} url doesn't do \
                                                much, try accessing the \
                                                /connect-chat url instead.\n"))))
            }
        }
        (_, true) => {
            //handle any other url with an Upgrade header field
            Ok(Response::new(Body::from("Getting warmer, but I'm \
                                                only letting you connect \
                                                via websocket over on \
                                                /connect-chat, try that url.\n")))
        }
    }
}

fn connect_chat(mut request: Request<Body>, addr: SocketAddr, state: ServerStateSafe) -> Result<Response<Body>, Infallible> {
    //assume request is a handshake, so create the handshake response
    let response =
        match handshake::server::create_response_with_body(&request, Body::empty) {
            Ok(response) => {

                //in case the handshake response creation succeeds,
                //spawn a task to handle the websocket connection
                tokio::spawn(async move {
                    //using the hyper feature of upgrading a connection
                    match upgrade::on(&mut request).await {
                        //if successfully upgraded
                        Ok(upgraded) => {
                            //create a websocket stream from the upgraded object
                            let ws_stream = WebSocketStream::from_raw_socket(
                                //pass the upgraded object
                                //as the base layer stream of the Websocket
                                upgraded,
                                tungstenite::protocol::Role::Server,
                                None,
                            ).await;

                            chat_server::handle_new_connection(ws_stream, addr, &state).await;
                        }
                        Err(e) =>
                            println!("error when trying to upgrade connection \
                                         to websocket connection. \
                                        Error is: {e}"),
                    }
                });
                //return the response to the handshake request
                response
            }
            Err(error) => {
                //probably the handshake request is not up to spec for websocket
                println!("Failed to create websocket response \
                                to request. \
                                Error is: {error}");
                let mut res = Response::new(Body::from(format!("Failed to create websocket: {}", error)));
                *res.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(res);
            }
        };

    Ok::<_, Infallible>(response)
}