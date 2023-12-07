mod common;

use common::*;
use generic_pool_calculator_interface::SOL_TO_LST_IX_ACCOUNTS_LEN;
use s_controller_lib::{
    remove_liquidity_ix_full, try_pool_state, AddRemoveLiquidityExtraAccounts,
    RemoveLiquidityByMintFreeArgs, RemoveLiquidityIxFullArgs,
};
use sanctum_utils::token::{token_2022_mint_supply, token_account_balance};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::{processor, ProgramTestContext};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{
    banks_client_get_account, jito_stake_pool, jitosol, mock_lp_token_account, mock_token_account,
    MockTokenAccountArgs, JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

#[tokio::test]
async fn basic_redeem_full_no_fees() {
    const LP_TOKEN_SUPPLY: u64 = 1_000_000_000;
    const LP_TOKENS_TO_REMOVE: u64 = LP_TOKEN_SUPPLY;
    const JITOSOL_RESERVES_BALANCE: u64 = 1_000_000_000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let liquidity_provider_jitosol_acc_addr = Pubkey::new_unique();
    let liquidity_provider_lp_token_acc_addr = Pubkey::new_unique();

    let mut program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_RESERVES_BALANCE, // will increase on SyncSolValue
        jitosol_reserves: JITOSOL_RESERVES_BALANCE,
        lp_token_mint,
        lp_token_supply: LP_TOKEN_SUPPLY,
        msol_sol_value: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
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
            amount: 0,
        }),
    );
    program_test.add_account(
        liquidity_provider_lp_token_acc_addr,
        mock_lp_token_account(MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: LP_TOKENS_TO_REMOVE,
        }),
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

    let jito_stake_pool_acc =
        banks_client_get_account(&mut banks_client, jito_stake_pool::ID).await;
    let jito_sol_val_calc_args = SplLstSolCommonFreeArgsConst {
        spl_stake_pool: KeyedReadonlyAccount {
            key: jito_stake_pool::ID,
            account: jito_stake_pool_acc,
        },
    };
    let jito_sol_val_calc_keys: generic_pool_calculator_interface::SolToLstKeys =
        jito_sol_val_calc_args
            .resolve()
            .unwrap()
            .resolve::<SplSolValCalc>()
            .into();
    let jito_sol_val_calc_accounts: [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] =
        (&jito_sol_val_calc_keys).into();

    let pool_state_account = banks_client_get_pool_state_acc(&mut banks_client).await;
    let lst_state_list_account = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let jitosol_mint_account = banks_client_get_account(&mut banks_client, jitosol::ID).await;

    let args = RemoveLiquidityByMintFreeArgs {
        signer: liquidity_provider.pubkey(),
        src_lp_acc: liquidity_provider_lp_token_acc_addr,
        dst_lst_acc: liquidity_provider_jitosol_acc_addr,
        pool_state: pool_state_account,
        lst_state_list: &lst_state_list_account,
        lst_mint: KeyedReadonlyAccount {
            key: jitosol::ID,
            account: jitosol_mint_account,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let pool_reserves = keys.pool_reserves;
    let protocol_fee_accumulator = keys.protocol_fee_accumulator;
    let ix = remove_liquidity_ix_full(
        keys,
        RemoveLiquidityIxFullArgs {
            lst_index,
            lp_token_amount: LP_TOKENS_TO_REMOVE,
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id: spl_calculator_lib::program::ID,
            pricing_program_id: no_fee_pricing_program::ID,
            lst_calculator_accounts: &jito_sol_val_calc_accounts,
            pricing_program_price_lp_accounts: &[AccountMeta {
                pubkey: jitosol::ID,
                is_signer: false,
                is_writable: false,
            }],
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &liquidity_provider], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    // no fee pricing program, so
    // - reserves should be empty
    // - protocol_fee_accumulator should be empty
    let pool_reserves_account = banks_client_get_account(&mut banks_client, pool_reserves).await;
    assert_eq!(token_account_balance(pool_reserves_account).unwrap(), 0);

    let liquidity_provider_jitosol_account =
        banks_client_get_account(&mut banks_client, liquidity_provider_jitosol_acc_addr).await;
    assert_eq!(
        token_account_balance(liquidity_provider_jitosol_account).unwrap(),
        JITOSOL_RESERVES_BALANCE
    );

    let protocol_fee_accumulator_account =
        banks_client_get_account(&mut banks_client, protocol_fee_accumulator).await;
    assert_eq!(
        token_account_balance(protocol_fee_accumulator_account).unwrap(),
        0
    );

    let lp_mint_account = banks_client_get_account(&mut banks_client, lp_token_mint).await;
    assert_eq!(token_2022_mint_supply(lp_mint_account).unwrap(), 0);

    let pool_state_account = banks_client_get_pool_state_acc(&mut banks_client).await;
    let pool_state = try_pool_state(&pool_state_account.data).unwrap();
    assert_eq!(pool_state.total_sol_value, 0);
}
