mod utils;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Result as IoResult, Error as IoError};
use std::thread;
use btclib::traits::Saveable;
use btclib::types::BlockChain;
use utils::handle_client;
use std::sync::{Arc, RwLock};


fn main() -> Result<(), IoError> {
    let chain: BlockChain;
    let local_chain = BlockChain::load_from_file("blockchain.dat");
    match local_chain {
        Ok(local_chain) => { chain = local_chain; },
        Err(_) => { 
            println!("starting new chain");
            chain = BlockChain::new();
        },
    }
    let blockchain = Arc::new(RwLock::new(chain));
    let listener = TcpListener::bind("127.0.0.1:8333")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let blockchain = Arc::clone(&blockchain);
                thread::spawn(move || {
                    handle_client(&blockchain, &stream).unwrap()
                });
            },
            Err(e) => eprintln!("{}", e)
        }
    }
    Ok(())
}
