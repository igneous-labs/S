# Instructions

Controller program's instructions.

For more information about the pricing programs CPIs, see [interface doc](/docs/pricing-programs/)

For more information about the SOL value calculator programs CPIs, see [interface doc](/docs/sol-value-calculator-programs/)

## SyncSolValue

Permissionless crank to update and record the SOL value of one of the pool's LST reserves.

### Data

| Name         | Value                                | Type |
| ------------ | ------------------------------------ | ---- |
| discriminant | 0                                    | u8   |
| lst_index    | index of the LST in `lst_state_list` | u32  |

### Accounts

| Account             | Description                                                                                                                                                                               | Read/Write (R/W) | Signer (Y/N) |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| lst                 | Mint of the LST to sync SOL value for                                                                                                                                                     | R                | N            |
| pool_state          | The pool's state singleton PDA                                                                                                                                                            | W                | N            |
| lst_state_list      | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                    | W                | N            |
| pool_reserves       | LST reserves token account of the pool                                                                                                                                                    | R                | N            |
| lst_value_calc_accs | Accounts to invoke token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |

### Procedure

- Verify pool is not rebalancing and not disabled
- Verify index
- CPI the LST's SOL value calculator program LstToSol
- Update pool_state's sol_value by subtracting LST's old SOL value and adding newly returned SOL value
- Record returned SOL value in pool_state

## SwapExactIn

Swap to output LST from an exact amount of given input LST.

### Data

| Name                    | Value                                                                                                                                                                                                     | Type |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| discriminant            | 1                                                                                                                                                                                                         | u8   |
| src_lst_value_calc_accs | number of accounts following dst_lst_acc to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself | u8   |
| dst_lst_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself             | u8   |
| pricing_accs            | number of accounts following to invoke pricing program PriceExactIn with, including the program itself                                                                                                    | u8   |
| src_lst_index           | index of src_lst in `lst_state_list`                                                                                                                                                                      | u32  |
| dst_lst_index           | index of dst_lst in `lst_state_list`                                                                                                                                                                      | u32  |
| amount                  | amount of src tokens to swap                                                                                                                                                                              | u64  |

### Accounts

| Account                  | Description                                                                                                                                                                                   | Read/Write (R/W) | Signer (Y/N) |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| signer                   | Authority of src_lst_acc. User making the swap.                                                                                                                                               | R                | Y            |
| src_lst                  | Mint of the LST being swapped from                                                                                                                                                            | R                | N            |
| dst_lst                  | Mint of the LST being swapped to                                                                                                                                                              | R                | N            |
| src_lst_acc              | LST token account being swapped from                                                                                                                                                          | W                | N            |
| dst_lst_acc              | LST token account to swap to                                                                                                                                                                  | W                | N            |
| protocol_fee_accumulator | Protocol fee accumulator token account for dst_lst                                                                                                                                            | W                | N            |
| src_lst_token_program    | Source LST token program                                                                                                                                                                      | R                | N            |
| dst_lst_token_program    | Destination LST token program                                                                                                                                                                 | R                | N            |
| pool_state               | The pool's state singleton PDA                                                                                                                                                                | W                | N            |
| lst_state_list           | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                        | W                | N            |
| src_pool_reserves        | Source LST reserves token account of the pool                                                                                                                                                 | W                | N            |
| dst_pool_reserves        | Destination LST reserves token account of the pool                                                                                                                                            | W                | N            |
| src_lst_value_calc_accs  | Accounts to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |
| dst_lst_value_calc_accs  | Accounts to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |
| pricing_accs             | Accounts to invoke pricing program PriceExactIn with. First account should be the pricing program itself. Multiple Accounts.                                                                  | ...              | ...          |

### Procedure

- Verify pool is not rebalancing and not disabled
- Verify input not disabled for src_lst
- Self CPI SyncSolValue for src_lst
- Self CPI SyncSolValue for dst_lst
- CPI src token's SOL value calculator program LstToSol to get SOL value of input amount
- CPI pricing program PriceExactIn to get output SOL value
- Apply protocol fees to fee amount = input SOL value - output SOL value
- CPI dst token's SOL value calculator program SolToLst with protocol fees SOL value to get protocol fees amount
- CPI dst token's SOL value calculator program SolToLst with output SOL value to get output token amount
- Transfer input amount src tokens from src_lst_acc to src token reserves
- Transfer protocol fees to protocol_fee_accumulator
- Transfer remaining output dst tokens from dst token reserves to dst_lst_acc
- Self CPI SyncSolValue for src_lst
- Self CPI SyncSolValue for dst_lst

## SwapExactOut

Swap to an exact amount of output LST from input LST.

Same as [SwapExactIn](#swapexactin-instruction), but:

- discriminator = 2
- amount is amount of dst tokens to receive
- the core part goes like this instead:
  - CPI dst token's SOL value calculator program LstToSol to get SOL value of output amount
  - CPI pricing program PriceExactOut to get input SOL value
  - CPI src token's SOL value calculator program SolToLst with input SOL value to get input token amount

Note protocol fees are always levied on dst_lst

## AddLiquidity

Add single-LST liquidity to the pool.

### Data

| Name                | Value                                                                                                                                                                                              | Type |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| discriminant        | 3                                                                                                                                                                                                  | u8   |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. | u8   |
| pricing_accs        | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself                                                                                      | u8   |
| lst_index           | index of lst in `lst_state_list`                                                                                                                                                                   | u32  |
| amount              | amount of tokens to add as liquidity                                                                                                                                                               | u64  |

### Accounts

| Account             | Description                                                                                                                                                                               | Read/Write (R/W) | Signer (Y/N) |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| signer              | Authority of lst_acc. User who's adding liquidity.                                                                                                                                        | R                | Y            |
| lst                 | Mint of the LST                                                                                                                                                                           | R                | N            |
| src_lst_acc         | LST token account to add liquidity from                                                                                                                                                   | W                | N            |
| dst_lp_acc          | LP token account to mint new LP tokens to                                                                                                                                                 | W                | N            |
| token_program       | Token program                                                                                                                                                                             | R                | N            |
| pool_state          | The pool's state singleton PDA                                                                                                                                                            | W                | N            |
| lst_state_list      | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                    | W                | N            |
| pool_reserves       | LST reserves token account of the pool                                                                                                                                                    | W                | N            |
| lst_value_calc_accs | Accounts to invoke token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |
| pricing_accs        | Accounts to invoke pricing program PriceLpTokensToMint with. First account should be the pricing program itself. Multiple Accounts.                                                       | ...              | ...          |

### Procedure

- Verify pool is not rebalancing and not disabled
- Verify input not disabled for LST
- Self CPI SyncSolValue for LST
- CPI LST's SOL value calculator program LstToSol to get SOL value of amount to add
- CPI pricing program's PriceLpTokensToMint to get SOL value of LP tokens to mint
- Transfer amount from src_lst_acc to pool_reserves
- Mint LP tokens proportional to SOL value of LP tokens to mint to dst_lp_token_acc
- Self CPI SyncSolValue for LST

## RemoveLiquidity

Remove single-LST liquidity from the pool.

### Data

| Name                | Value                                                                                                                                                                                             | Type |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| discriminant        | 4                                                                                                                                                                                                 | u8   |
| lst_value_calc_accs | number of accounts following to invoke the input LST's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself | u8   |
| pricing_accs        | number of accounts following to invoke pricing program PriceLpTokensToMint with, including the program itself                                                                                     | u8   |
| lst_index           | index of lst in `lst_state_list`                                                                                                                                                                  | u32  |
| amount              | amount of LP tokens to burn and redeem                                                                                                                                                            | u64  |

### Accounts

| Account                  | Description                                                                                                                                                                               | Read/Write (R/W) | Signer (Y/N) |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| signer                   | Authority of lp_acc. User who's removing liquidity.                                                                                                                                       | R                | Y            |
| lst                      | Mint of the LST                                                                                                                                                                           | R                | N            |
| dst_lst_acc              | LST token account to redeem to                                                                                                                                                            | W                | N            |
| src_lp_acc               | LP token account to burn LP tokens from                                                                                                                                                   | W                | N            |
| protocol_fee_accumulator | Protocol fee accumulator token account                                                                                                                                                    | W                | N            |
| token_program            | Token program                                                                                                                                                                             | R                | N            |
| pool_state               | The pool's state singleton PDA                                                                                                                                                            | W                | N            |
| lst_state_list           | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                    | W                | N            |
| pool_reserves            | LST reserves token account of the pool                                                                                                                                                    | W                | N            |
| lst_value_calc_accs      | Accounts to invoke token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |
| pricing_accs             | Accounts to invoke pricing program PriceLpTokensToRedeem with. First account should be the pricing program itself. Multiple Accounts.                                                     | ...              | ...          |

### Procedure

- Verify pool is not rebalancing and not disabled
- Self CPI SyncSolValue for LST
- CPI pricing program's PriceLpTokensToRedeem with SOL value of LP tokens to be burnt
- CPI LST's SOL value calculator program SolToLst to get amount of LST due
- Burn LP tokens
- Subtract and transfer protocol fees
- Transfer remaining LST due to dst_acc
- Self CPI SyncSolValue for LST

## DisableLstInput

Disable input for a LST to prepare for removal

### Data

| Name         | Value                            | Type |
| ------------ | -------------------------------- | ---- |
| discriminant | 5                                | u8   |
| index        | index of lst in `lst_state_list` | u32  |

### Accounts

| Account        | Description                                            | Read/Write (R/W) | Signer (Y/N) |
| -------------- | ------------------------------------------------------ | ---------------- | ------------ |
| admin          | The pool's admin                                       | R                | Y            |
| lst            | Mint of the LST to disable input for                   | R                | N            |
| pool_state     | The pool's state singleton PDA                         | W                | N            |
| lst_state_list | Dynamic list PDA of LstStates for each LST in the pool | W                | N            |

## EnableLstInput

Re-enable input for a LST

### Data

| Name         | Value                            | Type |
| ------------ | -------------------------------- | ---- |
| discriminant | 6                                | u8   |
| index        | index of lst in `lst_state_list` | u32  |

### Accounts

| Account        | Description                                            | Read/Write (R/W) | Signer (Y/N) |
| -------------- | ------------------------------------------------------ | ---------------- | ------------ |
| admin          | The pool's admin                                       | R                | Y            |
| lst            | Mint of the LST to re-enable input for                 | R                | N            |
| pool_state     | The pool's state singleton PDA                         | W                | N            |
| lst_state_list | Dynamic list PDA of LstStates for each LST in the pool | W                | N            |

## AddLst

Add a LST to the pool

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 7     | u8   |

### Accounts

| Account                       | Description                                                                    | Read/Write (R/W) | Signer (Y/N) |
| ----------------------------- | ------------------------------------------------------------------------------ | ---------------- | ------------ |
| admin                         | The pool's admin                                                               | R                | Y            |
| payer                         | Account paying the SOL rent for the new space and accounts                     | W                | Y            |
| lst                           | Mint of the new LST to add                                                     | R                | N            |
| pool_reserves                 | LST reserves token account to create                                           | W                | N            |
| protocol_fee_accumulator      | The LST protocol fee accumulator token account to create                       | W                | N            |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W                | N            |
| sol_value_calculator          | The LST's SOL value calculator program                                         | R                | N            |
| pool_state                    | The pool's state singleton PDA                                                 | R                | N            |
| lst_state_list                | Dynamic list PDA of LstStates for each LST in the pool                         | W                | N            |
| system_program                | System program                                                                 | R                | N            |

### Procedure

- Verify pool is not rebalancing and not disabled
- Create reserves token account
- Create protocol_fee_accumulator token account
- Reallocate additional space for an additional LstState on `lst_state_list`
- Write initial SOL value = 0 and sol_value_calculator program

## RemoveLst

Remove a LST from the pool

### Data

| Name         | Value                            | Type |
| ------------ | -------------------------------- | ---- |
| discriminant | 8                                | u8   |
| index        | index of lst in `lst_state_list` | u32  |

### Accounts

| Account                       | Description                                                                    | Read/Write (R/W) | Signer (Y/N) |
| ----------------------------- | ------------------------------------------------------------------------------ | ---------------- | ------------ |
| admin                         | The pool's admin                                                               | R                | Y            |
| refund_rent_to                | The account to refund SOL rent to                                              | W                | N            |
| lst                           | Mint of the LST to remove                                                      | R                | N            |
| pool_reserves                 | LST reserves token account to destroy                                          | W                | N            |
| protocol_fee_accumulator      | The LST protocol fee accumulator token account to destroy                      | W                | N            |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W                | N            |
| pool_state                    | The pool's state singleton PDA                                                 | R                | N            |
| lst_state_list                | Dynamic list PDA of LstStates for each LST in the pool                         | W                | N            |
| system_program                | System program                                                                 | R                | N            |

### Procedure

- Verify pool is not rebalancing and not disabled
- Delete reserves token account
- Delete protocol_fee_accumulator token account
- Remove LstState from list and reallocate to smaller space

## SetSolValueCalculator

Update the SOL value calculator program for a LST

### Data

| Name         | Value                            | Type |
| ------------ | -------------------------------- | ---- |
| discriminant | 9                                | u8   |
| index        | index of lst in `lst_state_list` | u32  |

### Accounts

| Account             | Description                                                                                                                                              | Read/Write (R/W) | Signer (Y/N) |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| admin               | The pool's admin                                                                                                                                         | R                | Y            |
| lst                 | Mint of the LST to remove                                                                                                                                | R                | N            |
| pool_state          | The pool's state singleton PDA                                                                                                                           | R                | N            |
| pool_reserves       | LST reserves token account of the pool                                                                                                                   | R                | N            |
| lst_state_list      | Dynamic list PDA of LstStates for each LST in the pool                                                                                                   | W                | N            |
| lst_value_calc_accs | Accounts to invoke token's new SOL value calculator program LstToSol with. First account should be the new calculator program itself. Multiple Accounts. | ...              | ...          |

### Procedure

- Overwrite sol_value_calculator in `lst_state_list`
- Self CPI SyncSolValue

## SetAdmin

Updates the admin authority pubkey of the pool

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 10    | u8   |

### Accounts

| Account       | Description                    | Read/Write (R/W) | Signer (Y/N) |
| ------------- | ------------------------------ | ---------------- | ------------ |
| current_admin | The pool's current admin       | R                | Y            |
| new_admin     | The pool's new admin           | R                | N            |
| pool_state    | The pool's state singleton PDA | W                | N            |

## SetProtocolFee

Updates the protocol fee rate of the pool

### Data

| Name                 | Value | Type |
| -------------------- | ----- | ---- |
| discriminant         | 11    | u8   |
| new_protocol_fee_bps | -     | u16  |

### Accounts

| Account    | Description                    | Read/Write (R/W) | Signer (Y/N) |
| ---------- | ------------------------------ | ---------------- | ------------ |
| admin      | The pool's admin               | R                | Y            |
| pool_state | The pool's state singleton PDA | W                | N            |

## SetProtocolFeeBeneficiary

Updates the pool's protocol fee beneficiary

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 12    | u8   |

### Accounts

| Account             | Description                                 | Read/Write (R/W) | Signer (Y/N) |
| ------------------- | ------------------------------------------- | ---------------- | ------------ |
| current_beneficiary | The pool's current protocol fee beneficiary | R                | Y            |
| new_beneficiary     | The pool's new protocol fee beneficiary     | R                | N            |
| pool_state          | The pool's state singleton PDA              | W                | N            |

## SetPricingProgram

Updates the pool's pricing program.

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 13    | u8   |

### Accounts

| Account             | Description                    | Read/Write (R/W) | Signer (Y/N) |
| ------------------- | ------------------------------ | ---------------- | ------------ |
| admin               | The pool's admin               | R                | Y            |
| new_pricing_program | The pool's new pricing program | R                | N            |
| pool_state          | The pool's state singleton PDA | W                | N            |

## WithdrawProtocolFees

Withdraw all accumulated protocol fees. Only the protocol_fee_beneficiary is authorized to call this.

### Data

| Name         | Value                     | Type |
| ------------ | ------------------------- | ---- |
| discriminant | 14                        | u8   |
| amount       | amount of LST to withdraw | u64  |

### Accounts

| Account                       | Description                                                                    | Read/Write (R/W) | Signer (Y/N) |
| ----------------------------- | ------------------------------------------------------------------------------ | ---------------- | ------------ |
| protocol_fee_beneficiary      | The pool's protocol fee beneficiary                                            | R                | Y            |
| withdraw_to                   | Token account to withdraw all accumulated protocol fees to                     | W                | N            |
| protocol_fee_accumulator      | The LST protocol fee accumulator token account to withdraw from                | W                | N            |
| protocol_fee_accumulator_auth | The protocol fee accumulator token account authority PDA. PDA ["protocol_fee"] | W                | N            |
| token_program                 | Token program                                                                  | R                | N            |
| pool_state                    | The pool's state singleton PDA                                                 | W                | N            |

## AddDisablePoolAuthority

Add a disable pool authority

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 15    | u8   |

### Accounts

| Account                     | Description                                          | Read/Write (R/W) | Signer (Y/N) |
| --------------------------- | ---------------------------------------------------- | ---------------- | ------------ |
| payer                       | Account paying for additional rent for realloc       | W                | Y            |
| admin                       | The pool's admin                                     | R                | Y            |
| new_authority               | The new disable pool authority to add                | R                | N            |
| disable_pool_authority_list | The pool's disable pool authority list singleton PDA | W                | N            |
| system_program              | System program                                       | R                | N            |

### Procedure

- realloc and extend disable_pool_authority_list, and write new_authority in

## RemoveDisablePoolAuthority

Remove a disable pool authority

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 16    | u8   |

### Accounts

| Account                     | Description                                          | Read/Write (R/W) | Signer (Y/N)                     |
| --------------------------- | ---------------------------------------------------- | ---------------- | -------------------------------- |
| refund_rent_to              | The account to refund SOL rent to after resizing     | W                | N                                |
| admin                       | The pool's admin                                     | R                | Y if authority signature missing |
| authority                   | The authority to remove                              | R                | Y if admin signature missing     |
| disable_pool_authority_list | The pool's disable pool authority list singleton PDA | W                | N                                |

### Procedure

- rewrite array and resize list down

## DisablePool

Disable functionality of the entire pool.

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 17    | u8   |

### Accounts

| Account                     | Description                                                                                | Read/Write (R/W) | Signer (Y/N) |
| --------------------------- | ------------------------------------------------------------------------------------------ | ---------------- | ------------ |
| authority                   | The pool's admin or a disable pool authority                                               | R                | Y            |
| pool_state                  | The pool's state singleton PDA                                                             | W                | N            |
| disable_pool_authority_list | The pool's disable pool authority list singleton PDA. Optional if authority = pool's admin | R                | N            |

### Procedure

- set bool flag on pool_state

## EnablePool

Re-enable functionality of the entire pool.

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 18    | u8   |

### Accounts

| Account    | Description                    | Read/Write (R/W) | Signer (Y/N) |
| ---------- | ------------------------------ | ---------------- | ------------ |
| admin      | The pool's admin               | R                | Y            |
| pool_state | The pool's state singleton PDA | W                | N            |

### Procedure

- unset bool flag on pool_state

## StartRebalance

Start a flash rebalancing procedure to rebalance from one LST type into another without causing a decrease in pool SOL value

### Data

| Name                    | Value                                                                                                                                                                                                     | Type |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| discriminant            | 19                                                                                                                                                                                                        | u8   |
| src_lst_value_calc_accs | number of accounts following dst_lst_acc to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself | u8   |
| dst_lst_value_calc_accs | number of accounts following to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself             | u8   |
| src_lst_index           | index of src_lst in `lst_state_list`                                                                                                                                                                      | u32  |
| dst_lst_index           | index of dst_lst in `lst_state_list`                                                                                                                                                                      | u32  |
| amount                  | amount of from_lst tokens to flash withdraw to rebalance                                                                                                                                                  | u64  |

### Accounts

| Account                 | Description                                                                                                                                                                                   | Read/Write (R/W) | Signer (Y/N) |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| payer                   | Account paying the 1 lamport rent for RebalanceRecord                                                                                                                                         | W                | Y            |
| rebalance_authority     | The pool's rebalance authority                                                                                                                                                                | R                | Y            |
| pool_state              | The pool's state singleton PDA                                                                                                                                                                | W                | N            |
| lst_state_list          | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                        | W                | N            |
| rebalance_record        | The RebalanceRecord PDA                                                                                                                                                                       | W                | N            |
| src_lst                 | Mint of the LST to rebalance from                                                                                                                                                             | R                | N            |
| dst_lst                 | Mint of the LST to rebalance to                                                                                                                                                               | R                | N            |
| src_pool_reserves       | Source LST reserves token account of the pool                                                                                                                                                 | W                | N            |
| dst_pool_reserves       | Destination LST reserves token account of the pool                                                                                                                                            | W                | N            |
| withdraw_to             | Source LST token account to withdraw to                                                                                                                                                       | W                | N            |
| instructions            | Instructions sysvar                                                                                                                                                                           | R                | N            |
| system_program          | System program                                                                                                                                                                                | R                | N            |
| src_lst_value_calc_accs | Accounts to invoke src token's SOL value calculator program LstToSol with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |
| dst_lst_value_calc_accs | Accounts to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |

### Procedure

- Verify pool is not rebalancing and not disabled
- Verify input is not disabled for dst_lst
- Verify a corresponding EndRebalance instruction follows
- Self CPI SyncSolValue for dst_lst
- Self CPI SyncSolValue for src_lst
- Withdraw amount src_lst from reserves to withdraw_to
- Self CPI SyncSolValue for src_lst
- Initialize rebalance_record with sol_value = the difference between pool's total SOL value before and after the second SyncSolValue for src_lst
- Set is_rebalancing = true

## EndRebalance

End a flash rebalancing procedure after returning the funds to the pool

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 20    | u8   |

### Accounts

| Account                 | Description                                                                                                                                                                                   | Read/Write (R/W) | Signer (Y/N) |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ------------ |
| refund_rent_to          | The account to refund the 1 lamport rent for RebalanceRecord                                                                                                                                  | W                | N            |
| pool_state              | The pool's state singleton PDA                                                                                                                                                                | W                | N            |
| lst_state_list          | Dynamic list PDA of LstStates for each LST in the pool                                                                                                                                        | W                | N            |
| rebalance_record        | The RebalanceRecord PDA                                                                                                                                                                       | W                | N            |
| dst_lst                 | Mint of the LST to rebalance to                                                                                                                                                               | R                | N            |
| dst_pool_reserves       | Destination LST reserves token account of the pool                                                                                                                                            | R                | N            |
| dst_lst_value_calc_accs | Accounts to invoke dst token's SOL value calculator program SolToLst with, excluding the interface prefix accounts. First account should be the calculator program itself. Multiple Accounts. | ...              | ...          |

### Procedure

- Verify pool is rebalancing and not disabled
- Set is_rebalancing = false
- Self CPI SyncSolValue for dst_lst
- Verify increase in pool's SOL value after self CPI >= amount recorded in rebalance_record
- Close rebalance_record to refund_rent_to

## SetRebalanceAuthority

Set a new rebalance authority for the pool.

### Data

| Name         | Value | Type |
| ------------ | ----- | ---- |
| discriminant | 21    | u8   |

### Accounts

| Account                 | Description                             | Read/Write (R/W) | Signer (Y/N) |
| ----------------------- | --------------------------------------- | ---------------- | ------------ |
| authority               | The pool's rebalance authority or admin | R                | Y            |
| new_rebalance_authority | The new rebalance authority to set to   | R                | N            |
| pool_state              | The pool's state singleton PDA          | W                | N            |

## Initialize

Initialize the pool. Can only be called once.

### Data

| Name             | Value                | Type |
| ---------------- | -------------------- | ---- |
| discriminant     | 22                   | u8   |
| protocol_fee_bps | initial protocol fee | u16  |

### Accounts

| Account                     | Description                                                                           | Read/Write (R/W) | Signer (Y/N) |
| --------------------------- | ------------------------------------------------------------------------------------- | ---------------- | ------------ |
| payer                       | Account paying for rent                                                               | W                | Y            |
| pool_state                  | The pool's state singleton PDA                                                        | W                | N            |
| lst_state_list              | Dynamic list PDA of LstStates for each LST in the pool                                | W                | N            |
| disable_pool_authority_list | The pool's disable pool authority list singleton PDA                                  | W                | N            |
| token_2022                  | Token 2022 program                                                                    | R                | N            |
| system_program              | System program                                                                        | R                | N            |
| remaining_accounts          | accounts required to initialize the LP token and transfer fee and metadata extensions | ...              | ...          |

### Procedure

- Set all state fields - admin, rebalance_authority, protocol_fee_beneficiary etc to hardcoded defaults
- Create the LP token
