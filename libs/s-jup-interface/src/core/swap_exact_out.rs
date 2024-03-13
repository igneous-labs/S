use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::{PriceExactOutIxArgs, PriceExactOutKeys};
use s_controller_interface::SControllerError;
use s_controller_lib::{
    calc_swap_protocol_fees, swap_exact_out_ix_by_mint_full_for_prog, CalcSwapProtocolFeesArgs,
    SrcDstLstSolValueCalcAccountSuffixes, SwapByMintsFreeArgs, SwapExactOutAmounts,
};
use s_pricing_prog_aggregate::PricingProg;
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_token_lib::MintWithTokenProgram;
use sanctum_token_ratio::AmtsAfterFeeBuilder;
use solana_sdk::instruction::Instruction;

use crate::{LstData, SPoolJup};

use super::{apply_sync_sol_value, calc_quote_fees};

impl SPoolJup {
    pub(crate) fn quote_swap_exact_out(
        &self,
        QuoteParams {
            amount,
            input_mint,
            output_mint,
            ..
        }: &QuoteParams,
    ) -> anyhow::Result<Quote> {
        let pool_state = self.pool_state()?;
        let pricing_prog = self
            .pricing_prog
            .as_ref()
            .ok_or_else(|| anyhow!("pricing program not fetched"))?;

        let (input_lst_state, input_lst_data) = self.find_ready_lst(*input_mint)?;
        let (pool_state, _input_lst_state, _input_reserves_balance) =
            apply_sync_sol_value(*pool_state, *input_lst_state, input_lst_data)?;
        let (output_lst_state, output_lst_data) = self.find_ready_lst(*output_mint)?;
        let (pool_state, _output_lst_state, output_reserves_balance) =
            apply_sync_sol_value(pool_state, *output_lst_state, output_lst_data)?;

        let out_sol_value = output_lst_data.sol_val_calc.lst_to_sol(*amount)?.get_max();
        if out_sol_value == 0 {
            return Err(SControllerError::ZeroValue.into());
        }
        let in_sol_value = pricing_prog.quote_exact_out(
            PriceExactOutKeys {
                input_lst_mint: *input_mint,
                output_lst_mint: *output_mint,
            },
            &PriceExactOutIxArgs {
                amount: *amount,
                sol_value: out_sol_value,
            },
        )?;
        if out_sol_value > in_sol_value {
            return Err(SControllerError::PoolWouldLoseSolValue.into());
        }
        let src_lst_in = input_lst_data
            .sol_val_calc
            .sol_to_lst(in_sol_value)?
            .get_max();
        if src_lst_in == 0 {
            return Err(SControllerError::ZeroValue.into());
        }
        let to_protocol_fees_lst_amount = calc_swap_protocol_fees(CalcSwapProtocolFeesArgs {
            in_sol_value,
            out_sol_value,
            dst_lst_out: *amount,
            trading_protocol_fee_bps: pool_state.trading_protocol_fee_bps,
        })?;
        let total_dst_lst_out = amount
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
            in_amount: src_lst_in,
            out_amount: *amount,
            fee_mint: *output_mint,
            fee_amount,
            fee_pct,
        })
    }

    pub(crate) fn swap_exact_out_ix(
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
        Ok(swap_exact_out_ix_by_mint_full_for_prog(
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
            SwapExactOutAmounts {
                // TODO: where did other_amount_threshold go?
                max_amount_in: *in_amount,
                amount: *out_amount,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &src_sol_val_calc.ix_accounts(),
                dst_lst_calculator_accounts: &dst_sol_val_calc.ix_accounts(),
            },
            &self
                .pricing_prog()?
                .price_exact_out_accounts(PriceExactOutKeys {
                    input_lst_mint: *source_mint,
                    output_lst_mint: *destination_mint,
                })?,
            self.pool_state()?.pricing_program,
        )?)
    }

    pub(crate) fn swap_exact_out_swap_and_account_metas(
        &self,
        params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let Instruction { accounts, .. } = self.swap_exact_out_ix(params)?;
        Ok(SwapAndAccountMetas {
            // TODO: jup to update this once new variant introduced
            swap: jupiter_amm_interface::Swap::StakeDexStakeWrappedSol,
            account_metas: accounts,
        })
    }
}
