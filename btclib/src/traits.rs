use std::path::Path;
use std::fs::File;
use std::io::{Read, Write, Result as IoResult};
use wincode::{SchemaRead, SchemaWrite, config::Configuration};
use crate::crypto::{PublicKey, PrivateKey};

// Encode trait for wincode encodable data
pub trait Encode: SchemaWrite<Configuration, Src = Self> {}
impl<T: SchemaWrite<Configuration, Src = T>> Encode for T {}

// Decode trait for wincode decodable data
pub trait Decode<'de>: SchemaRead<'de, Configuration, Dst = Self> {}
impl<'de, T: SchemaRead<'de,  Configuration, Dst = T>> Decode<'de> for T {}

// Saveable trait for saving wincode serializeable data
pub trait Saveable {
    fn save_to_file(&self, file_name: &str) -> IoResult<()> 
    where Self: Encode
    {
        let bytes = wincode::serialize(self).unwrap();
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/{file_name}", cargo_path.display());
        let mut f = File::create(path)?;
        f.write_all(&bytes)?;
        
        Ok(())
    }

    fn load_from_file(file_name: &str) -> IoResult<Self> 
    where Self: for<'de> Decode<'de> + Sized
    {
        let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = format!("{}/{file_name}", cargo_path.display());
        let mut f = File::open(path)?;
        
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let source = wincode::deserialize(&buf).unwrap();

        Ok(source)
    }
}

pub trait KeyStore {
    fn get_value(&self, key: PublicKey) -> Option<PrivateKey>;
}
