use socean_migration::migrate_ix;
use solana_sdk::{signer::Signer, transaction::Transaction};

use crate::common::base_program_test;

#[tokio::test]
async fn migrate_success() {
    let (pt, migrate_auth) = base_program_test();

    let ctx = pt.start_with_context().await;
    let mut bc = ctx.banks_client;
    let last_blockhash = ctx.last_blockhash;

    let ix = migrate_ix();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&migrate_auth.pubkey()));
    tx.sign(&[&migrate_auth], last_blockhash);

    bc.process_transaction(tx).await.unwrap();
}
