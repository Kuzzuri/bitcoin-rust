use crate::crypto::{PrivateKey, PublicKey, Signature};
use crate::sha256::Hash;
use crate::VERSION;
use wincode::{SchemaRead, SchemaWrite};
use crate::U256;

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct Transaction {
    version: u32,
    input_count: u64,
    pub inputs: Vec<TransactionInput>,
    output_count: u64,
    pub outputs: Vec<TransactionOutput>,
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
        Hash::hash256(bytes)
    }
    pub fn sign_inputs(&mut self) {
        let inputs = self.inputs.clone();
        for input in inputs {
            // 1. fetch output from prev_tx_id+vout
            // 2. get scriptPubKey from the output
            // 3. fetch privateKey corresponding to the scriptPubKey locally
            
            // input.sign_input(self, pub_key, priv_key);
        }
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct TransactionInput {
    pub prev_txid: Hash,
    pub vout: u32,
    pub script_sig: Option<ScriptSig>
}

impl TransactionInput {
    pub fn sign_input(&self, tx: &mut Transaction, pub_key: PublicKey, priv_key: PrivateKey) {
        let mut tx_clone = tx.clone();
        for input in tx_clone.inputs.iter_mut() {
            input.script_sig = None;
        }
        let input_bytes = wincode::serialize(self).unwrap();
        let mut input_index: Option<u8> = None;
        for (idx, input) in tx_clone.inputs.iter_mut().enumerate() {
            if input_bytes == wincode::serialize(input).unwrap() {
                input_index = Some(idx as u8);
                input.script_sig = Some(ScriptSig {public_key: pub_key, signature: None});
            }
        }
        let tx_hash = Hash::hash256(tx_clone);
        let signature = priv_key.sign_data(&tx_hash.as_bytes());
        let signature_input: &mut TransactionInput = tx.inputs.iter_mut().nth(input_index.unwrap() as usize).unwrap();
        signature_input.script_sig = Some(ScriptSig {public_key: pub_key, signature: Some(signature)});
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct TransactionOutput {
    pub amount: u64,
    pub script_pub_key: PublicKey,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct ScriptSig {
    public_key: PublicKey,
    signature: Option<Signature> 
}
