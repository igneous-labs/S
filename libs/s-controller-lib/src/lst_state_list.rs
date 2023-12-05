use s_controller_interface::{LstState, SControllerError};
use solana_program::pubkey::Pubkey;

/// Checks identity of `lst_mint` against `lst_state_list[lst_index]`,
/// returning `lst_state_list[lst_index]` if matches
pub fn try_match_lst_mint_on_list<I: TryInto<usize>>(
    lst_mint: Pubkey,
    lst_state_list: &[LstState],
    lst_index: I,
) -> Result<&LstState, SControllerError> {
    let i = lst_index
        .try_into()
        .map_err(|_e| SControllerError::InvalidLstIndex)?;

    let lst_state = lst_state_list
        .get(i)
        .ok_or(SControllerError::InvalidLstIndex)?;
    if lst_mint != lst_state.mint {
        return Err(SControllerError::InvalidLstIndex);
    }
    Ok(lst_state)
}

pub fn try_find_lst_mint_on_list(
    lst_mint: Pubkey,
    lst_state_list: &[LstState],
) -> Result<(usize, &LstState), SControllerError> {
    lst_state_list
        .iter()
        .enumerate()
        .find(|(_i, s)| s.mint == lst_mint)
        .ok_or(SControllerError::InvalidLstIndex)
}
