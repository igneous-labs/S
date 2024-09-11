use flat_fee_interface::{
    FeeAccount, FlatFeeError, PriceLpTokensToMintKeys, ProgramState,
    PRICE_EXACT_IN_IX_ACCOUNTS_LEN, PRICE_EXACT_OUT_IX_ACCOUNTS_LEN,
    PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN, PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN,
};
use flat_fee_lib::{
    account_resolvers::{
        PriceExactInFreeArgs, PriceExactInWithBumpFreeArgs, PriceExactOutFreeArgs,
        PriceExactOutWithBumpFreeArgs, PriceLpTokensToRedeemFreeArgs,
    },
    calc::{
        calculate_price_exact_in, calculate_price_exact_out, calculate_price_lp_tokens_to_redeem,
        CalculatePriceExactInArgs, CalculatePriceExactOutArgs,
    },
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    utils::{try_fee_account, try_program_state},
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::collections::HashMap;

use crate::{KnownPricingProg, MutablePricingProg, PricingProg, PricingProgErr};

#[derive(Clone, Debug, Default)]
pub struct FlatFeePricingProg {
    program_id: Pubkey,
    program_state: Option<ProgramState>,
    mints_to_fee_accounts: HashMap<Pubkey, Option<FeeAccount>>, // value = None means FeeAccount not yet fetched
}

impl FlatFeePricingProg {
    pub fn find_program_state_addr(&self) -> Pubkey {
        ProgramStateFindPdaArgs {
            program_id: self.program_id,
        }
        .get_program_state_address_and_bump_seed()
        .0
    }

    fn get_fee_account_checked(&self, lst_mint: &Pubkey) -> Result<&FeeAccount, FlatFeeError> {
        match self.mints_to_fee_accounts.get(lst_mint) {
            Some(Some(a)) => Ok(a),
            _ => Err(FlatFeeError::UnsupportedLstMint), // TODO: better error for when FeeAccount not yet fetched?
        }
    }

    /// Returns (input_bump, output_bump)
    fn get_cached_fee_account_bumps(
        &self,
        input_lst_mint: Pubkey,
        output_lst_mint: Pubkey,
    ) -> Option<(u8, u8)> {
        match (
            self.get_fee_account_checked(&input_lst_mint),
            self.get_fee_account_checked(&output_lst_mint),
        ) {
            (
                Ok(FeeAccount {
                    bump: input_bump, ..
                }),
                Ok(FeeAccount {
                    bump: output_bump, ..
                }),
            ) => Some((*input_bump, *output_bump)),
            _ => None,
        }
    }

    fn fee_account_for_mint(
        &self,
        lst_mint: &Pubkey,
        fee_account_opt: &Option<FeeAccount>,
    ) -> Pubkey {
        let find_pda_args = FeeAccountFindPdaArgs {
            program_id: self.program_id,
            lst_mint: *lst_mint,
        };
        let bump = match fee_account_opt {
            Some(FeeAccount { bump, .. }) => bump,
            None => return find_pda_args.get_fee_account_address_and_bump_seed().0,
        };
        FeeAccountCreatePdaArgs {
            find_pda_args,
            bump: *bump,
        }
        .get_fee_account_address()
        .unwrap_or_else(|_e| find_pda_args.get_fee_account_address_and_bump_seed().0)
    }
}

impl MutablePricingProg for FlatFeePricingProg {
    fn try_new<I: Iterator<Item = Pubkey>>(
        program_id: Pubkey,
        mints: I,
    ) -> Result<Self, PricingProgErr>
    where
        Self: Sized,
    {
        Ok(Self {
            program_id,
            program_state: None,
            mints_to_fee_accounts: mints.map(|pk| (pk, None)).collect(),
        })
    }

    fn get_accounts_to_update_for_liquidity(&self) -> Vec<Pubkey> {
        vec![self.find_program_state_addr()]
    }

    fn get_accounts_to_update_for_all_lsts(&self) -> Vec<Pubkey> {
        self.mints_to_fee_accounts
            .iter()
            .map(|(lst_mint, fee_account_opt)| self.fee_account_for_mint(lst_mint, fee_account_opt))
            .collect()
    }

    fn get_accounts_to_update_for_lsts<I: Iterator<Item = Pubkey>>(
        &self,
        lst_mints: I,
    ) -> Vec<Pubkey> {
        lst_mints
            .map(|lst_mint| {
                let fee_account_opt = self
                    .mints_to_fee_accounts
                    .get(&lst_mint)
                    .map_or_else(|| &None, |opt| opt);
                self.fee_account_for_mint(&lst_mint, fee_account_opt)
            })
            .collect()
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        let psa = self.find_program_state_addr();
        if let Some(acc) = account_map.get(&psa) {
            self.program_state = Some(*try_program_state(&acc.data())?);
        }

        for (lst_mint, fee_account_opt) in self.mints_to_fee_accounts.iter_mut() {
            let find_pda_args = FeeAccountFindPdaArgs {
                program_id: self.program_id,
                lst_mint: *lst_mint,
            };
            let faa = match fee_account_opt {
                Some(FeeAccount { bump, .. }) => FeeAccountCreatePdaArgs {
                    find_pda_args,
                    bump: *bump,
                }
                .get_fee_account_address()?,
                None => find_pda_args.get_fee_account_address_and_bump_seed().0,
            };
            if let Some(acc) = account_map.get(&faa) {
                *fee_account_opt = Some(*try_fee_account(&acc.data())?);
            }
        }

        Ok(())
    }
}

impl PricingProg for FlatFeePricingProg {
    fn pricing_program_id(&self) -> Pubkey {
        self.program_id
    }

    fn quote_lp_tokens_to_redeem(
        &self,
        _output_lst_mint: Pubkey,
        pricing_programs_interface::PriceLpTokensToRedeemIxArgs { sol_value, .. }: &pricing_programs_interface::PriceLpTokensToRedeemIxArgs,
    ) -> anyhow::Result<u64> {
        let lp_withdrawal_fee_bps = self
            .program_state
            .ok_or(FlatFeeError::InvalidProgramStateData)?
            .lp_withdrawal_fee_bps;
        Ok(calculate_price_lp_tokens_to_redeem(
            lp_withdrawal_fee_bps,
            *sol_value,
        )?)
    }

    fn price_lp_tokens_to_redeem_accounts(
        &self,
        output_lst_mint: Pubkey,
    ) -> anyhow::Result<Vec<AccountMeta>> {
        Ok(
            <[AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>::from(
                PriceLpTokensToRedeemFreeArgs { output_lst_mint }.resolve_for_prog(self.program_id),
            )
            .into(),
        )
    }

    fn quote_lp_tokens_to_mint(
        &self,
        _input_lst_mint: Pubkey,
        pricing_programs_interface::PriceLpTokensToMintIxArgs { sol_value, .. }: &pricing_programs_interface::PriceLpTokensToMintIxArgs,
    ) -> anyhow::Result<u64> {
        Ok(*sol_value)
    }

    fn price_lp_tokens_to_mint_accounts(
        &self,
        input_lst_mint: Pubkey,
    ) -> anyhow::Result<Vec<AccountMeta>> {
        Ok(
            <[AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]>::from(
                PriceLpTokensToMintKeys { input_lst_mint },
            )
            .into(),
        )
    }

    fn quote_exact_in(
        &self,
        pricing_programs_interface::PriceExactInKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactInKeys,
        pricing_programs_interface::PriceExactInIxArgs { sol_value, .. }: &pricing_programs_interface::PriceExactInIxArgs,
    ) -> anyhow::Result<u64> {
        let FeeAccount { input_fee_bps, .. } = self.get_fee_account_checked(&input_lst_mint)?;
        let FeeAccount { output_fee_bps, .. } = self.get_fee_account_checked(&output_lst_mint)?;
        Ok(calculate_price_exact_in(CalculatePriceExactInArgs {
            input_fee_bps: *input_fee_bps,
            output_fee_bps: *output_fee_bps,
            in_sol_value: *sol_value,
        })?)
    }

    fn price_exact_in_accounts(
        &self,
        pricing_programs_interface::PriceExactInKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactInKeys,
    ) -> anyhow::Result<Vec<AccountMeta>> {
        let args = PriceExactInFreeArgs {
            input_lst_mint,
            output_lst_mint,
        };
        let keys = match self.get_cached_fee_account_bumps(input_lst_mint, output_lst_mint) {
            Some((input_fee_acc_bump, output_fee_acc_bump)) => PriceExactInWithBumpFreeArgs {
                args,
                input_fee_acc_bump,
                output_fee_acc_bump,
            }
            .resolve()?,
            None => args.resolve(),
        };
        Ok(<[AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]>::from(keys).into())
    }

    fn quote_exact_out(
        &self,
        pricing_programs_interface::PriceExactOutKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactOutKeys,
        pricing_programs_interface::PriceExactOutIxArgs { sol_value, .. }: &pricing_programs_interface::PriceExactOutIxArgs,
    ) -> anyhow::Result<u64> {
        let FeeAccount { input_fee_bps, .. } = self.get_fee_account_checked(&input_lst_mint)?;
        let FeeAccount { output_fee_bps, .. } = self.get_fee_account_checked(&output_lst_mint)?;
        Ok(calculate_price_exact_out(CalculatePriceExactOutArgs {
            input_fee_bps: *input_fee_bps,
            output_fee_bps: *output_fee_bps,
            out_sol_value: *sol_value,
        })?)
    }

    fn price_exact_out_accounts(
        &self,
        pricing_programs_interface::PriceExactOutKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactOutKeys,
    ) -> anyhow::Result<Vec<AccountMeta>> {
        let args = PriceExactOutFreeArgs {
            input_lst_mint,
            output_lst_mint,
        };
        let keys = match self.get_cached_fee_account_bumps(input_lst_mint, output_lst_mint) {
            Some((input_fee_acc_bump, output_fee_acc_bump)) => PriceExactOutWithBumpFreeArgs {
                args,
                input_fee_acc_bump,
                output_fee_acc_bump,
            }
            .resolve()?,
            None => args.resolve(),
        };
        Ok(<[AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]>::from(keys).into())
    }
}

impl TryFrom<KnownPricingProg> for FlatFeePricingProg {
    type Error = PricingProgErr;

    fn try_from(value: KnownPricingProg) -> Result<Self, Self::Error> {
        match value {
            KnownPricingProg::FlatFee(f) => Ok(f),
            // TODO: uncomment when we add more variants
            // _ => Err(PricingProgErr::WrongPricingProg),
        }
    }
}
