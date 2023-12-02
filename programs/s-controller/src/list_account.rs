use bytemuck::AnyBitPattern;
use s_controller_interface::SControllerError;
use sanctum_onchain_utils::system_program::{allocate_pda, assign_pda, TransferAccounts};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, rent::Rent, sysvar::Sysvar,
};

pub struct ExtendListPdaAccounts<'me, 'info> {
    pub list_pda: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
}

/// Extends a bytemuck list account owned by the program by 1 element,
/// creating the account if it was empty before,
/// and transfering enough lamports from `payer` to make it rent-exempt
pub fn extend_list_pda<T: AnyBitPattern>(
    ExtendListPdaAccounts { list_pda, payer }: ExtendListPdaAccounts,
    list_pda_signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    if list_pda.data_is_empty() {
        allocate_pda(list_pda, 0, list_pda_signer_seeds)?;
        assign_pda(
            list_pda,
            s_controller_lib::program::ID,
            list_pda_signer_seeds,
        )?;
    }
    let new_len = list_pda
        .data_len()
        .checked_add(std::mem::size_of::<T>())
        .ok_or(SControllerError::MathError)?;

    list_pda.realloc(new_len, false)?;

    let lamports_short = Rent::get()?
        .minimum_balance(new_len)
        .saturating_sub(list_pda.lamports());

    if lamports_short > 0 {
        sanctum_onchain_utils::system_program::transfer(
            TransferAccounts {
                from: payer,
                to: list_pda,
            },
            lamports_short,
        )?;
    }

    Ok(())
}
