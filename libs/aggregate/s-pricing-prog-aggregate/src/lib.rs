use pricing_programs_interface::{
    PriceExactInIxArgs, PriceExactInKeys, PriceExactOutIxArgs, PriceExactOutKeys,
    PriceLpTokensToMintIxArgs, PriceLpTokensToRedeemIxArgs,
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::{collections::HashMap, error::Error};

mod err;
mod flat_fee;
mod traits;

pub use err::*;
pub use flat_fee::*;
pub use traits::*;

#[derive(Clone, Debug)]
pub enum KnownPricingProg {
    FlatFee(FlatFeePricingProg), // only variant for now
}

impl MutablePricingProg for KnownPricingProg {
    fn try_new<I: Iterator<Item = Pubkey>>(
        program_id: Pubkey,
        mints: I,
    ) -> Result<Self, PricingProgErr> {
        Ok(match program_id {
            flat_fee_lib::program::ID => {
                Self::FlatFee(FlatFeePricingProg::try_new(program_id, mints)?)
            }
            _ => Err(PricingProgErr::UnknownPricingProg)?,
        })
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::FlatFee(p) => p.get_accounts_to_update(),
        }
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.update(account_map),
        }
    }
}

impl PricingProg for KnownPricingProg {
    fn quote_lp_tokens_to_redeem(
        &self,
        output_lst_mint: Pubkey,
        args: &PriceLpTokensToRedeemIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.quote_lp_tokens_to_redeem(output_lst_mint, args),
        }
    }

    fn price_lp_tokens_to_redeem_accounts(
        &self,
        output_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.price_lp_tokens_to_redeem_accounts(output_lst_mint),
        }
    }

    fn quote_lp_tokens_to_mint(
        &self,
        input_lst_mint: Pubkey,
        args: &PriceLpTokensToMintIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.quote_lp_tokens_to_mint(input_lst_mint, args),
        }
    }

    fn price_lp_tokens_to_mint_accounts(
        &self,
        input_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.price_lp_tokens_to_mint_accounts(input_lst_mint),
        }
    }

    fn quote_exact_in(
        &self,
        keys: PriceExactInKeys,
        args: &PriceExactInIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.quote_exact_in(keys, args),
        }
    }

    fn price_exact_in_accounts(
        &self,
        keys: PriceExactInKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.price_exact_in_accounts(keys),
        }
    }

    fn quote_exact_out(
        &self,
        keys: PriceExactOutKeys,
        args: &PriceExactOutIxArgs,
    ) -> Result<u64, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.quote_exact_out(keys, args),
        }
    }

    fn price_exact_out_accounts(
        &self,
        keys: PriceExactOutKeys,
    ) -> Result<Vec<AccountMeta>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::FlatFee(p) => p.price_exact_out_accounts(keys),
        }
    }
}

impl From<FlatFeePricingProg> for KnownPricingProg {
    fn from(value: FlatFeePricingProg) -> Self {
        Self::FlatFee(value)
    }
}
