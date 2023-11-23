# SPL

SOL value calculator program for SPL stake pool program.

To avoid being rugged by compromise of the SPL stake pool program, this program records the last updated slot of the SPL stake pool program and errors if the current one does not match.

A manager is solely authorized to whitelist the current SPL stake pool program deployed.

## Notes

- Only considers stake withdrawal fee for both interface instructions, never deposit fee or SOL withdraw fee
- Always assume the manager fee account is valid and withdrawal fees are levied

## Accounts

### SplCalculatorState

The SplCalculatorState singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name              | Value                                                                   | Type   |
| ----------------- | ----------------------------------------------------------------------- | ------ |
| manager           | The SOL value calculator program manager                                | Pubkey |
| last_upgrade_slot | The last recorded slot at which the SPL stake pool program was upgraded | u64    |

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account          | Description                          | Read/Write (R/W) | Signer (Y/N) |
| ---------------- | ------------------------------------ | ---------------- | ------------ |
| lst              | See interface                        | R                | N            |
| state            | The SplCalculatorState singleton PDA | R                | N            |
| stake_pool       | The main stake pool account          | R                | N            |
| spl_program      | spl program                          | R                | N            |
| spl_program_data | spl program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check stake_pool program hardcoded address
- Check stake_pool program data matches that on stake_pool program
- Check state.last_upgrade_slot matches that on program data
- Check lst mint addr matches stake_pool's, stake_pool owner = SPL program, account_type = AccountType::StakePool
- Check stake_pool updated for current epoch
- Calculate output SOL based on code copied from `process_withdraw_stake()`

#### SolToLst

##### Accounts

| Account          | Description                          | Read/Write (R/W) | Signer (Y/N) |
| ---------------- | ------------------------------------ | ---------------- | ------------ |
| lst              | See interface                        | R                | N            |
| state            | The SplCalculatorState singleton PDA | R                | N            |
| stake_pool       | The main stake pool account          | R                | N            |
| spl_program      | spl program                          | R                | N            |
| spl_program_data | spl program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check stake_pool program hardcoded address
- Check stake_pool program data matches that on stake_pool program
- Check state.last_upgrade_slot matches that on program data
- Check lst mint addr matches stake_pool's, stake_pool owner = SPL program, account_type = AccountType::StakePool
- Check stake_pool updated for current epoch
- Calculate LST amount by reversing procedure in `process_withdraw_stake()`

### Management Instructions

#### UpdateLastUpgradeSlot

Update last_upgrade_slot to SPL program's current one.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 253   | u8   |

##### Accounts

| Account          | Description                          | Read/Write (R/W) | Signer (Y/N) |
| ---------------- | ------------------------------------ | ---------------- | ------------ |
| manager          | The manager pubkey                   | R                | Y            |
| state            | The SplCalculatorState singleton PDA | W                | N            |
| spl_program      | SPL program                          | R                | N            |
| spl_program_data | SPL program executable data          | R                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Check SPL program hardcoded address
- Check SPL program data matches that on SPL program
- Write last_upgrade_slot to state

#### SetManager

Set a new manager.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 254   | u8   |

##### Accounts

| Account     | Description                          | Read/Write (R/W) | Signer (Y/N) |
| ----------- | ------------------------------------ | ---------------- | ------------ |
| manager     | The manager pubkey                   | R                | Y            |
| new_manager | The new manager to set               | R                | N            |
| state       | The SplCalculatorState singleton PDA | W                | N            |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Write new_manager to state

#### Init

Initialize SplCalculatorState, can only be called once.

#### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 255   | u8   |

##### Accounts

| Account        | Description                                      | Read/Write (R/W) | Signer (Y/N) |
| -------------- | ------------------------------------------------ | ---------------- | ------------ |
| payer          | The account paying for SplCalculatorState's rent | W                | Y            |
| state          | The SplCalculatorState singleton PDA             | W                | N            |
| system_program | System Program                                   | R                | N            |

##### Procedure

- Initialize state
- Set manager to initial hardcoded manager
- Set last_upgrade_slot to 0
