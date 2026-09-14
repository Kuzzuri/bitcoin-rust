mod utils;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Result as IoResult, Error as IoError};
use std::thread;
use utils::handle_client;


fn main() -> Result<(), IoError> {
    let listener = TcpListener::bind("127.0.0.1:8333")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(&stream).unwrap()
                });
            },
            Err(e) => eprintln!("{}", e)
        }
    }
    Ok(())
}
