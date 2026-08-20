use crate::U256;
use crate::types::Transaction;

pub struct Block {
    version: u32,
    previous_block: U256,
    merkle_root: U256,
    time: u32,
    bits: u32,
    nonce: u32,
    transaction_count: u64,
    transactions: Vec<Transaction>
}
