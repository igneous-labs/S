use s_controller_interface::LstState;
use s_controller_lib::{try_lst_state_list_mut, LST_STATE_SIZE};
use sanctum_utils::associated_token::{find_ata_address, FindAtaAddressArgs};
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;
use test_utils::{est_rent_exempt_lamports, mock_token_account, MockTokenAccountArgs};

use super::banks_client_get_account;

/// TODO: add protocol fee accumulator amt if required
#[derive(Clone, Copy, Debug)]
pub struct MockLstStateArgs {
    pub mint: Pubkey,
    pub sol_value: u64,
    pub reserves_amt: u64,
}

/// TODO: add protocol fee accumulator account if required
#[derive(Clone, Debug)]
pub struct MockLstStateRet {
    pub lst_state: LstState,
    pub reserves_address: Pubkey,
    pub reserves_account: Account,
}

/// Assumes LST uses original spl-token program
pub fn mock_lst_state(
    MockLstStateArgs {
        mint,
        sol_value,
        reserves_amt,
    }: MockLstStateArgs,
) -> MockLstStateRet {
    let (reserves_address, reserves_bump) = find_ata_address(FindAtaAddressArgs {
        wallet: s_controller_lib::program::STATE_ID,
        mint,
        token_program: spl_token::ID,
    });
    let (_, protocol_fee_accumulator_bump) = find_ata_address(FindAtaAddressArgs {
        wallet: s_controller_lib::program::PROTOCOL_FEE_ID,
        mint,
        token_program: spl_token::ID,
    });
    let lst_state = LstState {
        mint,
        sol_value,
        is_input_disabled: 0,
        reserves_bump,
        protocol_fee_accumulator_bump,
        padding: Default::default(),
        sol_value_calculator: spl_calculator_lib::program::ID,
    };
    let reserves_account = mock_token_account(MockTokenAccountArgs {
        mint,
        authority: s_controller_lib::program::STATE_ID,
        amount: reserves_amt,
        is_native: mint == spl_token::native_mint::ID,
    });
    MockLstStateRet {
        lst_state,
        reserves_address,
        reserves_account,
    }
}

pub const fn lst_state_list_rent_exempt_lamports(lst_state_list: &[LstState]) -> u64 {
    est_rent_exempt_lamports(lst_state_list.len() * LST_STATE_SIZE)
}

pub fn program_test_add_lst_state_list(
    mut program_test: ProgramTest,
    lst_states: &[LstState],
) -> ProgramTest {
    let mut data = vec![0u8; lst_states.len() * LST_STATE_SIZE];
    let lst_state_list = try_lst_state_list_mut(&mut data).unwrap();
    lst_state_list.copy_from_slice(lst_states);

    let account = Account {
        data,
        lamports: lst_state_list_rent_exempt_lamports(lst_states),
        owner: s_controller_lib::program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    };

    program_test.add_account(s_controller_lib::program::LST_STATE_LIST_ID, account);
    program_test
}

pub fn program_test_add_mock_lst_states(
    mut program_test: ProgramTest,
    args: &[MockLstStateArgs],
) -> ProgramTest {
    let mut lst_states = Vec::new();
    for arg in args {
        let MockLstStateRet {
            lst_state,
            reserves_address,
            reserves_account,
        } = mock_lst_state(*arg);
        program_test.add_account(reserves_address, reserves_account);
        lst_states.push(lst_state);
    }
    program_test_add_lst_state_list(program_test, &lst_states)
}

pub async fn banks_client_get_lst_state_list_acc(banks_client: &mut BanksClient) -> Account {
    banks_client_get_account(banks_client, s_controller_lib::program::LST_STATE_LIST_ID).await
}
