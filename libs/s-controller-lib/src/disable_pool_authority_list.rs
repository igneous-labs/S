// TODO: possibly generalize this module and combine it with lst_state_list
use s_controller_interface::SControllerError;
use solana_program::pubkey::Pubkey;

/// Checks identity of element against `list[index]`,
/// returning `list[index]` if matches
pub fn try_match_element_in_list<I: Into<usize>>(
    element: Pubkey,
    list: &[Pubkey],
    index: I,
) -> Result<&Pubkey, SControllerError> {
    let element_in_list = list
        .get(index.into())
        .ok_or(SControllerError::InvalidDisablePoolAuthorityIndex)?; // TODO: more general list error
    if element != *element_in_list {
        return Err(SControllerError::InvalidDisablePoolAuthorityIndex);
    }
    Ok(element_in_list)
}

pub fn try_find_element_in_list(
    element: Pubkey,
    list: &[Pubkey],
) -> Result<(usize, &Pubkey), SControllerError> {
    list.iter()
        .enumerate()
        .find(|(_i, &s)| s == element)
        .ok_or(SControllerError::InvalidDisablePoolAuthorityIndex)
}
