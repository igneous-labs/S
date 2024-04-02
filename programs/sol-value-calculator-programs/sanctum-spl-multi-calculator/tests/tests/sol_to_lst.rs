use generic_pool_calculator_interface::{
    sol_to_lst_ix_with_program_id, SolToLstIxArgs, SolToLstKeys,
};
use generic_pool_calculator_lib::GenericPoolSolValCalc;
use sanctum_token_ratio::{U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN};
use solana_program::clock::Clock;
use solana_program_test::ProgramTestContext;

use spl_calculator_lib::{SanctumSplMultiSolValCalc, SplLstSolCommonFreeArgs};
use test_utils::{BorshReturnDataBanksClient, JUP_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::common::{jup_normal_program_test, JupNormalProgramTest};

#[tokio::test]
async fn jup_basic() {
    const LAMPORTS_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LST_RANGE: U64ValueRange =
        U64ValueRange::from_min_max_unchecked(1_000_000_000, 1_000_000_001);

    let JupNormalProgramTest {
        program_test,
        jup_stake_pool,
        spl_stake_pool_prog,
    } = jup_normal_program_test();

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JUP_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let free_args = SplLstSolCommonFreeArgs {
        spl_stake_pool: jup_stake_pool,
        spl_stake_pool_prog,
    };
    let (intermediate, _stake_pool) = free_args.resolve_sanctum_spl_multi().unwrap();
    let accounts: SolToLstKeys = intermediate
        .resolve::<SanctumSplMultiSolValCalc>()
        .unwrap()
        .into();

    let ix = sol_to_lst_ix_with_program_id(
        SanctumSplMultiSolValCalc::ID,
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
