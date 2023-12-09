use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
use marinade_calculator_lib::{MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS};
use marinade_keys::msol;
use s_controller_lib::{
    swap_exact_in_ix_by_mint_full, try_pool_state, SrcDstLstSolValueCalcAccounts,
    SwapByMintsFreeArgs, SwapExactInAmounts,
};
use sanctum_utils::{mint_with_token_program::MintWithTokenProgram, token::token_account_balance};
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey};
use solana_program_test::ProgramTestContext;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_calculator_lib::{SplLstSolCommonFreeArgsConst, SplSolValCalc};
use test_utils::{
    banks_client_get_account, jito_stake_pool, jitosol, mock_token_account, MockTokenAccountArgs,
    JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
};

use crate::common::*;

#[tokio::test]
async fn basic_no_fee() {
    const JITOSOL_POOL_RESERVES: u64 = 10_000_000_000;
    const MSOL_POOL_RESERVES: u64 = 10_000_000_000;
    const MSOL_TO_SWAP_IN: u64 = 1_000_000_000;

    let swapper = Keypair::new();
    let lp_token_mint = Pubkey::new_unique();
    let swapper_jitosol_acc_addr = Pubkey::new_unique();
    let swapper_msol_acc_addr = Pubkey::new_unique();

    let mut program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_reserves: JITOSOL_POOL_RESERVES,
        msol_reserves: MSOL_POOL_RESERVES,
        jitosol_sol_value: JITOSOL_POOL_RESERVES, // updated on sync
        msol_sol_value: MSOL_POOL_RESERVES,       // updated on sync
        // dont cares
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint,
        lp_token_supply: 0,
    });
    program_test.add_account(
        swapper_jitosol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: jitosol::ID,
            authority: swapper.pubkey(),
            amount: 0,
        }),
    );
    program_test.add_account(
        swapper_msol_acc_addr,
        mock_token_account(MockTokenAccountArgs {
            mint: msol::ID,
            authority: swapper.pubkey(),
            amount: MSOL_TO_SWAP_IN,
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

    let pool_state_account = banks_client_get_pool_state_acc(&mut banks_client).await;
    // hasnt synced yet, should be MSOL_POOL_RESERVES + JITOSOL_POOL_RESERVES
    let start_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    let lst_state_list_account = banks_client_get_lst_state_list_acc(&mut banks_client).await;

    let jito_stake_pool_acc =
        banks_client_get_account(&mut banks_client, jito_stake_pool::ID).await;
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

    let marinade_sol_val_calc_keys: generic_pool_calculator_interface::SolToLstKeys =
        MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();
    let marinade_sol_val_calc_accounts: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] =
        (&marinade_sol_val_calc_keys).into();

    let ix = swap_exact_in_ix_by_mint_full(
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
        SwapExactInAmounts {
            // mSOL worth more than jitoSOL
            min_amount_out: MSOL_TO_SWAP_IN,
            amount: MSOL_TO_SWAP_IN,
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

    let msol_account = banks_client_get_account(&mut banks_client, swapper_msol_acc_addr).await;
    assert_eq!(token_account_balance(msol_account).unwrap(), 0);

    let jitosol_account =
        banks_client_get_account(&mut banks_client, swapper_jitosol_acc_addr).await;
    let jitosol_received = token_account_balance(jitosol_account).unwrap();
    // mSOL worth more than jitoSOL
    assert!(jitosol_received > MSOL_TO_SWAP_IN);

    let msol_pool_reserves_account =
        banks_client_get_account(&mut banks_client, msol_pool_reserves).await;
    assert_eq!(
        token_account_balance(msol_pool_reserves_account).unwrap(),
        MSOL_POOL_RESERVES + MSOL_TO_SWAP_IN
    );

    let jitosol_pool_reserves_account =
        banks_client_get_account(&mut banks_client, jitosol_pool_reserves).await;
    assert_eq!(
        token_account_balance(jitosol_pool_reserves_account).unwrap(),
        JITOSOL_POOL_RESERVES - jitosol_received
    );

    let pool_state_account = banks_client_get_pool_state_acc(&mut banks_client).await;
    let end_pool_total_sol_value = try_pool_state(&pool_state_account.data)
        .unwrap()
        .total_sol_value;
    assert!(start_pool_total_sol_value < end_pool_total_sol_value);
}
