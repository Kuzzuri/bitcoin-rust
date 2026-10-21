use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Read, Result, Write};
use wincode::{SchemaWrite, SchemaRead};
use crate::types::{Block, Transaction, TransactionOutput};
use crate::crypto::PublicKey;

#[derive(Debug, SchemaWrite, SchemaRead)]
pub enum Message {
    FetchBlockTemplate(PublicKey),
    CandidateBlock(Block),
    ValidateTransaction(Transaction),
    RequestUtxos,
    Utxos(HashMap<String, TransactionOutput>)
}

impl Message {
    pub fn send(&self, mut stream: &TcpStream) -> Result<()> {
        let message = wincode::serialize(&self).unwrap();
        let msg_len = message.len() as u32;
        let buffer = [&msg_len.to_le_bytes()[..], &message].concat();
        stream.write_all(&buffer)?;
        Ok(())
    }

    pub fn receive(mut stream: &TcpStream) -> Result<Self> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        let msg_len = u32::from_le_bytes(buf);
        let mut buffer = vec![0u8; msg_len as usize];
        stream.read_exact(&mut buffer)?;
        Ok(wincode::deserialize(&buffer).unwrap()) 
    }
}
