use crate::ChannelMessage;
use crate::ChannelMessage::Disconnect;
use crate::TxChannel;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct Client {
    pub name: String,
    stream: TcpStream,
    channel: TxChannel,
    is_connected: bool,
}

impl Client {
    fn handle_new_msg(&mut self, msg: &String) -> Option<ChannelMessage> {
        use ChannelMessage::*;

        if let Ok(message) = serde_json::from_str::<ClientMessage>(msg) {
            Some(match message {
                ClientMessage::Message(payload) => BroadcastMessage {
                    sender: self.name.clone(),
                    content: payload,
                },
                ClientMessage::Connect(payload) => {
                    self.name = payload;
                    self.is_connected = true;
                    NewClient((self.name.clone(), self.stream.try_clone().unwrap()))
                }
                ClientMessage::PrivateMessage { recipient, content } => {
                    PrivateMessage { recipient, content }
                }
                ClientMessage::Disconnect => Disconnect(self.name.clone()),
            })
        } else {
            eprintln!("'{}' sent an errored message", self.name);
            None
        }
    }

    pub fn detach(mut self) {
        std::thread::spawn(move || loop {
            let mut reader = BufReader::new(&self.stream);
            let mut buff = String::new();

            if let Ok(size) = reader.read_line(&mut buff) {
                if size == 0 {
                    self.channel
                        .send(Disconnect(self.name.clone()))
                        .expect("Failed to notify server");
                    break;
                }
                if let Some(msg) = self.handle_new_msg(&buff) {
                    match &msg {
                        ChannelMessage::Disconnect(_) => {
                            println!("User {} disconnected", self.name);
                            return;
                        }
                        _ => {
                            if !self.is_connected {
                                println!(
                                    "{} user tried to do an action not being connected",
                                    self.name
                                );
                                continue;
                            }
                        }
                    }
                    self.channel.send(msg).unwrap();
                }
            }
        });
    }

    pub fn new(stream: TcpStream, channel: TxChannel) -> Self {
        Self {
            name: String::default(),
            stream,
            channel,
            is_connected: false,
        }
    }
}

///Representation of client-side incoming message
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Connect(String),
    Message(String),
    PrivateMessage { recipient: String, content: String },
    Disconnect,
}
