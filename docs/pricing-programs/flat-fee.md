# Flat Fee Pricing Program

Pricing program that levies flat fees.

Fees are deducted by taking a set portion from the calculated resulting value depending on the token type.

## Accounts
### ProgramState

The program state singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| manager | The manager authorized to update the fee accounts for each LST and LP | Pubkey |
| lp_withdrawal_fee | Fee in bips to impose when redeeming LP token for LST | u16 |

### FeeAccount

The Account that describes the fee for each pricing type. The FeeAccount is located at PDA ["fee", token_mint].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

NOTE: a negative fee value means incentivization for given route

| Name | Value | Type |
| -- | -- | -- |
| input_fee | Fee in bips to impose when the token type is used as input | i16 |
| output_fee | Fee in bips to impose when the token type is used as output | i16 |

## Instructions
### Common Interface
#### PriceExactIn

Given an input LST amount and its SOL value, calculate the output SOL value by:
 - calculate total fee in bips by adding `fee_acc_input.input_fee` and `fee_acc_output.output_fee`
 - calculate output LST's sol value after imposing fee by using the calculated fee and the given `sol_value` of input lst

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| amount | Amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_authority | Pricing authority PDA | R | Y |
| lst_input | Input LST token mint | R | N |
| lst_output | Output LST token mint | R | N |
| fee_acc_input | FeeAccount PDA for input LST | R | N |
| fee_acc_output | FeeAccount PDA for output LST | R | N |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | The calculated output SOL value | u64 |

##### Procedure

#### PriceExactOut

Given an output LST amount and its SOL value, calculate the input SOL value by:
 - calculate total fee in bips by adding `fee_acc.input_fee` and `fee_acc.output_fee`
 - calculate input LST's sol value using given `sol_value` of output lst assuming that the calculated fee was imposed to resulting input lst's SOL value

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| amount | Amount of output LST | u64 |
| sol_value | SOL value of amount output LST | u64 |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | The calculated input SOL value | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_authority | Pricing authority PDA | R | Y |
| lst_input | Input LST token mint | R | N |
| lst_output | Output LST token mint | R | N |
| fee_acc_input | FeeAccount PDA for input LST | R | N |
| fee_acc_output | FeeAccount PDA for output LST | R | N |

##### Procedure

#### PriceLpTokensToMint

Given an input LST amount and its SOL value, calculate the SOL value of the LP tokens to mint.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 2 | u8 |
| amount | Amount of input LST | u64 |
| sol_value | SOL value of amount input LST | u64 |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | The calculated SOL value of LP tokens to mint | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_authority | Pricing authority PDA | R | Y |
| lst_input | Input LST token mint | R | N |

##### Procedure

#### PriceLpTokensToRedeem

Given an input LP token amount and its SOL value, calculate the SOL value of the LST to redeem.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 3 | u8 |
| amount | Amount of input LP | u64 |
| sol_value | SOL value of amount input LP | u64 |

##### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | The calculated SOL value of the LST to redeem | u64 |

#####  Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| pricing_authority | Pricing authority PDA | R | Y |
| lst_output | Output LST token mint | R | N |
| state | Program state PDA | R | N |

##### Procedure

Regardless of how the price is calculated, the pricing program should guarantee that this instruction levies sufficient fees on the redeem amount such that LPs cannot extract value from the pool by adding liquidity right before the epoch boundary and then removing liquidity right after the SOL value increase from staking rewards. 

### Management Instructions

Only the current manager is authorized to execute.

#### Init

Initialize the program state. Can only be called once with hardcoded init authority.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 255 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| init_authority | The hardcoded init authority of pricing program | R | Y |
| manager | The program manager | R | N |
| state | Program state PDA | W | N |

#### SetManager

Update the manager authority of the pricing program.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 254 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| manager | The program manager | R | Y |
| new_manager | The new program manager to set to | R | N |
| state | Program state PDA | W | N |

#### SetFee

Update the fees for given type of pricing action.

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 253 | u8 |
| input_fee | Fee in bips to impose when the token type is used as input | i16 |
| output_fee | Fee in bips to impose when the token type is used as output | i16 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| manager | The program manager | R | Y |
| fee_acc | FeeAccount PDA to modify | W | N |

#### SetLpWithdrawalFee

Update the fees imposed for redeeming LP token for LST

##### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 252 | u8 |
| lp_withdrawal_fee | Fee in bips to impose when redeeming LP token for LST | u16 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| signer | Authority of pricing program | R | Y |
| state | Program state PDA | W | N |
