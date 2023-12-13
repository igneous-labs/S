use s_controller_interface::{
    initialize_verify_account_keys, initialize_verify_account_privileges, InitializeAccounts,
    PoolState, SControllerError,
};
use s_controller_lib::{
    try_pool_state_mut, InitializeFreeArgs, CURRENT_PROGRAM_VERS, DEFAULT_LP_PROTOCOL_FEE_BPS,
    DEFAULT_PRICING_PROGRAM, DEFAULT_TRADING_PROTOCOL_FEE_BPS, POOL_STATE_SIZE,
};
use sanctum_onchain_utils::{
    system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs},
    token_program::{set_authority, SetAuthorityAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::{instruction::AuthorityType, native_mint, state::Mint};

pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts = verify_initialize(accounts)?;

    create_pda(
        CreateAccountAccounts {
            from: accounts.payer,
            to: accounts.pool_state,
        },
        CreateAccountArgs {
            space: POOL_STATE_SIZE,
            owner: s_controller_lib::program::ID,
            lamports: None,
        },
        &[&[
            s_controller_lib::program::POOL_STATE_SEED,
            &[s_controller_lib::program::POOL_STATE_BUMP],
        ]],
    )?;

    // need to drop borrow of pool_state before mint CPIs
    {
        let mut pool_state_data = accounts.pool_state.try_borrow_mut_data()?;
        let pool_state = try_pool_state_mut(&mut pool_state_data)?;
        *pool_state = PoolState {
            total_sol_value: 0,
            trading_protocol_fee_bps: DEFAULT_TRADING_PROTOCOL_FEE_BPS,
            lp_protocol_fee_bps: DEFAULT_LP_PROTOCOL_FEE_BPS,
            version: CURRENT_PROGRAM_VERS,
            is_disabled: 0,
            is_rebalancing: 0,
            padding: [0],
            admin: *accounts.authority.key,
            rebalance_authority: *accounts.authority.key,
            protocol_fee_beneficiary: *accounts.authority.key,
            pricing_program: DEFAULT_PRICING_PROGRAM,
            lp_token_mint: *accounts.lp_token_mint.key,
        };
    }

    let set_authority_accounts = SetAuthorityAccounts {
        token_program: accounts.lp_token_program,
        to_change: accounts.lp_token_mint,
        current_authority: accounts.authority,
    };

    set_authority(
        set_authority_accounts,
        AuthorityType::MintTokens,
        Some(*accounts.pool_state.key),
    )?;
    set_authority(
        set_authority_accounts,
        AuthorityType::FreezeAccount,
        Some(*accounts.pool_state.key),
    )
}

fn verify_initialize<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<InitializeAccounts<'a, 'info>, ProgramError> {
    let actual: InitializeAccounts = load_accounts(accounts)?;

    let expected = InitializeFreeArgs {
        payer: *actual.payer.key,
        lp_token_mint: *actual.lp_token_mint.key,
    }
    .resolve();

    initialize_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    initialize_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    verify_lp_token_mint(actual.lp_token_mint)?;

    Ok(actual)
}

fn verify_lp_token_mint(lp_token_mint: &AccountInfo<'_>) -> Result<(), ProgramError> {
    if *lp_token_mint.owner != spl_token::ID {
        return Err(SControllerError::IncorrectLpMintInitialization.into());
    }
    let mint = Mint::unpack(&lp_token_mint.try_borrow_data()?)?;
    if mint.supply != 0 {
        return Err(SControllerError::IncorrectLpMintInitialization.into());
    }
    if mint.decimals != native_mint::DECIMALS {
        return Err(SControllerError::IncorrectLpMintInitialization.into());
    }
    Ok(())
}
