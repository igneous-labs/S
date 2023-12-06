use s_controller_lib::try_disable_pool_authority_list_mut;
use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;
use test_utils::{banks_client_get_account, est_rent_exempt_lamports};

pub async fn banks_client_get_disable_pool_list_acc(banks_client: &mut BanksClient) -> Account {
    banks_client_get_account(
        banks_client,
        s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID,
    )
    .await
}

pub const fn disable_pool_authority_list_rent_exempt_lamports(
    disable_pool_authority_list: &[Pubkey],
) -> u64 {
    est_rent_exempt_lamports(disable_pool_authority_list.len() * PUBKEY_BYTES)
}

pub fn program_test_add_disable_pool_authority_list(
    mut program_test: ProgramTest,
    disable_pool_authorities: &[Pubkey],
) -> ProgramTest {
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

    program_test.add_account(
        s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID,
        account,
    );
    program_test
}
