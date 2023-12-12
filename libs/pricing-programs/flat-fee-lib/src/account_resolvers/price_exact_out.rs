use flat_fee_interface::{PriceExactOutKeys, PRICE_EXACT_OUT_IX_ACCOUNTS_LEN};
use solana_program::{
    instruction::AccountMeta,
    pubkey::{Pubkey, PubkeyError},
};

use crate::pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs};

/// Uses find_program_address, for use with
/// - initial creation
/// - client side
pub struct PriceExactOutFreeArgs {
    pub input_lst_mint: Pubkey,
    pub output_lst_mint: Pubkey,
}

impl PriceExactOutFreeArgs {
    pub fn resolve(self) -> PriceExactOutKeys {
        let input_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.input_lst_mint,
        };
        let (input_fee_acc, _bump) = input_find_pda_args.get_fee_account_address_and_bump_seed();

        let output_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.output_lst_mint,
        };
        let (output_fee_acc, _bump) = output_find_pda_args.get_fee_account_address_and_bump_seed();

        PriceExactOutKeys {
            input_lst_mint: self.input_lst_mint,
            output_lst_mint: self.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        }
    }

    pub fn resolve_to_account_metas(self) -> [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] {
        let keys = self.resolve();
        keys.into()
    }
}

pub struct PriceExactOutWithBumpFreeArgs {
    pub find_pda_args: PriceExactOutFreeArgs,
    pub input_fee_acc_bump: u8,
    pub output_fee_acc_bump: u8,
}

impl PriceExactOutWithBumpFreeArgs {
    pub fn resolve(self) -> Result<PriceExactOutKeys, PubkeyError> {
        let input_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.find_pda_args.input_lst_mint,
            },
            bump: self.input_fee_acc_bump,
        };
        let input_fee_acc = input_create_pda_args.get_fee_account_address()?;

        let output_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.find_pda_args.output_lst_mint,
            },
            bump: self.output_fee_acc_bump,
        };
        let output_fee_acc = output_create_pda_args.get_fee_account_address()?;

        Ok(PriceExactOutKeys {
            input_lst_mint: self.find_pda_args.input_lst_mint,
            output_lst_mint: self.find_pda_args.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        })
    }
}
