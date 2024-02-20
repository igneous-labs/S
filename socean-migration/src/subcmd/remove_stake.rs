use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_program::message::{v0::Message, VersionedMessage};

use super::Subcmd;

#[derive(Args, Debug)]
pub struct RemoveStakeArgs {
    // #[arg(
    //     long,
    //     short,
    //     value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    // )]
    // pub validator_stake_account: Pubkey,
}

impl RemoveStakeArgs {
    pub async fn run(args: crate::Args) {
        // let Self {
        //     validator_stake_account,
        // } = match args.subcmd {
        //     Subcmd::RemoveStake(a) => a,
        //     _ => unreachable!(),
        // };

        // let rpc = args.config.nonblocking_rpc_client();
        // let payer = args.config.signer();

        // let auth_signer = args.migrate_auth.map(|s| parse_signer(&s).unwrap());
        // let auth = auth_signer.as_ref().unwrap_or(&payer);

        // let mut signers = vec![payer.as_ref(), auth.as_ref()];
        // signers.dedup();
    }
}
