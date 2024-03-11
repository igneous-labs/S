use anyhow::anyhow;
use jupiter_amm_interface::{
    AccountMap, Amm, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapParams,
};
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, try_lst_state_list, try_pool_state,
};
use s_pricing_prog_aggregate::{KnownPricingProg, MutablePricingProg};
use s_sol_val_calc_prog_aggregate::{
    KnownLstSolValCalc, LidoLstSolValCalc, LstSolValCalc, MarinadeLstSolValCalc,
    MutableLstSolValCalc, SanctumSplLstSolValCalc, SplLstSolValCalc, SplLstSolValCalcInitKeys,
    WsolLstSolValCalc,
};
use sanctum_associated_token_lib::{CreateAtaAddressArgs, FindAtaAddressArgs};
use sanctum_lst_list::{PoolInfo, SanctumLst, SanctumLstList, SplPoolAccounts};
use sanctum_token_lib::{mint_supply, token_account_balance};
use solana_program::pubkey::{Pubkey, PubkeyError};
use std::str::FromStr;

pub const LABEL: &str = "Sanctum Infinity";

#[derive(Debug, Clone)]
pub struct LstData {
    pub sol_val_calc: KnownLstSolValCalc,
    pub reserves_balance: Option<u64>,
    pub token_program: Pubkey,
}

#[derive(Debug, Clone)]
pub struct SPool {
    pub program_id: Pubkey,
    pub lst_state_list_addr: Pubkey,
    pub pool_state_addr: Pubkey,
    pub lp_mint_supply: Option<u64>,
    pub pool_state: Option<PoolState>,
    pub pricing_prog: Option<KnownPricingProg>,
    pub lst_state_list: Vec<LstState>,
    // indices match that of lst_state_list.
    // None means we don't know how to handle the given lst
    // this could be due to incomplete data or unknown LST sol value calculator program
    pub lst_data_list: Vec<Option<LstData>>,
}

impl Default for SPool {
    fn default() -> Self {
        Self {
            program_id: s_controller_lib::program::ID,
            lst_state_list_addr: s_controller_lib::program::LST_STATE_LIST_ID,
            pool_state_addr: s_controller_lib::program::POOL_STATE_ID,
            lp_mint_supply: None,
            pool_state: None,
            pricing_prog: None,
            lst_state_list: Vec::new(),
            lst_data_list: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SPoolInitAccounts {
    pub lst_state_list: Pubkey,
    pub pool_state: Pubkey,
}

impl From<SPoolInitAccounts> for [Pubkey; 2] {
    fn from(
        SPoolInitAccounts {
            lst_state_list,
            pool_state,
        }: SPoolInitAccounts,
    ) -> Self {
        [lst_state_list, pool_state]
    }
}

impl Default for SPoolInitAccounts {
    fn default() -> Self {
        Self {
            lst_state_list: s_controller_lib::program::LST_STATE_LIST_ID,
            pool_state: s_controller_lib::program::POOL_STATE_ID,
        }
    }
}

impl SPool {
    /// Gets the list of accounts that must be fetched first to initialize
    /// SPool by passing the result into [`Self::from_fetched_accounts`]
    pub fn init_accounts(program_id: Pubkey) -> SPoolInitAccounts {
        SPoolInitAccounts {
            lst_state_list: find_lst_state_list_address(program_id).0,
            pool_state: find_pool_state_address(program_id).0,
        }
    }

    pub fn from_lst_state_list(
        program_id: Pubkey,
        lst_state_list: Vec<LstState>,
        lst_list: &[SanctumLst],
    ) -> Self {
        let SPoolInitAccounts {
            lst_state_list: lst_state_list_addr,
            pool_state: pool_state_addr,
        } = Self::init_accounts(program_id);
        let lst_data_list = lst_state_list
            .iter()
            .map(|lst_state| try_lst_data(lst_list, lst_state))
            .collect();
        Self {
            program_id,
            lst_state_list_addr,
            pool_state_addr,
            pool_state: None,
            pricing_prog: None,
            lp_mint_supply: None,
            lst_state_list,
            lst_data_list,
        }
    }

    pub fn from_fetched_accounts(
        program_id: Pubkey,
        accounts: &AccountMap,
        lst_list: &[SanctumLst],
    ) -> anyhow::Result<Self> {
        let SPoolInitAccounts {
            lst_state_list: lst_state_list_addr,
            pool_state: pool_state_addr,
        } = Self::init_accounts(program_id);

        let lst_state_list_acc = accounts
            .get(&lst_state_list_addr)
            .ok_or_else(|| anyhow!("Missing LST state list {lst_state_list_addr}"))?;
        let lst_state_list = Vec::from(try_lst_state_list(&lst_state_list_acc.data)?);
        let pool_state_acc = accounts
            .get(&pool_state_addr)
            .ok_or_else(|| anyhow!("Missing pool state {pool_state_addr}"))?;
        let pool_state = try_pool_state(&pool_state_acc.data)?;
        let pricing_prog = try_pricing_prog(pool_state, &lst_state_list)?;

        let mut res = Self::from_lst_state_list(program_id, lst_state_list, lst_list);
        res.pool_state = Some(*pool_state);
        res.pricing_prog = Some(pricing_prog);
        Ok(res)
    }

    fn update_lst_state_list(&mut self, new_lst_state_list: Vec<LstState>) {
        // simple model for diffs:
        // - if new and old list differs in mints, then try to find the mismatches and replace them
        // - if sol val calc program changed, then just invalidate to None. Otherwise we would need a
        //   SanctumLstList to reinitialize the KnownLstSolValCalc
        // - if list was extended, the new entries will just be None and we cant handle it. Otherwise we would need a
        //   SanctumLstList to reinitialize the KnownLstSolValCalc
        if self.lst_state_list.len() == new_lst_state_list.len()
            && self
                .lst_state_list
                .iter()
                .zip(new_lst_state_list.iter())
                .all(|(old_lst_state, new_lst_state)| {
                    old_lst_state.mint == new_lst_state.mint
                        && old_lst_state.sol_value_calculator == new_lst_state.sol_value_calculator
                })
        {
            self.lst_state_list = new_lst_state_list;
            return;
        }
        // Either at least 1 sol value calculator changed or mint changed:
        // rebuild entire sol val calcs and lst_reserves_balance vec by cloning from old vec
        let mut new_lst_data_list = vec![None; new_lst_state_list.len()];
        self.lst_state_list
            .iter()
            .zip(self.lst_data_list.iter())
            .zip(new_lst_state_list.iter())
            .zip(new_lst_data_list.iter_mut())
            .for_each(
                |(((old_lst_state, old_lst_data), new_lst_state), new_lst_data)| {
                    let replacement = if old_lst_state.mint != new_lst_state.mint {
                        self.lst_data_list
                            .iter()
                            .find(|opt| match opt {
                                Some(ld) => {
                                    ld.sol_val_calc.lst_mint() == new_lst_state.mint
                                        && ld.sol_val_calc.sol_value_calculator_program_id()
                                            == new_lst_state.sol_value_calculator
                                }
                                None => false,
                            })
                            .cloned()
                            .flatten()
                    } else {
                        old_lst_data
                            .as_ref()
                            .map_or_else(
                                || None,
                                |ld| {
                                    if ld.sol_val_calc.sol_value_calculator_program_id()
                                        == new_lst_state.sol_value_calculator
                                    {
                                        Some(ld)
                                    } else {
                                        None
                                    }
                                },
                            )
                            .cloned()
                    };
                    *new_lst_data = replacement;
                },
            );
        self.lst_data_list = new_lst_data_list;
        self.lst_state_list = new_lst_state_list;
    }

    pub fn pool_reserves_account(
        &self,
        LstState {
            mint,
            pool_reserves_bump,
            ..
        }: &LstState,
        LstData { token_program, .. }: &LstData,
    ) -> Result<Pubkey, PubkeyError> {
        CreateAtaAddressArgs {
            find_ata_args: FindAtaAddressArgs {
                wallet: self.pool_state_addr,
                mint: *mint,
                token_program: *token_program,
            },
            bump: *pool_reserves_bump,
        }
        .create_ata_address()
    }

    pub fn quote_swap_exact_in(&self, QuoteParams { .. }: &QuoteParams) -> anyhow::Result<Quote> {
        todo!()
    }
}

impl Amm for SPool {
    /// Initialized by lst_state_list account, NOT pool_state.
    ///
    /// Params can optionally be a b58-encoded pubkey string that is the S controller program's program_id
    fn from_keyed_account(
        KeyedAccount {
            key,
            account,
            params,
        }: &KeyedAccount,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let (program_id, lst_state_list_addr) = match params {
            // default to INF if program-id params not provided
            None => (
                s_controller_lib::program::ID,
                s_controller_lib::program::LST_STATE_LIST_ID,
            ),
            Some(value) => {
                // TODO: maybe unnecessary clone() here?
                let program_id =
                    Pubkey::from_str(&serde_json::from_value::<String>(value.clone())?)?;
                (program_id, find_lst_state_list_address(program_id).0)
            }
        };
        if *key != lst_state_list_addr {
            return Err(anyhow!(
                "Incorrect LST state list addr. Expected {lst_state_list_addr}. Got {key}"
            ));
        }
        let lst_state_list = Vec::from(try_lst_state_list(&account.data)?);
        let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
        Ok(Self::from_lst_state_list(
            program_id,
            lst_state_list,
            &sanctum_lst_list,
        ))
    }

    fn label(&self) -> String {
        LABEL.into()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    /// S Pools are 1 per program
    fn key(&self) -> Pubkey {
        self.program_id()
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        let mut res: Vec<Pubkey> = self
            .lst_state_list
            .iter()
            .map(|LstState { mint, .. }| *mint)
            .collect();
        if let Some(pool_state) = self.pool_state {
            res.push(pool_state.lp_token_mint);
        }
        res
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut res = vec![self.lst_state_list_addr, self.pool_state_addr];
        if let Some(pricing_prog) = &self.pricing_prog {
            res.extend(pricing_prog.get_accounts_to_update());
        }
        if let Some(pool_state) = &self.pool_state {
            res.push(pool_state.lp_token_mint);
        }
        res.extend(
            self.lst_state_list
                .iter()
                .zip(self.lst_data_list.iter())
                .filter_map(|(lst_state, lst_data)| {
                    let lst_data = lst_data.as_ref()?;
                    let mut res = lst_data.sol_val_calc.get_accounts_to_update();
                    if let Ok(ata) = self.pool_reserves_account(lst_state, lst_data) {
                        res.push(ata);
                    }
                    Some(res)
                })
                .flatten(),
        );
        res
    }

    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        // returns the first encountered error, but tries to update everything eagerly
        // even after encountering an error

        // use raw indices to avoid lifetime errs from borrowing immut field (self.lst_state_list)
        // while borrowing mut field (self.lst_data_list)
        #[allow(clippy::manual_try_fold)] // we dont want to short-circuit, so dont try_fold()
        let mut res = (0..self.lst_data_list.len())
            .map(|i| {
                let ld = match &self.lst_data_list[i] {
                    Some(l) => l,
                    None => return Ok(()),
                };
                let ata_res = self.pool_reserves_account(&self.lst_state_list[i], ld);
                let ld = match &mut self.lst_data_list[i] {
                    Some(l) => l,
                    None => return Ok(()),
                };
                let r = ld.sol_val_calc.update(account_map);
                r.and(ata_res.map_or_else(
                    |e| Err(e.into()),
                    |ata| {
                        if let Some(fetched) = account_map.get(&ata) {
                            ld.reserves_balance = Some(token_account_balance(fetched)?);
                        }
                        Ok(())
                    },
                ))
            })
            .fold(Ok(()), |res, curr_res| res.and(curr_res));

        if let Some(pp) = self.pricing_prog.as_mut() {
            res = res.and(pp.update(account_map));
        }

        // update pool state and lst_state_list last so we can invalidate
        // pricing_prog and lst_sol_val_calcs if any of them changed

        // update lst_state_list first so we can use the new lst_state_list to reset pricing program
        if let Some(lst_state_list_acc) = account_map.get(&self.lst_state_list_addr) {
            res = res.and(try_lst_state_list(&lst_state_list_acc.data).map_or_else(
                |e| Err(e.into()),
                |lst_state_list| {
                    self.update_lst_state_list(Vec::from(lst_state_list));
                    Ok(())
                },
            ));
        }

        if let Some(pool_state_acc) = account_map.get(&self.pool_state_addr) {
            res = res.and(try_pool_state(&pool_state_acc.data).map_or_else(
                |e| Err(e.into()),
                |ps| {
                    let mut r = Ok(());
                    // reinitialize pricing program if changed
                    if let Some(old_ps) = self.pool_state {
                        if old_ps.pricing_program != ps.pricing_program {
                            let new_pricing_prog = try_pricing_prog(ps, &self.lst_state_list)
                                .map(|mut pp| {
                                    r = pp.update(account_map);
                                    pp
                                })
                                .ok();
                            self.pricing_prog = new_pricing_prog;
                        }
                    }
                    self.pool_state = Some(*ps);
                    r
                },
            ));
        }

        // finally, update LP token supply after pool state has been updated
        if let Some(pool_state) = self.pool_state {
            if let Some(lp_token_mint_acc) = account_map.get(&pool_state.lp_token_mint) {
                match mint_supply(lp_token_mint_acc) {
                    Ok(supply) => self.lp_mint_supply = Some(supply),
                    Err(e) => res = res.and(Err(e.into())),
                }
            }
        }

        res
    }

    fn quote(&self, _quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        todo!()
    }

    fn get_swap_and_account_metas(
        &self,
        _swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        todo!()
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn has_dynamic_accounts(&self) -> bool {
        true
    }

    /// TODO: this is not true for AddLiquidity and RemoveLiquidity
    fn supports_exact_out(&self) -> bool {
        true
    }
}

fn try_pricing_prog(
    pool_state: &PoolState,
    lst_state_list: &[LstState],
) -> anyhow::Result<KnownPricingProg> {
    Ok(KnownPricingProg::try_new(
        pool_state.pricing_program,
        lst_state_list.iter().map(|LstState { mint, .. }| *mint),
    )?)
}

fn try_lst_data(
    lst_list: &[SanctumLst],
    LstState {
        mint,
        sol_value_calculator,
        ..
    }: &LstState,
) -> Option<LstData> {
    let SanctumLst {
        pool,
        token_program,
        ..
    } = lst_list.iter().find(|s| s.mint == *mint)?;
    let calc = match pool {
        PoolInfo::Lido => KnownLstSolValCalc::Lido(LidoLstSolValCalc::default()),
        PoolInfo::Marinade => KnownLstSolValCalc::Marinade(MarinadeLstSolValCalc::default()),
        PoolInfo::ReservePool => KnownLstSolValCalc::Wsol(WsolLstSolValCalc),
        PoolInfo::SanctumSpl(SplPoolAccounts { pool, .. }) => KnownLstSolValCalc::SanctumSpl(
            SanctumSplLstSolValCalc::from_keys(SplLstSolValCalcInitKeys {
                lst_mint: *mint,
                stake_pool_addr: *pool,
            }),
        ),
        PoolInfo::Spl(SplPoolAccounts { pool, .. }) => {
            KnownLstSolValCalc::Spl(SplLstSolValCalc::from_keys(SplLstSolValCalcInitKeys {
                lst_mint: *mint,
                stake_pool_addr: *pool,
            }))
        }
        PoolInfo::SPool(_) => None?,
    };
    if *sol_value_calculator != calc.sol_value_calculator_program_id() {
        None
    } else {
        Some(LstData {
            sol_val_calc: calc,
            reserves_balance: None,
            token_program: *token_program,
        })
    }
}
