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
pub enum GenericPoolCalculatorProgramIx {
    LstToSol(LstToSolIxArgs),
    SolToLst(SolToLstIxArgs),
    UpdateLastUpgradeSlot,
    SetManager,
    Init,
}
impl GenericPoolCalculatorProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            LST_TO_SOL_IX_DISCM => Ok(Self::LstToSol(LstToSolIxArgs::deserialize(&mut reader)?)),
            SOL_TO_LST_IX_DISCM => Ok(Self::SolToLst(SolToLstIxArgs::deserialize(&mut reader)?)),
            UPDATE_LAST_UPGRADE_SLOT_IX_DISCM => Ok(Self::UpdateLastUpgradeSlot),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager),
            INIT_IX_DISCM => Ok(Self::Init),
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
            Self::UpdateLastUpgradeSlot => writer.write_all(&[UPDATE_LAST_UPGRADE_SLOT_IX_DISCM]),
            Self::SetManager => writer.write_all(&[SET_MANAGER_IX_DISCM]),
            Self::Init => writer.write_all(&[INIT_IX_DISCM]),
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
pub const LST_TO_SOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct LstToSolAccounts<'me, 'info> {
    ///The LST mint
    pub lst_mint: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The main stake pool state account
    pub pool_state: &'me AccountInfo<'info>,
    ///The stake pool program
    pub pool_program: &'me AccountInfo<'info>,
    ///The stake pool program executable data
    pub pool_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct LstToSolKeys {
    ///The LST mint
    pub lst_mint: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
    ///The main stake pool state account
    pub pool_state: Pubkey,
    ///The stake pool program
    pub pool_program: Pubkey,
    ///The stake pool program executable data
    pub pool_program_data: Pubkey,
}
impl From<LstToSolAccounts<'_, '_>> for LstToSolKeys {
    fn from(accounts: LstToSolAccounts) -> Self {
        Self {
            lst_mint: *accounts.lst_mint.key,
            state: *accounts.state.key,
            pool_state: *accounts.pool_state.key,
            pool_program: *accounts.pool_program.key,
            pool_program_data: *accounts.pool_program_data.key,
        }
    }
}
impl From<LstToSolKeys> for [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: LstToSolKeys) -> Self {
        [
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
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program_data,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]> for LstToSolKeys {
    fn from(pubkeys: [Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: pubkeys[0],
            state: pubkeys[1],
            pool_state: pubkeys[2],
            pool_program: pubkeys[3],
            pool_program_data: pubkeys[4],
        }
    }
}
impl<'info> From<LstToSolAccounts<'_, 'info>> for [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    fn from(accounts: LstToSolAccounts<'_, 'info>) -> Self {
        [
            accounts.lst_mint.clone(),
            accounts.state.clone(),
            accounts.pool_state.clone(),
            accounts.pool_program.clone(),
            accounts.pool_program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]>
    for LstToSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: &arr[0],
            state: &arr[1],
            pool_state: &arr[2],
            pool_program: &arr[3],
            pool_program_data: &arr[4],
        }
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
pub fn lst_to_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: LstToSolKeys,
    args: LstToSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: LstToSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lst_to_sol_ix(keys: LstToSolKeys, args: LstToSolIxArgs) -> std::io::Result<Instruction> {
    lst_to_sol_ix_with_program_id(crate::ID, keys, args)
}
pub fn lst_to_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LstToSolAccounts<'_, '_>,
    args: LstToSolIxArgs,
) -> ProgramResult {
    let keys: LstToSolKeys = accounts.into();
    let ix = lst_to_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn lst_to_sol_invoke(
    accounts: LstToSolAccounts<'_, '_>,
    args: LstToSolIxArgs,
) -> ProgramResult {
    lst_to_sol_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn lst_to_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LstToSolAccounts<'_, '_>,
    args: LstToSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LstToSolKeys = accounts.into();
    let ix = lst_to_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lst_to_sol_invoke_signed(
    accounts: LstToSolAccounts<'_, '_>,
    args: LstToSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lst_to_sol_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn lst_to_sol_verify_account_keys(
    accounts: LstToSolAccounts<'_, '_>,
    keys: LstToSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.state.key, &keys.state),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.pool_program.key, &keys.pool_program),
        (accounts.pool_program_data.key, &keys.pool_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const SOL_TO_LST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SolToLstAccounts<'me, 'info> {
    ///The LST mint
    pub lst_mint: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The main stake pool state account
    pub pool_state: &'me AccountInfo<'info>,
    ///The stake pool program
    pub pool_program: &'me AccountInfo<'info>,
    ///The stake pool program executable data
    pub pool_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SolToLstKeys {
    ///The LST mint
    pub lst_mint: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
    ///The main stake pool state account
    pub pool_state: Pubkey,
    ///The stake pool program
    pub pool_program: Pubkey,
    ///The stake pool program executable data
    pub pool_program_data: Pubkey,
}
impl From<SolToLstAccounts<'_, '_>> for SolToLstKeys {
    fn from(accounts: SolToLstAccounts) -> Self {
        Self {
            lst_mint: *accounts.lst_mint.key,
            state: *accounts.state.key,
            pool_state: *accounts.pool_state.key,
            pool_program: *accounts.pool_program.key,
            pool_program_data: *accounts.pool_program_data.key,
        }
    }
}
impl From<SolToLstKeys> for [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: SolToLstKeys) -> Self {
        [
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
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program_data,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]> for SolToLstKeys {
    fn from(pubkeys: [Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: pubkeys[0],
            state: pubkeys[1],
            pool_state: pubkeys[2],
            pool_program: pubkeys[3],
            pool_program_data: pubkeys[4],
        }
    }
}
impl<'info> From<SolToLstAccounts<'_, 'info>> for [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: SolToLstAccounts<'_, 'info>) -> Self {
        [
            accounts.lst_mint.clone(),
            accounts.state.clone(),
            accounts.pool_state.clone(),
            accounts.pool_program.clone(),
            accounts.pool_program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]>
    for SolToLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: &arr[0],
            state: &arr[1],
            pool_state: &arr[2],
            pool_program: &arr[3],
            pool_program_data: &arr[4],
        }
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
pub fn sol_to_lst_ix_with_program_id(
    program_id: Pubkey,
    keys: SolToLstKeys,
    args: SolToLstIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] = keys.into();
    let data: SolToLstIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sol_to_lst_ix(keys: SolToLstKeys, args: SolToLstIxArgs) -> std::io::Result<Instruction> {
    sol_to_lst_ix_with_program_id(crate::ID, keys, args)
}
pub fn sol_to_lst_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SolToLstAccounts<'_, '_>,
    args: SolToLstIxArgs,
) -> ProgramResult {
    let keys: SolToLstKeys = accounts.into();
    let ix = sol_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sol_to_lst_invoke(
    accounts: SolToLstAccounts<'_, '_>,
    args: SolToLstIxArgs,
) -> ProgramResult {
    sol_to_lst_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn sol_to_lst_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SolToLstAccounts<'_, '_>,
    args: SolToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SolToLstKeys = accounts.into();
    let ix = sol_to_lst_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sol_to_lst_invoke_signed(
    accounts: SolToLstAccounts<'_, '_>,
    args: SolToLstIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sol_to_lst_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn sol_to_lst_verify_account_keys(
    accounts: SolToLstAccounts<'_, '_>,
    keys: SolToLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.state.key, &keys.state),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.pool_program.key, &keys.pool_program),
        (accounts.pool_program_data.key, &keys.pool_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub const UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLastUpgradeSlotAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The stake pool program
    pub pool_program: &'me AccountInfo<'info>,
    ///The stake pool program executable data
    pub pool_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLastUpgradeSlotKeys {
    ///The program manager
    pub manager: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
    ///The stake pool program
    pub pool_program: Pubkey,
    ///The stake pool program executable data
    pub pool_program_data: Pubkey,
}
impl From<UpdateLastUpgradeSlotAccounts<'_, '_>> for UpdateLastUpgradeSlotKeys {
    fn from(accounts: UpdateLastUpgradeSlotAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            state: *accounts.state.key,
            pool_program: *accounts.pool_program.key,
            pool_program_data: *accounts.pool_program_data.key,
        }
    }
}
impl From<UpdateLastUpgradeSlotKeys> for [AccountMeta; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLastUpgradeSlotKeys) -> Self {
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
            AccountMeta {
                pubkey: keys.pool_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_program_data,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]> for UpdateLastUpgradeSlotKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            state: pubkeys[1],
            pool_program: pubkeys[2],
            pool_program_data: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateLastUpgradeSlotAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: UpdateLastUpgradeSlotAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.state.clone(),
            accounts.pool_program.clone(),
            accounts.pool_program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]>
    for UpdateLastUpgradeSlotAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: &arr[0],
            state: &arr[1],
            pool_program: &arr[2],
            pool_program_data: &arr[3],
        }
    }
}
pub const UPDATE_LAST_UPGRADE_SLOT_IX_DISCM: u8 = 253u8;
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLastUpgradeSlotIxData;
impl UpdateLastUpgradeSlotIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_LAST_UPGRADE_SLOT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_LAST_UPGRADE_SLOT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_LAST_UPGRADE_SLOT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_last_upgrade_slot_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLastUpgradeSlotKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateLastUpgradeSlotIxData.try_to_vec()?,
    })
}
pub fn update_last_upgrade_slot_ix(
    keys: UpdateLastUpgradeSlotKeys,
) -> std::io::Result<Instruction> {
    update_last_upgrade_slot_ix_with_program_id(crate::ID, keys)
}
pub fn update_last_upgrade_slot_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLastUpgradeSlotAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateLastUpgradeSlotKeys = accounts.into();
    let ix = update_last_upgrade_slot_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_last_upgrade_slot_invoke(
    accounts: UpdateLastUpgradeSlotAccounts<'_, '_>,
) -> ProgramResult {
    update_last_upgrade_slot_invoke_with_program_id(crate::ID, accounts)
}
pub fn update_last_upgrade_slot_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLastUpgradeSlotAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLastUpgradeSlotKeys = accounts.into();
    let ix = update_last_upgrade_slot_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_last_upgrade_slot_invoke_signed(
    accounts: UpdateLastUpgradeSlotAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_last_upgrade_slot_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn update_last_upgrade_slot_verify_account_keys(
    accounts: UpdateLastUpgradeSlotAccounts<'_, '_>,
    keys: UpdateLastUpgradeSlotKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.state.key, &keys.state),
        (accounts.pool_program.key, &keys.pool_program),
        (accounts.pool_program_data.key, &keys.pool_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_last_upgrade_slot_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLastUpgradeSlotAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_last_upgrade_slot_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLastUpgradeSlotAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_last_upgrade_slot_verify_account_privileges<'me, 'info>(
    accounts: UpdateLastUpgradeSlotAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_last_upgrade_slot_verify_writable_privileges(accounts)?;
    update_last_upgrade_slot_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_MANAGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetManagerAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///The new program manager to set to
    pub new_manager: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetManagerKeys {
    ///The program manager
    pub manager: Pubkey,
    ///The new program manager to set to
    pub new_manager: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
}
impl From<SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: SetManagerAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            new_manager: *accounts.new_manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.manager,
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
            manager: pubkeys[0],
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
            accounts.manager.clone(),
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
            manager: &arr[0],
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
        (accounts.manager.key, &keys.manager),
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
    for should_be_signer in [accounts.manager] {
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
pub const INIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitAccounts<'me, 'info> {
    ///The account paying for CalculatorState's rent
    pub payer: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///System Program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitKeys {
    ///The account paying for CalculatorState's rent
    pub payer: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
    ///System Program
    pub system_program: Pubkey,
}
impl From<InitAccounts<'_, '_>> for InitKeys {
    fn from(accounts: InitAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            state: *accounts.state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitKeys> for [AccountMeta; INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitKeys) -> Self {
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
impl From<[Pubkey; INIT_IX_ACCOUNTS_LEN]> for InitKeys {
    fn from(pubkeys: [Pubkey; INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            state: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitAccounts<'_, 'info>> for [AccountInfo<'info>; INIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitAccounts<'_, 'info>) -> Self {
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
pub const INIT_IX_DISCM: u8 = 255u8;
#[derive(Clone, Debug, PartialEq)]
pub struct InitIxData;
impl InitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INIT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_ix_with_program_id(program_id: Pubkey, keys: InitKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitIxData.try_to_vec()?,
    })
}
pub fn init_ix(keys: InitKeys) -> std::io::Result<Instruction> {
    init_ix_with_program_id(crate::ID, keys)
}
pub fn init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitKeys = accounts.into();
    let ix = init_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_invoke(accounts: InitAccounts<'_, '_>) -> ProgramResult {
    init_invoke_with_program_id(crate::ID, accounts)
}
pub fn init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitKeys = accounts.into();
    let ix = init_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_invoke_signed(accounts: InitAccounts<'_, '_>, seeds: &[&[&[u8]]]) -> ProgramResult {
    init_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn init_verify_account_keys(
    accounts: InitAccounts<'_, '_>,
    keys: InitKeys,
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
pub fn init_verify_writable_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_verify_signer_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_verify_account_privileges<'me, 'info>(
    accounts: InitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_verify_writable_privileges(accounts)?;
    init_verify_signer_privileges(accounts)?;
    Ok(())
}
