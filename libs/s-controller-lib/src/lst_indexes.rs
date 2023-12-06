#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SrcDstLstIndexes {
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SrcDstLstValueCalcAccs {
    pub src_lst_value_calc_accs: u8,
    pub dst_lst_value_calc_accs: u8,
}
