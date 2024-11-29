//! Reimplementation of stuff in stakedex-sdk.
//! Required due to:
//! - change in required ix format for rebal-stake
//! - allow quoting for pools with deposit auths

mod deposit_sol;
mod deposit_stake;

pub use deposit_sol::*;
pub use deposit_stake::*;
