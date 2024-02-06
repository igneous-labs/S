use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address,
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, LST_STATE_LIST_ID},
    try_disable_pool_authority_list, try_find_element_in_list, try_find_lst_mint_on_list,
    try_lst_state_list, try_pool_state, FindLstPdaAtaKeys, U8Bool,
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

pub async fn assert_lst_removed(
    banks_client: &mut BanksClient,
    keys: FindLstPdaAtaKeys,
    list_len_before: usize,
) {
    assert!(list_len_before >= 1);
    if list_len_before == 1 {
        assert!(banks_client
            .get_account(LST_STATE_LIST_ID)
            .await
            .unwrap()
            .is_none());
    } else {
        let lst_state_list_acc = banks_client.get_lst_state_list_acc().await;
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        assert_eq!(lst_state_list.len(), list_len_before - 1);
        assert!(try_find_lst_mint_on_list(keys.lst_mint, lst_state_list).is_err());
    }
    assert_pool_token_accounts_deleted_for_lst(banks_client, keys).await;
}

async fn assert_pool_token_accounts_deleted_for_lst(
    banks_client: &mut BanksClient,
    keys: FindLstPdaAtaKeys,
) {
    let pool_reserves_addr = find_pool_reserves_address(keys).0;
    let protocol_fee_accumulator_addr = find_protocol_fee_accumulator_address(keys).0;
    for should_be_deleted in [pool_reserves_addr, protocol_fee_accumulator_addr] {
        assert!(banks_client
            .get_account(should_be_deleted)
            .await
            .unwrap()
            .is_none())
    }
}

pub async fn assert_pricing_prog_set(
    banks_client: &mut BanksClient,
    expected_pricing_prog: Pubkey,
) {
    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert_eq!(pool_state.pricing_program, expected_pricing_prog);
}

pub async fn assert_admin(bc: &mut BanksClient, expected_admin: Pubkey) {
    let pool_state_acc = bc.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(pool_state.admin == expected_admin);
}
