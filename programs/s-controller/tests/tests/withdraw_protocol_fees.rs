use marinade_keys::msol;
use s_controller_interface::{withdraw_protocol_fees_ix, WithdrawProtocolFeesIxArgs};
use s_controller_lib::{
    find_protocol_fee_accumulator_address, program::POOL_STATE_ID, FindLstPdaAtaKeys,
    WithdrawProtocolFeesFreeArgs,
};

use sanctum_utils::{
    mint_with_token_program::MintWithTokenProgram,
    token::{token_account_balance, token_account_balance_program_agnostic},
};
use solana_program::{clock::Clock, pubkey::Pubkey};
use solana_program_test::*;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use test_utils::{
    banks_client_get_account, mock_token_account, test_fixtures_dir, MockTokenAccountArgs,
    JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

use crate::common::*;

#[tokio::test]
async fn basic_withdraw_protocol_fees() {
    // let mock_auth_kp =
    //     read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
    //         .unwrap();
    const MSOL_DEFAULT_AMOUNT: u64 = 0;
    const MSOL_FEES_TO_WITHDRAW: u64 = 1_000_000_000;
    const MSOL_ACCUMULATED_FEES: u64 = 10_000_000_000;

    let withdrawer = Keypair::new();
    let withdrawer_msol_acc_addr = Pubkey::new_unique();

    let mut program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
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
        withdrawer_msol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: msol::ID,
            authority: withdrawer.pubkey(),
            amount: MSOL_DEFAULT_AMOUNT,
        }),
    );

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;

    let msol_account = banks_client_get_account(&mut banks_client, withdrawer_msol_acc_addr).await;

    // Withdraw protocol fees
    let ix = withdraw_protocol_fees_ix(
        WithdrawProtocolFeesFreeArgs {
            pool_state: KeyedReadonlyAccount {
                key: POOL_STATE_ID,
                account: pool_state_acc.clone(),
            },
            token_program: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
            withdraw_to: KeyedReadonlyAccount {
                key: withdrawer_msol_acc_addr,
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
    tx.sign(&[&payer, &withdrawer], last_blockhash);

    let find_pda_keys = FindLstPdaAtaKeys {
        lst_mint: msol::ID,
        token_program: spl_token::ID,
    };

    let (protocol_fee_accumulator, _protocol_fee_accumulator_bump) =
        find_protocol_fee_accumulator_address(find_pda_keys);

    let protocol_fee_accumulator_acc = banks_client
        .get_account(protocol_fee_accumulator)
        .await
        .unwrap()
        .unwrap();
    let protocol_fee_accumulator_balance =
        token_account_balance_program_agnostic(protocol_fee_accumulator_acc).unwrap();

    banks_client.process_transaction(tx).await.unwrap();

    let msol_account = banks_client_get_account(&mut banks_client, withdrawer_msol_acc_addr).await;
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
        token_account_balance_program_agnostic(new_protocol_fee_accumulator_acc).unwrap();
    assert_eq!(
        protocol_fee_accumulator_balance,
        new_protocol_fee_accumulator_balance + MSOL_FEES_TO_WITHDRAW
    );
}
