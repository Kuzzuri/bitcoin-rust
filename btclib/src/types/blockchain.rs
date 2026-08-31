use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::sha256::Hash;
use crate::types::{Transaction, TransactionOutput, Block};
use crate::crypto::PrivateKey;
use wincode::{SchemaRead, SchemaWrite, config::ConfigCore, TypeMeta};
use crate::error::BtcError;
use crate::traits::Saveable;

#[derive(Debug, SchemaRead, SchemaWrite)]
struct BlockChain {
    pub utxos: HashMap<String, TransactionOutput>, 
    pub mempool: Vec<(Transaction, TimeStamp)>,
    pub blocks: Vec<Block>
}

impl BlockChain {
    fn new() -> Self {
        let block_reward = TransactionOutput { amount: 50, script_pub_key: PrivateKey::new().public_key() };
        let coinbase_tx = Transaction::new(vec![], vec![block_reward], 0);
        let genesis_block = Block::new(Hash::zero(), vec![coinbase_tx.clone()]);
        let ctx_key = format!("{}:{}", coinbase_tx.get_txid().as_hex(), 0);
        let mut utxos = HashMap::new();
        utxos.insert(ctx_key, block_reward);
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
}

impl Saveable for BlockChain {}

#[derive(Debug)]
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
