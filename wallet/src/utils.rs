use std::net::{TcpStream, Shutdown};
use std::io::{Error as IoError};
use crate::types::Wallet;
use btclib::crypto::PrivateKey;
use btclib::network::Message;

pub fn handle_stream(stream: TcpStream, wallet: Wallet) -> Result<(), IoError> {
    let tx = wallet.create_transaction(vec![0], PrivateKey::new().public_key(), 100).unwrap();
    let message = Message::ValidateTransaction(tx.clone());
    message.send(&stream)?;
    Ok(())
}
