# flat_fee_interface

## Generate

In project root:
```
solores \
    -o ./generated/pricing-programs \
    -z ProgramState \
    -z FeeAccount \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    --bytemuck-vers "workspace=true" \
    idl/pricing-programs/flat_fee.json
```

Generated with solores v0.7.0
