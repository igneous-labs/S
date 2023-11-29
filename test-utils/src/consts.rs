//! Well-known constants of test accounts etc

pub const SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT: u64 = 178_976_956;

pub const JITO_STAKE_POOL_LAST_UPDATE_EPOCH: u64 = 539;

pub const MARINADE_PROG_LAST_UPDATED_SLOT: u64 = 229_946_024;

pub const TOKEN_ACC_RENT_EXEMPT_LAMPORTS: u64 = 2039280;

pub const ZERO_SIZE_RENT_EXEMPT_LAMPORTS: u64 = 890880;

pub const RENT_EXEMPT_LAMPORT_PER_BYTE: u64 = 6960;

pub mod jito_stake_pool {
    sanctum_macros::declare_program_keys!("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb", []);
}

pub mod jitosol {
    sanctum_macros::declare_program_keys!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn", []);
}

pub const fn est_rent_exempt_lamports(account_data_len: usize) -> u64 {
    ZERO_SIZE_RENT_EXEMPT_LAMPORTS + account_data_len as u64 * RENT_EXEMPT_LAMPORT_PER_BYTE
}
