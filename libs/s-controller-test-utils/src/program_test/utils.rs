use generic_pool_calculator_lib::GenericPoolSolValCalc;
use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use lido_calculator_lib::LidoSolValCalc;
use marinade_calculator_lib::MarinadeSolValCalc;
use sanctum_solana_test_utils::{
    token::{tokenkeg::mock_tokenkeg_account, MockTokenAccountArgs},
    ExtendedProgramTest, IntoAccount,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use spl_calculator_lib::SplSolValCalc;
use test_utils::{
    LIDO_PROG_LAST_UPDATED_SLOT, MARINADE_PROG_LAST_UPDATED_SLOT,
    SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
};

pub trait AddSplProgramTest {
    fn add_spl_progs(self) -> Self;

    fn add_jito_stake_pool(self) -> Self;
}

impl AddSplProgramTest for ProgramTest {
    fn add_spl_progs(mut self) -> Self {
        // name must match <name>.so filename
        self.add_program(
            "spl_calculator",
            spl_calculator_lib::program::ID,
            processor!(spl_calculator::entrypoint::process_instruction),
        );
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
            owner: SplSolValCalc::ID,
        })
        .add_test_fixtures_account("spl-stake-pool-prog.json")
        .add_test_fixtures_account("spl-stake-pool-prog-data.json")
    }

    fn add_jito_stake_pool(self) -> Self {
        self.add_test_fixtures_account("jito-stake-pool.json")
            .add_test_fixtures_account("jitosol-mint.json")
    }
}

pub trait AddMarinadeProgramTest {
    fn add_marinade_progs(self) -> Self;

    fn add_marinade_stake_pool(self) -> Self;
}

impl AddMarinadeProgramTest for ProgramTest {
    fn add_marinade_progs(mut self) -> Self {
        // name must match <name>.so filename
        self.add_program(
            "marinade_calculator",
            marinade_calculator_lib::program::ID,
            processor!(marinade_calculator::entrypoint::process_instruction),
        );
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: MARINADE_PROG_LAST_UPDATED_SLOT,
            owner: MarinadeSolValCalc::ID,
        })
        .add_test_fixtures_account("marinade-prog.json")
        .add_test_fixtures_account("marinade-prog-data.json")
    }

    fn add_marinade_stake_pool(self) -> Self {
        self.add_test_fixtures_account("marinade-state.json")
            .add_test_fixtures_account("msol-mint.json")
    }
}

pub trait AddLidoProgramTest {
    fn add_lido_progs(self) -> Self;

    fn add_lido_stake_pool(self) -> Self;
}

impl AddLidoProgramTest for ProgramTest {
    fn add_lido_progs(mut self) -> Self {
        // name must match <name>.so filename
        self.add_program(
            "lido_calculator",
            lido_calculator_lib::program::ID,
            processor!(lido_calculator::entrypoint::process_instruction),
        );
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: LIDO_PROG_LAST_UPDATED_SLOT,
            owner: LidoSolValCalc::ID,
        })
        .add_test_fixtures_account("lido-prog.json")
        .add_test_fixtures_account("lido-prog-data.json")
    }

    fn add_lido_stake_pool(self) -> Self {
        self.add_test_fixtures_account("lido-state.json")
            .add_test_fixtures_account("stsol-mint.json")
    }
}

pub trait GenAndAddTokenAccountProgramTest {
    fn gen_and_add_token_account(&mut self, args: MockTokenAccountArgs) -> Pubkey;
}

impl GenAndAddTokenAccountProgramTest for ProgramTest {
    fn gen_and_add_token_account(&mut self, args: MockTokenAccountArgs) -> Pubkey {
        let addr = Pubkey::new_unique();
        let token_acc = mock_tokenkeg_account(args);
        self.add_account(addr, token_acc.into_account());
        addr
    }
}
