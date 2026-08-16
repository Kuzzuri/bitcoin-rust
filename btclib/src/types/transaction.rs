use crate::crypto::PublicKey;
use crate::VERSION;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct TransactionInput {
    txid: u32,
    vout: u8,
}

#[derive(Debug)]
pub struct TransactionOutput {
    amount: u8,
    public_key: PublicKey,
}
