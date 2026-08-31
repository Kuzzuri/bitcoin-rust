use std::collections::HashMap;
use std::io::{Read, Write};
use chrono::{DateTime, Utc};
use crate::sha256::Hash;
use crate::types::{Transaction, TransactionOutput};
use crate::types::Block;
use crate::crypto::PrivateKey;
use wincode::{SchemaRead, SchemaWrite};
use wincode::config::ConfigCore;
use wincode::TypeMeta;
use std::path::Path;
use std::fs::File;
use std::io::Result as IoResult;

#[derive(Debug, SchemaRead, SchemaWrite)]
struct BlockChain {
    utxos: HashMap<Vec<u8>, TransactionOutput>, 
    mempool: Vec<(Transaction, TimeStamp)>,
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

    fn save_to_disk(&self) -> IoResult<()>{
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/blockchain.dat", cargo_path.display());
        let mut f = File::create(path)?;

        let bytes = wincode::serialize(self).unwrap();
        f.write_all(&bytes)?;

        Ok(())
    }

    fn load_from_disk() -> IoResult<Self> {
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/blockchain.dat", cargo_path.display());
        let mut f = File::open(path)?;
        let mut buffer = vec![];
        f.read_to_end(&mut buffer)?;

        let blockchain = wincode::deserialize(&buffer).unwrap();
        Ok(blockchain)
    }
}

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
