# wSOL

SOL value calculator program for wrapped SOL.

Basically a no-op

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description   | Read/Write (R/W) | Signer (Y/N) |
| ------- | ------------- | ---------------- | ------------ |
| lst     | See interface | R                | N            |

##### Procedure

- Verify lst = wrapped SOL mint address
- Return passed in amount

#### SolToLst

##### Accounts

| Account | Description   | Read/Write (R/W) | Signer (Y/N) |
| ------- | ------------- | ---------------- | ------------ |
| lst     | See interface | R                | N            |

##### Procedure

- Verify lst = wrapped SOL mint address
- Return passed in amount
