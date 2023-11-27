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

## Overview of User Authorities

The authorities defined by the programs:

| name | count (1 / N)| description |
| - | - | - |
| admin | 1 | The pool's admin |
| rebalancing authority | 1 | The authority that can rebalance the pool's token reserves |
| disable authorities | N | The authority that can disable the pool's operation |
| pricing manager | 1 | The authority that can manage pricing program's state |
| protocol fee beneficiary | 1 | The authority that can receive protocol fee |
| user | N | The normal users (e.g. LPer, swappers) |

## CLI Client

Program intended to be used by the authorities to manage the pool.
