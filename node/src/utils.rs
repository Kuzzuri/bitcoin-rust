use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write, Result as IoResult};
use std::sync::RwLock;
use btclib::crypto::{PrivateKey, PublicKey};
use btclib::error::BtcError;
use btclib::network::Message::{self, *};
use btclib::sha256::Hash;
use btclib::traits::Saveable;
use wincode;
use btclib::types::{BlockChain, Block};
use btclib::types::TimeStamp;

pub fn handle_client(blockchain_lock: &RwLock<BlockChain>, mut stream: &TcpStream) -> IoResult<()>{
    println!("new client connceted");
    loop {
        let mut size_bytes = [0u8; 4];
        stream.read_exact(&mut size_bytes)?;
        let msg_size = u32::from_le_bytes(size_bytes);
        let mut msg_bytes = vec![0u8; msg_size as usize];
        stream.read_exact(&mut msg_bytes)?;

        let message = wincode::deserialize(&msg_bytes).unwrap();

        match message {
            FetchBlockTemplate(address) => {
                println!("sending candidate block");

                let blockchain = blockchain_lock.read().unwrap();
                let candidate_block = blockchain.create_block_template(address);
                drop(blockchain);
                let message = CandidateBlock(candidate_block);
                message.send(stream)?;
            },
            CandidateBlock(block) => {
                println!("received candidate block, verifying");

                let mut blockchain = blockchain_lock.write().unwrap();
                match blockchain.validate_block(block.clone()) {
                    Ok(_) => {
                        blockchain.blocks.push(block.clone());
                        blockchain.handle_block_txs(block);
                        blockchain.save_to_file("blockchain.dat").unwrap();
                    },
                    Err(error) => {
                        println!("invalid block: {}", error);
                    },
                }
            },
            ValidateTransaction(tx) => {

                let mut blockchain = blockchain_lock.write().unwrap();
                if let Err(e) = blockchain.validate_transcation(&tx) {
                    eprintln!("{:?}, terminating connection", e);
                    stream.shutdown(Shutdown::Both)?;
                    return Ok(());
                }
                

                blockchain.mempool.push((tx, TimeStamp::new()));
                blockchain.save_to_file("blockchain.dat")?;
                return Ok(());
            },
            Version => {
                let blockchain = blockchain_lock.read().unwrap();
                let msg = Message::VerAck(blockchain.block_height() as u8);
                msg.send(&stream).unwrap();
            },
            VerAck(block_height) => {
                println!("{:?}", block_height);
            }
            _ => {
                println!("unknown message, terminating connection");
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            },
        }
    }
}

pub fn handle_peer(blockchain_lock: &RwLock<BlockChain>, addr: String) {
    let stream = TcpStream::connect(format!("{addr}:8333")).unwrap();
    let msg = Message::Version;
    msg.send(&stream).unwrap();
    let response = Message::receive(&stream).unwrap();
    match response {
        VerAck(block_height) => { 
            let blockchain = blockchain_lock.read().unwrap();
            if (blockchain.block_height() as u8) < block_height {
                println!("get_headers()");
            }
        },
        _ => { eprintln!("{}", BtcError::InvalidMessage) }
    }
}
