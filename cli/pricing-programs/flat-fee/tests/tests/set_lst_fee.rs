use flat_fee_interface::ProgramState;
use flat_fee_test_utils::MockFeeAccountArgs;
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::common::{setup, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn set_lst_fee_success() {
    const INPUT_FEE_BPS: i16 = 69;
    const OUTPUT_FEE_BPS: i16 = 420;

    let lst_mint: Pubkey = Pubkey::new_unique();
    let payer = Keypair::new();

    let program_state = ProgramState {
        manager: payer.pubkey(),
        lp_withdrawal_fee_bps: Default::default(),
    };

    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup(
        ProgramTest::default(),
        payer,
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
        .cmd_set_lst_fee()
        .arg(lst_mint.to_string())
        .arg(INPUT_FEE_BPS.to_string())
        .arg(OUTPUT_FEE_BPS.to_string());

    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
}
