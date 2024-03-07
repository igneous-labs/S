use std::{collections::HashMap, error::Error};

use sanctum_token_ratio::U64ValueRange;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;

/// Split from [`LstSolValCalc`] to make [`LstSolValCalc`] object-safe.
///
/// Example to introduce a new trait to make it object-safe by constraining the generics to concrete types:
///
/// ```rust ignore
/// use solana_sdk::account::Account;
///
/// trait JupLstSolValCalc {
///     fn get_accounts_to_update(&self) -> Vec<Pubkey>;
///
///     fn update(
///         &mut self,
///         account_map: &HashMap<Pubkey, Account>,
///     ) -> Result<(), Box<dyn Error + Send + Sync>>;
/// }
///
/// impl<P: MutableLstSolValCalc> JupPricingProg for P {
///     fn get_accounts_to_update(&self) -> Vec<Pubkey> {
///         MutableLstSolValCalc::get_accounts_to_update(self)
///     }
///
///     fn update(
///         &mut self,
///         account_map: &HashMap<Pubkey, Account>,
///     ) -> Result<(), Box<dyn Error + Send + Sync>> {
///         MutableLstSolValCalc::update(self, account_map)
///     }
/// }
/// ```
pub trait MutableLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    /// Currently, all update() implementations
    /// - no-ops if account to update is not in account_map
    /// - errors if account exists but deserialization failed / other failure
    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// Each LstSolValCalc handles SOL value calculation for a single LST mint
pub trait LstSolValCalc {
    /// Returns the LST mint this calculator works for
    fn lst_mint(&self) -> Pubkey;

    /// Returns lamport value range of `lst_amount`
    fn lst_to_sol(&self, lst_amount: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>>;

    /// Returns LST value range of `lamports`
    fn sol_to_lst(&self, lamports: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>>;

    /// Returns the account inputs to the program's SolToLst and LstToSol
    /// instructions. Both should be the same.
    ///
    /// This should exclude the program_id and include the common interface account prefixes
    fn ix_accounts(&self) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>>;
}
