use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolState {
    pub total_sol_value: u64,
    pub trading_protocol_fee_bps: u16,
    pub lp_protocol_fee_bps: u16,
    pub version: u8,
    pub is_disabled: u8,
    pub is_rebalancing: u8,
    pub padding: [u8; 1],
    pub admin: Pubkey,
    pub rebalance_authority: Pubkey,
    pub protocol_fee_beneficiary: Pubkey,
    pub pricing_program: Pubkey,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LstState {
    pub is_input_disabled: u8,
    pub reserves_bump: u8,
    pub protocol_fee_accumulator_bump: u8,
    pub padding: [u8; 5],
    pub sol_value: u64,
    pub mint: Pubkey,
    pub sol_value_calculator: Pubkey,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RebalanceRecord {
    pub old_total_sol_value: u64,
    pub padding: [u8; 4],
    pub dst_lst_index: u32,
}
