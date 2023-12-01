use flat_fee_interface::{
    add_lst_verify_account_keys, add_lst_verify_account_privileges, AddLstAccounts, AddLstKeys,
};
use sanctum_onchain_utils::{
    system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_resolvers::AddLstFreeArgs,
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs, FEE_ACCOUNT_SIZE},
    program,
    utils::try_fee_account_mut,
};

pub fn process_add_lst_unchecked(
    accounts: AddLstAccounts,
    input_fee_bps: i16,
    output_fee_bps: i16,
) -> ProgramResult {
    let find_pda_args = FeeAccountFindPdaArgs {
        lst_mint: *accounts.lst_mint.key,
    };
    process_add_lst_with_bump_unchecked(
        accounts,
        find_pda_args.into(),
        input_fee_bps,
        output_fee_bps,
    )
}

pub fn process_add_lst_with_bump_unchecked(
    AddLstAccounts {
        manager: _,
        payer,
        fee_acc,
        lst_mint: _,
        state: _,
        system_program: _,
    }: AddLstAccounts,
    create_pda_args: FeeAccountCreatePdaArgs,
    input_fee_bps: i16,
    output_fee_bps: i16,
) -> ProgramResult {
    create_pda(
        CreateAccountAccounts {
            from: payer,
            to: fee_acc,
        },
        CreateAccountArgs {
            space: FEE_ACCOUNT_SIZE,
            owner: program::ID,
        },
        &[create_pda_args.to_signer_seeds().as_slice()],
    )?;

    let mut bytes = fee_acc.try_borrow_mut_data()?;
    let fee_acc = try_fee_account_mut(&mut bytes)?;

    let [bump] = create_pda_args.bump;
    fee_acc.bump = bump;
    fee_acc.input_fee_bps = input_fee_bps;
    fee_acc.output_fee_bps = output_fee_bps;

    Ok(())
}

pub fn verify_add_lst<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<AddLstAccounts<'me, 'info>, ProgramError> {
    let actual: AddLstAccounts = load_accounts(accounts)?;

    let free_args = AddLstFreeArgs {
        payer: *actual.payer.key,
        state: actual.state,
        lst_mint: *actual.lst_mint.key,
    };
    let expected: AddLstKeys = free_args.resolve()?;

    add_lst_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    add_lst_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}

// TODO: add cases for when you know bump
