use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::sha256::Hash;
use crate::types::{Transaction, TransactionOutput};
use crate::types::Block;
use crate::crypto::PrivateKey;

#[derive(Debug)]
struct BlockChain {
    utxos: HashMap<Vec<u8>, TransactionOutput>, 
    mempool: Vec<(Transaction, DateTime<Utc>)>,
    blocks: Vec<Block>
}

impl BlockChain {
    fn new() -> Self {
        let coinbase_tx = Transaction::new(vec![], vec![TransactionOutput { amount: 50, script_pub_key: PrivateKey::new().public_key() }], 0);
        let genesis_block = Block::new(Hash::zero(), vec![coinbase_tx]);
        BlockChain { utxos: HashMap::new(), mempool: Vec::new(), blocks: vec![genesis_block] }
    }

    fn block_height(&self) -> usize {
        self.blocks.len()
    }
}
