use generic_pool_calculator_lib::{
    pda::CalculatorStateFindPdaArgs, utils::try_calculator_state_mut, CALCULATOR_STATE_SIZE,
};
use sanctum_solana_test_utils::{ExtendedProgramTest, IntoAccount};
use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, pubkey::Pubkey};

pub struct MockCalculatorStateAccountArgs {
    pub manager: Pubkey,
    pub last_upgrade_slot: u64,

    /// GenericPoolCalculator program ID
    pub owner: Pubkey,
}

impl IntoAccount for MockCalculatorStateAccountArgs {
    fn into_account(self) -> Account {
        let Self {
            manager,
            last_upgrade_slot,
            owner,
        } = self;
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
}

pub trait GenericPoolCalculatorProgramTest {
    fn add_mock_calculator_state(self, args: MockCalculatorStateAccountArgs) -> Self;
}

impl GenericPoolCalculatorProgramTest for ProgramTest {
    fn add_mock_calculator_state(self, args: MockCalculatorStateAccountArgs) -> Self {
        let (addr, _bump) = CalculatorStateFindPdaArgs {
            program_id: args.owner,
        }
        .get_calculator_state_address_and_bump_seed();
        self.add_account_chained(addr, args.into_account())
    }
}
