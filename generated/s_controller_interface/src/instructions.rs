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
pub enum SControllerProgramIx {
    SyncSolValue(SyncSolValueIxArgs),
    SwapExactIn(SwapExactInIxArgs),
    SwapExactOut(SwapExactOutIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    DisableLstInput(DisableLstInputIxArgs),
    EnableLstInput(EnableLstInputIxArgs),
    AddLst(AddLstIxArgs),
    RemoveLst(RemoveLstIxArgs),
    SetSolValueCalculator(SetSolValueCalculatorIxArgs),
    SetAdmin(SetAdminIxArgs),
    SetProtocolFee(SetProtocolFeeIxArgs),
    SetProtocolFeeBeneficiary(SetProtocolFeeBeneficiaryIxArgs),
    SetPricingProgram(SetPricingProgramIxArgs),
    WithdrawProtocolFees(WithdrawProtocolFeesIxArgs),
    AddDisablePoolAuthority(AddDisablePoolAuthorityIxArgs),
    RemoveDisablePoolAuthority(RemoveDisablePoolAuthorityIxArgs),
    DisablePool(DisablePoolIxArgs),
    EnablePool(EnablePoolIxArgs),
    StartRebalance(StartRebalanceIxArgs),
    EndRebalance(EndRebalanceIxArgs),
    SetRebalanceAuthority(SetRebalanceAuthorityIxArgs),
    Initialize(InitializeIxArgs),
}
impl BorshSerialize for SControllerProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::SyncSolValue(args) => {
                SYNC_SOL_VALUE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SwapExactIn(args) => {
                SWAP_EXACT_IN_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SwapExactOut(args) => {
                SWAP_EXACT_OUT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddLiquidity(args) => {
                ADD_LIQUIDITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveLiquidity(args) => {
                REMOVE_LIQUIDITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DisableLstInput(args) => {
                DISABLE_LST_INPUT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::EnableLstInput(args) => {
                ENABLE_LST_INPUT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddLst(args) => {
                ADD_LST_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveLst(args) => {
                REMOVE_LST_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetSolValueCalculator(args) => {
                SET_SOL_VALUE_CALCULATOR_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetAdmin(args) => {
                SET_ADMIN_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetProtocolFee(args) => {
                SET_PROTOCOL_FEE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetProtocolFeeBeneficiary(args) => {
                SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetPricingProgram(args) => {
                SET_PRICING_PROGRAM_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::WithdrawProtocolFees(args) => {
                WITHDRAW_PROTOCOL_FEES_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddDisablePoolAuthority(args) => {
                ADD_DISABLE_POOL_AUTHORITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveDisablePoolAuthority(args) => {
                REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DisablePool(args) => {
                DISABLE_POOL_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::EnablePool(args) => {
                ENABLE_POOL_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::StartRebalance(args) => {
                START_REBALANCE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::EndRebalance(args) => {
                END_REBALANCE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetRebalanceAuthority(args) => {
                SET_REBALANCE_AUTHORITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Initialize(args) => {
                INITIALIZE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl SControllerProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            SYNC_SOL_VALUE_IX_DISCM => {
                Ok(Self::SyncSolValue(SyncSolValueIxArgs::deserialize(buf)?))
            }
            SWAP_EXACT_IN_IX_DISCM => Ok(Self::SwapExactIn(SwapExactInIxArgs::deserialize(buf)?)),
            SWAP_EXACT_OUT_IX_DISCM => {
                Ok(Self::SwapExactOut(SwapExactOutIxArgs::deserialize(buf)?))
            }
            ADD_LIQUIDITY_IX_DISCM => Ok(Self::AddLiquidity(AddLiquidityIxArgs::deserialize(buf)?)),
            REMOVE_LIQUIDITY_IX_DISCM => Ok(Self::RemoveLiquidity(
                RemoveLiquidityIxArgs::deserialize(buf)?,
            )),
            DISABLE_LST_INPUT_IX_DISCM => Ok(Self::DisableLstInput(
                DisableLstInputIxArgs::deserialize(buf)?,
            )),
            ENABLE_LST_INPUT_IX_DISCM => Ok(Self::EnableLstInput(
                EnableLstInputIxArgs::deserialize(buf)?,
            )),
            ADD_LST_IX_DISCM => Ok(Self::AddLst(AddLstIxArgs::deserialize(buf)?)),
            REMOVE_LST_IX_DISCM => Ok(Self::RemoveLst(RemoveLstIxArgs::deserialize(buf)?)),
            SET_SOL_VALUE_CALCULATOR_IX_DISCM => Ok(Self::SetSolValueCalculator(
                SetSolValueCalculatorIxArgs::deserialize(buf)?,
            )),
            SET_ADMIN_IX_DISCM => Ok(Self::SetAdmin(SetAdminIxArgs::deserialize(buf)?)),
            SET_PROTOCOL_FEE_IX_DISCM => Ok(Self::SetProtocolFee(
                SetProtocolFeeIxArgs::deserialize(buf)?,
            )),
            SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM => Ok(Self::SetProtocolFeeBeneficiary(
                SetProtocolFeeBeneficiaryIxArgs::deserialize(buf)?,
            )),
            SET_PRICING_PROGRAM_IX_DISCM => Ok(Self::SetPricingProgram(
                SetPricingProgramIxArgs::deserialize(buf)?,
            )),
            WITHDRAW_PROTOCOL_FEES_IX_DISCM => Ok(Self::WithdrawProtocolFees(
                WithdrawProtocolFeesIxArgs::deserialize(buf)?,
            )),
            ADD_DISABLE_POOL_AUTHORITY_IX_DISCM => Ok(Self::AddDisablePoolAuthority(
                AddDisablePoolAuthorityIxArgs::deserialize(buf)?,
            )),
            REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM => Ok(Self::RemoveDisablePoolAuthority(
                RemoveDisablePoolAuthorityIxArgs::deserialize(buf)?,
            )),
            DISABLE_POOL_IX_DISCM => Ok(Self::DisablePool(DisablePoolIxArgs::deserialize(buf)?)),
            ENABLE_POOL_IX_DISCM => Ok(Self::EnablePool(EnablePoolIxArgs::deserialize(buf)?)),
            START_REBALANCE_IX_DISCM => Ok(Self::StartRebalance(
                StartRebalanceIxArgs::deserialize(buf)?,
            )),
            END_REBALANCE_IX_DISCM => Ok(Self::EndRebalance(EndRebalanceIxArgs::deserialize(buf)?)),
            SET_REBALANCE_AUTHORITY_IX_DISCM => Ok(Self::SetRebalanceAuthority(
                SetRebalanceAuthorityIxArgs::deserialize(buf)?,
            )),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const SYNC_SOL_VALUE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SyncSolValueAccounts<'me, 'info> {
    ///Mint of the LST to sync SOL value for
    pub lst_mint: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///LST reserves token account of the pool
    pub pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SyncSolValueKeys {
    ///Mint of the LST to sync SOL value for
    pub lst_mint: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///LST reserves token account of the pool
    pub pool_reserves: Pubkey,
}
impl From<&SyncSolValueAccounts<'_, '_>> for SyncSolValueKeys {
    fn from(accounts: &SyncSolValueAccounts) -> Self {
        Self {
            lst_mint: *accounts.lst_mint.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            pool_reserves: *accounts.pool_reserves.key,
        }
    }
}
impl From<&SyncSolValueKeys> for [AccountMeta; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SyncSolValueKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new_readonly(keys.pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN]> for SyncSolValueKeys {
    fn from(pubkeys: [Pubkey; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: pubkeys[0],
            pool_state: pubkeys[1],
            lst_state_list: pubkeys[2],
            pool_reserves: pubkeys[3],
        }
    }
}
impl<'info> From<&SyncSolValueAccounts<'_, 'info>>
    for [AccountInfo<'info>; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SyncSolValueAccounts<'_, 'info>) -> Self {
        [
            accounts.lst_mint.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN]>
    for SyncSolValueAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lst_mint: &arr[0],
            pool_state: &arr[1],
            lst_state_list: &arr[2],
            pool_reserves: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SyncSolValueIxArgs {
    pub lst_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SyncSolValueIxData(pub SyncSolValueIxArgs);
pub const SYNC_SOL_VALUE_IX_DISCM: u8 = 0u8;
impl From<SyncSolValueIxArgs> for SyncSolValueIxData {
    fn from(args: SyncSolValueIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SyncSolValueIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SYNC_SOL_VALUE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SyncSolValueIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SYNC_SOL_VALUE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SYNC_SOL_VALUE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SyncSolValueIxArgs::deserialize(buf)?))
    }
}
pub fn sync_sol_value_ix<K: Into<SyncSolValueKeys>, A: Into<SyncSolValueIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SyncSolValueKeys = accounts.into();
    let metas: [AccountMeta; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SyncSolValueIxArgs = args.into();
    let data: SyncSolValueIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sync_sol_value_invoke<'info, A: Into<SyncSolValueIxArgs>>(
    accounts: &SyncSolValueAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = sync_sol_value_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sync_sol_value_invoke_signed<'info, A: Into<SyncSolValueIxArgs>>(
    accounts: &SyncSolValueAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sync_sol_value_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SYNC_SOL_VALUE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn sync_sol_value_verify_account_keys(
    accounts: &SyncSolValueAccounts<'_, '_>,
    keys: &SyncSolValueKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.pool_reserves.key, &keys.pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn sync_sol_value_verify_account_privileges<'me, 'info>(
    accounts: &SyncSolValueAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.lst_state_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const SWAP_EXACT_IN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactInAccounts<'me, 'info> {
    ///Authority of src_lst_acc. User making the swap.
    pub signer: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped from
    pub src_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: &'me AccountInfo<'info>,
    ///LST token account being swapped from
    pub src_lst_acc: &'me AccountInfo<'info>,
    ///LST token account to swapped to
    pub dst_lst_acc: &'me AccountInfo<'info>,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///Source LST token program
    pub src_lst_token_program: &'me AccountInfo<'info>,
    ///Destination LST token program
    pub dst_lst_token_program: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: &'me AccountInfo<'info>,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapExactInKeys {
    ///Authority of src_lst_acc. User making the swap.
    pub signer: Pubkey,
    ///Mint of the LST being swapped from
    pub src_lst_mint: Pubkey,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: Pubkey,
    ///LST token account being swapped from
    pub src_lst_acc: Pubkey,
    ///LST token account to swapped to
    pub dst_lst_acc: Pubkey,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: Pubkey,
    ///Source LST token program
    pub src_lst_token_program: Pubkey,
    ///Destination LST token program
    pub dst_lst_token_program: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: Pubkey,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: Pubkey,
}
impl From<&SwapExactInAccounts<'_, '_>> for SwapExactInKeys {
    fn from(accounts: &SwapExactInAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            src_lst_mint: *accounts.src_lst_mint.key,
            dst_lst_mint: *accounts.dst_lst_mint.key,
            src_lst_acc: *accounts.src_lst_acc.key,
            dst_lst_acc: *accounts.dst_lst_acc.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            src_lst_token_program: *accounts.src_lst_token_program.key,
            dst_lst_token_program: *accounts.dst_lst_token_program.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            src_pool_reserves: *accounts.src_pool_reserves.key,
            dst_pool_reserves: *accounts.dst_pool_reserves.key,
        }
    }
}
impl From<&SwapExactInKeys> for [AccountMeta; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] {
    fn from(keys: &SwapExactInKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.signer, true),
            AccountMeta::new_readonly(keys.src_lst_mint, false),
            AccountMeta::new_readonly(keys.dst_lst_mint, false),
            AccountMeta::new(keys.src_lst_acc, false),
            AccountMeta::new(keys.dst_lst_acc, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new_readonly(keys.src_lst_token_program, false),
            AccountMeta::new_readonly(keys.dst_lst_token_program, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.src_pool_reserves, false),
            AccountMeta::new(keys.dst_pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]> for SwapExactInKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            src_lst_mint: pubkeys[1],
            dst_lst_mint: pubkeys[2],
            src_lst_acc: pubkeys[3],
            dst_lst_acc: pubkeys[4],
            protocol_fee_accumulator: pubkeys[5],
            src_lst_token_program: pubkeys[6],
            dst_lst_token_program: pubkeys[7],
            pool_state: pubkeys[8],
            lst_state_list: pubkeys[9],
            src_pool_reserves: pubkeys[10],
            dst_pool_reserves: pubkeys[11],
        }
    }
}
impl<'info> From<&SwapExactInAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SwapExactInAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.src_lst_mint.clone(),
            accounts.dst_lst_mint.clone(),
            accounts.src_lst_acc.clone(),
            accounts.dst_lst_acc.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.src_lst_token_program.clone(),
            accounts.dst_lst_token_program.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.src_pool_reserves.clone(),
            accounts.dst_pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]>
    for SwapExactInAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            src_lst_mint: &arr[1],
            dst_lst_mint: &arr[2],
            src_lst_acc: &arr[3],
            dst_lst_acc: &arr[4],
            protocol_fee_accumulator: &arr[5],
            src_lst_token_program: &arr[6],
            dst_lst_token_program: &arr[7],
            pool_state: &arr[8],
            lst_state_list: &arr[9],
            src_pool_reserves: &arr[10],
            dst_pool_reserves: &arr[11],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapExactInIxArgs {
    pub src_lst_value_calc_accs: u8,
    pub dst_lst_value_calc_accs: u8,
    pub src_lst_index: u32,
    pub dst_lst_index: u32,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactInIxData(pub SwapExactInIxArgs);
pub const SWAP_EXACT_IN_IX_DISCM: u8 = 1u8;
impl From<SwapExactInIxArgs> for SwapExactInIxData {
    fn from(args: SwapExactInIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapExactInIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_EXACT_IN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SwapExactInIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SWAP_EXACT_IN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_EXACT_IN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapExactInIxArgs::deserialize(buf)?))
    }
}
pub fn swap_exact_in_ix<K: Into<SwapExactInKeys>, A: Into<SwapExactInIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapExactInKeys = accounts.into();
    let metas: [AccountMeta; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SwapExactInIxArgs = args.into();
    let data: SwapExactInIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_in_invoke<'info, A: Into<SwapExactInIxArgs>>(
    accounts: &SwapExactInAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = swap_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_exact_in_invoke_signed<'info, A: Into<SwapExactInIxArgs>>(
    accounts: &SwapExactInAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_exact_in_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_EXACT_IN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn swap_exact_in_verify_account_keys(
    accounts: &SwapExactInAccounts<'_, '_>,
    keys: &SwapExactInKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.signer.key, &keys.signer),
        (accounts.src_lst_mint.key, &keys.src_lst_mint),
        (accounts.dst_lst_mint.key, &keys.dst_lst_mint),
        (accounts.src_lst_acc.key, &keys.src_lst_acc),
        (accounts.dst_lst_acc.key, &keys.dst_lst_acc),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (
            accounts.src_lst_token_program.key,
            &keys.src_lst_token_program,
        ),
        (
            accounts.dst_lst_token_program.key,
            &keys.dst_lst_token_program,
        ),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.src_pool_reserves.key, &keys.src_pool_reserves),
        (accounts.dst_pool_reserves.key, &keys.dst_pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_exact_in_verify_account_privileges<'me, 'info>(
    accounts: &SwapExactInAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.src_lst_acc,
        accounts.dst_lst_acc,
        accounts.protocol_fee_accumulator,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.src_pool_reserves,
        accounts.dst_pool_reserves,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SWAP_EXACT_OUT_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutAccounts<'me, 'info> {
    ///Authority of src_lst_acc. User making the swap.
    pub signer: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped from
    pub src_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: &'me AccountInfo<'info>,
    ///LST token account being swapped from
    pub src_lst_acc: &'me AccountInfo<'info>,
    ///LST token account to swapped to
    pub dst_lst_acc: &'me AccountInfo<'info>,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///
    pub src_lst_token_program: &'me AccountInfo<'info>,
    ///
    pub dst_lst_token_program: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: &'me AccountInfo<'info>,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapExactOutKeys {
    ///Authority of src_lst_acc. User making the swap.
    pub signer: Pubkey,
    ///Mint of the LST being swapped from
    pub src_lst_mint: Pubkey,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: Pubkey,
    ///LST token account being swapped from
    pub src_lst_acc: Pubkey,
    ///LST token account to swapped to
    pub dst_lst_acc: Pubkey,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: Pubkey,
    ///
    pub src_lst_token_program: Pubkey,
    ///
    pub dst_lst_token_program: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: Pubkey,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: Pubkey,
}
impl From<&SwapExactOutAccounts<'_, '_>> for SwapExactOutKeys {
    fn from(accounts: &SwapExactOutAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            src_lst_mint: *accounts.src_lst_mint.key,
            dst_lst_mint: *accounts.dst_lst_mint.key,
            src_lst_acc: *accounts.src_lst_acc.key,
            dst_lst_acc: *accounts.dst_lst_acc.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            src_lst_token_program: *accounts.src_lst_token_program.key,
            dst_lst_token_program: *accounts.dst_lst_token_program.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            src_pool_reserves: *accounts.src_pool_reserves.key,
            dst_pool_reserves: *accounts.dst_pool_reserves.key,
        }
    }
}
impl From<&SwapExactOutKeys> for [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &SwapExactOutKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.signer, true),
            AccountMeta::new_readonly(keys.src_lst_mint, false),
            AccountMeta::new_readonly(keys.dst_lst_mint, false),
            AccountMeta::new(keys.src_lst_acc, false),
            AccountMeta::new(keys.dst_lst_acc, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new_readonly(keys.src_lst_token_program, false),
            AccountMeta::new_readonly(keys.dst_lst_token_program, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.src_pool_reserves, false),
            AccountMeta::new(keys.dst_pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]> for SwapExactOutKeys {
    fn from(pubkeys: [Pubkey; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            src_lst_mint: pubkeys[1],
            dst_lst_mint: pubkeys[2],
            src_lst_acc: pubkeys[3],
            dst_lst_acc: pubkeys[4],
            protocol_fee_accumulator: pubkeys[5],
            src_lst_token_program: pubkeys[6],
            dst_lst_token_program: pubkeys[7],
            pool_state: pubkeys[8],
            lst_state_list: pubkeys[9],
            src_pool_reserves: pubkeys[10],
            dst_pool_reserves: pubkeys[11],
        }
    }
}
impl<'info> From<&SwapExactOutAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SwapExactOutAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.src_lst_mint.clone(),
            accounts.dst_lst_mint.clone(),
            accounts.src_lst_acc.clone(),
            accounts.dst_lst_acc.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.src_lst_token_program.clone(),
            accounts.dst_lst_token_program.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.src_pool_reserves.clone(),
            accounts.dst_pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]>
    for SwapExactOutAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            src_lst_mint: &arr[1],
            dst_lst_mint: &arr[2],
            src_lst_acc: &arr[3],
            dst_lst_acc: &arr[4],
            protocol_fee_accumulator: &arr[5],
            src_lst_token_program: &arr[6],
            dst_lst_token_program: &arr[7],
            pool_state: &arr[8],
            lst_state_list: &arr[9],
            src_pool_reserves: &arr[10],
            dst_pool_reserves: &arr[11],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapExactOutIxArgs {
    pub src_lst_value_calc_accs: u8,
    pub dst_lst_value_calc_accs: u8,
    pub src_lst_index: u32,
    pub dst_lst_index: u32,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapExactOutIxData(pub SwapExactOutIxArgs);
pub const SWAP_EXACT_OUT_IX_DISCM: u8 = 2u8;
impl From<SwapExactOutIxArgs> for SwapExactOutIxData {
    fn from(args: SwapExactOutIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapExactOutIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_EXACT_OUT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SwapExactOutIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SWAP_EXACT_OUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_EXACT_OUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapExactOutIxArgs::deserialize(buf)?))
    }
}
pub fn swap_exact_out_ix<K: Into<SwapExactOutKeys>, A: Into<SwapExactOutIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapExactOutKeys = accounts.into();
    let metas: [AccountMeta; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SwapExactOutIxArgs = args.into();
    let data: SwapExactOutIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_exact_out_invoke<'info, A: Into<SwapExactOutIxArgs>>(
    accounts: &SwapExactOutAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = swap_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_exact_out_invoke_signed<'info, A: Into<SwapExactOutIxArgs>>(
    accounts: &SwapExactOutAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_exact_out_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_EXACT_OUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn swap_exact_out_verify_account_keys(
    accounts: &SwapExactOutAccounts<'_, '_>,
    keys: &SwapExactOutKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.signer.key, &keys.signer),
        (accounts.src_lst_mint.key, &keys.src_lst_mint),
        (accounts.dst_lst_mint.key, &keys.dst_lst_mint),
        (accounts.src_lst_acc.key, &keys.src_lst_acc),
        (accounts.dst_lst_acc.key, &keys.dst_lst_acc),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (
            accounts.src_lst_token_program.key,
            &keys.src_lst_token_program,
        ),
        (
            accounts.dst_lst_token_program.key,
            &keys.dst_lst_token_program,
        ),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.src_pool_reserves.key, &keys.src_pool_reserves),
        (accounts.dst_pool_reserves.key, &keys.dst_pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_exact_out_verify_account_privileges<'me, 'info>(
    accounts: &SwapExactOutAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.src_lst_acc,
        accounts.dst_lst_acc,
        accounts.protocol_fee_accumulator,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.src_pool_reserves,
        accounts.dst_pool_reserves,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    ///Authority of src_lst_acc. User who's adding liquidity.
    pub signer: &'me AccountInfo<'info>,
    ///Mint of the LST
    pub lst_mint: &'me AccountInfo<'info>,
    ///LST token account to add liquidity from
    pub src_lst_acc: &'me AccountInfo<'info>,
    ///LP token account to mint new LP tokens to
    pub dst_lp_acc: &'me AccountInfo<'info>,
    ///LP token mint
    pub lp_token_mint: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Token 2022 program for use with LP token mint
    pub token_2022: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///LST reserves token account of the pool
    pub pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityKeys {
    ///Authority of src_lst_acc. User who's adding liquidity.
    pub signer: Pubkey,
    ///Mint of the LST
    pub lst_mint: Pubkey,
    ///LST token account to add liquidity from
    pub src_lst_acc: Pubkey,
    ///LP token account to mint new LP tokens to
    pub dst_lp_acc: Pubkey,
    ///LP token mint
    pub lp_token_mint: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Token 2022 program for use with LP token mint
    pub token_2022: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///LST reserves token account of the pool
    pub pool_reserves: Pubkey,
}
impl From<&AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: &AddLiquidityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lst_mint: *accounts.lst_mint.key,
            src_lst_acc: *accounts.src_lst_acc.key,
            dst_lp_acc: *accounts.dst_lp_acc.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            token_2022: *accounts.token_2022.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            pool_reserves: *accounts.pool_reserves.key,
        }
    }
}
impl From<&AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddLiquidityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.signer, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.src_lst_acc, false),
            AccountMeta::new(keys.dst_lp_acc, false),
            AccountMeta::new(keys.lp_token_mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.token_2022, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lst_mint: pubkeys[1],
            src_lst_acc: pubkeys[2],
            dst_lp_acc: pubkeys[3],
            lp_token_mint: pubkeys[4],
            token_program: pubkeys[5],
            token_2022: pubkeys[6],
            pool_state: pubkeys[7],
            lst_state_list: pubkeys[8],
            pool_reserves: pubkeys[9],
        }
    }
}
impl<'info> From<&AddLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.lst_mint.clone(),
            accounts.src_lst_acc.clone(),
            accounts.dst_lp_acc.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.token_2022.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for AddLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lst_mint: &arr[1],
            src_lst_acc: &arr[2],
            dst_lp_acc: &arr[3],
            lp_token_mint: &arr[4],
            token_program: &arr[5],
            token_2022: &arr[6],
            pool_state: &arr[7],
            lst_state_list: &arr[8],
            pool_reserves: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityIxArgs {
    pub lst_value_calc_accs: u8,
    pub lst_index: u32,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
pub const ADD_LIQUIDITY_IX_DISCM: u8 = 3u8;
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddLiquidityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ADD_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLiquidityIxArgs::deserialize(buf)?))
    }
}
pub fn add_liquidity_ix<K: Into<AddLiquidityKeys>, A: Into<AddLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddLiquidityKeys = accounts.into();
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddLiquidityIxArgs = args.into();
    let data: AddLiquidityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_invoke<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_liquidity_invoke_signed<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_liquidity_verify_account_keys(
    accounts: &AddLiquidityAccounts<'_, '_>,
    keys: &AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.signer.key, &keys.signer),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.src_lst_acc.key, &keys.src_lst_acc),
        (accounts.dst_lp_acc.key, &keys.dst_lp_acc),
        (accounts.lp_token_mint.key, &keys.lp_token_mint),
        (accounts.token_program.key, &keys.token_program),
        (accounts.token_2022.key, &keys.token_2022),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.pool_reserves.key, &keys.pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: &AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.src_lst_acc,
        accounts.dst_lp_acc,
        accounts.lp_token_mint,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.pool_reserves,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    ///Authority of lp_acc. User who's removing liquidity.
    pub signer: &'me AccountInfo<'info>,
    ///Mint of the LST
    pub lst_mint: &'me AccountInfo<'info>,
    ///LST token account to redeem to
    pub dst_lst_acc: &'me AccountInfo<'info>,
    ///LP token account to burn LP tokens from
    pub src_lp_acc: &'me AccountInfo<'info>,
    ///LP token mint
    pub lp_token_mint: &'me AccountInfo<'info>,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Token 2022 program for use with LP token mint
    pub token_2022: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///LST reserves token account of the pool
    pub pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityKeys {
    ///Authority of lp_acc. User who's removing liquidity.
    pub signer: Pubkey,
    ///Mint of the LST
    pub lst_mint: Pubkey,
    ///LST token account to redeem to
    pub dst_lst_acc: Pubkey,
    ///LP token account to burn LP tokens from
    pub src_lp_acc: Pubkey,
    ///LP token mint
    pub lp_token_mint: Pubkey,
    ///Protocol fee accumulator token account
    pub protocol_fee_accumulator: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Token 2022 program for use with LP token mint
    pub token_2022: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///LST reserves token account of the pool
    pub pool_reserves: Pubkey,
}
impl From<&RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: &RemoveLiquidityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            lst_mint: *accounts.lst_mint.key,
            dst_lst_acc: *accounts.dst_lst_acc.key,
            src_lp_acc: *accounts.src_lp_acc.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            token_program: *accounts.token_program.key,
            token_2022: *accounts.token_2022.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            pool_reserves: *accounts.pool_reserves.key,
        }
    }
}
impl From<&RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveLiquidityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.signer, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.dst_lst_acc, false),
            AccountMeta::new(keys.src_lp_acc, false),
            AccountMeta::new(keys.lp_token_mint, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.token_2022, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            lst_mint: pubkeys[1],
            dst_lst_acc: pubkeys[2],
            src_lp_acc: pubkeys[3],
            lp_token_mint: pubkeys[4],
            protocol_fee_accumulator: pubkeys[5],
            token_program: pubkeys[6],
            token_2022: pubkeys[7],
            pool_state: pubkeys[8],
            lst_state_list: pubkeys[9],
            pool_reserves: pubkeys[10],
        }
    }
}
impl<'info> From<&RemoveLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.lst_mint.clone(),
            accounts.dst_lst_acc.clone(),
            accounts.src_lp_acc.clone(),
            accounts.lp_token_mint.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.token_program.clone(),
            accounts.token_2022.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for RemoveLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            lst_mint: &arr[1],
            dst_lst_acc: &arr[2],
            src_lp_acc: &arr[3],
            lp_token_mint: &arr[4],
            protocol_fee_accumulator: &arr[5],
            token_program: &arr[6],
            token_2022: &arr[7],
            pool_state: &arr[8],
            lst_state_list: &arr[9],
            pool_reserves: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityIxArgs {
    pub lst_value_calc_accs: u8,
    pub lst_index: u32,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
pub const REMOVE_LIQUIDITY_IX_DISCM: u8 = 4u8;
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveLiquidityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_LIQUIDITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveLiquidityIxArgs::deserialize(buf)?))
    }
}
pub fn remove_liquidity_ix<K: Into<RemoveLiquidityKeys>, A: Into<RemoveLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveLiquidityKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveLiquidityIxArgs = args.into();
    let data: RemoveLiquidityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_invoke<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_liquidity_invoke_signed<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_liquidity_verify_account_keys(
    accounts: &RemoveLiquidityAccounts<'_, '_>,
    keys: &RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.signer.key, &keys.signer),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.dst_lst_acc.key, &keys.dst_lst_acc),
        (accounts.src_lp_acc.key, &keys.src_lp_acc),
        (accounts.lp_token_mint.key, &keys.lp_token_mint),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (accounts.token_program.key, &keys.token_program),
        (accounts.token_2022.key, &keys.token_2022),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.pool_reserves.key, &keys.pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: &RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.dst_lst_acc,
        accounts.src_lp_acc,
        accounts.lp_token_mint,
        accounts.protocol_fee_accumulator,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.pool_reserves,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DISABLE_LST_INPUT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct DisableLstInputAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///Mint of the LST to disable input for
    pub lst_mint: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DisableLstInputKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///Mint of the LST to disable input for
    pub lst_mint: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
}
impl From<&DisableLstInputAccounts<'_, '_>> for DisableLstInputKeys {
    fn from(accounts: &DisableLstInputAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            lst_mint: *accounts.lst_mint.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
        }
    }
}
impl From<&DisableLstInputKeys> for [AccountMeta; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DisableLstInputKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
        ]
    }
}
impl From<[Pubkey; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN]> for DisableLstInputKeys {
    fn from(pubkeys: [Pubkey; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            lst_mint: pubkeys[1],
            pool_state: pubkeys[2],
            lst_state_list: pubkeys[3],
        }
    }
}
impl<'info> From<&DisableLstInputAccounts<'_, 'info>>
    for [AccountInfo<'info>; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DisableLstInputAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.lst_mint.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN]>
    for DisableLstInputAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            lst_mint: &arr[1],
            pool_state: &arr[2],
            lst_state_list: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisableLstInputIxArgs {
    pub index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DisableLstInputIxData(pub DisableLstInputIxArgs);
pub const DISABLE_LST_INPUT_IX_DISCM: u8 = 5u8;
impl From<DisableLstInputIxArgs> for DisableLstInputIxData {
    fn from(args: DisableLstInputIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DisableLstInputIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DISABLE_LST_INPUT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl DisableLstInputIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DISABLE_LST_INPUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DISABLE_LST_INPUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DisableLstInputIxArgs::deserialize(buf)?))
    }
}
pub fn disable_lst_input_ix<K: Into<DisableLstInputKeys>, A: Into<DisableLstInputIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DisableLstInputKeys = accounts.into();
    let metas: [AccountMeta; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DisableLstInputIxArgs = args.into();
    let data: DisableLstInputIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn disable_lst_input_invoke<'info, A: Into<DisableLstInputIxArgs>>(
    accounts: &DisableLstInputAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = disable_lst_input_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn disable_lst_input_invoke_signed<'info, A: Into<DisableLstInputIxArgs>>(
    accounts: &DisableLstInputAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = disable_lst_input_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DISABLE_LST_INPUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn disable_lst_input_verify_account_keys(
    accounts: &DisableLstInputAccounts<'_, '_>,
    keys: &DisableLstInputKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn disable_lst_input_verify_account_privileges<'me, 'info>(
    accounts: &DisableLstInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.lst_state_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ENABLE_LST_INPUT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct EnableLstInputAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///Mint of the LST to re-enable input for
    pub lst_mint: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct EnableLstInputKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///Mint of the LST to re-enable input for
    pub lst_mint: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
}
impl From<&EnableLstInputAccounts<'_, '_>> for EnableLstInputKeys {
    fn from(accounts: &EnableLstInputAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            lst_mint: *accounts.lst_mint.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
        }
    }
}
impl From<&EnableLstInputKeys> for [AccountMeta; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN] {
    fn from(keys: &EnableLstInputKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
        ]
    }
}
impl From<[Pubkey; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN]> for EnableLstInputKeys {
    fn from(pubkeys: [Pubkey; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            lst_mint: pubkeys[1],
            pool_state: pubkeys[2],
            lst_state_list: pubkeys[3],
        }
    }
}
impl<'info> From<&EnableLstInputAccounts<'_, 'info>>
    for [AccountInfo<'info>; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EnableLstInputAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.lst_mint.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN]>
    for EnableLstInputAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            lst_mint: &arr[1],
            pool_state: &arr[2],
            lst_state_list: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnableLstInputIxArgs {
    pub index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnableLstInputIxData(pub EnableLstInputIxArgs);
pub const ENABLE_LST_INPUT_IX_DISCM: u8 = 6u8;
impl From<EnableLstInputIxArgs> for EnableLstInputIxData {
    fn from(args: EnableLstInputIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EnableLstInputIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ENABLE_LST_INPUT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl EnableLstInputIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ENABLE_LST_INPUT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ENABLE_LST_INPUT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EnableLstInputIxArgs::deserialize(buf)?))
    }
}
pub fn enable_lst_input_ix<K: Into<EnableLstInputKeys>, A: Into<EnableLstInputIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EnableLstInputKeys = accounts.into();
    let metas: [AccountMeta; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EnableLstInputIxArgs = args.into();
    let data: EnableLstInputIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_lst_input_invoke<'info, A: Into<EnableLstInputIxArgs>>(
    accounts: &EnableLstInputAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = enable_lst_input_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn enable_lst_input_invoke_signed<'info, A: Into<EnableLstInputIxArgs>>(
    accounts: &EnableLstInputAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = enable_lst_input_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ENABLE_LST_INPUT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn enable_lst_input_verify_account_keys(
    accounts: &EnableLstInputAccounts<'_, '_>,
    keys: &EnableLstInputKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn enable_lst_input_verify_account_privileges<'me, 'info>(
    accounts: &EnableLstInputAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.lst_state_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_LST_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct AddLstAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///Account paying the SOL rent for the new space and accounts
    pub payer: &'me AccountInfo<'info>,
    ///Mint of the new LST to add
    pub lst_mint: &'me AccountInfo<'info>,
    ///LST reserves token account to create
    pub pool_reserves: &'me AccountInfo<'info>,
    ///The LST protocol fee accumulator token account to create
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLstKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///Account paying the SOL rent for the new space and accounts
    pub payer: Pubkey,
    ///Mint of the new LST to add
    pub lst_mint: Pubkey,
    ///LST reserves token account to create
    pub pool_reserves: Pubkey,
    ///The LST protocol fee accumulator token account to create
    pub protocol_fee_accumulator: Pubkey,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&AddLstAccounts<'_, '_>> for AddLstKeys {
    fn from(accounts: &AddLstAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            payer: *accounts.payer.key,
            lst_mint: *accounts.lst_mint.key,
            pool_reserves: *accounts.pool_reserves.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            protocol_fee_accumulator_auth: *accounts.protocol_fee_accumulator_auth.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&AddLstKeys> for [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddLstKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.pool_reserves, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new(keys.protocol_fee_accumulator_auth, false),
            AccountMeta::new_readonly(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_LST_IX_ACCOUNTS_LEN]> for AddLstKeys {
    fn from(pubkeys: [Pubkey; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            payer: pubkeys[1],
            lst_mint: pubkeys[2],
            pool_reserves: pubkeys[3],
            protocol_fee_accumulator: pubkeys[4],
            protocol_fee_accumulator_auth: pubkeys[5],
            pool_state: pubkeys[6],
            lst_state_list: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<&AddLstAccounts<'_, 'info>> for [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] {
    fn from(accounts: &AddLstAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.payer.clone(),
            accounts.lst_mint.clone(),
            accounts.pool_reserves.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.protocol_fee_accumulator_auth.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]>
    for AddLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            payer: &arr[1],
            lst_mint: &arr[2],
            pool_reserves: &arr[3],
            protocol_fee_accumulator: &arr[4],
            protocol_fee_accumulator_auth: &arr[5],
            pool_state: &arr[6],
            lst_state_list: &arr[7],
            system_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLstIxArgs {
    pub index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLstIxData(pub AddLstIxArgs);
pub const ADD_LST_IX_DISCM: u8 = 7u8;
impl From<AddLstIxArgs> for AddLstIxData {
    fn from(args: AddLstIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddLstIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ADD_LST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl AddLstIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ADD_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLstIxArgs::deserialize(buf)?))
    }
}
pub fn add_lst_ix<K: Into<AddLstKeys>, A: Into<AddLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddLstKeys = accounts.into();
    let metas: [AccountMeta; ADD_LST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddLstIxArgs = args.into();
    let data: AddLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_lst_invoke<'info, A: Into<AddLstIxArgs>>(
    accounts: &AddLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_lst_invoke_signed<'info, A: Into<AddLstIxArgs>>(
    accounts: &AddLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_lst_verify_account_keys(
    accounts: &AddLstAccounts<'_, '_>,
    keys: &AddLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.payer.key, &keys.payer),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_reserves.key, &keys.pool_reserves),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (
            accounts.protocol_fee_accumulator_auth.key,
            &keys.protocol_fee_accumulator_auth,
        ),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_lst_verify_account_privileges<'me, 'info>(
    accounts: &AddLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_reserves,
        accounts.protocol_fee_accumulator,
        accounts.protocol_fee_accumulator_auth,
        accounts.lst_state_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_LST_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///Account to refund SOL rent to
    pub refund_rent_to: &'me AccountInfo<'info>,
    ///Mint of the new LST to remove
    pub lst_mint: &'me AccountInfo<'info>,
    ///LST reserves token account to destory
    pub pool_reserves: &'me AccountInfo<'info>,
    ///The LST protocol fee accumulator token account to destroy
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLstKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///Account to refund SOL rent to
    pub refund_rent_to: Pubkey,
    ///Mint of the new LST to remove
    pub lst_mint: Pubkey,
    ///LST reserves token account to destory
    pub pool_reserves: Pubkey,
    ///The LST protocol fee accumulator token account to destroy
    pub protocol_fee_accumulator: Pubkey,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&RemoveLstAccounts<'_, '_>> for RemoveLstKeys {
    fn from(accounts: &RemoveLstAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            refund_rent_to: *accounts.refund_rent_to.key,
            lst_mint: *accounts.lst_mint.key,
            pool_reserves: *accounts.pool_reserves.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            protocol_fee_accumulator_auth: *accounts.protocol_fee_accumulator_auth.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&RemoveLstKeys> for [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveLstKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.refund_rent_to, false),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new(keys.pool_reserves, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new(keys.protocol_fee_accumulator_auth, false),
            AccountMeta::new_readonly(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]> for RemoveLstKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            refund_rent_to: pubkeys[1],
            lst_mint: pubkeys[2],
            pool_reserves: pubkeys[3],
            protocol_fee_accumulator: pubkeys[4],
            protocol_fee_accumulator_auth: pubkeys[5],
            pool_state: pubkeys[6],
            lst_state_list: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<&RemoveLstAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveLstAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.refund_rent_to.clone(),
            accounts.lst_mint.clone(),
            accounts.pool_reserves.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.protocol_fee_accumulator_auth.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]>
    for RemoveLstAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            refund_rent_to: &arr[1],
            lst_mint: &arr[2],
            pool_reserves: &arr[3],
            protocol_fee_accumulator: &arr[4],
            protocol_fee_accumulator_auth: &arr[5],
            pool_state: &arr[6],
            lst_state_list: &arr[7],
            system_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLstIxArgs {
    pub index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLstIxData(pub RemoveLstIxArgs);
pub const REMOVE_LST_IX_DISCM: u8 = 8u8;
impl From<RemoveLstIxArgs> for RemoveLstIxData {
    fn from(args: RemoveLstIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveLstIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_LST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RemoveLstIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REMOVE_LST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveLstIxArgs::deserialize(buf)?))
    }
}
pub fn remove_lst_ix<K: Into<RemoveLstKeys>, A: Into<RemoveLstIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveLstKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_LST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveLstIxArgs = args.into();
    let data: RemoveLstIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_lst_invoke<'info, A: Into<RemoveLstIxArgs>>(
    accounts: &RemoveLstAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_lst_invoke_signed<'info, A: Into<RemoveLstIxArgs>>(
    accounts: &RemoveLstAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_lst_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LST_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_lst_verify_account_keys(
    accounts: &RemoveLstAccounts<'_, '_>,
    keys: &RemoveLstKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.refund_rent_to.key, &keys.refund_rent_to),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_reserves.key, &keys.pool_reserves),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (
            accounts.protocol_fee_accumulator_auth.key,
            &keys.protocol_fee_accumulator_auth,
        ),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_lst_verify_account_privileges<'me, 'info>(
    accounts: &RemoveLstAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.refund_rent_to,
        accounts.pool_reserves,
        accounts.protocol_fee_accumulator,
        accounts.protocol_fee_accumulator_auth,
        accounts.lst_state_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetSolValueCalculatorAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///Mint of the LST to set SOL value calculator for
    pub lst_mint: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///LST reserves token account of the pool
    pub pool_reserves: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetSolValueCalculatorKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///Mint of the LST to set SOL value calculator for
    pub lst_mint: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///LST reserves token account of the pool
    pub pool_reserves: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
}
impl From<&SetSolValueCalculatorAccounts<'_, '_>> for SetSolValueCalculatorKeys {
    fn from(accounts: &SetSolValueCalculatorAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            lst_mint: *accounts.lst_mint.key,
            pool_state: *accounts.pool_state.key,
            pool_reserves: *accounts.pool_reserves.key,
            lst_state_list: *accounts.lst_state_list.key,
        }
    }
}
impl From<&SetSolValueCalculatorKeys> for [AccountMeta; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetSolValueCalculatorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.lst_mint, false),
            AccountMeta::new_readonly(keys.pool_state, false),
            AccountMeta::new_readonly(keys.pool_reserves, false),
            AccountMeta::new(keys.lst_state_list, false),
        ]
    }
}
impl From<[Pubkey; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN]> for SetSolValueCalculatorKeys {
    fn from(pubkeys: [Pubkey; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            lst_mint: pubkeys[1],
            pool_state: pubkeys[2],
            pool_reserves: pubkeys[3],
            lst_state_list: pubkeys[4],
        }
    }
}
impl<'info> From<&SetSolValueCalculatorAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetSolValueCalculatorAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.lst_mint.clone(),
            accounts.pool_state.clone(),
            accounts.pool_reserves.clone(),
            accounts.lst_state_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN]>
    for SetSolValueCalculatorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            lst_mint: &arr[1],
            pool_state: &arr[2],
            pool_reserves: &arr[3],
            lst_state_list: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetSolValueCalculatorIxArgs {
    pub index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetSolValueCalculatorIxData(pub SetSolValueCalculatorIxArgs);
pub const SET_SOL_VALUE_CALCULATOR_IX_DISCM: u8 = 9u8;
impl From<SetSolValueCalculatorIxArgs> for SetSolValueCalculatorIxData {
    fn from(args: SetSolValueCalculatorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetSolValueCalculatorIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_SOL_VALUE_CALCULATOR_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetSolValueCalculatorIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_SOL_VALUE_CALCULATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_SOL_VALUE_CALCULATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetSolValueCalculatorIxArgs::deserialize(buf)?))
    }
}
pub fn set_sol_value_calculator_ix<
    K: Into<SetSolValueCalculatorKeys>,
    A: Into<SetSolValueCalculatorIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetSolValueCalculatorKeys = accounts.into();
    let metas: [AccountMeta; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetSolValueCalculatorIxArgs = args.into();
    let data: SetSolValueCalculatorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_sol_value_calculator_invoke<'info, A: Into<SetSolValueCalculatorIxArgs>>(
    accounts: &SetSolValueCalculatorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_sol_value_calculator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_sol_value_calculator_invoke_signed<'info, A: Into<SetSolValueCalculatorIxArgs>>(
    accounts: &SetSolValueCalculatorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_sol_value_calculator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_sol_value_calculator_verify_account_keys(
    accounts: &SetSolValueCalculatorAccounts<'_, '_>,
    keys: &SetSolValueCalculatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.lst_mint.key, &keys.lst_mint),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.pool_reserves.key, &keys.pool_reserves),
        (accounts.lst_state_list.key, &keys.lst_state_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_sol_value_calculator_verify_account_privileges<'me, 'info>(
    accounts: &SetSolValueCalculatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lst_state_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_ADMIN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetAdminAccounts<'me, 'info> {
    ///The pool's current admin
    pub current_admin: &'me AccountInfo<'info>,
    ///The pool's new admin
    pub new_admin: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetAdminKeys {
    ///The pool's current admin
    pub current_admin: Pubkey,
    ///The pool's new admin
    pub new_admin: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&SetAdminAccounts<'_, '_>> for SetAdminKeys {
    fn from(accounts: &SetAdminAccounts) -> Self {
        Self {
            current_admin: *accounts.current_admin.key,
            new_admin: *accounts.new_admin.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&SetAdminKeys> for [AccountMeta; SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetAdminKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.current_admin, true),
            AccountMeta::new_readonly(keys.new_admin, false),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; SET_ADMIN_IX_ACCOUNTS_LEN]> for SetAdminKeys {
    fn from(pubkeys: [Pubkey; SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_admin: pubkeys[0],
            new_admin: pubkeys[1],
            pool_state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetAdminAccounts<'_, 'info>> for [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SetAdminAccounts<'_, 'info>) -> Self {
        [
            accounts.current_admin.clone(),
            accounts.new_admin.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN]>
    for SetAdminAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_admin: &arr[0],
            new_admin: &arr[1],
            pool_state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetAdminIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAdminIxData(pub SetAdminIxArgs);
pub const SET_ADMIN_IX_DISCM: u8 = 10u8;
impl From<SetAdminIxArgs> for SetAdminIxData {
    fn from(args: SetAdminIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetAdminIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_ADMIN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetAdminIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_ADMIN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_ADMIN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetAdminIxArgs::deserialize(buf)?))
    }
}
pub fn set_admin_ix<K: Into<SetAdminKeys>, A: Into<SetAdminIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetAdminKeys = accounts.into();
    let metas: [AccountMeta; SET_ADMIN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetAdminIxArgs = args.into();
    let data: SetAdminIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_admin_invoke<'info, A: Into<SetAdminIxArgs>>(
    accounts: &SetAdminAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_admin_invoke_signed<'info, A: Into<SetAdminIxArgs>>(
    accounts: &SetAdminAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_admin_verify_account_keys(
    accounts: &SetAdminAccounts<'_, '_>,
    keys: &SetAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.current_admin.key, &keys.current_admin),
        (accounts.new_admin.key, &keys.new_admin),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_admin_verify_account_privileges<'me, 'info>(
    accounts: &SetAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.current_admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&SetProtocolFeeAccounts<'_, '_>> for SetProtocolFeeKeys {
    fn from(accounts: &SetProtocolFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&SetProtocolFeeKeys> for [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetProtocolFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for SetProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            pool_state: pubkeys[1],
        }
    }
}
impl<'info> From<&SetProtocolFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetProtocolFeeAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
    for SetProtocolFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            pool_state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProtocolFeeIxArgs {
    pub new_protocol_fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProtocolFeeIxData(pub SetProtocolFeeIxArgs);
pub const SET_PROTOCOL_FEE_IX_DISCM: u8 = 11u8;
impl From<SetProtocolFeeIxArgs> for SetProtocolFeeIxData {
    fn from(args: SetProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetProtocolFeeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_PROTOCOL_FEE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetProtocolFeeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_PROTOCOL_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_PROTOCOL_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetProtocolFeeIxArgs::deserialize(buf)?))
    }
}
pub fn set_protocol_fee_ix<K: Into<SetProtocolFeeKeys>, A: Into<SetProtocolFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetProtocolFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetProtocolFeeIxArgs = args.into();
    let data: SetProtocolFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_invoke<'info, A: Into<SetProtocolFeeIxArgs>>(
    accounts: &SetProtocolFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_protocol_fee_invoke_signed<'info, A: Into<SetProtocolFeeIxArgs>>(
    accounts: &SetProtocolFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_protocol_fee_verify_account_keys(
    accounts: &SetProtocolFeeAccounts<'_, '_>,
    keys: &SetProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: &SetProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeBeneficiaryAccounts<'me, 'info> {
    ///The pool's current protocol fee beneficiary
    pub current_beneficiary: &'me AccountInfo<'info>,
    ///The pool's new protocol fee beneficiary
    pub new_beneficiary: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeBeneficiaryKeys {
    ///The pool's current protocol fee beneficiary
    pub current_beneficiary: Pubkey,
    ///The pool's new protocol fee beneficiary
    pub new_beneficiary: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&SetProtocolFeeBeneficiaryAccounts<'_, '_>> for SetProtocolFeeBeneficiaryKeys {
    fn from(accounts: &SetProtocolFeeBeneficiaryAccounts) -> Self {
        Self {
            current_beneficiary: *accounts.current_beneficiary.key,
            new_beneficiary: *accounts.new_beneficiary.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&SetProtocolFeeBeneficiaryKeys>
    for [AccountMeta; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SetProtocolFeeBeneficiaryKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.current_beneficiary, true),
            AccountMeta::new_readonly(keys.new_beneficiary, false),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]>
    for SetProtocolFeeBeneficiaryKeys
{
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_beneficiary: pubkeys[0],
            new_beneficiary: pubkeys[1],
            pool_state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetProtocolFeeBeneficiaryAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetProtocolFeeBeneficiaryAccounts<'_, 'info>) -> Self {
        [
            accounts.current_beneficiary.clone(),
            accounts.new_beneficiary.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]>
    for SetProtocolFeeBeneficiaryAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            current_beneficiary: &arr[0],
            new_beneficiary: &arr[1],
            pool_state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProtocolFeeBeneficiaryIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetProtocolFeeBeneficiaryIxData(pub SetProtocolFeeBeneficiaryIxArgs);
pub const SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM: u8 = 12u8;
impl From<SetProtocolFeeBeneficiaryIxArgs> for SetProtocolFeeBeneficiaryIxData {
    fn from(args: SetProtocolFeeBeneficiaryIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetProtocolFeeBeneficiaryIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetProtocolFeeBeneficiaryIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_PROTOCOL_FEE_BENEFICIARY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetProtocolFeeBeneficiaryIxArgs::deserialize(buf)?))
    }
}
pub fn set_protocol_fee_beneficiary_ix<
    K: Into<SetProtocolFeeBeneficiaryKeys>,
    A: Into<SetProtocolFeeBeneficiaryIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetProtocolFeeBeneficiaryKeys = accounts.into();
    let metas: [AccountMeta; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetProtocolFeeBeneficiaryIxArgs = args.into();
    let data: SetProtocolFeeBeneficiaryIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_beneficiary_invoke<'info, A: Into<SetProtocolFeeBeneficiaryIxArgs>>(
    accounts: &SetProtocolFeeBeneficiaryAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_protocol_fee_beneficiary_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_protocol_fee_beneficiary_invoke_signed<
    'info,
    A: Into<SetProtocolFeeBeneficiaryIxArgs>,
>(
    accounts: &SetProtocolFeeBeneficiaryAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_protocol_fee_beneficiary_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_BENEFICIARY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_protocol_fee_beneficiary_verify_account_keys(
    accounts: &SetProtocolFeeBeneficiaryAccounts<'_, '_>,
    keys: &SetProtocolFeeBeneficiaryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.current_beneficiary.key, &keys.current_beneficiary),
        (accounts.new_beneficiary.key, &keys.new_beneficiary),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_protocol_fee_beneficiary_verify_account_privileges<'me, 'info>(
    accounts: &SetProtocolFeeBeneficiaryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.current_beneficiary] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetPricingProgramAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///The pool's new pricing program
    pub new_pricing_program: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetPricingProgramKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///The pool's new pricing program
    pub new_pricing_program: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&SetPricingProgramAccounts<'_, '_>> for SetPricingProgramKeys {
    fn from(accounts: &SetPricingProgramAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            new_pricing_program: *accounts.new_pricing_program.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&SetPricingProgramKeys> for [AccountMeta; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetPricingProgramKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.new_pricing_program, false),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN]> for SetPricingProgramKeys {
    fn from(pubkeys: [Pubkey; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            new_pricing_program: pubkeys[1],
            pool_state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetPricingProgramAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetPricingProgramAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.new_pricing_program.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN]>
    for SetPricingProgramAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            new_pricing_program: &arr[1],
            pool_state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPricingProgramIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetPricingProgramIxData(pub SetPricingProgramIxArgs);
pub const SET_PRICING_PROGRAM_IX_DISCM: u8 = 13u8;
impl From<SetPricingProgramIxArgs> for SetPricingProgramIxData {
    fn from(args: SetPricingProgramIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetPricingProgramIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_PRICING_PROGRAM_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetPricingProgramIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_PRICING_PROGRAM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_PRICING_PROGRAM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetPricingProgramIxArgs::deserialize(buf)?))
    }
}
pub fn set_pricing_program_ix<K: Into<SetPricingProgramKeys>, A: Into<SetPricingProgramIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetPricingProgramKeys = accounts.into();
    let metas: [AccountMeta; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetPricingProgramIxArgs = args.into();
    let data: SetPricingProgramIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_pricing_program_invoke<'info, A: Into<SetPricingProgramIxArgs>>(
    accounts: &SetPricingProgramAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_pricing_program_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_pricing_program_invoke_signed<'info, A: Into<SetPricingProgramIxArgs>>(
    accounts: &SetPricingProgramAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_pricing_program_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PRICING_PROGRAM_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_pricing_program_verify_account_keys(
    accounts: &SetPricingProgramAccounts<'_, '_>,
    keys: &SetPricingProgramKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.new_pricing_program.key, &keys.new_pricing_program),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_pricing_program_verify_account_privileges<'me, 'info>(
    accounts: &SetPricingProgramAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesAccounts<'me, 'info> {
    ///The pool's protocol fee beneficiary
    pub protocol_fee_beneficiary: &'me AccountInfo<'info>,
    ///Token account to withdraw all accumulated protocol fees to
    pub withdraw_to: &'me AccountInfo<'info>,
    ///The LST protocol fee accumulator token account to create
    pub protocol_fee_accumulator: &'me AccountInfo<'info>,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeesKeys {
    ///The pool's protocol fee beneficiary
    pub protocol_fee_beneficiary: Pubkey,
    ///Token account to withdraw all accumulated protocol fees to
    pub withdraw_to: Pubkey,
    ///The LST protocol fee accumulator token account to create
    pub protocol_fee_accumulator: Pubkey,
    ///The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"]
    pub protocol_fee_accumulator_auth: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&WithdrawProtocolFeesAccounts<'_, '_>> for WithdrawProtocolFeesKeys {
    fn from(accounts: &WithdrawProtocolFeesAccounts) -> Self {
        Self {
            protocol_fee_beneficiary: *accounts.protocol_fee_beneficiary.key,
            withdraw_to: *accounts.withdraw_to.key,
            protocol_fee_accumulator: *accounts.protocol_fee_accumulator.key,
            protocol_fee_accumulator_auth: *accounts.protocol_fee_accumulator_auth.key,
            token_program: *accounts.token_program.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&WithdrawProtocolFeesKeys> for [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: &WithdrawProtocolFeesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.protocol_fee_beneficiary, true),
            AccountMeta::new(keys.withdraw_to, false),
            AccountMeta::new(keys.protocol_fee_accumulator, false),
            AccountMeta::new(keys.protocol_fee_accumulator_auth, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]> for WithdrawProtocolFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_fee_beneficiary: pubkeys[0],
            withdraw_to: pubkeys[1],
            protocol_fee_accumulator: pubkeys[2],
            protocol_fee_accumulator_auth: pubkeys[3],
            token_program: pubkeys[4],
            pool_state: pubkeys[5],
        }
    }
}
impl<'info> From<&WithdrawProtocolFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &WithdrawProtocolFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.protocol_fee_beneficiary.clone(),
            accounts.withdraw_to.clone(),
            accounts.protocol_fee_accumulator.clone(),
            accounts.protocol_fee_accumulator_auth.clone(),
            accounts.token_program.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]>
    for WithdrawProtocolFeesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            protocol_fee_beneficiary: &arr[0],
            withdraw_to: &arr[1],
            protocol_fee_accumulator: &arr[2],
            protocol_fee_accumulator_auth: &arr[3],
            token_program: &arr[4],
            pool_state: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawProtocolFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeesIxData(pub WithdrawProtocolFeesIxArgs);
pub const WITHDRAW_PROTOCOL_FEES_IX_DISCM: u8 = 14u8;
impl From<WithdrawProtocolFeesIxArgs> for WithdrawProtocolFeesIxData {
    fn from(args: WithdrawProtocolFeesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for WithdrawProtocolFeesIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_PROTOCOL_FEES_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl WithdrawProtocolFeesIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_PROTOCOL_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawProtocolFeesIxArgs::deserialize(buf)?))
    }
}
pub fn withdraw_protocol_fees_ix<
    K: Into<WithdrawProtocolFeesKeys>,
    A: Into<WithdrawProtocolFeesIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawProtocolFeesKeys = accounts.into();
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: WithdrawProtocolFeesIxArgs = args.into();
    let data: WithdrawProtocolFeesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fees_invoke<'info, A: Into<WithdrawProtocolFeesIxArgs>>(
    accounts: &WithdrawProtocolFeesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_protocol_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_protocol_fees_invoke_signed<'info, A: Into<WithdrawProtocolFeesIxArgs>>(
    accounts: &WithdrawProtocolFeesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_protocol_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn withdraw_protocol_fees_verify_account_keys(
    accounts: &WithdrawProtocolFeesAccounts<'_, '_>,
    keys: &WithdrawProtocolFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.protocol_fee_beneficiary.key,
            &keys.protocol_fee_beneficiary,
        ),
        (accounts.withdraw_to.key, &keys.withdraw_to),
        (
            accounts.protocol_fee_accumulator.key,
            &keys.protocol_fee_accumulator,
        ),
        (
            accounts.protocol_fee_accumulator_auth.key,
            &keys.protocol_fee_accumulator_auth,
        ),
        (accounts.token_program.key, &keys.token_program),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fees_verify_account_privileges<'me, 'info>(
    accounts: &WithdrawProtocolFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_to,
        accounts.protocol_fee_accumulator,
        accounts.protocol_fee_accumulator_auth,
        accounts.pool_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.protocol_fee_beneficiary] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct AddDisablePoolAuthorityAccounts<'me, 'info> {
    ///Account paying for additional rent for realloc
    pub payer: &'me AccountInfo<'info>,
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///The new disable pool authority to add
    pub new_authority: &'me AccountInfo<'info>,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddDisablePoolAuthorityKeys {
    ///Account paying for additional rent for realloc
    pub payer: Pubkey,
    ///The pool's admin
    pub admin: Pubkey,
    ///The new disable pool authority to add
    pub new_authority: Pubkey,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&AddDisablePoolAuthorityAccounts<'_, '_>> for AddDisablePoolAuthorityKeys {
    fn from(accounts: &AddDisablePoolAuthorityAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            admin: *accounts.admin.key,
            new_authority: *accounts.new_authority.key,
            disable_pool_authority_list: *accounts.disable_pool_authority_list.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&AddDisablePoolAuthorityKeys>
    for [AccountMeta; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &AddDisablePoolAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.new_authority, false),
            AccountMeta::new(keys.disable_pool_authority_list, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]> for AddDisablePoolAuthorityKeys {
    fn from(pubkeys: [Pubkey; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            admin: pubkeys[1],
            new_authority: pubkeys[2],
            disable_pool_authority_list: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<&AddDisablePoolAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddDisablePoolAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.admin.clone(),
            accounts.new_authority.clone(),
            accounts.disable_pool_authority_list.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]>
    for AddDisablePoolAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            admin: &arr[1],
            new_authority: &arr[2],
            disable_pool_authority_list: &arr[3],
            system_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddDisablePoolAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct AddDisablePoolAuthorityIxData(pub AddDisablePoolAuthorityIxArgs);
pub const ADD_DISABLE_POOL_AUTHORITY_IX_DISCM: u8 = 15u8;
impl From<AddDisablePoolAuthorityIxArgs> for AddDisablePoolAuthorityIxData {
    fn from(args: AddDisablePoolAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddDisablePoolAuthorityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ADD_DISABLE_POOL_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl AddDisablePoolAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ADD_DISABLE_POOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_DISABLE_POOL_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddDisablePoolAuthorityIxArgs::deserialize(buf)?))
    }
}
pub fn add_disable_pool_authority_ix<
    K: Into<AddDisablePoolAuthorityKeys>,
    A: Into<AddDisablePoolAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddDisablePoolAuthorityKeys = accounts.into();
    let metas: [AccountMeta; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddDisablePoolAuthorityIxArgs = args.into();
    let data: AddDisablePoolAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_disable_pool_authority_invoke<'info, A: Into<AddDisablePoolAuthorityIxArgs>>(
    accounts: &AddDisablePoolAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_disable_pool_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_disable_pool_authority_invoke_signed<'info, A: Into<AddDisablePoolAuthorityIxArgs>>(
    accounts: &AddDisablePoolAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_disable_pool_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_disable_pool_authority_verify_account_keys(
    accounts: &AddDisablePoolAuthorityAccounts<'_, '_>,
    keys: &AddDisablePoolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.admin.key, &keys.admin),
        (accounts.new_authority.key, &keys.new_authority),
        (
            accounts.disable_pool_authority_list.key,
            &keys.disable_pool_authority_list,
        ),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_disable_pool_authority_verify_account_privileges<'me, 'info>(
    accounts: &AddDisablePoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.disable_pool_authority_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer, accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RemoveDisablePoolAuthorityAccounts<'me, 'info> {
    ///The account to refund SOL rent to after resizing
    pub refund_rent_to: &'me AccountInfo<'info>,
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///The authority to remove
    pub authority: &'me AccountInfo<'info>,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveDisablePoolAuthorityKeys {
    ///The account to refund SOL rent to after resizing
    pub refund_rent_to: Pubkey,
    ///The pool's admin
    pub admin: Pubkey,
    ///The authority to remove
    pub authority: Pubkey,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: Pubkey,
}
impl From<&RemoveDisablePoolAuthorityAccounts<'_, '_>> for RemoveDisablePoolAuthorityKeys {
    fn from(accounts: &RemoveDisablePoolAuthorityAccounts) -> Self {
        Self {
            refund_rent_to: *accounts.refund_rent_to.key,
            admin: *accounts.admin.key,
            authority: *accounts.authority.key,
            disable_pool_authority_list: *accounts.disable_pool_authority_list.key,
        }
    }
}
impl From<&RemoveDisablePoolAuthorityKeys>
    for [AccountMeta; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemoveDisablePoolAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.refund_rent_to, true),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.disable_pool_authority_list, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]>
    for RemoveDisablePoolAuthorityKeys
{
    fn from(pubkeys: [Pubkey; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            refund_rent_to: pubkeys[0],
            admin: pubkeys[1],
            authority: pubkeys[2],
            disable_pool_authority_list: pubkeys[3],
        }
    }
}
impl<'info> From<&RemoveDisablePoolAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveDisablePoolAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.refund_rent_to.clone(),
            accounts.admin.clone(),
            accounts.authority.clone(),
            accounts.disable_pool_authority_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]>
    for RemoveDisablePoolAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            refund_rent_to: &arr[0],
            admin: &arr[1],
            authority: &arr[2],
            disable_pool_authority_list: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveDisablePoolAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveDisablePoolAuthorityIxData(pub RemoveDisablePoolAuthorityIxArgs);
pub const REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM: u8 = 16u8;
impl From<RemoveDisablePoolAuthorityIxArgs> for RemoveDisablePoolAuthorityIxData {
    fn from(args: RemoveDisablePoolAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveDisablePoolAuthorityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RemoveDisablePoolAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_DISABLE_POOL_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveDisablePoolAuthorityIxArgs::deserialize(buf)?))
    }
}
pub fn remove_disable_pool_authority_ix<
    K: Into<RemoveDisablePoolAuthorityKeys>,
    A: Into<RemoveDisablePoolAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveDisablePoolAuthorityKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveDisablePoolAuthorityIxArgs = args.into();
    let data: RemoveDisablePoolAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_disable_pool_authority_invoke<'info, A: Into<RemoveDisablePoolAuthorityIxArgs>>(
    accounts: &RemoveDisablePoolAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_disable_pool_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_disable_pool_authority_invoke_signed<
    'info,
    A: Into<RemoveDisablePoolAuthorityIxArgs>,
>(
    accounts: &RemoveDisablePoolAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_disable_pool_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_DISABLE_POOL_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_disable_pool_authority_verify_account_keys(
    accounts: &RemoveDisablePoolAuthorityAccounts<'_, '_>,
    keys: &RemoveDisablePoolAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.refund_rent_to.key, &keys.refund_rent_to),
        (accounts.admin.key, &keys.admin),
        (accounts.authority.key, &keys.authority),
        (
            accounts.disable_pool_authority_list.key,
            &keys.disable_pool_authority_list,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_disable_pool_authority_verify_account_privileges<'me, 'info>(
    accounts: &RemoveDisablePoolAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.refund_rent_to,
        accounts.disable_pool_authority_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.refund_rent_to, accounts.admin, accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DISABLE_POOL_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DisablePoolAccounts<'me, 'info> {
    ///The pool's admin or a disable pool authority
    pub authority: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DisablePoolKeys {
    ///The pool's admin or a disable pool authority
    pub authority: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///The pool's disable pool authority list singleton PDA
    pub disable_pool_authority_list: Pubkey,
}
impl From<&DisablePoolAccounts<'_, '_>> for DisablePoolKeys {
    fn from(accounts: &DisablePoolAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            disable_pool_authority_list: *accounts.disable_pool_authority_list.key,
        }
    }
}
impl From<&DisablePoolKeys> for [AccountMeta; DISABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &DisablePoolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.disable_pool_authority_list, false),
        ]
    }
}
impl From<[Pubkey; DISABLE_POOL_IX_ACCOUNTS_LEN]> for DisablePoolKeys {
    fn from(pubkeys: [Pubkey; DISABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            pool_state: pubkeys[1],
            disable_pool_authority_list: pubkeys[2],
        }
    }
}
impl<'info> From<&DisablePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DisablePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.disable_pool_authority_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN]>
    for DisablePoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            pool_state: &arr[1],
            disable_pool_authority_list: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DisablePoolIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct DisablePoolIxData(pub DisablePoolIxArgs);
pub const DISABLE_POOL_IX_DISCM: u8 = 17u8;
impl From<DisablePoolIxArgs> for DisablePoolIxData {
    fn from(args: DisablePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DisablePoolIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DISABLE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl DisablePoolIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DISABLE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DISABLE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DisablePoolIxArgs::deserialize(buf)?))
    }
}
pub fn disable_pool_ix<K: Into<DisablePoolKeys>, A: Into<DisablePoolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DisablePoolKeys = accounts.into();
    let metas: [AccountMeta; DISABLE_POOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DisablePoolIxArgs = args.into();
    let data: DisablePoolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn disable_pool_invoke<'info, A: Into<DisablePoolIxArgs>>(
    accounts: &DisablePoolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = disable_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn disable_pool_invoke_signed<'info, A: Into<DisablePoolIxArgs>>(
    accounts: &DisablePoolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = disable_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DISABLE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn disable_pool_verify_account_keys(
    accounts: &DisablePoolAccounts<'_, '_>,
    keys: &DisablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.authority.key, &keys.authority),
        (accounts.pool_state.key, &keys.pool_state),
        (
            accounts.disable_pool_authority_list.key,
            &keys.disable_pool_authority_list,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn disable_pool_verify_account_privileges<'me, 'info>(
    accounts: &DisablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state, accounts.disable_pool_authority_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ENABLE_POOL_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct EnablePoolAccounts<'me, 'info> {
    ///The pool's admin
    pub admin: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct EnablePoolKeys {
    ///The pool's admin
    pub admin: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&EnablePoolAccounts<'_, '_>> for EnablePoolKeys {
    fn from(accounts: &EnablePoolAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&EnablePoolKeys> for [AccountMeta; ENABLE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &EnablePoolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; ENABLE_POOL_IX_ACCOUNTS_LEN]> for EnablePoolKeys {
    fn from(pubkeys: [Pubkey; ENABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            pool_state: pubkeys[1],
        }
    }
}
impl<'info> From<&EnablePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EnablePoolAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.pool_state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN]>
    for EnablePoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            pool_state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnablePoolIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct EnablePoolIxData(pub EnablePoolIxArgs);
pub const ENABLE_POOL_IX_DISCM: u8 = 18u8;
impl From<EnablePoolIxArgs> for EnablePoolIxData {
    fn from(args: EnablePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EnablePoolIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[ENABLE_POOL_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl EnablePoolIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != ENABLE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ENABLE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EnablePoolIxArgs::deserialize(buf)?))
    }
}
pub fn enable_pool_ix<K: Into<EnablePoolKeys>, A: Into<EnablePoolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EnablePoolKeys = accounts.into();
    let metas: [AccountMeta; ENABLE_POOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EnablePoolIxArgs = args.into();
    let data: EnablePoolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn enable_pool_invoke<'info, A: Into<EnablePoolIxArgs>>(
    accounts: &EnablePoolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = enable_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn enable_pool_invoke_signed<'info, A: Into<EnablePoolIxArgs>>(
    accounts: &EnablePoolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = enable_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ENABLE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn enable_pool_verify_account_keys(
    accounts: &EnablePoolAccounts<'_, '_>,
    keys: &EnablePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn enable_pool_verify_account_privileges<'me, 'info>(
    accounts: &EnablePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const START_REBALANCE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct StartRebalanceAccounts<'me, 'info> {
    ///Account paying the 1 lamport rent for RebalanceRecord
    pub payer: &'me AccountInfo<'info>,
    ///The pool's rebalance authority
    pub rebalance_authority: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each lst in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///The RebalanceRecord PDA
    pub rebalance_record: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped from
    pub src_lst_mint: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: &'me AccountInfo<'info>,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: &'me AccountInfo<'info>,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: &'me AccountInfo<'info>,
    ///Source LST token account to withdraw to
    pub withdraw_to: &'me AccountInfo<'info>,
    ///Instructions sysvar
    pub instructions: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Source LST token program
    pub src_lst_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct StartRebalanceKeys {
    ///Account paying the 1 lamport rent for RebalanceRecord
    pub payer: Pubkey,
    ///The pool's rebalance authority
    pub rebalance_authority: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each lst in the pool
    pub lst_state_list: Pubkey,
    ///The RebalanceRecord PDA
    pub rebalance_record: Pubkey,
    ///Mint of the LST being swapped from
    pub src_lst_mint: Pubkey,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: Pubkey,
    ///Source LST reserves token account of the pool
    pub src_pool_reserves: Pubkey,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: Pubkey,
    ///Source LST token account to withdraw to
    pub withdraw_to: Pubkey,
    ///Instructions sysvar
    pub instructions: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Source LST token program
    pub src_lst_token_program: Pubkey,
}
impl From<&StartRebalanceAccounts<'_, '_>> for StartRebalanceKeys {
    fn from(accounts: &StartRebalanceAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            rebalance_authority: *accounts.rebalance_authority.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            rebalance_record: *accounts.rebalance_record.key,
            src_lst_mint: *accounts.src_lst_mint.key,
            dst_lst_mint: *accounts.dst_lst_mint.key,
            src_pool_reserves: *accounts.src_pool_reserves.key,
            dst_pool_reserves: *accounts.dst_pool_reserves.key,
            withdraw_to: *accounts.withdraw_to.key,
            instructions: *accounts.instructions.key,
            system_program: *accounts.system_program.key,
            src_lst_token_program: *accounts.src_lst_token_program.key,
        }
    }
}
impl From<&StartRebalanceKeys> for [AccountMeta; START_REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: &StartRebalanceKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rebalance_authority, true),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.rebalance_record, false),
            AccountMeta::new_readonly(keys.src_lst_mint, false),
            AccountMeta::new_readonly(keys.dst_lst_mint, false),
            AccountMeta::new(keys.src_pool_reserves, false),
            AccountMeta::new(keys.dst_pool_reserves, false),
            AccountMeta::new(keys.withdraw_to, false),
            AccountMeta::new_readonly(keys.instructions, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.src_lst_token_program, false),
        ]
    }
}
impl From<[Pubkey; START_REBALANCE_IX_ACCOUNTS_LEN]> for StartRebalanceKeys {
    fn from(pubkeys: [Pubkey; START_REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            rebalance_authority: pubkeys[1],
            pool_state: pubkeys[2],
            lst_state_list: pubkeys[3],
            rebalance_record: pubkeys[4],
            src_lst_mint: pubkeys[5],
            dst_lst_mint: pubkeys[6],
            src_pool_reserves: pubkeys[7],
            dst_pool_reserves: pubkeys[8],
            withdraw_to: pubkeys[9],
            instructions: pubkeys[10],
            system_program: pubkeys[11],
            src_lst_token_program: pubkeys[12],
        }
    }
}
impl<'info> From<&StartRebalanceAccounts<'_, 'info>>
    for [AccountInfo<'info>; START_REBALANCE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &StartRebalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.rebalance_authority.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.rebalance_record.clone(),
            accounts.src_lst_mint.clone(),
            accounts.dst_lst_mint.clone(),
            accounts.src_pool_reserves.clone(),
            accounts.dst_pool_reserves.clone(),
            accounts.withdraw_to.clone(),
            accounts.instructions.clone(),
            accounts.system_program.clone(),
            accounts.src_lst_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; START_REBALANCE_IX_ACCOUNTS_LEN]>
    for StartRebalanceAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; START_REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            rebalance_authority: &arr[1],
            pool_state: &arr[2],
            lst_state_list: &arr[3],
            rebalance_record: &arr[4],
            src_lst_mint: &arr[5],
            dst_lst_mint: &arr[6],
            src_pool_reserves: &arr[7],
            dst_pool_reserves: &arr[8],
            withdraw_to: &arr[9],
            instructions: &arr[10],
            system_program: &arr[11],
            src_lst_token_program: &arr[12],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StartRebalanceIxArgs {
    pub src_lst_calc_accs: u8,
    pub src_lst_index: u32,
    pub dst_lst_index: u32,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StartRebalanceIxData(pub StartRebalanceIxArgs);
pub const START_REBALANCE_IX_DISCM: u8 = 19u8;
impl From<StartRebalanceIxArgs> for StartRebalanceIxData {
    fn from(args: StartRebalanceIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StartRebalanceIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[START_REBALANCE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl StartRebalanceIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != START_REBALANCE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    START_REBALANCE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StartRebalanceIxArgs::deserialize(buf)?))
    }
}
pub fn start_rebalance_ix<K: Into<StartRebalanceKeys>, A: Into<StartRebalanceIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StartRebalanceKeys = accounts.into();
    let metas: [AccountMeta; START_REBALANCE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: StartRebalanceIxArgs = args.into();
    let data: StartRebalanceIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn start_rebalance_invoke<'info, A: Into<StartRebalanceIxArgs>>(
    accounts: &StartRebalanceAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = start_rebalance_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; START_REBALANCE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn start_rebalance_invoke_signed<'info, A: Into<StartRebalanceIxArgs>>(
    accounts: &StartRebalanceAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = start_rebalance_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; START_REBALANCE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn start_rebalance_verify_account_keys(
    accounts: &StartRebalanceAccounts<'_, '_>,
    keys: &StartRebalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.rebalance_authority.key, &keys.rebalance_authority),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.rebalance_record.key, &keys.rebalance_record),
        (accounts.src_lst_mint.key, &keys.src_lst_mint),
        (accounts.dst_lst_mint.key, &keys.dst_lst_mint),
        (accounts.src_pool_reserves.key, &keys.src_pool_reserves),
        (accounts.dst_pool_reserves.key, &keys.dst_pool_reserves),
        (accounts.withdraw_to.key, &keys.withdraw_to),
        (accounts.instructions.key, &keys.instructions),
        (accounts.system_program.key, &keys.system_program),
        (
            accounts.src_lst_token_program.key,
            &keys.src_lst_token_program,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn start_rebalance_verify_account_privileges<'me, 'info>(
    accounts: &StartRebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.rebalance_record,
        accounts.src_pool_reserves,
        accounts.dst_pool_reserves,
        accounts.withdraw_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer, accounts.rebalance_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const END_REBALANCE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct EndRebalanceAccounts<'me, 'info> {
    ///The account to refund the 1 lamport rent for RebalanceRecord
    pub refund_rent_to: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each lst in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///The RebalanceRecord PDA
    pub rebalance_record: &'me AccountInfo<'info>,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: &'me AccountInfo<'info>,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct EndRebalanceKeys {
    ///The account to refund the 1 lamport rent for RebalanceRecord
    pub refund_rent_to: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each lst in the pool
    pub lst_state_list: Pubkey,
    ///The RebalanceRecord PDA
    pub rebalance_record: Pubkey,
    ///Mint of the LST being swapped to
    pub dst_lst_mint: Pubkey,
    ///Destination LST reserves token account of the pool
    pub dst_pool_reserves: Pubkey,
}
impl From<&EndRebalanceAccounts<'_, '_>> for EndRebalanceKeys {
    fn from(accounts: &EndRebalanceAccounts) -> Self {
        Self {
            refund_rent_to: *accounts.refund_rent_to.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            rebalance_record: *accounts.rebalance_record.key,
            dst_lst_mint: *accounts.dst_lst_mint.key,
            dst_pool_reserves: *accounts.dst_pool_reserves.key,
        }
    }
}
impl From<&EndRebalanceKeys> for [AccountMeta; END_REBALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: &EndRebalanceKeys) -> Self {
        [
            AccountMeta::new(keys.refund_rent_to, false),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.rebalance_record, false),
            AccountMeta::new_readonly(keys.dst_lst_mint, false),
            AccountMeta::new_readonly(keys.dst_pool_reserves, false),
        ]
    }
}
impl From<[Pubkey; END_REBALANCE_IX_ACCOUNTS_LEN]> for EndRebalanceKeys {
    fn from(pubkeys: [Pubkey; END_REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            refund_rent_to: pubkeys[0],
            pool_state: pubkeys[1],
            lst_state_list: pubkeys[2],
            rebalance_record: pubkeys[3],
            dst_lst_mint: pubkeys[4],
            dst_pool_reserves: pubkeys[5],
        }
    }
}
impl<'info> From<&EndRebalanceAccounts<'_, 'info>>
    for [AccountInfo<'info>; END_REBALANCE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EndRebalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.refund_rent_to.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.rebalance_record.clone(),
            accounts.dst_lst_mint.clone(),
            accounts.dst_pool_reserves.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; END_REBALANCE_IX_ACCOUNTS_LEN]>
    for EndRebalanceAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; END_REBALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            refund_rent_to: &arr[0],
            pool_state: &arr[1],
            lst_state_list: &arr[2],
            rebalance_record: &arr[3],
            dst_lst_mint: &arr[4],
            dst_pool_reserves: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndRebalanceIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct EndRebalanceIxData(pub EndRebalanceIxArgs);
pub const END_REBALANCE_IX_DISCM: u8 = 20u8;
impl From<EndRebalanceIxArgs> for EndRebalanceIxData {
    fn from(args: EndRebalanceIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EndRebalanceIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[END_REBALANCE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl EndRebalanceIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != END_REBALANCE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    END_REBALANCE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EndRebalanceIxArgs::deserialize(buf)?))
    }
}
pub fn end_rebalance_ix<K: Into<EndRebalanceKeys>, A: Into<EndRebalanceIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EndRebalanceKeys = accounts.into();
    let metas: [AccountMeta; END_REBALANCE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EndRebalanceIxArgs = args.into();
    let data: EndRebalanceIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn end_rebalance_invoke<'info, A: Into<EndRebalanceIxArgs>>(
    accounts: &EndRebalanceAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = end_rebalance_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; END_REBALANCE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn end_rebalance_invoke_signed<'info, A: Into<EndRebalanceIxArgs>>(
    accounts: &EndRebalanceAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = end_rebalance_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; END_REBALANCE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn end_rebalance_verify_account_keys(
    accounts: &EndRebalanceAccounts<'_, '_>,
    keys: &EndRebalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.refund_rent_to.key, &keys.refund_rent_to),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.rebalance_record.key, &keys.rebalance_record),
        (accounts.dst_lst_mint.key, &keys.dst_lst_mint),
        (accounts.dst_pool_reserves.key, &keys.dst_pool_reserves),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn end_rebalance_verify_account_privileges<'me, 'info>(
    accounts: &EndRebalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.refund_rent_to,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.rebalance_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetRebalanceAuthorityAccounts<'me, 'info> {
    ///The pool's rebalance authority or admin
    pub authority: &'me AccountInfo<'info>,
    ///The new rebalance authority to set to
    pub new_rebalance_authority: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetRebalanceAuthorityKeys {
    ///The pool's rebalance authority or admin
    pub authority: Pubkey,
    ///The new rebalance authority to set to
    pub new_rebalance_authority: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
}
impl From<&SetRebalanceAuthorityAccounts<'_, '_>> for SetRebalanceAuthorityKeys {
    fn from(accounts: &SetRebalanceAuthorityAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            new_rebalance_authority: *accounts.new_rebalance_authority.key,
            pool_state: *accounts.pool_state.key,
        }
    }
}
impl From<&SetRebalanceAuthorityKeys> for [AccountMeta; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetRebalanceAuthorityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new_readonly(keys.new_rebalance_authority, false),
            AccountMeta::new(keys.pool_state, false),
        ]
    }
}
impl From<[Pubkey; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN]> for SetRebalanceAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            new_rebalance_authority: pubkeys[1],
            pool_state: pubkeys[2],
        }
    }
}
impl<'info> From<&SetRebalanceAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetRebalanceAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.new_rebalance_authority.clone(),
            accounts.pool_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for SetRebalanceAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            new_rebalance_authority: &arr[1],
            pool_state: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRebalanceAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetRebalanceAuthorityIxData(pub SetRebalanceAuthorityIxArgs);
pub const SET_REBALANCE_AUTHORITY_IX_DISCM: u8 = 21u8;
impl From<SetRebalanceAuthorityIxArgs> for SetRebalanceAuthorityIxData {
    fn from(args: SetRebalanceAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetRebalanceAuthorityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_REBALANCE_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SetRebalanceAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_REBALANCE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_REBALANCE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetRebalanceAuthorityIxArgs::deserialize(buf)?))
    }
}
pub fn set_rebalance_authority_ix<
    K: Into<SetRebalanceAuthorityKeys>,
    A: Into<SetRebalanceAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetRebalanceAuthorityKeys = accounts.into();
    let metas: [AccountMeta; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetRebalanceAuthorityIxArgs = args.into();
    let data: SetRebalanceAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_rebalance_authority_invoke<'info, A: Into<SetRebalanceAuthorityIxArgs>>(
    accounts: &SetRebalanceAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_rebalance_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_rebalance_authority_invoke_signed<'info, A: Into<SetRebalanceAuthorityIxArgs>>(
    accounts: &SetRebalanceAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_rebalance_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_REBALANCE_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_rebalance_authority_verify_account_keys(
    accounts: &SetRebalanceAuthorityAccounts<'_, '_>,
    keys: &SetRebalanceAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.authority.key, &keys.authority),
        (
            accounts.new_rebalance_authority.key,
            &keys.new_rebalance_authority,
        ),
        (accounts.pool_state.key, &keys.pool_state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_rebalance_authority_verify_account_privileges<'me, 'info>(
    accounts: &SetRebalanceAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.pool_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///Account paying for rent
    pub payer: &'me AccountInfo<'info>,
    ///The hardcoded pubkey allowed to initialize the pool
    pub authority: &'me AccountInfo<'info>,
    ///The pool's state singleton PDA
    pub pool_state: &'me AccountInfo<'info>,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: &'me AccountInfo<'info>,
    ///The LP token mint to create
    pub lp_token_mint: &'me AccountInfo<'info>,
    ///Token 2022 program
    pub token_2022: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///Account paying for rent
    pub payer: Pubkey,
    ///The hardcoded pubkey allowed to initialize the pool
    pub authority: Pubkey,
    ///The pool's state singleton PDA
    pub pool_state: Pubkey,
    ///Dynamic list PDA of LstStates for each LST in the pool
    pub lst_state_list: Pubkey,
    ///The LP token mint to create
    pub lp_token_mint: Pubkey,
    ///Token 2022 program
    pub token_2022: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: &InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            pool_state: *accounts.pool_state.key,
            lst_state_list: *accounts.lst_state_list.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_2022: *accounts.token_2022.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.pool_state, false),
            AccountMeta::new(keys.lst_state_list, false),
            AccountMeta::new(keys.lp_token_mint, true),
            AccountMeta::new_readonly(keys.token_2022, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            authority: pubkeys[1],
            pool_state: pubkeys[2],
            lst_state_list: pubkeys[3],
            lp_token_mint: pubkeys[4],
            token_2022: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<&InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.pool_state.clone(),
            accounts.lst_state_list.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_2022.clone(),
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
            authority: &arr[1],
            pool_state: &arr[2],
            lst_state_list: &arr[3],
            lp_token_mint: &arr[4],
            token_2022: &arr[5],
            system_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub protocol_fee_bps: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
pub const INITIALIZE_IX_DISCM: u8 = 22u8;
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(buf)?))
    }
}
pub fn initialize_ix<K: Into<InitializeKeys>, A: Into<InitializeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeIxArgs = args.into();
    let data: InitializeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_invoke<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_invoke_signed<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: &InitializeAccounts<'_, '_>,
    keys: &InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.payer.key, &keys.payer),
        (accounts.authority.key, &keys.authority),
        (accounts.pool_state.key, &keys.pool_state),
        (accounts.lst_state_list.key, &keys.lst_state_list),
        (accounts.lp_token_mint.key, &keys.lp_token_mint),
        (accounts.token_2022.key, &keys.token_2022),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: &InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.pool_state,
        accounts.lst_state_list,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer, accounts.authority, accounts.lp_token_mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
