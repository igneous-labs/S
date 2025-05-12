#![allow(non_local_definitions)]

solana_program::declare_id!("f1tUoNEKrDp1oeGn4zxr7bh41eN6VcfHjfrL3ZqQday");
pub mod typedefs;
pub use typedefs::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
