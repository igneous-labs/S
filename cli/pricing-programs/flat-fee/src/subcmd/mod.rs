use batch_set_fees::BatchSetFeesArgs;
use clap::Subcommand;

mod add_lst;
mod batch_set_fees;
mod common;
mod create_lut;
mod initialize;
mod price_exact_in;
mod price_exact_out;
mod price_lp_tokens_to_mint;
mod price_lp_tokens_to_redeem;
mod remove_lst;
mod set_lp_withdrawal_fee;
mod set_lst_fee;
mod set_manager;
mod view;
mod view_lst;

use add_lst::AddLstArgs;
use create_lut::CreateLutArgs;
use initialize::InitializeArgs;
use remove_lst::RemoveLstArgs;
use set_lp_withdrawal_fee::SetLpWithdrawalFeeArgs;
use set_lst_fee::SetLstFeeArgs;
use set_manager::SetManagerArgs;

use self::{
    price_exact_in::PriceExactInArgs, price_exact_out::PriceExactOutArgs,
    price_lp_tokens_to_mint::PriceLpTokensToMintArgs,
    price_lp_tokens_to_redeem::PriceLpTokensToRedeemArgs, view::ViewArgs, view_lst::ViewLstArgs,
};

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Initialize,
    SetManager(SetManagerArgs),
    AddLst(AddLstArgs),
    RemoveLst(RemoveLstArgs),
    SetLstFee(SetLstFeeArgs),
    SetLpWithdrawalFee(SetLpWithdrawalFeeArgs),
    View(ViewArgs),
    ViewLst(ViewLstArgs),
    PriceExactIn(PriceExactInArgs),
    PriceExactOut(PriceExactOutArgs),
    PriceLpTokensToMint(PriceLpTokensToMintArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemArgs),
    CreateLut(CreateLutArgs),
    BatchSetFees(BatchSetFeesArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Initialize => InitializeArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::RemoveLst(_) => RemoveLstArgs::run(args).await,
            Self::SetLstFee(_) => SetLstFeeArgs::run(args).await,
            Self::SetLpWithdrawalFee(_) => SetLpWithdrawalFeeArgs::run(args).await,
            Self::View(_) => ViewArgs::run(args).await,
            Self::ViewLst(_) => ViewLstArgs::run(args).await,
            Self::PriceExactIn(_) => PriceExactInArgs::run(args).await,
            Self::PriceExactOut(_) => PriceExactOutArgs::run(args).await,
            Self::PriceLpTokensToMint(_) => PriceLpTokensToMintArgs::run(args).await,
            Self::PriceLpTokensToRedeem(_) => PriceLpTokensToRedeemArgs::run(args).await,
            Self::CreateLut(_) => CreateLutArgs::run(args).await,
            Self::BatchSetFees(_) => BatchSetFeesArgs::run(args).await,
        }
    }
}
