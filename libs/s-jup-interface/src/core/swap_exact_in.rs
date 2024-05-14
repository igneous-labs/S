use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::{PriceExactInIxArgs, PriceExactInKeys};
use s_controller_interface::{swap_exact_in_ix, SControllerError, SwapExactInIxArgs};
use s_controller_lib::{
    account_metas_extend_with_pricing_program_price_swap_accounts,
    account_metas_extend_with_src_dst_sol_value_calculator_accounts, calc_swap_protocol_fees,
    index_to_u32, swap_exact_in_ix_by_mint_full_for_prog, try_pool_state, CalcSwapProtocolFeesArgs,
    SrcDstLstIndexes, SrcDstLstSolValueCalcAccountSuffixes, SrcDstLstSolValueCalcAccounts,
    SrcDstLstSolValueCalcExtendCount, SrcDstLstSolValueCalcProgramIds, SwapByMintsFreeArgs,
    SwapExactInAmounts,
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

    pub(crate) fn swap_by_mints_free_args(
        &self,
        src_token_program: Pubkey,
        dst_token_program: Pubkey,
        SwapParams {
            source_mint,
            destination_mint,
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            ..
        }: &SwapParams,
    ) -> anyhow::Result<SwapByMintsFreeArgs<MintWithTokenProgram, MintWithTokenProgram, &L>> {
        Ok(SwapByMintsFreeArgs {
            signer: *token_transfer_authority,
            src_lst_acc: *source_token_account,
            dst_lst_acc: *destination_token_account,
            src_lst_mint: MintWithTokenProgram {
                pubkey: *source_mint,
                token_program: src_token_program,
            },
            dst_lst_mint: MintWithTokenProgram {
                pubkey: *destination_mint,
                token_program: dst_token_program,
            },
            lst_state_list: &self.lst_state_list_account,
        })
    }

    pub(crate) fn swap_exact_in_ix(&self, swap_params: &SwapParams) -> anyhow::Result<Instruction> {
        let SwapParams {
            in_amount,
            out_amount,
            source_mint,
            destination_mint,
            ..
        } = swap_params;
        let [src_rdy, dst_rdy] =
            [source_mint, destination_mint].map(|mint| self.find_ready_lst(*mint));
        let [(
            _,
            LstData {
                token_program: src_token_program,
                sol_val_calc: src_sol_val_calc,
                ..
            },
        ), (
            _,
            LstData {
                token_program: dst_token_program,
                sol_val_calc: dst_sol_val_calc,
                ..
            },
        )] = [src_rdy?, dst_rdy?];

        let pricing_program = {
            let pool_state_data = self.pool_state_data()?;
            try_pool_state(&pool_state_data)?.pricing_program
        };
        Ok(swap_exact_in_ix_by_mint_full_for_prog(
            self.program_id,
            self.swap_by_mints_free_args(*src_token_program, *dst_token_program, swap_params)?,
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
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let SwapParams {
            source_mint,
            destination_mint,
            ..
        } = swap_params;
        let [src_rdy, dst_rdy] =
            [source_mint, destination_mint].map(|mint| self.find_ready_lst(*mint));
        let [(
            _,
            LstData {
                token_program: src_token_program,
                sol_val_calc: src_sol_val_calc,
                ..
            },
        ), (
            _,
            LstData {
                token_program: dst_token_program,
                sol_val_calc: dst_sol_val_calc,
                ..
            },
        )] = [src_rdy?, dst_rdy?];
        let free_args =
            self.swap_by_mints_free_args(*src_token_program, *dst_token_program, swap_params)?;
        let (
            keys,
            SrcDstLstIndexes {
                src_lst_index,
                dst_lst_index,
            },
            SrcDstLstSolValueCalcProgramIds {
                src_lst_calculator_program_id,
                dst_lst_calculator_program_id,
            },
        ) = free_args.resolve_exact_in_for_prog(self.program_id)?;

        let mut account_metas = vec![AccountMeta {
            pubkey: self.program_id,
            is_signer: false,
            is_writable: false,
        }];
        account_metas.extend(
            swap_exact_in_ix(
                keys,
                SwapExactInIxArgs {
                    // dont cares, we're only using the ix's accounts
                    src_lst_value_calc_accs: 0,
                    dst_lst_value_calc_accs: 0,
                    src_lst_index: 0,
                    dst_lst_index: 0,
                    min_amount_out: 0,
                    amount: 0,
                },
            )?
            .accounts,
        );

        let [src_calculator_accounts, dst_calculator_accounts] =
            [src_sol_val_calc, dst_sol_val_calc].map(|calc| calc.ix_accounts());
        let SrcDstLstSolValueCalcExtendCount {
            src_lst: src_lst_value_calc_accs,
            dst_lst: dst_lst_value_calc_accs,
        } = account_metas_extend_with_src_dst_sol_value_calculator_accounts(
            &mut account_metas,
            SrcDstLstSolValueCalcAccounts {
                src_lst_calculator_program_id,
                dst_lst_calculator_program_id,
                src_lst_calculator_accounts: &src_calculator_accounts,
                dst_lst_calculator_accounts: &dst_calculator_accounts,
            },
        )?;

        let pricing_prog = self.pricing_prog()?;
        account_metas_extend_with_pricing_program_price_swap_accounts(
            &mut account_metas,
            &pricing_prog.price_exact_in_accounts(PriceExactInKeys {
                input_lst_mint: *source_mint,
                output_lst_mint: *destination_mint,
            })?,
            pricing_prog.pricing_program_id(),
        )?;

        Ok(SwapAndAccountMetas {
            swap: jupiter_amm_interface::Swap::SanctumS {
                src_lst_value_calc_accs,
                dst_lst_value_calc_accs,
                src_lst_index: index_to_u32(src_lst_index)?,
                dst_lst_index: index_to_u32(dst_lst_index)?,
            },
            account_metas,
        })
    }
}
