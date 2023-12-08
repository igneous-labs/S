use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lido {
    pub account_type: u8,
    pub lido_version: u8,
    pub manager: Pubkey,
    pub st_sol_mint: Pubkey,
    pub exchange_rate: ExchangeRate,
    pub sol_reserve_account_bump_seed: u8,
    pub stake_authority_bump_seed: u8,
    pub mint_authority_bump_seed: u8,
    pub reward_distribution: RewardDistribution,
    pub fee_recipients: FeeRecipients,
    pub metrics: Metrics,
    pub criteria: Criteria,
    pub validator_list: Pubkey,
    pub validator_perf_list: Pubkey,
    pub maintainer_list: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExchangeRate {
    pub computed_in_epoch: u64,
    pub st_sol_supply: u64,
    pub sol_balance: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RewardDistribution {
    pub treasury_fee: u32,
    pub developer_fee: u32,
    pub st_sol_appreciation: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeRecipients {
    pub treasury_account: Pubkey,
    pub developer_account: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metrics {
    pub fee_treasury_total_lamports: u64,
    pub fee_validation_total_lamports: u64,
    pub fee_developer_total_lamports: u64,
    pub st_sol_appreciation_total_lamports: u64,
    pub fee_treasury_total_st_lamports: u64,
    pub fee_validation_total_st_lamports: u64,
    pub fee_developer_total_st_lamports: u64,
    pub deposit_amount: LamportsHistogram,
    pub withdraw_amount: WithdrawMetric,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LamportsHistogram {
    pub counts: [u64; 12],
    pub total: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawMetric {
    pub total_st_sol_amount: u64,
    pub total_sol_amount: u64,
    pub count: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Criteria {
    pub max_commission: u8,
    pub min_block_production_rate: u64,
    pub min_vote_success_rate: u64,
}
