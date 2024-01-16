use s_controller_interface::{
    add_lst_verify_account_keys, add_lst_verify_account_privileges, AddLstAccounts, LstState,
    SControllerError,
};
use s_controller_lib::{
    program::{LST_STATE_LIST_BUMP, LST_STATE_LIST_SEED},
    try_lst_state_list, try_lst_state_list_mut, try_pool_state, AddLstFreeArgs, LstStateBumps,
};
use sanctum_associated_token_lib::{create_ata_invoke, CreateAtaAccounts};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_s_common::token::{verify_token_account_authority, verify_tokenkeg_or_22_mint};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    list_account::{extend_list_pda, ExtendListPdaAccounts},
    verify::{verify_not_rebalancing_and_not_disabled, verify_sol_value_calculator_is_program},
};

pub fn process_add_lst(accounts: &[AccountInfo]) -> ProgramResult {
    let (
        accounts,
        LstStateBumps {
            protocol_fee_accumulator: protocol_fee_accumulator_bump,
            pool_reserves: pool_reserves_bump,
        },
    ) = verify_add_lst(accounts)?;

    create_ata_if_not_exist(CreateAtaAccounts {
        ata_to_create: accounts.pool_reserves,
        wallet: accounts.pool_state,

        payer: accounts.payer,
        mint: accounts.lst_mint,
        system_program: accounts.system_program,
        token_program: accounts.lst_token_program,
    })?;

    create_ata_if_not_exist(CreateAtaAccounts {
        ata_to_create: accounts.protocol_fee_accumulator,
        wallet: accounts.protocol_fee_accumulator_auth,

        payer: accounts.payer,
        mint: accounts.lst_mint,
        system_program: accounts.system_program,
        token_program: accounts.lst_token_program,
    })?;

    extend_list_pda::<LstState>(
        ExtendListPdaAccounts {
            list_pda: accounts.lst_state_list,
            payer: accounts.payer,
        },
        &[&[LST_STATE_LIST_SEED, &[LST_STATE_LIST_BUMP]]],
    )?;

    let mut lst_state_list_data = accounts.lst_state_list.try_borrow_mut_data()?;
    let list = try_lst_state_list_mut(&mut lst_state_list_data)?;
    let new_entry = list
        .last_mut()
        .ok_or(SControllerError::InvalidLstStateListData)?;

    *new_entry = LstState {
        pool_reserves_bump,
        protocol_fee_accumulator_bump,
        sol_value: 0,
        mint: *accounts.lst_mint.key,
        sol_value_calculator: *accounts.sol_value_calculator.key,
        is_input_disabled: 0,
        padding: [0u8; 5],
    };

    Ok(())
}

fn create_ata_if_not_exist(accounts: CreateAtaAccounts) -> Result<(), ProgramError> {
    if accounts.ata_to_create.data_is_empty() {
        return create_ata_invoke(accounts);
    }
    verify_token_account_authority(accounts.ata_to_create, *accounts.wallet.key)?;
    Ok(())
}

fn verify_not_duplicate(
    lst_state_list: &AccountInfo,
    lst_mint: Pubkey,
) -> Result<(), ProgramError> {
    let d = lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&d)?;
    match lst_state_list
        .iter()
        .any(|LstState { mint, .. }| *mint == lst_mint)
    {
        true => Err(SControllerError::DuplicateLst.into()),
        false => Ok(()),
    }
}

fn verify_add_lst<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<(AddLstAccounts<'a, 'info>, LstStateBumps), ProgramError> {
    let actual: AddLstAccounts = load_accounts(accounts)?;

    let free_args = AddLstFreeArgs {
        payer: *actual.payer.key,
        sol_value_calculator: *actual.sol_value_calculator.key,
        pool_state: actual.pool_state,
        lst_mint: actual.lst_mint,
    };
    let (expected, bumps) = free_args.resolve()?;

    add_lst_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    add_lst_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    verify_tokenkeg_or_22_mint(actual.lst_mint)?;
    verify_sol_value_calculator_is_program(actual.sol_value_calculator)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    verify_not_duplicate(actual.lst_state_list, *actual.lst_mint.key)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok((actual, bumps))
}
