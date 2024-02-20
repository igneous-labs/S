use s_controller_test_utils::AddSplProgramTest;
use sanctum_solana_test_utils::{test_fixtures_dir, ExtendedProgramTest};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

/// Creates a ProgramTest with all accounts required for final migration:
/// - spl-stake-pool prog + spl-sol-value-calculator prog + state to facilitate testing of SyncSolValue after
/// - laine stake pool accounts - lainesol mint, pool, validator list, reserves, lainesol fee dest, vsa
/// - socean stake pool accounts - scnsol mint, socean stake pool, validator list, laine VSA
/// - metaplex program
/// - metadata PDA
/// - testing migrate auth
/// - sets migration program
pub fn base_program_test() -> (ProgramTest, Keypair) {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let mut pt = ProgramTest::default();
    pt.add_program(
        "socean_migration",
        s_controller_lib::program::ID,
        processor!(socean_migration::process_instruction),
    );
    (
        pt.add_spl_progs()
            .add_test_fixtures_account("lainesol-mint.json")
            .add_test_fixtures_account("lainesol-pool.json")
            .add_test_fixtures_account("lainesol-list.json")
            .add_test_fixtures_account("lainesol-reserves.json")
            .add_test_fixtures_account("lainesol-fee-dest.json")
            .add_test_fixtures_account("lainesol-vsa.json")
            .add_test_fixtures_account("scnsol-mint.json")
            .add_test_fixtures_account("socean-pool.json")
            .add_test_fixtures_account("socean-list.json")
            .add_test_fixtures_account("socean-laine-vsa.json")
            .add_test_fixtures_account("token-metadata-prog.json")
            .add_test_fixtures_account("token-metadata-prog-data.json")
            .add_test_fixtures_account("scnsol-metadata.json")
            .add_system_account(mock_auth_kp.pubkey(), 100_000_000), // 0.1 SOL
        mock_auth_kp,
    )
}
