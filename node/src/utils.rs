use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write, Result as IoResult};
use std::sync::RwLock;
use btclib::error::BtcError;
use btclib::network::Message::{self, *};
use btclib::traits::Saveable;
use wincode;
use btclib::types::{BlockChain, Block};
use btclib::types::TimeStamp;

pub fn handle_client(blockchain: &RwLock<BlockChain>, mut stream: &TcpStream) -> IoResult<()>{
    println!("new client connceted");
    loop {
        let mut size_bytes = [0u8; 4];
        stream.read_exact(&mut size_bytes)?;
        let msg_size = u32::from_le_bytes(size_bytes);
        let mut msg_bytes = vec![0u8; msg_size as usize];
        stream.read_exact(&mut msg_bytes)?;

        let message = wincode::deserialize(&msg_bytes).unwrap();

        let mut chain = blockchain.write().unwrap();
        match message {
            FetchBlockTemplate(address) => {
                println!("sending candidate block");
                let candidate_block = chain.create_block_template(address);
                let message = CandidateBlock(candidate_block.clone());
                message.send(stream)?;
            },
            CandidateBlock(block) => {
                println!("received candidate block, verifying");
                match chain.validate_block(block.clone()) {
                    Ok(_) => {
                        chain.blocks.push(block.clone());
                        chain.handle_block_txs(block);
                        chain.save_to_file("blockchain.dat").unwrap();
                    },
                    Err(error) => {
                        println!("invalid block: {}", error);
                    },
                }
            },
            ValidateTransaction(tx) => {
                if let Err(e) = chain.validate_transcation(&tx) {
                    eprintln!("{:?}, terminating connection", e);
                    stream.shutdown(Shutdown::Both)?;
                    return Ok(());
                }
                

                chain.mempool.push((tx, TimeStamp::new()));
                chain.save_to_file("blockchain.dat")?;
                return Ok(());
            },
            _ => {
                println!("unknown message, terminating connection");
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            },
        }
    }
}
