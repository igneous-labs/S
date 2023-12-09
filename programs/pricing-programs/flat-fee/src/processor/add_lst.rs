use flat_fee_interface::{
    add_lst_verify_account_keys, add_lst_verify_account_privileges, AddLstAccounts, AddLstIxArgs,
    AddLstKeys,
};
use flat_fee_lib::{
    account_resolvers::AddLstFreeArgs, fee_bound::verify_signed_fee_bps_bound,
    pda::FeeAccountCreatePdaArgs, program, utils::try_fee_account_mut,
};
use sanctum_onchain_utils::{
    system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_add_lst(accounts: &[AccountInfo], args: AddLstIxArgs) -> ProgramResult {
    let (
        AddLstAccounts { payer, fee_acc, .. },
        AddLstIxArgs {
            input_fee_bps,
            output_fee_bps,
        },
        create_pda_args,
    ) = verify_add_lst(accounts, args)?;

    create_pda(
        CreateAccountAccounts {
            from: payer,
            to: fee_acc,
        },
        CreateAccountArgs {
            space: program::FEE_ACCOUNT_SIZE,
            owner: program::ID,
            lamports: None,
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

fn verify_add_lst<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
    args: AddLstIxArgs,
) -> Result<
    (
        AddLstAccounts<'me, 'info>,
        AddLstIxArgs,
        FeeAccountCreatePdaArgs,
    ),
    ProgramError,
> {
    let actual: AddLstAccounts = load_accounts(accounts)?;

    let free_args = AddLstFreeArgs {
        payer: *actual.payer.key,
        state_acc: actual.state,
        lst_mint: *actual.lst_mint.key,
    };
    let (expected, fee_account_create_pda_args): (AddLstKeys, FeeAccountCreatePdaArgs) =
        free_args.resolve()?;

    add_lst_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    add_lst_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    verify_signed_fee_bps_bound(args.input_fee_bps)?;
    verify_signed_fee_bps_bound(args.output_fee_bps)?;

    Ok((actual, args, fee_account_create_pda_args))
}
