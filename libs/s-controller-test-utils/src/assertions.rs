use s_controller_lib::U8Bool;
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;

use crate::LstStateListBanksClient;

pub async fn assert_lst_input_disabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state = banks_client.get_lst_state(lst_mint).await;
    assert!(U8Bool(lst_state.is_input_disabled).is_true())
}

pub async fn assert_lst_input_enabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state = banks_client.get_lst_state(lst_mint).await;
    assert!(U8Bool(lst_state.is_input_disabled).is_false())
}
