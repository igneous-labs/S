# Accounts

Controller program accounts schema and PDA definitions.

## PoolState

The pool state singleton is located at PDA ["state"].

### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| total_sol_value | The last recorded total SOL value of the pool, updated by SyncSolValue | u64 |
| protocol_fee | The flat protocol fee to charge on output amounts | Ratio |
| admin | The admin pubkey authorized to perform all admin actions | Pubkey |
| protocol_fee_beneficiary | Beneficiary of protocol fees that is authorized to withdraw accumulated protocol fees | Pubkey |
| pricing_program | Address of pricing program used by pool | Pubkey |
| lst_states | Dynamic list of LstStates for each LST in the pool | LstState[] |

#### Ratio Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| num | Numerator | u64 |
| denom | Denominator | u64 |

#### LstState Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| is_input_disabled | Flag indicating if inputs for this LST are disabled | bool |
| sol_value | SOL value of this LST's pool reserves balance, updated by SyncSolValue | u64 |
| token | The LST's mint | Pubkey |
| sol_value_calculator | The LST's SOL value calculator program | Pubkey |

## LST Reserves

For each LST, the LST reserve is located at the associated token address (ATA) of the pool state singleton.

## Protocol Fee Accumulators

For each LST, protocol fees are accumulated at the associated token address (ATA) of PDA ["protocol_fee"]
