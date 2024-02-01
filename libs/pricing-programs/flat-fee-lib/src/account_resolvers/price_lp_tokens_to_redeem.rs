use flat_fee_interface::{PriceLpTokensToRedeemKeys, PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use crate::{pda::ProgramStateFindPdaArgs, program as flat_fee_program};

pub struct PriceLpTokensToRedeemFreeArgs {
    pub output_lst_mint: Pubkey,
}

impl PriceLpTokensToRedeemFreeArgs {
    pub fn resolve(&self) -> PriceLpTokensToRedeemKeys {
        self.resolve_inner(flat_fee_program::STATE_ID)
    }

    pub fn resolve_for_prog(&self, program_id: Pubkey) -> PriceLpTokensToRedeemKeys {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        self.resolve_inner(state_id)
    }

    fn resolve_inner(&self, state_id: Pubkey) -> PriceLpTokensToRedeemKeys {
        PriceLpTokensToRedeemKeys {
            output_lst_mint: self.output_lst_mint,
            state: state_id,
        }
    }

    pub fn resolve_to_account_metas(
        self,
    ) -> [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
        let keys = self.resolve();
        keys.into()
    }
}
