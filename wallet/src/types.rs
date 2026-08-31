use btclib::crypto::{PrivateKey, PublicKey};
use wincode::{SchemaRead, SchemaWrite};
use btclib::traits::Saveable;

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
}

impl Saveable for Wallet {}
