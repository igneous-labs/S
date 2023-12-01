use flat_fee_interface::SetLpWithdrawalFeeIxArgs;
use flat_fee_lib::processor::{
    process_set_lp_withdrawal_fee_unchecked, verify_set_lp_withdrawal_fee,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_set_lp_withdrawal_fee(
    accounts: &[AccountInfo],
    SetLpWithdrawalFeeIxArgs {
        lp_withdrawal_fee_bps,
    }: SetLpWithdrawalFeeIxArgs,
) -> ProgramResult {
    let checked = verify_set_lp_withdrawal_fee(accounts)?;
    process_set_lp_withdrawal_fee_unchecked(checked, lp_withdrawal_fee_bps)
}
