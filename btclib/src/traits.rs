use std::path::Path;
use std::fs::File;
use crate::sha256::{Decode, Encode};
use std::io::{Read, Write, Result as IoResult};

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
        let wallet = wincode::deserialize(&buf).unwrap();

        Ok(wallet)
    }
}
