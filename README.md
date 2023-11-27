# Sanctum S

## Overview

The programs collectively enable mutl-LST AMM—the "Curve of LSTs" idea—by holding hundreds of LSTs, thus allowing a capital efficient LST-LST swaps.

The development procedure of this repository follows "IDL-first" approach, the idea laid out in [Ideally](https://github.com/igneous-labs/ideally) manifesto.

## On-chain Programs

Sanctum S comprises three main on-chain component programs:
 - [S Controller Program](./docs/s-controller-program/README.md)
 - [SOL Value Calculator Programs](./docs/sol-value-calculator-programs/README.md)
 - [Pricing Programs](./docs/pricing-programs/README.md)

## Overview of User Authorities

The authorities defined by the programs:

| name | count (1 / N)| description |
| - | - | - |
| admin | 1 | The pool's admin |
| disable authorities | N | The pool's disable authority |
| pricing manager | 1 | The manager of a pricing program |
| user | N | The normal users (e.g. LPer, swappers) |

## CLI Client for Admin

Program intended to be used by the admin to manage the pool.
