use s_controller_interface::{StartRebalanceAccounts, SwapExactInAccounts, SwapExactOutAccounts};
use solana_program::account_info::AccountInfo;

pub trait GetSrcDstLstMintAccountInfo<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info>;
    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info>;
}

/// For use with GetLstMintAccountInfo
#[derive(Debug, Clone, Copy)]
pub struct SrcLstMintOf<A>(pub A);

/// For use with GetLstMintAccountInfo
#[derive(Debug, Clone, Copy)]
pub struct DstLstMintOf<A>(pub A);

impl<'me, 'info, T: GetSrcDstLstMintAccountInfo<'me, 'info>> GetSrcDstLstMintAccountInfo<'me, 'info>
    for &T
{
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info> {
        (*self).get_src_lst_mint()
    }

    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info> {
        (*self).get_dst_lst_mint()
    }
}

impl<'me, 'info> GetSrcDstLstMintAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.src_lst_mint
    }

    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.dst_lst_mint
    }
}

impl<'me, 'info> GetSrcDstLstMintAccountInfo<'me, 'info> for SwapExactInAccounts<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.src_lst_mint
    }

    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.dst_lst_mint
    }
}

impl<'me, 'info> GetSrcDstLstMintAccountInfo<'me, 'info> for SwapExactOutAccounts<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.src_lst_mint
    }

    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.dst_lst_mint
    }
}

pub trait GetSrcDstLstPoolReservesAccountInfo<'me, 'info> {
    fn get_src_lst_pool_reserves(&self) -> &'me AccountInfo<'info>;
    fn get_dst_lst_pool_reserves(&self) -> &'me AccountInfo<'info>;
}

/// For use with GetPoolReservesAccountInfo
#[derive(Debug, Clone, Copy)]
pub struct SrcLstPoolReservesOf<A>(pub A);

/// For use with GetPoolReservesAccountInfo
#[derive(Debug, Clone, Copy)]
pub struct DstLstPoolReservesOf<A>(pub A);

impl<'me, 'info, T: GetSrcDstLstPoolReservesAccountInfo<'me, 'info>>
    GetSrcDstLstPoolReservesAccountInfo<'me, 'info> for &T
{
    fn get_src_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        (*self).get_src_lst_pool_reserves()
    }

    fn get_dst_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        (*self).get_dst_lst_pool_reserves()
    }
}

impl<'me, 'info> GetSrcDstLstPoolReservesAccountInfo<'me, 'info>
    for StartRebalanceAccounts<'me, 'info>
{
    fn get_src_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.src_pool_reserves
    }

    fn get_dst_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.dst_pool_reserves
    }
}

impl<'me, 'info> GetSrcDstLstPoolReservesAccountInfo<'me, 'info>
    for SwapExactInAccounts<'me, 'info>
{
    fn get_src_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.src_pool_reserves
    }

    fn get_dst_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.dst_pool_reserves
    }
}

impl<'me, 'info> GetSrcDstLstPoolReservesAccountInfo<'me, 'info>
    for SwapExactOutAccounts<'me, 'info>
{
    fn get_src_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.src_pool_reserves
    }

    fn get_dst_lst_pool_reserves(&self) -> &'me AccountInfo<'info> {
        self.dst_pool_reserves
    }
}
