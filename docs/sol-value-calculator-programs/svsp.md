# SVSP

SOL value calculator program for Single Validator Stake Pool program.

To avoid being rugged by compromise of the SVSP program, this program records the last updated slot of the SVSP program and errors if the current one does not match.

A manager is solely authorized to whitelist the current SVSP program deployed. 

## Accounts

### SvspCalculatorState

The SvspCalculatorState singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| manager | The SOL value calculator program manager | Pubkey |
| last_upgrade_slot | The last recorded slot at which the SVSP program was upgraded | u64 |

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | See interface | R | N |
| state | The SvspCalculatorState singleton PDA | R | N |
| pool | The SVSP pool account | R | N |
| svsp_program | SVSP program | R | N |
| svsp_program_data | SVSP program executable data | R | N |

##### Procedure

- Check state PDA
- Check SVSP program hardcoded address
- Check SVSP program data matches that on SVSP program
- Check state.last_upgrade_slot matches that on program data
- Check pool program ownership + AccountType, lst mint PDA.
- Calculate output SOL based on code copied from `process_withdraw()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | See interface | R | N |
| state | The SvspCalculatorState singleton PDA | R | N |
| pool | The SVSP pool account | R | N |
| svsp_program | SVSP program | R | N |
| svsp_program_data | SVSP program executable data | R | N |

##### Procedure

- Check state PDA
- Check SVSP program hardcoded address
- Check SVSP program data matches that on SVSP program
- Check state.last_upgrade_slot matches that on program data
- Check pool program ownership + AccountType, lst mint PDA.
- Calculate LST amount by reversing procedure in `process_withdraw()`

### Management Instructions

#### UpdateLastUpgradeSlot

Update last_upgrade_slot to SVSP program's current one.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 253 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| manager | The manager pubkey | R | Y |
| state | The SvspCalculatorState singleton PDA | W | N |
| svsp_program | SVSP program | R | N |
| svsp_program_data | SVSP program executable data | R | N |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Check SVSP program hardcoded address
- Check SVSP program data matches that on SVSP program
- Write last_upgrade_slot to state

#### SetManager

Set a new manager.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 254 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| manager | The manager pubkey | R | Y |
| new_manager | The new manager to set | R | N |
| state | The SvspCalculatorState singleton PDA | W | N |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Write new_manager to state

#### Init

Initialize SvspCalculatorState, can only be called once.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 255 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| state | The SvspCalculatorState singleton PDA | W | N |

##### Procedure

- Initialize state
- Set manager to initial hardcoded manager
- Set last_upgrade_slot to 0
