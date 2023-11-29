//! TODO: Find a better way to do this program_id overriding from generic interface thing than copy pasting

use generic_pool_calculator_interface::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
};

pub fn marinade_lst_to_sol_ix<K: Into<LstToSolKeys>, A: Into<LstToSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    lst_to_sol_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn marinade_lst_to_sol_invoke<'info, A: Into<LstToSolIxArgs>>(
    accounts: &LstToSolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = marinade_lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_lst_to_sol_invoke_signed<'info, A: Into<LstToSolIxArgs>>(
    accounts: &LstToSolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn marinade_sol_to_lst_ix<K: Into<SolToLstKeys>, A: Into<SolToLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    sol_to_lst_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn marinade_sol_to_lst_invoke<'info, A: Into<SolToLstIxArgs>>(
    accounts: &SolToLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = marinade_sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_sol_to_lst_invoke_signed<'info, A: Into<SolToLstIxArgs>>(
    accounts: &SolToLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn marinade_update_last_upgrade_slot_ix<
    K: Into<UpdateLastUpgradeSlotKeys>,
    A: Into<UpdateLastUpgradeSlotIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    update_last_upgrade_slot_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn marinade_update_last_upgrade_slot_invoke<'info, A: Into<UpdateLastUpgradeSlotIxArgs>>(
    accounts: &UpdateLastUpgradeSlotAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = marinade_update_last_upgrade_slot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_update_last_upgrade_slot_invoke_signed<
    'info,
    A: Into<UpdateLastUpgradeSlotIxArgs>,
>(
    accounts: &UpdateLastUpgradeSlotAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_update_last_upgrade_slot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn marinade_set_manager_ix<K: Into<SetManagerKeys>, A: Into<SetManagerIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    set_manager_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn marinade_set_manager_invoke<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = marinade_set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_set_manager_invoke_signed<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn marinade_init_ix<K: Into<InitKeys>, A: Into<InitIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    init_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn marinade_init_invoke<'info, A: Into<InitIxArgs>>(
    accounts: &InitAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = marinade_init_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn marinade_init_invoke_signed<'info, A: Into<InitIxArgs>>(
    accounts: &InitAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = marinade_init_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
