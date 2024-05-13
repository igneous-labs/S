use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapParams};
use pricing_programs_interface::PriceLpTokensToMintIxArgs;
use s_controller_interface::{add_liquidity_ix, AddLiquidityIxArgs, SControllerError};
use s_controller_lib::{
    account_metas_extend_with_pricing_program_price_lp_accounts,
    account_metas_extend_with_sol_value_calculator_accounts,
    add_liquidity_ix_by_mint_full_for_prog, calc_lp_tokens_to_mint, index_to_u32, try_pool_state,
    AddLiquidityByMintFreeArgs, AddLiquidityIxAmts, AddRemoveLiquidityAccountSuffixes,
    AddRemoveLiquidityProgramIds, LpTokenRateArgs,
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
    pub(crate) fn quote_add_liquidity(
        &self,
        QuoteParams {
            amount, input_mint, ..
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

        let (input_lst_state, input_lst_data) = self.find_ready_lst(*input_mint)?;
        let (pool_state, _input_lst_state, _input_reserves_balance) =
            apply_sync_sol_value(*pool_state, input_lst_state, input_lst_data)?;

        let lst_amount_sol_value = input_lst_data.sol_val_calc.lst_to_sol(*amount)?.get_min();

        let lst_amount_sol_value_after_fees = pricing_prog.quote_lp_tokens_to_mint(
            *input_mint,
            &PriceLpTokensToMintIxArgs {
                amount: *amount,
                sol_value: lst_amount_sol_value,
            },
        )?;
        if lst_amount_sol_value_after_fees > lst_amount_sol_value {
            return Err(SControllerError::PoolWouldLoseSolValue.into());
        }
        let lp_tokens_to_mint = calc_lp_tokens_to_mint(
            LpTokenRateArgs {
                lp_token_supply,
                pool_total_sol_value: pool_state.total_sol_value,
            },
            lst_amount_sol_value_after_fees,
        )?;
        let (fee_amount, fee_pct) = calc_quote_fees(
            AmtsAfterFeeBuilder::new_amt_bef_fee(lst_amount_sol_value)
                .with_amt_aft_fee(lst_amount_sol_value_after_fees)?,
            &input_lst_data.sol_val_calc,
        )?;
        Ok(Quote {
            in_amount: *amount,
            out_amount: lp_tokens_to_mint,
            fee_mint: *input_mint,
            fee_amount,
            fee_pct,
            ..Default::default()
        })
    }

    fn add_liquidity_free_args(
        &self,
        source_token_program: &Pubkey,
        SwapParams {
            source_mint,
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            ..
        }: &SwapParams,
    ) -> anyhow::Result<AddLiquidityByMintFreeArgs<&S, &L, MintWithTokenProgram>> {
        Ok(AddLiquidityByMintFreeArgs {
            signer: *token_transfer_authority,
            src_lst_acc: *source_token_account,
            dst_lp_acc: *destination_token_account,
            pool_state: self
                .pool_state_account
                .as_ref()
                .ok_or_else(|| anyhow!("Pool state not fetched"))?,
            lst_state_list: &self.lst_state_list_account,
            lst_mint: MintWithTokenProgram {
                pubkey: *source_mint,
                token_program: *source_token_program,
            },
        })
    }

    pub(crate) fn add_liquidity_ix(&self, swap_params: &SwapParams) -> anyhow::Result<Instruction> {
        let SwapParams {
            in_amount,
            out_amount,
            source_mint,
            ..
        } = swap_params;
        let (
            _,
            LstData {
                token_program: src_token_program,
                sol_val_calc: src_sol_val_calc,
                ..
            },
        ) = self.find_ready_lst(*source_mint)?;
        Ok(add_liquidity_ix_by_mint_full_for_prog(
            self.program_id,
            self.add_liquidity_free_args(src_token_program, swap_params)?,
            AddLiquidityIxAmts {
                lst_amount: *in_amount,
                min_lp_out: *out_amount,
            },
            AddRemoveLiquidityAccountSuffixes {
                lst_calculator_accounts: &src_sol_val_calc.ix_accounts(),
                pricing_program_price_lp_accounts: &self
                    .pricing_prog()?
                    .price_lp_tokens_to_mint_accounts(*source_mint)?,
            },
        )?)
    }

    pub(crate) fn add_liquidity_swap_and_account_metas(
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
        ) = self.find_ready_lst(swap_params.source_mint)?;
        let free_args = self.add_liquidity_free_args(src_token_program, swap_params)?;
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
            add_liquidity_ix(
                keys,
                AddLiquidityIxArgs {
                    // dont cares, we're only using the ix's accounts
                    lst_value_calc_accs: 0,
                    lst_index: 0,
                    lst_amount: 0,
                    min_lp_out: 0,
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
                .price_lp_tokens_to_mint_accounts(swap_params.source_mint)?,
            pricing_program_id,
        )?;

        Ok(SwapAndAccountMetas {
            swap: jupiter_amm_interface::Swap::SanctumSAddLiquidity {
                lst_value_calc_accs,
                lst_index: index_to_u32(lst_index)?,
            },
            account_metas,
        })
    }
}
