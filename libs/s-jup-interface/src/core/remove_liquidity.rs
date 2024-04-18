use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::PriceLpTokensToRedeemIxArgs;
use s_controller_interface::{remove_liquidity_ix, RemoveLiquidityIxArgs, SControllerError};
use s_controller_lib::{
    account_metas_extend_with_pricing_program_price_lp_accounts,
    account_metas_extend_with_sol_value_calculator_accounts, calc_lp_tokens_sol_value,
    calc_remove_liquidity_protocol_fees, index_to_u32, remove_liquidity_ix_by_mint_full_for_prog,
    try_pool_state, AddRemoveLiquidityAccountSuffixes, AddRemoveLiquidityProgramIds,
    CalcRemoveLiquidityProtocolFeesArgs, LpTokenRateArgs, RemoveLiquidityByMintFreeArgs,
    RemoveLiquidityIxAmts,
};
use s_pricing_prog_aggregate::PricingProg;
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_token_lib::MintWithTokenProgram;
use sanctum_token_ratio::AmtsAfterFeeBuilder;
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{LstData, SPool};

use super::{apply_sync_sol_value, calc_quote_fees};

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> SPool<S, L> {
    pub(crate) fn quote_remove_liquidity(
        &self,
        QuoteParams {
            amount,
            output_mint,
            ..
        }: &QuoteParams,
    ) -> anyhow::Result<Quote> {
        let pool_state_data = self.pool_state_data()?;
        let pool_state = try_pool_state(&pool_state_data)?;
        let pricing_prog = self
            .pricing_prog
            .as_ref()
            .ok_or_else(|| anyhow!("pricing program not fetched"))?;
        let lp_token_supply = self
            .lp_mint_supply
            .ok_or_else(|| anyhow!("LP mint not fetched"))?;

        let (output_lst_state, output_lst_data) = self.find_ready_lst(*output_mint)?;
        let (pool_state, _output_lst_state, output_reserves_balance) =
            apply_sync_sol_value(*pool_state, output_lst_state, output_lst_data)?;

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

    fn remove_liquidity_free_args(
        &self,
        source_token_program: &Pubkey,
        SwapParams {
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            destination_mint,
            ..
        }: &SwapParams,
    ) -> anyhow::Result<RemoveLiquidityByMintFreeArgs<&S, &L, MintWithTokenProgram>> {
        Ok(RemoveLiquidityByMintFreeArgs {
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
                token_program: *source_token_program,
            },
        })
    }

    pub(crate) fn remove_liquidity_ix(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<Instruction> {
        let SwapParams {
            in_amount,
            out_amount,
            destination_mint,
            ..
        } = swap_params;
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
            self.remove_liquidity_free_args(src_token_program, swap_params)?,
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
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let (
            _,
            LstData {
                token_program: src_token_program,
                sol_val_calc: src_sol_val_calc,
                ..
            },
        ) = self.find_ready_lst(swap_params.destination_mint)?;
        let free_args = self.remove_liquidity_free_args(src_token_program, swap_params)?;
        let (
            keys,
            lst_index,
            AddRemoveLiquidityProgramIds {
                lst_calculator_program_id,
                pricing_program_id,
            },
        ) = free_args.resolve_for_prog(self.program_id)?;

        let mut account_metas = vec![AccountMeta {
            pubkey: self.program_id,
            is_signer: false,
            is_writable: false,
        }];
        account_metas.extend(
            remove_liquidity_ix(
                keys,
                RemoveLiquidityIxArgs {
                    // dont cares, we're only using the ix's accounts
                    lst_value_calc_accs: 0,
                    lst_index: 0,
                    lp_token_amount: 0,
                    min_lst_out: 0,
                },
            )?
            .accounts,
        );

        let lst_value_calc_accs = account_metas_extend_with_sol_value_calculator_accounts(
            &mut account_metas,
            &src_sol_val_calc.ix_accounts(),
            lst_calculator_program_id,
        )?;

        account_metas_extend_with_pricing_program_price_lp_accounts(
            &mut account_metas,
            &self
                .pricing_prog()?
                .price_lp_tokens_to_redeem_accounts(swap_params.source_mint)?,
            pricing_program_id,
        )?;

        Ok(SwapAndAccountMetas {
            // TODO: jup to update this once new variant introduced
            swap: jupiter_amm_interface::Swap::SanctumSRemoveLiquidity {
                lst_value_calc_accs,
                lst_index: index_to_u32(lst_index)?,
            },
            account_metas,
        })
    }
}
