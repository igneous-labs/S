use flat_fee_interface::FeeAccount;
use flat_fee_lib::{
    pda::FeeAccountFindPdaArgs, program::FEE_ACCOUNT_SIZE, utils::try_fee_account_mut,
};
use sanctum_solana_test_utils::{est_rent_exempt_lamports, IntoAccount};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

pub struct MockFeeAccount(pub FeeAccount);

impl IntoAccount for MockFeeAccount {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; FEE_ACCOUNT_SIZE];
        let dst = try_fee_account_mut(&mut data).unwrap();
        *dst = self.0;
        Account {
            lamports: est_rent_exempt_lamports(FEE_ACCOUNT_SIZE),
            data,
            owner: flat_fee_lib::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

pub struct MockFeeAccountArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
    pub lst_mint: Pubkey,
}

impl MockFeeAccountArgs {
    pub fn to_fee_account_and_addr(&self) -> (FeeAccount, Pubkey) {
        let Self {
            input_fee_bps,
            output_fee_bps,
            lst_mint,
        } = self;
        let (addr, bump) = FeeAccountFindPdaArgs {
            lst_mint: *lst_mint,
        }
        .get_fee_account_address_and_bump_seed();
        (
            FeeAccount {
                input_fee_bps: *input_fee_bps,
                output_fee_bps: *output_fee_bps,
                bump,
                padding: 0u8,
            },
            addr,
        )
    }
}
