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

To ensure that the build is as close to the reproducible build as possible, match:

- solana + solana toolchain versions of `ellipsislabs/solana:1.17.6`
- main rust toolchain version to `solana-labs/solana/rust-toolchain.toml`

```sh
sh -c "$(curl -sSfL https://release.solana.com/v1.17.6/install)"
cargo-build-sbf --version && rustc --version

solana-cargo-build-sbf 1.17.6
platform-tools v1.37
rustc 1.68.0 # rust version used by cargo-build-sbf to build the bpf programs .so. solana currently has it locked to 1.68
rustc 1.73.0 (cc66ad468 2023-10-03) # rust version dictated by rust-toolchain.toml, used for building everything else
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

```sh
cargo-test-sbf --features testing
```

## Build

### Verifiable

[Install solana-verify](https://github.com/Ellipsis-Labs/solana-verifiable-build/tree/master#installation)

```sh
solana-verify build
```

## Deploy

### Mainnet Program Hashes

#### S Controller (INF)

```
9bbcaada4b4aa3099023cf551749ddf4c956afa46ceade4ddf772709673b0496
```

#### Flat Fee Pricing Program

```
b09e282d68af7534a46006c8e995f55c2766b4bf7d1d3e08c88abf0b449f72a6
```

#### SPL SOL Value Calculator

```
5acac02e196d9c6130bdd0e8b3b063e9beba22765c02b748b83b648b5ab99c7f
```

#### Sanctum SPL SOL Value Calculator

```
facb55e2af9bcc88a71a2c9d3a661ed29fb15ddd698e3a8ccdc034554416efe8
```

#### Sanctum SPL Multi SOL Value Calculator

```
e8454a6077c84296597df29565bd284c998c0350d36761fcaede0c059b53ad39
```

#### Marinade SOL Value Calculator

```
ab2844122c5abd7eeda53693556e02cff81c5d8afaafeb85e4e7c71919d2fb2e
```

#### Lido SOL Value Calculator

```
1bd4c83cea83719e25aef6ef12ecaec8c055c8d2b17bf45e918b8e8adc5aea87
```

#### wSOL SOL Value Calculator

```
a166b6d886bb7bd8960fb40aa2efc3f923d9db96d83bfcfdb68b398aca6539dc
```
