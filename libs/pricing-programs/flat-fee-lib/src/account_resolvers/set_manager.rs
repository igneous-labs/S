use flat_fee_interface::{FlatFeeError, ProgramState, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program, utils::try_program_state};

pub struct SetManagerFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state: S,
}

// `KeyedAccount` bound not used here
impl<S: KeyedAccount + ReadonlyAccountData> SetManagerFreeArgs<S> {
    pub fn resolve(self) -> Result<SetManagerKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetManagerKeys {
            current_manager: state.manager,
            new_manager: self.new_manager,
            state: program::STATE_ID,
        })
    }
}
