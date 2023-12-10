use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_interface::SControllerError;
use s_controller_lib::{
    end_rebalance_ix_full,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, REBALANCE_RECORD_ID},
    start_rebalance_ix_full, try_lst_state_list, try_pool_state,
    EndRebalanceFromStartRebalanceKeys, SrcDstLstIndexes, SrcDstLstSolValueCalcAccounts,
    StartRebalanceByMintsFreeArgs, StartRebalanceIxFullArgs, U8Bool,
};
use sanctum_utils::{mint_with_token_program::MintWithTokenProgram, token::TransferKeys};
use solana_program::{clock::Clock, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use spl_calculator_lib::SplLstSolCommonFreeArgsConst;
use test_utils::{
    assert_custom_err, banks_client_get_account, jito_stake_pool, jitosol, test_fixtures_dir,
    MockTokenAccountArgs, JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

use crate::common::*;

#[tokio::test]
async fn rebalance_basic() {
    const JITOSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const MSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const JITOSOL_WITHDRAW_AMT: u64 = 500_000_000;
    const MSOL_DONATE_AMT: u64 = 500_000_000;

    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_START_SOL_VALUE,
        msol_sol_value: MSOL_START_SOL_VALUE,
        jitosol_reserves: 1_000_000_000,
        msol_reserves: 1_000_000_000,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    });

    let withdraw_jitosol_to_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: mock_auth_kp.pubkey(),
            amount: 0,
        },
    );
    let donate_msol_from_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: msol::ID,
            authority: mock_auth_kp.pubkey(),
            amount: MSOL_DONATE_AMT,
        },
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

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
    let jito_stake_pool_acc =
        banks_client_get_account(&mut banks_client, jito_stake_pool::ID).await;

    let jito_sol_val_calc_accounts = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedReadonlyAccount {
            key: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    }
    .resolve_to_account_metas()
    .unwrap();

    let marinade_sol_val_calc_accounts = marinade_sol_val_calc_account_metas();

    let args = StartRebalanceByMintsFreeArgs {
        withdraw_to: withdraw_jitosol_to_addr,
        lst_state_list: KeyedReadonlyAccount {
            key: LST_STATE_LIST_ID,
            account: lst_state_list_acc,
        },
        pool_state: KeyedReadonlyAccount {
            key: POOL_STATE_ID,
            account: pool_state_acc,
        },
        src_lst_mint: MintWithTokenProgram {
            pubkey: jitosol::ID,
            token_program: spl_token::ID,
        },
        dst_lst_mint: MintWithTokenProgram {
            pubkey: msol::ID,
            token_program: spl_token::ID,
        },
    };
    let (
        start_rebalance_keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = args.resolve().unwrap();
    let end_rebalance_keys = EndRebalanceFromStartRebalanceKeys(&start_rebalance_keys).resolve();

    let start_rebalance_ix = start_rebalance_ix_full(
        start_rebalance_keys,
        StartRebalanceIxFullArgs {
            src_lst_index,
            dst_lst_index,
            amount: JITOSOL_WITHDRAW_AMT,
        },
        SrcDstLstSolValueCalcAccounts {
            src_lst_calculator_program_id: spl_calculator_lib::program::ID,
            dst_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            src_lst_calculator_accounts: &jito_sol_val_calc_accounts,
            dst_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
        },
    )
    .unwrap();

    let donate_msol_ix = TransferKeys {
        token_program: spl_token::ID,
        from: donate_msol_from_addr,
        to: end_rebalance_keys.dst_pool_reserves,
        authority: mock_auth_kp.pubkey(),
    }
    .to_ix(MSOL_DONATE_AMT)
    .unwrap();
    let end_rebalance_ix = end_rebalance_ix_full(
        end_rebalance_keys,
        &marinade_sol_val_calc_accounts,
        marinade_calculator_lib::program::ID,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(
        &[start_rebalance_ix, donate_msol_ix, end_rebalance_ix],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(U8Bool(pool_state.is_rebalancing).is_false());
    assert!(pool_state.total_sol_value >= JITOSOL_START_SOL_VALUE + MSOL_START_SOL_VALUE);

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    for lst_state in lst_state_list {
        if lst_state.mint == jitosol::ID {
            assert!(lst_state.sol_value < JITOSOL_START_SOL_VALUE);
        } else {
            assert!(lst_state.sol_value > MSOL_START_SOL_VALUE);
        }
    }

    let rebalance_record = banks_client.get_account(REBALANCE_RECORD_ID).await.unwrap();
    assert!(rebalance_record.is_none());
}

#[tokio::test]
async fn rebalance_fail_no_end() {
    const JITOSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const MSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const JITOSOL_WITHDRAW_AMT: u64 = 500_000_000;

    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_START_SOL_VALUE,
        msol_sol_value: MSOL_START_SOL_VALUE,
        jitosol_reserves: 1_000_000_000,
        msol_reserves: 1_000_000_000,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    });

    let withdraw_jitosol_to_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: mock_auth_kp.pubkey(),
            amount: 0,
        },
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

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
    let jito_stake_pool_acc =
        banks_client_get_account(&mut banks_client, jito_stake_pool::ID).await;

    let jito_sol_val_calc_accounts = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedReadonlyAccount {
            key: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    }
    .resolve_to_account_metas()
    .unwrap();

    let marinade_sol_val_calc_accounts = marinade_sol_val_calc_account_metas();

    let args = StartRebalanceByMintsFreeArgs {
        withdraw_to: withdraw_jitosol_to_addr,
        lst_state_list: KeyedReadonlyAccount {
            key: LST_STATE_LIST_ID,
            account: lst_state_list_acc,
        },
        pool_state: KeyedReadonlyAccount {
            key: POOL_STATE_ID,
            account: pool_state_acc,
        },
        src_lst_mint: MintWithTokenProgram {
            pubkey: jitosol::ID,
            token_program: spl_token::ID,
        },
        dst_lst_mint: MintWithTokenProgram {
            pubkey: msol::ID,
            token_program: spl_token::ID,
        },
    };
    let (
        start_rebalance_keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = args.resolve().unwrap();

    let start_rebalance_ix = start_rebalance_ix_full(
        start_rebalance_keys,
        StartRebalanceIxFullArgs {
            src_lst_index,
            dst_lst_index,
            amount: JITOSOL_WITHDRAW_AMT,
        },
        SrcDstLstSolValueCalcAccounts {
            src_lst_calculator_program_id: spl_calculator_lib::program::ID,
            dst_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            src_lst_calculator_accounts: &jito_sol_val_calc_accounts,
            dst_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[start_rebalance_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_custom_err(err, SControllerError::NoSucceedingEndRebalance);
}
