use lido_keys::stsol;
use s_controller_interface::PoolState;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use spl_token::native_mint;

use crate::common::{
    AddLidoProgramTest, LpTokenProgramTest, LstStateListProgramTest, MockLstStateArgs,
    DEFAULT_POOL_STATE,
};

#[derive(Clone, Copy, Default, Debug)]
pub struct LidoWsolProgramTestArgs {
    pub wsol_reserves: u64,
    pub stsol_sol_value: u64,
    pub stsol_reserves: u64,
    pub wsol_protocol_fee_accumulator: u64,
    pub stsol_protocol_fee_accumulator: u64,
    pub lp_token_mint: Pubkey,
    pub lp_token_supply: u64,
}

/// Need to set pricing_program_id on returned PoolState
/// before adding account
pub fn lido_wsol_base_program_test(
    LidoWsolProgramTestArgs {
        wsol_reserves,
        stsol_sol_value,
        stsol_reserves,
        wsol_protocol_fee_accumulator,
        stsol_protocol_fee_accumulator,
        lp_token_mint,
        lp_token_supply,
    }: LidoWsolProgramTestArgs,
) -> (ProgramTest, PoolState) {
    let mut program_test = ProgramTest::default();
    // name must match <name>.so filename
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    program_test.add_program(
        "wsol_calculator",
        wsol_calculator_lib::program::ID,
        processor!(wsol_calculator::process_instruction),
    );
    program_test = program_test
        .add_lido_progs()
        .add_lido_stake_pool()
        .add_mock_lst_states(&[
            MockLstStateArgs {
                mint: stsol::ID,
                sol_value: stsol_sol_value,
                reserves_amt: stsol_reserves,
                protocol_fee_accumulator_amt: stsol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: lido_calculator_lib::program::ID,
            },
            MockLstStateArgs {
                mint: native_mint::ID,
                sol_value: wsol_reserves,
                reserves_amt: wsol_reserves,
                protocol_fee_accumulator_amt: wsol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: wsol_calculator_lib::program::ID,
            },
        ])
        .add_mock_lp_mint(lp_token_mint, lp_token_supply);

    let total_sol_value = stsol_sol_value + wsol_reserves;

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.total_sol_value = total_sol_value;
    pool_state.lp_token_mint = lp_token_mint;

    (program_test, pool_state)
}
