use std::net::TcpStream;
use btclib::sha256::Hash;
use btclib::{crypto::PublicKey, network::Message, types::Block};
use btclib::TARGET;
use btclib::types::BlockChain;
use std::{time, thread};


struct Miner {
    address: PublicKey
}

impl Miner {
    fn new(address: PublicKey) -> Self {
        Miner { address }
    }

    fn mine_block(&self, mut block: Block) -> Block {
        let dur = time::Duration::from_millis(1000);

       while block.hash().0 >= TARGET {
           block.nonce += 1;
           println!("{:?}", block.nonce);
           thread::sleep(dur);
       }
       block
    }

    // fn submit_block(block: Block) -> Result<()> {}
}

async fn handle_stream(stream: &TcpStream) {
    
}

#[tokio::main]
async fn main(){
    let stream = TcpStream::connect("127.0.0.1:8333").unwrap();
    handle_stream(&stream).await;
}


#[cfg(test)]
mod tests {
   use btclib::{crypto::PrivateKey, traits::Saveable};

use super::*;

   #[test]
   fn test() {
      let miner = Miner::new(PrivateKey::new().public_key());
      let bc = BlockChain::load_from_file("blockchain.dat").unwrap();
      let template = bc.create_block_template(miner.address);
      let next_block = miner.mine_block(template);
      println!("{:#?}", next_block);
   }
}
