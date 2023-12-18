use async_trait::async_trait;
use flat_fee_interface::ProgramState;
use flat_fee_lib::{
    initial_constants::{initial_manager, INITIAL_LP_WITHDRAWAL_FEE_BPS},
    program::STATE_SIZE,
    utils::try_program_state_mut,
};
use sanctum_solana_test_utils::{est_rent_exempt_lamports, ExtendedBanksClient, IntoAccount};
use solana_program_test::BanksClient;
use solana_sdk::account::Account;

pub const DEFAULT_PROGRAM_STATE: ProgramState = ProgramState {
    manager: initial_manager::ID,
    lp_withdrawal_fee_bps: INITIAL_LP_WITHDRAWAL_FEE_BPS,
};

pub struct MockProgramState(pub ProgramState);

impl IntoAccount for MockProgramState {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; STATE_SIZE];
        let dst = try_program_state_mut(&mut data).unwrap();
        *dst = self.0;
        Account {
            lamports: est_rent_exempt_lamports(STATE_SIZE),
            data,
            owner: flat_fee_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

#[async_trait]
pub trait FlatFeePricingProgramTestBanksClient {
    async fn get_flat_fee_program_state(&mut self) -> Account;
}

#[async_trait]
impl FlatFeePricingProgramTestBanksClient for BanksClient {
    async fn get_flat_fee_program_state(&mut self) -> Account {
        self.get_account_unwrapped(flat_fee_lib::program::STATE_ID)
            .await
    }
}
