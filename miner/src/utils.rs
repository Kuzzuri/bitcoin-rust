use crate::types::Miner;
use std::io::{Write, Result as IoResult};
use std::net::{TcpStream, Shutdown};
use std::{time, thread};
use btclib::network::Message::{self, *};

pub async fn handle_stream(stream: &TcpStream, miner: Miner) -> IoResult<()> {
    let dur = time::Duration::from_millis(3000);
    loop {
        let msg = FetchBlockTemplate(miner.address);
        msg.send(stream)?;
        let response = Message::receive(stream)?;
        match  response {
            CandidateBlock(block) => {
                let candidate_block = miner.mine_block(block);
                let message = CandidateBlock(candidate_block);
                message.send(stream)?;
            },
            _ => {
                println!("unknown message, terminating connection");
                stream.shutdown(Shutdown::Both)?;
                return Ok(());
            },
        }
       thread::sleep(dur);
    }
}
