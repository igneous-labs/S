use s_controller_interface::PoolState;
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state_mut, DEFAULT_PRICING_PROGRAM,
    POOL_STATE_SIZE,
};
use solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use spl_token::state::Mint;
use test_utils::{banks_client_get_account, est_rent_exempt_lamports};

#[derive(Clone, Copy, Debug, Default)]
pub struct MockProtocolFeeBps {
    pub trading: u16,
    pub lp: u16,
}

pub const DEFAULT_POOL_STATE: PoolState = PoolState {
    total_sol_value: 0,
    trading_protocol_fee_bps: 0,
    lp_protocol_fee_bps: 0,
    version: 0,
    is_disabled: 0,
    is_rebalancing: 0,
    padding: [0u8; 1],
    admin: initial_authority::ID,
    rebalance_authority: initial_authority::ID,
    protocol_fee_beneficiary: initial_authority::ID,
    pricing_program: DEFAULT_PRICING_PROGRAM,
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
    banks_client_get_account(banks_client, s_controller_lib::program::POOL_STATE_ID).await
}

fn mock_lp_token_mint_base(authority: Pubkey, supply: u64) -> Account {
    let mut data = vec![0; Mint::LEN];
    Mint::pack(
        Mint {
            mint_authority: COption::Some(authority),
            supply,
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::Some(authority),
        },
        &mut data,
    )
    .unwrap();
    Account {
        lamports: est_rent_exempt_lamports(Mint::LEN),
        data,
        owner: spl_token::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub fn mock_lp_mint_to_init(initial_authority: Pubkey) -> Account {
    mock_lp_token_mint_base(initial_authority, 0)
}

pub fn mock_lp_mint(supply: u64) -> Account {
    mock_lp_token_mint_base(POOL_STATE_ID, supply)
}
