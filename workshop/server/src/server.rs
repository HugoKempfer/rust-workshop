use crate::client::Client;
use crate::ChannelMessage;
use std::collections::HashMap;
use std::io::BufWriter;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

pub struct Server {
    clients: HashMap<String, (String, TcpStream)>,
    socket: TcpListener,
    rx_channel: mpsc::Receiver<ChannelMessage>,
    tx_channel: mpsc::Sender<ChannelMessage>,
    addr: String,
}

impl Server {
    pub fn open(domain: &str, port: u16) -> Result<Self, String> {
        let addr = format!("{}:{}", domain, port);
        let (tx, rx) = mpsc::channel::<ChannelMessage>();

        match TcpListener::bind(&addr) {
            Ok(socket) => Ok(Self {
                socket,
                clients: HashMap::new(),
                rx_channel: rx,
                tx_channel: tx,
                addr,
            }),
            Err(msg) => Err(msg.to_string()),
        }
    }

    fn listen_for_clients(socket: TcpListener, channel: mpsc::Sender<ChannelMessage>) {
        for new_client in socket.incoming() {
            match new_client {
                Ok(stream) => {
                    let new_channel = channel.clone();
                    println!("New connection received");
                    Client::new(stream, new_channel).detach();
                }
                Err(msg) => eprintln!("{}", msg),
            }
        }
    }

    fn broadcast_message(clients: &mut HashMap<String, (String, TcpStream)>, msg: &String) {
        for (_, v) in clients.iter_mut() {
            v.1.write((serde_json::to_string(msg).unwrap_or("{}".to_owned()) + "\n").as_bytes())
                .unwrap();
        }
    }

    fn handle_client_messages(&mut self) {
        for msg in (&self.rx_channel).iter() {
            match msg {
                ChannelMessage::NewClient(client) => {
                    println!("New client connected => {}", client.0);
                    self.clients.insert(client.0.clone(), (client.0, client.1));
                }
                ChannelMessage::PrivateMessage { recipient, content } => {
                    println!("Sending private message");
                }
                ChannelMessage::BroadcastMessage(msg) => {
                    Server::broadcast_message(&mut self.clients, &msg);
                }
                ChannelMessage::Disconnect(client) => {
                    println!("Client {} disconnected", client);
                    self.clients.remove(&client);
                }
            }
        }
    }

    pub fn start(mut self) {
        let listener = self.socket.try_clone().unwrap();
        let channel = self.tx_channel.clone();

        std::thread::spawn(move || Server::listen_for_clients(listener, channel));
        println!("Server listening for noobs on {}", &self.addr);
        self.handle_client_messages();
    }
}
