mod types; mod utils;

use std::net::{TcpStream};
use std::io::{Error as IoError};
use btclib::traits::Saveable;
use types::Wallet;
use utils::handle_stream;
use btclib::crypto::PublicKey;

fn main() -> Result<(), IoError> {
    let stream = TcpStream::connect("127.0.0.1:8333")?;

    let wallet = Wallet::load_from_file("wallet.dat")?;
    let public_key = PublicKey::load_from_file("umut_public.pem")?;
    let keypair = wallet.find_keypair(public_key).unwrap();
    handle_stream(stream, wallet)?;
    Ok(())
}
