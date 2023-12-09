use flat_fee_interface::{PriceLpTokensToMintKeys, PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

pub struct PriceLpTokensToMintFreeArgs {
    pub input_lst_mint: Pubkey,
}

impl PriceLpTokensToMintFreeArgs {
    pub fn resolve(self) -> PriceLpTokensToMintKeys {
        PriceLpTokensToMintKeys {
            input_lst_mint: self.input_lst_mint,
        }
    }

    pub fn resolve_to_account_metas(
        self,
    ) -> [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] {
        let keys = self.resolve();
        (&keys).into()
    }
}
