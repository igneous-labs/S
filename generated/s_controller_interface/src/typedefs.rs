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
    pub admin: Pubkey,
    pub rebalancing_authority: Pubkey,
    pub protocol_fee_beneficiary: Pubkey,
    pub pricing_program: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LstState {
    pub is_input_disabled: u8,
    pub sol_value: u64,
    pub token: Pubkey,
    pub sol_value_calculator: Pubkey,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LstStateList {
    pub lst_states: Vec<LstState>,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisablePoolAuthorityList {
    pub whitelisted_pubkeys: Vec<Pubkey>,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RebalanceRecord {
    pub sol_value: u64,
    pub dst_lst_index: u64,
    pub dst_lst_value_calc_accs: u8,
    pub dst_lst: Pubkey,
}
