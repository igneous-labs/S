use solana_program_test::{BanksClientError, BanksTransactionResultWithMetadata};

pub fn assert_all_txs_success(
    exec_res: &[Result<BanksTransactionResultWithMetadata, BanksClientError>],
) {
    for res in exec_res {
        res.as_ref().unwrap().result.as_ref().unwrap();
    }
}
