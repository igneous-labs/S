# SPL

SOL value calculator program for SPL and Socean stake pool program.

## Notes

- Only considers stake withdrawal fee for both interface instructions, never deposit fee or SOL withdraw fee
- Always assume the manager fee account is valid and withdrawal fees are levied

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| stake_pool | The main stake pool account | R | N |

##### Procedure

- Check stake_pool owner = SPL or Socean program, account_type = AccountType::StakePool
- Calculate output SOL based on code copied from `process_withdraw_stake()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| stake_pool | The main stake pool account | R | N |

##### Procedure

- Check stake_pool owner = SPL or Socean program, account_type = AccountType::StakePool
- Calculate LST amount by reversing procedure in `process_withdraw_stake()`
