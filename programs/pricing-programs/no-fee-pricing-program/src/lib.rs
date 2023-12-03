use pricing_programs_interface::{
    PriceExactInIxArgs, PriceExactOutIxArgs, PriceLpTokensToMintIxArgs,
    PriceLpTokensToRedeemIxArgs, PricingProgramsProgramIx,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data, pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

sanctum_macros::declare_program_keys!("NoFEEPR1C1NGPRoGRAM111111111111111111111111", []);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = PricingProgramsProgramIx::deserialize(&mut &instruction_data[..])?;
    solana_program::msg!("{:?}", ix);

    let sol_value = match ix {
        PricingProgramsProgramIx::PriceExactIn(PriceExactInIxArgs { sol_value, .. }) => sol_value,
        PricingProgramsProgramIx::PriceExactOut(PriceExactOutIxArgs { sol_value, .. }) => sol_value,
        PricingProgramsProgramIx::PriceLpTokensToMint(PriceLpTokensToMintIxArgs {
            sol_value,
            ..
        }) => sol_value,
        PricingProgramsProgramIx::PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs {
            sol_value,
            ..
        }) => sol_value,
    };
    let sol_value_le = sol_value.to_le_bytes();
    set_return_data(&sol_value_le);
    Ok(())
}
