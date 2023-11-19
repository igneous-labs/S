# SVSP

SOL value calculator program for Single Validator Stake Pool program.

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| token | See interface | R | N |
| pool | The SVSP pool struct | R | N |

##### Procedure

- Check pool program ownership + AccountType, token PDA.
- Calculate output SOL based on code copied from `process_withdraw()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| token | See interface | R | N |
| pool | The SVSP pool struct | R | N |

##### Procedure

- Check pool program ownership + AccountType, token PDA.
- Calculate LST amount by reversing procedure in `process_withdraw()`
