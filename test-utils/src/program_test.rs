use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_sdk::account::Account;

pub async fn banks_client_get_account(banks_client: &mut BanksClient, pubkey: Pubkey) -> Account {
    banks_client.get_account(pubkey).await.unwrap().unwrap()
}
