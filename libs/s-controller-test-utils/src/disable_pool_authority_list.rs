use async_trait::async_trait;
use s_controller_lib::try_disable_pool_authority_list_mut;
use sanctum_solana_test_utils::{
    est_rent_exempt_lamports, ExtendedBanksClient, ExtendedProgramTest,
};
use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;

#[async_trait]
pub trait DisablePoolAuthorityListBanksClient {
    async fn get_disable_pool_list_acc(&mut self) -> Account;
}

#[async_trait]
impl DisablePoolAuthorityListBanksClient for BanksClient {
    async fn get_disable_pool_list_acc(&mut self) -> Account {
        self.get_account_unwrapped(s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID)
            .await
    }
}

pub const fn disable_pool_authority_list_rent_exempt_lamports(
    disable_pool_authority_list: &[Pubkey],
) -> u64 {
    est_rent_exempt_lamports(disable_pool_authority_list.len() * PUBKEY_BYTES)
}

pub trait DisablePoolAuthorityListProgramTest {
    fn add_disable_pool_authority_list(self, disable_pool_authorities: &[Pubkey]) -> Self;
}

impl DisablePoolAuthorityListProgramTest for ProgramTest {
    fn add_disable_pool_authority_list(self, disable_pool_authorities: &[Pubkey]) -> Self {
        let mut data = vec![0u8; disable_pool_authorities.len() * PUBKEY_BYTES];
        let disable_pool_authority_list = try_disable_pool_authority_list_mut(&mut data).unwrap();
        disable_pool_authority_list.copy_from_slice(disable_pool_authorities);

        let account = Account {
            data,
            lamports: disable_pool_authority_list_rent_exempt_lamports(disable_pool_authorities),
            owner: s_controller_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        };

        self.add_account_chained(
            s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID,
            account,
        )
    }
}
