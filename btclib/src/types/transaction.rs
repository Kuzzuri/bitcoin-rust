use crate::crypto::{PublicKey, PrivateKey};
use crate::hash::Hash;
use crate::VERSION;
use wincode::{SchemaRead, SchemaWrite};

#[derive(SchemaWrite, SchemaRead, Debug)]
pub struct Transaction {
    version: u8,
    input_count: u8,
    input: Vec<TransactionInput>,
    output_count: u8,
    output: Vec<TransactionOutput>,
    locktime: u8
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>, locktime: u8) -> Self {
        Transaction {
            version: VERSION,
            input_count: inputs.len() as u8,
            input: inputs,
            output_count: outputs.len() as u8,
            output: outputs,
            locktime: locktime,
        }
    }
    pub fn get_txid(&self) -> Hash {
        let bytes = wincode::serialize(self).unwrap();
        Hash::hash(Hash::hash(bytes))
    }

}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct TransactionInput {
    txid: u32,
    vout: u8,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct TransactionOutput {
    amount: u8,
    public_key: PublicKey,
}
