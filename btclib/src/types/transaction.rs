use std::collections::HashMap;

use crate::crypto::{PrivateKey, PublicKey, Signature};
use crate::sha256::Hash;
use crate::VERSION;
use crate::traits::{Saveable, KeyStore};
use wincode::{SchemaRead, SchemaWrite};
use crate::types::{BlockChain, blockchain};
use crate::error::BtcError;

#[derive(SchemaWrite, SchemaRead, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    locktime: u32
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>, locktime: u32) -> Self {
        Transaction {
            version: VERSION,
            inputs: inputs,
            outputs: outputs,
            locktime: locktime,
        }
    }
    pub fn get_txid(&self) -> Hash {
        Hash::hash256(self)
    }
    pub fn sign_inputs(&mut self, utxos: HashMap<String, TransactionOutput>, keychain: &impl KeyStore) -> Result<(), BtcError>{
        let inputs = self.inputs.clone();
        for input in inputs {
            let Some(output) = utxos.get(&format!("{:x}:{}", input.prev_txid.0, input.vout)) else {
                return Err(BtcError::InvalidTransaction); 
            };
            let priv_key = keychain.get_value(output.script_pub_key).unwrap(); 
            input.sign_input(self, output.script_pub_key, priv_key);
        }
        Ok(())
    }

    pub fn calculate_miner_fee(&self, blockchain: &BlockChain) -> Result<u64, BtcError> {
        let utxos = &blockchain.utxos;
        let mut budget = 0;
        let mut spent = 0;
        for input in &self.inputs {
            let Some(txout) = utxos.get(&format!("{:x}:{}", input.prev_txid.0, input.vout)) else {
               return Err(BtcError::InvalidTransaction);
            };
            budget += txout.amount;
        }

        for output in &self.outputs {
            spent += output.amount;
        }

        if spent > budget {
            return Err(BtcError::InvalidTransaction);
        }
        Ok(budget - spent)
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy, PartialEq, Eq)]
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
                input.script_sig = Some(ScriptSig {block_height: None, public_key: pub_key, signature: None});
            }
        }
        let tx_hash = Hash::hash256(tx_clone);
        let signature = priv_key.sign_data(&tx_hash.as_bytes());
        let signature_input: &mut TransactionInput = tx.inputs.iter_mut().nth(input_index.unwrap() as usize).unwrap();
        signature_input.script_sig = Some(ScriptSig {block_height: None, public_key: pub_key, signature: Some(signature)});
    }

    pub fn verify_signature(&self, tx: &Transaction) -> bool {
        let mut tx_clone = tx.clone();
        let signature = self.script_sig.unwrap().signature.unwrap(); 
        let input_bytes = wincode::serialize(self).unwrap();
        for input in tx_clone.inputs.iter_mut() {
            if input_bytes == wincode::serialize(input).unwrap() {
                input.script_sig.as_mut().unwrap().signature = None;
            } else {
                input.script_sig = None;
            }
        }
        let tx_hash = Hash::hash256(tx_clone);
        self.script_sig.unwrap().public_key.verify(&tx_hash.as_bytes(), signature).is_ok()
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransactionOutput {
    pub amount: u64,
    pub script_pub_key: PublicKey,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptSig {
    pub block_height: Option<u32>,
    pub public_key: PublicKey,
    pub signature: Option<Signature> 
}
