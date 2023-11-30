# sol_value_calculator_interface

Contains the partial IDL for the SOL value calculator interface.

NB: the generated instruction functions are incomplete and not meant to be used directly.

## Generate

In project root:

```
solores \
    -o ./generated/sol-value-calculator-programs \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --serde-vers "workspace=true" \
    idl/sol-value-calculator-programs/interface.json
```
