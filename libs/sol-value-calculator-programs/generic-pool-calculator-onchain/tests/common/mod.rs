use generic_pool_calculator_lib::{utils::try_calculator_state_mut, CALCULATOR_STATE_SIZE};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub struct MockCalculatorStateAccountArgs {
    pub manager: Pubkey,
    pub last_upgrade_slot: u64,

    /// GenericPoolCalculator program ID
    pub owner: Pubkey,
}

pub fn mock_calculator_state_account(
    MockCalculatorStateAccountArgs {
        manager,
        last_upgrade_slot,
        owner,
    }: MockCalculatorStateAccountArgs,
) -> Account {
    let mut data = vec![0u8; CALCULATOR_STATE_SIZE];
    let state = try_calculator_state_mut(&mut data).unwrap();
    state.manager = manager;
    state.last_upgrade_slot = last_upgrade_slot;
    Account {
        lamports: 1_000_000_000, // just do 1 SOL lol
        data,
        owner,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
