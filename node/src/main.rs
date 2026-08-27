use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Result, Write};
use std::thread;
use btclib::network::Message;
use tokio::time::{self, Duration};

fn handle_client(stream: &TcpStream) {
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8333")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(&stream)
                });
            },
            Err(e) => eprintln!("{}", e)
        }
    }
    Ok(())
}
