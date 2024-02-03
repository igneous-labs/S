use assert_cmd::Command;
use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_interface::PoolState;
use s_controller_lib::{initial_authority, DEFAULT_PRICING_PROGRAM};
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer,
    cli::TempCliConfig,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
    ExtendedProgramTest,
};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::common::{base_cmd, SctrProgramTest, TestSctrCmd};

// TODO: move DEFAULT_POOL_STATE from programs/s-controller/tests/common/state.rs into a test-util and use the type
const DEFAULT_POOL_STATE: PoolState = PoolState {
    total_sol_value: 0,
    trading_protocol_fee_bps: 0,
    lp_protocol_fee_bps: 0,
    version: 0,
    is_disabled: 0,
    is_rebalancing: 0,
    padding: [0u8; 1],
    admin: initial_authority::ID,
    rebalance_authority: initial_authority::ID,
    protocol_fee_beneficiary: initial_authority::ID,
    pricing_program: DEFAULT_PRICING_PROGRAM,
    lp_token_mint: Pubkey::new_from_array([0u8; 32]),
};

pub async fn setup_with_payer(
    pt: ProgramTest,
    payer_kp: &Keypair,
) -> (Command, TempCliConfig, BanksClient) {
    let (bc, _rng_payer, _rbh) = pt.start().await;
    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(payer_kp, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc)
}

#[tokio::test(flavor = "multi_thread")]
async fn set_admin_success_payer_admin() {
    let payer_kp = Keypair::new();
    let lp_mint = Pubkey::new_unique();
    let new_admin = Pubkey::new_unique();

    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(PoolState {
            admin: payer_kp.pubkey(),
            ..DEFAULT_POOL_STATE
        })
        .add_system_account(payer_kp.pubkey(), 1_000_000_000)
        .add_tokenkeg_mint_from_args(
            lp_mint,
            MockMintArgs {
                mint_authority: Some(payer_kp.pubkey()),
                freeze_authority: Some(payer_kp.pubkey()),
                supply: 0,
                decimals: 9,
            },
        );

    let (mut cmd, _cfg, mut bc) = setup_with_payer(pt, &payer_kp).await;

    cmd.cmd_set_admin().arg(new_admin.to_string()).unwrap();
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
}
