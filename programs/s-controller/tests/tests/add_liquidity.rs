use flat_fee_test_utils::MockFeeAccountArgs;
use lido_calculator_lib::lido_sol_val_calc_account_metas;
use lido_keys::stsol;
use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_lib::{
    add_liquidity_ix_full, create_pool_reserves_address, try_lst_state_list, try_pool_state,
    AddLiquidityByMintFreeArgs, AddLiquidityIxFullArgs, AddRemoveLiquidityExtraAccounts,
};
use sanctum_utils::token::{token_2022_account_balance, token_account_balance};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::SplLstSolCommonFreeArgsConst;
use spl_token::native_mint;
use test_utils::{
    banks_client_get_account, jito_stake_pool, jitosol, MockTokenAccountArgs,
    JITO_STAKE_POOL_LAST_UPDATE_EPOCH, LIDO_STATE_LAST_UPDATE_EPOCH,
};
use wsol_calculator_lib::WSOL_LST_TO_SOL_METAS;

use crate::common::*;

#[tokio::test]
async fn basic_add_liquidity_twice_no_fee() {
    const JITOSOL_TO_ADD: u64 = 1_000_000_000;
    const MSOL_TO_ADD: u64 = 1_000_000_000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    });
    let liquidity_provider_jitosol_acc_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: liquidity_provider.pubkey(),
            amount: JITOSOL_TO_ADD,
        },
    );
    let liquidity_provider_msol_acc_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: msol::ID,
            authority: liquidity_provider.pubkey(),
            amount: MSOL_TO_ADD,
        },
    );
    let liquidity_provider_lp_token_acc_addr = add_mock_lp_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        },
    );
    let mut ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let jito_stake_pool_acc =
        banks_client_get_account(&mut ctx.banks_client, jito_stake_pool::ID).await;

    exec_verify_add_liq_success_no_fees(ExecVerifyAddLiqSuccessNoFeesArgs {
        program_test_ctx: &mut ctx,
        liquidity_provider: &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        lst_account_to_add_from: liquidity_provider_jitosol_acc_addr,
        lst_mint: jitosol::ID,
        lst_calculator_program_id: spl_calculator_lib::program::ID,
        lst_calculator_accounts: &SplLstSolCommonFreeArgsConst {
            spl_stake_pool: KeyedAccount {
                pubkey: jito_stake_pool::ID,
                account: jito_stake_pool_acc,
            },
        }
        .resolve_to_account_metas()
        .unwrap(),
        pricing_program_id: no_fee_pricing_program::ID,
        pricing_program_accounts: &[AccountMeta {
            pubkey: jitosol::ID,
            is_signer: false,
            is_writable: false,
        }],
    })
    .await;

    exec_verify_add_liq_success_no_fees(ExecVerifyAddLiqSuccessNoFeesArgs {
        program_test_ctx: &mut ctx,
        liquidity_provider: &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        lst_account_to_add_from: liquidity_provider_msol_acc_addr,
        lst_mint: msol::ID,
        lst_calculator_program_id: marinade_calculator_lib::program::ID,
        lst_calculator_accounts: &marinade_sol_val_calc_account_metas(),
        pricing_program_id: no_fee_pricing_program::ID,
        pricing_program_accounts: &[AccountMeta {
            pubkey: jitosol::ID,
            is_signer: false,
            is_writable: false,
        }],
    })
    .await;
}

/// flat fee program doesnt charge fees on add liquidity
/// so this should be same as above
#[tokio::test]
async fn basic_add_liquidity_twice_flat_fee() {
    const STSOL_TO_ADD: u64 = 1_000_000_000;
    const WSOL_TO_ADD: u64 = 1_000_000_000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = lido_wsol_flat_fee_program_test(
        LidoWsolProgramTestArgs {
            wsol_reserves: 0,
            stsol_sol_value: 0,
            stsol_reserves: 0,
            wsol_protocol_fee_accumulator: 0,
            stsol_protocol_fee_accumulator: 0,
            lp_token_mint,
            lp_token_supply: 0,
        },
        // put in random fee values to make sure they dont affect 0 fees
        flat_fee_interface::ProgramState {
            manager: Default::default(),
            lp_withdrawal_fee_bps: 1000,
        },
        [
            MockFeeAccountArgs {
                input_fee_bps: 100,
                output_fee_bps: 100,
                lst_mint: stsol::ID,
            },
            MockFeeAccountArgs {
                input_fee_bps: 100,
                output_fee_bps: 100,
                lst_mint: native_mint::ID,
            },
        ],
        MockProtocolFeeBps {
            trading: 100,
            lp: 100,
        },
    );
    let liquidity_provider_stsol_acc_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: stsol::ID,
            authority: liquidity_provider.pubkey(),
            amount: STSOL_TO_ADD,
        },
    );
    let liquidity_provider_wsol_acc_addr = add_mock_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: native_mint::ID,
            authority: liquidity_provider.pubkey(),
            amount: WSOL_TO_ADD,
        },
    );
    let liquidity_provider_lp_token_acc_addr = add_mock_lp_token_account(
        &mut program_test,
        MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        },
    );
    let mut ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: LIDO_STATE_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    // not used, set by _ix_full()
    let flat_fee_add_liq_account_metas: [AccountMeta; 1] = [AccountMeta::default()];
    exec_verify_add_liq_success_no_fees(ExecVerifyAddLiqSuccessNoFeesArgs {
        program_test_ctx: &mut ctx,
        liquidity_provider: &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        lst_account_to_add_from: liquidity_provider_stsol_acc_addr,
        lst_mint: stsol::ID,
        lst_calculator_program_id: lido_calculator_lib::program::ID,
        lst_calculator_accounts: &lido_sol_val_calc_account_metas(),
        pricing_program_id: flat_fee_lib::program::ID,
        pricing_program_accounts: &flat_fee_add_liq_account_metas,
    })
    .await;

    exec_verify_add_liq_success_no_fees(ExecVerifyAddLiqSuccessNoFeesArgs {
        program_test_ctx: &mut ctx,
        liquidity_provider: &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        lst_account_to_add_from: liquidity_provider_wsol_acc_addr,
        lst_mint: native_mint::ID,
        lst_calculator_program_id: wsol_calculator_lib::program::ID,
        lst_calculator_accounts: &WSOL_LST_TO_SOL_METAS,
        pricing_program_id: flat_fee_lib::program::ID,
        pricing_program_accounts: &flat_fee_add_liq_account_metas,
    })
    .await;
}

struct ExecVerifyAddLiqSuccessNoFeesArgs<'a> {
    pub program_test_ctx: &'a mut ProgramTestContext,
    pub liquidity_provider: &'a Keypair,
    pub liquidity_provider_lp_token_acc_addr: Pubkey,
    pub lst_account_to_add_from: Pubkey,
    pub lst_mint: Pubkey,
    pub lst_calculator_program_id: Pubkey,
    pub lst_calculator_accounts: &'a [AccountMeta],
    pub pricing_program_id: Pubkey,
    pub pricing_program_accounts: &'a [AccountMeta],
}

async fn exec_verify_add_liq_success_no_fees(
    ExecVerifyAddLiqSuccessNoFeesArgs {
        program_test_ctx:
            ProgramTestContext {
                banks_client,
                last_blockhash,
                payer,
                ..
            },
        liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        lst_account_to_add_from,
        lst_mint,
        lst_calculator_program_id,
        lst_calculator_accounts,
        pricing_program_accounts,
        pricing_program_id,
    }: ExecVerifyAddLiqSuccessNoFeesArgs<'_>,
) {
    let lp_token_account =
        banks_client_get_account(banks_client, liquidity_provider_lp_token_acc_addr).await;
    let lp_token_acc_balance = token_2022_account_balance(&lp_token_account).unwrap();
    let lst_account = banks_client_get_account(banks_client, lst_account_to_add_from).await;
    let lst_account_starting_balance = token_account_balance(lst_account).unwrap();
    assert!(lst_account_starting_balance > 0);

    let pool_state_account = banks_client_get_pool_state_acc(banks_client).await;
    let pool_total_sol_value_before = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let lst_state_list_account = banks_client_get_lst_state_list_acc(banks_client).await;
    let lst_mint_account = banks_client.get_account(lst_mint).await.unwrap().unwrap();

    let args = AddLiquidityByMintFreeArgs {
        signer: liquidity_provider.pubkey(),
        src_lst_acc: lst_account_to_add_from,
        dst_lp_acc: liquidity_provider_lp_token_acc_addr,
        pool_state: pool_state_account,
        lst_state_list: &lst_state_list_account,
        lst_mint: KeyedAccount {
            pubkey: lst_mint,
            account: lst_mint_account,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let ix = add_liquidity_ix_full(
        keys,
        AddLiquidityIxFullArgs {
            lst_index,
            lst_amount: lst_account_starting_balance,
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id,
            pricing_program_id,
            lst_calculator_accounts,
            pricing_program_price_lp_accounts: pricing_program_accounts,
        },
    )
    .unwrap();
    let pool_reserves = {
        let lst_state_list = try_lst_state_list(&lst_state_list_account.data).unwrap();
        let lst_state = lst_state_list[lst_index];
        create_pool_reserves_address(&lst_state, spl_token::ID).unwrap()
    };
    let pool_reserves_account = banks_client_get_account(banks_client, pool_reserves).await;
    let pool_reserves_balance = token_account_balance(&pool_reserves_account).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer, liquidity_provider], *last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lp_token_account =
        banks_client_get_account(banks_client, liquidity_provider_lp_token_acc_addr).await;
    let lp_token_acc_balance_after = token_2022_account_balance(&lp_token_account).unwrap();
    let lp_token_increase = lp_token_acc_balance_after - lp_token_acc_balance;
    // since we are adding from 0 so lp_token should be 1:1 SOL value
    if lst_mint == native_mint::ID {
        assert_eq!(lp_token_increase, lst_account_starting_balance);
    } else {
        // since LST should be worth >1 SOL
        assert!(lp_token_increase > lst_account_starting_balance);
    }

    let lst_account = banks_client_get_account(banks_client, lst_account_to_add_from).await;
    let lst_account_balance_after = token_account_balance(lst_account).unwrap();
    assert_eq!(lst_account_balance_after, 0);

    let pool_reserves_account = banks_client_get_account(banks_client, pool_reserves).await;
    let pool_reserves_balance_after = token_account_balance(&pool_reserves_account).unwrap();
    let pool_lst_increase = pool_reserves_balance_after - pool_reserves_balance;
    // since no fees
    assert_eq!(pool_lst_increase, lst_account_starting_balance);

    let pool_state_account = banks_client_get_pool_state_acc(banks_client).await;
    let pool_total_sol_value_after = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let pool_total_sol_value_inc = pool_total_sol_value_after - pool_total_sol_value_before;
    if lst_mint == native_mint::ID {
        assert_eq!(pool_total_sol_value_inc, lst_account_starting_balance);
    } else {
        // since LST should be worth >1 SOL
        assert!(pool_total_sol_value_inc > lst_account_starting_balance);
    }
}
