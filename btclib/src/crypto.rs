use k256::{ecdsa::{Signature, SigningKey, VerifyingKey, Error as EcdsaError, signature::{Signer, Verifier}}};
use k256::elliptic_curve::Generate;
use wincode::{SchemaRead, SchemaWrite, TypeMeta};
use wincode::config::ConfigCore;
use wincode::WriteResult;

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct PrivateKey (
        #[wincode(with = "PrivKeySerializer")]
        SigningKey
    );

impl PrivateKey {
    pub fn new() -> Self {
       PrivateKey(
           SigningKey::generate()
           ) 
    }
    pub fn public_key(&self) -> PublicKey {
       PublicKey(
           *self.0.verifying_key()
           ) 
    }
    pub fn sign_data(&self, data: &[u8]) -> Signature {
        self.0.sign(data)
    }
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy)]
pub struct PublicKey(
        #[wincode(with = "PubKeySerializer")]
        VerifyingKey
    );

impl PublicKey {
    pub fn verify(&self, data: &[u8], signature: Signature) -> Result<(), EcdsaError> {
        self.0.verify(data, &signature)
    }
}

struct PrivKeySerializer;

unsafe impl<C: ConfigCore> SchemaWrite<C> for PrivKeySerializer {
    type Src = SigningKey;

    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn size_of(src: &Self::Src) -> WriteResult<usize> {
       Ok(src.to_bytes().len()) 
    }
    fn write(mut writer: impl wincode::io::Writer, src: &Self::Src) -> WriteResult<()> {
        writer.write(&src.to_bytes())?;
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for PrivKeySerializer {
   type Dst = SigningKey;
    
    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn read(reader: impl wincode::io::Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
        let bytes = <[u8; 32] as SchemaRead<'de, C>>::get(reader)?;
        let priv_key = SigningKey::from_slice(&bytes).map_err(|_| wincode::error::ReadError::Custom("invalid public key bytes"))?;
        dst.write(priv_key);
        Ok(())
    }
}



struct PubKeySerializer;

unsafe impl<C: ConfigCore> SchemaWrite<C> for PubKeySerializer {
    type Src = VerifyingKey;

    const TYPE_META: TypeMeta = TypeMeta::Dynamic;
    
    fn size_of(src: &Self::Src) -> WriteResult<usize> {
        Ok(src.to_sec1_bytes().len())
    }

    fn write(mut writer: impl wincode::io::Writer, src: &Self::Src) -> WriteResult<()> {
        writer.write(&src.to_sec1_bytes())?;
        Ok(())
    }

}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for PubKeySerializer {
   type Dst = VerifyingKey;
    
    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn read(reader: impl wincode::io::Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
        let bytes= <[u8; 33] as SchemaRead<'de, C>>::get(reader)?;
        let pub_key = VerifyingKey::from_sec1_bytes(&bytes).map_err(|_| wincode::error::ReadError::Custom("invalid public key bytes"))?;
        dst.write(pub_key);
        Ok(())
    }
}
