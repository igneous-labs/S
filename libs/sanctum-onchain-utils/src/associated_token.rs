use solana_program::{account_info::AccountInfo, program::invoke, program_error::ProgramError};
use spl_associated_token_account::instruction::create_associated_token_account;

pub struct CreateAtaAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub ata_to_create: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

pub fn create_ata(
    CreateAtaAccounts {
        payer,
        ata_to_create,
        wallet,
        mint,
        system_program,
        token_program,
    }: CreateAtaAccounts,
) -> Result<(), ProgramError> {
    let ix = create_associated_token_account(payer.key, wallet.key, mint.key, token_program.key);
    invoke(
        &ix,
        &[
            payer.clone(),
            ata_to_create.clone(),
            wallet.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )
}
