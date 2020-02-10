# Create a TCP chat in 1 hour

During this workshop you'll learn some basic Rust concepts that will enable you building an IRC-like.

## Setup

To install the Rust toolchain let's execute `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Now restart your shell and try to execute the `cargo` binary. If nothing is found you'll have to add `~/.cargo/bin/` to your $PATH environment variable.

Then you have to switch on the nightly channel of the toolchain via : `rustup default nightly`

## Introduction

As the chat server has already been implemented, you'll have to write a simple client by yourself.

* A small stdin parser
* A TcpStream listening to incoming message on a dedicated thread
* A message dispatcher



## Building the client

In the client directory you can try to run the binary via `cargo run`

After building the dependencies the binary will be executed. Look at the `fn main()` function and the arguments it takes.



## Implement your enum message

The network protocol supports those message types :

* Connect : String - Connect to the server with a given username
* Message: String - Send a message to everyone
* PrivateMessage: {recipient: String, content: String} - Send a message to a single user
* Disconnect



When your enum is implemented add above the declaration this maccro:

```rust
#[derive(Serialize, Deserialize)]
```

This macro enable you to serialize and deserialize messages thank to the *serde* crate (libray).



## Open a new TcpStream

Now its time to connect to the server. You can take a look to the great Rust official documentation here https://doc.rust-lang.org/std/net/struct.TcpStream.html.

Or if you're too lazy here's an example of the use of TCP Stream :

```rust
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
```



## Connection

As you're connected to the server, you can send messages.

To do so, just use the enum you declared, for example to send your username to the server use the Connect variant.

The server accepts json formatted messages, separated by newline character ("\n").

Use the serde function to serialize the messages :

```rust
let message = serde_json::to_string::<YOUR_ENUM_TYPE>(YOUR_MESSAGE).expect("Failed to serialize the message");
```



## Read user inputs

RTFM https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line ¯\\_(ツ)_/¯



## Listening to incoming messages

As reading user inputs is blocking you can't listen to network messages at the same time.

To handle that, we can either use non-blocking sockets or parallelize our code.

You'll see that's pretty easy with Rust.

First, let's duplicate our Tcpstream using

```rust
let newStream = currentStream.try_clone().expect("Failed to duplicate tcp stream");
```

This new stream is a separated owned object pointing to the same socket as your first stream.

In easy words, you just duplicated your socket into a new independent object, you cave **move** it to a new function that will open a new thread.

Opening a new thread :

```rust
std::thread::spawn(move || {
    //Your code
});
```



In this new thread you can use a **loop** to read incoming messages.

## Reading new messages

As the TcpStream only enable us to read messages separated by `EOF` we'll use a Bufreader to readline over the socket.

https://doc.rust-lang.org/std/io/trait.BufRead.html

https://doc.rust-lang.org/std/io/struct.BufReader.html

You can use the ```println!("{}", "hello");``` macro to print formatted messages.