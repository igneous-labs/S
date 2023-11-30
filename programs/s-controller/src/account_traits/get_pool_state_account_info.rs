use s_controller_interface::{StartRebalanceAccounts, SyncSolValueAccounts};
use solana_program::account_info::AccountInfo;

pub trait GetPoolStateAccountInfo<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}
