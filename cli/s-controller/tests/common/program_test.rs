use solana_program_test::{processor, ProgramTest};

pub trait SctrProgramTest {
    fn add_s_program(self) -> Self;
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
}
