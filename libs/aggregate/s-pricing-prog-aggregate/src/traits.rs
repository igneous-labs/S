use std::{collections::HashMap, error::Error};

use pricing_programs_interface::{
    PriceExactInIxArgs, PriceExactInKeys, PriceExactOutIxArgs, PriceExactOutKeys,
    PriceLpTokensToMintIxArgs, PriceLpTokensToRedeemIxArgs,
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;

use crate::PricingProgErr;

/// Split from [`PricingProg`] to make [`PricingProg`] object-safe.
///
/// Example to introduce a new trait to make it object-safe by constraining the generics to concrete types:
///
/// ```rust ignore
/// use solana_sdk::account::Account;
///
/// trait JupPricingProg {
///     fn get_accounts_to_update(&self) -> Vec<Pubkey>;
///
///     fn update(
///         &mut self,
///         account_map: &HashMap<Pubkey, Account>,
///     ) -> Result<(), Box<dyn Error + Send + Sync>>;
/// }
///
/// impl<P: MutablePricingProg> JupPricingProg for P {
///     fn get_accounts_to_update(&self) -> Vec<Pubkey> {
///         MutablePricingProg::get_accounts_to_update(self)
///     }
///
///     fn update(
///         &mut self,
///         account_map: &HashMap<Pubkey, Account>,
///     ) -> Result<(), Box<dyn Error + Send + Sync>> {
///         MutablePricingProg::update(self, account_map)
///     }
/// }
/// ```
pub trait MutablePricingProg {
    fn try_new<I: Iterator<Item = Pubkey>>(
        program_id: Pubkey,
        mints: I,
    ) -> Result<Self, PricingProgErr>
    where
        Self: Sized;

    fn get_accounts_to_update(&self) -> Vec<Pubkey>;

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub trait PricingProg {
    /// Returns SOL value of the LST to redeem
    fn quote_lp_tokens_to_redeem(
        &self,
        output_lst_mint: Pubkey,
        args: &PriceLpTokensToRedeemIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>>;

    /// Returns the account inputs to the program's PriceLpTokensToRedeem
    /// instruction.
    ///
    /// This should exclude the program_id and include the common interface account prefixes
    fn price_lp_tokens_to_redeem_accounts(
        &self,
        output_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>>;

    /// Returns SOL value of the LP tokens to mint
    fn quote_lp_tokens_to_mint(
        &self,
        input_lst_mint: Pubkey,
        args: &PriceLpTokensToMintIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>>;

    /// Returns the account inputs to the program's PriceLpTokensToMint
    /// instruction.
    ///
    /// This should exclude the program_id and include the common interface account prefixes
    fn price_lp_tokens_to_mint_accounts(
        &self,
        input_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>>;

    /// Returns SOL value of the output LST
    fn quote_exact_in(
        &self,
        keys: PriceExactInKeys,
        args: &PriceExactInIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>>;

    /// Returns the account inputs to the program's PriceExactIn
    /// instruction.
    ///
    /// This should exclude the program_id and include the common interface account prefixes
    fn price_exact_in_accounts(
        &self,
        keys: PriceExactInKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>>;

    /// Returns SOL value of the input LST
    fn quote_exact_out(
        &self,
        keys: PriceExactOutKeys,
        args: &PriceExactOutIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>>;

    /// Returns the account inputs to the program's PriceExactOut
    /// instruction.
    ///
    /// This should exclude the program_id and include the common interface account prefixes
    fn price_exact_out_accounts(
        &self,
        keys: PriceExactOutKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>>;
}
