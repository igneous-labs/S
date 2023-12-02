# Sanctum S

## Overview

The programs collectively enable mutl-LST AMM—the "Curve of LSTs" idea—by holding hundreds of LSTs, thus allowing a capital efficient LST-LST swaps.

The development procedure of this repository follows "IDL-first" approach, the idea laid out in [Ideally](https://github.com/igneous-labs/ideally) manifesto.

## On-chain Programs

Sanctum S comprises three main on-chain component programs:

- [S Controller Program](./docs/s-controller-program/)
- [SOL Value Calculator Programs](./docs/sol-value-calculator-programs/)
- [Pricing Programs](./docs/pricing-programs/)

See the overview [here](./docs/).

## Setup

Install solana toolchain 1.16.20

```sh
$ sh -c "$(curl -sSfL https://release.solana.com/v1.16.20/install)"
$ cargo-build-sbf --version
solana-cargo-build-sbf 1.16.20
platform-tools v1.37
rustc 1.68.0
```

## Overview of User Authorities

The authorities defined by the programs:

| name                     | count (1 / N) | description                                                |
| ------------------------ | ------------- | ---------------------------------------------------------- |
| admin                    | 1             | The pool's admin                                           |
| rebalancing authority    | 1             | The authority that can rebalance the pool's token reserves |
| disable authorities      | N             | The authority that can disable the pool's operation        |
| pricing manager          | 1             | The authority that can manage pricing program's state      |
| protocol fee beneficiary | 1             | The authority that can receive protocol fee                |
| user                     | N             | The normal users (e.g. LPer, swappers)                     |

## CLI Client

Program intended to be used by the authorities to manage the pool.

## Testing

### test-sbf

We use a `testing` feature flag for test vs prod env in the following programs:

- s-controller

Unfortunately it seems like `cargo-test-sbf` has some issues with workspaces: running `cargo-test-sbf --features testing` in workspace root results in `error: none of the selected packages contains these features: testing`.

SBF tests for these programs will fail unless you run `cargo-test-sbf --features testing` in the program crate's root directory.
