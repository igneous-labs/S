# Interface

The common interface all SOL value calculator programs must follow.

## Instructions

### LstToSol

Given a LST quantity, calculate its SOL value.

Should validate accounts passed in and conditions - e.g. stake pool has been updated for this epoch for SPL.

#### Data

| Name         | Value          | Type |
| ------------ | -------------- | ---- |
| discriminant | 0              | u8   |
| amount       | amount of LSTs | u64  |

#### Accounts

| Account            | Description                                                                                                 | Read/Write (R/W) | Signer (Y/N) |
| ------------------ | ----------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| lst_mint           | Token mint of the lst                                                                                       | R                | N            |
| remaining_accounts | Any remaining accounts the program needs. Varies with each liquid staking program. Must be same as SolToLst | ...              | ...          |

#### Return Data

| Name | Value                                                  | Type |
| ---- | ------------------------------------------------------ | ---- |
| min  | minimum value of calculated SOL value range, inclusive | u64  |
| max  | maximum value of calculated SOL value range, inclusive | u64  |

### SolToLst

Given a SOL value, calculate its LST quantity.

Slightly confusing but following the [definition of SOL value](../overview.md#sol-value), this should be thought of as "how much LST do I need to redeem into the given SOL value at this current instant" instead of "how much LST can I get mint from staking the given SOL value at this current instant".

Should validate accounts passed in and conditions - e.g. stake pool has been updated for this epoch for SPL.

#### Data

| Name         | Value         | Type |
| ------------ | ------------- | ---- |
| discriminant | 1             | u8   |
| amount       | amount of SOL | u64  |

#### Accounts

| Account            | Description                                                                                                 | Read/Write (R/W) | Signer (Y/N) |
| ------------------ | ----------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| lst_mint           | Token mint of the lst                                                                                       | R                | N            |
| remaining_accounts | Any remaining accounts the program needs. Varies with each liquid staking program. Must be same as LstToSol | ...              | ...          |

#### Return Data

| Name | Value                                                   | Type |
| ---- | ------------------------------------------------------- | ---- |
| min  | minimum value of calculated LST amount range, inclusive | u64  |
| max  | maximum value of calculated LST amount range, inclusive | u64  |

### LstToSol - SolToLst Accounts Symmetricality

Implementations are free to specify any remaining accounts required, but they must be the same for both `LstToSol` and `SolToLst` such that a program is able to invoke both instructions successfully with the same set of accounts.
