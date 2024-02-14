//! Well-known constants of test accounts etc

pub const SANCTUM_SPL_STAKE_POOL_PROG_LAST_UDPATED_SLOT: u64 = 241_131_019;

pub const SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT: u64 = 178_976_956;

pub const JITO_STAKE_POOL_LAST_UPDATE_EPOCH: u64 = 539;

pub const PWR_STAKE_POOL_LAST_UPDATE_EPOCH: u64 = 573;

pub const MARINADE_PROG_LAST_UPDATED_SLOT: u64 = 229_946_024;

pub const LIDO_PROG_LAST_UPDATED_SLOT: u64 = 165_468_732;

pub const LIDO_STATE_LAST_UPDATE_EPOCH: u64 = 543;

pub mod jito_stake_pool {
    sanctum_macros::declare_program_keys!("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb", []);
}

pub mod jitosol {
    sanctum_macros::declare_program_keys!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn", []);
}
