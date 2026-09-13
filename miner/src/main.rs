mod types;
mod utils;

use std::net::TcpStream;
use btclib::traits::Saveable;
use btclib::{crypto::PublicKey};
use types::Miner;
use utils::handle_stream;


#[tokio::main]
async fn main(){
    let pk = PublicKey::load_from_file("umut_public.pem").unwrap();
    let miner = Miner::new(pk);

    let stream = TcpStream::connect("127.0.0.1:8333").unwrap();
    handle_stream(&stream, miner).await.unwrap();
}
