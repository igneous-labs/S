use flat_fee_interface::AddLstIxArgs;
use flat_fee_lib::{pda::FeeAccountFindPdaArgs, program, utils::try_fee_account};
use sanctum_solana_test_utils::ExtendedBanksClient;
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;

pub async fn verify_fee_account(
    banks_client: &mut BanksClient,
    lst_mint: Pubkey,
    expected: AddLstIxArgs,
) {
    let find_pda_args = FeeAccountFindPdaArgs {
        lst_mint,
        program_id: program::ID,
    };
    let (addr, bump) = find_pda_args.get_fee_account_address_and_bump_seed();
    let actual_acc = banks_client.get_account_unwrapped(addr).await;
    let actual = try_fee_account(&actual_acc.data).unwrap();
    assert_eq!(actual.bump, bump);
    assert_eq!(actual.input_fee_bps, expected.input_fee_bps);
    assert_eq!(actual.output_fee_bps, expected.output_fee_bps);
}

pub async fn verify_fee_account_does_not_exist(banks_client: &mut BanksClient, lst_mint: Pubkey) {
    let find_pda_args = FeeAccountFindPdaArgs {
        lst_mint,
        program_id: program::ID,
    };
    let (addr, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();
    assert!(banks_client.get_account(addr).await.unwrap().is_none());
}
