use flat_fee_lib::account_resolvers::PriceLpTokensToRedeemFreeArgs;
use flat_fee_test_utils::MockFeeAccountArgs;
use lido_keys::stsol;
use s_controller_interface::SControllerError;
use s_controller_lib::{
    remove_liquidity_ix_full, try_pool_state, AddRemoveLiquidityExtraAccounts,
    RemoveLiquidityByMintFreeArgs, RemoveLiquidityIxAmts, RemoveLiquidityIxFullArgs,
};
use s_controller_test_utils::{
    jito_marinade_no_fee_program_test, lido_wsol_flat_fee_program_test,
    GenAndAddTokenAccountProgramTest, JitoMarinadeProgramTestArgs, LidoWsolProgramTestArgs,
    LstStateListBanksClient, MockProtocolFeeBps, PoolStateBanksClient,
};
use sanctum_solana_test_utils::{
    assert_custom_err, token::MockTokenAccountArgs, ExtendedBanksClient,
};
use sanctum_token_lib::{mint_supply, token_account_balance, MintWithTokenProgram};
use sanctum_token_ratio::{CeilDiv, ReversibleFee, U64BpsFee};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::SplLstSolCommonFreeArgsConst;
use spl_token::native_mint;
use test_utils::{jito_stake_pool, jitosol, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};
use wsol_calculator_lib::WSOL_SOL_TO_LST_METAS;

use crate::common::SControllerProgramTest;

#[tokio::test]
async fn basic_redeem_full_no_fees() {
    const LP_TOKEN_SUPPLY: u64 = 1_000_000_000;
    const LP_TOKENS_TO_REMOVE: u64 = LP_TOKEN_SUPPLY;
    const JITOSOL_RESERVES_STARTING_BALANCE: u64 = 1_000_000_000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_RESERVES_STARTING_BALANCE, // will increase on SyncSolValue
        jitosol_reserves: JITOSOL_RESERVES_STARTING_BALANCE,
        lp_token_mint,
        lp_token_supply: LP_TOKEN_SUPPLY,
        msol_sol_value: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
    })
    .add_s_program();
    let liquidity_provider_jitosol_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        });
    let liquidity_provider_lp_token_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: LP_TOKENS_TO_REMOVE,
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

    let jito_stake_pool_acc = banks_client
        .get_account_unwrapped(jito_stake_pool::ID)
        .await;
    let pool_state_account = banks_client.get_pool_state_acc().await;
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;

    let args = RemoveLiquidityByMintFreeArgs {
        signer: liquidity_provider.pubkey(),
        src_lp_acc: liquidity_provider_lp_token_acc_addr,
        dst_lst_acc: liquidity_provider_jitosol_acc_addr,
        pool_state: pool_state_account,
        lst_state_list: &lst_state_list_account,
        lst_mint: MintWithTokenProgram {
            pubkey: jitosol::ID,
            token_program: spl_token::ID,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let pool_reserves = keys.pool_reserves;
    let protocol_fee_accumulator = keys.protocol_fee_accumulator;
    let ix = remove_liquidity_ix_full(
        keys,
        RemoveLiquidityIxFullArgs {
            lst_index,
            amts: RemoveLiquidityIxAmts {
                lp_token_amount: LP_TOKENS_TO_REMOVE,
                min_lst_out: 0,
            },
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id: spl_calculator_lib::program::ID,
            pricing_program_id: no_fee_pricing_program::ID,
            lst_calculator_accounts: &SplLstSolCommonFreeArgsConst {
                spl_stake_pool: KeyedAccount {
                    pubkey: jito_stake_pool::ID,
                    account: jito_stake_pool_acc,
                },
            }
            .resolve_to_account_metas()
            .unwrap(),
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
    // - reserves should be empty,
    //   but 1 or 2 lamports might be left in the pool due to rounding down in sync_sol_value and SolToLst
    // - protocol_fee_accumulator should be empty
    let pool_reserves_account = banks_client.get_account_unwrapped(pool_reserves).await;
    let pool_reserves_balance = token_account_balance(pool_reserves_account).unwrap();
    assert_eq!(pool_reserves_balance, 2);

    let liquidity_provider_jitosol_account = banks_client
        .get_account_unwrapped(liquidity_provider_jitosol_acc_addr)
        .await;
    assert_eq!(
        token_account_balance(liquidity_provider_jitosol_account).unwrap(),
        JITOSOL_RESERVES_STARTING_BALANCE - pool_reserves_balance
    );

    let protocol_fee_accumulator_account = banks_client
        .get_account_unwrapped(protocol_fee_accumulator)
        .await;
    assert_eq!(
        token_account_balance(protocol_fee_accumulator_account).unwrap(),
        0
    );

    let lp_mint_account = banks_client.get_account_unwrapped(lp_token_mint).await;
    assert_eq!(mint_supply(lp_mint_account).unwrap(), 0);

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_account.data).unwrap();
    // SOL value of 2 jitoLamports
    assert_eq!(pool_state.total_sol_value, 1);
}

#[tokio::test]
async fn basic_redeem_full_flat_fees() {
    const LP_TOKEN_SUPPLY: u64 = 1_000_000_000;
    const LP_TOKENS_TO_REMOVE: u64 = LP_TOKEN_SUPPLY;
    // 10x exchange rate
    const WSOL_RESERVES_STARTING_BALANCE: u64 = 10_000_000_000;
    const LP_WITHDRAWAL_FEE_BPS: u16 = 10;
    const PROTOCOL_FEE_BPS: u16 = 5000;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = lido_wsol_flat_fee_program_test(
        LidoWsolProgramTestArgs {
            wsol_reserves: WSOL_RESERVES_STARTING_BALANCE,
            stsol_sol_value: 0,
            stsol_reserves: 0,
            wsol_protocol_fee_accumulator: 0,
            stsol_protocol_fee_accumulator: 0,
            lp_token_mint,
            lp_token_supply: LP_TOKEN_SUPPLY,
        },
        flat_fee_interface::ProgramState {
            manager: Pubkey::default(),
            lp_withdrawal_fee_bps: LP_WITHDRAWAL_FEE_BPS,
        },
        [
            MockFeeAccountArgs {
                input_fee_bps: Default::default(),
                output_fee_bps: Default::default(),
                lst_mint: native_mint::ID,
            },
            MockFeeAccountArgs {
                input_fee_bps: Default::default(),
                output_fee_bps: Default::default(),
                lst_mint: stsol::ID,
            },
        ],
        MockProtocolFeeBps {
            trading: Default::default(),
            lp: PROTOCOL_FEE_BPS,
        },
    )
    .add_s_program();
    let liquidity_provider_wsol_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: native_mint::ID,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        });
    let liquidity_provider_lp_token_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: LP_TOKENS_TO_REMOVE,
        });

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;

    let args = RemoveLiquidityByMintFreeArgs {
        signer: liquidity_provider.pubkey(),
        src_lp_acc: liquidity_provider_lp_token_acc_addr,
        dst_lst_acc: liquidity_provider_wsol_acc_addr,
        pool_state: pool_state_account,
        lst_state_list: &lst_state_list_account,
        lst_mint: MintWithTokenProgram {
            pubkey: native_mint::ID,
            token_program: spl_token::ID,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let pool_reserves = keys.pool_reserves;
    let protocol_fee_accumulator = keys.protocol_fee_accumulator;
    let ix = remove_liquidity_ix_full(
        keys,
        RemoveLiquidityIxFullArgs {
            lst_index,
            amts: RemoveLiquidityIxAmts {
                lp_token_amount: LP_TOKENS_TO_REMOVE,
                min_lst_out: 0,
            },
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id: wsol_calculator_lib::program::ID,
            lst_calculator_accounts: &WSOL_SOL_TO_LST_METAS,
            pricing_program_id: flat_fee_lib::program::ID,
            pricing_program_price_lp_accounts: &PriceLpTokensToRedeemFreeArgs {
                output_lst_mint: native_mint::ID,
            }
            .resolve_to_account_metas(),
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &liquidity_provider], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let aaf = CeilDiv(U64BpsFee::new_unchecked(LP_WITHDRAWAL_FEE_BPS))
        .apply(WSOL_RESERVES_STARTING_BALANCE)
        .unwrap();
    let amt_after_fee = aaf.amt_after_fee();
    let fee_charged = aaf.fee_charged();
    let aaf = CeilDiv(U64BpsFee::new_unchecked(PROTOCOL_FEE_BPS))
        .apply(fee_charged)
        .unwrap();
    let protocol_fees_charged = aaf.fee_charged();
    let fees_withheld_in_reserves = aaf.amt_after_fee();

    let pool_reserves_account = banks_client.get_account_unwrapped(pool_reserves).await;
    let pool_reserves_ending_balance = token_account_balance(pool_reserves_account).unwrap();

    assert!(pool_reserves_ending_balance > 0);
    assert_eq!(pool_reserves_ending_balance, fees_withheld_in_reserves);

    let liquidity_provider_wsol_account = banks_client
        .get_account_unwrapped(liquidity_provider_wsol_acc_addr)
        .await;
    assert_eq!(
        token_account_balance(liquidity_provider_wsol_account).unwrap(),
        amt_after_fee
    );

    let protocol_fee_accumulator_account = banks_client
        .get_account_unwrapped(protocol_fee_accumulator)
        .await;
    assert_eq!(
        token_account_balance(protocol_fee_accumulator_account).unwrap(),
        protocol_fees_charged
    );
}

#[tokio::test]
async fn fail_remove_liquidity_slippage() {
    const UNREALISTIC_MIN_LST_EXPECTED: u64 = 10_000_000_000;
    const LP_TOKEN_SUPPLY: u64 = 999_999_999;
    const LP_TOKENS_TO_REMOVE: u64 = LP_TOKEN_SUPPLY;
    const JITOSOL_RESERVES_STARTING_BALANCE: u64 = 999_999_999;

    let liquidity_provider = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: JITOSOL_RESERVES_STARTING_BALANCE,
        msol_sol_value: 0,
        jitosol_reserves: JITOSOL_RESERVES_STARTING_BALANCE,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: LP_TOKEN_SUPPLY,
    })
    .add_s_program();
    let liquidity_provider_jitosol_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: liquidity_provider.pubkey(),
            amount: 0,
        });
    let liquidity_provider_lp_token_acc_addr =
        program_test.gen_and_add_token_account(MockTokenAccountArgs {
            mint: lp_token_mint,
            authority: liquidity_provider.pubkey(),
            amount: LP_TOKENS_TO_REMOVE,
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

    let jito_stake_pool_acc = banks_client
        .get_account_unwrapped(jito_stake_pool::ID)
        .await;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let lst_mint_account = banks_client
        .get_account(jitosol::ID)
        .await
        .unwrap()
        .unwrap();

    let args = RemoveLiquidityByMintFreeArgs {
        signer: liquidity_provider.pubkey(),
        src_lp_acc: liquidity_provider_lp_token_acc_addr,
        dst_lst_acc: liquidity_provider_jitosol_acc_addr,
        pool_state: pool_state_account,
        lst_state_list: &lst_state_list_account,
        lst_mint: KeyedAccount {
            pubkey: jitosol::ID,
            account: lst_mint_account,
        },
    };
    let (keys, lst_index) = args.resolve().unwrap();
    let ix = remove_liquidity_ix_full(
        keys,
        RemoveLiquidityIxFullArgs {
            lst_index,
            amts: RemoveLiquidityIxAmts {
                lp_token_amount: LP_TOKENS_TO_REMOVE,
                min_lst_out: UNREALISTIC_MIN_LST_EXPECTED,
            },
        },
        AddRemoveLiquidityExtraAccounts {
            lst_calculator_program_id: spl_calculator_lib::program::ID,
            pricing_program_id: no_fee_pricing_program::ID,
            lst_calculator_accounts: &SplLstSolCommonFreeArgsConst {
                spl_stake_pool: KeyedAccount {
                    pubkey: jito_stake_pool::ID,
                    account: jito_stake_pool_acc,
                },
            }
            .resolve_to_account_metas()
            .unwrap(),
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

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::SlippageToleranceExceeded);
}
