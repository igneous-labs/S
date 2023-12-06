use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use test_utils::banks_client_get_account;

pub async fn banks_client_get_disable_pool_list_acc(banks_client: &mut BanksClient) -> Account {
    banks_client_get_account(
        banks_client,
        s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID,
    )
    .await
}
