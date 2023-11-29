# Accounts

Controller program accounts schema and PDA definitions.

Note that `PodBool` type is represented with `u8`.

## PoolState

The pool state singleton is located at PDA ["state"].

### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name                     | Value                                                                                 | Type    |
| ------------------------ | ------------------------------------------------------------------------------------- | ------- |
| total_sol_value          | The last recorded total SOL value of the pool, updated by SyncSolValue                | u64     |
| trading_protocol_fee_bps | The flat protocol fee to charge on swap fees in bps                                   | u16     |
| lp_protocol_fee_bps      | The flat protocol fee to charge on LP withdrawal fees in bps                          | u16     |
| version                  | incrementing counter representing schema version number. Starts at 1                  | u8      |
| is_disabled              | true if all functionality of the pool has been disabled by DisablePool                | PodBool |
| is_rebalancing           | true if a rebalance is currently occuring                                             | PodBool |
| admin                    | The admin pubkey authorized to perform all admin actions                              | Pubkey  |
| rebalance_authority      | The pubkey authorized to rebalance                                                    | Pubkey  |
| protocol_fee_beneficiary | Beneficiary of protocol fees that is authorized to withdraw accumulated protocol fees | Pubkey  |
| pricing_program          | Address of pricing program used by pool                                               | Pubkey  |

## LstStateList

The LST state list singleton is located at PDA ["lst-state-list"].

### Schema

| Name           | Value                                              | Type       |
| -------------- | -------------------------------------------------- | ---------- |
| lst_state_list | Dynamic list of LstStates for each LST in the pool | LstState[] |

#### LstState Schema

The struct is bytemuck/zero_copy as well since PoolState is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name                          | Value                                                                  | Type    |
| ----------------------------- | ---------------------------------------------------------------------- | ------- |
| is_input_disabled             | Flag indicating if inputs for this LST are disabled                    | PodBool |
| reserves_bump                 | bump seed of this LST's reserves ATA                                   | u8      |
| protocol_fee_accumulator_bump | bump seed of this LST's protocol fee accumulator ATA                   | u8      |
| sol_value                     | SOL value of this LST's pool reserves balance, updated by SyncSolValue | u64     |
| mint                          | The LST's mint                                                         | Pubkey  |
| sol_value_calculator          | The LST's SOL value calculator program                                 | Pubkey  |

## DisablePoolAuthorityList

List of pubkeys authorized to disable the pool. PDA ["disable-pool-authority-list"].

### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name                | Value                                                       | Type     |
| ------------------- | ----------------------------------------------------------- | -------- |
| whitelisted_pubkeys | list of pubkeys allowed to call the DisablePool instruction | Pubkey[] |

## RebalanceRecord

Transient hot potato account that records data about the current rebalancing. PDA ["rebalance-record"].

### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name                    | Value                                                                                                                                                                                         | Type   |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| sol_value               | SOL value of the LST tokens withdrawn to be rebalanced                                                                                                                                        | u64    |
| dst_lst_index           | index of dst_lst in PoolState.lst_state_list                                                                                                                                                  | u64    |
| dst_lst_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself | u8     |
| dst_lst                 | token mint of the LST being rebalanced to                                                                                                                                                     | Pubkey |

## LST Reserves

For each LST, the LST reserve is located at the associated token address (ATA) of the pool state singleton.

## Protocol Fee Accumulators

For each LST, protocol fees are accumulated at the associated token address (ATA) of PDA ["protocol-fee"]
