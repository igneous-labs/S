use s_controller_interface::PoolState;
use s_controller_lib::{
    initial_authority, initial_token_metadata_size, program::POOL_STATE_ID, try_pool_state_mut,
    DEFAULT_LP_TOKEN_METADATA_NAME, DEFAULT_LP_TOKEN_METADATA_SYMBOL,
    DEFAULT_LP_TOKEN_METADATA_URI, DEFAULT_PRICING_PROGRAM, POOL_STATE_SIZE,
};
use solana_program::{program_option::COption, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_2022::extension::{
    metadata_pointer::MetadataPointer, transfer_fee::TransferFeeConfig, ExtensionType,
    StateWithExtensionsMut,
};
use spl_token_metadata_interface::state::TokenMetadata;
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

pub fn mock_lp_mint(mint_addr: Pubkey, supply: u64) -> Account {
    let account_size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&[
        ExtensionType::TransferFeeConfig,
        ExtensionType::MetadataPointer,
    ])
    .unwrap()
        + initial_token_metadata_size().unwrap();
    let mut data = vec![0; account_size];
    let mut state =
        StateWithExtensionsMut::<spl_token_2022::state::Mint>::unpack_uninitialized(&mut data)
            .unwrap();
    // dont care abt transfer fee config all zeros for now
    state.init_extension::<TransferFeeConfig>(true).unwrap();
    let metadata_ptr = state.init_extension::<MetadataPointer>(true).unwrap();
    metadata_ptr.metadata_address = OptionalNonZeroPubkey::try_from(Some(mint_addr)).unwrap();
    state.base = spl_token_2022::state::Mint {
        mint_authority: COption::Some(POOL_STATE_ID),
        supply,
        decimals: 9,
        is_initialized: true,
        freeze_authority: COption::Some(POOL_STATE_ID),
    };
    state.pack_base();
    state.init_account_type();
    let token_metadata = TokenMetadata {
        name: DEFAULT_LP_TOKEN_METADATA_NAME.into(),
        symbol: DEFAULT_LP_TOKEN_METADATA_SYMBOL.into(),
        uri: DEFAULT_LP_TOKEN_METADATA_URI.into(),
        update_authority: Some(initial_authority::ID).try_into().unwrap(),
        mint: mint_addr,
        ..Default::default()
    };
    state
        .init_variable_len_extension(&token_metadata, true)
        .unwrap();
    Account {
        lamports: est_rent_exempt_lamports(account_size),
        data,
        owner: spl_token_2022::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
