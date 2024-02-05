use solana_program_test::{processor, ProgramTest};

pub trait SControllerProgramTest {
    fn add_s_controller_prog(self) -> Self;
}

impl SControllerProgramTest for ProgramTest {
    fn add_s_controller_prog(mut self) -> Self {
        self.add_program(
            "s_controller",
            s_controller_lib::program::ID,
            processor!(s_controller::entrypoint::process_instruction),
        );
        self
    }
}
