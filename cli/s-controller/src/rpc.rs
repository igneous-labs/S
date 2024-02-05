use s_controller_lib::find_pool_state_address;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};

pub async fn fetch_pool_state(rpc: &RpcClient, program_id: Pubkey) -> Account {
    rpc.get_account(&find_pool_state_address(program_id).0)
        .await
        .unwrap()
}