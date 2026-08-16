mod types;
mod hash;
mod crypto;

use uint::construct_uint;
use wincode::{SchemaRead, SchemaWrite};

construct_uint! {
    #[derive(SchemaWrite, SchemaRead)]
    struct U256(4);
}
