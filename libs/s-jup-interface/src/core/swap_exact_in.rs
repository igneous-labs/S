use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::{PriceExactInIxArgs, PriceExactInKeys};
use s_controller_interface::SControllerError;
use s_controller_lib::{
    calc_swap_protocol_fees, swap_exact_in_ix_by_mint_full_for_prog, try_pool_state,
    CalcSwapProtocolFeesArgs, SrcDstLstSolValueCalcAccountSuffixes, SwapByMintsFreeArgs,
    SwapExactInAmounts,
};
use s_pricing_prog_aggregate::PricingProg;
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_token_lib::MintWithTokenProgram;
use sanctum_token_ratio::AmtsAfterFeeBuilder;
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::instruction::Instruction;

use crate::{LstData, SPool};

use super::{apply_sync_sol_value, calc_quote_fees};

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> SPool<S, L> {
    pub(crate) fn quote_swap_exact_in(
        &self,
        QuoteParams {
            amount,
            input_mint,
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

        let (input_lst_state, input_lst_data) = self.find_ready_lst(*input_mint)?;
        let (pool_state, _input_lst_state, _input_reserves_balance) =
            apply_sync_sol_value(*pool_state, input_lst_state, input_lst_data)?;
        let (output_lst_state, output_lst_data) = self.find_ready_lst(*output_mint)?;
        let (pool_state, _output_lst_state, output_reserves_balance) =
            apply_sync_sol_value(pool_state, output_lst_state, output_lst_data)?;

        let in_sol_value = input_lst_data.sol_val_calc.lst_to_sol(*amount)?.get_min();
        if in_sol_value == 0 {
            return Err(SControllerError::ZeroValue.into());
        }
        let out_sol_value = pricing_prog.quote_exact_in(
            PriceExactInKeys {
                input_lst_mint: *input_mint,
                output_lst_mint: *output_mint,
            },
            &PriceExactInIxArgs {
                amount: *amount,
                sol_value: in_sol_value,
            },
        )?;
        if out_sol_value > in_sol_value {
            return Err(SControllerError::PoolWouldLoseSolValue.into());
        }
        let dst_lst_out = output_lst_data
            .sol_val_calc
            .sol_to_lst(out_sol_value)?
            .get_min();
        if dst_lst_out == 0 {
            return Err(SControllerError::ZeroValue.into());
        }
        let to_protocol_fees_lst_amount = calc_swap_protocol_fees(CalcSwapProtocolFeesArgs {
            in_sol_value,
            out_sol_value,
            dst_lst_out,
            trading_protocol_fee_bps: pool_state.trading_protocol_fee_bps,
        })?;
        let total_dst_lst_out = dst_lst_out
            .checked_add(to_protocol_fees_lst_amount)
            .ok_or(SControllerError::MathError)?;
        let not_enough_liquidity = total_dst_lst_out > output_reserves_balance;
        let (fee_amount, fee_pct) = calc_quote_fees(
            AmtsAfterFeeBuilder::new_amt_bef_fee(in_sol_value).with_amt_aft_fee(out_sol_value)?,
            &output_lst_data.sol_val_calc,
        )?;
        Ok(Quote {
            not_enough_liquidity,
            min_in_amount: None,
            min_out_amount: None,
            in_amount: *amount,
            out_amount: dst_lst_out,
            fee_mint: *output_mint,
            fee_amount,
            fee_pct,
        })
    }

    pub(crate) fn swap_exact_in_ix(
        &self,
        SwapParams {
            in_amount,
            out_amount,
            source_mint,
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
        ) = self.find_ready_lst(*source_mint)?;
        let (
            _,
            LstData {
                token_program: dst_token_program,
                sol_val_calc: dst_sol_val_calc,
                ..
            },
        ) = self.find_ready_lst(*destination_mint)?;
        let pricing_program = {
            let pool_state_data = self.pool_state_data()?;
            try_pool_state(&pool_state_data)?.pricing_program
        };
        Ok(swap_exact_in_ix_by_mint_full_for_prog(
            self.program_id,
            SwapByMintsFreeArgs {
                signer: *token_transfer_authority,
                src_lst_acc: *source_token_account,
                dst_lst_acc: *destination_token_account,
                src_lst_mint: MintWithTokenProgram {
                    pubkey: *source_mint,
                    token_program: *src_token_program,
                },
                dst_lst_mint: MintWithTokenProgram {
                    pubkey: *destination_mint,
                    token_program: *dst_token_program,
                },
                lst_state_list: &self.lst_state_list_account,
            },
            SwapExactInAmounts {
                // TODO: where did other_amount_threshold go?
                min_amount_out: *out_amount,
                amount: *in_amount,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &src_sol_val_calc.ix_accounts(),
                dst_lst_calculator_accounts: &dst_sol_val_calc.ix_accounts(),
            },
            &self
                .pricing_prog()?
                .price_exact_in_accounts(PriceExactInKeys {
                    input_lst_mint: *source_mint,
                    output_lst_mint: *destination_mint,
                })?,
            pricing_program,
        )?)
    }

    pub(crate) fn swap_exact_in_swap_and_account_metas(
        &self,
        params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let Instruction { accounts, .. } = self.swap_exact_in_ix(params)?;
        Ok(SwapAndAccountMetas {
            // TODO: jup to update this once new variant introduced
            swap: jupiter_amm_interface::Swap::StakeDexStakeWrappedSol,
            account_metas: accounts,
        })
    }
}
