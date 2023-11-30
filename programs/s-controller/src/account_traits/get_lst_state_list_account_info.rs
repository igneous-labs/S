use s_controller_interface::{StartRebalanceAccounts, SyncSolValueAccounts};
use solana_program::account_info::AccountInfo;

pub trait GetLstStateListAccountInfo<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}
