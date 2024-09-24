use async_trait::async_trait;
use jupiter_amm_interface::{Amm, AmmContext, ClockRef, KeyedAccount};
use s_jup_interface::{SPoolInitKeys, SPoolJup};
use sanctum_solana_test_utils::ExtendedBanksClient;
use solana_program_test::BanksClient;
use solana_sdk::{clock::Clock, pubkey::Pubkey, sysvar::clock};
use std::collections::{HashMap, HashSet};

pub async fn fully_init_amm(bc: &mut BanksClient, program_id: Pubkey) -> SPoolJup {
    let SPoolInitKeys { lst_state_list, .. } = SPoolJup::init_keys(program_id);
    let lst_state_list_acc = bc.get_account_unwrapped(lst_state_list).await;
    let clock: Clock = bincode::deserialize(&bc.get_account_data(clock::ID).await).unwrap();
    SPoolJup::from_keyed_account(
        &KeyedAccount {
            key: lst_state_list,
            account: lst_state_list_acc,
            params: Some(serde_json::Value::String(program_id.to_string())),
        },
        &AmmContext {
            clock_ref: ClockRef::from(clock),
        },
    )
    .unwrap()
    .initial_update(bc)
    .await
}

#[async_trait]
pub trait UpdatingSPoolJup {
    async fn update_with_banks(self, bc: &mut BanksClient) -> Self;

    // Run the initial 2x update to fully initialize the Amm
    async fn initial_update(self, bc: &mut BanksClient) -> Self;
}

#[async_trait]
impl UpdatingSPoolJup for SPoolJup {
    async fn update_with_banks(mut self, bc: &mut BanksClient) -> Self {
        let accounts =
            self.get_accounts_to_update()
                .into_iter()
                .fold(HashSet::new(), |mut hs, pk| {
                    hs.insert(pk);
                    hs
                });
        let mut fetched = HashMap::new();
        for pk in accounts {
            fetched.insert(pk, bc.get_account_unwrapped(pk).await);
        }
        self.update(&fetched).unwrap();
        self
    }

    async fn initial_update(self, bc: &mut BanksClient) -> Self {
        self.update_with_banks(bc).await.update_with_banks(bc).await
    }
}
