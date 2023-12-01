use s_controller_interface::{
    initialize_verify_account_keys, initialize_verify_account_privileges, InitializeAccounts,
    PoolState,
};
use s_controller_lib::{
    try_pool_state_mut, InitializeFreeArgs, DEFAULT_LP_PROTOCOL_FEE_BPS,
    DEFAULT_LP_TOKEN_METADATA_NAME, DEFAULT_LP_TOKEN_METADATA_SYMBOL,
    DEFAULT_LP_TOKEN_METADATA_URI, DEFAULT_MAXIMUM_TRANSFER_FEE, DEFAULT_PRICING_PROGRAM,
    DEFAULT_TRADING_PROTOCOL_FEE_BPS, DEFAULT_TRANSFER_FEE_BPS, POOL_STATE_SIZE,
};
use sanctum_onchain_utils::{
    system_program::{create_blank_account, create_pda, CreateAccountAccounts, CreateAccountArgs},
    token_2022::{
        initialize_mint2, initialize_mint_token_metadata, initialize_transfer_fee_config,
        InitializeMint2Args, InitializeMintTokenMetadataArgs, InitializeTransferFeeConfigArgs,
    },
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};
use spl_token_2022::{extension::ExtensionType, native_mint, state::Mint};

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
        },
        &[&[
            s_controller_lib::program::STATE_SEED,
            &[s_controller_lib::program::STATE_BUMP],
        ]],
    )?;

    let mut pool_state_data = accounts.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_data)?;
    *pool_state = PoolState {
        total_sol_value: 0,
        trading_protocol_fee_bps: DEFAULT_TRADING_PROTOCOL_FEE_BPS,
        lp_protocol_fee_bps: DEFAULT_LP_PROTOCOL_FEE_BPS,
        version: 1,
        is_disabled: 0,
        is_rebalancing: 0,
        padding: [0],
        admin: *accounts.authority.key,
        rebalance_authority: *accounts.authority.key,
        protocol_fee_beneficiary: *accounts.authority.key,
        pricing_program: DEFAULT_PRICING_PROGRAM,
        lp_token_mint: *accounts.lp_token_mint.key,
    };

    let mint_size = ExtensionType::try_calculate_account_len::<Mint>(&[
        ExtensionType::TransferFeeConfig,
        ExtensionType::TokenMetadata,
        ExtensionType::MetadataPointer,
    ])?;
    create_blank_account(
        CreateAccountAccounts {
            from: accounts.payer,
            to: accounts.lp_token_mint,
        },
        CreateAccountArgs {
            space: mint_size,
            owner: spl_token_2022::ID,
        },
    )?;

    initialize_transfer_fee_config(
        accounts.lp_token_mint,
        InitializeTransferFeeConfigArgs {
            transfer_fee_config_authority: Some(*accounts.authority.key),
            withdraw_withheld_authority: Some(*accounts.authority.key),
            transfer_fee_basis_points: DEFAULT_TRANSFER_FEE_BPS,
            maximum_fee: DEFAULT_MAXIMUM_TRANSFER_FEE,
        },
    )?;
    initialize_mint_token_metadata(
        accounts.lp_token_mint,
        InitializeMintTokenMetadataArgs {
            initial_metadata: spl_token_metadata_interface::instruction::Initialize {
                name: DEFAULT_LP_TOKEN_METADATA_NAME.to_owned(),
                symbol: DEFAULT_LP_TOKEN_METADATA_SYMBOL.to_owned(),
                uri: DEFAULT_LP_TOKEN_METADATA_URI.to_owned(),
            },
            update_authority: *accounts.authority.key,
            mint_authority: *accounts.pool_state.key,
        },
    )?;
    // TODO: METADATA POINTER

    initialize_mint2(
        accounts.lp_token_mint,
        InitializeMint2Args {
            decimals: native_mint::DECIMALS,
            mint_authority: *accounts.pool_state.key,
            freeze_authority: Some(*accounts.pool_state.key),
        },
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

    initialize_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    initialize_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
