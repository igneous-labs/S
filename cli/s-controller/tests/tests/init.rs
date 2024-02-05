use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_test_utils::PoolStateBanksClient;
use sanctum_solana_test_utils::{
    test_fixtures_dir,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
};
use solana_program_test::ProgramTest;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn init_success_payer_init_auth() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let lp_mint = Pubkey::new_unique();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_tokenkeg_mint_from_args(
            lp_mint,
            MockMintArgs {
                mint_authority: Some(mock_auth_kp.pubkey()),
                freeze_authority: Some(mock_auth_kp.pubkey()),
                supply: 0,
                decimals: 9,
            },
        );
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_init().arg(lp_mint.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    // check pool state exists
    bc.get_pool_state_acc().await;
}
