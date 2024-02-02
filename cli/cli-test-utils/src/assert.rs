use solana_program_test::{BanksClientError, BanksTransactionResultWithMetadata};

pub fn assert_all_txs_success_nonempty(
    exec_res: &[Result<BanksTransactionResultWithMetadata, BanksClientError>],
) {
    if exec_res.is_empty() {
        panic!("exec_res is empty");
    }
    for res in exec_res {
        res.as_ref().unwrap().result.as_ref().unwrap();
    }
}
