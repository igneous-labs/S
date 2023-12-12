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
pub enum SolValueCalculatorProgramIx {
    LstToSol(LstToSolIxArgs),
    SolToLst(SolToLstIxArgs),
}
impl SolValueCalculatorProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            LST_TO_SOL_IX_DISCM => Ok(Self::LstToSol(LstToSolIxArgs::deserialize(&mut reader)?)),
            SOL_TO_LST_IX_DISCM => Ok(Self::SolToLst(SolToLstIxArgs::deserialize(&mut reader)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::LstToSol(args) => {
                writer.write_all(&[LST_TO_SOL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SolToLst(args) => {
                writer.write_all(&[SOL_TO_LST_IX_DISCM])?;
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
pub const LST_TO_SOL_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct LstToSolAccounts<'me, 'info> {
    ///The LST mint
    pub lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct LstToSolKeys {
    ///The LST mint
    pub lst_mint: Pubkey,
}
impl From<LstToSolAccounts<'_, '_>> for LstToSolKeys {
    fn from(accounts: LstToSolAccounts) -> Self {
        Self {
            lst_mint: *accounts.lst_mint.key,
        }
    }
}
impl From<LstToSolKeys> for [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: LstToSolKeys) -> Self {
        [AccountMeta {
            pubkey: keys.lst_mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]> for LstToSolKeys {
    fn from(pubkeys: [Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<LstToSolAccounts<'_, 'info>> for [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: LstToSolAccounts<'_, 'info>) -> Self {
        [accounts.lst_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]>
    for LstToSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self { lst_mint: &arr[0] }
    }
}
pub const LST_TO_SOL_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LstToSolIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LstToSolIxData(pub LstToSolIxArgs);
impl From<LstToSolIxArgs> for LstToSolIxData {
    fn from(args: LstToSolIxArgs) -> Self {
        Self(args)
    }
}
impl LstToSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != LST_TO_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LST_TO_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LstToSolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[LST_TO_SOL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lst_to_sol_ix<K: Into<LstToSolKeys>, A: Into<LstToSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LstToSolKeys = accounts.into();
    let metas: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let args_full: LstToSolIxArgs = args.into();
    let data: LstToSolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lst_to_sol_invoke<'info, A: Into<LstToSolIxArgs>>(
    accounts: LstToSolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn lst_to_sol_invoke_signed<'info, A: Into<LstToSolIxArgs>>(
    accounts: LstToSolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn lst_to_sol_verify_account_keys(
    accounts: LstToSolAccounts<'_, '_>,
    keys: LstToSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.lst_mint.key, &keys.lst_mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const SOL_TO_LST_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct SolToLstAccounts<'me, 'info> {
    ///The LST mint
    pub lst_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SolToLstKeys {
    ///The LST mint
    pub lst_mint: Pubkey,
}
impl From<SolToLstAccounts<'_, '_>> for SolToLstKeys {
    fn from(accounts: SolToLstAccounts) -> Self {
        Self {
            lst_mint: *accounts.lst_mint.key,
        }
    }
}
impl From<SolToLstKeys> for [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: SolToLstKeys) -> Self {
        [AccountMeta {
            pubkey: keys.lst_mint,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]> for SolToLstKeys {
    fn from(pubkeys: [Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: pubkeys[0],
        }
    }
}
impl<'info> From<SolToLstAccounts<'_, 'info>> for [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolToLstAccounts<'_, 'info>) -> Self {
        [accounts.lst_mint.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]>
    for SolToLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self { lst_mint: &arr[0] }
    }
}
pub const SOL_TO_LST_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SolToLstIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolToLstIxData(pub SolToLstIxArgs);
impl From<SolToLstIxArgs> for SolToLstIxData {
    fn from(args: SolToLstIxArgs) -> Self {
        Self(args)
    }
}
impl SolToLstIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SOL_TO_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SOL_TO_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SolToLstIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SOL_TO_LST_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sol_to_lst_ix<K: Into<SolToLstKeys>, A: Into<SolToLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SolToLstKeys = accounts.into();
    let metas: [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] = keys.into();
    let args_full: SolToLstIxArgs = args.into();
    let data: SolToLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sol_to_lst_invoke<'info, A: Into<SolToLstIxArgs>>(
    accounts: SolToLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sol_to_lst_invoke_signed<'info, A: Into<SolToLstIxArgs>>(
    accounts: SolToLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn sol_to_lst_verify_account_keys(
    accounts: SolToLstAccounts<'_, '_>,
    keys: SolToLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.lst_mint.key, &keys.lst_mint)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
