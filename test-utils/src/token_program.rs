use solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey};
use solana_sdk::account::Account;

use crate::TOKEN_ACC_RENT_EXEMPT_LAMPORTS;

pub struct MockTokenAccountArgs {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub amount: u64,
}

pub fn mock_token_account(
    MockTokenAccountArgs {
        mint,
        authority,
        amount,
    }: MockTokenAccountArgs,
) -> Account {
    let is_native = mint == spl_token::native_mint::ID;
    let token_account = spl_token::state::Account {
        mint,
        owner: authority,
        amount,
        delegate: COption::None,
        state: spl_token::state::AccountState::Initialized,
        is_native: if is_native {
            COption::Some(TOKEN_ACC_RENT_EXEMPT_LAMPORTS)
        } else {
            COption::None
        },
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account::pack(token_account, &mut data).unwrap();
    Account {
        lamports: TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
        data,
        owner: spl_token::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
