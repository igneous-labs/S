use bytemuck::AnyBitPattern;
use s_controller_interface::SControllerError;
use sanctum_system_program_lib::{
    allocate_invoke_signed, assign_invoke_signed, close_account, transfer_direct_increment,
    transfer_invoke, CloseAccountAccounts, ResizableAccount, TransferAccounts,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

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
        allocate_invoke_signed(list_pda, 0, list_pda_signer_seeds)?;
        assign_invoke_signed(
            list_pda,
            s_controller_lib::program::ID,
            list_pda_signer_seeds,
        )?;
    }

    let lamports_short = list_pda.extend_by(std::mem::size_of::<T>())?;

    if lamports_short > 0 {
        transfer_invoke(
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
    let index_byte_offset = index
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(SControllerError::MathError)?;
    let index_plus_one_byte_offset = index_byte_offset
        .checked_add(std::mem::size_of::<T>())
        .ok_or(SControllerError::MathError)?;
    let remaining_byte_count = list_pda
        .data_len()
        .checked_sub(index_plus_one_byte_offset)
        .ok_or(SControllerError::MathError)?;
    let data_ptr = list_pda.try_borrow_mut_data()?.as_mut_ptr();
    unsafe {
        let remaining_start = data_ptr.add(index_plus_one_byte_offset);
        let to_remove_start = data_ptr.add(index_byte_offset);
        std::ptr::copy(remaining_start, to_remove_start, remaining_byte_count);
    }

    let excess_lamports = list_pda.shrink_by(std::mem::size_of::<T>())?;
    if list_pda.data_is_empty() {
        close_account(CloseAccountAccounts {
            refund_rent_to,
            close: list_pda,
        })?;
        return Ok(());
    }
    if excess_lamports > 0 {
        transfer_direct_increment(
            TransferAccounts {
                from: list_pda,
                to: refund_rent_to,
            },
            excess_lamports,
        )?;
    }

    Ok(())
}
