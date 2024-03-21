use solana_program::pubkey::Pubkey;

pub mod initial_authority {
    #[cfg(feature = "testing")]
    sanctum_macros::declare_program_keys!("9S3avfRxH9RYbMHbvxnhwiwpdF9iuXG7uWiatqWvQskT", []);

    #[cfg(not(feature = "testing"))]
    sanctum_macros::declare_program_keys!("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1", []);
}

pub const CURRENT_PROGRAM_VERS: u8 = 1;

/// 10% of trading fees
pub const DEFAULT_TRADING_PROTOCOL_FEE_BPS: u16 = 1_000;

/// 10% of LP withdrawal fees
pub const DEFAULT_LP_PROTOCOL_FEE_BPS: u16 = 1_000;

pub const DEFAULT_PRICING_PROGRAM: Pubkey = flat_fee_lib::program::ID;

// Compute Unit ceilings for instructions
pub const ADD_LST_IX_COMPUTE_UNIT_CEIL: u32 = 100_000;
