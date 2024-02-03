use s_controller_interface::PoolState;
use s_controller_lib::{find_pool_state_address, try_pool_state_mut, POOL_STATE_SIZE};
use sanctum_solana_test_utils::ExtendedProgramTest;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::account::Account;

pub trait SctrProgramTest {
    fn add_s_program(self) -> Self;

    fn add_pool_state(self, pool_state: PoolState) -> Self;
}

impl SctrProgramTest for ProgramTest {
    fn add_s_program(mut self) -> Self {
        self.add_program(
            "s_controller",
            s_controller_lib::program::ID,
            processor!(s_controller::entrypoint::process_instruction),
        );
        self
    }

    fn add_pool_state(self, pool_state: PoolState) -> Self {
        let pool_state_pda = find_pool_state_address(s_controller_lib::program::ID).0;
        // TODO: move MockPoolState from programs/s-controller/tests/common/state.rs into a test-util and use the type
        let mut data = vec![0u8; POOL_STATE_SIZE];
        let dst = try_pool_state_mut(&mut data).unwrap();
        *dst = pool_state;
        let pool_state_acc = Account {
            lamports: 1_000_000_000,
            data,
            owner: s_controller_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        };
        self.add_account_chained(pool_state_pda, pool_state_acc)
    }
}
