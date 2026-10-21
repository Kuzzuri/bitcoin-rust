mod types; mod utils;

use std::net::{TcpStream};
use std::io::{Error as IoError};
use btclib::traits::Saveable;
use btclib::types::BlockChain;
use types::Wallet;
use utils::handle_stream;
use btclib::crypto::PublicKey;

fn main() -> Result<(), IoError> {
    let stream = TcpStream::connect("127.0.0.1:8333")?;

    let wallet = Wallet::load_from_file("wallet.dat")?;
    handle_stream(stream, wallet).unwrap();
    Ok(())
}
