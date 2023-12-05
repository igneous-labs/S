use s_controller_interface::{
    AddLiquidityAccounts, EndRebalanceAccounts, RemoveLiquidityAccounts, SyncSolValueAccounts,
};
use solana_program::account_info::AccountInfo;

use super::{DstLstPoolReservesOf, GetSrcDstLstPoolReservesAccountInfo, SrcLstPoolReservesOf};

pub trait GetPoolReservesAccountInfo<'me, 'info> {
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info, T: GetPoolReservesAccountInfo<'me, 'info>> GetPoolReservesAccountInfo<'me, 'info>
    for &T
{
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        (*self).get_pool_reserves_account_info()
    }
}

impl<'me, 'info> GetPoolReservesAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_reserves
    }
}

impl<'me, 'info> GetPoolReservesAccountInfo<'me, 'info> for EndRebalanceAccounts<'me, 'info> {
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.dst_pool_reserves
    }
}

impl<'me, 'info> GetPoolReservesAccountInfo<'me, 'info> for AddLiquidityAccounts<'me, 'info> {
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_reserves
    }
}

impl<'me, 'info> GetPoolReservesAccountInfo<'me, 'info> for RemoveLiquidityAccounts<'me, 'info> {
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.pool_reserves
    }
}

// SrcLstPoolReservesOf + DstLstPoolReservesOf

impl<'a, 'me, 'info, A> GetPoolReservesAccountInfo<'me, 'info> for SrcLstPoolReservesOf<'a, A>
where
    A: GetSrcDstLstPoolReservesAccountInfo<'me, 'info>,
{
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_src_lst_pool_reserves()
    }
}

impl<'a, 'me, 'info, A> GetPoolReservesAccountInfo<'me, 'info> for DstLstPoolReservesOf<'a, A>
where
    A: GetSrcDstLstPoolReservesAccountInfo<'me, 'info>,
{
    fn get_pool_reserves_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_dst_lst_pool_reserves()
    }
}
