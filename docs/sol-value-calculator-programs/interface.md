# Interface

The common interface all SOL value calculator programs must follow.

## Instructions

### LstToSol

Given a LST quantity, calculate its SOL value.

Should validate accounts passed in and conditions - e.g. stake pool has been updated for this epoch for SPL.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 0 | u8 |
| amount | amount of LSTs | u64 |

#### Accounts

Varies with each LST program.

#### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated SOL value | u64 |

### SolToLst

Given a SOL value, calculate its LST quantity.

Slightly confusing but following the [definition of SOL value](../overview.md#sol-value), this should be thought of as "how much LST do I need to redeem into the given SOL value at this current instant" instead of "how much LST can I get mint from staking the given SOL value at this current instant".

Should validate accounts passed in and conditions - e.g. stake pool has been updated for this epoch for SPL.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 1 | u8 |
| amount | amount of SOL | u64 |

#### Accounts

Varies with each LST program.

#### Return Data

| Name | Value | Type |
| -- | -- | -- |
| result | the calculated LST amount | u64 |
