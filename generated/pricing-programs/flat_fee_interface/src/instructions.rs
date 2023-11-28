use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum FlatFeeProgramIx {
    PriceExactIn(PriceExactInIxArgs),
    PriceExactOut(PriceExactOutIxArgs),
    PriceLpTokensToMint(PriceLpTokensToMintIxArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs),
    SetLpWithdrawalFee(SetLpWithdrawalFeeIxArgs),
    SetFee(SetFeeIxArgs),
    SetManager(SetManagerIxArgs),
    Init(InitIxArgs),
}
impl BorshSerialize for FlatFeeProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::PriceExactIn(args) => {
                PRICE_EXACT_IN_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceExactOut(args) => {
                PRICE_EXACT_OUT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceLpTokensToMint(args) => {
                PRICE_LP_TOKENS_TO_MINT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PriceLpTokensToRedeem(args) => {
                PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetLpWithdrawalFee(args) => {
                SET_LP_WITHDRAWAL_FEE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetFee(args) => {
                SET_FEE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetManager(args) => {
                SET_MANAGER_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Init(args) => {
                INIT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl FlatFeeProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            PRICE_EXACT_IN_IX_DISCM => {
                Ok(Self::PriceExactIn(PriceExactInIxArgs::deserialize(buf)?))
            }
            PRICE_EXACT_OUT_IX_DISCM => {
                Ok(Self::PriceExactOut(PriceExactOutIxArgs::deserialize(buf)?))
            }
            PRICE_LP_TOKENS_TO_MINT_IX_DISCM => Ok(Self::PriceLpTokensToMint(
                PriceLpTokensToMintIxArgs::deserialize(buf)?,
            )),
            PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM => Ok(Self::PriceLpTokensToRedeem(
                PriceLpTokensToRedeemIxArgs::deserialize(buf)?,
            )),
            SET_LP_WITHDRAWAL_FEE_IX_DISCM => Ok(Self::SetLpWithdrawalFee(
                SetLpWithdrawalFeeIxArgs::deserialize(buf)?,
            )),
            SET_FEE_IX_DISCM => Ok(Self::SetFee(SetFeeIxArgs::deserialize(buf)?)),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager(SetManagerIxArgs::deserialize(buf)?)),
            INIT_IX_DISCM => Ok(Self::Init(InitIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const PRICE_EXACT_IN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInAccounts<'me, 'info> {
    ///Input LST token mint
    pub lst_input: &'me AccountInfo<'info>,
    ///Output LST token mint
    pub lst_output: &'me AccountInfo<'info>,
    ///FeeAccount PDA for input LST
    pub fee_acc_input: &'me AccountInfo<'info>,
    ///FeeAccount PDA for output LST
    pub fee_acc_output: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInKeys {
    ///Input LST token mint
    pub lst_input: Pubkey,
    ///Output LST token mint
    pub lst_output: Pubkey,
    ///FeeAccount PDA for input LST
    pub fee_acc_input: Pubkey,
    ///FeeAccount PDA for output LST
    pub fee_acc_output: Pubkey,
}
impl From<&PriceExactInAccounts<'_, '_>> for PriceExactInKeys {
    fn from(accounts: &PriceExactInAccounts) -> Self {
        Self {
            lst_input: *accounts.lst_input.key,
            lst_output: *accounts.lst_output.key,
            fee_acc_input: *accounts.fee_acc_input.key,
            fee_acc_output: *accounts.fee_acc_output.key,
        }
    }
}
impl From<&PriceExactInKeys> for [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactInKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst_input, false),
            AccountMeta::new_readonly(keys.lst_output, false),
            AccountMeta::new_readonly(keys.fee_acc_input, false),
            AccountMeta::new_readonly(keys.fee_acc_output, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]> for PriceExactInKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_input: pubkeys[0],
            lst_output: pubkeys[1],
            fee_acc_input: pubkeys[2],
            fee_acc_output: pubkeys[3],
        }
    }
}
impl<'info> From<&PriceExactInAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.lst_input.clone(),
            accounts.lst_output.clone(),
            accounts.fee_acc_input.clone(),
            accounts.fee_acc_output.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]>
    for PriceExactInAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_input: &arr[0],
            lst_output: &arr[1],
            fee_acc_input: &arr[2],
            fee_acc_output: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactInIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactInIxData(pub PriceExactInIxArgs);
pub const PRICE_EXACT_IN_IX_DISCM: u8 = 0u8;
impl From<PriceExactInIxArgs> for PriceExactInIxData {
    fn from(args: PriceExactInIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceExactInIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_IN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceExactInIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactInIxArgs::deserialize(buf)?))
    }
}
pub fn price_exact_in_ix<K: Into<PriceExactInKeys>, A: Into<PriceExactInIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceExactInKeys = accounts.into();
    let metas: [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceExactInIxArgs = args.into();
    let data: PriceExactInIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_in_invoke<'info, A: Into<PriceExactInIxArgs>>(
    accounts: &PriceExactInAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_exact_in_invoke_signed<'info, A: Into<PriceExactInIxArgs>>(
    accounts: &PriceExactInAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_exact_in_verify_account_keys(
    accounts: &PriceExactInAccounts<'_, '_>,
    keys: &PriceExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_input.key, &keys.lst_input),
        (accounts.lst_output.key, &keys.lst_output),
        (accounts.fee_acc_input.key, &keys.fee_acc_input),
        (accounts.fee_acc_output.key, &keys.fee_acc_output),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_exact_in_verify_account_privileges<'me, 'info>(
    accounts: &PriceExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutAccounts<'me, 'info> {
    ///Input LST token mint
    pub lst_input: &'me AccountInfo<'info>,
    ///Output LST token mint
    pub lst_output: &'me AccountInfo<'info>,
    ///FeeAccount PDA for input LST
    pub fee_acc_input: &'me AccountInfo<'info>,
    ///FeeAccount PDA for output LST
    pub fee_acc_output: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutKeys {
    ///Input LST token mint
    pub lst_input: Pubkey,
    ///Output LST token mint
    pub lst_output: Pubkey,
    ///FeeAccount PDA for input LST
    pub fee_acc_input: Pubkey,
    ///FeeAccount PDA for output LST
    pub fee_acc_output: Pubkey,
}
impl From<&PriceExactOutAccounts<'_, '_>> for PriceExactOutKeys {
    fn from(accounts: &PriceExactOutAccounts) -> Self {
        Self {
            lst_input: *accounts.lst_input.key,
            lst_output: *accounts.lst_output.key,
            fee_acc_input: *accounts.fee_acc_input.key,
            fee_acc_output: *accounts.fee_acc_output.key,
        }
    }
}
impl From<&PriceExactOutKeys> for [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactOutKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst_input, false),
            AccountMeta::new_readonly(keys.lst_output, false),
            AccountMeta::new_readonly(keys.fee_acc_input, false),
            AccountMeta::new_readonly(keys.fee_acc_output, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]> for PriceExactOutKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_input: pubkeys[0],
            lst_output: pubkeys[1],
            fee_acc_input: pubkeys[2],
            fee_acc_output: pubkeys[3],
        }
    }
}
impl<'info> From<&PriceExactOutAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.lst_input.clone(),
            accounts.lst_output.clone(),
            accounts.fee_acc_input.clone(),
            accounts.fee_acc_output.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]>
    for PriceExactOutAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_input: &arr[0],
            lst_output: &arr[1],
            fee_acc_input: &arr[2],
            fee_acc_output: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactOutIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactOutIxData(pub PriceExactOutIxArgs);
pub const PRICE_EXACT_OUT_IX_DISCM: u8 = 1u8;
impl From<PriceExactOutIxArgs> for PriceExactOutIxData {
    fn from(args: PriceExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceExactOutIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_OUT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceExactOutIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactOutIxArgs::deserialize(buf)?))
    }
}
pub fn price_exact_out_ix<K: Into<PriceExactOutKeys>, A: Into<PriceExactOutIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceExactOutKeys = accounts.into();
    let metas: [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceExactOutIxArgs = args.into();
    let data: PriceExactOutIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_out_invoke<'info, A: Into<PriceExactOutIxArgs>>(
    accounts: &PriceExactOutAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_exact_out_invoke_signed<'info, A: Into<PriceExactOutIxArgs>>(
    accounts: &PriceExactOutAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_exact_out_verify_account_keys(
    accounts: &PriceExactOutAccounts<'_, '_>,
    keys: &PriceExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_input.key, &keys.lst_input),
        (accounts.lst_output.key, &keys.lst_output),
        (accounts.fee_acc_input.key, &keys.fee_acc_input),
        (accounts.fee_acc_output.key, &keys.fee_acc_output),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_exact_out_verify_account_privileges<'me, 'info>(
    accounts: &PriceExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToMintAccounts<'me, 'info> {
    ///Input LST token mint
    pub lst_input: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToMintKeys {
    ///Input LST token mint
    pub lst_input: Pubkey,
}
impl From<&PriceLpTokensToMintAccounts<'_, '_>> for PriceLpTokensToMintKeys {
    fn from(accounts: &PriceLpTokensToMintAccounts) -> Self {
        Self {
            lst_input: *accounts.lst_input.key,
        }
    }
}
impl From<&PriceLpTokensToMintKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToMintKeys) -> Self {
        [AccountMeta::new_readonly(keys.lst_input, false)]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]> for PriceLpTokensToMintKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_input: pubkeys[0],
        }
    }
}
impl<'info> From<&PriceLpTokensToMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToMintAccounts<'_, 'info>) -> Self {
        [accounts.lst_input.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self { lst_input: &arr[0] }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToMintIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToMintIxData(pub PriceLpTokensToMintIxArgs);
pub const PRICE_LP_TOKENS_TO_MINT_IX_DISCM: u8 = 2u8;
impl From<PriceLpTokensToMintIxArgs> for PriceLpTokensToMintIxData {
    fn from(args: PriceLpTokensToMintIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceLpTokensToMintIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_MINT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceLpTokensToMintIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_LP_TOKENS_TO_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToMintIxArgs::deserialize(buf)?))
    }
}
pub fn price_lp_tokens_to_mint_ix<
    K: Into<PriceLpTokensToMintKeys>,
    A: Into<PriceLpTokensToMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceLpTokensToMintKeys = accounts.into();
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceLpTokensToMintIxArgs = args.into();
    let data: PriceLpTokensToMintIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_mint_invoke<'info, A: Into<PriceLpTokensToMintIxArgs>>(
    accounts: &PriceLpTokensToMintAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_lp_tokens_to_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_lp_tokens_to_mint_invoke_signed<'info, A: Into<PriceLpTokensToMintIxArgs>>(
    accounts: &PriceLpTokensToMintAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_lp_tokens_to_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_lp_tokens_to_mint_verify_account_keys(
    accounts: &PriceLpTokensToMintAccounts<'_, '_>,
    keys: &PriceLpTokensToMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.lst_input.key, &keys.lst_input)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_lp_tokens_to_mint_verify_account_privileges<'me, 'info>(
    accounts: &PriceLpTokensToMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemAccounts<'me, 'info> {
    ///Output LST token mint
    pub lst_output: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemKeys {
    ///Output LST token mint
    pub lst_output: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
}
impl From<&PriceLpTokensToRedeemAccounts<'_, '_>> for PriceLpTokensToRedeemKeys {
    fn from(accounts: &PriceLpTokensToRedeemAccounts) -> Self {
        Self {
            lst_output: *accounts.lst_output.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&PriceLpTokensToRedeemKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToRedeemKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst_output, false),
            AccountMeta::new_readonly(keys.state, false),
        ]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]> for PriceLpTokensToRedeemKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_output: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&PriceLpTokensToRedeemAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>) -> Self {
        [accounts.lst_output.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToRedeemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_output: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToRedeemIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToRedeemIxData(pub PriceLpTokensToRedeemIxArgs);
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM: u8 = 3u8;
impl From<PriceLpTokensToRedeemIxArgs> for PriceLpTokensToRedeemIxData {
    fn from(args: PriceLpTokensToRedeemIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PriceLpTokensToRedeemIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PriceLpTokensToRedeemIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToRedeemIxArgs::deserialize(buf)?))
    }
}
pub fn price_lp_tokens_to_redeem_ix<
    K: Into<PriceLpTokensToRedeemKeys>,
    A: Into<PriceLpTokensToRedeemIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PriceLpTokensToRedeemKeys = accounts.into();
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PriceLpTokensToRedeemIxArgs = args.into();
    let data: PriceLpTokensToRedeemIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_redeem_invoke<'info, A: Into<PriceLpTokensToRedeemIxArgs>>(
    accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = price_lp_tokens_to_redeem_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn price_lp_tokens_to_redeem_invoke_signed<'info, A: Into<PriceLpTokensToRedeemIxArgs>>(
    accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = price_lp_tokens_to_redeem_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn price_lp_tokens_to_redeem_verify_account_keys(
    accounts: &PriceLpTokensToRedeemAccounts<'_, '_>,
    keys: &PriceLpTokensToRedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_output.key, &keys.lst_output),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn price_lp_tokens_to_redeem_verify_account_privileges<'me, 'info>(
    accounts: &PriceLpTokensToRedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLpWithdrawalFeeAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLpWithdrawalFeeKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
}
impl From<&SetLpWithdrawalFeeAccounts<'_, '_>> for SetLpWithdrawalFeeKeys {
    fn from(accounts: &SetLpWithdrawalFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetLpWithdrawalFeeKeys> for [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetLpWithdrawalFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]> for SetLpWithdrawalFeeKeys {
    fn from(pubkeys: [Pubkey; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&SetLpWithdrawalFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>) -> Self {
        [accounts.manager.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]>
    for SetLpWithdrawalFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLpWithdrawalFeeIxArgs {
    pub lp_withdrawal_fee: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpWithdrawalFeeIxData(pub SetLpWithdrawalFeeIxArgs);
pub const SET_LP_WITHDRAWAL_FEE_IX_DISCM: u8 = 252u8;
impl From<SetLpWithdrawalFeeIxArgs> for SetLpWithdrawalFeeIxData {
    fn from(args: SetLpWithdrawalFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetLpWithdrawalFeeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_LP_WITHDRAWAL_FEE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetLpWithdrawalFeeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_LP_WITHDRAWAL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LP_WITHDRAWAL_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLpWithdrawalFeeIxArgs::deserialize(buf)?))
    }
}
pub fn set_lp_withdrawal_fee_ix<
    K: Into<SetLpWithdrawalFeeKeys>,
    A: Into<SetLpWithdrawalFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetLpWithdrawalFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetLpWithdrawalFeeIxArgs = args.into();
    let data: SetLpWithdrawalFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_withdrawal_fee_invoke<'info, A: Into<SetLpWithdrawalFeeIxArgs>>(
    accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_lp_withdrawal_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_lp_withdrawal_fee_invoke_signed<'info, A: Into<SetLpWithdrawalFeeIxArgs>>(
    accounts: &SetLpWithdrawalFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_lp_withdrawal_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_lp_withdrawal_fee_verify_account_keys(
    accounts: &SetLpWithdrawalFeeAccounts<'_, '_>,
    keys: &SetLpWithdrawalFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lp_withdrawal_fee_verify_account_privileges<'me, 'info>(
    accounts: &SetLpWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///FeeAccount PDA to modify
    pub fee_acc: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeKeys {
    ///The program manager
    pub manager: Pubkey,
    ///FeeAccount PDA to modify
    pub fee_acc: Pubkey,
}
impl From<&SetFeeAccounts<'_, '_>> for SetFeeKeys {
    fn from(accounts: &SetFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fee_acc: *accounts.fee_acc.key,
        }
    }
}
impl From<&SetFeeKeys> for [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.fee_acc, false),
        ]
    }
}
impl From<[Pubkey; SET_FEE_IX_ACCOUNTS_LEN]> for SetFeeKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fee_acc: pubkeys[1],
        }
    }
}
impl<'info> From<&SetFeeAccounts<'_, 'info>> for [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SetFeeAccounts<'_, 'info>) -> Self {
        [accounts.manager.clone(), accounts.fee_acc.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]>
    for SetFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fee_acc: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetFeeIxArgs {
    pub input_fee: i16,
    pub output_fee: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeIxData(pub SetFeeIxArgs);
pub const SET_FEE_IX_DISCM: u8 = 253u8;
impl From<SetFeeIxArgs> for SetFeeIxData {
    fn from(args: SetFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetFeeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_FEE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetFeeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetFeeIxArgs::deserialize(buf)?))
    }
}
pub fn set_fee_ix<K: Into<SetFeeKeys>, A: Into<SetFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetFeeIxArgs = args.into();
    let data: SetFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_invoke<'info, A: Into<SetFeeIxArgs>>(
    accounts: &SetFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_fee_invoke_signed<'info, A: Into<SetFeeIxArgs>>(
    accounts: &SetFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_fee_verify_account_keys(
    accounts: &SetFeeAccounts<'_, '_>,
    keys: &SetFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.fee_acc.key, &keys.fee_acc),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_fee_verify_account_privileges<'me, 'info>(
    accounts: &SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_MANAGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetManagerAccounts<'me, 'info> {
    ///The current program manager
    pub current_manager: &'me AccountInfo<'info>,
    ///The new program manager to set to
    pub new_manager: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetManagerKeys {
    ///The current program manager
    pub current_manager: Pubkey,
    ///The new program manager to set to
    pub new_manager: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
}
impl From<&SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: &SetManagerAccounts) -> Self {
        Self {
            current_manager: *accounts.current_manager.key,
            new_manager: *accounts.new_manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetManagerKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.current_manager, true),
            AccountMeta::new_readonly(keys.new_manager, false),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]> for SetManagerKeys {
    fn from(pubkeys: [Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_manager: pubkeys[0],
            new_manager: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetManagerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.current_manager.clone(),
            accounts.new_manager.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]>
    for SetManagerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_manager: &arr[0],
            new_manager: &arr[1],
            state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetManagerIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetManagerIxData(pub SetManagerIxArgs);
pub const SET_MANAGER_IX_DISCM: u8 = 254u8;
impl From<SetManagerIxArgs> for SetManagerIxData {
    fn from(args: SetManagerIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetManagerIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_MANAGER_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetManagerIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_MANAGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_MANAGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetManagerIxArgs::deserialize(buf)?))
    }
}
pub fn set_manager_ix<K: Into<SetManagerKeys>, A: Into<SetManagerIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetManagerKeys = accounts.into();
    let metas: [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetManagerIxArgs = args.into();
    let data: SetManagerIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_manager_invoke<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_manager_invoke_signed<'info, A: Into<SetManagerIxArgs>>(
    accounts: &SetManagerAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_manager_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_manager_verify_account_keys(
    accounts: &SetManagerAccounts<'_, '_>,
    keys: &SetManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.current_manager.key, &keys.current_manager),
        (accounts.new_manager.key, &keys.new_manager),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_manager_verify_account_privileges<'me, 'info>(
    accounts: &SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.current_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitAccounts<'me, 'info> {
    ///The account paying for ProgramState's rent
    pub payer: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitKeys {
    ///The account paying for ProgramState's rent
    pub payer: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&InitAccounts<'_, '_>> for InitKeys {
    fn from(accounts: &InitAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitKeys> for [AccountMeta; INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INIT_IX_ACCOUNTS_LEN]> for InitKeys {
    fn from(pubkeys: [Pubkey; INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<&InitAccounts<'_, 'info>> for [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: &InitAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN]>
    for InitAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            state: &arr[1],
            system_program: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct InitIxData(pub InitIxArgs);
pub const INIT_IX_DISCM: u8 = 255u8;
impl From<InitIxArgs> for InitIxData {
    fn from(args: InitIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[INIT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl InitIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != INIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitIxArgs::deserialize(buf)?))
    }
}
pub fn init_ix<K: Into<InitKeys>, A: Into<InitIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitKeys = accounts.into();
    let metas: [AccountMeta; INIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitIxArgs = args.into();
    let data: InitIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_invoke<'info, A: Into<InitIxArgs>>(
    accounts: &InitAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = init_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn init_invoke_signed<'info, A: Into<InitIxArgs>>(
    accounts: &InitAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = init_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn init_verify_account_keys(
    accounts: &InitAccounts<'_, '_>,
    keys: &InitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.state.key, &keys.state),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn init_verify_account_privileges<'me, 'info>(
    accounts: &InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
