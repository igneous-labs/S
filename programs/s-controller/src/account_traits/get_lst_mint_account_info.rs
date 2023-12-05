use s_controller_interface::{
    AddLiquidityAccounts, EndRebalanceAccounts, RemoveLiquidityAccounts, SyncSolValueAccounts,
};
use solana_program::account_info::AccountInfo;

use super::{DstLstMintOf, GetSrcDstLstMintAccountInfo, SrcLstMintOf};

pub trait GetLstMintAccountInfo<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info, T: GetLstMintAccountInfo<'me, 'info>> GetLstMintAccountInfo<'me, 'info> for &T {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        (*self).get_lst_mint_account_info()
    }
}

impl<'me, 'info> GetLstMintAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_mint
    }
}

impl<'me, 'info> GetLstMintAccountInfo<'me, 'info> for EndRebalanceAccounts<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.dst_lst_mint
    }
}

impl<'me, 'info> GetLstMintAccountInfo<'me, 'info> for AddLiquidityAccounts<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_mint
    }
}

impl<'me, 'info> GetLstMintAccountInfo<'me, 'info> for RemoveLiquidityAccounts<'me, 'info> {
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_mint
    }
}

// SrcLstMintOf + DstLstMintOf

impl<'a, 'me, 'info, A> GetLstMintAccountInfo<'me, 'info> for SrcLstMintOf<'a, A>
where
    A: GetSrcDstLstMintAccountInfo<'me, 'info>,
{
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_src_lst_mint()
    }
}

impl<'a, 'me, 'info, A> GetLstMintAccountInfo<'me, 'info> for DstLstMintOf<'a, A>
where
    A: GetSrcDstLstMintAccountInfo<'me, 'info>,
{
    fn get_lst_mint_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_dst_lst_mint()
    }
}
