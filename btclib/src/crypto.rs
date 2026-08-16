use k256::{ecdsa::{Signature, SigningKey, VerifyingKey, Error as EcdsaError, signature::{Signer, Verifier}}};
use k256::elliptic_curve::Generate;

#[derive(Debug)]
struct PrivateKey (
        SigningKey
    );

impl PrivateKey {
    fn new() -> Self {
       PrivateKey(
           SigningKey::generate()
           ) 
    }
    fn public_key(&self) -> PublicKey {
       PublicKey(
           *self.0.verifying_key()
           ) 
    }
    fn sign_data(&self, data: &[u8]) -> Signature {
        self.0.sign(data)
    }
}

#[derive(Debug)]
struct PublicKey(
    VerifyingKey
    );

impl PublicKey {
    fn verify(&self, data: &[u8], signature: Signature) -> Result<(), EcdsaError> {
        self.0.verify(data, &signature)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let priv_key = PrivateKey::new();
        let pub_key = priv_key.public_key();
        let data = String::from("Kobe");
        let datta = String::from("kobe");
        let signature = priv_key.sign_data(data.as_bytes());
        let result = pub_key.verify(data.as_bytes(), signature);
        println!("signature: {:?}", result);
    }
}
