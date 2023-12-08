use flat_fee_interface::{
    initialize_verify_account_keys, initialize_verify_account_privileges, InitializeAccounts,
    InitializeKeys,
};
use flat_fee_lib::{
    account_resolvers::InitializeFreeArgs,
    initial_constants::{initial_manager, INITIAL_LP_WITHDRAWAL_FEE_BPS},
    program,
    utils::try_program_state_mut,
};
use sanctum_onchain_utils::{
    system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    let InitializeAccounts { payer, state, .. } = verify_initialize(accounts)?;

    create_pda(
        CreateAccountAccounts {
            from: payer,
            to: state,
        },
        CreateAccountArgs {
            space: program::STATE_SIZE,
            owner: program::ID,
            lamports: None,
        },
        &[&[program::STATE_SEED, &[program::STATE_BUMP]]],
    )?;

    let mut bytes = state.try_borrow_mut_data()?;
    let state = try_program_state_mut(&mut bytes)?;

    state.manager = initial_manager::ID;
    state.lp_withdrawal_fee_bps = INITIAL_LP_WITHDRAWAL_FEE_BPS;

    Ok(())
}

fn verify_initialize<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<InitializeAccounts<'me, 'info>, ProgramError> {
    let actual: InitializeAccounts = load_accounts(accounts)?;

    let free_args = InitializeFreeArgs {
        payer: *actual.payer.key,
    };
    let expected: InitializeKeys = free_args.resolve();

    initialize_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    initialize_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
