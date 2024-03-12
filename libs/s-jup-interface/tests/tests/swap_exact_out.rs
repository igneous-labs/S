use flat_fee_test_utils::MockFeeAccountArgs;
use jupiter_amm_interface::{QuoteParams, SwapMode};
use marinade_keys::msol;
use s_controller_test_utils::{
    jito_marinade_flat_fee_program_test, JitoMarinadeProgramTestArgs, MockProtocolFeeBps,
};
use sanctum_solana_test_utils::ExtendedProgramTest;
use sanctum_token_lib::MintWithTokenProgram;
use solana_program_test::ProgramTestContext;
use solana_sdk::{clock::Clock, pubkey::Pubkey, signature::Keypair, signer::Signer};
use test_utils::{jitosol, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::{assert_quote_swap_eq, fully_init_amm, MiscProgramTest};

#[tokio::test]
async fn swap_exact_out_jito_marinade_basic() {
    const AMT: u64 = 1_000_000_000;

    let wallet = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let pt = jito_marinade_flat_fee_program_test(
        JitoMarinadeProgramTestArgs {
            jitosol_sol_value: 10_000_000_000,
            msol_sol_value: 10_000_000_000,
            jitosol_reserves: 10_000_000_000,
            msol_reserves: 10_000_000_000,
            jitosol_protocol_fee_accumulator: 0,
            msol_protocol_fee_accumulator: 0,
            lp_token_mint,
            lp_token_supply: 0,
        },
        flat_fee_interface::ProgramState {
            manager: Pubkey::default(),
            lp_withdrawal_fee_bps: 0,
        },
        [
            MockFeeAccountArgs {
                input_fee_bps: 4,
                output_fee_bps: 4,
                lst_mint: jitosol::ID,
            },
            MockFeeAccountArgs {
                input_fee_bps: 4,
                output_fee_bps: 4,
                lst_mint: msol::ID,
            },
        ],
        MockProtocolFeeBps {
            trading: 1000,
            lp: 1000,
        },
    )
    .add_s_program()
    .add_system_account(wallet.pubkey(), 1_000_000_000)
    .add_ata(
        wallet.pubkey(),
        MintWithTokenProgram {
            pubkey: jitosol::ID,
            token_program: spl_token::ID,
        },
        4 * AMT, // make sure enough for exactout
    )
    .add_ata(
        wallet.pubkey(),
        MintWithTokenProgram {
            pubkey: msol::ID,
            token_program: spl_token::ID,
        },
        0,
    );
    let ctx = pt.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        banks_client: mut bc,
        ..
    } = ctx;

    let s = fully_init_amm(&mut bc, s_controller_lib::program::ID).await;
    assert_quote_swap_eq(
        &mut bc,
        &s,
        &wallet,
        &QuoteParams {
            amount: AMT,
            input_mint: jitosol::ID,
            output_mint: msol::ID,
            swap_mode: SwapMode::ExactOut,
        },
    )
    .await;
}
