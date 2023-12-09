use flat_fee_interface::{PriceLpTokensToRedeemKeys, PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use crate::program;

pub struct PriceLpTokensToRedeemFreeArgs {
    pub output_lst_mint: Pubkey,
}

impl PriceLpTokensToRedeemFreeArgs {
    pub fn resolve(&self) -> PriceLpTokensToRedeemKeys {
        PriceLpTokensToRedeemKeys {
            output_lst_mint: self.output_lst_mint,
            state: program::STATE_ID,
        }
    }
    pub fn resolve_to_account_metas(
        self,
    ) -> [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
        let keys = self.resolve();
        (&keys).into()
    }
}
