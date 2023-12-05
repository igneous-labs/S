use solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey};
use solana_sdk::account::Account;
use spl_token_2022::extension::{
    transfer_fee::TransferFeeAmount, ExtensionType, StateWithExtensionsMut,
};

use crate::{est_rent_exempt_lamports, TOKEN_ACC_RENT_EXEMPT_LAMPORTS};

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

pub fn mock_lp_token_account(
    MockTokenAccountArgs {
        mint,
        authority,
        amount,
    }: MockTokenAccountArgs,
) -> Account {
    let account_size =
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(&[
            ExtensionType::TransferFeeAmount,
        ])
        .unwrap();
    let mut data = vec![0; account_size];
    let mut state =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack_uninitialized(&mut data)
            .unwrap();
    state.init_extension::<TransferFeeAmount>(true).unwrap();
    state.base = spl_token_2022::state::Account {
        mint,
        owner: authority,
        amount,
        delegate: COption::None,
        state: spl_token_2022::state::AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    state.pack_base();
    state.init_account_type().unwrap();
    Account {
        lamports: est_rent_exempt_lamports(account_size),
        data,
        owner: spl_token_2022::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
