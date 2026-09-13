use btclib::{types::Block, crypto::PublicKey};
use btclib::TARGET;
use std::{thread, time};

pub struct Miner {
    pub address: PublicKey
}

impl Miner {
    pub fn new(address: PublicKey) -> Self {
        Miner { address }
    }

    pub fn mine_block(&self, mut block: Block) -> Block {
        let dur = time::Duration::from_millis(1000);

       while block.hash().0 >= TARGET {
           block.nonce += 1;
           println!("trying hash with nonce: {:?}", block.nonce);
           thread::sleep(dur);
       }
       block
    }

    // fn submit_block(block: Block) -> Result<()> {}
}
