use flat_fee_test_utils::MockFeeAccountArgs;
use s_controller_lib::{find_pool_reserves_address, FindLstPdaAtaKeys};
use s_controller_test_utils::{
    jito_wsol_flat_fee_program_test, JitoWsolProgramTestArgs, MockProtocolFeeBps,
};
use sanctum_associated_token_lib::FindAtaAddressArgs;
use sanctum_solana_test_utils::{
    cli::{assert_all_txs_success_nonempty, ExtendedCommand},
    test_fixtures_dir,
    token::{tokenkeg::TokenkegProgramTest, MockTokenAccountArgs},
    ExtendedProgramTest,
};
use sanctum_token_lib::token_account_balance;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};
use spl_token::native_mint;
use stakedex_sdk_common::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn rebal_all_sol_to_jitosol_rebal_auth_payer() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let pt = jito_wsol_flat_fee_program_test(
        JitoWsolProgramTestArgs {
            jitosol_sol_value: 1_000_000_000,
            jitosol_reserves: 1_000_000_000,
            wsol_sol_value: 1_000_000_000,
            wsol_reserves: 1_000_000_000,
            // dont cares
            jitosol_protocol_fee_accumulator: 0,
            wsol_protocol_fee_accumulator: 0,
            lp_token_mint: Pubkey::new_unique(),
            lp_token_supply: 0,
        },
        // dont cares
        flat_fee_interface::ProgramState {
            manager: Pubkey::default(),
            lp_withdrawal_fee_bps: 0,
        },
        [
            MockFeeAccountArgs {
                input_fee_bps: 0,
                output_fee_bps: 0,
                lst_mint: native_mint::ID,
            },
            MockFeeAccountArgs {
                input_fee_bps: 0,
                output_fee_bps: 0,
                lst_mint: jitosol::ID,
            },
        ],
        MockProtocolFeeBps { trading: 0, lp: 0 },
    )
    .add_s_program()
    .add_stakedex_program()
    .add_test_fixtures_account("stakedex-jitosol-fee-acc.json")
    .add_test_fixtures_account("stakedex-sol-bridge-out.json")
    .add_test_fixtures_account("srlut.json")
    .add_tokenkeg_account_from_args(
        FindAtaAddressArgs {
            wallet: mock_auth_kp.pubkey(),
            mint: jitosol::ID,
            token_program: spl_token::ID,
        }
        .find_ata_address()
        .0,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: mock_auth_kp.pubkey(),
            amount: 1_000_000_000,
        },
    );

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_rebal_sol("all", "jitosol");
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);

    let (wsol_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
        lst_mint: native_mint::ID,
        token_program: spl_token::ID,
    });
    let wsol_reserves_after = bc.get_account(wsol_reserves).await.unwrap().unwrap();
    assert_eq!(token_account_balance(wsol_reserves_after).unwrap(), 0);
}
