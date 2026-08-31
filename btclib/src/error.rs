use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid Transaction")]
    InvalidTransaction,
}
