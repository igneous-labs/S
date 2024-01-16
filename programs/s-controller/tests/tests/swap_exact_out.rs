use flat_fee_lib::account_resolvers::PriceExactOutFreeArgs;
use flat_fee_test_utils::MockFeeAccountArgs;
use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
use marinade_calculator_lib::{
    marinade_sol_val_calc_account_metas, MarinadeSolValCalc,
    MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use marinade_keys::msol;
use s_controller_interface::SControllerError;
use s_controller_lib::{
    swap_exact_out_ix_by_mint_full, try_pool_state, SrcDstLstSolValueCalcAccounts,
    SwapByMintsFreeArgs, SwapExactOutAmounts,
};
use sanctum_solana_test_utils::{
    assert_custom_err,
    token::{tokenkeg::TokenkegProgramTest, MockTokenAccountArgs},
    ExtendedBanksClient,
};
use sanctum_token_lib::{token_account_balance, MintWithTokenProgram};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{jito_stake_pool, jitosol, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::common::*;

#[tokio::test]
async fn basic_swap_exact_out_no_fee() {
    const JITOSOL_STARTING_POOL_RESERVES: u64 = 10_000_000_000;
    const MSOL_STARTING_POOL_RESERVES: u64 = 10_000_000_000;
    const JITOSOL_TO_RECEIVE: u64 = 1_000_000_000;
    // mSOL worth more than jitoSOL, so this should be enough
    // to swap into JITOSOL_TO_RECEIVE
    const MSOL_STARTING_BALANCE: u64 = 1_000_000_000;

    let swapper = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let swapper_jitosol_acc_addr = Pubkey::new_unique();
    let swapper_msol_acc_addr = Pubkey::new_unique();

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_reserves: JITOSOL_STARTING_POOL_RESERVES,
        msol_reserves: MSOL_STARTING_POOL_RESERVES,
        jitosol_sol_value: JITOSOL_STARTING_POOL_RESERVES, // updated on sync
        msol_sol_value: MSOL_STARTING_POOL_RESERVES,       // updated on sync
        // dont cares
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    })
    .add_tokenkeg_account_from_args(
        swapper_jitosol_acc_addr,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: swapper.pubkey(),
            amount: 0,
        },
    )
    .add_tokenkeg_account_from_args(
        swapper_msol_acc_addr,
        MockTokenAccountArgs {
            mint: msol::ID,
            authority: swapper.pubkey(),
            amount: MSOL_STARTING_BALANCE,
        },
    );

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    // hasnt synced yet, should be MSOL_POOL_RESERVES + JITOSOL_POOL_RESERVES
    let start_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;

    let jito_stake_pool_acc = banks_client
        .get_account_unwrapped(jito_stake_pool::ID)
        .await;
    let jito_sol_val_calc_args = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedAccount {
            pubkey: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    };
    let jito_sol_val_calc_keys: generic_pool_calculator_interface::LstToSolKeys =
        jito_sol_val_calc_args
            .resolve()
            .unwrap()
            .resolve::<SplSolValCalc>()
            .into();
    let jito_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        jito_sol_val_calc_keys.into();

    let marinade_sol_val_calc_keys: generic_pool_calculator_interface::SolToLstKeys =
        MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();
    let marinade_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        marinade_sol_val_calc_keys.into();

    let ix = swap_exact_out_ix_by_mint_full(
        SwapByMintsFreeArgs {
            signer: swapper.pubkey(),
            src_lst_acc: swapper_msol_acc_addr,
            dst_lst_acc: swapper_jitosol_acc_addr,
            src_lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
            dst_lst_mint: MintWithTokenProgram {
                pubkey: jitosol::ID,
                token_program: spl_token::ID,
            },
            lst_state_list: lst_state_list_account,
        },
        SwapExactOutAmounts {
            // mSOL worth more than jitoSOL
            max_amount_in: JITOSOL_TO_RECEIVE,
            amount: JITOSOL_TO_RECEIVE,
        },
        SrcDstLstSolValueCalcAccounts {
            dst_lst_calculator_program_id: spl_calculator_lib::program::ID,
            dst_lst_calculator_accounts: &jito_sol_val_calc_accounts,
            src_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            src_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
        },
        &[
            AccountMeta {
                pubkey: msol::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: jitosol::ID,
                is_signer: false,
                is_writable: false,
            },
        ],
        no_fee_pricing_program::ID,
    )
    .unwrap();
    let msol_pool_reserves = ix.accounts[10].pubkey;
    let jitosol_pool_reserves = ix.accounts[11].pubkey;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &swapper], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let msol_account = banks_client
        .get_account_unwrapped(swapper_msol_acc_addr)
        .await;
    let msol_paid = MSOL_STARTING_BALANCE - token_account_balance(msol_account).unwrap();
    assert!(msol_paid < JITOSOL_TO_RECEIVE);

    let jitosol_account = banks_client
        .get_account_unwrapped(swapper_jitosol_acc_addr)
        .await;
    let jitosol_received = token_account_balance(jitosol_account).unwrap();
    assert_eq!(jitosol_received, JITOSOL_TO_RECEIVE);

    let msol_pool_reserves_account = banks_client.get_account_unwrapped(msol_pool_reserves).await;
    assert_eq!(
        token_account_balance(msol_pool_reserves_account).unwrap(),
        MSOL_STARTING_POOL_RESERVES + msol_paid
    );

    let jitosol_pool_reserves_account = banks_client
        .get_account_unwrapped(jitosol_pool_reserves)
        .await;
    assert_eq!(
        token_account_balance(jitosol_pool_reserves_account).unwrap(),
        JITOSOL_STARTING_POOL_RESERVES - JITOSOL_TO_RECEIVE
    );

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let end_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    assert!(start_pool_total_sol_value < end_pool_total_sol_value);
}

#[tokio::test]
async fn basic_swap_exact_out_flat_fee() {
    const JITOSOL_STARTING_POOL_RESERVES: u64 = 10_000_000_000;
    const MSOL_STARTING_POOL_RESERVES: u64 = 10_000_000_000;
    const JITOSOL_TO_RECEIVE: u64 = 1_000_000_000;
    // mSOL worth more than jitoSOL, so this should be enough
    // to swap into JITOSOL_TO_RECEIVE
    const MSOL_STARTING_BALANCE: u64 = 1_000_000_000;

    const JITOSOL_OUT_FEE_BPS: i16 = 6;
    const MSOL_IN_FEE_BPS: i16 = 9;
    const TRADING_PROTOCOL_FEE_BPS: u16 = 5_000;

    let swapper = Keypair::new();

    let mut program_test = jito_marinade_flat_fee_program_test(
        JitoMarinadeProgramTestArgs {
            jitosol_reserves: JITOSOL_STARTING_POOL_RESERVES,
            msol_reserves: MSOL_STARTING_POOL_RESERVES,
            jitosol_sol_value: JITOSOL_STARTING_POOL_RESERVES, // updated on sync
            msol_sol_value: MSOL_STARTING_POOL_RESERVES,       // updated on sync
            // dont cares
            jitosol_protocol_fee_accumulator: 0,
            msol_protocol_fee_accumulator: 0,
            lp_token_mint: Pubkey::new_unique(),
            lp_token_supply: 0,
        },
        flat_fee_interface::ProgramState {
            manager: Default::default(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        [
            MockFeeAccountArgs {
                input_fee_bps: Default::default(),
                output_fee_bps: JITOSOL_OUT_FEE_BPS,
                lst_mint: jitosol::ID,
            },
            MockFeeAccountArgs {
                input_fee_bps: MSOL_IN_FEE_BPS,
                output_fee_bps: Default::default(),
                lst_mint: msol::ID,
            },
        ],
        MockProtocolFeeBps {
            trading: TRADING_PROTOCOL_FEE_BPS,
            lp: Default::default(),
        },
    );

    let swapper_jitosol_acc_addr = program_test.gen_and_add_token_account(MockTokenAccountArgs {
        mint: jitosol::ID,
        authority: swapper.pubkey(),
        amount: 0,
    });
    let swapper_msol_acc_addr = program_test.gen_and_add_token_account(MockTokenAccountArgs {
        mint: msol::ID,
        authority: swapper.pubkey(),
        amount: MSOL_STARTING_BALANCE,
    });

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    // hasnt synced yet, should be MSOL_POOL_RESERVES + JITOSOL_POOL_RESERVES
    let start_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;

    let jito_stake_pool_acc = banks_client
        .get_account_unwrapped(jito_stake_pool::ID)
        .await;
    let jito_sol_val_calc_accounts = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedAccount {
            pubkey: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    }
    .resolve_to_account_metas()
    .unwrap();

    let marinade_sol_val_calc_accounts = marinade_sol_val_calc_account_metas();

    let ix = swap_exact_out_ix_by_mint_full(
        SwapByMintsFreeArgs {
            signer: swapper.pubkey(),
            src_lst_acc: swapper_msol_acc_addr,
            dst_lst_acc: swapper_jitosol_acc_addr,
            src_lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
            dst_lst_mint: MintWithTokenProgram {
                pubkey: jitosol::ID,
                token_program: spl_token::ID,
            },
            lst_state_list: lst_state_list_account,
        },
        SwapExactOutAmounts {
            // mSOL worth more than jitoSOL, so this shouldnt trigger slippage exceeded
            max_amount_in: JITOSOL_TO_RECEIVE,
            amount: JITOSOL_TO_RECEIVE,
        },
        SrcDstLstSolValueCalcAccounts {
            dst_lst_calculator_program_id: spl_calculator_lib::program::ID,
            dst_lst_calculator_accounts: &jito_sol_val_calc_accounts,
            src_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            src_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
        },
        &PriceExactOutFreeArgs {
            input_lst_mint: msol::ID,
            output_lst_mint: jitosol::ID,
        }
        .resolve_to_account_metas(),
        flat_fee_lib::program::ID,
    )
    .unwrap();
    let msol_pool_reserves = ix.accounts[10].pubkey;
    let jitosol_pool_reserves = ix.accounts[11].pubkey;
    let jitosol_protocol_fee_accumulator = ix.accounts[5].pubkey;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &swapper], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let msol_account = banks_client
        .get_account_unwrapped(swapper_msol_acc_addr)
        .await;
    let msol_paid = MSOL_STARTING_BALANCE - token_account_balance(msol_account).unwrap();
    assert!(msol_paid < JITOSOL_TO_RECEIVE);

    let jitosol_account = banks_client
        .get_account_unwrapped(swapper_jitosol_acc_addr)
        .await;
    let jitosol_received = token_account_balance(jitosol_account).unwrap();
    assert_eq!(jitosol_received, JITOSOL_TO_RECEIVE);

    let msol_pool_reserves_account = banks_client.get_account_unwrapped(msol_pool_reserves).await;
    assert_eq!(
        token_account_balance(msol_pool_reserves_account).unwrap(),
        MSOL_STARTING_POOL_RESERVES + msol_paid
    );

    let jitosol_pool_reserves_account = banks_client
        .get_account_unwrapped(jitosol_pool_reserves)
        .await;
    let jitosol_pool_reserves_balance =
        token_account_balance(jitosol_pool_reserves_account).unwrap();
    let jitosol_protocol_fee_accumulator_account = banks_client
        .get_account_unwrapped(jitosol_protocol_fee_accumulator)
        .await;
    let protocol_fee_accumulator_balance =
        token_account_balance(jitosol_protocol_fee_accumulator_account).unwrap();
    assert!(protocol_fee_accumulator_balance > 0);
    assert!(protocol_fee_accumulator_balance < jitosol_received);
    assert_eq!(
        jitosol_pool_reserves_balance + jitosol_received + protocol_fee_accumulator_balance,
        JITOSOL_STARTING_POOL_RESERVES
    );

    // TODO: verify fee percentages and amounts

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let end_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    assert!(start_pool_total_sol_value < end_pool_total_sol_value);
}

#[tokio::test]
async fn fail_swap_exact_out_same_mint() {
    const MSOL_STARTING_POOL_RESERVES: u64 = 10_000_000_000;
    const MSOL_STARTING_BALANCE: u64 = 1_000_000_000;

    let swapper = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let swapper_msol_acc_addr = Pubkey::new_unique();

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        msol_reserves: MSOL_STARTING_POOL_RESERVES,
        msol_sol_value: MSOL_STARTING_POOL_RESERVES, // updated on sync
        // dont cares
        jitosol_reserves: 0,
        jitosol_sol_value: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    })
    .add_tokenkeg_account_from_args(
        swapper_msol_acc_addr,
        MockTokenAccountArgs {
            mint: msol::ID,
            authority: swapper.pubkey(),
            amount: MSOL_STARTING_BALANCE,
        },
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;

    let marinade_sol_val_calc_keys: generic_pool_calculator_interface::SolToLstKeys =
        MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();
    let marinade_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        marinade_sol_val_calc_keys.into();

    let ix = swap_exact_out_ix_by_mint_full(
        SwapByMintsFreeArgs {
            signer: swapper.pubkey(),
            src_lst_acc: swapper_msol_acc_addr,
            dst_lst_acc: swapper_msol_acc_addr,
            src_lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
            dst_lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
            lst_state_list: lst_state_list_account,
        },
        SwapExactOutAmounts {
            max_amount_in: u64::MAX,
            amount: MSOL_STARTING_BALANCE,
        },
        SrcDstLstSolValueCalcAccounts {
            dst_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            dst_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
            src_lst_calculator_program_id: marinade_calculator_lib::program::ID,
            src_lst_calculator_accounts: &marinade_sol_val_calc_accounts,
        },
        &[
            AccountMeta {
                pubkey: msol::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: msol::ID,
                is_signer: false,
                is_writable: false,
            },
        ],
        no_fee_pricing_program::ID,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &swapper], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::SwapSameLst);
}
