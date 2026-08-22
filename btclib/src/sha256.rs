use crate::U256;
use sha2::{Sha256, Digest};
use wincode::{SchemaRead, SchemaWrite};
use wincode::config::Configuration;

// Encode trait for wincode encodable data
pub trait Encode: SchemaWrite<Configuration, Src = Self> {}
impl<T: SchemaWrite<Configuration, Src = T>> Encode for T {}

// Decode trait for wincode decodable data
pub trait Decode<'de>: SchemaRead<'de, Configuration> {}
impl<'de, T: SchemaRead<'de, Configuration>> Decode<'de> for T {}

#[derive(SchemaWrite, SchemaRead, Debug)]
pub struct Hash(U256);

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
        let bytes = self.as_bytes();
        hex::encode(bytes)
    }
    pub fn matches_target(&self, target: Hash) -> bool {
        self.0 < target.0 
    }
}
