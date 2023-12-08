use flat_fee_interface::ProgramState;
use flat_fee_lib::{
    initial_constants::{initial_manager, INITIAL_LP_WITHDRAWAL_FEE_BPS},
    program::STATE_SIZE,
    utils::try_program_state_mut,
};
use solana_program_test::BanksClient;
use solana_sdk::account::Account;
use test_utils::{banks_client_get_account, est_rent_exempt_lamports};

pub const DEFAULT_PROGRAM_STATE: ProgramState = ProgramState {
    manager: initial_manager::ID,
    lp_withdrawal_fee_bps: INITIAL_LP_WITHDRAWAL_FEE_BPS,
};

pub fn program_state_to_account(state: ProgramState) -> Account {
    let mut data = vec![0u8; STATE_SIZE];
    let dst = try_program_state_mut(&mut data).unwrap();
    *dst = state;
    Account {
        lamports: est_rent_exempt_lamports(STATE_SIZE),
        data,
        owner: flat_fee_lib::program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub async fn banks_client_get_program_state_acc(banks_client: &mut BanksClient) -> Account {
    banks_client_get_account(banks_client, flat_fee_lib::program::STATE_ID).await
}
