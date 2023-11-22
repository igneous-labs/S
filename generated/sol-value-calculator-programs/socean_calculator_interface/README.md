# socean_calculator_interface

## Generate

In project root:

```
solores \
    -o ./generated/sol-value-calculator-programs \
    -z SoceanCalculatorState \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    --bytemuck-vers "workspace=true" \
    idl/sol-value-calculator-programs/socean_calculator.json
```
