# Marinade

SOL value calculator program for Marinade program.

To avoid being rugged by compromise of the marinade program, this program records the last updated slot of the marinade program and errors if the current one does not match.

A manager is solely authorized to whitelist the current marinade program deployed.

Compatible with [generic_pool interface](./generic_pool.md)

## Accounts

### MarinadeCalculatorState

The MarinadeCalculatorState singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name              | Value                                                             | Type   |
| ----------------- | ----------------------------------------------------------------- | ------ |
| manager           | The SOL value calculator program manager                          | Pubkey |
| last_upgrade_slot | The last recorded slot at which the marinade program was upgraded | u64    |

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account               | Description                               | Read/Write (R/W) | Signer (Y/N) |
| --------------------- | ----------------------------------------- | ---------------- | ------------ |
| lst                   | See interface                             | R                | N            |
| state                 | The MarinadeCalculatorState singleton PDA | R                | N            |
| marinade_state        | The marinade state account                | R                | N            |
| marinade_program      | marinade program                          | R                | N            |
| marinade_program_data | marinade program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check marinade program hardcoded address
- Check marinade program data matches that on marinade program
- Check state.last_upgrade_slot matches that on program data
- Check mSOL mint addr, marinade_state hardcoded address
- Calculate output SOL based on code copied from `OrderUnstake::process()`

#### SolToLst

##### Accounts

| Account               | Description                               | Read/Write (R/W) | Signer (Y/N) |
| --------------------- | ----------------------------------------- | ---------------- | ------------ |
| lst                   | See interface                             | R                | N            |
| state                 | The MarinadeCalculatorState singleton PDA | R                | N            |
| marinade_state        | The marinade state account                | R                | N            |
| marinade_program      | marinade program                          | R                | N            |
| marinade_program_data | marinade program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check marinade program hardcoded address
- Check marinade program data matches that on marinade program
- Check state.last_upgrade_slot matches that on program data
- Check mSOL mint addr, marinade_state hardcoded address
- Calculate LST amount by reversing procedure in `OrderUnstake::process()`

### Management Instructions

#### UpdateLastUpgradeSlot

Update last_upgrade_slot to marinade program's current one.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 253   | u8   |

##### Accounts

| Account               | Description                               | Read/Write (R/W) | Signer (Y/N) |
| --------------------- | ----------------------------------------- | ---------------- | ------------ |
| manager               | The manager pubkey                        | R                | Y            |
| state                 | The MarinadeCalculatorState singleton PDA | W                | N            |
| marinade_program      | marinade program                          | R                | N            |
| marinade_program_data | marinade program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Check marinade program hardcoded address
- Check marinade program data matches that on marinade program
- Write last_upgrade_slot to state

#### SetManager

Set a new manager.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 254   | u8   |

##### Accounts

| Account     | Description                               | Read/Write (R/W) | Signer (Y/N) |
| ----------- | ----------------------------------------- | ---------------- | ------------ |
| manager     | The manager pubkey                        | R                | Y            |
| new_manager | The new manager to set                    | R                | N            |
| state       | The MarinadeCalculatorState singleton PDA | W                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Write new_manager to state

#### Init

Initialize MarinadeCalculatorState, can only be called once.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 255   | u8   |

##### Accounts

| Account | Description                               | Read/Write (R/W) | Signer (Y/N) |
| ------- | ----------------------------------------- | ---------------- | ------------ |
| state   | The MarinadeCalculatorState singleton PDA | W                | N            |

##### Procedure

- Initialize state
- Set manager to initial hardcoded manager
- Set last_upgrade_slot to 0
