# Overview

Design overview of the S multi-LST pool.

## Key Concepts

### SOL Value

Since we're dealing exclusively with LSTs, all accounting and calculations are done in SOL terms.

The SOL value of a LST can be thought of as "how much unstaked SOL will I get in the next epoch if I redeem and deactivate the LST right now". This therefore includes the stake pool's withdrawal fees at that instant.

## Programs

### Controller Program

- is the main program - contains all the main human-facing instructions
- has authority over the LST token reserves
- CPIs the other programs below to achieve the pool's functionality

### SOL Value Calculator Programs

Each LST program will have its corresponding SOL value calculator program that:

- calculates SOL value of a given LST amount
- calculates LST amount of a given SOL value

[Full interface definition](/docs/sol-value-calculator-programs/README.md)

### Pricing Programs

Separated from the controller program for the sake of keeping separation of concerns.

At any time, a single pricing program is active for the pool.

The pricing program has instructions that:

- determine how much SOL value should be exchanged for a given amount of LST and its SOL value + any other accounts it might require. This could include pool fee information, pool's target LST allocations, etc.
- determine how much SOL value should be redeemed for a given amount of LP tokens given the desired output LST + any other accounts it might require.
- determine how much SOL value should be minted in new LP tokens for a given amount of input LST and its SOL value, + any other accounts it might require.

[Full interface definition](/docs/pricing-programs/README.md)

## General Guidelines

- Protocol fees are always levied on the fees charged, not on the principal amount
