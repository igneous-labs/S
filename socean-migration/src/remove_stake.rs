//! Transfers ownership of a SPL stake pool stake account away.
//! Used for VSAs that have been DOS'd by SOL donations and removing the pool reserves

use sanctum_misc_utils::load_accounts;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey, stake, sysvar,
};
use stake_program_interface::{AuthorizeAccounts, AuthorizeIxArgs, StakeAuthorize};

use crate::keys::{designated_stake_authority, migrate_auth, socean_program};

// TODO: move this, share with migrate
const SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS: &[&[&[u8]]] = &[&[
    socean_program::WITHDRAW_AUTH_SEED_0,
    socean_program::WITHDRAW_AUTH_SEED_1,
    &[socean_program::WITHDRAW_AUTH_BUMP],
]];

pub const REMOVE_STAKE_ACCOUNTS_LEN: usize = 5;

pub struct RemoveStakeKeys {
    pub migrate_auth: Pubkey,
    pub socean_withdraw_auth: Pubkey,
    pub stake_program: Pubkey,
    pub clock: Pubkey,
}

pub const REMOVE_STAKE_KEYS: RemoveStakeKeys = RemoveStakeKeys {
    migrate_auth: migrate_auth::ID,
    socean_withdraw_auth: socean_program::WITHDRAW_AUTH_ID,
    stake_program: stake::program::ID,
    clock: sysvar::clock::ID,
};

pub struct RemoveStakeAccounts<'me, 'info> {
    pub migrate_auth: &'me AccountInfo<'info>,
    pub socean_withdraw_auth: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub validator_stake_account: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
}

impl RemoveStakeAccounts<'_, '_> {
    pub fn verify(&self) -> Result<(), ProgramError> {
        if *self.migrate_auth.key != REMOVE_STAKE_KEYS.migrate_auth {
            return Err(ProgramError::InvalidAccountData);
        }
        if !self.migrate_auth.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // TODO: check additional stuff
        Ok(())
    }
}

impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_STAKE_ACCOUNTS_LEN]>
    for RemoveStakeAccounts<'me, 'info>
{
    fn from(
        [
            migrate_auth,
            socean_withdraw_auth,
            stake_program,
            validator_stake_account,
            clock,
        ]: &'me [AccountInfo<'info>; REMOVE_STAKE_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            migrate_auth,
            socean_withdraw_auth,
            stake_program,
            validator_stake_account,
            clock,
        }
    }
}

pub fn process_remove_stake(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts: RemoveStakeAccounts = load_accounts(accounts)?;
    accounts.verify()?;

    stake_program_interface::authorize_invoke_signed(
        AuthorizeAccounts {
            stake: accounts.validator_stake_account,
            clock: accounts.clock,
            authority: accounts.socean_withdraw_auth,
        },
        AuthorizeIxArgs {
            new_authority: designated_stake_authority::ID,
            stake_authorize: StakeAuthorize::Withdrawer,
        },
        SOCEAN_WITHDRAW_AUTH_SIGNER_SEEDS,
    )?;

    Ok(())
}
