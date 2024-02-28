use pricing_programs_interface::{
    PriceExactInIxArgs, PriceExactInKeys, PriceExactOutIxArgs, PriceExactOutKeys,
    PriceLpTokensToMintIxArgs, PriceLpTokensToRedeemIxArgs,
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::{collections::HashMap, error::Error};

mod err;
mod flat_fee;

pub use err::*;
pub use flat_fee::*;

pub enum PricingProg {
    FlatFee(FlatFeePricingProg), // only variant for now
}

impl PricingProg {
    pub fn try_new_known_program(
        program_id: Pubkey,
        mints: impl Iterator<Item = Pubkey>,
    ) -> Result<Self, PricingProgErr> {
        Ok(match program_id {
            flat_fee_lib::program::ID => Self::FlatFee(FlatFeePricingProg::new(program_id, mints)),
            _ => Err(PricingProgErr::UnknownPricingProg)?,
        })
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::FlatFee(p) => p.get_accounts_to_update(),
        }
    }

    pub fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.update(account_map)?,
        }
        Ok(())
    }

    /// Returns SOL value of the LST to redeem
    pub fn quote_lp_tokens_to_redeem(
        &self,
        output_lst_mint: Pubkey,
        args: &PriceLpTokensToRedeemIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.quote_lp_tokens_to_redeem(output_lst_mint, args)?,
        })
    }

    pub fn price_lp_tokens_to_redeem_accounts_suffix(
        &self,
        output_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.price_lp_tokens_to_redeem_accounts_suffix(output_lst_mint)?,
        })
    }

    /// Returns SOL value of the LP tokens to mint
    pub fn quote_lp_tokens_to_mint(
        &self,
        input_lst_mint: Pubkey,
        args: &PriceLpTokensToMintIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.quote_lp_tokens_to_mint(input_lst_mint, args)?,
        })
    }

    pub fn price_lp_tokens_to_mint_accounts_suffix(
        &self,
        input_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.price_lp_tokens_to_mint_accounts_suffix(input_lst_mint)?,
        })
    }

    /// Returns SOL value of the output LST
    pub fn quote_exact_in(
        &self,
        keys: PriceExactInKeys,
        args: &PriceExactInIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.quote_exact_in(keys, args)?,
        })
    }

    pub fn price_exact_in_accounts_suffix(
        &self,
        keys: PriceExactInKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.price_exact_in_accounts_suffix(keys)?,
        })
    }

    /// Returns SOL value of the input LST
    pub fn quote_exact_out(
        &self,
        keys: PriceExactOutKeys,
        args: &PriceExactOutIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.quote_exact_out(keys, args)?,
        })
    }

    pub fn price_exact_out_accounts_suffix(
        &self,
        keys: PriceExactOutKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        Ok(match self {
            Self::FlatFee(p) => p.price_exact_out_accounts_suffix(keys)?,
        })
    }
}
