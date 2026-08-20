use crate::crypto::{PublicKey, Signature};
use crate::hash::Hash;
use crate::VERSION;
use wincode::{SchemaRead, SchemaWrite};
use crate::U256;

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct Transaction {
    version: u32,
    input_count: u64,
    inputs: Vec<TransactionInput>,
    output_count: u64,
    outputs: Vec<TransactionOutput>,
    locktime: u32
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>, locktime: u32) -> Self {
        Transaction {
            version: VERSION,
            input_count: inputs.len() as u64,
            inputs: inputs,
            output_count: outputs.len() as u64,
            outputs: outputs,
            locktime: locktime,
        }
    }
    pub fn get_txid(&self) -> Hash {
        let bytes = wincode::serialize(self).unwrap();
        Hash::hash(Hash::hash(bytes))
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct TransactionInput {
    prev_txid: U256,
    vout: u32,
    script_sig: Option<ScriptSig>
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct TransactionOutput {
    amount: u64,
    script_pub_key: PublicKey,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct ScriptSig {
    public_key: PublicKey,
    signature: Option<Signature> 
}
