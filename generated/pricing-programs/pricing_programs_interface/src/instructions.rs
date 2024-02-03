use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum PricingProgramsProgramIx {
    PriceExactIn(PriceExactInIxArgs),
    PriceExactOut(PriceExactOutIxArgs),
    PriceLpTokensToMint(PriceLpTokensToMintIxArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs),
}
impl PricingProgramsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            PRICE_EXACT_IN_IX_DISCM => Ok(Self::PriceExactIn(PriceExactInIxArgs::deserialize(
                &mut reader,
            )?)),
            PRICE_EXACT_OUT_IX_DISCM => Ok(Self::PriceExactOut(PriceExactOutIxArgs::deserialize(
                &mut reader,
            )?)),
            PRICE_LP_TOKENS_TO_MINT_IX_DISCM => Ok(Self::PriceLpTokensToMint(
                PriceLpTokensToMintIxArgs::deserialize(&mut reader)?,
            )),
            PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM => Ok(Self::PriceLpTokensToRedeem(
                PriceLpTokensToRedeemIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::PriceExactIn(args) => {
                writer.write_all(&[PRICE_EXACT_IN_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PriceExactOut(args) => {
                writer.write_all(&[PRICE_EXACT_OUT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PriceLpTokensToMint(args) => {
                writer.write_all(&[PRICE_LP_TOKENS_TO_MINT_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PriceLpTokensToRedeem(args) => {
                writer.write_all(&[PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
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
impl From<PriceExactInAccounts<'_, '_>> for PriceExactInKeys {
    fn from(accounts: PriceExactInAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<PriceExactInKeys> for [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: PriceExactInKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.input_lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_lst_mint,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PriceExactInAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_IN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PriceExactInAccounts<'_, 'info>) -> Self {
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
pub const PRICE_EXACT_IN_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactInIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactInIxData(pub PriceExactInIxArgs);
impl From<PriceExactInIxArgs> for PriceExactInIxData {
    fn from(args: PriceExactInIxArgs) -> Self {
        Self(args)
    }
}
impl PriceExactInIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PRICE_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactInIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_IN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn price_exact_in_ix_with_program_id(
    program_id: Pubkey,
    keys: PriceExactInKeys,
    args: PriceExactInIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRICE_EXACT_IN_IX_ACCOUNTS_LEN] = keys.into();
    let data: PriceExactInIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_in_ix(
    keys: PriceExactInKeys,
    args: PriceExactInIxArgs,
) -> std::io::Result<Instruction> {
    price_exact_in_ix_with_program_id(crate::ID, keys, args)
}
pub fn price_exact_in_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PriceExactInAccounts<'_, '_>,
    args: PriceExactInIxArgs,
) -> ProgramResult {
    let keys: PriceExactInKeys = accounts.into();
    let ix = price_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn price_exact_in_invoke(
    accounts: PriceExactInAccounts<'_, '_>,
    args: PriceExactInIxArgs,
) -> ProgramResult {
    price_exact_in_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn price_exact_in_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PriceExactInAccounts<'_, '_>,
    args: PriceExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PriceExactInKeys = accounts.into();
    let ix = price_exact_in_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn price_exact_in_invoke_signed(
    accounts: PriceExactInAccounts<'_, '_>,
    args: PriceExactInIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    price_exact_in_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn price_exact_in_verify_account_keys(
    accounts: PriceExactInAccounts<'_, '_>,
    keys: PriceExactInKeys,
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
impl From<PriceExactOutAccounts<'_, '_>> for PriceExactOutKeys {
    fn from(accounts: PriceExactOutAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<PriceExactOutKeys> for [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: PriceExactOutKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.input_lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_lst_mint,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PriceExactOutAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PriceExactOutAccounts<'_, 'info>) -> Self {
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
pub const PRICE_EXACT_OUT_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceExactOutIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceExactOutIxData(pub PriceExactOutIxArgs);
impl From<PriceExactOutIxArgs> for PriceExactOutIxData {
    fn from(args: PriceExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl PriceExactOutIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PRICE_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceExactOutIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_EXACT_OUT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn price_exact_out_ix_with_program_id(
    program_id: Pubkey,
    keys: PriceExactOutKeys,
    args: PriceExactOutIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRICE_EXACT_OUT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PriceExactOutIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_exact_out_ix(
    keys: PriceExactOutKeys,
    args: PriceExactOutIxArgs,
) -> std::io::Result<Instruction> {
    price_exact_out_ix_with_program_id(crate::ID, keys, args)
}
pub fn price_exact_out_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PriceExactOutAccounts<'_, '_>,
    args: PriceExactOutIxArgs,
) -> ProgramResult {
    let keys: PriceExactOutKeys = accounts.into();
    let ix = price_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn price_exact_out_invoke(
    accounts: PriceExactOutAccounts<'_, '_>,
    args: PriceExactOutIxArgs,
) -> ProgramResult {
    price_exact_out_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn price_exact_out_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PriceExactOutAccounts<'_, '_>,
    args: PriceExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PriceExactOutKeys = accounts.into();
    let ix = price_exact_out_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn price_exact_out_invoke_signed(
    accounts: PriceExactOutAccounts<'_, '_>,
    args: PriceExactOutIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    price_exact_out_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn price_exact_out_verify_account_keys(
    accounts: PriceExactOutAccounts<'_, '_>,
    keys: PriceExactOutKeys,
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
impl From<PriceLpTokensToMintAccounts<'_, '_>> for PriceLpTokensToMintKeys {
    fn from(accounts: PriceLpTokensToMintAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
        }
    }
}
impl From<PriceLpTokensToMintKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: PriceLpTokensToMintKeys) -> Self {
        [AccountMeta {
            pubkey: keys.input_lst_mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]> for PriceLpTokensToMintKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            input_lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<PriceLpTokensToMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PriceLpTokensToMintAccounts<'_, 'info>) -> Self {
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
pub const PRICE_LP_TOKENS_TO_MINT_IX_DISCM: u8 = 2u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToMintIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToMintIxData(pub PriceLpTokensToMintIxArgs);
impl From<PriceLpTokensToMintIxArgs> for PriceLpTokensToMintIxData {
    fn from(args: PriceLpTokensToMintIxArgs) -> Self {
        Self(args)
    }
}
impl PriceLpTokensToMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PRICE_LP_TOKENS_TO_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToMintIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_MINT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn price_lp_tokens_to_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: PriceLpTokensToMintKeys,
    args: PriceLpTokensToMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: PriceLpTokensToMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_mint_ix(
    keys: PriceLpTokensToMintKeys,
    args: PriceLpTokensToMintIxArgs,
) -> std::io::Result<Instruction> {
    price_lp_tokens_to_mint_ix_with_program_id(crate::ID, keys, args)
}
pub fn price_lp_tokens_to_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PriceLpTokensToMintAccounts<'_, '_>,
    args: PriceLpTokensToMintIxArgs,
) -> ProgramResult {
    let keys: PriceLpTokensToMintKeys = accounts.into();
    let ix = price_lp_tokens_to_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn price_lp_tokens_to_mint_invoke(
    accounts: PriceLpTokensToMintAccounts<'_, '_>,
    args: PriceLpTokensToMintIxArgs,
) -> ProgramResult {
    price_lp_tokens_to_mint_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn price_lp_tokens_to_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PriceLpTokensToMintAccounts<'_, '_>,
    args: PriceLpTokensToMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PriceLpTokensToMintKeys = accounts.into();
    let ix = price_lp_tokens_to_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn price_lp_tokens_to_mint_invoke_signed(
    accounts: PriceLpTokensToMintAccounts<'_, '_>,
    args: PriceLpTokensToMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    price_lp_tokens_to_mint_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn price_lp_tokens_to_mint_verify_account_keys(
    accounts: PriceLpTokensToMintAccounts<'_, '_>,
    keys: PriceLpTokensToMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.input_lst_mint.key, &keys.input_lst_mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
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
impl From<PriceLpTokensToRedeemAccounts<'_, '_>> for PriceLpTokensToRedeemKeys {
    fn from(accounts: PriceLpTokensToRedeemAccounts) -> Self {
        Self {
            output_lst_mint: *accounts.output_lst_mint.key,
        }
    }
}
impl From<PriceLpTokensToRedeemKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: PriceLpTokensToRedeemKeys) -> Self {
        [AccountMeta {
            pubkey: keys.output_lst_mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]> for PriceLpTokensToRedeemKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<PriceLpTokensToRedeemAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PriceLpTokensToRedeemAccounts<'_, 'info>) -> Self {
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
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM: u8 = 3u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLpTokensToRedeemIxArgs {
    pub amount: u64,
    pub sol_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PriceLpTokensToRedeemIxData(pub PriceLpTokensToRedeemIxArgs);
impl From<PriceLpTokensToRedeemIxArgs> for PriceLpTokensToRedeemIxData {
    fn from(args: PriceLpTokensToRedeemIxArgs) -> Self {
        Self(args)
    }
}
impl PriceLpTokensToRedeemIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PriceLpTokensToRedeemIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PRICE_LP_TOKENS_TO_REDEEM_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn price_lp_tokens_to_redeem_ix_with_program_id(
    program_id: Pubkey,
    keys: PriceLpTokensToRedeemKeys,
    args: PriceLpTokensToRedeemIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] = keys.into();
    let data: PriceLpTokensToRedeemIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn price_lp_tokens_to_redeem_ix(
    keys: PriceLpTokensToRedeemKeys,
    args: PriceLpTokensToRedeemIxArgs,
) -> std::io::Result<Instruction> {
    price_lp_tokens_to_redeem_ix_with_program_id(crate::ID, keys, args)
}
pub fn price_lp_tokens_to_redeem_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PriceLpTokensToRedeemAccounts<'_, '_>,
    args: PriceLpTokensToRedeemIxArgs,
) -> ProgramResult {
    let keys: PriceLpTokensToRedeemKeys = accounts.into();
    let ix = price_lp_tokens_to_redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn price_lp_tokens_to_redeem_invoke(
    accounts: PriceLpTokensToRedeemAccounts<'_, '_>,
    args: PriceLpTokensToRedeemIxArgs,
) -> ProgramResult {
    price_lp_tokens_to_redeem_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn price_lp_tokens_to_redeem_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PriceLpTokensToRedeemAccounts<'_, '_>,
    args: PriceLpTokensToRedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PriceLpTokensToRedeemKeys = accounts.into();
    let ix = price_lp_tokens_to_redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn price_lp_tokens_to_redeem_invoke_signed(
    accounts: PriceLpTokensToRedeemAccounts<'_, '_>,
    args: PriceLpTokensToRedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    price_lp_tokens_to_redeem_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn price_lp_tokens_to_redeem_verify_account_keys(
    accounts: PriceLpTokensToRedeemAccounts<'_, '_>,
    keys: PriceLpTokensToRedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.output_lst_mint.key, &keys.output_lst_mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
