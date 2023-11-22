# Lido

SOL value calculator program for Lido for Solana program.

To avoid being rugged by compromise of the lido program, this program records the last updated slot of the lido program and errors if the current one does not match.

A manager is solely authorized to whitelist the current lido program deployed. 

## Accounts

### LidoCalculatorState

The LidoCalculatorState singleton is located at PDA ["state"].

#### Schema

The struct is bytemuck/zero_copy. Explicit manual padding is required, but not shown.

| Name | Value | Type |
| -- | -- | -- |
| manager | The SOL value calculator program manager | Pubkey |
| last_upgrade_slot | The last recorded slot at which the lido program was upgraded | u64 |

## Instructions

### Common Interface

#### LstToSol

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | See interface | R | N |
| state | The LidoCalculatorState singleton PDA | R | N |
| lido_state | The lido state account | R | N |
| lido_program | lido program | R | N |
| lido_program_data | lido program executable data | R | N |

##### Procedure

- Check state PDA
- Check lido program hardcoded address
- Check lido program data matches that on lido program
- Check state.last_upgrade_slot matches that on program data
- Check stSOL mint addr, lido_state hardcoded address
- Calculate output SOL based on code copied from `process_withdraw()`

#### SolToLst

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| lst | See interface | R | N |
| state | The LidoCalculatorState singleton PDA | R | N |
| lido_state | The lido state account | R | N |
| lido_program | lido program | R | N |
| lido_program_data | lido program executable data | R | N |

##### Procedure

- Check state PDA
- Check lido program hardcoded address
- Check lido program data matches that on lido program
- Check state.last_upgrade_slot matches that on program data
- Check stSOL mint addr, lido_state hardcoded address
- Calculate stSOL amount by reversing procedure in `process_withdraw()`

### Management Instructions

#### UpdateLastUpgradeSlot

Update last_upgrade_slot to lido program's current one.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 253 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| manager | The manager pubkey | R | Y |
| state | The LidoCalculatorState singleton PDA | W | N |
| lido_program | lido program | R | N |
| lido_program_data | lido program executable data | R | N |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Check lido program hardcoded address
- Check lido program data matches that on lido program
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
| state | The LidoCalculatorState singleton PDA | W | N |

##### Procedure

- Check state PDA
- Check manager pubkey and signature
- Write new_manager to state

#### Init

Initialize LidoCalculatorState, can only be called once.

#### Data

| Name | Value | Type |
| -- | -- | -- |
| discriminant | 255 | u8 |

##### Accounts

| Account | Description | Read/Write (R/W) | Signer (Y/N) |
| -- | -- | -- | -- |
| state | The LidoCalculatorState singleton PDA | W | N |

##### Procedure

- Initialize state
- Set manager to initial hardcoded manager
- Set last_upgrade_slot to 0
