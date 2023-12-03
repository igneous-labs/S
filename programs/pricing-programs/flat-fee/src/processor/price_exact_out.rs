use flat_fee_interface::PriceExactOutIxArgs;
use flat_fee_lib::processor::{process_price_exact_out_unchecked, verify_price_exact_out};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_price_exact_out(
    accounts: &[AccountInfo],
    PriceExactOutIxArgs { amount, sol_value }: PriceExactOutIxArgs,
) -> ProgramResult {
    let checked = verify_price_exact_out(accounts)?;
    process_price_exact_out_unchecked(checked, amount, sol_value)
}
