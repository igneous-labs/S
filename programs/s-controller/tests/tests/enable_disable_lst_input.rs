use s_controller_lib::{
    disable_lst_input_ix_by_mint_full, enable_lst_input_ix_by_mint_full,
    DisableEnableLstInputByMintFreeArgs,
};
use s_controller_test_utils::{
    assert_lst_input_disabled, assert_lst_input_enabled, jito_marinade_no_fee_program_test,
    JitoMarinadeProgramTestArgs, LstStateListBanksClient, PoolStateBanksClient,
};
use sanctum_solana_test_utils::test_fixtures_dir;
use solana_program::pubkey::Pubkey;
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
    )
    .add_s_program();
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

    assert_lst_input_disabled(&mut banks_client, jitosol::ID).await;

    // re-enable jitoSOL
    let ix = enable_lst_input_ix_by_mint_full(&args).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    assert_lst_input_enabled(&mut banks_client, jitosol::ID).await;
}
