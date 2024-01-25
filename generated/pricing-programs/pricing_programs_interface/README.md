# pricing_programs_interface

Common interface across pricing programs.

## Generate

In project root:

```sh
solores \
    -o ./generated/pricing-programs \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    idl/pricing-programs/interface.json
```

Generated with solores v0.7.0
