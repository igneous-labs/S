# Interfaces

Programs interfaces

## Controller Program

### SwapExactIn Instruction

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | instruction discriminant | u8 |
| src_token_value_calc_accs | number of accounts following dst_token_acc to invoke src token's SOL value calculator program with, including the program itself | u8 |
| dst_token_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program with, including the program itself | u8 |
| pricing_accs | number of accounts following to invoke pricing program with, including the program itself | u8 |
| amount | amount of src tokens to swap | u64 |

#### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of src_token_acc. User making the swap. | R | Y |
| src_token_acc | LST token account being swapped from | W | N |
| dst_token_acc | LST token account to swap to | W | N |
| pool_src_reserves | src token token account reserves of the pool | W | N |
| pool_dst_reserves | dst token token account reserves of the pool | W | N |
| pool_state | the pool's state singleton | W | N |
| src_token_value_calc_accs | accounts to invoke src token's SOL value calculator program with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| dst_token_value_calc_accs | accounts to invoke dst token's SOL value calculator program with. First account should be the calculator program itself. Multiple Accounts. | ... | ... |
| pricing_accs | accounts to invoke pricing program with. First account should be the pricing program itself. Multiple Accounts. | ... | ... |

## SOL Value Calculator Program

### LstToSol Instruction

Given a LST quantity, calculate its SOL value.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| amount | amount of LSTs | u64 |

#### Accounts

Varies with each LST program.

#### Return Data

| Name | Value | Type |
| result | the calculated SOL value | u64 | 

### SolToLst Instruction

Given a SOL value, calculate its LST quantity.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| amount | amount of SOL | u64 |

#### Accounts

Varies with each LST program.

#### Return Data

| Name | Value | Type |
| result | the calculated LST amount | u64 | 

## Pricing Program

### PriceExactOut Instruction

### PriceExactIn Instruction
