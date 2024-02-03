use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum FlatFeeProgramIx {
    PriceExactIn(PriceExactInIxArgs),
    PriceExactOut(PriceExactOutIxArgs),
    PriceLpTokensToMint(PriceLpTokensToMintIxArgs),
    PriceLpTokensToRedeem(PriceLpTokensToRedeemIxArgs),
    SetLpWithdrawalFee(SetLpWithdrawalFeeIxArgs),
    SetLstFee(SetLstFeeIxArgs),
    RemoveLst,
    AddLst(AddLstIxArgs),
    SetManager,
    Initialize,
}
impl FlatFeeProgramIx {
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
            SET_LP_WITHDRAWAL_FEE_IX_DISCM => Ok(Self::SetLpWithdrawalFee(
                SetLpWithdrawalFeeIxArgs::deserialize(&mut reader)?,
            )),
            SET_LST_FEE_IX_DISCM => Ok(Self::SetLstFee(SetLstFeeIxArgs::deserialize(&mut reader)?)),
            REMOVE_LST_IX_DISCM => Ok(Self::RemoveLst),
            ADD_LST_IX_DISCM => Ok(Self::AddLst(AddLstIxArgs::deserialize(&mut reader)?)),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize),
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
            Self::SetLpWithdrawalFee(args) => {
                writer.write_all(&[SET_LP_WITHDRAWAL_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetLstFee(args) => {
                writer.write_all(&[SET_LST_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RemoveLst => writer.write_all(&[REMOVE_LST_IX_DISCM]),
            Self::AddLst(args) => {
                writer.write_all(&[ADD_LST_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetManager => writer.write_all(&[SET_MANAGER_IX_DISCM]),
            Self::Initialize => writer.write_all(&[INITIALIZE_IX_DISCM]),
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
pub const PRICE_EXACT_IN_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactInKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: Pubkey,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: Pubkey,
}
impl From<PriceExactInAccounts<'_, '_>> for PriceExactInKeys {
    fn from(accounts: PriceExactInAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
            input_fee_acc: *accounts.input_fee_acc.key,
            output_fee_acc: *accounts.output_fee_acc.key,
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
            AccountMeta {
                pubkey: keys.input_fee_acc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_fee_acc,
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
            input_fee_acc: pubkeys[2],
            output_fee_acc: pubkeys[3],
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
            accounts.input_fee_acc.clone(),
            accounts.output_fee_acc.clone(),
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
            input_fee_acc: &arr[2],
            output_fee_acc: &arr[3],
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
        (accounts.input_fee_acc.key, &keys.input_fee_acc),
        (accounts.output_fee_acc.key, &keys.output_fee_acc),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const PRICE_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutAccounts<'me, 'info> {
    ///Mint of the input LST
    pub input_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: &'me AccountInfo<'info>,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceExactOutKeys {
    ///Mint of the input LST
    pub input_lst_mint: Pubkey,
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///FeeAccount PDA for the input LST
    pub input_fee_acc: Pubkey,
    ///FeeAccount PDA for the output LST
    pub output_fee_acc: Pubkey,
}
impl From<PriceExactOutAccounts<'_, '_>> for PriceExactOutKeys {
    fn from(accounts: PriceExactOutAccounts) -> Self {
        Self {
            input_lst_mint: *accounts.input_lst_mint.key,
            output_lst_mint: *accounts.output_lst_mint.key,
            input_fee_acc: *accounts.input_fee_acc.key,
            output_fee_acc: *accounts.output_fee_acc.key,
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
            AccountMeta {
                pubkey: keys.input_fee_acc,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_fee_acc,
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
            input_fee_acc: pubkeys[2],
            output_fee_acc: pubkeys[3],
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
            accounts.input_fee_acc.clone(),
            accounts.output_fee_acc.clone(),
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
            input_fee_acc: &arr[2],
            output_fee_acc: &arr[3],
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
        (accounts.input_fee_acc.key, &keys.input_fee_acc),
        (accounts.output_fee_acc.key, &keys.output_fee_acc),
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
pub const PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemAccounts<'me, 'info> {
    ///Mint of the output LST
    pub output_lst_mint: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PriceLpTokensToRedeemKeys {
    ///Mint of the output LST
    pub output_lst_mint: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
}
impl From<PriceLpTokensToRedeemAccounts<'_, '_>> for PriceLpTokensToRedeemKeys {
    fn from(accounts: PriceLpTokensToRedeemAccounts) -> Self {
        Self {
            output_lst_mint: *accounts.output_lst_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<PriceLpTokensToRedeemKeys> for [AccountMeta; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: PriceLpTokensToRedeemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.output_lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]> for PriceLpTokensToRedeemKeys {
    fn from(pubkeys: [Pubkey; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<PriceLpTokensToRedeemAccounts<'_, 'info>>
    for [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PriceLpTokensToRedeemAccounts<'_, 'info>) -> Self {
        [accounts.output_lst_mint.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]>
    for PriceLpTokensToRedeemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PRICE_LP_TOKENS_TO_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            output_lst_mint: &arr[0],
            state: &arr[1],
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
    for (actual, expected) in [
        (accounts.output_lst_mint.key, &keys.output_lst_mint),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
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
impl From<SetLpWithdrawalFeeAccounts<'_, '_>> for SetLpWithdrawalFeeKeys {
    fn from(accounts: SetLpWithdrawalFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<SetLpWithdrawalFeeKeys> for [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLpWithdrawalFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
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
impl<'info> From<SetLpWithdrawalFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetLpWithdrawalFeeAccounts<'_, 'info>) -> Self {
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
pub const SET_LP_WITHDRAWAL_FEE_IX_DISCM: u8 = 250u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLpWithdrawalFeeIxArgs {
    pub lp_withdrawal_fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpWithdrawalFeeIxData(pub SetLpWithdrawalFeeIxArgs);
impl From<SetLpWithdrawalFeeIxArgs> for SetLpWithdrawalFeeIxData {
    fn from(args: SetLpWithdrawalFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetLpWithdrawalFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_LP_WITHDRAWAL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LP_WITHDRAWAL_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLpWithdrawalFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_LP_WITHDRAWAL_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_lp_withdrawal_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLpWithdrawalFeeKeys,
    args: SetLpWithdrawalFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LP_WITHDRAWAL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLpWithdrawalFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_withdrawal_fee_ix(
    keys: SetLpWithdrawalFeeKeys,
    args: SetLpWithdrawalFeeIxArgs,
) -> std::io::Result<Instruction> {
    set_lp_withdrawal_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_lp_withdrawal_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLpWithdrawalFeeAccounts<'_, '_>,
    args: SetLpWithdrawalFeeIxArgs,
) -> ProgramResult {
    let keys: SetLpWithdrawalFeeKeys = accounts.into();
    let ix = set_lp_withdrawal_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_lp_withdrawal_fee_invoke(
    accounts: SetLpWithdrawalFeeAccounts<'_, '_>,
    args: SetLpWithdrawalFeeIxArgs,
) -> ProgramResult {
    set_lp_withdrawal_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_lp_withdrawal_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLpWithdrawalFeeAccounts<'_, '_>,
    args: SetLpWithdrawalFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLpWithdrawalFeeKeys = accounts.into();
    let ix = set_lp_withdrawal_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lp_withdrawal_fee_invoke_signed(
    accounts: SetLpWithdrawalFeeAccounts<'_, '_>,
    args: SetLpWithdrawalFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lp_withdrawal_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_lp_withdrawal_fee_verify_account_keys(
    accounts: SetLpWithdrawalFeeAccounts<'_, '_>,
    keys: SetLpWithdrawalFeeKeys,
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
pub fn set_lp_withdrawal_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetLpWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lp_withdrawal_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetLpWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lp_withdrawal_fee_verify_account_privileges<'me, 'info>(
    accounts: SetLpWithdrawalFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lp_withdrawal_fee_verify_writable_privileges(accounts)?;
    set_lp_withdrawal_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LST_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetLstFeeAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///FeeAccount PDA to modify
    pub fee_acc: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLstFeeKeys {
    ///The program manager
    pub manager: Pubkey,
    ///FeeAccount PDA to modify
    pub fee_acc: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
}
impl From<SetLstFeeAccounts<'_, '_>> for SetLstFeeKeys {
    fn from(accounts: SetLstFeeAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            fee_acc: *accounts.fee_acc.key,
            state: *accounts.state.key,
        }
    }
}
impl From<SetLstFeeKeys> for [AccountMeta; SET_LST_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLstFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_acc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LST_FEE_IX_ACCOUNTS_LEN]> for SetLstFeeKeys {
    fn from(pubkeys: [Pubkey; SET_LST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            fee_acc: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<SetLstFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetLstFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.fee_acc.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]>
    for SetLstFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LST_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            fee_acc: &arr[1],
            state: &arr[2],
        }
    }
}
pub const SET_LST_FEE_IX_DISCM: u8 = 251u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLstFeeIxArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLstFeeIxData(pub SetLstFeeIxArgs);
impl From<SetLstFeeIxArgs> for SetLstFeeIxData {
    fn from(args: SetLstFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetLstFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_LST_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LST_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLstFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_LST_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_lst_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLstFeeKeys,
    args: SetLstFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_LST_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetLstFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lst_fee_ix(keys: SetLstFeeKeys, args: SetLstFeeIxArgs) -> std::io::Result<Instruction> {
    set_lst_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_lst_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLstFeeAccounts<'_, '_>,
    args: SetLstFeeIxArgs,
) -> ProgramResult {
    let keys: SetLstFeeKeys = accounts.into();
    let ix = set_lst_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_lst_fee_invoke(
    accounts: SetLstFeeAccounts<'_, '_>,
    args: SetLstFeeIxArgs,
) -> ProgramResult {
    set_lst_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_lst_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLstFeeAccounts<'_, '_>,
    args: SetLstFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLstFeeKeys = accounts.into();
    let ix = set_lst_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lst_fee_invoke_signed(
    accounts: SetLstFeeAccounts<'_, '_>,
    args: SetLstFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lst_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_lst_fee_verify_account_keys(
    accounts: SetLstFeeAccounts<'_, '_>,
    keys: SetLstFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lst_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetLstFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lst_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetLstFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lst_fee_verify_account_privileges<'me, 'info>(
    accounts: SetLstFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lst_fee_verify_writable_privileges(accounts)?;
    set_lst_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_LST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Account to refund SOL rent to
    pub refund_rent_to: &'me AccountInfo<'info>,
    ///FeeAccount PDA to be created
    pub fee_acc: &'me AccountInfo<'info>,
    ///Mint of the LST
    pub lst_mint: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Account to refund SOL rent to
    pub refund_rent_to: Pubkey,
    ///FeeAccount PDA to be created
    pub fee_acc: Pubkey,
    ///Mint of the LST
    pub lst_mint: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
}
impl From<RemoveLstAccounts<'_, '_>> for RemoveLstKeys {
    fn from(accounts: RemoveLstAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            refund_rent_to: *accounts.refund_rent_to.key,
            fee_acc: *accounts.fee_acc.key,
            lst_mint: *accounts.lst_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<RemoveLstKeys> for [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: RemoveLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.refund_rent_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_acc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]> for RemoveLstKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            refund_rent_to: pubkeys[1],
            fee_acc: pubkeys[2],
            lst_mint: pubkeys[3],
            state: pubkeys[4],
        }
    }
}
impl<'info> From<RemoveLstAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RemoveLstAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.refund_rent_to.clone(),
            accounts.fee_acc.clone(),
            accounts.lst_mint.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]>
    for RemoveLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            refund_rent_to: &arr[1],
            fee_acc: &arr[2],
            lst_mint: &arr[3],
            state: &arr[4],
        }
    }
}
pub const REMOVE_LST_IX_DISCM: u8 = 252u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLstIxData;
impl RemoveLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REMOVE_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_LST_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveLstKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveLstIxData.try_to_vec()?,
    })
}
pub fn remove_lst_ix(keys: RemoveLstKeys) -> std::io::Result<Instruction> {
    remove_lst_ix_with_program_id(crate::ID, keys)
}
pub fn remove_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLstAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveLstKeys = accounts.into();
    let ix = remove_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_lst_invoke(accounts: RemoveLstAccounts<'_, '_>) -> ProgramResult {
    remove_lst_invoke_with_program_id(crate::ID, accounts)
}
pub fn remove_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveLstKeys = accounts.into();
    let ix = remove_lst_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_lst_invoke_signed(
    accounts: RemoveLstAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_lst_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn remove_lst_verify_account_keys(
    accounts: RemoveLstAccounts<'_, '_>,
    keys: RemoveLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.refund_rent_to.key, &keys.refund_rent_to),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_lst_verify_writable_privileges<'me, 'info>(
    accounts: RemoveLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.refund_rent_to, accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_lst_verify_signer_privileges<'me, 'info>(
    accounts: RemoveLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_lst_verify_account_privileges<'me, 'info>(
    accounts: RemoveLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_lst_verify_writable_privileges(accounts)?;
    remove_lst_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_LST_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct AddLstAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///Account paying for FeeAccount's rent
    pub payer: &'me AccountInfo<'info>,
    ///FeeAccount PDA to be created
    pub fee_acc: &'me AccountInfo<'info>,
    ///Mint of the LST
    pub lst_mint: &'me AccountInfo<'info>,
    ///The program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLstKeys {
    ///The program manager
    pub manager: Pubkey,
    ///Account paying for FeeAccount's rent
    pub payer: Pubkey,
    ///FeeAccount PDA to be created
    pub fee_acc: Pubkey,
    ///Mint of the LST
    pub lst_mint: Pubkey,
    ///The program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<AddLstAccounts<'_, '_>> for AddLstKeys {
    fn from(accounts: AddLstAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            payer: *accounts.payer.key,
            fee_acc: *accounts.fee_acc.key,
            lst_mint: *accounts.lst_mint.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<AddLstKeys> for [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: AddLstKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_acc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lst_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_LST_IX_ACCOUNTS_LEN]> for AddLstKeys {
    fn from(pubkeys: [Pubkey; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            payer: pubkeys[1],
            fee_acc: pubkeys[2],
            lst_mint: pubkeys[3],
            state: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<AddLstAccounts<'_, 'info>> for [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: AddLstAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.payer.clone(),
            accounts.fee_acc.clone(),
            accounts.lst_mint.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]>
    for AddLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            payer: &arr[1],
            fee_acc: &arr[2],
            lst_mint: &arr[3],
            state: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const ADD_LST_IX_DISCM: u8 = 253u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLstIxArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLstIxData(pub AddLstIxArgs);
impl From<AddLstIxArgs> for AddLstIxData {
    fn from(args: AddLstIxArgs) -> Self {
        Self(args)
    }
}
impl AddLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ADD_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLstIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ADD_LST_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: AddLstKeys,
    args: AddLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_lst_ix(keys: AddLstKeys, args: AddLstIxArgs) -> std::io::Result<Instruction> {
    add_lst_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddLstAccounts<'_, '_>,
    args: AddLstIxArgs,
) -> ProgramResult {
    let keys: AddLstKeys = accounts.into();
    let ix = add_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_lst_invoke(accounts: AddLstAccounts<'_, '_>, args: AddLstIxArgs) -> ProgramResult {
    add_lst_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddLstAccounts<'_, '_>,
    args: AddLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddLstKeys = accounts.into();
    let ix = add_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_lst_invoke_signed(
    accounts: AddLstAccounts<'_, '_>,
    args: AddLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_lst_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn add_lst_verify_account_keys(
    accounts: AddLstAccounts<'_, '_>,
    keys: AddLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.payer.key, &keys.payer),
        (accounts.fee_acc.key, &keys.fee_acc),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.state.key, &keys.state),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_lst_verify_writable_privileges<'me, 'info>(
    accounts: AddLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.fee_acc] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_lst_verify_signer_privileges<'me, 'info>(
    accounts: AddLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_lst_verify_account_privileges<'me, 'info>(
    accounts: AddLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_lst_verify_writable_privileges(accounts)?;
    add_lst_verify_signer_privileges(accounts)?;
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
impl From<SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: SetManagerAccounts) -> Self {
        Self {
            current_manager: *accounts.current_manager.key,
            new_manager: *accounts.new_manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.current_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_manager,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
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
impl<'info> From<SetManagerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetManagerAccounts<'_, 'info>) -> Self {
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
pub const SET_MANAGER_IX_DISCM: u8 = 254u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetManagerIxData;
impl SetManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_MANAGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_MANAGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_MANAGER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: SetManagerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetManagerIxData.try_to_vec()?,
    })
}
pub fn set_manager_ix(keys: SetManagerKeys) -> std::io::Result<Instruction> {
    set_manager_ix_with_program_id(crate::ID, keys)
}
pub fn set_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetManagerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetManagerKeys = accounts.into();
    let ix = set_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_manager_invoke(accounts: SetManagerAccounts<'_, '_>) -> ProgramResult {
    set_manager_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetManagerKeys = accounts.into();
    let ix = set_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_manager_invoke_signed(
    accounts: SetManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_manager_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn set_manager_verify_account_keys(
    accounts: SetManagerAccounts<'_, '_>,
    keys: SetManagerKeys,
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
pub fn set_manager_verify_writable_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_manager_verify_signer_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.current_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_manager_verify_account_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_manager_verify_writable_privileges(accounts)?;
    set_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///Account paying for ProgramState's rent
    pub payer: &'me AccountInfo<'info>,
    ///Program state PDA
    pub state: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///Account paying for ProgramState's rent
    pub payer: Pubkey,
    ///Program state PDA
    pub state: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            state: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_IX_DISCM: u8 = 255u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData;
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeIxData.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(accounts: InitializeAccounts<'_, '_>) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
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
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
