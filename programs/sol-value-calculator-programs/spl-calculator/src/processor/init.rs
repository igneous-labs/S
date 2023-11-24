use generic_pool_calculator_interface::{
    init_verify_account_keys, init_verify_account_privileges, InitAccounts, InitKeys,
};
use generic_pool_calculator_lib::account_resolvers::InitRootAccounts;
use generic_pool_calculator_onchain::processor::process_init_unchecked;
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::{initial_manager, SplSolValCalc};

pub fn process_init(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify(accounts)?;
    process_init_unchecked::<SplSolValCalc>(checked, initial_manager::ID)
}

fn verify<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<InitAccounts<'me, 'info>, ProgramError> {
    let actual: InitAccounts = load_accounts(accounts)?;

    let root_keys: InitRootAccounts<SplSolValCalc> = InitRootAccounts {
        payer: *actual.payer.key,
        phantom: Default::default(),
    };
    let expected: InitKeys = root_keys.resolve();

    init_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    init_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
