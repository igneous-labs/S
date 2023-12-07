use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
use marinade_calculator_lib::{MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS};
use marinade_keys::msol;
use s_controller_lib::{
    add_liquidity_ix_full, create_pool_reserves_address, try_lst_state_list, try_pool_state,
    AddLiquidityByMintFreeArgs, AddLiquidityIxFullArgs, AddRemoveLiquidityExtraAccounts,
};
use sanctum_utils::token::{token_2022_account_balance, token_account_balance};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::{processor, ProgramTestContext};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{
    banks_client_get_account, jito_stake_pool, jitosol, mock_lp_token_account, mock_token_account,
    MockTokenAccountArgs, JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

use crate::common::*;

#[tokio::test]
async fn basic_add_liquidity_twice() {
    const JITOSOL_TO_ADD: u64 = 1_000_000_000;
    const MSOL_TO_ADD: u64 = 1_000_000_000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let liquidity_provider_jitosol_acc_addr = Pubkey::new_unique();
    let liquidity_provider_msol_acc_addr = Pubkey::new_unique();
    let liquidity_provider_lp_token_acc_addr = Pubkey::new_unique();

    let mut program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    });
    program_test.add_program(
        "no_fee_pricing_program",
        no_fee_pricing_program::ID,
        processor!(no_fee_pricing_program::process_instruction),
    );
    program_test.add_account(
        liquidity_provider_jitosol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: liquidity_provider.pubkey(),
            amount: JITOSOL_TO_ADD,
        }),
    );
    program_test.add_account(
        liquidity_provider_msol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: msol::ID,
            authority: liquidity_provider.pubkey(),
            amount: MSOL_TO_ADD,
        }),
    );
    program_test.add_account(
        liquidity_provider_lp_token_acc_addr,
        mock_lp_token_account(MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        }),
    );
    let mut ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let jito_stake_pool_acc =
        banks_client_get_account(&mut ctx.banks_client, jito_stake_pool::ID).await;
    let jito_sol_val_calc_args = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedReadonlyAccount {
            key: jito_stake_pool::ID,
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
        (&jito_sol_val_calc_keys).into();
    exec_verify_add_liq_success(
        &mut ctx,
        &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        liquidity_provider_jitosol_acc_addr,
        jitosol::ID,
        spl_calculator_lib::program::ID,
        &jito_sol_val_calc_accounts,
    )
    .await;

    let marinade_sol_val_calc_keys: generic_pool_calculator_interface::LstToSolKeys =
        MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();
    let marinade_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        (&marinade_sol_val_calc_keys).into();
    exec_verify_add_liq_success(
        &mut ctx,
        &liquidity_provider,
        liquidity_provider_lp_token_acc_addr,
        liquidity_provider_msol_acc_addr,
        msol::ID,
        marinade_calculator_lib::program::ID,
        &marinade_sol_val_calc_accounts,
    )
    .await;
}

async fn exec_verify_add_liq_success(
    ProgramTestContext {
        banks_client,
        last_blockhash,
        payer,
        ..
    }: &mut ProgramTestContext,
    liquidity_provider: &Keypair,
    liquidity_provider_lp_token_acc_addr: Pubkey,
    lst_account_to_add_from: Pubkey,
    lst_mint: Pubkey,
    lst_calculator_program_id: Pubkey,
    lst_calculator_accounts: &[AccountMeta],
) {
    let lp_token_account =
        banks_client_get_account(banks_client, liquidity_provider_lp_token_acc_addr).await;
    let lp_token_acc_balance = token_2022_account_balance(&lp_token_account).unwrap();
    let lst_account = banks_client_get_account(banks_client, lst_account_to_add_from).await;
    let lst_account_balance = token_account_balance(lst_account).unwrap();
    assert!(lst_account_balance > 0);

    let pool_state_account = banks_client_get_pool_state_acc(banks_client).await;
    let pool_total_sol_value = try_pool_state(&pool_state_account.data)
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
        lst_mint: KeyedReadonlyAccount {
            key: lst_mint,
            account: lst_mint_account,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let ix = add_liquidity_ix_full(
        keys,
        AddLiquidityIxFullArgs {
            lst_index,
            lst_amount: lst_account_balance,
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id,
            pricing_program_id: no_fee_pricing_program::ID,
            lst_calculator_accounts,
            pricing_program_price_lp_accounts: &[AccountMeta {
                pubkey: lst_mint,
                is_signer: false,
                is_writable: false,
            }],
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
    // since LST should be worth >1 SOL and we are adding from 0
    assert!(lp_token_increase > lst_account_balance);

    let lst_account = banks_client_get_account(banks_client, lst_account_to_add_from).await;
    let lst_account_balance_after = token_account_balance(lst_account).unwrap();
    assert_eq!(lst_account_balance_after, 0);

    let pool_reserves_account = banks_client_get_account(banks_client, pool_reserves).await;
    let pool_reserves_balance_after = token_account_balance(&pool_reserves_account).unwrap();
    let pool_lst_increase = pool_reserves_balance_after - pool_reserves_balance;
    assert_eq!(pool_lst_increase, lst_account_balance);

    let pool_state_account = banks_client_get_pool_state_acc(banks_client).await;
    let pool_total_sol_value_after = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let pool_total_sol_value_inc = pool_total_sol_value_after - pool_total_sol_value;
    assert!(pool_total_sol_value_inc > lst_account_balance);
}
