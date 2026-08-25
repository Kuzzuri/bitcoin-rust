use crate::{U256, VERSION};
use crate::types::{Transaction, transaction};
use crate::sha256::Hash;

pub struct Block {
    version: u32,
    previous_block: Hash,
    merkle_root: U256,
    time: u32,
    bits: u32,
    nonce: u32,
    transaction_count: u64,
    transactions: Vec<Transaction>
}

impl Block {
    fn new(prev_block: Hash, transactions: Vec<Transaction>) {

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


