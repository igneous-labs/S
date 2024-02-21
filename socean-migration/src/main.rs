//! CLI to run the migration program

mod cli;

pub fn main() {
    #[cfg(feature = "cli")]
    cli::main();
}
