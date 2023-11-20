# Flat Fee Pricing Program

Pricing program that levies flat fees.

Fees are deducted by taking a set portion from the calculated resulting value depending on the token type.

## Accounts
### ProgramState

The program state singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| manager | The manager authorized to update the fee accounts for each LST and LP | Pubkey |

### FeeAccount

The FeeAccount is located at PDA ["fee", token_mint].

#### Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| input_fee | fee in ratio to impose when the token type is used as input | Ratio |
| output_fee | fee in ratio to impose when the token type is used as output | Ratio |

#### Ratio Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| num | Numerator | u64 |
| denom | Denominator | u64 |

## Instructions
### Common Interface
#### PriceExactIn

Given an input LST amount and its SOL value, calculate the output SOL value.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| amount | amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst_input | input LST token mint | R | N |
| lst_output | output LST token mint | R | N |
| fee_acc | account that describes the fee for each pricing type | R | N |
| pricing_authority | PDA for pricing authorization | R | Y |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated output SOL value | u64 |

#### PriceExactOut

Given an output LST amount and its SOL value, calculate the input SOL value.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| amount | amount of output LST | u64 |
| sol_value | SOL value of amount output LST | u64 |
| pricing_authority | PDA for pricing authorization | R | Y |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated input SOL value | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst_input | input LST token mint | R | N |
| lst_output | output LST token mint | R | N |
| fee_acc | account that describes the fee for each pricing type | R | N |
| pricing_authority | PDA for pricing authorization | R | Y |

#### PriceLpTokensToMint

Given an input LST amount and its SOL value, calculate the SOL value of the LP tokens to mint.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 2 | u8 |
| amount | amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |
| pricing_authority | PDA for pricing authorization | R | Y |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated SOL value of LP tokens to mint | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst_input | input LST token mint | R | N |
| fee_acc | account that describes the fee for each pricing type | R | N |
| pricing_authority | PDA for pricing authorization | R | Y |

#### PriceLpTokensToRedeem

Given an input LP token amount and its SOL value, calculate the SOL value of the LST to redeem.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 3 | u8 |
| amount | amount of input LP | u64 |
| sol_value | SOL value of amount input LP | u64 |
| pricing_authority | PDA for pricing authorization | R | Y |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated SOL value of the LST to redeem | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst_output | output LST token mint | R | N |
| fee_acc | account that describes the fee for each pricing type | R | N |
| pricing_authority | PDA for pricing authorization | R | Y |

##### Procedure

Regardless of how the price is calculated, the pricing program should guarantee that this instruction levies sufficient fees on the redeem amount such that LPs cannot extract value from the pool by adding liquidity right before the epoch boundary and then removing liquidity right after the SOL value increase from staking rewards. 

### Management Instructions

Only the current manager is authorized to execute.

#### Initialize

Initialize the program state.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 4 | u8 |
| manager | The manager authorized to update the fee accounts for each LST and LP | Pubkey |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of pricing program | R | Y |

#### SetManager

Update the manager authority of the pricing program.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 5 | u8 |
| manager | The manager authorized to update the fee accounts for each LST and LP | Pubkey |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of pricing program | R | Y |

#### SetFee

Update the fees for given type of pricing action.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 6 | u8 |
| input_fee | fee in bips to impose when the token type is used as input | u16 |
| output_fee | fee in bips to impose when the token type is used as output | u16 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| fee_acc | account that describes the fee for each pricing type | R | N |
| signer | Authority of pricing program | R | Y |
