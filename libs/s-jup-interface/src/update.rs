use jupiter_amm_interface::AccountMap;
use s_controller_lib::{try_lst_state_list, try_pool_state};
use s_pricing_prog_aggregate::MutablePricingProg;
use s_sol_val_calc_prog_aggregate::{LstSolValCalc, MutableLstSolValCalc};
use sanctum_token_lib::{mint_supply, token_account_balance};

use crate::{utils::try_pricing_prog, SPoolJup};

impl SPoolJup {
    pub(crate) fn update_pricing_prog(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        if let Some(pp) = self.pricing_prog.as_mut() {
            pp.update(account_map)?;
        }
        Ok(())
    }

    pub(crate) fn update_lst_data_list(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        // use raw indices to avoid lifetime errs from borrowing immut field (self.lst_state_list)
        // while borrowing mut field (self.lst_data_list)
        #[allow(clippy::manual_try_fold)] // we dont want to short-circuit, so dont try_fold()
        (0..self.lst_data_list.len())
            .map(|i| {
                let ld = match &self.lst_data_list[i] {
                    Some(l) => l,
                    None => return Ok(()),
                };
                let ata_res = self.pool_reserves_account(&self.lst_state_list()?[i], ld);
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

    pub(crate) fn update_lst_state_list(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
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
        let lst_state_list = self.lst_state_list()?;
        let new_lst_state_list = try_lst_state_list(&new_lst_state_list_account.data)?;
        if lst_state_list.len() == new_lst_state_list.len()
            && lst_state_list.iter().zip(new_lst_state_list.iter()).all(
                |(old_lst_state, new_lst_state)| {
                    old_lst_state.mint == new_lst_state.mint
                        && old_lst_state.sol_value_calculator == new_lst_state.sol_value_calculator
                },
            )
        {
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
        self.lst_state_list_account = new_lst_state_list_account;
        Ok(())
    }

    pub(crate) fn update_pool_state(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        let pool_state_acc = match account_map.get(&self.pool_state_addr) {
            Some(a) => a,
            None => return Ok(()),
        };
        try_pool_state(&pool_state_acc.data).map_or_else(
            |e| Err(e.into()),
            |ps| {
                let lst_state_list = self.lst_state_list()?;
                let mut r = Ok(());
                // reinitialize pricing program if changed
                if let Ok(old_ps) = self.pool_state() {
                    if old_ps.pricing_program != ps.pricing_program {
                        let new_pricing_prog = try_pricing_prog(ps, lst_state_list)
                            .map(|mut pp| {
                                r = pp.update(account_map);
                                pp
                            })
                            .ok();
                        self.pricing_prog = new_pricing_prog;
                    }
                }
                self.pool_state_account = Some(pool_state_acc.clone());
                r
            },
        )
    }

    pub(crate) fn update_lp_token_supply(
        &mut self,
        account_map: &AccountMap,
    ) -> anyhow::Result<()> {
        let pool_state = match self.pool_state() {
            Ok(p) => p,
            Err(_e) => return Ok(()),
        };
        let lp_token_mint_acc = match account_map.get(&pool_state.lp_token_mint) {
            Some(l) => l,
            None => return Ok(()),
        };
        let supply = mint_supply(lp_token_mint_acc)?;
        self.lp_mint_supply = Some(supply);
        Ok(())
    }
}
