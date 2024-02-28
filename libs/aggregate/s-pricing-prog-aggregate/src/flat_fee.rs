use flat_fee_interface::{
    FeeAccount, FlatFeeError, PriceLpTokensToRedeemKeys, ProgramState,
    PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN,
};
use flat_fee_lib::{
    calc::calculate_price_lp_tokens_to_redeem,
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    utils::{try_fee_account, try_program_state},
};
use pricing_programs_interface::PriceLpTokensToRedeemIxArgs;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::collections::HashMap;

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
    /// - if bump stored in FeeAccount is not valid
    pub fn find_or_create_fee_account_addr(&self, lst_mint: Pubkey) -> Pubkey {
        let find_pda_args = FeeAccountFindPdaArgs {
            program_id: self.program_id,
            lst_mint,
        };
        match self.mints_to_fee_accounts.get(&lst_mint) {
            Some(Some(FeeAccount { bump, .. })) => FeeAccountCreatePdaArgs {
                find_pda_args,
                bump: *bump,
            }
            .get_fee_account_address()
            .unwrap(),
            _ => find_pda_args.get_fee_account_address_and_bump_seed().0,
        }
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        std::iter::once(self.find_program_state_addr())
            .chain(
                self.mints_to_fee_accounts
                    .keys()
                    .map(|lst_mint| self.find_or_create_fee_account_addr(*lst_mint)),
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
        PriceLpTokensToRedeemIxArgs { sol_value, .. }: &PriceLpTokensToRedeemIxArgs,
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
    ) -> Vec<AccountMeta> {
        <[AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>::from(
            PriceLpTokensToRedeemKeys {
                output_lst_mint,
                state: self.find_program_state_addr(),
            },
        )
        .into()
    }
}
