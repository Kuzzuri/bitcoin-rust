use std::fmt::{Debug, Display, Formatter, Result as FormatResult};
use crate::U256;
use sha2::{Sha256, Digest};
use wincode::{SchemaRead, SchemaWrite};
use crate::traits::{Encode, Decode};

#[derive(SchemaWrite, SchemaRead, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash(pub U256);

impl Hash {
    pub fn hash<T: Encode>(data: T) -> Self {
        let serialized = wincode::serialize(&data).unwrap();
        let hashed = Sha256::digest(serialized);
        Hash(U256::from_little_endian(&hashed))
    }
    pub fn hash256<T: Encode>(data: T) -> Self {
        let serialized = wincode::serialize(&data).unwrap();
        let hashed = Sha256::digest(Sha256::digest(serialized));
        Hash(U256::from_little_endian(&hashed))
    }
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_little_endian()
    }
    pub fn as_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }
    pub fn matches_target(&self, target: Hash) -> bool {
        self.0 <= target.0 
    }
    pub fn hex_str_to_hash(hex_str: &str) -> Hash {
        let dec = U256::from_str_radix(hex_str, 16).unwrap();
        Hash(dec)
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:x}", self.0) 
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:x}", self.0) 
    }
}
