use s_controller_interface::{
    AddLiquidityAccounts, EndRebalanceAccounts, RemoveLiquidityAccounts,
    SetSolValueCalculatorAccounts, StartRebalanceAccounts, SwapExactInAccounts,
    SwapExactOutAccounts, SyncSolValueAccounts,
};
use solana_program::account_info::AccountInfo;

use super::{DstLstPoolReservesOf, SrcLstPoolReservesOf};

pub trait GetPoolStateAccountInfo<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info, T: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for &T
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        (*self).get_pool_state_account_info()
    }
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

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for EndRebalanceAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for AddLiquidityAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for RemoveLiquidityAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for SwapExactInAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for SwapExactOutAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

impl<'me, 'info> GetPoolStateAccountInfo<'me, 'info> for SetSolValueCalculatorAccounts<'me, 'info> {
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_state
    }
}

// impls for src_dst wrapper newtypes

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for SrcLstPoolReservesOf<A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}

impl<'me, 'info, A: GetPoolStateAccountInfo<'me, 'info>> GetPoolStateAccountInfo<'me, 'info>
    for DstLstPoolReservesOf<A>
{
    fn get_pool_state_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_pool_state_account_info()
    }
}
