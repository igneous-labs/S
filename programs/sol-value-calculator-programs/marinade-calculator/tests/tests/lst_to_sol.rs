use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use marinade_calculator_lib::{
    marinade_lst_to_sol_ix, MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use sanctum_token_ratio::{U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN};
use test_utils::BorshReturnDataBanksClient;

use crate::common::marinade_normal_program_test;

#[tokio::test]
async fn basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_RANGE: U64ValueRange = U64ValueRange::single(1_151_526_823);

    let program_test = marinade_normal_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let accounts: LstToSolKeys = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    let ix = marinade_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();

    banks_client
        .exec_verify_borsh_return_data::<U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN>(
            &payer,
            last_blockhash,
            ix,
            EXPECTED_LAMPORTS_RANGE,
        )
        .await;
}
