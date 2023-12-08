use generic_pool_calculator_interface::{SolToLstIxArgs, SolToLstKeys};
use marinade_calculator_lib::{
    marinade_sol_to_lst_ix, MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use test_utils::exec_verify_u64_le_return_data;

use crate::common::marinade_normal_program_test;

#[tokio::test]
async fn basic() {
    const LAMPORTS_AMOUNT: u64 = 1_151_526_823;
    const EXPECTED_LST_AMOUNT: u64 = 1_000_000_000;

    let program_test = marinade_normal_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let accounts: SolToLstKeys = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    let ix = marinade_sol_to_lst_ix(
        accounts,
        SolToLstIxArgs {
            amount: 1_151_526_823,
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
