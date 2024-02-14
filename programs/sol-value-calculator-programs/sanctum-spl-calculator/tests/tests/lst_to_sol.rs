use generic_pool_calculator_interface::{
    lst_to_sol_ix_with_program_id, LstToSolIxArgs, LstToSolKeys,
};
use generic_pool_calculator_lib::GenericPoolSolValCalc;
use sanctum_token_ratio::{U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN};
use solana_program::clock::Clock;
use solana_program_test::ProgramTestContext;

use spl_calculator_lib::{SanctumSplSolValCalc, SplLstSolCommonFreeArgs};
use test_utils::{BorshReturnDataBanksClient, PWR_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::common::{pwr_normal_program_test, PwrNormalProgramTest};

#[tokio::test]
async fn pwr_basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_RANGE: U64ValueRange = U64ValueRange::single(1_005_776_791);

    let PwrNormalProgramTest {
        program_test,
        pwr_stake_pool,
        spl_stake_pool_prog,
    } = pwr_normal_program_test();

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: PWR_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let free_args = SplLstSolCommonFreeArgs {
        spl_stake_pool: pwr_stake_pool,
        spl_stake_pool_prog,
    };
    let (intermediate, _stake_pool) = free_args.resolve_sanctum_spl().unwrap();
    let accounts: LstToSolKeys = intermediate
        .resolve::<SanctumSplSolValCalc>()
        .unwrap()
        .into();

    let ix = lst_to_sol_ix_with_program_id(
        SanctumSplSolValCalc::ID,
        accounts,
        LstToSolIxArgs { amount: LST_AMOUNT },
    )
    .unwrap();

    banks_client
        .exec_verify_borsh_return_data::<U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN>(
            &payer,
            last_blockhash,
            ix,
            EXPECTED_LAMPORTS_RANGE,
        )
        .await;
}
