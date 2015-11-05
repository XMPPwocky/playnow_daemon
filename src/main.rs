extern crate serde;
extern crate serde_json;
extern crate steamwebapi;
extern crate hyper;
extern crate redis;
extern crate websocket;
extern crate steamid;
#[macro_use]
extern crate quick_error;

use std::thread;

mod backend;
mod wshandler;

type QueueStatus = ();

fn get_apikey() -> String {
    match std::env::var("STEAM_APIKEY") {
        Ok(key) => key,
        Err(_) => panic!("No Steam API key found. Set the STEAM_APIKEY environment variable.")
    }
}

fn main() {
    let ws_server = websocket::Server::bind("127.0.0.1:2794").unwrap();
    //let mut backend = backend::Backend::new();

    for connection in ws_server {
        if let Ok(connection) = connection {
            thread::spawn(move || {
                wshandler::handler(connection) 
            });
        } else if let Err(e) = connection {
            println!("Error: {:?}", e);
            break;
        }
    }
}
