use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::sha256::Hash;
use crate::types::{Transaction, TransactionOutput, Block};
use crate::crypto::{PrivateKey, PublicKey};
use wincode::{SchemaRead, SchemaWrite, config::ConfigCore, TypeMeta};
use crate::error::BtcError;
use crate::traits::Saveable;
use crate::{BLOCK_REWARD, TARGET};

#[derive(Debug, SchemaRead, SchemaWrite)]
pub struct BlockChain {
    pub utxos: HashMap<String, TransactionOutput>, 
    pub mempool: Vec<(Transaction, TimeStamp)>,
    pub blocks: Vec<Block>
}

impl BlockChain {
    fn new() -> Self {
        let genesis_block = Block::new(PrivateKey::new().public_key(), Hash::zero(), vec![]);
        let coinbase_tx = genesis_block.transactions.iter().nth(0).unwrap();
        let ctx_key = format!("{}:{}", coinbase_tx.get_txid().as_hex(), 0);
        let mut utxos = HashMap::new();
        utxos.insert(ctx_key, *coinbase_tx.outputs.iter().nth(0).unwrap());
        BlockChain { utxos, mempool: Vec::new(), blocks: vec![genesis_block] }
    }

    fn block_height(&self) -> usize {
        self.blocks.len()
    }

    fn validate_transcation(&self, transaction: &Transaction) -> Result<(), BtcError> {
        let mut total_budget = 0;
        let mut total_spent = 0;
        for input in &transaction.inputs {
               let utxo_key = format!("{}:{}", input.prev_txid.as_hex(), input.vout);
               if let Some(txout) = self.utxos.get(&utxo_key) {
                    total_budget += txout.amount;
               } else {
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

    fn validate_block(&self, block: Block) {
        // validate prev block Hash
        // validate syntatically
        // validate merkle root
    }

    fn create_block_template(&self, address: PublicKey) -> Block {
        let mut txs: Vec<Transaction> = self.mempool.clone().into_iter().map(|(tx, _)| tx).take(50).collect();
        txs.sort_by_key(|tx| tx.calculate_miner_fee().unwrap());
        let prev_block = self.blocks.last();
        let block = Block::new(address, prev_block.unwrap().get_header(), txs);
        block
    }
}

impl Saveable for BlockChain {}

#[derive(Debug, Clone, Copy)]
struct TimeStamp(DateTime<Utc>);

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

    #[test]
    fn test() {
        let bc = BlockChain::load_from_file("blockchain.dat").unwrap();
        let block = Block::new(PrivateKey::new().public_key(), Hash::zero(), vec![]);
        bc.validate_block(block);
    }
}
