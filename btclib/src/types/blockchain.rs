use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::{BLOCK_REWARD, TARGET, bits_to_target, target_to_bits};
use crate::sha256::Hash;
use crate::types::{Block, Transaction, TransactionInput, TransactionOutput};
use crate::crypto::{PrivateKey, PublicKey};
use wincode::{SchemaRead, SchemaWrite, config::ConfigCore, TypeMeta};
use crate::error::BtcError;
use crate::traits::Saveable;
use std::cmp::Reverse;

#[derive(Debug, SchemaRead, SchemaWrite)]
pub struct BlockChain {
    pub utxos: HashMap<String, TransactionOutput>, 
    pub mempool: Vec<(Transaction, TimeStamp)>,
    pub blocks: Vec<Block>
}

impl BlockChain {
    fn new() -> Self {
        let genesis_block = Block::new(PrivateKey::new().public_key(), Hash::zero(), vec![], 0);
        let utxos = HashMap::new();
        BlockChain { utxos, mempool: Vec::new(), blocks: vec![genesis_block] }
    }

    fn block_height(&self) -> usize {
        self.blocks.len()
    }

    pub fn validate_transcation(&self, transaction: &Transaction) -> Result<(), BtcError> {
        let mut total_budget = 0;
        let mut total_spent = 0;
        let mut used_inputs: Vec<String> = Vec::new();
        for input in &transaction.inputs {
               let utxo_key = format!("{:x}:{}", input.prev_txid.0, input.vout);
               if used_inputs.contains(&utxo_key) {
                   eprint!("double spend");
                   return Err(BtcError::InvalidTransaction);
               }
               if let Some(txout) = self.utxos.get(&utxo_key) {
                    total_budget += txout.amount;
                    used_inputs.push(utxo_key);
               } else {
                   println!("key not found: {}", utxo_key);
                    return Err(BtcError::InvalidTransaction);
               }
        }
        for output in &transaction.outputs {
            total_spent += output.amount;
        }
        if total_budget <= 0 || total_spent <= 0 || total_spent > total_budget {
            return Err(BtcError::InvalidTransaction);
        }

        Ok(())
    }

    pub fn validate_block(&mut self, block: Block) -> Result<(), BtcError> {
        // Nbits check
        if block.bits != target_to_bits(TARGET) {
            return Err(BtcError::InvalidBlock);
        }

        // Pow check
        if block.hash().0 > TARGET {
            return Err(BtcError::InvalidBlock); 
        };

        // Validating prev block hash
        if self.blocks.last().unwrap().hash() != block.previous_block {
            return Err(BtcError::InvalidBlock);
        }
        // First tx is coinbase
        let coinbase = block.transactions.first().unwrap();
        let coinbase_reward = BLOCK_REWARD + block.transactions.iter().skip(1).map(|tx| tx.calculate_miner_fee().unwrap()).sum::<u64>();
        if coinbase.outputs.len() != 1 || coinbase.outputs.first().unwrap().amount !=  coinbase_reward {
            return Err(BtcError::InvalidBlock);
        }

        // No other coinbase
        for tx in block.transactions.iter().skip(1) {
            if tx.outputs.len() == 0 {
                return Err(BtcError::InvalidBlock);
            }
        }

        // Duplicate tx check
        let mut txs = Vec::new();
        for tx in &block.transactions {
            let curr_tx = tx.get_txid();
            if txs.contains(&curr_tx) {
                return Err(BtcError::InvalidBlock);
            } else {
                txs.push(curr_tx);
            }

            // Validate tx sig
            for input in &tx.inputs {
                if input.prev_txid == Hash::zero() {
                    continue;
                }
                if !input.verify_signature(&tx) {
                    return Err(BtcError::InvalidTransaction);
                }
            }


        }
        // Validate txs
        for tx in block.transactions.iter().skip(1) {
            if !self.validate_transcation(tx).is_ok() {
                return Err(BtcError::InvalidTransaction);
            };
        }
        // Validate merkle root
        if block.merkle_root != Block::merkle_root(&block.transactions) {
            return Err(BtcError::InvalidBlock);
        }
        // no dupe blocks
        for tx in &block.transactions {
            for input in &tx.inputs {
                self.utxos.remove(&format!("{:x}:{}", input.prev_txid.0, input.vout));
            }
        }
        Ok(())
    }

    pub fn create_block_template(&self, address: PublicKey) -> Block {
        let mut txs: Vec<Transaction> = self.mempool.clone().into_iter().map(|(tx, _)| tx).collect::<Vec<Transaction>>();
        txs.sort_by_key(|tx| Reverse(tx.calculate_miner_fee().unwrap()));
        let txs = txs.into_iter().take(50).collect();
        let prev_block = self.blocks.last();
        let block = Block::new(address, prev_block.unwrap().hash(), txs, self.block_height() as u32);
        block
    }

    pub fn handle_block_txs(&mut self, block: Block) {
        // remove tx's from mempool
        let block_txs = block.transactions;
        self.mempool.retain(|(tx, _)| !block_txs.contains(tx));

        // add tx outputs to utxos set 
        for tx in block_txs {
            let outputs = &tx.outputs;
            for (idx, output) in outputs.iter().enumerate() {
                let utxo_key = format!("{:x}:{}", tx.get_txid().0, idx);
                println!("adding utxo with txid: {:x} with vout: {idx}", tx.get_txid().0);
                self.utxos.insert(utxo_key, *output);
            }
        }
    }
}

impl Saveable for BlockChain {}

#[derive(Debug, Clone, Copy)]
pub struct TimeStamp(DateTime<Utc>);

impl TimeStamp {
    pub fn new() -> Self {
        let now_utc = Utc::now();
        TimeStamp(now_utc)
    }
}

unsafe impl<C: ConfigCore> SchemaWrite<C> for TimeStamp {
    type Src = TimeStamp;

    const TYPE_META: wincode::TypeMeta = TypeMeta::Dynamic;

    fn size_of(src: &Self::Src) -> wincode::WriteResult<usize> {
        Ok(src.0.timestamp_millis().to_le_bytes().len()) 
    }

    fn write(mut writer: impl wincode::io::Writer, src: &Self::Src) -> wincode::WriteResult<()> {
        let bytes = src.0.timestamp_millis().to_le_bytes();
        writer.write(&bytes).unwrap();
    
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for TimeStamp {
    type Dst = TimeStamp;

    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn read(reader: impl wincode::io::Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
        let bytes = <[u8; 8] as SchemaRead<'de, C>>::get(reader).unwrap();
        let millis = u64::from_le_bytes(bytes);
        let timestamp = DateTime::from_timestamp_millis(millis as i64).unwrap();
        dst.write(TimeStamp(timestamp));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_chain() {
        let block_chain = BlockChain::new();
        block_chain.save_to_file("blockchain.dat");

    }

    #[test]
    fn utxo_check() {
        let block_chain = BlockChain::load_from_file("blockchain.dat").unwrap();
        println!("blocks: {:#?}", block_chain.blocks);
        println!("utxos: {:#?}", block_chain.utxos);
        println!("mempool: {:#?}", block_chain.mempool);
        for block in block_chain.blocks {
            println!("{:#?}", block.hash());
        }
    }
}
