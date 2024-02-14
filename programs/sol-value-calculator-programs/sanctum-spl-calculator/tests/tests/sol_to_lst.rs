use generic_pool_calculator_interface::{
    sol_to_lst_ix_with_program_id, SolToLstIxArgs, SolToLstKeys,
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
    const LAMPORTS_AMOUNT: u64 = 1_005_776_791;
    const EXPECTED_LST_RANGE: U64ValueRange =
        U64ValueRange::from_min_max_unchecked(999_999_999, 1_000_000_001);

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
    let accounts: SolToLstKeys = intermediate
        .resolve::<SanctumSplSolValCalc>()
        .unwrap()
        .into();

    let ix = sol_to_lst_ix_with_program_id(
        SanctumSplSolValCalc::ID,
        accounts,
        SolToLstIxArgs {
            amount: LAMPORTS_AMOUNT,
        },
    )
    .unwrap();

    banks_client
        .exec_verify_borsh_return_data::<U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN>(
            &payer,
            last_blockhash,
            ix,
            EXPECTED_LST_RANGE,
        )
        .await;
}
