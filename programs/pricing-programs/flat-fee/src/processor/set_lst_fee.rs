use flat_fee_interface::SetLstFeeIxArgs;
use flat_fee_lib::processor::{process_set_lst_fee_unchecked, verify_set_lst_fee};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_set_lst_fee(
    accounts: &[AccountInfo],
    SetLstFeeIxArgs {
        input_fee_bps,
        output_fee_bps,
    }: SetLstFeeIxArgs,
) -> ProgramResult {
    let checked = verify_set_lst_fee(accounts)?;
    process_set_lst_fee_unchecked(checked, input_fee_bps, output_fee_bps)
}
