use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::{
    sync_sol_value_ix_by_mint_full, try_lst_state_list, try_pool_state, SyncSolValueByMintFreeArgs,
};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{
    banks_client_get_account, jito_stake_pool, jitosol, JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

use crate::common::*;

#[tokio::test]
async fn basic() {
    const EXPECTED_NEW_JITOSOL_SOL_VALUE: u64 = 1_072_326_756;
    const EXPECTED_NEW_TOTAL_SOL_VALUE: u64 = 2_072_326_756;

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 1_000_000_000,
        msol_sol_value: 1_000_000_000,
        jitosol_reserves: 1_000_000_000,
        msol_reserves: 1_000_000_000,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    });
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
    let jitosol_mint_acc = banks_client_get_account(&mut banks_client, jitosol::ID).await;

    let free_args = SyncSolValueByMintFreeArgs {
        lst_state_list: lst_state_list_acc,
        lst_mint: KeyedReadonlyAccount {
            key: jitosol::ID,
            account: jitosol_mint_acc,
        },
    };

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

    let ix = sync_sol_value_ix_by_mint_full(
        free_args,
        &jito_sol_val_calc_accounts,
        spl_calculator_lib::program::ID,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    let LstState { sol_value, .. } = lst_state_list
        .iter()
        .find(|s| s.mint == jitosol::ID)
        .unwrap();
    assert_eq!(*sol_value, EXPECTED_NEW_JITOSOL_SOL_VALUE);

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
    let PoolState {
        total_sol_value, ..
    } = try_pool_state(&pool_state_acc.data).unwrap();
    assert_eq!(*total_sol_value, EXPECTED_NEW_TOTAL_SOL_VALUE);
}
