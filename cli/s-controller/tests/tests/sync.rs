use s_controller_test_utils::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::common::{setup_with_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn sync_success_jitosol_symbol_force() {
    let payer = Keypair::new();
    let pt = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 1_000_000_000,
        jitosol_reserves: 1_000_000_000,
        // dont cares
        msol_sol_value: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_program();

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_payer(pt, payer).await;

    cmd.cmd_sync().arg("-f").arg("jitosol");
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    // TODO: check new synced sol value correct
}
