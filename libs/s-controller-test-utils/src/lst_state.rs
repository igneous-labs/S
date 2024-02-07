use async_trait::async_trait;
use s_controller_interface::LstState;
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, try_find_lst_mint_on_list,
    try_lst_state_list, try_lst_state_list_mut, FindLstPdaAtaKeys, LST_STATE_SIZE,
};
use sanctum_solana_test_utils::{
    est_rent_exempt_lamports,
    token::{tokenkeg::mock_tokenkeg_account, MockTokenAccountArgs},
    ExtendedBanksClient, IntoAccount,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;

#[derive(Clone, Copy, Debug)]
pub struct MockLstStateArgs {
    pub mint: Pubkey,
    pub sol_value_calculator: Pubkey,
    pub token_program: Pubkey,
    pub sol_value: u64,
    pub reserves_amt: u64,
    pub protocol_fee_accumulator_amt: u64,
    pub is_input_disabled: bool,
}

#[derive(Clone, Debug)]
pub struct MockLstStateRet {
    pub lst_state: LstState,
    pub reserves_address: Pubkey,
    pub reserves_account: Account,
    pub protocol_fee_accumulator_address: Pubkey,
    pub protocol_fee_accumulator_account: Account,
}

/// Assumes LST uses original spl-token program
pub fn mock_lst_state(
    MockLstStateArgs {
        mint,
        sol_value_calculator,
        token_program,
        sol_value,
        reserves_amt,
        protocol_fee_accumulator_amt,
        is_input_disabled,
    }: MockLstStateArgs,
) -> MockLstStateRet {
    let find_keys = FindLstPdaAtaKeys {
        lst_mint: mint,
        token_program,
    };
    let (reserves_address, pool_reserves_bump) = find_pool_reserves_address(find_keys);
    let (protocol_fee_accumulator_address, protocol_fee_accumulator_bump) =
        find_protocol_fee_accumulator_address(find_keys);
    let lst_state = LstState {
        mint,
        sol_value,
        is_input_disabled: is_input_disabled.into(),
        pool_reserves_bump,
        protocol_fee_accumulator_bump,
        padding: Default::default(),
        sol_value_calculator,
    };
    let reserves_account = mock_tokenkeg_account(MockTokenAccountArgs {
        mint,
        authority: s_controller_lib::program::POOL_STATE_ID,
        amount: reserves_amt,
    });
    let protocol_fee_accumulator_account = mock_tokenkeg_account(MockTokenAccountArgs {
        mint,
        authority: s_controller_lib::program::PROTOCOL_FEE_ID,
        amount: protocol_fee_accumulator_amt,
    });
    MockLstStateRet {
        lst_state,
        reserves_address,
        reserves_account: reserves_account.into_account(),
        protocol_fee_accumulator_address,
        protocol_fee_accumulator_account: protocol_fee_accumulator_account.into_account(),
    }
}

pub const fn lst_state_list_rent_exempt_lamports(lst_state_list: &[LstState]) -> u64 {
    est_rent_exempt_lamports(lst_state_list.len() * LST_STATE_SIZE)
}

pub trait LstStateListProgramTest {
    fn add_lst_state_list(self, lst_states: &[LstState]) -> Self;

    fn add_mock_lst_states(self, args: &[MockLstStateArgs]) -> Self;
}

impl LstStateListProgramTest for ProgramTest {
    fn add_lst_state_list(mut self, lst_states: &[LstState]) -> Self {
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

        self.add_account(s_controller_lib::program::LST_STATE_LIST_ID, account);
        self
    }

    fn add_mock_lst_states(mut self, args: &[MockLstStateArgs]) -> Self {
        let mut lst_states = Vec::new();
        for arg in args {
            let MockLstStateRet {
                lst_state,
                reserves_address,
                reserves_account,
                protocol_fee_accumulator_address,
                protocol_fee_accumulator_account,
            } = mock_lst_state(*arg);
            self.add_account(reserves_address, reserves_account);
            self.add_account(
                protocol_fee_accumulator_address,
                protocol_fee_accumulator_account,
            );
            lst_states.push(lst_state);
        }
        self.add_lst_state_list(&lst_states)
    }
}

// TODO: _for_prog() counterparts
#[async_trait]
pub trait LstStateListBanksClient {
    async fn get_lst_state_list_acc(&mut self) -> Account;

    async fn get_lst_state(&mut self, lst_mint: Pubkey) -> LstState {
        let lst_state_list_acc = self.get_lst_state_list_acc().await;
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        let (_i, lst_state) = try_find_lst_mint_on_list(lst_mint, lst_state_list).unwrap();
        *lst_state
    }
}

#[async_trait]
impl LstStateListBanksClient for BanksClient {
    async fn get_lst_state_list_acc(&mut self) -> Account {
        self.get_account_unwrapped(s_controller_lib::program::LST_STATE_LIST_ID)
            .await
    }
}
