//! TODO: Find a better way to do this program_id overriding from generic interface thing than copy pasting

use generic_pool_calculator_interface::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
};

pub fn spl_lst_to_sol_ix<K: Into<LstToSolKeys>, A: Into<LstToSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    lst_to_sol_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn spl_lst_to_sol_invoke<'info, A: Into<LstToSolIxArgs>>(
    accounts: LstToSolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = spl_lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_lst_to_sol_invoke_signed<'info, A: Into<LstToSolIxArgs>>(
    accounts: LstToSolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn spl_sol_to_lst_ix<K: Into<SolToLstKeys>, A: Into<SolToLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    sol_to_lst_ix(accounts, args).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn spl_sol_to_lst_invoke<'info, A: Into<SolToLstIxArgs>>(
    accounts: SolToLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = spl_sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_sol_to_lst_invoke_signed<'info, A: Into<SolToLstIxArgs>>(
    accounts: SolToLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn spl_update_last_upgrade_slot_ix<K: Into<UpdateLastUpgradeSlotKeys>>(
    accounts: K,
) -> std::io::Result<Instruction> {
    update_last_upgrade_slot_ix(accounts).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn spl_update_last_upgrade_slot_invoke<'info>(
    accounts: UpdateLastUpgradeSlotAccounts<'_, 'info>,
) -> ProgramResult {
    let ix = spl_update_last_upgrade_slot_ix(accounts)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_update_last_upgrade_slot_invoke_signed<'info>(
    accounts: UpdateLastUpgradeSlotAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_update_last_upgrade_slot_ix(accounts)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn spl_set_manager_ix<K: Into<SetManagerKeys>>(accounts: K) -> std::io::Result<Instruction> {
    set_manager_ix(accounts).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn spl_set_manager_invoke<'info>(accounts: SetManagerAccounts<'_, 'info>) -> ProgramResult {
    let ix = spl_set_manager_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_set_manager_invoke_signed<'info>(
    accounts: SetManagerAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_set_manager_ix(accounts)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}

pub fn spl_init_ix<K: Into<InitKeys>>(accounts: K) -> std::io::Result<Instruction> {
    init_ix(accounts).map(|mut ix| {
        ix.program_id = crate::program::ID;
        ix
    })
}
pub fn spl_init_invoke<'info>(accounts: InitAccounts<'_, 'info>) -> ProgramResult {
    let ix = spl_init_ix(accounts)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn spl_init_invoke_signed<'info>(
    accounts: InitAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = spl_init_ix(accounts)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
