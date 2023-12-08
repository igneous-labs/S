use flat_fee_interface::SetLpWithdrawalFeeKeys;
use solana_program::pubkey::Pubkey;

use crate::program::STATE_ID;

pub struct SetLpWithdrawalFeeFreeArgs {
    pub manager: Pubkey,
}

impl SetLpWithdrawalFeeFreeArgs {
    pub fn resolve(self) -> SetLpWithdrawalFeeKeys {
        SetLpWithdrawalFeeKeys {
            manager: self.manager,
            state: STATE_ID,
        }
    }
}
