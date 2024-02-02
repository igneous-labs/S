# s_controller_interface

## Generate

In project root:

```
solores \
    -o ./generated \
    -z PoolState \
    -z LstState \
    -z RebalanceRecord \
    --solana-program-vers "workspace=true" \
    --borsh-vers "workspace=true" \
    --thiserror-vers "workspace=true" \
    --num-derive-vers "workspace=true" \
    --num-traits-vers "workspace=true" \
    --serde-vers "workspace=true" \
    --bytemuck-vers "workspace=true" \
    idl/s_controller.json
```

Generated with `solores v0.7.0`
