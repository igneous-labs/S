use flat_fee_interface::PriceExactInKeys;
use solana_program::pubkey::{Pubkey, PubkeyError};

use crate::pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs};

pub struct PriceExactInFreeArgs {
    pub input_lst_mint: Pubkey,
    pub output_lst_mint: Pubkey,
}

impl PriceExactInFreeArgs {
    pub fn resolve(self) -> PriceExactInKeys {
        let input_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.input_lst_mint,
        };
        let (input_fee_acc, _bump) = input_find_pda_args.get_fee_account_address_and_bump_seed();

        let output_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.input_lst_mint,
        };
        let (output_fee_acc, _bump) = output_find_pda_args.get_fee_account_address_and_bump_seed();

        PriceExactInKeys {
            input_lst_mint: self.input_lst_mint,
            output_lst_mint: self.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        }
    }
}

pub struct PriceExactInWithBumpFreeArgs {
    pub find_pda_args: PriceExactInFreeArgs,
    pub input_fee_acc_bump: u8,
    pub output_fee_acc_bump: u8,
}

impl PriceExactInWithBumpFreeArgs {
    pub fn resolve(self) -> Result<PriceExactInKeys, PubkeyError> {
        let input_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.find_pda_args.input_lst_mint,
            },
            bump: [self.input_fee_acc_bump],
        };
        let input_fee_acc = input_create_pda_args.get_fee_account_address()?;

        let output_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.find_pda_args.output_lst_mint,
            },
            bump: [self.output_fee_acc_bump],
        };
        let output_fee_acc = output_create_pda_args.get_fee_account_address()?;

        Ok(PriceExactInKeys {
            input_lst_mint: self.find_pda_args.input_lst_mint,
            output_lst_mint: self.find_pda_args.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        })
    }
}
