use s_controller_interface::{LstState, PoolState, RebalanceRecord};
use static_assertions::const_assert_eq;

mod accounts_resolvers;
mod accounts_serde;
mod consts;
mod instructions;
mod lp_token_metadata;
mod lst_indexes;
mod lst_state_list;
mod pda;
mod state;
mod u8bool;

pub use accounts_resolvers::*;
pub use accounts_serde::*;
pub use consts::*;
pub use instructions::*;
pub use lp_token_metadata::*;
pub use lst_indexes::*;
pub use lst_state_list::*;
pub use pda::*;
pub use state::*;
pub use u8bool::*;

// std::mem::size_of and std::mem::align_of are const fns so we dont technically need these
// but the const asserts helps guard against unexpected size changes

pub const POOL_STATE_SIZE: usize = 176;
const_assert_eq!(std::mem::size_of::<PoolState>(), POOL_STATE_SIZE);
pub const POOL_STATE_ALIGN: usize = 8;
const_assert_eq!(std::mem::align_of::<PoolState>(), POOL_STATE_ALIGN);

pub const LST_STATE_SIZE: usize = 80;
const_assert_eq!(std::mem::size_of::<LstState>(), LST_STATE_SIZE);
pub const LST_STATE_ALIGN: usize = 8;
const_assert_eq!(std::mem::align_of::<LstState>(), LST_STATE_ALIGN);

pub const REBALANCE_RECORD_SIZE: usize = 16;
const_assert_eq!(
    std::mem::size_of::<RebalanceRecord>(),
    REBALANCE_RECORD_SIZE
);
pub const REBALANCE_RECORD_ALIGN: usize = 8;
const_assert_eq!(
    std::mem::align_of::<RebalanceRecord>(),
    REBALANCE_RECORD_ALIGN
);

pub mod program {
    sanctum_macros::declare_program_keys!(
        "scoeWYRwSor53KxfQ8EkNCkka1vasF8td3P3nfHQvsv",
        [
            ("pool-state", b"state"),
            ("lst-state-list", b"lst-state-list"),
            (
                "disable-pool-authority-list",
                b"disable-pool-authority-list"
            ),
            ("rebalance-record", b"rebalance-record"),
            ("protocol-fee", b"protocol-fee"),
        ]
    );
}
