use flat_fee_interface::ProgramState;
use flat_fee_lib::{
    initial_constants::{initial_manager, INITIAL_LP_WITHDRAWAL_FEE_BPS},
    program::STATE_SIZE,
    utils::try_program_state_mut,
};
use solana_sdk::account::Account;
use test_utils::est_rent_exempt_lamports;

pub const DEFAULT_PROGRAM_STATE: ProgramState = ProgramState {
    manager: initial_manager::ID,
    lp_withdrawal_fee_bps: INITIAL_LP_WITHDRAWAL_FEE_BPS,
};
//     total_sol_value: 0,
//     trading_protocol_fee_bps: 0,
//     lp_protocol_fee_bps: 0,
//     version: 0,
//     is_disabled: 0,
//     is_rebalancing: 0,
//     padding: [0u8; 1],
//     admin: initial_authority::ID,
//     rebalance_authority: initial_authority::ID,
//     protocol_fee_beneficiary: initial_authority::ID,
//     pricing_program: DEFAULT_PRICING_PROGRAM,
//     lp_token_mint: Pubkey::new_from_array([0u8; 32]),
// };

pub fn program_state_to_account(state: ProgramState) -> Account {
    let mut data = vec![0u8; STATE_SIZE];
    let dst = try_program_state_mut(&mut data).unwrap();
    *dst = state;
    Account {
        lamports: est_rent_exempt_lamports(STATE_SIZE),
        data,
        owner: flat_fee_lib::program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
