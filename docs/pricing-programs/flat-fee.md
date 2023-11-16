# Flat Fee Pricing Program

Pricing program that levies flat fees to each LST quotes.

## Instructions

### PriceExactIn

Given an input LST amount and its SOL value, calculate the output SOL value.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| amount | amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |

#### Accounts

Varies with each pricing program. Should include controller program's pricing program authority PDA for authorization and the 2 LSTs involved.

#### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated SOL value | u64 |

### PriceExactOut

Given an output LST amount and its SOL value, calculate the input SOL value.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| amount | amount of output LST | u64 |
| sol_value | SOL value of amount output LST | u64 |

#### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated SOL value | u64 |

### PriceLpTokensToMint

Given an input LST amount and its SOL value, calculate the SOL value of the LP tokens to mint.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 2 | u8 |
| amount | amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |

#### Accounts

Varies with each pricing program. Should include controller program's pricing program authority PDA for authorization and the input LST.

### PriceLpTokensToRedeem

Given an input LP token amount and its SOL value, calculate the SOL value of the LST to redeem.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 3 | u8 |
| amount | amount of input LP | u64 |
| sol_value | SOL value of amount input LP | u64 |

#### Accounts

Varies with each pricing program. Should include controller program's pricing program authority PDA for authorization and the output LST.

#### Procedure

Regardless of how the price is calculated, the pricing program should guarantee that this instruction levies sufficient fees on the redeem amount such that LPs cannot extract value from the pool by adding liquidity right before the epoch boundary and then removing liquidity right after the SOL value increase from staking rewards. 

### Other Instructions

Each pricing program may also have different instructions for state management and control.
