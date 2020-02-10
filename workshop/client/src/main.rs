extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub enum Message {}

fn main() {
    let username = std::env::args().nth(1).expect("You must provide a username");
    let ip = std::env::args().nth(2).unwrap_or(String::from("0.0.0.0:5000"));
}
