use std::collections::HashMap;
use std::net::{TcpStream, Shutdown};
use std::io::{Error as IoError};
use crate::types::Wallet;
use btclib::crypto::PrivateKey;
use btclib::error::BtcError;
use btclib::network::Message;
use btclib::types::TransactionOutput;

pub fn handle_stream(stream: TcpStream, wallet: Wallet) -> Result<(), BtcError> {
    let message = Message::RequestUtxos;
    message.send(&stream).unwrap();
    match Message::receive(&stream).unwrap() {
        Message::Utxos(utxo) => {
            let tx = wallet.create_transaction(utxo, vec![0], PrivateKey::new().public_key(), 100).unwrap();
            let message = Message::ValidateTransaction(tx.clone());
            message.send(&stream).unwrap();
            Ok(())
        },
        _ => {
            stream.shutdown(Shutdown::Both).unwrap();
            return Err(BtcError::InvalidMessage);
        }
    }

}
