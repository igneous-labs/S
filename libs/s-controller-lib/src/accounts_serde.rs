use bytemuck::{try_from_bytes, try_from_bytes_mut};
use s_controller_interface::{LstState, PoolState, RebalanceRecord, SControllerError};
use solana_program::pubkey::Pubkey;

/// Tries to reinterpret `pool_state_acc_data` bytes as a PoolState
pub fn try_pool_state(pool_state_acc_data: &[u8]) -> Result<&PoolState, SControllerError> {
    try_from_bytes(pool_state_acc_data).map_err(|_e| SControllerError::InvalidPoolStateData)
}

/// Tries to reinterpret `pool_state_acc_data` bytes as a mutable PoolState
pub fn try_pool_state_mut(
    pool_state_acc_data: &mut [u8],
) -> Result<&mut PoolState, SControllerError> {
    try_from_bytes_mut(pool_state_acc_data).map_err(|_e| SControllerError::InvalidPoolStateData)
}

/// Tries to reinterpret `lst_state_list_acc_data` bytes as a LstStateList
pub fn try_lst_state_list(lst_state_list_acc_data: &[u8]) -> Result<&[LstState], SControllerError> {
    try_list(lst_state_list_acc_data).ok_or(SControllerError::InvalidLstStateListData)
}

/// Tries to reinterpret `lst_state_list_acc_data` bytes as a mutable LstStateList
pub fn try_lst_state_list_mut(
    lst_state_list_acc_data: &mut [u8],
) -> Result<&mut [LstState], SControllerError> {
    try_list_mut(lst_state_list_acc_data).ok_or(SControllerError::InvalidLstStateListData)
}

pub fn try_disable_pool_authority_list(
    disable_pool_authority_list_acc_data: &[u8],
) -> Result<&[Pubkey], SControllerError> {
    try_list(disable_pool_authority_list_acc_data)
        .ok_or(SControllerError::InvalidDisablePoolAuthorityListData)
}

pub fn try_disable_pool_authority_list_mut(
    disable_pool_authority_list_acc_data: &mut [u8],
) -> Result<&mut [Pubkey], SControllerError> {
    try_list_mut(disable_pool_authority_list_acc_data)
        .ok_or(SControllerError::InvalidDisablePoolAuthorityListData)
}

pub fn try_rebalance_record(
    rebalance_record_acc_data: &[u8],
) -> Result<&RebalanceRecord, SControllerError> {
    try_from_bytes(rebalance_record_acc_data)
        .map_err(|_e| SControllerError::InvalidRebalanceRecordData)
}

pub fn try_rebalance_record_mut(
    rebalance_record_acc_data: &mut [u8],
) -> Result<&mut RebalanceRecord, SControllerError> {
    try_from_bytes_mut(rebalance_record_acc_data)
        .map_err(|_e| SControllerError::InvalidRebalanceRecordData)
}

/// Tries to reinterpret `list_acc_data` bytes as a slice.
///
/// `list_acc_data` should only contain data of the items, no headers etc
///
/// Returns None if failed. Could be due to:
/// - `list_acc_data` is not divisible by T's len
/// - `list_acc_data` is not aligned to T's align
fn try_list<T>(list_acc_data: &[u8]) -> Option<&[T]> {
    if list_acc_data.len() % std::mem::size_of::<T>() != 0 {
        return None;
    }
    let ptr = list_acc_data.as_ptr();
    if ptr.align_offset(std::mem::align_of::<T>()) != 0 {
        return None;
    }
    let len = list_acc_data.len() / std::mem::size_of::<T>();
    Some(unsafe { std::slice::from_raw_parts(ptr as *const T, len) })
}

/// Tries to reinterpret `list_acc_data` bytes as a mutable slice.
///
/// `list_acc_data` should only contain data of the items, no headers etc
///
/// Returns None if failed. Could be due to:
/// - `list_acc_data` is not divisible by T's len
/// - `list_acc_data` is not aligned to T's align
fn try_list_mut<T>(list_acc_data: &mut [u8]) -> Option<&mut [T]> {
    if list_acc_data.len() % std::mem::size_of::<T>() != 0 {
        return None;
    }
    let ptr = list_acc_data.as_mut_ptr();
    if ptr.align_offset(std::mem::align_of::<T>()) != 0 {
        return None;
    }
    let len = list_acc_data.len() / std::mem::size_of::<T>();
    Some(unsafe { std::slice::from_raw_parts_mut(ptr as *mut T, len) })
}
