use s_controller_interface::PoolState;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::pubkey::Pubkey;
use spl_token::native_mint;
use test_utils::jitosol;

use crate::{
    AddSplProgramTest, LpTokenProgramTest, LstStateListProgramTest, MockLstStateArgs,
    DEFAULT_POOL_STATE,
};

#[derive(Clone, Copy, Default, Debug)]
pub struct JitoWsolProgramTestArgs {
    pub jitosol_sol_value: u64,
    pub wsol_sol_value: u64,
    pub jitosol_reserves: u64,
    pub wsol_reserves: u64,
    pub jitosol_protocol_fee_accumulator: u64,
    pub wsol_protocol_fee_accumulator: u64,
    pub lp_token_mint: Pubkey,
    pub lp_token_supply: u64,
}

impl JitoWsolProgramTestArgs {
    pub fn with_lp_token_mint(mut self, lp_token_mint: Pubkey) -> Self {
        self.lp_token_mint = lp_token_mint;
        self
    }
}

/// Need to set pricing_program_id on returned PoolState
/// before adding account.
/// Dont forget to add the s_controller program afterwards.
/// Omitted to avoid circular dependencies
pub fn jito_wsol_base_program_test(
    JitoWsolProgramTestArgs {
        jitosol_sol_value,
        wsol_sol_value,
        jitosol_reserves,
        wsol_reserves,
        jitosol_protocol_fee_accumulator,
        wsol_protocol_fee_accumulator,
        lp_token_mint,
        lp_token_supply,
    }: JitoWsolProgramTestArgs,
) -> (ProgramTest, PoolState) {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "wsol_calculator",
        wsol_calculator_lib::program::ID,
        processor!(wsol_calculator::process_instruction),
    );
    program_test = program_test
        .add_spl_progs()
        .add_jito_stake_pool()
        .add_mock_lst_states(&[
            MockLstStateArgs {
                mint: jitosol::ID,
                sol_value: jitosol_sol_value,
                reserves_amt: jitosol_reserves,
                protocol_fee_accumulator_amt: jitosol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: spl_calculator_lib::program::ID,
                is_input_disabled: false,
            },
            MockLstStateArgs {
                mint: native_mint::ID,
                sol_value: wsol_sol_value,
                reserves_amt: wsol_reserves,
                protocol_fee_accumulator_amt: wsol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: wsol_calculator_lib::program::ID,
                is_input_disabled: false,
            },
        ])
        .add_mock_lp_mint(lp_token_mint, lp_token_supply);

    let total_sol_value = jitosol_sol_value + wsol_sol_value;

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.total_sol_value = total_sol_value;
    pool_state.lp_token_mint = lp_token_mint;

    (program_test, pool_state)
}
