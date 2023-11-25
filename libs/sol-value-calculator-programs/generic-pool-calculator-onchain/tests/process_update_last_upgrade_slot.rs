use generic_pool_calculator_interface::{update_last_upgrade_slot_ix, UpdateLastUpgradeSlotIxArgs};
use generic_pool_calculator_lib::{
    account_resolvers::UpdateLastUpgradeSlotRootAccounts, utils::try_calculator_state,
};
use generic_pool_calculator_test_utils::{
    mock_calculator_state_account, MockCalculatorStateAccountArgs,
};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_stake_pool_keys::spl_stake_pool_program;
use test_utils::{AddAccount, KeyedUiAccount, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT};

use crate::mock_calculator_program::MockCalculatorProgram;

mod mock_calculator_program {
    use generic_pool_calculator_interface::{
        UpdateLastUpgradeSlotAccounts, UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN,
    };
    use generic_pool_calculator_lib::GenericPoolSolValCalc;
    use generic_pool_calculator_onchain::processor::process_update_last_upgrade_slot_unchecked;
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
    use spl_stake_pool_keys::spl_stake_pool_program;

    sanctum_macros::declare_program_keys!(
        "8kbLzKfKo5gjbGQf2HmULGGTXQx6hnfYGJ8inL1zvVeL",
        [("state", b"state")]
    );

    pub struct MockCalculatorProgram;

    impl GenericPoolSolValCalc for MockCalculatorProgram {
        const POOL_PROGRAM_ID: Pubkey = spl_stake_pool_program::ID;
        const CALCULATOR_STATE_PDA: Pubkey = STATE_ID;
        const CALCULATOR_STATE_BUMP: u8 = STATE_BUMP;
        const ID: Pubkey = ID;
    }

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_arr: &[AccountInfo; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
            accounts.try_into().unwrap();
        let ix_accounts: UpdateLastUpgradeSlotAccounts = accounts_arr.into();
        process_update_last_upgrade_slot_unchecked(ix_accounts)
    }
}

#[tokio::test]
async fn update_last_upgrade_slot_basic() {
    const INITIAL_LAST_UPGRADE_SLOT: u64 = 69;
    let manager = Keypair::new();

    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "mock_calculator_program",
        mock_calculator_program::ID,
        processor!(mock_calculator_program::process_instruction),
    );
    let spl_stake_pool_prog = KeyedUiAccount::from_test_fixtures_file("spl-stake-pool-prog.json");
    let spl_stake_pool_prog_acc: Account = spl_stake_pool_prog.account.decode().unwrap();
    program_test = program_test
        .add_keyed_ui_account(spl_stake_pool_prog)
        .add_test_fixtures_account("spl-stake-pool-prog-data.json");

    let mock_state = mock_calculator_state_account(MockCalculatorStateAccountArgs {
        manager: manager.pubkey(),
        last_upgrade_slot: INITIAL_LAST_UPGRADE_SLOT,
        owner: mock_calculator_program::ID,
    });
    program_test.add_account(mock_calculator_program::STATE_ID, mock_state.clone());

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let root_accounts = UpdateLastUpgradeSlotRootAccounts {
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
        root_accounts.resolve::<MockCalculatorProgram>().unwrap(),
        UpdateLastUpgradeSlotIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], recent_blockhash);

    let state_account = banks_client
        .get_account(mock_calculator_program::STATE_ID)
        .await
        .unwrap()
        .unwrap();
    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();
    assert_eq!(calc_state.last_upgrade_slot, INITIAL_LAST_UPGRADE_SLOT);

    assert!(banks_client.process_transaction(tx).await.is_ok());

    let state_account = banks_client
        .get_account(mock_calculator_program::STATE_ID)
        .await
        .unwrap()
        .unwrap();
    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();

    assert_eq!(
        calc_state.last_upgrade_slot,
        SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT
    );
    assert_eq!(calc_state.manager, manager.pubkey());
}
