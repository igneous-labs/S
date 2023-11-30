use s_controller_interface::StartRebalanceAccounts;
use solana_program::account_info::AccountInfo;

pub trait GetSrcDstLstMintAccountInfo<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info>;
    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info>;
}

/// For use with GetLstMintAccountInfo
pub struct SrcLstMintOf<'a, A>(pub &'a A);

/// For use with GetLstMintAccountInfo
pub struct DstLstMintOf<'a, A>(pub &'a A);

impl<'me, 'info> GetSrcDstLstMintAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_src_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.src_lst_mint
    }

    fn get_dst_lst_mint(&self) -> &'me AccountInfo<'info> {
        self.dst_lst_mint
    }
}
