use flat_fee_interface::SetLpWithdrawalFeeKeys;
use solana_program::pubkey::Pubkey;

use crate::program;

pub struct SetLpWithdrawalFeeRootAccounts {
    pub manager: Pubkey,
}

impl SetLpWithdrawalFeeRootAccounts {
    pub fn resolve(self) -> SetLpWithdrawalFeeKeys {
        SetLpWithdrawalFeeKeys {
            manager: self.manager,
            state: program::STATE_ID,
        }
    }
}
