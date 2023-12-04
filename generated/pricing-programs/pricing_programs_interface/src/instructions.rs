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
pub enum PricingProgramsProgramIx {
    PriceExactIn(PriceExactInIxArgs),
    PriceExactOut(PriceExactOutIxArgs),
    PriceLpTokensToMint(PriceLpTokensToMintIxArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs),
}
impl BorshSerialize for PricingProgramsProgramIx {
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
        }
    }
}
impl PricingProgramsProgramIx {
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
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const PRICE_EXACT_IN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
}
impl From<&PriceExactInAccounts<'_, '_>> for PriceExactInKeys {
    fn from(accounts: &PriceExactInAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<&PriceExactInKeys> for [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactInKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.input_lst_mint, false),
            AccountMeta::new_readonly(keys.output_lst_mint, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]> for PriceExactInKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
            output_lst_mint: pubkeys[1],
        }
    }
}
impl<'info> From<&PriceExactInAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.input_lst_mint.clone(),
            accounts.output_lst_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]>
    for PriceExactInAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
            output_lst_mint: &arr[1],
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
        (accounts.input_lst_mint.key, &keys.input_lst_mint),
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
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
pub const PRICE_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
}
impl From<&PriceExactOutAccounts<'_, '_>> for PriceExactOutKeys {
    fn from(accounts: &PriceExactOutAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<&PriceExactOutKeys> for [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceExactOutKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.input_lst_mint, false),
            AccountMeta::new_readonly(keys.output_lst_mint, false),
        ]
    }
}
impl From<[Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]> for PriceExactOutKeys {
    fn from(pubkeys: [Pubkey; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
            output_lst_mint: pubkeys[1],
        }
    }
}
impl<'info> From<&PriceExactOutAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.input_lst_mint.clone(),
            accounts.output_lst_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]>
    for PriceExactOutAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
            output_lst_mint: &arr[1],
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
        (accounts.input_lst_mint.key, &keys.input_lst_mint),
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
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
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToMintKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
}
impl From<&PriceLpTokensToMintAccounts<'_, '_>> for PriceLpTokensToMintKeys {
    fn from(accounts: &PriceLpTokensToMintAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
        }
    }
}
impl From<&PriceLpTokensToMintKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToMintKeys) -> Self {
        [AccountMeta::new_readonly(keys.input_lst_mint, false)]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]> for PriceLpTokensToMintKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<&PriceLpTokensToMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToMintAccounts<'_, 'info>) -> Self {
        [accounts.input_lst_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: &arr[0],
        }
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
    for (actual, expected) in [(accounts.input_lst_mint.key, &keys.input_lst_mint)] {
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
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemAccounts<'me, 'info> {
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemKeys {
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
}
impl From<&PriceLpTokensToRedeemAccounts<'_, '_>> for PriceLpTokensToRedeemKeys {
    fn from(accounts: &PriceLpTokensToRedeemAccounts) -> Self {
        Self {
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<&PriceLpTokensToRedeemKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: &PriceLpTokensToRedeemKeys) -> Self {
        [AccountMeta::new_readonly(keys.output_lst_mint, false)]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]> for PriceLpTokensToRedeemKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<&PriceLpTokensToRedeemAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PriceLpTokensToRedeemAccounts<'_, 'info>) -> Self {
        [accounts.output_lst_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToRedeemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: &arr[0],
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
    for (actual, expected) in [(accounts.output_lst_mint.key, &keys.output_lst_mint)] {
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
