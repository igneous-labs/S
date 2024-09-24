use jupiter_amm_interface::{Amm, Quote, QuoteParams, SwapParams};
use s_jup_interface::SPoolJup;
use sanctum_associated_token_lib::FindAtaAddressArgs;
use sanctum_solana_test_utils::ExtendedBanksClient;
use sanctum_token_lib::token_account_balance;
use solana_program_test::BanksClient;
use solana_sdk::{pubkey::Pubkey, signer::Signer, transaction::Transaction};

/// Prerequisites:
/// - ata_wallet must have enough SOL to pay for the tx
/// - source and dst ATAs for ata_wallet must have been created beforehand
/// - source ATA must have enough tokens to perform the swap
pub async fn assert_quote_swap_eq(
    bc: &mut BanksClient,
    s: &SPoolJup,
    ata_wallet: &dyn Signer,
    quote: &QuoteParams,
) {
    let Quote {
        in_amount,
        out_amount,
        ..
    } = s.quote(quote).unwrap();
    let input_token_prog = bc.get_account_unwrapped(quote.input_mint).await.owner;
    let output_token_prog = bc.get_account_unwrapped(quote.output_mint).await.owner;
    let [source_token_account, destination_token_account] = [
        (quote.input_mint, input_token_prog),
        (quote.output_mint, output_token_prog),
    ]
    .map(|(mint, token_program)| {
        FindAtaAddressArgs {
            wallet: ata_wallet.pubkey(),
            mint,
            token_program,
        }
        .find_ata_address()
        .0
    });
    let source_token_account_before =
        token_account_balance(bc.get_account_unwrapped(source_token_account).await).unwrap();
    let destination_token_account_before =
        token_account_balance(bc.get_account_unwrapped(destination_token_account).await).unwrap();

    let ix = s
        .swap_ix(
            &SwapParams {
                in_amount,
                out_amount,
                source_mint: quote.input_mint,
                destination_mint: quote.output_mint,
                source_token_account,
                destination_token_account,
                token_transfer_authority: ata_wallet.pubkey(),
                open_order_address: None,
                quote_mint_to_referrer: None,
                jupiter_program_id: &Pubkey::default(),
                missing_dynamic_accounts_as_default: false,
            },
            quote.swap_mode,
        )
        .unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&ata_wallet.pubkey()));
    let last_blockhash = bc.get_latest_blockhash().await.unwrap();
    tx.sign(&[ata_wallet], last_blockhash);

    assert!(bc.process_transaction(tx).await.is_ok());

    let source_token_account_after =
        token_account_balance(bc.get_account_unwrapped(source_token_account).await).unwrap();
    let destination_token_account_after =
        token_account_balance(bc.get_account_unwrapped(destination_token_account).await).unwrap();

    let actual_input = source_token_account_before - source_token_account_after;
    let actual_output = destination_token_account_after - destination_token_account_before;

    assert_eq!(actual_input, in_amount);
    assert_eq!(actual_output, out_amount);
}
