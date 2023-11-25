use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SplStakePool {
    pub account_type: AccountType,
    pub manager: Pubkey,
    pub staker: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub stake_withdraw_bump_seed: u8,
    pub validator_list: Pubkey,
    pub reserve_stake: Pubkey,
    pub pool_mint: Pubkey,
    pub manager_fee_account: Pubkey,
    pub token_program_id: Pubkey,
    pub total_lamports: u64,
    pub pool_token_supply: u64,
    pub last_update_epoch: u64,
    pub lockup: Lockup,
    pub epoch_fee: Fee,
    pub next_epoch_fee: FutureEpochFee,
    pub preferred_deposit_validator_vote_address: Option<Pubkey>,
    pub preferred_withdraw_validator_vote_address: Option<Pubkey>,
    pub stake_deposit_fee: Fee,
    pub stake_withdrawal_fee: Fee,
    pub next_stake_withdrawal_fee: FutureEpochFee,
    pub stake_referral_fee: u8,
    pub sol_deposit_authority: Option<Pubkey>,
    pub sol_deposit_fee: Fee,
    pub sol_referral_fee: u8,
    pub sol_withdraw_authority: Option<Pubkey>,
    pub sol_withdrawal_fee: Fee,
    pub next_sol_withdrawal_fee: FutureEpochFee,
    pub last_epoch_pool_token_supply: u64,
    pub last_epoch_total_lamports: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountType {
    Uninitialized,
    StakePool,
    ValidatorList,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lockup {
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fee {
    pub denominator: u64,
    pub numerator: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FutureEpochFee {
    None,
    One(Fee),
    Two(Fee),
}
