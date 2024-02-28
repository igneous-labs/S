use flat_fee_interface::{
    FeeAccount, FlatFeeError, PriceLpTokensToMintKeys, ProgramState,
    PRICE_EXACT_IN_IX_ACCOUNTS_LEN, PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN,
    PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN,
};
use flat_fee_lib::{
    account_resolvers::{
        PriceExactInFreeArgs, PriceExactInWithBumpFreeArgs, PriceLpTokensToRedeemFreeArgs,
    },
    calc::{
        calculate_price_exact_in, calculate_price_lp_tokens_to_redeem, CalculatePriceExactInArgs,
    },
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    utils::{try_fee_account, try_program_state},
};
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::{collections::HashMap, convert::Infallible};

#[derive(Clone, Debug, Default)]
pub struct FlatFeePricingProg {
    program_id: Pubkey,
    program_state: Option<ProgramState>,
    mints_to_fee_accounts: HashMap<Pubkey, Option<FeeAccount>>, // value = None means FeeAccount not yet fetched
}

impl FlatFeePricingProg {
    pub fn new(program_id: Pubkey, mints: impl Iterator<Item = Pubkey>) -> Self {
        Self {
            program_id,
            program_state: None,
            mints_to_fee_accounts: mints.map(|pk| (pk, None)).collect(),
        }
    }

    pub fn program_id(&self) -> Pubkey {
        self.program_id
    }

    pub fn find_program_state_addr(&self) -> Pubkey {
        ProgramStateFindPdaArgs {
            program_id: self.program_id,
        }
        .get_program_state_address_and_bump_seed()
        .0
    }

    /// ## Panics
    /// - if a bump stored in self.mints_to_fee_accounts is invalid
    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        std::iter::once(self.find_program_state_addr())
            .chain(
                self.mints_to_fee_accounts
                    .iter()
                    .map(|(lst_mint, fee_account_opt)| {
                        let find_pda_args = FeeAccountFindPdaArgs {
                            program_id: self.program_id,
                            lst_mint: *lst_mint,
                        };
                        match fee_account_opt {
                            Some(FeeAccount { bump, .. }) => FeeAccountCreatePdaArgs {
                                find_pda_args,
                                bump: *bump,
                            }
                            .get_fee_account_address()
                            .unwrap(),
                            None => find_pda_args.get_fee_account_address_and_bump_seed().0,
                        }
                    }),
            )
            .collect()
    }

    pub fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), ProgramError> {
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

    pub fn quote_lp_tokens_to_redeem(
        &self,
        _output_lst_mint: Pubkey,
        pricing_programs_interface::PriceLpTokensToRedeemIxArgs { sol_value, .. }: &pricing_programs_interface::PriceLpTokensToRedeemIxArgs,
    ) -> Result<u64, FlatFeeError> {
        let lp_withdrawal_fee_bps = self
            .program_state
            .ok_or(FlatFeeError::InvalidProgramStateData)?
            .lp_withdrawal_fee_bps;
        calculate_price_lp_tokens_to_redeem(lp_withdrawal_fee_bps, *sol_value)
    }

    pub fn price_lp_tokens_to_redeem_accounts_suffix(
        &self,
        output_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Infallible> {
        Ok(
            <[AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>::from(
                PriceLpTokensToRedeemFreeArgs { output_lst_mint }.resolve_for_prog(self.program_id),
            )
            .into(),
        )
    }

    pub fn quote_lp_tokens_to_mint(
        &self,
        _input_lst_mint: Pubkey,
        pricing_programs_interface::PriceLpTokensToMintIxArgs { sol_value, .. }: &pricing_programs_interface::PriceLpTokensToMintIxArgs,
    ) -> Result<u64, FlatFeeError> {
        Ok(*sol_value)
    }

    pub fn price_lp_tokens_to_mint_accounts_suffix(
        &self,
        input_lst_mint: Pubkey,
    ) -> Result<Vec<AccountMeta>, Infallible> {
        Ok(
            <[AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]>::from(
                PriceLpTokensToMintKeys { input_lst_mint },
            )
            .into(),
        )
    }

    pub fn quote_exact_in(
        &self,
        pricing_programs_interface::PriceExactInKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactInKeys,
        pricing_programs_interface::PriceExactInIxArgs { sol_value, .. }: &pricing_programs_interface::PriceExactInIxArgs,
    ) -> Result<u64, FlatFeeError> {
        let FeeAccount { input_fee_bps, .. } = self.get_fee_account_checked(&input_lst_mint)?;
        let FeeAccount { output_fee_bps, .. } = self.get_fee_account_checked(&output_lst_mint)?;
        calculate_price_exact_in(CalculatePriceExactInArgs {
            input_fee_bps: *input_fee_bps,
            output_fee_bps: *output_fee_bps,
            in_sol_value: *sol_value,
        })
    }

    pub fn price_exact_in_accounts_suffix(
        &self,
        pricing_programs_interface::PriceExactInKeys {
            input_lst_mint,
            output_lst_mint,
        }: pricing_programs_interface::PriceExactInKeys,
    ) -> Result<Vec<AccountMeta>, ProgramError> {
        let args = PriceExactInFreeArgs {
            input_lst_mint,
            output_lst_mint,
        };
        let keys = match self.get_cached_bumps(input_lst_mint, output_lst_mint) {
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

    fn get_fee_account_checked(&self, lst_mint: &Pubkey) -> Result<&FeeAccount, FlatFeeError> {
        match self.mints_to_fee_accounts.get(lst_mint) {
            Some(Some(a)) => Ok(a),
            _ => Err(FlatFeeError::UnsupportedLstMint), // TODO: better error for when FeeAccount not yet fetched?
        }
    }

    fn get_cached_bumps(
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
}
