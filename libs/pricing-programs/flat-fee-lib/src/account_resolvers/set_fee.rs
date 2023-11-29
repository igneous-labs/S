use flat_fee_interface::SetFeeKeys;
use solana_program::pubkey::Pubkey;

use crate::pda::FeeAccountFindPdaArgs;

// TODO: two versions: bump known and not known

pub struct SetFeeRootAccounts {
    pub manager: Pubkey,
    pub lst: Pubkey,
}

impl SetFeeRootAccounts {
    pub fn resolve(self) -> SetFeeKeys {
        let find_pda_args = FeeAccountFindPdaArgs { lst: self.lst };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        SetFeeKeys {
            manager: self.manager,
            fee_acc,
        }
    }
}
