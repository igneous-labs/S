use std::process::Output;

use flat_fee_interface::ProgramState;
use flat_fee_test_utils::MockFeeAccountArgs;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Keypair;

use crate::common::{setup, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn view_lst_success() {
    let lst_mint: Pubkey = Pubkey::new_unique();

    let program_state = ProgramState {
        manager: Pubkey::default(),
        lp_withdrawal_fee_bps: Default::default(),
    };

    let (mut cmd, _cfg, _bc, _payer, _rbh) = setup(
        ProgramTest::default(),
        Keypair::new(),
        Some(program_state),
        &[],
        &[MockFeeAccountArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
            lst_mint,
        }],
    )
    .await;

    cmd.with_flat_fee_program()
        .cmd_view_lst()
        .arg(lst_mint.to_string());

    let Output { status, stdout, .. } = cmd.output().unwrap();
    assert!(status.success());
    eprintln!("{}", std::str::from_utf8(&stdout).unwrap());
}
