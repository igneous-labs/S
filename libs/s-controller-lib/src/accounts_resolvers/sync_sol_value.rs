use s_controller_interface::{LstState, SControllerError, SyncSolValueKeys};
use sanctum_utils::associated_token::{create_ata_address, CreateAtaAddressArgs};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    program::{LST_STATE_LIST_ID, STATE_ID},
    try_lst_state_list,
};

pub struct SyncSolValueFreeAccounts<
    I: TryInto<usize>,
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner,
> {
    pub lst_index: I,
    pub lst_state_list: L,

    /// Only used to get the mint's owner program to derive ATA
    pub lst_mint: M,
}

impl<I: TryInto<usize>, L: ReadonlyAccountData, M: ReadonlyAccountOwner>
    SyncSolValueFreeAccounts<I, L, M>
{
    pub fn resolve(self) -> Result<SyncSolValueKeys, SControllerError> {
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let i = self
            .lst_index
            .try_into()
            .map_err(|_e| SControllerError::InvalidLstIndex)?;

        let LstState {
            mint,
            reserves_bump,
            ..
        } = list.get(i).ok_or(SControllerError::InvalidLstIndex)?;

        let token_program = self.lst_mint.owner();

        let pool_reserves = create_ata_address(CreateAtaAddressArgs {
            wallet: STATE_ID,
            mint: *mint,
            token_program: *token_program,
            bump: *reserves_bump,
        })
        .map_err(|_e| SControllerError::InvalidReserves)?;

        Ok(SyncSolValueKeys {
            lst_mint: *mint,
            pool_state: STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            pool_reserves,
        })
    }
}
