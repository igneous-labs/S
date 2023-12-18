use async_trait::async_trait;
use borsh::BorshDeserialize;
use sanctum_solana_test_utils::{zero_padded_return_data, ExtendedBanksClient};
use solana_program::instruction::Instruction;
use solana_program_test::BanksClient;
use solana_sdk::{
    signature::Keypair, signer::Signer, transaction::Transaction,
    transaction_context::TransactionReturnData,
};

#[async_trait]
pub trait BorshReturnDataBanksClient {
    async fn exec_verify_borsh_return_data<
        T: BorshDeserialize + Send + Sync + PartialEq + std::fmt::Debug,
        const SER_LEN: usize,
    >(
        &mut self,
        payer: &Keypair,
        last_blockhash: solana_program::hash::Hash,
        ix: Instruction,
        expected_return: T,
    );
}

#[async_trait]
impl BorshReturnDataBanksClient for BanksClient {
    async fn exec_verify_borsh_return_data<
        T: BorshDeserialize + Send + Sync + PartialEq + std::fmt::Debug,
        const SER_LEN: usize,
    >(
        &mut self,
        payer: &Keypair,
        last_blockhash: solana_program::hash::Hash,
        ix: Instruction,
        expected_return: T,
    ) {
        let ix_program_id = ix.program_id;
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[payer], last_blockhash);
        let TransactionReturnData { program_id, data } = self.exec_get_return_data(tx).await;
        assert_eq!(program_id, ix_program_id);
        let buf: [u8; SER_LEN] = zero_padded_return_data(&data);
        let deser = T::deserialize(&mut buf.as_ref()).unwrap();
        assert_eq!(deser, expected_return);
    }
}
