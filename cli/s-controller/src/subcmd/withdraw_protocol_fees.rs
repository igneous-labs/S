use clap::Args;
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::{
    withdraw_protocol_fees_ix_with_program_id, WithdrawProtocolFeesIxArgs,
};
use s_controller_lib::{
    find_pool_state_address, find_protocol_fee_accumulator_address, find_protocol_fee_address,
    try_pool_state, FindLstPdaAtaKeys, WithdrawProtocolFeesByMintFreeArgs,
    WithdrawProtocolFeesPdas,
};
use sanctum_associated_token_lib::FindAtaAddressArgs;
use sanctum_solana_cli_utils::PubkeySrc;
use sanctum_token_lib::{token_account_balance, MintWithTokenProgram};
use solana_sdk::{native_token::sol_to_lamports, pubkey::Pubkey};
use spl_associated_token_account::instruction::create_associated_token_account;

use crate::lst_arg::LstArg;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Withdraw accumulated protocol fees for a given LST.")]
pub struct WithdrawProtocolFeesArgs {
    #[arg(
        help = "Mint of the LST to withdraw protocol fees for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'"
    )]
    pub mint: String,

    #[arg(
        long,
        short,
        help = "The program's protocol fee beneficiary signer. Defaults to config wallet if not set."
    )]
    pub beneficiary: Option<String>,

    #[arg(
        long,
        short,
        help = "The destination token account to withdraw the protocol fees to. Defaults to associated token account of beneficiary if not set."
    )]
    pub withdraw_to: Option<Pubkey>,

    #[arg(
        long,
        short,
        help = "The token program of the LST. Must be provided if mint is not on sanctum-lst-list."
    )]
    pub token_program: Option<Pubkey>,

    #[arg(
        long,
        short,
        help = "Amount to withdraw. Defaults to entire balance of the protocol fee accumulator account if not provided."
    )]
    pub amount: Option<f64>,
}

enum WithdrawToAtaStatus {
    Beneficiary,
    Payer,
    Neither,
}

impl WithdrawProtocolFeesArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            mint,
            beneficiary,
            withdraw_to,
            amount,
            token_program,
        } = match args.subcmd {
            Subcmd::WithdrawProtocolFees(a) => a,
            _ => unreachable!(),
        };
        let mint = LstArg::parse_arg(&mint, &slsts).unwrap();

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let token_program = token_program.unwrap_or_else(|| {
            mint.token_program()
                .expect("Unknown mint, token program must be provided")
        });
        let beneficiary_signer =
            beneficiary.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let beneficiary = beneficiary_signer.as_ref().unwrap_or(&payer);
        let beneficiary_ata = FindAtaAddressArgs {
            wallet: beneficiary.pubkey(),
            mint: mint.mint(),
            token_program,
        }
        .find_ata_address()
        .0;
        let payer_ata = FindAtaAddressArgs {
            wallet: beneficiary.pubkey(),
            mint: mint.mint(),
            token_program,
        }
        .find_ata_address()
        .0;
        let (withdraw_to, withdraw_to_ata_status) = withdraw_to.map_or_else(
            || (beneficiary_ata, WithdrawToAtaStatus::Beneficiary),
            |withdraw_to| {
                (
                    withdraw_to,
                    if withdraw_to == beneficiary_ata {
                        WithdrawToAtaStatus::Beneficiary
                    } else if withdraw_to == payer_ata {
                        WithdrawToAtaStatus::Payer
                    } else {
                        WithdrawToAtaStatus::Neither
                    },
                )
            },
        );
        let protocol_fee_accumulator_addr =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: mint.mint(),
                token_program,
            })
            .0;
        let pool_state_addr = find_pool_state_address(program_id).0;

        let mut fetched_accs = rpc
            .get_multiple_accounts(&[protocol_fee_accumulator_addr, pool_state_addr, withdraw_to])
            .await
            .unwrap();
        let withdraw_to_opt = fetched_accs.pop().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();
        let protocol_fee_accumulator_acc = fetched_accs.pop().unwrap().unwrap();

        let expected_beneficiary = try_pool_state(&pool_state_acc.data)
            .unwrap()
            .protocol_fee_beneficiary;
        if expected_beneficiary != beneficiary.pubkey() {
            eprintln!(
                "Wrong beneficiary. Expected {expected_beneficiary}, got {}",
                beneficiary.pubkey()
            );
            return;
        }

        let mut ixs = if withdraw_to_opt.is_none() {
            match withdraw_to_ata_status {
                WithdrawToAtaStatus::Neither => {
                    eprintln!("Can only create withdraw_to token account if it's the associated token account of beneficiary or payer");
                    return;
                }
                WithdrawToAtaStatus::Beneficiary => {
                    vec![create_associated_token_account(
                        &payer.pubkey(),
                        &beneficiary.pubkey(),
                        &mint.mint(),
                        &token_program,
                    )]
                }
                WithdrawToAtaStatus::Payer => {
                    vec![create_associated_token_account(
                        &payer.pubkey(),
                        &payer.pubkey(),
                        &mint.mint(),
                        &token_program,
                    )]
                }
            }
        } else {
            vec![]
        };
        let amount = amount.map_or_else(
            || token_account_balance(protocol_fee_accumulator_acc).unwrap(),
            sol_to_lamports, // assume all LSTs are 9 d.p.
        );
        ixs.push(
            withdraw_protocol_fees_ix_with_program_id(
                program_id,
                WithdrawProtocolFeesByMintFreeArgs {
                    pool_state: pool_state_acc,
                    lst_mint: MintWithTokenProgram {
                        pubkey: mint.mint(),
                        token_program,
                    },
                    withdraw_to,
                }
                .resolve_with_pdas(WithdrawProtocolFeesPdas {
                    pool_state: pool_state_addr,
                    protocol_fee_accumulator_auth: find_protocol_fee_address(program_id).0,
                    protocol_fee_accumulator: protocol_fee_accumulator_addr,
                })
                .unwrap(),
                WithdrawProtocolFeesIxArgs { amount },
            )
            .unwrap(),
        );

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            ixs,
            &[],
            &mut [payer.as_ref(), beneficiary.as_ref()],
        )
        .await;
    }
}
