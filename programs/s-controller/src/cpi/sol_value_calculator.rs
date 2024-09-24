use s_controller_interface::SControllerError;
use s_controller_lib::try_lst_state_list;
use sanctum_misc_utils::{get_borsh_return_data, ToAccountMeta};
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_interface::{
    LstToSolIxArgs, LstToSolIxData, SolToLstIxArgs, SolToLstIxData,
};
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

#[derive(Clone, Copy, Debug)]
pub struct SolValueCalculatorCpi<'me, 'info> {
    /// The SOL value calculator program to invoke
    pub program: &'me AccountInfo<'info>,

    /// The mint of the LST that the calculator program works for
    pub lst_mint: &'me AccountInfo<'info>,

    /// Remaining accounts required by the SOL value calculator program
    pub remaining_accounts: &'me [AccountInfo<'info>],
}

impl<'me, 'info> SolValueCalculatorCpi<'me, 'info> {
    /// Args:
    /// - `lst_mint`
    /// - `accounts_suffix_slice`: subslice of instruction accounts where first account is the SOL value calculator program
    ///     and remaining slice is remaining_accounts (excludes `lst_mint`)
    pub fn from_lst_mint_and_account_suffix_slice(
        lst_mint: &'me AccountInfo<'info>,
        accounts_suffix_slice: &'me [AccountInfo<'info>],
    ) -> Result<Self, ProgramError> {
        let program = accounts_suffix_slice
            .first()
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
        Ok(Self {
            program,
            lst_mint,
            remaining_accounts: accounts_suffix_slice
                .get(1..)
                .ok_or(ProgramError::NotEnoughAccountKeys)?,
        })
    }

    /// Args;
    /// - `lst_state_list`
    /// - `lst_index`
    pub fn verify_correct_sol_value_calculator_program(
        &self,
        lst_state_list: &'me AccountInfo<'info>,
        lst_index: usize,
    ) -> Result<(), ProgramError> {
        let lst_state_list_bytes = lst_state_list.try_borrow_data()?;
        let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
        let lst_state = lst_state_list
            .get(lst_index)
            .ok_or(SControllerError::InvalidLstIndex)?;

        if *self.program.key != lst_state.sol_value_calculator {
            return Err(SControllerError::IncorrectSolValueCalculator.into());
        }
        Ok(())
    }

    pub fn invoke_sol_to_lst(self, sol_amt: u64) -> Result<U64ValueRange, ProgramError> {
        let ix = self.create_sol_to_lst_ix(sol_amt)?;
        self.invoke_interface_ix(ix)
    }

    pub fn invoke_lst_to_sol(self, lst_amt: u64) -> Result<U64ValueRange, ProgramError> {
        let ix = self.create_lst_to_sol_ix(lst_amt)?;
        self.invoke_interface_ix(ix)
    }

    fn invoke_interface_ix(self, interface_ix: Instruction) -> Result<U64ValueRange, ProgramError> {
        let accounts = self.create_account_info_slice();
        invoke(&interface_ix, &accounts)?;
        let (_pk, res) =
            get_borsh_return_data().ok_or(SControllerError::FaultySolValueCalculator)?;
        Ok(res)
    }

    fn create_account_info_slice(self) -> Vec<AccountInfo<'info>> {
        let Self {
            lst_mint,
            remaining_accounts,
            ..
        } = self;
        [std::slice::from_ref(lst_mint), remaining_accounts].concat()
    }

    fn create_account_metas(&self) -> Vec<AccountMeta> {
        let mut res = vec![AccountMeta::new_readonly(*self.lst_mint.key, false)];
        for r in self.remaining_accounts.iter() {
            res.push(r.to_account_meta());
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

pub struct SrcDstLstSolValueCalculatorCpis<'me, 'info> {
    pub src_lst: SolValueCalculatorCpi<'me, 'info>,
    pub dst_lst: SolValueCalculatorCpi<'me, 'info>,
}
