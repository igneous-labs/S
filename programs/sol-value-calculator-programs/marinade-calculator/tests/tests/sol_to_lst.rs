use generic_pool_calculator_interface::{SolToLstIxArgs, SolToLstKeys};
use marinade_calculator_lib::{
    marinade_sol_to_lst_ix, MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use sanctum_token_ratio::{U64ValueRange, U64_VALUE_RANGE_BORSH_SER_LEN};
use test_utils::BorshReturnDataBanksClient;

use crate::common::marinade_normal_program_test;

#[tokio::test]
async fn basic() {
    const LAMPORTS_AMOUNT: u64 = 1_162_900_315;
    const EXPECTED_LST_RANGE: U64ValueRange =
        U64ValueRange::from_min_max_unchecked(999_999_998, 1_000_000_002);

    let program_test = marinade_normal_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let accounts: SolToLstKeys = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    let ix = marinade_sol_to_lst_ix(
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
