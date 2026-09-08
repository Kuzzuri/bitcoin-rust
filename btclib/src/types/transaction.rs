use crate::crypto::{PrivateKey, PublicKey, Signature};
use crate::sha256::Hash;
use crate::VERSION;
use crate::traits::{Saveable, KeyStore};
use wincode::{SchemaRead, SchemaWrite};
use crate::types::{BlockChain, blockchain};
use crate::error::BtcError;

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
    pub fn sign_inputs(&mut self, keychain: &impl KeyStore) -> Result<(), BtcError>{
        let inputs = self.inputs.clone();
        let blockchain = BlockChain::load_from_file("blockchain.dat").unwrap();
        let utxos = blockchain.utxos;
        for input in inputs {
            let Some(output) = utxos.get(&format!("{:?}:{}", input.prev_txid, input.vout)) else {
                return Err(BtcError::InvalidTransaction); 
            };
            let priv_key = keychain.get_value(input.script_sig.unwrap().public_key).unwrap(); 
            input.sign_input(self, output.script_pub_key, priv_key);
        }
        Ok(())
    }

    pub fn calculate_miner_fee(&self) -> Result<u64, BtcError> {
        let blockchain = BlockChain::load_from_file("blockchain.dat").unwrap();
        let utxos = blockchain.utxos;
        let mut budget = 0;
        let mut spent = 0;
        for input in &self.inputs {
            let Some(txout) = utxos.get(&format!("{}:{}", input.prev_txid.as_hex(), input.vout)) else {
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

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
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

    pub fn verify_signature(&self) -> bool {
        true
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct TransactionOutput {
    pub amount: u64,
    pub script_pub_key: PublicKey,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct ScriptSig {
    pub public_key: PublicKey,
    pub signature: Option<Signature> 
}
