use crate::crypto::PublicKey;
use crate::{TARGET, VERSION, BLOCK_REWARD, target_to_bits};
use crate::types::{Transaction, TransactionOutput};
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
    pub fn new(adress: PublicKey, previous_block: Hash, transactions: Vec<Transaction>) -> Self {
        let coinbase_tx = Transaction::new(vec![], vec![
            TransactionOutput {
                amount: BLOCK_REWARD,
                script_pub_key: adress
            }
        ], 0);
        let mut txs = vec![coinbase_tx];
        txs.extend_from_slice(&transactions);
        Block {
            version: VERSION,
            previous_block,
            merkle_root: Block::merkle_root(&txs),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
            bits: target_to_bits(TARGET),
            nonce: 0,
            transaction_count: txs.len() as u64,
            transactions: txs
        }
    }

    pub fn merkle_root(transactions: &Vec<Transaction>) -> Hash {
        let mut txs: Vec<Hash> = transactions.clone().iter().map(|t| Hash::hash256(wincode::serialize(t).unwrap())).collect();
        while txs.len() > 1 {
            let mut next = Vec::new();
            for tx_pair in txs.chunks(2) {
                if tx_pair.len() == 2 {
                    let tx1 = &tx_pair[0];
                    let tx2 = &tx_pair[1];
                    let cat = [tx1.as_bytes(), tx2.as_bytes()].concat();
                    let hashed = Hash::hash256(cat);
                    next.push(hashed);
                } else if tx_pair.len() == 1 {
                    let tx1 = &tx_pair[0];
                    let tx2 = &tx_pair[0];
                    let cat = [tx1.as_bytes(), tx2.as_bytes()].concat();
                    let hashed = Hash::hash256(cat);
                    next.push(hashed);
                }
            }
            txs = next;
        } 
        txs[0]
    }

    pub fn get_header(&self) -> Hash {
        let mut header = Vec::with_capacity(80);
        header.extend_from_slice(&self.version.to_le_bytes());
        header.extend_from_slice(&self.previous_block.as_bytes());
        header.extend_from_slice(&Block::merkle_root(&self.transactions).as_bytes());
        header.extend_from_slice(&self.time.to_le_bytes());
        header.extend_from_slice(&self.bits.to_le_bytes());
        header.extend_from_slice(&self.nonce.to_le_bytes());

        Hash::hash256(header)
    }
}
