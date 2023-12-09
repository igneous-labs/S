use generic_pool_calculator_interface::{
    update_last_upgrade_slot_ix, UpdateLastUpgradeSlotIxArgs, UpdateLastUpgradeSlotKeys,
};
use generic_pool_calculator_lib::{
    account_resolvers::UpdateLastUpgradeSlotFreeArgs, utils::try_calculator_state,
};
use generic_pool_calculator_test_utils::{
    mock_calculator_state_account, MockCalculatorStateAccountArgs,
};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_stake_pool_keys::{spl_stake_pool_program, spl_stake_pool_program_progdata};
use test_utils::{
    assert_program_error, banks_client_get_account, AddAccount, KeyedUiAccount,
    SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
};

use mock_calculator_program::MockCalculatorProgram;

mod mock_calculator_program {
    use generic_pool_calculator_lib::GenericPoolSolValCalc;
    use generic_pool_calculator_onchain::processor::{
        process_update_last_upgrade_slot_unchecked, verify_update_last_upgrade_slot,
    };
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
    use spl_stake_pool_keys::{spl_stake_pool_program, spl_stake_pool_program_progdata};

    sanctum_macros::declare_program_keys!(
        "8kbLzKfKo5gjbGQf2HmULGGTXQx6hnfYGJ8inL1zvVeL",
        [("state", b"state")]
    );

    pub struct MockCalculatorProgram;

    impl GenericPoolSolValCalc for MockCalculatorProgram {
        const POOL_PROGRAM_ID: Pubkey = spl_stake_pool_program::ID;
        const POOL_PROGRAM_PROGDATA_ID: Pubkey = spl_stake_pool_program_progdata::ID;
        const CALCULATOR_STATE_PDA: Pubkey = STATE_ID;
        const CALCULATOR_STATE_BUMP: u8 = STATE_BUMP;
        const ID: Pubkey = ID;
    }

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let checked = verify_update_last_upgrade_slot::<MockCalculatorProgram>(accounts)?;
        process_update_last_upgrade_slot_unchecked(checked)
    }
}

const INITIAL_LAST_UPGRADE_SLOT: u64 = 69;

fn mock_prog_program_test_with_spl(manager: Pubkey) -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "mock_calculator_program",
        mock_calculator_program::ID,
        processor!(mock_calculator_program::process_instruction),
    );
    let spl_stake_pool_prog = KeyedUiAccount::from_test_fixtures_file("spl-stake-pool-prog.json");
    program_test = program_test
        .add_keyed_ui_account(spl_stake_pool_prog)
        .add_test_fixtures_account("spl-stake-pool-prog-data.json");

    let mock_state = mock_calculator_state_account(MockCalculatorStateAccountArgs {
        manager,
        last_upgrade_slot: INITIAL_LAST_UPGRADE_SLOT,
        owner: mock_calculator_program::ID,
    });
    program_test.add_account(mock_calculator_program::STATE_ID, mock_state);

    program_test
}

async fn verify_last_upgrade_slot(banks_client: &mut BanksClient, expected_last_upgrade_slot: u64) {
    let state_account =
        banks_client_get_account(banks_client, mock_calculator_program::STATE_ID).await;
    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();
    assert_eq!(calc_state.last_upgrade_slot, expected_last_upgrade_slot);
}

#[tokio::test]
async fn update_last_upgrade_slot_success() {
    let manager = Keypair::new();

    let program_test = mock_prog_program_test_with_spl(manager.pubkey());

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mock_state =
        banks_client_get_account(&mut banks_client, mock_calculator_program::STATE_ID).await;
    let spl_stake_pool_prog_acc =
        banks_client_get_account(&mut banks_client, spl_stake_pool_program::ID).await;

    verify_last_upgrade_slot(&mut banks_client, INITIAL_LAST_UPGRADE_SLOT).await;

    let free_args = UpdateLastUpgradeSlotFreeArgs {
        state: KeyedReadonlyAccount {
            key: mock_calculator_program::STATE_ID,
            account: mock_state,
        },
        pool_program: KeyedReadonlyAccount {
            key: spl_stake_pool_program::ID,
            account: spl_stake_pool_prog_acc,
        },
    };
    let mut ix = update_last_upgrade_slot_ix(
        free_args.resolve::<MockCalculatorProgram>().unwrap(),
        UpdateLastUpgradeSlotIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    verify_last_upgrade_slot(&mut banks_client, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT).await;
}

#[tokio::test]
async fn update_last_upgrade_slot_unauthorized() {
    let manager = Pubkey::new_unique();

    let program_test = mock_prog_program_test_with_spl(manager);

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    verify_last_upgrade_slot(&mut banks_client, INITIAL_LAST_UPGRADE_SLOT).await;

    let mut ix = update_last_upgrade_slot_ix(
        UpdateLastUpgradeSlotKeys {
            manager: payer.pubkey(),
            state: mock_calculator_program::STATE_ID,
            pool_program: spl_stake_pool_program::ID,
            pool_program_data: spl_stake_pool_program_progdata::ID,
        },
        UpdateLastUpgradeSlotIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::InvalidArgument);

    verify_last_upgrade_slot(&mut banks_client, INITIAL_LAST_UPGRADE_SLOT).await;
}

#[tokio::test]
async fn fail_update_last_upgrade_slot_missing_signature() {
    let manager = Pubkey::new_unique();

    let program_test = mock_prog_program_test_with_spl(manager);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    verify_last_upgrade_slot(&mut banks_client, INITIAL_LAST_UPGRADE_SLOT).await;

    let mut ix = update_last_upgrade_slot_ix(
        UpdateLastUpgradeSlotKeys {
            manager,
            state: mock_calculator_program::STATE_ID,
            pool_program: spl_stake_pool_program::ID,
            pool_program_data: spl_stake_pool_program_progdata::ID,
        },
        UpdateLastUpgradeSlotIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;
    ix.accounts[0].is_signer = false;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::MissingRequiredSignature);

    verify_last_upgrade_slot(&mut banks_client, INITIAL_LAST_UPGRADE_SLOT).await;
}
