use cli_test_utils::TestCliCmd;
use flat_fee_interface::ProgramState;
use sanctum_solana_test_utils::token::{tokenkeg::TokenkegProgramTest, MockMintArgs};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::common::{setup_with_program_state, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn set_add_lst_success() {
    const INPUT_FEE_BPS: i16 = 69;
    const OUTPUT_FEE_BPS: i16 = 420;

    let lst_mint: Pubkey = Pubkey::new_unique();

    let payer = Keypair::new();
    let program_state = ProgramState {
        manager: payer.pubkey(),
        lp_withdrawal_fee_bps: Default::default(),
    };
    let pt = ProgramTest::default().add_tokenkeg_mint_from_args(
        lst_mint,
        MockMintArgs {
            mint_authority: None,
            freeze_authority: None,
            supply: 0,
            decimals: 9,
        },
    );

    let (mut cmd, _cfg, mut bc, _payer, _rbh) =
        setup_with_program_state(pt, payer, program_state).await;

    cmd.with_flat_fee_program()
        .cmd_add_lst()
        .arg(lst_mint.to_string())
        .arg(INPUT_FEE_BPS.to_string())
        .arg(OUTPUT_FEE_BPS.to_string())
        .unwrap();
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    exec_res[0].as_ref().unwrap().result.as_ref().unwrap();
}
