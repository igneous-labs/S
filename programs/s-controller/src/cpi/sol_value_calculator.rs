use borsh::BorshSerialize;
use s_controller_interface::{SControllerError, SyncSolValueAccounts};
use s_controller_lib::try_lst_state_list;
use sanctum_onchain_utils::utils::account_info_to_account_meta;
use sol_value_calculator_interface::{
    LstToSolIxArgs, LstToSolIxData, SolToLstIxArgs, SolToLstIxData,
};
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use super::get_le_u64_return_data;

pub struct SolValueCalculatorCpi<'me, 'info> {
    /// The SOL value calculator program to invoke
    pub program: &'me AccountInfo<'info>,

    /// The mint of the LST that the calculator program works for
    pub lst: &'me AccountInfo<'info>,

    /// Remaining accounts required by the SOL value calculator program
    pub remaining_accounts: &'me [AccountInfo<'info>],
}

impl<'me, 'info> SolValueCalculatorCpi<'me, 'info> {
    /// Args:
    /// - `accounts_suffix_slice`: subslice of instruction accounts where first account is the SOL value calculator program
    ///     and remaining slice is remaining_accounts (excludes `lst`)
    pub fn from_accounts<G: GetLstMintAccountInfo<'me, 'info>>(
        ix_accounts: &G,
        accounts_suffix_slice: &'me [AccountInfo<'info>],
    ) -> Result<Self, ProgramError> {
        let program = accounts_suffix_slice
            .get(0)
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
        Ok(Self {
            program,
            lst: ix_accounts.get_lst_mint_account_info(),
            remaining_accounts: accounts_suffix_slice
                .get(1..)
                .ok_or(ProgramError::NotEnoughAccountKeys)?,
        })
    }

    pub fn verify_correct_sol_value_calculator_program<
        G: GetLstStateListAccountInfo<'me, 'info>,
        I: TryInto<usize>,
    >(
        &self,
        ix_accounts: &G,
        lst_index: I,
    ) -> Result<(), ProgramError> {
        let lst_state_list_bytes = ix_accounts
            .get_lst_state_list_account_info()
            .try_borrow_data()?;
        let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
        let lst_index = lst_index
            .try_into()
            .map_err(|_e| SControllerError::InvalidLstIndex)?;
        let lst_state = lst_state_list
            .get(lst_index)
            .ok_or(SControllerError::InvalidLstIndex)?;

        if *self.program.key != lst_state.sol_value_calculator {
            return Err(SControllerError::IncorrectSolValueCalculator.into());
        }
        Ok(())
    }

    pub fn invoke_sol_to_lst(self, sol_amt: u64) -> Result<u64, ProgramError> {
        let ix = self.create_sol_to_lst_ix(sol_amt)?;
        self.invoke_interface_ix(ix)
    }

    pub fn invoke_lst_to_sol(self, lst_amt: u64) -> Result<u64, ProgramError> {
        let ix = self.create_lst_to_sol_ix(lst_amt)?;
        self.invoke_interface_ix(ix)
    }

    fn invoke_interface_ix(self, interface_ix: Instruction) -> Result<u64, ProgramError> {
        let accounts = self.create_account_info_slice();
        invoke(&interface_ix, &accounts)?;
        let res = get_le_u64_return_data().ok_or(SControllerError::FaultySolValueCalculator)?;
        Ok(res)
    }

    fn create_account_info_slice(self) -> Vec<AccountInfo<'info>> {
        let Self {
            lst,
            remaining_accounts,
            ..
        } = self;
        [&[lst.clone()], remaining_accounts].concat()
    }

    fn create_account_metas(&self) -> Vec<AccountMeta> {
        let mut res = vec![AccountMeta::new_readonly(*self.lst.key, false)];
        for r in self.remaining_accounts.iter() {
            res.push(account_info_to_account_meta(r));
        }
        res
    }

    fn create_sol_to_lst_ix(&self, sol_amt: u64) -> Result<Instruction, ProgramError> {
        Ok(Instruction {
            program_id: *self.program.key,
            accounts: self.create_account_metas(),
            data: SolToLstIxData(SolToLstIxArgs { amount: sol_amt }).try_to_vec()?,
        })
    }

    fn create_lst_to_sol_ix(&self, lst_amt: u64) -> Result<Instruction, ProgramError> {
        Ok(Instruction {
            program_id: *self.program.key,
            accounts: self.create_account_metas(),
            data: LstToSolIxData(LstToSolIxArgs { amount: lst_amt }).try_to_vec()?,
        })
    }
}

pub trait GetLstMintAccountInfo<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info> GetLstMintAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst
    }
}

pub trait GetLstStateListAccountInfo<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}
