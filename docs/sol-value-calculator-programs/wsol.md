# wSOL

SOL value calculator program for wrapped SOL.

Basically a no-op

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account  | Description   | Read/Write (R/W) | Signer (Y/N) |
| -------- | ------------- | ---------------- | ------------ |
| lst_mint | See interface | R                | N            |

##### Procedure

- Verify lst_mint = wrapped SOL mint address
- Return passed in amount

#### SolToLst

##### Accounts

| Account  | Description   | Read/Write (R/W) | Signer (Y/N) |
| -------- | ------------- | ---------------- | ------------ |
| lst_mint | See interface | R                | N            |

##### Procedure

- Verify lst_mint = wrapped SOL mint address
- Return passed in amount
