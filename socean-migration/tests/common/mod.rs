use std::io::Write;

use s_controller_test_utils::AddSplProgramTest;
use sanctum_solana_test_utils::{test_fixtures_dir, ExtendedProgramTest};
use solana_program_test::{find_file, read_file, ProgramTest};
use solana_sdk::{
    account::Account,
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    pubkey::Pubkey,
    rent::Rent,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

/// TODO: add this fn to sanctum-solana-test-utils
/// ProgramTest::add_program() sets the owner to bpf_loader instead of
/// bpf_loader_upgradeable:
/// https://docs.rs/solana-program-test/latest/src/solana_program_test/lib.rs.html#657
fn add_upgradeable_program(
    pt: &mut ProgramTest,
    program_id: Pubkey,
    program_name: &str,
    upgrade_auth_addr: Option<Pubkey>,
    last_upgrade_slot: u64,
) {
    let so_file = format!("{program_name}.so");
    let pb = find_file(&so_file).unwrap_or_else(|| panic!("{so_file} not found"));
    let so_prog_data = read_file(pb);
    let (prog_data_addr, _bump) =
        Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::ID);
    let mut prog_acc_data = Vec::with_capacity(UpgradeableLoaderState::size_of_program());
    prog_acc_data.write_all(&2u32.to_le_bytes()).unwrap();
    prog_acc_data.write_all(prog_data_addr.as_ref()).unwrap();
    pt.add_account(
        program_id,
        Account {
            lamports: Rent::default().minimum_balance(UpgradeableLoaderState::size_of_program()),
            data: prog_acc_data,
            owner: bpf_loader_upgradeable::ID,
            executable: true,
            rent_epoch: u64::MAX,
        },
    );
    let mut prog_data_acc_data = Vec::with_capacity(
        UpgradeableLoaderState::size_of_programdata_metadata() + so_prog_data.len(),
    );
    prog_data_acc_data.write_all(&3u32.to_le_bytes()).unwrap();
    prog_data_acc_data
        .write_all(&last_upgrade_slot.to_le_bytes())
        .unwrap();
    match upgrade_auth_addr {
        Some(auth) => {
            prog_data_acc_data.write_all(&[1u8]).unwrap();
            prog_data_acc_data.write_all(auth.as_ref()).unwrap();
        }
        None => {
            prog_data_acc_data.write_all(&[0u8; 33]).unwrap();
        }
    }
    prog_data_acc_data.write_all(&so_prog_data).unwrap();
    pt.add_account(
        prog_data_addr,
        Account {
            lamports: Rent::default().minimum_balance(prog_data_acc_data.len()),
            data: prog_data_acc_data,
            owner: bpf_loader_upgradeable::ID,
            executable: false,
            rent_epoch: u64::MAX,
        },
    );
}

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
    add_upgradeable_program(
        &mut pt,
        s_controller_lib::program::ID,
        "socean_migration",
        Some(Pubkey::new_unique()), // dont care, since we will overwrite this acc via ProgramTestContext ltr
        0,
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
