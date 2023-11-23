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
pub enum LidoCalculatorProgramIx {
    LstToSol(LstToSolIxArgs),
    SolToLst(SolToLstIxArgs),
    UpdateLastUpgradeSlot(UpdateLastUpgradeSlotIxArgs),
    SetManager(SetManagerIxArgs),
    Init(InitIxArgs),
}
impl BorshSerialize for LidoCalculatorProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::LstToSol(args) => {
                LST_TO_SOL_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SolToLst(args) => {
                SOL_TO_LST_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::UpdateLastUpgradeSlot(args) => {
                UPDATE_LAST_UPGRADE_SLOT_IX_DISCM.serialize(writer)?;
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
impl LidoCalculatorProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            LST_TO_SOL_IX_DISCM => Ok(Self::LstToSol(LstToSolIxArgs::deserialize(buf)?)),
            SOL_TO_LST_IX_DISCM => Ok(Self::SolToLst(SolToLstIxArgs::deserialize(buf)?)),
            UPDATE_LAST_UPGRADE_SLOT_IX_DISCM => Ok(Self::UpdateLastUpgradeSlot(
                UpdateLastUpgradeSlotIxArgs::deserialize(buf)?,
            )),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager(SetManagerIxArgs::deserialize(buf)?)),
            INIT_IX_DISCM => Ok(Self::Init(InitIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const LST_TO_SOL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct LstToSolAccounts<'me, 'info> {
    ///stSOL mint
    pub lst: &'me AccountInfo<'info>,
    ///The LidoCalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The lido state account
    pub lido_state: &'me AccountInfo<'info>,
    ///The lido program
    pub lido_program: &'me AccountInfo<'info>,
    ///The lido program executable data
    pub lido_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct LstToSolKeys {
    ///stSOL mint
    pub lst: Pubkey,
    ///The LidoCalculatorState PDA
    pub state: Pubkey,
    ///The lido state account
    pub lido_state: Pubkey,
    ///The lido program
    pub lido_program: Pubkey,
    ///The lido program executable data
    pub lido_program_data: Pubkey,
}
impl From<&LstToSolAccounts<'_, '_>> for LstToSolKeys {
    fn from(accounts: &LstToSolAccounts) -> Self {
        Self {
            lst: *accounts.lst.key,
            state: *accounts.state.key,
            lido_state: *accounts.lido_state.key,
            lido_program: *accounts.lido_program.key,
            lido_program_data: *accounts.lido_program_data.key,
        }
    }
}
impl From<&LstToSolKeys> for [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &LstToSolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.lido_state, false),
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new_readonly(keys.lido_program_data, false),
        ]
    }
}
impl From<[Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]> for LstToSolKeys {
    fn from(pubkeys: [Pubkey; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst: pubkeys[0],
            state: pubkeys[1],
            lido_state: pubkeys[2],
            lido_program: pubkeys[3],
            lido_program_data: pubkeys[4],
        }
    }
}
impl<'info> From<&LstToSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LstToSolAccounts<'_, 'info>) -> Self {
        [
            accounts.lst.clone(),
            accounts.state.clone(),
            accounts.lido_state.clone(),
            accounts.lido_program.clone(),
            accounts.lido_program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]>
    for LstToSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst: &arr[0],
            state: &arr[1],
            lido_state: &arr[2],
            lido_program: &arr[3],
            lido_program_data: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LstToSolIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LstToSolIxData(pub LstToSolIxArgs);
pub const LST_TO_SOL_IX_DISCM: u8 = 0u8;
impl From<LstToSolIxArgs> for LstToSolIxData {
    fn from(args: LstToSolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LstToSolIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[LST_TO_SOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl LstToSolIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != LST_TO_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LST_TO_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LstToSolIxArgs::deserialize(buf)?))
    }
}
pub fn lst_to_sol_ix<K: Into<LstToSolKeys>, A: Into<LstToSolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LstToSolKeys = accounts.into();
    let metas: [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LstToSolIxArgs = args.into();
    let data: LstToSolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn lst_to_sol_invoke<'info, A: Into<LstToSolIxArgs>>(
    accounts: &LstToSolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn lst_to_sol_invoke_signed<'info, A: Into<LstToSolIxArgs>>(
    accounts: &LstToSolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = lst_to_sol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LST_TO_SOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn lst_to_sol_verify_account_keys(
    accounts: &LstToSolAccounts<'_, '_>,
    keys: &LstToSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst.key, &keys.lst),
        (accounts.state.key, &keys.state),
        (accounts.lido_state.key, &keys.lido_state),
        (accounts.lido_program.key, &keys.lido_program),
        (accounts.lido_program_data.key, &keys.lido_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn lst_to_sol_verify_account_privileges<'me, 'info>(
    accounts: &LstToSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const SOL_TO_LST_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SolToLstAccounts<'me, 'info> {
    ///stSOL mint
    pub lst: &'me AccountInfo<'info>,
    ///The LidoCalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The lido state account
    pub lido_state: &'me AccountInfo<'info>,
    ///The lido program
    pub lido_program: &'me AccountInfo<'info>,
    ///The lido program executable data
    pub lido_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SolToLstKeys {
    ///stSOL mint
    pub lst: Pubkey,
    ///The LidoCalculatorState PDA
    pub state: Pubkey,
    ///The lido state account
    pub lido_state: Pubkey,
    ///The lido program
    pub lido_program: Pubkey,
    ///The lido program executable data
    pub lido_program_data: Pubkey,
}
impl From<&SolToLstAccounts<'_, '_>> for SolToLstKeys {
    fn from(accounts: &SolToLstAccounts) -> Self {
        Self {
            lst: *accounts.lst.key,
            state: *accounts.state.key,
            lido_state: *accounts.lido_state.key,
            lido_program: *accounts.lido_program.key,
            lido_program_data: *accounts.lido_program_data.key,
        }
    }
}
impl From<&SolToLstKeys> for [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: &SolToLstKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.lido_state, false),
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new_readonly(keys.lido_program_data, false),
        ]
    }
}
impl From<[Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]> for SolToLstKeys {
    fn from(pubkeys: [Pubkey; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst: pubkeys[0],
            state: pubkeys[1],
            lido_state: pubkeys[2],
            lido_program: pubkeys[3],
            lido_program_data: pubkeys[4],
        }
    }
}
impl<'info> From<&SolToLstAccounts<'_, 'info>>
    for [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SolToLstAccounts<'_, 'info>) -> Self {
        [
            accounts.lst.clone(),
            accounts.state.clone(),
            accounts.lido_state.clone(),
            accounts.lido_program.clone(),
            accounts.lido_program_data.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]>
    for SolToLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst: &arr[0],
            state: &arr[1],
            lido_state: &arr[2],
            lido_program: &arr[3],
            lido_program_data: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SolToLstIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SolToLstIxData(pub SolToLstIxArgs);
pub const SOL_TO_LST_IX_DISCM: u8 = 1u8;
impl From<SolToLstIxArgs> for SolToLstIxData {
    fn from(args: SolToLstIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SolToLstIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SOL_TO_LST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SolToLstIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SOL_TO_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SOL_TO_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SolToLstIxArgs::deserialize(buf)?))
    }
}
pub fn sol_to_lst_ix<K: Into<SolToLstKeys>, A: Into<SolToLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SolToLstKeys = accounts.into();
    let metas: [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SolToLstIxArgs = args.into();
    let data: SolToLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sol_to_lst_invoke<'info, A: Into<SolToLstIxArgs>>(
    accounts: &SolToLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sol_to_lst_invoke_signed<'info, A: Into<SolToLstIxArgs>>(
    accounts: &SolToLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sol_to_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SOL_TO_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn sol_to_lst_verify_account_keys(
    accounts: &SolToLstAccounts<'_, '_>,
    keys: &SolToLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst.key, &keys.lst),
        (accounts.state.key, &keys.state),
        (accounts.lido_state.key, &keys.lido_state),
        (accounts.lido_program.key, &keys.lido_program),
        (accounts.lido_program_data.key, &keys.lido_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
#[allow(unused)]
pub fn sol_to_lst_verify_account_privileges<'me, 'info>(
    accounts: &SolToLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    Ok(())
}
pub const UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLastUpgradeSlotAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///The LidoCalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The lido program
    pub lido_program: &'me AccountInfo<'info>,
    ///The lido program executable data
    pub lido_program_data: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLastUpgradeSlotKeys {
    ///The program manager
    pub manager: Pubkey,
    ///The LidoCalculatorState PDA
    pub state: Pubkey,
    ///The lido program
    pub lido_program: Pubkey,
    ///The lido program executable data
    pub lido_program_data: Pubkey,
}
impl From<&UpdateLastUpgradeSlotAccounts<'_, '_>> for UpdateLastUpgradeSlotKeys {
    fn from(accounts: &UpdateLastUpgradeSlotAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            state: *accounts.state.key,
            lido_program: *accounts.lido_program.key,
            lido_program_data: *accounts.lido_program_data.key,
        }
    }
}
impl From<&UpdateLastUpgradeSlotKeys> for [AccountMeta; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateLastUpgradeSlotKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.lido_program, false),
            AccountMeta::new_readonly(keys.lido_program_data, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]> for UpdateLastUpgradeSlotKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            manager: pubkeys[0],
            state: pubkeys[1],
            lido_program: pubkeys[2],
            lido_program_data: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdateLastUpgradeSlotAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateLastUpgradeSlotAccounts<'_, 'info>) -> Self {
        [
            accounts.manager.clone(),
            accounts.state.clone(),
            accounts.lido_program.clone(),
            accounts.lido_program_data.clone(),
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
            lido_program: &arr[2],
            lido_program_data: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateLastUpgradeSlotIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLastUpgradeSlotIxData(pub UpdateLastUpgradeSlotIxArgs);
pub const UPDATE_LAST_UPGRADE_SLOT_IX_DISCM: u8 = 253u8;
impl From<UpdateLastUpgradeSlotIxArgs> for UpdateLastUpgradeSlotIxData {
    fn from(args: UpdateLastUpgradeSlotIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateLastUpgradeSlotIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_LAST_UPGRADE_SLOT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl UpdateLastUpgradeSlotIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UPDATE_LAST_UPGRADE_SLOT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_LAST_UPGRADE_SLOT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateLastUpgradeSlotIxArgs::deserialize(buf)?))
    }
}
pub fn update_last_upgrade_slot_ix<
    K: Into<UpdateLastUpgradeSlotKeys>,
    A: Into<UpdateLastUpgradeSlotIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateLastUpgradeSlotKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateLastUpgradeSlotIxArgs = args.into();
    let data: UpdateLastUpgradeSlotIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_last_upgrade_slot_invoke<'info, A: Into<UpdateLastUpgradeSlotIxArgs>>(
    accounts: &UpdateLastUpgradeSlotAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_last_upgrade_slot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_last_upgrade_slot_invoke_signed<'info, A: Into<UpdateLastUpgradeSlotIxArgs>>(
    accounts: &UpdateLastUpgradeSlotAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_last_upgrade_slot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LAST_UPGRADE_SLOT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_last_upgrade_slot_verify_account_keys(
    accounts: &UpdateLastUpgradeSlotAccounts<'_, '_>,
    keys: &UpdateLastUpgradeSlotKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.manager.key, &keys.manager),
        (accounts.state.key, &keys.state),
        (accounts.lido_program.key, &keys.lido_program),
        (accounts.lido_program_data.key, &keys.lido_program_data),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_last_upgrade_slot_verify_account_privileges<'me, 'info>(
    accounts: &UpdateLastUpgradeSlotAccounts<'me, 'info>,
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
pub const SET_MANAGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetManagerAccounts<'me, 'info> {
    ///The program manager
    pub manager: &'me AccountInfo<'info>,
    ///The new program manager to set to
    pub new_manager: &'me AccountInfo<'info>,
    ///The LidoCalculatorState PDA
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetManagerKeys {
    ///The program manager
    pub manager: Pubkey,
    ///The new program manager to set to
    pub new_manager: Pubkey,
    ///The LidoCalculatorState PDA
    pub state: Pubkey,
}
impl From<&SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: &SetManagerAccounts) -> Self {
        Self {
            manager: *accounts.manager.key,
            new_manager: *accounts.new_manager.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetManagerKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.manager, true),
            AccountMeta::new_readonly(keys.new_manager, false),
            AccountMeta::new(keys.state, false),
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
impl<'info> From<&SetManagerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetManagerAccounts<'_, 'info>) -> Self {
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
pub fn set_manager_verify_account_privileges<'me, 'info>(
    accounts: &SetManagerAccounts<'me, 'info>,
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
pub const INIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitAccounts<'me, 'info> {
    ///The account paying for LidoCalculatorState's rent
    pub payer: &'me AccountInfo<'info>,
    ///The LidoCalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///System Program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitKeys {
    ///The account paying for LidoCalculatorState's rent
    pub payer: Pubkey,
    ///The LidoCalculatorState PDA
    pub state: Pubkey,
    ///System Program
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
