//! Pre-requisites:
//! - All validators except laine removed from SPL pool. Use remove-stake to achieve this.
//! - All SOL is staked to laine (no SOL except rent-exempt reserves in SPL pool reserves). Use remove-stake to remove SPL pool reserves once reserves is empty.
//! - Validator List and Pool has enough SOL rent to pay for new accounts' rent (definitely, since our validator list is huge)
//! - migrate_auth must have enough SOL to pay for rent of new S pool state and lst state list first.
//!   We can only close the SPL stake pool accounts and refund the rent SOL last because the runtime sucks.  

use mpl_token_metadata::{
    instructions::{
        UpdateMetadataAccountV2Cpi, UpdateMetadataAccountV2CpiAccounts,
        UpdateMetadataAccountV2InstructionArgs,
    },
    types::DataV2,
};
use s_controller_interface::{LstState, PoolState, SControllerError};
use s_controller_lib::{
    initial_authority,
    program::{LST_STATE_LIST_BUMP, LST_STATE_LIST_SEED, POOL_STATE_ID},
    try_lst_state_list_mut, try_pool_state_mut, CURRENT_PROGRAM_VERS, DEFAULT_LP_PROTOCOL_FEE_BPS,
    DEFAULT_PRICING_PROGRAM, DEFAULT_TRADING_PROTOCOL_FEE_BPS, LST_STATE_SIZE, POOL_STATE_SIZE,
};
use sanctum_associated_token_lib::{create_ata_invoke, CreateAtaAccounts};
use sanctum_misc_utils::load_accounts;
use sanctum_system_program_lib::{
    close_account, init_rent_exempt_account_invoke_signed, CloseAccountAccounts,
    InitRentExemptAccountArgs,
};
use sanctum_token_lib::{set_authority_invoke_signed, SetAuthorityAccounts, SetAuthorityArgs};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    stake::{self, state::StakeAuthorize},
    system_program, sysvar,
};
use spl_token_2022::instruction::AuthorityType;
use system_program_interface::CreateAccountAccounts;

use crate::keys::{
    ata_program::{self, LAINESOL_PROTOCOL_FEE_ACCUM_BUMP, LAINESOL_RESERVES_BUMP},
    lainesol_fee_dest, lainesol_mint, lainesol_stake_pool, lainesol_stake_reserves,
    lainesol_validator_list, lainesol_vsa, migrate_auth, scnsol_metadata_update_auth, scnsol_mint,
    socean_laine_vsa, socean_program, socean_spl_pool, socean_spl_validator_list,
    spl_stake_pool_program::{self, LAINESOL_DEPOSIT_AUTH_ID, LAINESOL_WITHDRAW_AUTH_ID},
    token_metadata_program,
};

const SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS: &[&[&[u8]]] = &[&[
    socean_program::WITHDRAW_AUTH_SEED_0,
    socean_program::WITHDRAW_AUTH_SEED_1,
    &[socean_program::WITHDRAW_AUTH_BUMP],
]];

const INFSOL_NAME: &str = "Sanctum Infinity Staked SOL";
const INFSOL_SYMBOL: &str = "infSOL";
// TODO: set actual metadata URI
const INFSOL_METADATA_URI: &str = "https://google.com";

pub const MIGRATE_ACCOUNTS_LEN: usize = 30;

pub struct MigrateKeys {
    pub spl_pool: Pubkey,
    pub spl_validator_list: Pubkey,
    pub metadata_pda: Pubkey,
    pub metadata_update_authority: Pubkey,
    pub s_pool_state: Pubkey,
    pub s_lst_state_list: Pubkey,
    pub s_protocol_fee: Pubkey,
    pub lainesol_reserves: Pubkey,
    pub lainesol_protocol_fee_accum: Pubkey,
    pub scnsol_mint: Pubkey,
    pub socean_withdraw_auth: Pubkey, // also serves as scnsol mint auth
    pub lainesol_mint: Pubkey,
    pub token_metadata_program: Pubkey,
    pub stake_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub migrate_auth: Pubkey,
    pub spl_stake_pool_prog: Pubkey,
    pub socean_laine_vsa: Pubkey,
    pub lainesol_stake_pool: Pubkey,
    pub lainesol_validator_list: Pubkey,
    pub lainesol_vsa: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub lainesol_deposit_auth: Pubkey,
    pub lainesol_withdraw_auth: Pubkey,
    pub lainesol_stake_reserves: Pubkey,
    pub lainesol_fee_dest: Pubkey,
    pub ata_program: Pubkey,
}

impl From<MigrateKeys> for [AccountMeta; MIGRATE_ACCOUNTS_LEN] {
    fn from(
        MigrateKeys {
            spl_pool,
            spl_validator_list,
            metadata_pda,
            metadata_update_authority,
            s_pool_state,
            s_lst_state_list,
            s_protocol_fee,
            lainesol_reserves,
            lainesol_protocol_fee_accum,
            scnsol_mint,
            socean_withdraw_auth,
            lainesol_mint,
            token_metadata_program,
            stake_program,
            token_program,
            system_program,
            rent,
            migrate_auth,
            spl_stake_pool_prog,
            socean_laine_vsa,
            lainesol_stake_pool,
            lainesol_validator_list,
            lainesol_vsa,
            clock,
            stake_history,
            lainesol_deposit_auth,
            lainesol_withdraw_auth,
            lainesol_stake_reserves,
            lainesol_fee_dest,
            ata_program,
        }: MigrateKeys,
    ) -> Self {
        [
            AccountMeta {
                pubkey: spl_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: spl_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: metadata_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: metadata_update_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: s_pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: s_lst_state_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: s_protocol_fee,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: lainesol_reserves,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: lainesol_protocol_fee_accum,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: scnsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: socean_withdraw_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: lainesol_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: token_metadata_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: stake_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: migrate_auth,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: spl_stake_pool_prog,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: socean_laine_vsa,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: lainesol_stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: lainesol_validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: lainesol_vsa,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: lainesol_deposit_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: lainesol_withdraw_auth,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: lainesol_stake_reserves,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: lainesol_fee_dest,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: ata_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}

pub const MIGRATE_KEYS: MigrateKeys = MigrateKeys {
    spl_pool: socean_spl_pool::ID,
    spl_validator_list: socean_spl_validator_list::ID,
    metadata_pda: token_metadata_program::METADATA_PDA_ID,
    metadata_update_authority: scnsol_metadata_update_auth::ID,
    s_pool_state: s_controller_lib::program::POOL_STATE_ID,
    s_lst_state_list: s_controller_lib::program::LST_STATE_LIST_ID,
    s_protocol_fee: s_controller_lib::program::PROTOCOL_FEE_ID,
    lainesol_reserves: ata_program::LAINESOL_RESERVES_ID,
    lainesol_protocol_fee_accum: ata_program::LAINESOL_PROTOCOL_FEE_ACCUM_ID,
    scnsol_mint: scnsol_mint::ID,
    socean_withdraw_auth: socean_program::WITHDRAW_AUTH_ID,
    lainesol_mint: lainesol_mint::ID,
    token_metadata_program: token_metadata_program::ID,
    stake_program: stake::program::ID,
    token_program: spl_token::ID,
    system_program: system_program::ID,
    rent: sysvar::rent::ID,
    migrate_auth: migrate_auth::ID,
    spl_stake_pool_prog: spl_stake_pool_program::ID,
    socean_laine_vsa: socean_laine_vsa::ID,
    lainesol_stake_pool: lainesol_stake_pool::ID,
    lainesol_validator_list: lainesol_validator_list::ID,
    lainesol_vsa: lainesol_vsa::ID,
    clock: sysvar::clock::ID,
    stake_history: sysvar::stake_history::ID,
    lainesol_deposit_auth: LAINESOL_DEPOSIT_AUTH_ID,
    lainesol_withdraw_auth: LAINESOL_WITHDRAW_AUTH_ID,
    lainesol_stake_reserves: lainesol_stake_reserves::ID,
    lainesol_fee_dest: lainesol_fee_dest::ID,
    ata_program: spl_associated_token_account::ID,
};

pub struct MigrateAccounts<'me, 'info> {
    pub spl_pool: &'me AccountInfo<'info>,
    pub spl_validator_list: &'me AccountInfo<'info>,
    pub metadata_pda: &'me AccountInfo<'info>,
    pub metadata_update_authority: &'me AccountInfo<'info>,
    pub s_pool_state: &'me AccountInfo<'info>,
    pub s_lst_state_list: &'me AccountInfo<'info>,
    pub s_protocol_fee: &'me AccountInfo<'info>,
    pub lainesol_reserves: &'me AccountInfo<'info>,
    pub lainesol_protocol_fee_accum: &'me AccountInfo<'info>,
    pub scnsol_mint: &'me AccountInfo<'info>,
    pub socean_withdraw_auth: &'me AccountInfo<'info>,
    pub lainesol_mint: &'me AccountInfo<'info>,
    pub token_metadata_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub migrate_auth: &'me AccountInfo<'info>,
    pub spl_stake_pool_prog: &'me AccountInfo<'info>,
    pub socean_laine_vsa: &'me AccountInfo<'info>,
    pub lainesol_stake_pool: &'me AccountInfo<'info>,
    pub lainesol_validator_list: &'me AccountInfo<'info>,
    pub lainesol_vsa: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub lainesol_deposit_auth: &'me AccountInfo<'info>,
    pub lainesol_withdraw_auth: &'me AccountInfo<'info>,
    pub lainesol_stake_reserves: &'me AccountInfo<'info>,
    pub lainesol_fee_dest: &'me AccountInfo<'info>,
    pub ata_program: &'me AccountInfo<'info>,
}

impl MigrateAccounts<'_, '_> {
    pub fn verify(&self) -> Result<(), ProgramError> {
        // just check some important ones
        if *self.migrate_auth.key != MIGRATE_KEYS.migrate_auth {
            return Err(ProgramError::InvalidAccountData);
        }
        if !self.migrate_auth.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if *self.metadata_update_authority.key != MIGRATE_KEYS.metadata_update_authority {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_ACCOUNTS_LEN]>
    for MigrateAccounts<'me, 'info>
{
    fn from(
        [
        spl_pool,
        spl_validator_list,
        metadata_pda,
        metadata_update_authority,
        s_pool_state,
        s_lst_state_list,
        s_protocol_fee,
        lainesol_reserves,
        lainesol_protocol_fee_accum,
        scnsol_mint,
        socean_withdraw_auth,
        lainesol_mint,
        token_metadata_program,
        stake_program,
        token_program,
        system_program,
        rent,
        migrate_auth,
        spl_stake_pool_prog,
        socean_laine_vsa,
        lainesol_stake_pool,
        lainesol_validator_list,
        lainesol_vsa,
        clock,
        stake_history,
        lainesol_deposit_auth,
        lainesol_withdraw_auth,
        lainesol_stake_reserves,
        lainesol_fee_dest,
        ata_program,
    ]: &'me [AccountInfo<'info>; MIGRATE_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            spl_pool,
            spl_validator_list,
            metadata_pda,
            metadata_update_authority,
            s_pool_state,
            s_lst_state_list,
            s_protocol_fee,
            lainesol_reserves,
            lainesol_protocol_fee_accum,
            scnsol_mint,
            socean_withdraw_auth,
            lainesol_mint,
            token_metadata_program,
            stake_program,
            token_program,
            system_program,
            rent,
            migrate_auth,
            spl_stake_pool_prog,
            socean_laine_vsa,
            lainesol_stake_pool,
            lainesol_validator_list,
            lainesol_vsa,
            clock,
            stake_history,
            lainesol_deposit_auth,
            lainesol_withdraw_auth,
            lainesol_stake_reserves,
            lainesol_fee_dest,
            ata_program,
        }
    }
}

fn metadata() -> DataV2 {
    DataV2 {
        name: INFSOL_NAME.into(),
        symbol: INFSOL_SYMBOL.into(),
        uri: INFSOL_METADATA_URI.into(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    }
}

pub fn migrate_ix() -> Instruction {
    Instruction {
        program_id: s_controller_lib::program::ID,
        accounts: Vec::from(<[AccountMeta; MIGRATE_ACCOUNTS_LEN]>::from(MIGRATE_KEYS)),
        data: vec![0],
    }
}

pub fn process_migrate(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts: MigrateAccounts = load_accounts(accounts)?;
    accounts.verify()?;

    // Update scnSOL metadata to infSOL + metadata update auth
    UpdateMetadataAccountV2Cpi::new(
        accounts.token_metadata_program,
        UpdateMetadataAccountV2CpiAccounts {
            metadata: accounts.metadata_pda,
            update_authority: accounts.socean_withdraw_auth,
        },
        UpdateMetadataAccountV2InstructionArgs {
            data: Some(metadata()),
            new_update_authority: Some(*accounts.metadata_update_authority.key),
            primary_sale_happened: None,
            is_mutable: None,
        },
    )
    .invoke_signed(SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS)?;

    // Create new laineSOL ATAs
    create_ata_invoke(CreateAtaAccounts {
        ata_to_create: accounts.lainesol_reserves,
        wallet: accounts.s_pool_state,
        payer: accounts.migrate_auth,
        mint: accounts.lainesol_mint,
        system_program: accounts.system_program,
        token_program: accounts.token_program,
    })?;
    create_ata_invoke(CreateAtaAccounts {
        ata_to_create: accounts.lainesol_protocol_fee_accum,
        wallet: accounts.s_protocol_fee,
        payer: accounts.migrate_auth,
        mint: accounts.lainesol_mint,
        system_program: accounts.system_program,
        token_program: accounts.token_program,
    })?;

    // Deposit laine VSA and mint laineSOL to newly created reserves
    // Manually do it to avoid unnecessary Vec<Instruction> allocation with spl_stake_pool::instruction::deposit_stake()
    invoke_signed(
        &stake::instruction::authorize(
            accounts.socean_laine_vsa.key,
            accounts.socean_withdraw_auth.key,
            accounts.lainesol_deposit_auth.key,
            StakeAuthorize::Staker,
            None,
        ),
        &[
            accounts.socean_laine_vsa.clone(),
            accounts.clock.clone(),
            accounts.socean_withdraw_auth.clone(),
        ],
        SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS,
    )?;
    invoke_signed(
        &stake::instruction::authorize(
            accounts.socean_laine_vsa.key,
            accounts.socean_withdraw_auth.key,
            accounts.lainesol_deposit_auth.key,
            StakeAuthorize::Withdrawer,
            None,
        ),
        &[
            accounts.socean_laine_vsa.clone(),
            accounts.clock.clone(),
            accounts.socean_withdraw_auth.clone(),
        ],
        SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS,
    )?;
    invoke(
        &Instruction {
            program_id: *accounts.spl_stake_pool_prog.key,
            accounts: vec![
                AccountMeta {
                    pubkey: *accounts.lainesol_stake_pool.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_validator_list.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_deposit_auth.key,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_withdraw_auth.key,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *accounts.socean_laine_vsa.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_vsa.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_stake_reserves.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_reserves.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_fee_dest.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_reserves.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.lainesol_mint.key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts.clock.key,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *accounts.stake_history.key,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *accounts.token_program.key,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: *accounts.stake_program.key,
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data: vec![9],
        },
        &[
            accounts.lainesol_stake_pool.clone(),
            accounts.lainesol_validator_list.clone(),
            accounts.lainesol_deposit_auth.clone(),
            accounts.lainesol_withdraw_auth.clone(),
            accounts.socean_laine_vsa.clone(),
            accounts.lainesol_vsa.clone(),
            accounts.lainesol_stake_reserves.clone(),
            accounts.lainesol_reserves.clone(),
            accounts.lainesol_fee_dest.clone(),
            accounts.lainesol_reserves.clone(),
            accounts.lainesol_mint.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ],
    )?;

    // Transfer mint authority to POOL_STATE_ID
    // NB: Freeze authority can no longer be set since its been disabled
    set_authority_invoke_signed(
        SetAuthorityAccounts {
            token_program: accounts.token_program,
            to_change: accounts.scnsol_mint,
            current_authority: accounts.socean_withdraw_auth,
        },
        SetAuthorityArgs {
            authority_type: AuthorityType::MintTokens,
            new_authority: Some(POOL_STATE_ID),
        },
        SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS,
    )?;

    // Initialize PoolState and LstStateList
    init_rent_exempt_account_invoke_signed(
        CreateAccountAccounts {
            from: accounts.migrate_auth,
            to: accounts.s_pool_state,
        },
        InitRentExemptAccountArgs {
            space: POOL_STATE_SIZE,
            owner: s_controller_lib::program::ID,
        },
        &[&[
            s_controller_lib::program::POOL_STATE_SEED,
            &[s_controller_lib::program::POOL_STATE_BUMP],
        ]],
    )?;
    init_rent_exempt_account_invoke_signed(
        CreateAccountAccounts {
            from: accounts.migrate_auth,
            to: accounts.s_lst_state_list,
        },
        InitRentExemptAccountArgs {
            space: LST_STATE_SIZE, // single elem lsit
            owner: s_controller_lib::program::ID,
        },
        &[&[LST_STATE_LIST_SEED, &[LST_STATE_LIST_BUMP]]],
    )?;

    let mut pool_state_data = accounts.s_pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_data)?;
    *pool_state = PoolState {
        total_sol_value: 0,
        trading_protocol_fee_bps: DEFAULT_TRADING_PROTOCOL_FEE_BPS,
        lp_protocol_fee_bps: DEFAULT_LP_PROTOCOL_FEE_BPS,
        version: CURRENT_PROGRAM_VERS,
        is_disabled: 0,
        is_rebalancing: 0,
        padding: [0],
        admin: initial_authority::ID,
        rebalance_authority: initial_authority::ID,
        protocol_fee_beneficiary: initial_authority::ID,
        pricing_program: DEFAULT_PRICING_PROGRAM,
        lp_token_mint: *accounts.scnsol_mint.key,
    };

    let mut lst_state_list_data = accounts.s_lst_state_list.try_borrow_mut_data()?;
    let list = try_lst_state_list_mut(&mut lst_state_list_data)?;
    let new_entry = list
        .last_mut()
        .ok_or(SControllerError::InvalidLstStateListData)?;
    *new_entry = LstState {
        pool_reserves_bump: LAINESOL_RESERVES_BUMP,
        protocol_fee_accumulator_bump: LAINESOL_PROTOCOL_FEE_ACCUM_BUMP,
        sol_value: 0,
        mint: lainesol_mint::ID,
        sol_value_calculator: spl_calculator_lib::program::ID,
        is_input_disabled: 0,
        padding: [0u8; 5],
    };

    // Close validator list and stake pool.
    // Must do last to avoid
    // https://github.com/solana-labs/solana/issues/9711
    close_account(CloseAccountAccounts {
        refund_rent_to: accounts.migrate_auth,
        close: accounts.spl_pool,
    })?;
    close_account(CloseAccountAccounts {
        refund_rent_to: accounts.migrate_auth,
        close: accounts.spl_validator_list,
    })?;

    Ok(())
}
