use flat_fee_interface::AddLstIxArgs;
use flat_fee_lib::processor::{process_add_lst_unchecked, verify_add_lst};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_add_lst(
    accounts: &[AccountInfo],
    AddLstIxArgs {
        input_fee_bps,
        output_fee_bps,
    }: AddLstIxArgs,
) -> ProgramResult {
    let checked = verify_add_lst(accounts)?;
    process_add_lst_unchecked(checked, input_fee_bps, output_fee_bps)
}
