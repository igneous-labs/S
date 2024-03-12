use sanctum_associated_token_lib::FindAtaAddressArgs;
use sanctum_solana_test_utils::token::{tokenkeg::TokenkegProgramTest, MockTokenAccountArgs};
use sanctum_token_lib::MintWithTokenProgram;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::pubkey::Pubkey;

pub trait MiscProgramTest {
    // TODO: move this into sanctum-solana-test-utils
    fn add_ata(self, wallet: Pubkey, mwtp: MintWithTokenProgram, amount: u64) -> Self;

    // TODO: maybe factor out all the add_program()s into individual test utils crates
    fn add_s_program(self) -> Self;
}

impl MiscProgramTest for ProgramTest {
    fn add_ata(
        self,
        wallet: Pubkey,
        MintWithTokenProgram {
            pubkey: mint,
            token_program,
        }: MintWithTokenProgram,
        amount: u64,
    ) -> Self {
        let ata_addr = FindAtaAddressArgs {
            wallet,
            mint,
            token_program,
        }
        .find_ata_address()
        .0;
        // TODO: handle token-22, no LSTs use that yet
        self.add_tokenkeg_account_from_args(
            ata_addr,
            MockTokenAccountArgs {
                mint,
                authority: wallet,
                amount,
            },
        )
    }

    fn add_s_program(mut self) -> Self {
        self.add_program(
            "s_controller",
            s_controller_lib::program::ID,
            processor!(s_controller::entrypoint::process_instruction),
        );
        self
    }
}
