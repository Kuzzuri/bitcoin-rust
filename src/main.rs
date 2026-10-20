use btclib::{traits::Saveable, types::BlockChain};

fn main() {
    let block_chain = BlockChain::load_from_file("blockchain.dat").unwrap();

    println!("blockchain: {:#?}", block_chain.blocks);
}
