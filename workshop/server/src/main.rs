extern crate serde;
extern crate serde_json;

mod client;
mod server;

use server::Server;
use std::net::TcpStream;

const DEFAULT_PORT: u16 = 5000;

type TxChannel = std::sync::mpsc::Sender<ChannelMessage>;

///Internal inter-thread message passing
pub enum ChannelMessage {
    NewClient((String, TcpStream)),
    BroadcastMessage(String),
    PrivateMessage { recipient: String, content: String },
    Disconnect(String),
}

fn get_port() -> u16 {
    if let Some(port_arg) = std::env::args().nth(1) {
        port_arg.parse().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    }
}

fn main() {
    match Server::open("0.0.0.0", get_port()) {
        Ok(server) => server.start(),
        Err(msg) => panic!(msg),
    }
}
