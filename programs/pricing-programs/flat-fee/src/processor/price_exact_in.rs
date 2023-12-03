use flat_fee_interface::PriceExactInIxArgs;
use flat_fee_lib::processor::{process_price_exact_in_unchecked, verify_price_exact_in};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_price_exact_in(
    accounts: &[AccountInfo],
    PriceExactInIxArgs { amount, sol_value }: PriceExactInIxArgs,
) -> ProgramResult {
    let checked = verify_price_exact_in(accounts)?;
    process_price_exact_in_unchecked(checked, amount, sol_value)
}
