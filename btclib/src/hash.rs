use crate::U256;
use sha2::{Sha256, Digest};
use wincode::{SchemaRead, SchemaWrite};
use wincode::config::Configuration;

trait Encode: SchemaWrite<Configuration, Src = Self> {}

impl<T> Encode for T
where
    T: SchemaWrite<Configuration, Src = T>,
{}

#[derive(SchemaWrite, SchemaRead, Debug)]
pub struct Hash(U256);

impl Hash {
    pub fn new<T: Encode>(data: T) -> Self {
        let serialized = wincode::serialize(&data).unwrap();
        let hashed = Sha256::digest(serialized);
        Hash(U256::from_little_endian(&hashed))
    }
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
    pub fn as_bytes(&self) -> [u8; 32] {
        let bytes = self.0.to_little_endian();
        bytes
    }
    pub fn as_hex(&self) -> String {
        let bytes = self.as_bytes();
        hex::encode(bytes)
    }
    pub fn matches_target(&self, target: Hash) -> bool {
        self.0 < target.0 
    }
}
