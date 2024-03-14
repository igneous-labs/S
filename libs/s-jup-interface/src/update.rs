use std::collections::HashMap;

use s_controller_interface::LstState;
use s_controller_lib::{try_lst_state_list, try_pool_state};
use s_pricing_prog_aggregate::MutablePricingProg;
use s_sol_val_calc_prog_aggregate::{LstSolValCalc, MutableLstSolValCalc};
use sanctum_token_lib::{mint_supply, token_account_balance};
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::pubkey::Pubkey;

use crate::{utils::try_pricing_prog, LstData, SPool};

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> SPool<S, L> {
    pub fn get_accounts_to_update_full(&self) -> Vec<Pubkey> {
        let mut res: Vec<Pubkey> = self
            .get_accounts_to_update_base()
            .into_iter()
            .chain(self.get_accounts_to_update_pricing_prog())
            .chain(self.get_accounts_to_update_lsts_all())
            .collect();
        if let Ok(lp_token_mint) = self.lp_token_mint() {
            res.push(lp_token_mint)
        }
        res
    }
}

impl<D: ReadonlyAccountData + Clone> SPool<D, D> {
    pub fn update_full(&mut self, account_map: &HashMap<Pubkey, D>) -> anyhow::Result<()> {
        // returns the first encountered error, but tries to update everything eagerly
        // even after encountering an error

        // first, update lst_data_list and pricing_program
        //
        // then, update pool_state and lst_state_list so we can invalidate
        // pricing_prog and lst_sol_val_calcs if any of them changed
        //  - update lst_state_list before pool_state so we can use the new lst_state_list to reinitialize pricing program if required
        //
        // finally, update LP token supply using the newest pool state
        self.update_lst_data_list(account_map)
            .and(self.update_pricing_prog(account_map))
            .and(self.update_lst_state_list(account_map))
            .and(self.update_pool_state(account_map))
            .and(self.update_lp_token_supply(account_map))
    }
}

impl<S, L> SPool<S, L> {
    pub fn get_accounts_to_update_base(&self) -> [Pubkey; 2] {
        [self.lst_state_list_addr, self.pool_state_addr]
    }

    pub fn get_accounts_to_update_pricing_prog(&self) -> Vec<Pubkey> {
        self.pricing_prog
            .as_ref()
            .map_or_else(Vec::new, |pp| pp.get_accounts_to_update())
    }

    pub fn get_accounts_to_update_pricing_prog_for_lsts<I: Iterator<Item = Pubkey>>(
        &self,
        lst_mints: I,
    ) -> Vec<Pubkey> {
        self.pricing_prog
            .as_ref()
            .map_or_else(Vec::new, |pp| pp.get_accounts_to_update_for_lsts(lst_mints))
    }

    pub fn get_accounts_to_update_pricing_prog_for_liquidity(&self) -> Vec<Pubkey> {
        self.pricing_prog
            .as_ref()
            .map_or_else(Vec::new, |pp| pp.get_accounts_to_update_for_liquidity())
    }

    pub fn update_pricing_prog<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        if let Some(pp) = self.pricing_prog.as_mut() {
            pp.update(account_map)?;
        }
        Ok(())
    }
}

impl<S, L: ReadonlyAccountData> SPool<S, L> {
    fn lst_accounts_to_update(
        &self,
        lst_state: &LstState,
        lst_data: &Option<LstData>,
    ) -> Vec<Pubkey> {
        let lst_data = match lst_data.as_ref() {
            Some(l) => l,
            None => return vec![],
        };
        let mut res = lst_data.sol_val_calc.get_accounts_to_update();
        if let Ok(ata) = self.pool_reserves_account(lst_state, lst_data) {
            res.push(ata);
        }
        res
    }

    pub fn get_accounts_to_update_lsts_all(&self) -> Vec<Pubkey> {
        let lst_state_list_data = self.lst_state_list_account.data();
        let lst_state_list = match try_lst_state_list(&lst_state_list_data) {
            Ok(l) => l,
            Err(_) => return vec![],
        };
        lst_state_list
            .iter()
            .zip(self.lst_data_list.iter())
            .flat_map(|(lst_state, lst_data)| self.lst_accounts_to_update(lst_state, lst_data))
            .collect()
    }

    /// Used to only fetch certain accounts for partial updates for specific LSTs
    pub fn get_accounts_to_update_lsts_filtered<F: FnMut(&LstState, &Option<LstData>) -> bool>(
        &self,
        mut filter_pred: F,
    ) -> Vec<Pubkey> {
        let lst_state_list_data = self.lst_state_list_account.data();
        let lst_state_list = match try_lst_state_list(&lst_state_list_data) {
            Ok(l) => l,
            Err(_) => return vec![],
        };
        lst_state_list
            .iter()
            .zip(self.lst_data_list.iter())
            .filter_map(|(lst_state, lst_data)| {
                if filter_pred(lst_state, lst_data) {
                    Some(self.lst_accounts_to_update(lst_state, lst_data))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    pub fn update_lst_data_list<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        // use raw indices to avoid lifetime errs from borrowing immut field (self.lst_state_list)
        // while borrowing mut field (self.lst_data_list)
        #[allow(clippy::manual_try_fold)] // we dont want to short-circuit, so dont try_fold()
        (0..self.lst_data_list.len())
            .map(|i| {
                let ld = match &self.lst_data_list[i] {
                    Some(l) => l,
                    None => return Ok(()),
                };
                let lst_state_list_acc_data = self.lst_state_list_account.data();
                let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
                let ata_res = self.pool_reserves_account(&lst_state_list[i], ld);
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
            .fold(Ok(()), |res, curr_res| res.and(curr_res))
    }
}
impl<S, L: ReadonlyAccountData + Clone> SPool<S, L> {
    pub fn update_lst_state_list(
        &mut self,
        account_map: &HashMap<Pubkey, L>,
    ) -> anyhow::Result<()> {
        let new_lst_state_list_account = match account_map.get(&self.lst_state_list_addr) {
            Some(acc) => acc.clone(),
            None => return Ok(()),
        };
        // simple model for diffs:
        // - if new and old list differs in mints, then try to find the mismatches and replace them
        // - if sol val calc program changed, then just invalidate to None. Otherwise we would need a
        //   SanctumLstList to reinitialize the KnownLstSolValCalc
        // - if list was extended, the new entries will just be None and we cant handle it. Otherwise we would need a
        //   SanctumLstList to initialize the KnownLstSolValCalc
        let lst_state_list_acc_data = self.lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
        let new_lst_state_list_account_data = new_lst_state_list_account.data();
        let new_lst_state_list = try_lst_state_list(&new_lst_state_list_account_data)?;
        if lst_state_list.len() == new_lst_state_list.len()
            && lst_state_list.iter().zip(new_lst_state_list.iter()).all(
                |(old_lst_state, new_lst_state)| {
                    old_lst_state.mint == new_lst_state.mint
                        && old_lst_state.sol_value_calculator == new_lst_state.sol_value_calculator
                },
            )
        {
            drop(lst_state_list_acc_data);
            drop(new_lst_state_list_account_data);
            self.lst_state_list_account = new_lst_state_list_account;
            return Ok(());
        }
        // Either at least 1 sol value calculator changed or mint changed:
        // rebuild entire lst_data vec by cloning from old vec
        let mut new_lst_data_list = vec![None; new_lst_state_list.len()];
        lst_state_list
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
        drop(lst_state_list_acc_data);
        drop(new_lst_state_list_account_data);
        self.lst_state_list_account = new_lst_state_list_account;
        Ok(())
    }
}

impl<S: ReadonlyAccountData, L> SPool<S, L> {
    pub fn update_lp_token_supply<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        let supply = {
            let pool_state_data = match self.pool_state_data() {
                Ok(p) => p,
                Err(_e) => return Ok(()),
            };
            let pool_state = try_pool_state(&pool_state_data)?;
            let lp_token_mint_acc = match account_map.get(&pool_state.lp_token_mint) {
                Some(l) => l,
                None => return Ok(()),
            };
            mint_supply(lp_token_mint_acc)?
        };
        self.lp_mint_supply = Some(supply);
        Ok(())
    }
}

impl<S: ReadonlyAccountData + Clone, L: ReadonlyAccountData> SPool<S, L> {
    pub fn update_pool_state(&mut self, account_map: &HashMap<Pubkey, S>) -> anyhow::Result<()> {
        let new_pool_state_acc = match account_map.get(&self.pool_state_addr) {
            Some(a) => a,
            None => return Ok(()),
        };
        let old_pool_state = self
            .pool_state_data()
            .map_or_else(Err, |d| Ok(*try_pool_state(&d)?));
        let lst_state_list_acc_data = self.lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
        try_pool_state(&new_pool_state_acc.data()).map_or_else(
            |e| Err(e.into()),
            |new_pool_state| {
                let mut r = Ok(());
                // reinitialize pricing program if changed
                let should_reinitialize_pricing_program = self.pricing_prog.is_none()
                    || old_pool_state.map_or_else(
                        |_err| false,
                        |old_ps| old_ps.pricing_program != new_pool_state.pricing_program,
                    );
                if should_reinitialize_pricing_program {
                    // None if unable to initialize new_pricing_prog, with error captured
                    // for return later
                    let new_pricing_prog = try_pricing_prog(new_pool_state, lst_state_list)
                        .map(|mut pp| {
                            r = pp.update(account_map);
                            pp
                        })
                        .ok();
                    self.pricing_prog = new_pricing_prog;
                }
                self.pool_state_account = Some(new_pool_state_acc.clone());
                r
            },
        )
    }
}
