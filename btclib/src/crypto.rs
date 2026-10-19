use std::fmt::{Display, Debug};
use k256::{ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey, Error as EcdsaError, signature::{Signer, Verifier}}};
use k256::elliptic_curve::Generate;
use wincode::{SchemaRead, SchemaWrite, TypeMeta, WriteResult, ReadResult};
use wincode::config::ConfigCore;
use wincode::io::{Reader, Writer};

use crate::traits::Saveable;


#[derive(SchemaWrite, SchemaRead, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature(
        #[wincode(with = "SignatureSerializer")]
        EcdsaSignature
    );

struct SignatureSerializer;

unsafe impl<C: ConfigCore> SchemaWrite<C> for SignatureSerializer {
    type Src = EcdsaSignature;

    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn size_of(src: &Self::Src) -> WriteResult<usize> {
       Ok(src.to_bytes().len()) 
    }
    fn write(mut writer: impl wincode::io::Writer, src: &Self::Src) -> WriteResult<()> {
        writer.write(&src.to_bytes())?;
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for SignatureSerializer {
   type Dst = EcdsaSignature;
    
    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn read(reader: impl Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> ReadResult<()> {
        let bytes = <[u8; 64] as SchemaRead<'de, C>>::get(reader)?;
        let signature = EcdsaSignature::from_slice(&bytes).map_err(|_| wincode::error::ReadError::Custom("invalid public key bytes"))?;
        dst.write(signature);
        Ok(())
    }
}

#[derive(SchemaWrite, SchemaRead, Clone, PartialEq, Eq)]
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
        Signature(self.0.sign(data))
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0.to_bytes()))
    }
}

impl Debug for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0.to_bytes()))
    }
}

#[derive(SchemaWrite, SchemaRead, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(
        #[wincode(with = "PubKeySerializer")]
        VerifyingKey
    );

impl PublicKey {
    pub fn verify(&self, data: &[u8], signature: Signature) -> Result<(), EcdsaError> {
        self.0.verify(data, &signature.0)
    }
}

impl Saveable for PublicKey {}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0.to_sec1_bytes()))
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0.to_sec1_bytes()))
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

    fn read(reader: impl Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> ReadResult<()> {
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

    fn write(mut writer: impl Writer, src: &Self::Src) -> WriteResult<()> {
        writer.write(&src.to_sec1_bytes())?;
        Ok(())
    }

}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for PubKeySerializer {
   type Dst = VerifyingKey;
    
    const TYPE_META: TypeMeta = TypeMeta::Dynamic;

    fn read(reader: impl Reader<'de>, dst: &mut std::mem::MaybeUninit<Self::Dst>) -> ReadResult<()> {
        let bytes= <[u8; 33] as SchemaRead<'de, C>>::get(reader)?;
        let pub_key = VerifyingKey::from_sec1_bytes(&bytes).map_err(|_| wincode::error::ReadError::Custom("invalid public key bytes"))?;
        dst.write(pub_key);
        Ok(())
    }
}
