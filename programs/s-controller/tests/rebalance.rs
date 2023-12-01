use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
use marinade_calculator_lib::{MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS};
use marinade_keys::msol;
use s_controller_lib::{
    end_rebalance_ix_full,
    program::{LST_STATE_LIST_ID, REBALANCE_RECORD_ID, STATE_ID},
    start_rebalance_ix_full, try_lst_state_list, try_pool_state,
    EndRebalanceFromStartRebalanceKeys, SrcDstLstIndexes, SrcDstLstSolValueCalcAccounts,
    StartRebalanceByMintsFreeArgs, StartRebalanceIxArgsFull, U8Bool,
};
use sanctum_utils::token::TransferKeys;
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{
    jito_stake_pool, jitosol, mock_token_account, MockTokenAccountArgs,
    JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

mod common;

use crate::common::*;

#[tokio::test]
async fn basic() {
    const JITOSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const MSOL_START_SOL_VALUE: u64 = 1_000_000_000;
    const JITOSOL_WITHDRAW_AMT: u64 = 500_000_000;
    const MSOL_DONATE_AMT: u64 = 500_000_000;

    let donor_token_auth = Keypair::new();

    let mut program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_START_SOL_VALUE,
        msol_sol_value: MSOL_START_SOL_VALUE,
        jitosol_reserves: 1_000_000_000,
        msol_reserves: 1_000_000_000,
    });

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.rebalance_authority = donor_token_auth.pubkey();
    pool_state.total_sol_value = JITOSOL_START_SOL_VALUE + MSOL_START_SOL_VALUE;
    program_test.add_account(STATE_ID, pool_state_to_account(pool_state));

    let withdraw_jitosol_to = mock_token_account(MockTokenAccountArgs {
        mint: jitosol::ID,
        authority: donor_token_auth.pubkey(),
        amount: 0,
    });
    let withdraw_jitosol_to_addr = Pubkey::new_unique();
    let donate_msol_from = mock_token_account(MockTokenAccountArgs {
        mint: msol::ID,
        authority: donor_token_auth.pubkey(),
        amount: 500_000_000,
    });
    let donate_msol_from_addr = Pubkey::new_unique();

    program_test.add_account(withdraw_jitosol_to_addr, withdraw_jitosol_to);
    program_test.add_account(donate_msol_from_addr, donate_msol_from);

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
    let jitosol_mint_acc = banks_client_get_account(&mut banks_client, jitosol::ID).await;
    let msol_mint_acc = banks_client_get_account(&mut banks_client, msol::ID).await;
    let jito_stake_pool_acc =
        banks_client_get_account(&mut banks_client, jito_stake_pool::ID).await;

    let jito_sol_val_calc_args = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedReadonlyAccount {
            key: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    };
    let jito_sol_val_calc_keys: generic_pool_calculator_interface::LstToSolKeys =
        jito_sol_val_calc_args
            .resolve()
            .unwrap()
            .resolve::<SplSolValCalc>()
            .into();
    let jito_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        (&jito_sol_val_calc_keys).into();

    let marinade_sol_val_calc_keys: generic_pool_calculator_interface::LstToSolKeys =
        MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();
    let marinade_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        (&marinade_sol_val_calc_keys).into();

    let args = StartRebalanceByMintsFreeArgs {
        payer: payer.pubkey(),
        withdraw_to: withdraw_jitosol_to_addr,
        lst_state_list: KeyedReadonlyAccount {
            key: LST_STATE_LIST_ID,
            account: lst_state_list_acc,
        },
        pool_state: KeyedReadonlyAccount {
            key: STATE_ID,
            account: pool_state_acc,
        },
        src_lst_mint: KeyedReadonlyAccount {
            key: jitosol::ID,
            account: jitosol_mint_acc,
        },
        dst_lst_mint: KeyedReadonlyAccount {
            key: msol::ID,
            account: msol_mint_acc,
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
        StartRebalanceIxArgsFull {
            src_lst_index: src_lst_index.try_into().unwrap(),
            dst_lst_index: dst_lst_index.try_into().unwrap(),
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
        authority: donor_token_auth.pubkey(),
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
    tx.sign(&[&payer, &donor_token_auth], last_blockhash);

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
