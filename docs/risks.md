# Risks

Documenting potential risks and exploits.

## LSTs manipulating SOL value by frequently modifying withdrawal fees

Example:

- Malicious Lido operator wants to drain all the stSOL in the pool
- Sets Lido stake wtihdrawal fee to 99%
- Swap from any LST to stSOL to give ~100x the appropriate amount of stSOL
- Set Lido stake withdrawal fee back to 0%
- unstake the 100x stSOL from Lido

### Mitigation

- SPL has a minimal one epoch delay and maximum 2x increase to withdrawal fees change, giving some leeway to observe and prepare for such behaviour.
- Admin must constantly monitor and vet stake pools and LSTs that are allowed into the pool.

## Swap Time Arb

Example:

- Epoch just passed, all other LSTs have ran their crank but Marinade hasn't
- I somehow know Marinade will run their crank on the next block
- Right before that I swap 100 jitoSOL -> 100 mSOL
- Marinade crank runs, increasing the SOL value of mSOL by 4bps, assuming 0 fees, I swap 100 mSOL -> 100.004 jitoSOL
- Pool has just lost 0.004 jitoSOL to me

### Mitigation

- Swap fees must be enough to offset such potential losses

## LP Time Arb

Example:

- Similar scenario as in [Swap Time Arb](#swap-time-arb)
- Right before Marinade crank runs I add liquidity
- Marinade cranks runs, my LP tokens have increased in SOL value
- Remove liquidity. I've just made marinade's staking gains without waiting for stake warm-up

### Mitigation

- LP withdrawal fees must be enough to offset such potential losses
