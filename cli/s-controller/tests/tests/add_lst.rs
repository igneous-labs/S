use s_controller_lib::try_lst_state_list;
use s_controller_test_utils::{
    mock_lst_state, AddSplProgramTest, LstStateListBanksClient, MockLstStateArgs, MockLstStateRet,
    PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use sanctum_token_lib::MintWithTokenProgram;
use solana_program_test::{BanksClient, ProgramTest};
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

async fn assert_token_acc_created(
    bc: &mut BanksClient,
    MintWithTokenProgram {
        pubkey,
        token_program,
    }: MintWithTokenProgram,
) {
    let token_acc = bc.get_account(pubkey).await.unwrap().unwrap();
    assert_eq!(token_acc.owner, token_program);
}

async fn assert_lst_added(bc: &mut BanksClient, expected: MockLstStateArgs) {
    let MockLstStateRet {
        lst_state,
        reserves_address,
        protocol_fee_accumulator_address,
        ..
    } = mock_lst_state(expected);
    let lst_state_list_acc = bc.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    assert!(lst_state_list.contains(&lst_state));
    assert_token_acc_created(
        bc,
        MintWithTokenProgram {
            pubkey: reserves_address,
            token_program: expected.token_program,
        },
    )
    .await;
    assert_token_acc_created(
        bc,
        MintWithTokenProgram {
            pubkey: protocol_fee_accumulator_address,
            token_program: expected.token_program,
        },
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn add_lst_jito_matched_sol_val_calc_success_payer_init_auth() {
    let pt = ProgramTest::default()
        .add_spl_progs()
        .add_jito_stake_pool()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_add_lst().arg(jitosol::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lst_added(
        &mut bc,
        MockLstStateArgs {
            mint: jitosol::ID,
            sol_value_calculator: spl_calculator_lib::program::ID,
            token_program: spl_token::ID,
            sol_value: 0,
            reserves_amt: 0,
            protocol_fee_accumulator_amt: 0,
            is_input_disabled: false,
        },
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn add_lst_jitosol_token_name_matched_sol_val_calc_success_payer_init_auth() {
    let pt = ProgramTest::default()
        .add_spl_progs()
        .add_jito_stake_pool()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_add_lst().arg("JitoSOL");
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lst_added(
        &mut bc,
        MockLstStateArgs {
            mint: jitosol::ID,
            sol_value_calculator: spl_calculator_lib::program::ID,
            token_program: spl_token::ID,
            sol_value: 0,
            reserves_amt: 0,
            protocol_fee_accumulator_amt: 0,
            is_input_disabled: false,
        },
    )
    .await;
}
