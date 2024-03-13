use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::PriceLpTokensToRedeemIxArgs;
use s_controller_interface::SControllerError;
use s_controller_lib::{
    calc_lp_tokens_sol_value, calc_remove_liquidity_protocol_fees,
    remove_liquidity_ix_by_mint_full_for_prog, AddRemoveLiquidityAccountSuffixes,
    CalcRemoveLiquidityProtocolFeesArgs, LpTokenRateArgs, RemoveLiquidityByMintFreeArgs,
    RemoveLiquidityIxAmts,
};
use s_pricing_prog_aggregate::PricingProg;
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_token_lib::MintWithTokenProgram;
use sanctum_token_ratio::AmtsAfterFeeBuilder;
use solana_sdk::instruction::Instruction;

use crate::{LstData, SPoolJup};

use super::{apply_sync_sol_value, calc_quote_fees};

impl SPoolJup {
    pub(crate) fn quote_remove_liquidity(
        &self,
        QuoteParams {
            amount,
            output_mint,
            ..
        }: &QuoteParams,
    ) -> anyhow::Result<Quote> {
        let pool_state = self.pool_state()?;
        let pricing_prog = self
            .pricing_prog
            .as_ref()
            .ok_or_else(|| anyhow!("pricing program not fetched"))?;
        let lp_token_supply = self
            .lp_mint_supply
            .ok_or_else(|| anyhow!("LP mint not fetched"))?;

        let (output_lst_state, output_lst_data) = self.find_ready_lst(*output_mint)?;
        let (pool_state, _output_lst_state, output_reserves_balance) =
            apply_sync_sol_value(*pool_state, *output_lst_state, output_lst_data)?;

        let pool_total_sol_value = pool_state.total_sol_value;
        let lp_tokens_sol_value = calc_lp_tokens_sol_value(
            LpTokenRateArgs {
                lp_token_supply,
                pool_total_sol_value,
            },
            *amount,
        )?;

        let lp_tokens_sol_value_after_fees = pricing_prog.quote_lp_tokens_to_redeem(
            *output_mint,
            &PriceLpTokensToRedeemIxArgs {
                amount: *amount,
                sol_value: lp_tokens_sol_value,
            },
        )?;
        if lp_tokens_sol_value_after_fees > lp_tokens_sol_value {
            return Err(SControllerError::PoolWouldLoseSolValue.into());
        }
        let to_user_lst_amount = output_lst_data
            .sol_val_calc
            .sol_to_lst(lp_tokens_sol_value_after_fees)?
            .get_min();
        let to_protocol_fees_lst_amount =
            calc_remove_liquidity_protocol_fees(CalcRemoveLiquidityProtocolFeesArgs {
                lp_tokens_sol_value,
                lp_tokens_sol_value_after_fees,
                to_user_lst_amount,
                lp_protocol_fee_bps: pool_state.lp_protocol_fee_bps,
            })?;
        let total_dst_lst_out = to_user_lst_amount
            .checked_add(to_protocol_fees_lst_amount)
            .ok_or(SControllerError::MathError)?;
        let not_enough_liquidity = total_dst_lst_out > output_reserves_balance;
        let (fee_amount, fee_pct) = calc_quote_fees(
            AmtsAfterFeeBuilder::new_amt_bef_fee(lp_tokens_sol_value)
                .with_amt_aft_fee(lp_tokens_sol_value_after_fees)?,
            &output_lst_data.sol_val_calc,
        )?;
        Ok(Quote {
            not_enough_liquidity,
            min_in_amount: None,
            min_out_amount: None,
            in_amount: *amount,
            out_amount: to_user_lst_amount,
            fee_mint: *output_mint,
            fee_amount,
            fee_pct,
        })
    }

    pub(crate) fn remove_liquidity_ix(
        &self,
        SwapParams {
            in_amount,
            out_amount,
            destination_mint,
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            ..
        }: &SwapParams,
    ) -> anyhow::Result<Instruction> {
        let (
            _,
            LstData {
                token_program: src_token_program,
                sol_val_calc: src_sol_val_calc,
                ..
            },
        ) = self.find_ready_lst(*destination_mint)?;
        Ok(remove_liquidity_ix_by_mint_full_for_prog(
            self.program_id,
            RemoveLiquidityByMintFreeArgs {
                signer: *token_transfer_authority,
                src_lp_acc: *source_token_account,
                dst_lst_acc: *destination_token_account,
                pool_state: self
                    .pool_state_account
                    .as_ref()
                    .ok_or_else(|| anyhow!("Pool state not fetched"))?,
                lst_state_list: &self.lst_state_list_account,
                lst_mint: MintWithTokenProgram {
                    pubkey: *destination_mint,
                    token_program: *src_token_program,
                },
            },
            RemoveLiquidityIxAmts {
                lp_token_amount: *in_amount,
                min_lst_out: *out_amount,
            },
            AddRemoveLiquidityAccountSuffixes {
                lst_calculator_accounts: &src_sol_val_calc.ix_accounts(),
                pricing_program_price_lp_accounts: &self
                    .pricing_prog()?
                    .price_lp_tokens_to_redeem_accounts(*destination_mint)?,
            },
        )?)
    }

    pub(crate) fn remove_liquidity_swap_and_account_metas(
        &self,
        params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let Instruction { accounts, .. } = self.remove_liquidity_ix(params)?;
        Ok(SwapAndAccountMetas {
            // TODO: jup to update this once new variant introduced
            swap: jupiter_amm_interface::Swap::StakeDexStakeWrappedSol,
            account_metas: accounts,
        })
    }
}
