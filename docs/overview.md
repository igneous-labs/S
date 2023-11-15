# Overview

Design overview of the S multi-LST pool.

## Key Concepts

### SOL Value

Since we're dealing exclusively with LSTs, all accounting and calculations are done in SOL terms. 

The SOL value of a LST can be though of as "how much unstaked SOL will I get in the next epoch if I redeem and deactivate the LST right now". This therefore includes the stake pool's withdrawal fees at that instant.

## Programs

### Controller Program

- is the main program - contains all the main human-facing instructions
- has authority over the LST token reserves
- CPIs the other programs below to achieve the pool's functionality

### SOL Value Calculator Programs

Each LST program will have its corresponding SOL value calculator program that:
- calculates SOL value of a given LST amount
- calculates LST amount of a given SOL value

### Pricing Programs

At any time, a single pricing program is active for the pool.

The pricing program determines how much LST should be exchanged for another given their respective SOL values and any other parameters it might require.

This could include pool fee information, pool's target LST allocations, etc.
