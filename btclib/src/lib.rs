pub mod types;
pub mod sha256;
pub mod crypto;
pub mod network;
pub mod error;
pub mod traits;

use uint::construct_uint;
use wincode::{SchemaRead, SchemaWrite};

construct_uint! {
    #[derive(SchemaWrite, SchemaRead)]
    pub struct U256(4);
}


pub const VERSION: u32 = 1;
pub const BLOCK_REWARD: u64 = 50;
pub const TARGET: U256 = U256([5, 4000000, 9999999999999999, 999999999999999999]);
pub const MINING_FEE: u64 = 50;

fn bits_to_target(nbits: u32) -> U256 {
    let exp = nbits >> (3 * 8);
    let mantissa = nbits & 0x00FF_FFFF;
    U256::from(mantissa) << (exp - 3) * 8
}

fn target_to_bits(target: U256) -> u32 {
    let unsig_bytes = target.leading_zeros() / 8;
    let exp = 32 - unsig_bytes;
    let mantissa = (target >> (exp - 3) * 8).low_u32();
    (exp << (3 * 8)) | mantissa
}
