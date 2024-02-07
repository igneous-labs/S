use s_controller_lib::{
    program::DISABLE_POOL_AUTHORITY_LIST_ID, try_disable_pool_authority_list,
    try_find_element_in_list, try_pool_state, U8Bool,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;

use crate::{DisablePoolAuthorityListBanksClient, LstStateListBanksClient, PoolStateBanksClient};

// TODO: _for_prog()

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

pub async fn assert_disable_authority_removed(
    banks_client: &mut BanksClient,
    target_authority: Pubkey,
    list_len_before: usize,
) {
    assert!(list_len_before >= 1);
    if list_len_before == 1 {
        assert!(banks_client
            .get_account(DISABLE_POOL_AUTHORITY_LIST_ID)
            .await
            .unwrap()
            .is_none());
        return;
    }

    let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
    let disable_pool_authority_list =
        try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
    assert_eq!(disable_pool_authority_list.len(), list_len_before - 1);
    assert!(try_find_element_in_list(target_authority, disable_pool_authority_list).is_none());
}
