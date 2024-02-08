use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_test_utils::{jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs};
use sanctum_associated_token_lib::FindAtaAddressArgs;
use sanctum_token_lib::{token_account_balance, token_account_mint};
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn withdraw_all_protocol_fees_success_jitosol_symbol_beneficiary_payer_create_ata() {
    const JITOSOL_PROTOCOL_FEES_ACCUMULATED: u64 = 1_000_000_000;
    let pt = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_protocol_fee_accumulator: JITOSOL_PROTOCOL_FEES_ACCUMULATED,
        // dont cares
        jitosol_sol_value: 0,
        jitosol_reserves: 0,
        msol_sol_value: 0,
        msol_reserves: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_program();

    let (mut cmd, _cfg, mut bc, mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_withdraw_protocol_fees().arg("jitosol");
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    let created_ata_addr = FindAtaAddressArgs {
        wallet: mock_auth_kp.pubkey(),
        mint: jitosol::ID,
        token_program: spl_token::ID,
    }
    .find_ata_address()
    .0;
    let created_ata = bc.get_account(created_ata_addr).await.unwrap().unwrap();
    assert_eq!(token_account_mint(&created_ata).unwrap(), jitosol::ID);
    assert_eq!(
        token_account_balance(&created_ata).unwrap(),
        JITOSOL_PROTOCOL_FEES_ACCUMULATED
    );
}
