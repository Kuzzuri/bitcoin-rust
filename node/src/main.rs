mod utils;

use std::env;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Result as IoResult, Error as IoError};
use std::thread;
use btclib::traits::Saveable;
use btclib::types::BlockChain;
use utils::{handle_client, handle_peer};
use std::sync::{Arc, RwLock};
use wincode::{SchemaRead, SchemaWrite};
use ctrlc;

#[derive(SchemaWrite, SchemaRead, Clone)]
struct Nodes {
    addresses: Vec<String>
}

impl Nodes {
    fn new() -> Self {
        Nodes { addresses: Vec::new() }
    }
}

impl Saveable for Nodes {}

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
    let nodes: Nodes;
    let local_nodes = Nodes::load_from_file("peers.dat");
    match local_nodes {
        Ok(local_nodes) => { 
            nodes = local_nodes;
            println!("current connected nodes: {}", nodes.addresses.len());
            for node in nodes.clone().addresses {
                thread::spawn(move || {
                    handle_peer(node);
                });
            }
        }
        Err(_) => {
            println!("no peers found");
            nodes = Nodes::new();
        }
    }
    let blockchain = Arc::new(RwLock::new(chain));
    let local_chain = Arc::clone(&blockchain);
    let local_node = nodes.clone();
    ctrlc::set_handler(move || {
        local_chain.write().unwrap().save_to_file("blockchain.dat").unwrap();
        local_node.save_to_file("peers.dat").unwrap();
        std::process::exit(0);
    }).expect("ctrl c error");
    let ip_adress = env::var("ADDRESS".to_string()).unwrap_or("127.0.0.1".to_string());
    let listener = TcpListener::bind(format!("{}:8333", ip_adress))?;
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
