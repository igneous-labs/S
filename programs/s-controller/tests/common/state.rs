use s_controller_interface::PoolState;
use s_controller_lib::{try_pool_state_mut, POOL_STATE_SIZE};
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use test_utils::est_rent_exempt_lamports;

use super::banks_client_get_account;

pub const DEFAULT_POOL_STATE: PoolState = PoolState {
    total_sol_value: 0,
    trading_protocol_fee_bps: 0,
    lp_protocol_fee_bps: 0,
    version: 0,
    is_disabled: 0,
    is_rebalancing: 0,
    padding: [0u8; 1],
    admin: Pubkey::new_from_array([0u8; 32]),
    rebalance_authority: Pubkey::new_from_array([0u8; 32]),
    protocol_fee_beneficiary: Pubkey::new_from_array([0u8; 32]),
    pricing_program: Pubkey::new_from_array([0u8; 32]),
    lp_token_mint: Pubkey::new_from_array([0u8; 32]),
};

pub fn pool_state_to_account(pool_state: PoolState) -> Account {
    let mut data = vec![0u8; POOL_STATE_SIZE];
    let dst = try_pool_state_mut(&mut data).unwrap();
    *dst = pool_state;
    Account {
        lamports: est_rent_exempt_lamports(POOL_STATE_SIZE),
        data,
        owner: s_controller_lib::program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub async fn banks_client_get_pool_state_acc(banks_client: &mut BanksClient) -> Account {
    banks_client_get_account(banks_client, s_controller_lib::program::STATE_ID).await
}
