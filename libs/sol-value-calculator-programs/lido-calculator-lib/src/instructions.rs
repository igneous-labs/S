use generic_pool_calculator_interface::*;
use solana_program::{entrypoint::ProgramResult, instruction::Instruction};

pub fn lido_lst_to_sol_ix(
    keys: LstToSolKeys,
    args: LstToSolIxArgs,
) -> std::io::Result<Instruction> {
    lst_to_sol_ix_with_program_id(crate::program::ID, keys, args)
}

pub fn lido_lst_to_sol_invoke(accounts: LstToSolAccounts, args: LstToSolIxArgs) -> ProgramResult {
    lst_to_sol_invoke_with_program_id(crate::program::ID, accounts, args)
}
pub fn lido_lst_to_sol_invoke_signed(
    accounts: LstToSolAccounts,
    args: LstToSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lst_to_sol_invoke_signed_with_program_id(crate::program::ID, accounts, args, seeds)
}

pub fn lido_sol_to_lst_ix(
    keys: SolToLstKeys,
    args: SolToLstIxArgs,
) -> std::io::Result<Instruction> {
    sol_to_lst_ix_with_program_id(crate::program::ID, keys, args)
}
pub fn lido_sol_to_lst_invoke(accounts: SolToLstAccounts, args: SolToLstIxArgs) -> ProgramResult {
    sol_to_lst_invoke_with_program_id(crate::program::ID, accounts, args)
}
pub fn lido_sol_to_lst_invoke_signed(
    accounts: SolToLstAccounts,
    args: SolToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sol_to_lst_invoke_signed_with_program_id(crate::program::ID, accounts, args, seeds)
}

pub fn lido_update_last_upgrade_slot_ix(
    keys: UpdateLastUpgradeSlotKeys,
) -> std::io::Result<Instruction> {
    update_last_upgrade_slot_ix_with_program_id(crate::program::ID, keys)
}
pub fn lido_update_last_upgrade_slot_invoke(
    accounts: UpdateLastUpgradeSlotAccounts,
) -> ProgramResult {
    update_last_upgrade_slot_invoke_with_program_id(crate::program::ID, accounts)
}
pub fn lido_update_last_upgrade_slot_invoke_signed(
    accounts: UpdateLastUpgradeSlotAccounts,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_last_upgrade_slot_invoke_signed_with_program_id(crate::program::ID, accounts, seeds)
}

pub fn lido_set_manager_ix(keys: SetManagerKeys) -> std::io::Result<Instruction> {
    set_manager_ix_with_program_id(crate::program::ID, keys)
}
pub fn lido_set_manager_invoke(accounts: SetManagerAccounts) -> ProgramResult {
    set_manager_invoke_with_program_id(crate::program::ID, accounts)
}
pub fn lido_set_manager_invoke_signed(
    accounts: SetManagerAccounts,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_manager_invoke_signed_with_program_id(crate::program::ID, accounts, seeds)
}

pub fn lido_init_ix(keys: InitKeys) -> std::io::Result<Instruction> {
    init_ix_with_program_id(crate::program::ID, keys)
}
pub fn lido_init_invoke(accounts: InitAccounts) -> ProgramResult {
    init_invoke_with_program_id(crate::program::ID, accounts)
}
pub fn lido_init_invoke_signed(accounts: InitAccounts, seeds: &[&[&[u8]]]) -> ProgramResult {
    init_invoke_signed_with_program_id(crate::program::ID, accounts, seeds)
}
