# Instructions

Controller program's instructions.

For more information about the pricing programs CPIs, see [interface doc](../docs/pricing-programs/interface.md)

For more information about the SOL value calculator programs CPIs, see [interface doc](../docs/sol-value-calculator-programs/interface.md)

## Controller Program

### SyncSolValue

Permissionless crank to update and record the SOL value of one of the pool's LST reserves.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| lst_index | index of the lst in pool_state.lst_states | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | mint of the LST to sync SOL value for | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_reserves | LST token account reserves of the pool | R | N |
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Verify index
- CPI the LST's SOL value calculator program LstToSol
- Update pool_state's sol_value by subtracting LST's old SOL value and adding newly returned SOL value 
- Record returned SOL value in pool_state

### SwapExactIn

Swap to output LST from an exact amount of given input LST.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| src_lst_value_calc_accs | number of accounts following dst_lst_acc to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself | u8 |
| dst_lst_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceExactIn with, including the program itself | u8 |
| src_lst_index | index of src_lst in pool_state.lst_states | u64 |
| dst_lst_index | index of dst_lst in pool_state.lst_states | u64 |
| amount | amount of src tokens to swap | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of src_lst_acc. User making the swap. | R | Y |
| src_lst | mint of the LST being swapped from | R | N |
| dst_lst | mint of the LST being swapped to | R | N |
| src_lst_acc | LST token account being swapped from | W | N |
| dst_lst_acc | LST token account to swap to | W | N |
| protocol_fee_accumulator | protocol fee accumulator token account | W | N |
| src_lst_token_program | - | R | N |
| dst_lst_token_program | - | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_src_reserves | src token token account reserves of the pool | W | N |
| pool_dst_reserves | dst token token account reserves of the pool | W | N |
| src_lst_value_calc_accs | accounts to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| dst_lst_value_calc_accs | accounts to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceExactIn with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Verify input not disabled for src_lst
- Self CPI SyncSolValue for src_lst
- Self CPI SyncSolValue for dst_lst
- CPI src token's SOL value calculator program LstToSol to get SOL value of input amount
- CPI pricing program PriceExactIn to get output SOL value
- CPI dst token's SOL value calculator program SolToLst with output SOL value to get output token amount
- Transfer input amount src tokens from src_lst_acc to src token reserves
- Subtract and transfer protocol fees to protocol_fee_dest
- Transfer remaining output dst tokens from dst token reserves to dst_lst_acc
- Self CPI SyncSolValue for src_lst
- Self CPI SyncSolValue for dst_lst

### SwapExactOut

Swap to an exact amount of output LST from input LST.

Same as [SwapExactIn](#swapexactin-instruction), but discriminator = 2, amount is amount of dst tokens to receive and the core part goes like this instead:
- CPI dst token's SOL value calculator program LstToSol to get SOL value of output amount
- CPI pricing program PriceExactOut to get input SOL value
- CPI src token's SOL value calculator program SolToLst with input SOL value to get input token amount

### AddLiquidity

Add single-LST liquidity to the pool.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 3 | u8 |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself | u8 |
| lst_index | index of lst in pool_state.lst_states | u64 |
| amount | amount of tokens to add as liquidity | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of lst_acc. User who's adding liquidity. | R | Y |
| lst | LST token mint | R | N |
| src_lst_acc | LST token account to add liquidity from | W | N |
| dst_lp_token_acc | LP token account to mint new LP tokens to | W | N |
| token_program | - | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_reserves | pool's token reserves for the LST | W | N |
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceLpTokensToMint with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Verify input not disabled for LST
- Self CPI SyncSolValue for LST
- CPI LST's SOL value calculator program LstToSol to get SOL value of amount to add
- CPI pricing program's PriceLpTokensToMint to get SOL value of LP tokens to mint
- Transfer amount from src_lst_acc to pool_reserves
- Mint LP tokens proportional to SOL value of LP tokens to mint to dst_lp_token_acc
- Self CPI SyncSolValue for LST

### RemoveLiquidity

Remove single-LST liquidity from the pool.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 4 | u8 |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself | u8 |
| lst_index | index of lst in pool_state.lst_states | u64 |
| amount | amount of LP tokens to burn and redeem | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of lp_acc. User who's adding liquidity. | R | Y |
| lst | LST token mint | R | N |
| dst_lst_acc | LST token account to redeem to | W | N |
| src_lp_token_acc | LP token account to burn LP tokens from | W | N |
| protocol_fee_accumulator | protocol fee accumulator token account | W | N |
| token_program | - | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_reserves | pool's token reserves for the LST | W | N |
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceLpTokensToRedeem with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Self CPI SyncSolValue for LST
- CPI pricing program's PriceLpTokensToRedeem with SOL value of LP tokens to be burnt
- CPI LST's SOL value calculator program SolToLst to get amount of LST due
- Burn LP tokens
- Subtract and transfer protocol fees
- Transfer remaining LST due to dst_acc
- Self CPI SyncSolValue for LST

### CallPricingProgram

CPI the pricing program.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 5 | u8 |
| pricing program args | raw bytes passed directly to pricing program CPI | ... |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_accs | accounts to invoke pricing program with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- CPI pricing program with the accounts and data passed in, signed by the pricing program authority PDA

### DisableInput

Disable input for a LST to prepare for removal

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 6 | u8 |
| index | index of lst in pool_state.lst_states | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| lst | The LST to disable input for | R | N |
| pool_state | The pool's state singleton | W | N |

### EnableInput

Re-enable input for a LST

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 7 | u8 |
| index | index of lst in pool_state.lst_states | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| lst | The LST to re-enable input for | R | N |
| pool_state | The pool's state singleton | W | N |

### AddLst

Add a LST to the pool

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 8 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| payer | Account paying the SOL rent for the new space and accounts | W | Y | 
| lst | The new LST to add | R | N |
| reserves | The LST reserves token account to create | W | N |
| protocol_fee_accumulator | The LST protocol fee accumulator token account to create | W | N |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W | N |
| sol_value_calculator | The LST's SOL value calculator program | R | N |
| pool_state | The pool's state singleton | W | N |

#### Procedure

- Create reserves token account
- Create protocol_fee_accumulator token account
- Reallocate additional space for an additional LstState on pool_state.lst_states
- Write initial SOL value = 0 and sol_value_calculator program

### RemoveLst

Remove a LST from the pool

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 9 | u8 |
| index | index of lst in pool_state.lst_states | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| refund_rent_to | Account to refund SOL rent to | W | N | 
| lst | The LST to remove | R | N |
| reserves | The LST reserves token account to destroy | W | N |
| protocol_fee_accumulator | The LST protocol fee accumulator token account to destroy | W | N |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W | N |
| pool_state | The pool's state singleton | W | N |

#### Procedure

- Delete reserves token account
- Delete protocol_fee_accumulator token account
- Remove LstState from list and reallocate to smaller space

### SetSolValueCalculator

Update the SOL value calculator program for a LST

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 10 | u8 |
| index | index of lst in pool_state.lst_states | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| lst | The LST to remove | R | N |
| pool_state | The pool's state singleton | W | N |
| pool_reserves | LST token account reserves of the pool | R | N |
| lst_value_calc_accs | accounts to invoke token's new SOL value calculator program LstToSol with. First account should be the new calculator program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Overwrite sol_value_calculator in pool_state.lst_states
- Self CPI SyncSolValue

### SetAdmin

Updates the admin authority pubkey of the pool

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 11 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| old_admin | The pool's old admin | R | Y |
| new_admin | The pool's new admin | R | N |
| pool_state | The pool's state singleton | W | N |

### SetProtocolFee

Updates the protocol fee rate of the pool

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 12 | u8 |
| new_protocol_fee_bps | - | u16 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| pool_state | The pool's state singleton | W | N |

### SetProtocolFeeBeneficiary

Updates the pool's protocol fee beneficiary

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 13 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| old_beneficiary | The pool's old protocol fee beneficiary | R | Y |
| new_beneficiary | The pool's new protocol fee beneficiary | R | N |
| pool_state | The pool's state singleton | W | N |

### SetPricingProgram

Updates the pool's pricing program.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 14 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| new_pricing_program | The pool's new pricing program | R | N |
| pool_state | The pool's state singleton | W | N |

### WithdrawProtocolFees

Withdraw all accumulated protocol fees. Only the protocol_fee_beneficiary is authorized to call this.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 15 | u8 |
| amount | amount of LST to withdraw | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| protocol_fee_beneficiary | - | R | Y |
| withdraw_to_acc | token account to withdraw all accumulated protocol fees to | W | N | 
| protocol_fee_accumulator | The LST protocol fee accumulator token account to withdraw from | W | N |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W | N |
| token_program | - | R | N |
| pool_state | The pool's state singleton | W | N |

### AddDisablePoolAuthority

Add a disable pool authority

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 16 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| new_authority | The new disable pool authority to add | R | N |
| disable_pool_authority_list | DisablePoolAuthorityList PDA | W | N |

#### Procedure

- realloc and extend disable_pool_authority_list, and write new_authority in 

### RemoveDisablePoolAuthority

Remove a disable pool authority

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 17 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y if authority signature missing |
| authority | The authority to remove | R | Y if admin signature missing |
| disable_pool_authority_list | DisablePoolAuthorityList PDA | W | N |

#### Procedure

- rewrite array and resize pool down

### DisablePool

Disable functionality of the entire pool.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 18 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| authority | The pool's admin or a disable pool authority | R | Y |
| pool_state | The pool's state singleton | W | N |
| disable_pool_authority_list | DisablePoolAuthorityList PDA. Optional if authority = pool's admin | R | N |

#### Procedure

- set bool flag on pool_state

### EnablePool

Re-enable functionality of the entire pool.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 19 | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| admin | The pool's admin | R | Y |
| pool_state | The pool's state singleton | W | N |

#### Procedure

- unset bool flag on pool_state 

### Initialize

Initialize the pool. Can only be called once with hardcoded init authority.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 20 | u8 |
| protocol_fee_bps | initial protocol fee | u16 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| init_authority | The hardcoded init authority | R | Y |
| payer | Account paying for rent | W | Y |
| admin | The new pool's admin | R | N |
| protocol_fee_beneficiary | The new pool's protocol fee beneficiary | R | N |
| pricing_program | The new pool's pricing program | R | N |
| pool_state | The pool's state singleton | W | N |
| disable_pool_authority_list | The DisablePoolAuthorityList singleton | W | N |
| token_2022 | Token 2022 program | R | N |
| remaining_accounts | accounts required to initialize the LP token and transfer fee and metadata extensions | ... | ... |
