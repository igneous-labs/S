# Marinade

SOL value calculator program for Marinade program.

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| token | See interface | R | N |
| state | The marinade state account | R | N |

##### Procedure

- Check mSOL mint addr, stake_pool owner = marinade program and anchor account discriminator
- Calculate output SOL based on code copied from `OrderUnstake::process()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| token | See interface | R | N |
| state | The marinade state account | R | N |

##### Procedure

- Check mSOL mint addr, stake_pool owner = marinade program and anchor account discriminator
- Calculate LST amount by reversing procedure in `OrderUnstake::process()`
