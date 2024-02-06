use s_controller_lib::{try_pool_state, U8Bool};
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;

use crate::{LstStateListBanksClient, PoolStateBanksClient};

pub async fn assert_lst_input_disabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state = banks_client.get_lst_state(lst_mint).await;
    assert!(U8Bool(lst_state.is_input_disabled).is_true())
}

pub async fn assert_lst_input_enabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state = banks_client.get_lst_state(lst_mint).await;
    assert!(U8Bool(lst_state.is_input_disabled).is_false())
}

pub async fn assert_pool_disabled(banks_client: &mut BanksClient) {
    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(U8Bool(pool_state.is_disabled).is_true())
}

pub async fn assert_pool_enabled(banks_client: &mut BanksClient) {
    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(U8Bool(pool_state.is_disabled).is_false())
}
