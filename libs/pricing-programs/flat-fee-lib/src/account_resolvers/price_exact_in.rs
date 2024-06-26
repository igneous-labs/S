use flat_fee_interface::{PriceExactInKeys, PRICE_EXACT_IN_IX_ACCOUNTS_LEN};
use solana_program::{
    instruction::AccountMeta,
    pubkey::{Pubkey, PubkeyError},
};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs},
    program as flat_fee_program,
};

/// Uses find_program_address, for use with
/// - initial creation
/// - client side
pub struct PriceExactInFreeArgs {
    pub input_lst_mint: Pubkey,
    pub output_lst_mint: Pubkey,
}

impl PriceExactInFreeArgs {
    pub fn resolve(self) -> PriceExactInKeys {
        self.resolve_inner(flat_fee_program::ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> PriceExactInKeys {
        self.resolve_inner(program_id)
    }

    fn resolve_inner(self, program_id: Pubkey) -> PriceExactInKeys {
        let input_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.input_lst_mint,
            program_id,
        };
        let (input_fee_acc, _bump) = input_find_pda_args.get_fee_account_address_and_bump_seed();

        let output_find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.output_lst_mint,
            program_id,
        };
        let (output_fee_acc, _bump) = output_find_pda_args.get_fee_account_address_and_bump_seed();

        PriceExactInKeys {
            input_lst_mint: self.input_lst_mint,
            output_lst_mint: self.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        }
    }

    pub fn resolve_to_account_metas(self) -> [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] {
        let keys = self.resolve();
        keys.into()
    }
}

pub struct PriceExactInWithBumpFreeArgs {
    pub args: PriceExactInFreeArgs,
    pub input_fee_acc_bump: u8,
    pub output_fee_acc_bump: u8,
}

impl PriceExactInWithBumpFreeArgs {
    pub fn resolve(self) -> Result<PriceExactInKeys, PubkeyError> {
        self.resolve_inner(flat_fee_program::ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> Result<PriceExactInKeys, PubkeyError> {
        self.resolve_inner(program_id)
    }

    fn resolve_inner(self, program_id: Pubkey) -> Result<PriceExactInKeys, PubkeyError> {
        let input_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.args.input_lst_mint,
                program_id,
            },
            bump: self.input_fee_acc_bump,
        };
        let input_fee_acc = input_create_pda_args.get_fee_account_address()?;

        let output_create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.args.output_lst_mint,
                program_id,
            },
            bump: self.output_fee_acc_bump,
        };
        let output_fee_acc = output_create_pda_args.get_fee_account_address()?;

        Ok(PriceExactInKeys {
            input_lst_mint: self.args.input_lst_mint,
            output_lst_mint: self.args.output_lst_mint,
            input_fee_acc,
            output_fee_acc,
        })
    }
}
