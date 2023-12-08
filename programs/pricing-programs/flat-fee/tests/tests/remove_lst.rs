use flat_fee_interface::{remove_lst_ix, RemoveLstIxArgs};
use flat_fee_lib::{
    account_resolvers::RemoveLstByMintFreeArgs, pda::FeeAccountFindPdaArgs, program::STATE_ID,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use test_utils::{banks_client_get_account, jitosol};

use crate::common::{normal_program_test, MockFeeAccountArgs, DEFAULT_PROGRAM_STATE};

#[tokio::test]
async fn remove_lst_basic() {
    let auth = Keypair::new();

    let mut program_state = DEFAULT_PROGRAM_STATE;
    program_state.manager = auth.pubkey();
    let program_test = normal_program_test(
        program_state,
        &[MockFeeAccountArgs {
            input_fee_bps: 1,
            output_fee_bps: 2,
            lst_mint: jitosol::ID,
        }],
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let free_args = RemoveLstByMintFreeArgs {
        refund_rent_to: payer.pubkey(),
        lst_mint: jitosol::ID,
        state_acc: KeyedReadonlyAccount {
            key: STATE_ID,
            account: banks_client_get_account(&mut banks_client, STATE_ID).await,
        },
    };
    let ix = remove_lst_ix(free_args.resolve().unwrap(), RemoveLstIxArgs {}).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &auth], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_lst_removed(&mut banks_client, jitosol::ID).await;
}

async fn verify_lst_removed(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let (addr, _bump) = FeeAccountFindPdaArgs { lst_mint }.get_fee_account_address_and_bump_seed();
    let deleted_fee_account = banks_client.get_account(addr).await.unwrap();
    assert!(deleted_fee_account.is_none());
}