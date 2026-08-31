use btclib::crypto::{PrivateKey, PublicKey};
use wincode::{SchemaRead, SchemaWrite};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, Result as IoResult};

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

    fn save_to_disk(&self) -> IoResult<()> {
        let bytes = wincode::serialize(&self).unwrap();
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/wallet.dat", cargo_path.display());
        let mut f = File::create(path)?;
        f.write_all(&bytes)?;
        
        Ok(())
    }

    fn load_from_disk() -> IoResult<Self> {
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/wallet.dat", cargo_path.display());
        let mut f = File::open(path)?;
        
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        let wallet = wincode::deserialize(&buf).unwrap();

        Ok(wallet)
    }
}
