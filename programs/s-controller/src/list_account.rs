use bytemuck::AnyBitPattern;
use s_controller_interface::SControllerError;
use sanctum_onchain_utils::system_program::{
    allocate_pda, assign_pda, close_account, CloseAccountAccounts, TransferAccounts,
};
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

pub struct RemoveFromListPdaAccounts<'me, 'info> {
    pub list_pda: &'me AccountInfo<'info>,
    pub refund_rent_to: &'me AccountInfo<'info>,
}

/// Shrinks a bytemuck list account owned by the program by 1 element,
/// deleting the account if it is now empty,
/// or transfering any lamports excess of rent exemption to refund_rent_to
///
/// - `index`: index of the element to remove from the list.
///    Does not check if index is OOB, checks should be done before calling this
pub fn remove_from_list_pda<T: AnyBitPattern>(
    RemoveFromListPdaAccounts {
        list_pda,
        refund_rent_to,
    }: RemoveFromListPdaAccounts,
    index: usize,
) -> Result<(), ProgramError> {
    // shift [index+1..] items left to overwrite [index]
    let index_plus_one_byte_offset = index
        .checked_add(1)
        .and_then(|i_plus_1| i_plus_1.checked_mul(std::mem::size_of::<T>()))
        .ok_or(SControllerError::MathError)?;
    let remaining_byte_count = list_pda
        .data_len()
        .checked_sub(index_plus_one_byte_offset)
        .ok_or(SControllerError::MathError)?;
    unsafe {
        let mut data = list_pda.try_borrow_mut_data()?;
        let index_ptr = data.as_mut_ptr();
        std::ptr::copy(
            index_ptr.add(index_plus_one_byte_offset),
            index_ptr,
            remaining_byte_count,
        );
    }
    let new_len = list_pda
        .data_len()
        .checked_sub(std::mem::size_of::<T>())
        .ok_or(SControllerError::MathError)?;
    list_pda.realloc(new_len, false)?;

    if list_pda.data_is_empty() {
        close_account(CloseAccountAccounts {
            refund_rent_to,
            close: list_pda,
        })?;
        return Ok(());
    }

    let excess_lamports = list_pda
        .lamports()
        .saturating_sub(Rent::get()?.minimum_balance(new_len));

    if excess_lamports > 0 {
        sanctum_onchain_utils::system_program::transfer_direct_increment(
            TransferAccounts {
                from: list_pda,
                to: refund_rent_to,
            },
            excess_lamports,
        )?;
    }

    Ok(())
}
