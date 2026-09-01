use btclib::crypto::{PrivateKey, PublicKey};
use wincode::{SchemaRead, SchemaWrite};
use btclib::traits::Saveable;
use btclib::types::{Transaction, TransactionInput, TransactionOutput, BlockChain};

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

    fn delete_keypair(&mut self, keypair: KeyPair) {
        if let Some(keys) = &mut self.keys {
            keys.retain(|kp| *kp != keypair); 
        }
    }

    fn get_balance(&self) -> u64 {
        let mut balance = 0;
        let Some(keys) = &self.keys else {
            return balance;
        };
        let adresses = keys.iter().map(|kp| kp.public_key).collect::<Vec<_>>();
        let blockchain = BlockChain::load_from_file("blockchain.dat").unwrap();
        for (_, txout) in blockchain.utxos {
            let adress = txout.script_pub_key;

            if adresses.contains(&adress) {
                balance += txout.amount;
            }
        }
        balance
    }
}

impl Saveable for Wallet {}
