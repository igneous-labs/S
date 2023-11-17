# Lido

SOL value calculator program for Lido on Solana program.

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lido | The lido stake pool state account | R | N |

##### Procedure

- `Lido::deserialize_lido(lido)` - this checks program ownership etc
- Calculate output SOL based on code copied from `process_withdraw()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lido | The lido stake pool state account | R | N |

##### Procedure

- `Lido::deserialize_lido(lido)` - this checks program ownership etc
- Calculate LST amount by reversing procedure in `process_withdraw()`
