use s_controller_lib::{
    disable_lst_input_ix_by_mint_full, enable_lst_input_ix_by_mint_full, try_find_lst_mint_on_list,
    try_lst_state_list, DisableEnableLstInputByMintFreeArgs, U8Bool,
};
use sanctum_solana_test_utils::test_fixtures_dir;
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use test_utils::jitosol;

use crate::common::*;

#[tokio::test]
async fn basic_disable_then_enable() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let program_test = jito_marinade_no_fee_program_test(
        JitoMarinadeProgramTestArgs::default().with_lp_token_mint(Pubkey::new_unique()),
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let args = DisableEnableLstInputByMintFreeArgs {
        lst_mint: jitosol::ID,
        pool_state: banks_client.get_pool_state_acc().await,
        lst_state_list: banks_client.get_lst_state_list_acc().await,
    };

    // disable jitoSOL
    let ix = disable_lst_input_ix_by_mint_full(&args).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_lst_input_disabled(&mut banks_client, jitosol::ID).await;

    // re-enable jitoSOL
    let ix = enable_lst_input_ix_by_mint_full(&args).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_lst_input_enabled(&mut banks_client, jitosol::ID).await;
}

async fn verify_lst_input_disabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_account.data).unwrap();
    let (_i, lst_state) = try_find_lst_mint_on_list(lst_mint, lst_state_list).unwrap();
    assert!(U8Bool(lst_state.is_input_disabled).is_true())
}

async fn verify_lst_input_enabled(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_account.data).unwrap();
    let (_i, lst_state) = try_find_lst_mint_on_list(lst_mint, lst_state_list).unwrap();
    assert!(U8Bool(lst_state.is_input_disabled).is_false())
}
