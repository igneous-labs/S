use marinade_keys::msol;
use s_controller_interface::{withdraw_protocol_fees_ix, WithdrawProtocolFeesIxArgs};
use s_controller_lib::{
    find_protocol_fee_accumulator_address, program::POOL_STATE_ID, FindLstPdaAtaKeys,
    WithdrawProtocolFeesFreeArgs,
};
use sanctum_utils::token::{token_account_balance, token_account_mint};
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use test_utils::{
    banks_client_get_account, mock_token_account, test_fixtures_dir, MockTokenAccountArgs,
};

use crate::common::*;

#[tokio::test]
async fn basic_withdraw_protocol_fees() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    const MSOL_DEFAULT_AMOUNT: u64 = 0;
    const MSOL_FEES_TO_WITHDRAW: u64 = 1_000_000_000;
    const MSOL_ACCUMULATED_FEES: u64 = 10_000_000_000;

    let auth_msol_acc_addr = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: MSOL_ACCUMULATED_FEES,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    });
    program_test.add_account(
        auth_msol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: msol::ID,
            authority: mock_auth_kp.pubkey(),
            amount: MSOL_DEFAULT_AMOUNT,
        }),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;

    let msol_account = banks_client_get_account(&mut banks_client, auth_msol_acc_addr).await;

    // Withdraw protocol fees
    let ix = withdraw_protocol_fees_ix(
        WithdrawProtocolFeesFreeArgs {
            pool_state: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_acc.clone(),
            },
            withdraw_to: KeyedAccount {
                pubkey: auth_msol_acc_addr,
                account: msol_account.clone(),
            },
        }
        .resolve()
        .unwrap(),
        WithdrawProtocolFeesIxArgs {
            amount: MSOL_FEES_TO_WITHDRAW,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let lst_mint = token_account_mint(&msol_account).unwrap();
    let find_pda_keys = FindLstPdaAtaKeys {
        lst_mint,
        token_program: msol_account.owner,
    };

    let (protocol_fee_accumulator, _protocol_fee_accumulator_bump) =
        find_protocol_fee_accumulator_address(find_pda_keys);

    let protocol_fee_accumulator_acc =
        banks_client_get_account(&mut banks_client, protocol_fee_accumulator).await;
    let protocol_fee_accumulator_balance =
        token_account_balance(protocol_fee_accumulator_acc).unwrap();

    banks_client.process_transaction(tx).await.unwrap();

    let msol_account = banks_client_get_account(&mut banks_client, auth_msol_acc_addr).await;
    assert_eq!(
        token_account_balance(msol_account).unwrap(),
        MSOL_DEFAULT_AMOUNT + MSOL_FEES_TO_WITHDRAW
    );

    let new_protocol_fee_accumulator_acc = banks_client
        .get_account(protocol_fee_accumulator)
        .await
        .unwrap()
        .unwrap();
    let new_protocol_fee_accumulator_balance =
        token_account_balance(new_protocol_fee_accumulator_acc).unwrap();
    assert_eq!(
        protocol_fee_accumulator_balance,
        new_protocol_fee_accumulator_balance + MSOL_FEES_TO_WITHDRAW
    );
}
