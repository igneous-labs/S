use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProgramState {
    pub manager: Pubkey,
    pub lp_withdrawal_fee_bps: u16,
}
#[repr(C)]
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq, Pod, Copy, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeAccount {
    pub bump: u8,
    pub padding: u8,
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
