use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid Transaction")]
    InvalidTransaction,
    #[error("Invalid Block")]
    InvalidBlock,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error("Invalid Message")]
    InvalidMessage,
    #[error("Double Spend")]
    DoubleSpend
}
