use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use lido_calculator_lib::{
    lido_lst_to_sol_ix, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use sanctum_token_ratio::{U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN};
use solana_program::clock::Clock;
use solana_program_test::ProgramTestContext;
use test_utils::{BorshReturnDataBanksClient, LIDO_STATE_LAST_UPDATE_EPOCH};

use crate::common::lido_normal_program_test;

#[tokio::test]
async fn basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_RANGE: U64ValueRange = U64ValueRange::single(1_147_696_330);

    let program_test = lido_normal_program_test();

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: LIDO_STATE_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let accounts: LstToSolKeys = LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<LidoSolValCalc>()
        .into();

    let ix = lido_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();

    banks_client
        .exec_verify_borsh_return_data::<U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN>(
            &payer,
            last_blockhash,
            ix,
            EXPECTED_LAMPORTS_RANGE,
        )
        .await;
}
