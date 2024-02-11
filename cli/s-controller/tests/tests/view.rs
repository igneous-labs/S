use std::process::Output;

use s_controller_test_utils::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn view_basic() {
    let pt = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 999_000_000,
        jitosol_reserves: 900_000_000,
        msol_sol_value: 111_000_000,
        msol_reserves: 100_000_000,
        jitosol_protocol_fee_accumulator: 123_321,
        msol_protocol_fee_accumulator: 321_123,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 156_234,
    })
    .add_s_program();

    let (mut cmd, _cfg, _bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_view();
    let Output { stderr, .. } = cmd.output().unwrap();
    eprintln!("{}", std::str::from_utf8(&stderr).unwrap());
}
