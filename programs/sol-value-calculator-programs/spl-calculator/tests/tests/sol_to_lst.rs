use generic_pool_calculator_interface::{SolToLstIxArgs, SolToLstKeys};
use solana_program::clock::Clock;
use solana_program_test::ProgramTestContext;

use spl_calculator_lib::{spl_sol_to_lst_ix, SplLstSolCommonFreeArgs, SplSolValCalc};
use test_utils::{exec_verify_u64_le_return_data, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::common::{jito_normal_program_test, JitoNormalProgramTest};

#[tokio::test]
async fn jito_basic() {
    const LAMPORTS_AMOUNT: u64 = 1_072_326_756;
    const EXPECTED_LST_AMOUNT: u64 = 1_000_000_000;

    let JitoNormalProgramTest {
        program_test,
        jito_stake_pool,
        spl_stake_pool_prog,
    } = jito_normal_program_test();

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

    let free_args = SplLstSolCommonFreeArgs {
        spl_stake_pool: jito_stake_pool,
        spl_stake_pool_prog,
    };
    let (intermediate, _stake_pool) = free_args.resolve().unwrap();
    let accounts: SolToLstKeys = intermediate.resolve::<SplSolValCalc>().unwrap().into();

    let ix = spl_sol_to_lst_ix(
        accounts,
        SolToLstIxArgs {
            amount: LAMPORTS_AMOUNT,
        },
    )
    .unwrap();

    exec_verify_u64_le_return_data(
        &mut banks_client,
        &payer,
        last_blockhash,
        ix,
        EXPECTED_LST_AMOUNT,
    )
    .await;
}
