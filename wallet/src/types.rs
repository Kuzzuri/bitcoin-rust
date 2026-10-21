use btclib::U256;
use btclib::crypto::{PrivateKey, PublicKey};
use btclib::error::BtcError;
use wincode::{SchemaRead, SchemaWrite};
use btclib::traits::{Saveable, KeyStore};
use btclib::types::{Transaction, TransactionInput, TransactionOutput};
use btclib::sha256::Hash;
use std::collections::HashMap;

#[derive(Debug, SchemaRead, SchemaWrite, PartialEq, Eq, Clone)]
pub struct KeyPair {
    pub private_key: PrivateKey,
    pub public_key: PublicKey
}

impl KeyPair {
    fn new() -> KeyPair {
        let private_key = PrivateKey::new();
        let public_key = private_key.public_key();

        KeyPair { private_key, public_key }
    }
}

#[derive(Debug, SchemaRead, SchemaWrite)]
pub struct Wallet {
    pub keys: Option<Vec<KeyPair>>
}

impl Wallet {
    fn new() -> Self {
        Wallet { keys: None }
    }

    fn add_keypair(&mut self, keypair: KeyPair) {
        if let Some(keys) = &mut self.keys {
            keys.push(keypair);
        } else {
            self.keys = Some(vec![keypair]);
        }
    }

    pub fn find_keypair(&self, public_key: PublicKey) -> Option<KeyPair> {
        if let Some(keys) = &self.keys {
            let keypair = keys.iter().find(|kp| kp.public_key == public_key);
            return keypair.cloned();
        }
        None
    }

    fn delete_keypair(&mut self, public_key: PublicKey) {
        if let Some(keys) = &mut self.keys {
            keys.retain(|kp| kp.public_key != public_key); 
        }
    }

    pub fn get_balance(&self, utxos: HashMap<String, TransactionOutput>, selected_keys: &Vec<KeyPair>) -> u64 {
        let mut balance = 0;
        let keys;
        if selected_keys.len() > 0 {
            keys = selected_keys;           
        } else if let Some(full_keys) = &self.keys {
            keys = full_keys;
        } else {
            return balance;
        };
        let adresses = keys.iter().map(|kp| kp.public_key).collect::<Vec<_>>();
        for (_, txout) in utxos {
            let adress = txout.script_pub_key;

            if adresses.contains(&adress) {
                balance += txout.amount;
            }
        }
        balance
    }

    fn get_utxos(&self, utxos: HashMap<String, TransactionOutput>, selected_keys: &Vec<KeyPair>) -> Vec<(String, TransactionOutput)> {
        let keys;
        if selected_keys.len() > 0 {
            keys = selected_keys;           
        } else if let Some(full_keys) = &self.keys {
            keys = full_keys;
        } else {
            return Vec::new();
        };
        let adresses = keys.iter().map(|kp| kp.public_key).collect::<Vec<_>>();
        let mut user_utxos: Vec<(String, TransactionOutput)> = Vec::new();
        for (id, txout) in utxos {
            let adress = txout.script_pub_key;

            if adresses.contains(&adress) {
                user_utxos.push((id.clone(), txout.clone()));
            }
        }
        user_utxos
    }

    pub fn create_transaction(&self, utxos: HashMap<String, TransactionOutput>, idxs: Vec<u32>, address: PublicKey, amount: u64) -> Result<Transaction, BtcError> {
        if let Some(keys) = &self.keys {
            let selected_keys: Vec<KeyPair> = keys.iter().enumerate()
                .filter(|(i, _)| idxs.contains(&(*i as u32)))
                .map(|(_, kp)| kp.clone())
                .collect::<Vec<_>>();

            let balance = self.get_balance(utxos.clone(), &selected_keys);
            if balance < amount {
                return Err(BtcError::InvalidTransaction);
            }
            let user_utxos = self.get_utxos(utxos.clone(), &selected_keys);
            let mut curr_amount = 0;
            let mut inputs: Vec<TransactionInput> = Vec::new();
            for (id, txout) in user_utxos {
                let (prev_txid, vout) = id.split_once(":").unwrap();
                curr_amount += txout.amount;
                let prev_tx_hash = U256::from_str_radix(prev_txid, 16).unwrap();
                inputs.push(TransactionInput {prev_txid: Hash(prev_tx_hash), vout: vout.parse::<u32>().unwrap(), script_sig: None});
                if curr_amount >= amount {
                    break;
                }

            }
            let mut tx = Transaction::new(inputs, vec![TransactionOutput {amount, script_pub_key: address}], 0);
            tx.sign_inputs(utxos, self)?;
            return Ok(tx);
        }
        Err(BtcError::InvalidTransaction)
   }

}

impl Saveable for Wallet {}

impl KeyStore for Wallet {
    fn get_value(&self, key: PublicKey) -> Option<PrivateKey> {
        let keypair = self.find_keypair(key);
        Some(keypair.unwrap().private_key)
    }
}
