# generic_pool

Interface for a SOL value calculator program for a generic stake pool that has a single pool state account from which SOL values cant be directly calculated.

To avoid being rugged by compromise of the stake pool program, this program records the last updated slot of the SPL stake pool program and errors if the current one does not match.

A manager is solely authorized to whitelist the current stake pool program deployed.

## Accounts

### CalculatorState

The CalculatorState singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name              | Value                                                               | Type   |
| ----------------- | ------------------------------------------------------------------- | ------ |
| manager           | The SOL value calculator program manager                            | Pubkey |
| last_upgrade_slot | The last recorded slot at which the stake pool program was upgraded | u64    |

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account           | Description                            | Read/Write (R/W) | Signer (Y/N) |
| ----------------- | -------------------------------------- | ---------------- | ------------ |
| lst_mint          | See interface                          | R                | N            |
| state             | The CalculatorState singleton PDA      | R                | N            |
| pool_state        | The main stake pool state account      | R                | N            |
| pool_program      | The stake pool program                 | R                | N            |
| pool_program_data | The stake pool program executable data | R                | N            |

#### SolToLst

##### Accounts

| Account           | Description                            | Read/Write (R/W) | Signer (Y/N) |
| ----------------- | -------------------------------------- | ---------------- | ------------ |
| lst_mint          | See interface                          | R                | N            |
| state             | The CalculatorState singleton PDA      | R                | N            |
| pool_state        | The main stake pool state account      | R                | N            |
| pool_program      | The stake pool program                 | R                | N            |
| pool_program_data | The stake pool program executable data | R                | N            |

### Management Instructions

#### UpdateLastUpgradeSlot

Update last_upgrade_slot to the stake pool program's current one.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 253   | u8   |

##### Accounts

| Account           | Description                            | Read/Write (R/W) | Signer (Y/N) |
| ----------------- | -------------------------------------- | ---------------- | ------------ |
| manager           | The manager pubkey                     | R                | Y            |
| state             | The CalculatorState singleton PDA      | W                | N            |
| pool_program      | The stake pool program                 | R                | N            |
| pool_program_data | The stake pool program executable data | R                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Check pool_program address
- Check program data matches that on pool_program
- Write last_upgrade_slot to state

#### SetManager

Set a new manager.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 254   | u8   |

##### Accounts

| Account     | Description                       | Read/Write (R/W) | Signer (Y/N) |
| ----------- | --------------------------------- | ---------------- | ------------ |
| manager     | The manager pubkey                | R                | Y            |
| new_manager | The new manager to set            | R                | N            |
| state       | The CalculatorState singleton PDA | W                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Write new_manager to state

#### Init

Initialize CalculatorState, can only be called once.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 255   | u8   |

##### Accounts

| Account        | Description                                   | Read/Write (R/W) | Signer (Y/N) |
| -------------- | --------------------------------------------- | ---------------- | ------------ |
| payer          | The account paying for CalculatorState's rent | W                | Y            |
| state          | The CalculatorState singleton PDA             | W                | N            |
| system_program | System Program                                | R                | N            |

##### Procedure

- Initialize state
- Set manager to initial hardcoded manager
- Set last_upgrade_slot to 0
