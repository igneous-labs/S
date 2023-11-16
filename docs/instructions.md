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
| discriminant | instruction discriminant | u8 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | mint of the LST to sync SOL value for | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_reserves | LST token account reserves of the pool | R | N |
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program LstToSol with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |

#### Procedure

- CPI the LST's SOL value calculator program LstToSol
- Record returned SOL value in pool_state

### SwapExactIn

Swap to output LST from an exact amount of given input LST.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | instruction discriminant | u8 |
| src_lst_value_calc_accs | number of accounts following dst_lst_acc to invoke src token's SOL value calculator program LstToSol with, including the program itself | u8 |
| dst_lst_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program SolToLst with, including the program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceExactIn with, including the program itself | u8 |
| amount | amount of src tokens to swap | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of src_lst_acc. User making the swap. | R | Y |
| src_lst | mint of the LST being swapped from | R | N |
| dst_lst | mint of the LST being swapped to | R | N |
| src_lst_acc | LST token account being swapped from | W | N |
| dst_lst_acc | LST token account to swap to | W | N |
| protocol_fee_dest | dst_lst protocol fee destination token account | W | N |
| token_program | - | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_src_reserves | src token token account reserves of the pool | W | N |
| pool_dst_reserves | dst token token account reserves of the pool | W | N |
| src_lst_value_calc_accs | accounts to invoke src token's SOL value calculator program LstToSol with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| dst_lst_value_calc_accs | accounts to invoke dst token's SOL value calculator program SolToLst with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceExactIn with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

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

Same as [SwapExactIn](#swapexactin-instruction), but amount is amount of dst tokens to receive and the core part goes like this instead:
- CPI dst token's SOL value calculator program LstToSol to get SOL value of output amount
- CPI pricing program PriceExactOut to get input SOL value
- CPI src token's SOL value calculator program SolToLst with input SOL value to get input token amount

### AddLiquidity

Add single-LST liquidity to the pool.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | instruction discriminant | u8 |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program LstToSol with, including the program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself | u8 |
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
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program LstToSol with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceLpTokensToMint with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

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
| discriminant | instruction discriminant | u8 |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program SolToLst with, including the program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself | u8 |
| amount | amount of LP tokens to burn and redeem | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of lp_acc. User who's adding liquidity. | R | Y |
| lst | LST token mint | R | N |
| dst_lst_acc | LST token account to redeem to | W | N |
| src_lp_token_acc | LP token account to burn LP tokens from | W | N |
| protocol_fee_dest | dst_lst protocol fee destination token account | W | N |
| token_program | - | R | N |
| pool_state | the pool's state singleton | W | N |
| pool_reserves | pool's token reserves for the LST | W | N |
| lst_value_calc_accs | accounts to invoke token's SOL value calculator program SolToLst with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program PriceLpTokensToRedeem with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- Self CPI SyncSolValue for LST
- CPI pricing program's PriceLpTokensToRedeem with SOL value of LP tokens to be burt
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
| discriminant | instruction discriminant | u8 |
| pricing program args | ... | ... |

raw bytes of pricing program args are passed directly to pricing program CPI

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_accs | accounts to invoke pricing program with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

#### Procedure

- CPI pricing program with the accounts and data passed in, signed by the pricing program authority PDA
