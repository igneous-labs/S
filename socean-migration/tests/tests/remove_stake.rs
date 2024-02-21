use bincode::deserialize;
use socean_migration::{remove_stake_ix, socean_laine_vsa};
use solana_program::stake::state::StakeStateV2;
use solana_program_test::BanksClient;
use solana_sdk::{
    account::ReadableAccount, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};

use crate::common::base_program_test;

async fn verify_vsa_withdrawer(banks_client: &mut BanksClient, vsa: Pubkey, auth: Pubkey) {
    let vsa_acc = banks_client.get_account(vsa).await.unwrap().unwrap();
    let data = vsa_acc.data();
    let stake_state: StakeStateV2 = deserialize(data).unwrap();
    let authorized = stake_state.authorized().unwrap();
    assert_eq!(authorized.withdrawer, auth);
}

#[tokio::test]
async fn remove_stake_success() {
    let (pt, migrate_auth) = base_program_test();

    let ctx = pt.start_with_context().await;
    let mut bc = ctx.banks_client;
    let last_blockhash = ctx.last_blockhash;

    // NB: DO NOT REMOVE LAINE VSA WHEN ACTUALLY RUNNING THE MIGRATION
    let ix = remove_stake_ix(socean_laine_vsa::ID);
    let mut tx = Transaction::new_with_payer(&[ix], Some(&migrate_auth.pubkey()));
    tx.sign(&[&migrate_auth], last_blockhash);

    bc.process_transaction(tx).await.unwrap();

    verify_vsa_withdrawer(&mut bc, socean_laine_vsa::ID, migrate_auth.pubkey()).await;
}
