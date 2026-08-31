use crate::{TARGET, VERSION};
use crate::types::Transaction;
use crate::sha256::Hash;
use std::time::{SystemTime, UNIX_EPOCH};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Debug, SchemaWrite, SchemaRead)]
pub struct Block {
    version: u32,
    pub previous_block: Hash,
    pub merkle_root: Hash,
    time: u32,
    bits: u32,
    nonce: u32,
    transaction_count: u64,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(previous_block: Hash, transactions: Vec<Transaction>) -> Self {
        Block {
            version: VERSION,
            previous_block,
            merkle_root: Block::merkle_root(&transactions),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
            bits: TARGET,
            nonce: 0,
            transaction_count: transactions.len() as u64,
            transactions: transactions
        }
    }

    fn merkle_root(transactions: &Vec<Transaction>) -> Hash {
        let mut txs: Vec<Hash> = transactions.clone().iter().map(|t| Hash::hash256(wincode::serialize(t).unwrap())).collect();
        while txs.len() > 1 {
            let mut next = Vec::new();
            for tx_pair in txs.chunks(2) {
                if tx_pair.len() == 2 {
                    let tx1 = &tx_pair[0];
                    let tx2 = &tx_pair[1];
                    println!("{:?}", tx1 == tx2);
                    let cat = [tx1.as_bytes(), tx2.as_bytes()].concat();
                    let hashed = Hash::hash256(cat);
                    next.push(hashed);
                } else if tx_pair.len() == 1 {
                    let tx1 = &tx_pair[0];
                    let tx2 = &tx_pair[0];
                    println!("{:?}", tx1 == tx2);
                    let cat = [tx1.as_bytes(), tx2.as_bytes()].concat();
                    let hashed = Hash::hash256(cat);
                    next.push(hashed);
                }
            }
            txs = next;
        } 
        txs[0]
    }
}
