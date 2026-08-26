mod types;
mod sha256;
mod crypto;
mod network;

use uint::construct_uint;
use wincode::{SchemaRead, SchemaWrite};

construct_uint! {
    #[derive(SchemaWrite, SchemaRead)]
    struct U256(4);
}


pub const VERSION: u32 = 1;

pub const TARGET: u32 = 0x1d00ffff;
