use flat_fee_interface::FlatFeeProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::*;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != flat_fee_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let ix = FlatFeeProgramIx::deserialize(&mut &instruction_data[..])?;
    solana_program::msg!("{:?}", ix);

    match ix {
        FlatFeeProgramIx::PriceExactIn(args) => process_price_exact_in(accounts, args),
        FlatFeeProgramIx::PriceExactOut(args) => process_price_exact_out(accounts, args),
        FlatFeeProgramIx::PriceLpTokensToMint(args) => {
            process_price_lp_tokens_to_mint(accounts, args)
        }
        FlatFeeProgramIx::PriceLpTokensToRedeem(args) => {
            process_price_lp_tokens_to_redeem(accounts, args)
        }
        FlatFeeProgramIx::SetLpWithdrawalFee(args) => process_set_lp_withdrawal_fee(accounts, args),
        FlatFeeProgramIx::SetLstFee(args) => process_set_lst_fee(accounts, args),
        FlatFeeProgramIx::RemoveLst(_args) => process_remove_lst(accounts),
        FlatFeeProgramIx::AddLst(args) => process_add_lst(accounts, args),
        FlatFeeProgramIx::SetManager(_args) => process_set_manager(accounts),
        FlatFeeProgramIx::Initialize(_args) => process_initialize(accounts),
    }
}
