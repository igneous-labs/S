use s_controller_interface::SControllerProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::*;

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != s_controller_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let ix = SControllerProgramIx::deserialize(&mut &instruction_data[..])?;
    solana_program::msg!("{:?}", ix);

    match ix {
        SControllerProgramIx::SyncSolValue(args) => process_sync_sol_value(accounts, args),
        SControllerProgramIx::SwapExactIn(args) => process_swap_exact_in(accounts, args),
        SControllerProgramIx::SwapExactOut(args) => process_swap_exact_out(accounts, args),
        SControllerProgramIx::AddLiquidity(args) => process_add_liquidity(accounts, args),
        SControllerProgramIx::RemoveLiquidity(args) => process_remove_liquidity(accounts, args),
        SControllerProgramIx::DisableLstInput(args) => process_disable_lst_input(accounts, args),
        SControllerProgramIx::EnableLstInput(args) => process_enable_lst_input(accounts, args),
        SControllerProgramIx::AddLst(_args) => process_add_lst(accounts),
        SControllerProgramIx::RemoveLst(args) => process_remove_lst(accounts, args),
        SControllerProgramIx::SetSolValueCalculator(args) => {
            process_set_sol_value_calculator(accounts, args)
        }
        SControllerProgramIx::SetAdmin(_args) => process_set_admin(accounts),
        SControllerProgramIx::SetProtocolFee(args) => process_set_protocol_fee(accounts, args),
        SControllerProgramIx::SetProtocolFeeBeneficiary(_args) => {
            process_set_protocol_fee_beneficiary(accounts)
        }
        SControllerProgramIx::SetPricingProgram(_args) => process_set_pricing_program(accounts),
        SControllerProgramIx::WithdrawProtocolFees(args) => {
            process_withdraw_protocol_fees(accounts, args)
        }
        SControllerProgramIx::AddDisablePoolAuthority(_args) => {
            process_add_disable_pool_authority(accounts)
        }
        SControllerProgramIx::RemoveDisablePoolAuthority(_args) => {
            process_remove_disable_pool_authority(accounts)
        }
        SControllerProgramIx::DisablePool(_args) => process_disable_pool(accounts),
        SControllerProgramIx::EnablePool(_args) => process_enable_pool(accounts),
        SControllerProgramIx::StartRebalance(args) => process_start_rebalance(accounts, args),
        SControllerProgramIx::EndRebalance(_args) => process_end_rebalance(accounts),
        SControllerProgramIx::SetRebalanceAuthority(_args) => {
            process_set_rebalance_authority(accounts)
        }
        SControllerProgramIx::Initialize(_args) => process_initialize(accounts),
    }
}
