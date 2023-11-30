# Risks

Documenting potential risks and exploits.

## LSTs manipulating SOL value by frequently modifying withdrawal fees

Example:

- Malicious Lido operator wants to drain all the stSOL in the pool
- Sets Lido stake wtihdrawal fee to 99%
- Swap from any LST to stSOL to give ~100x the appropriate amount of stSOL
- Set Lido stake withdrawal fee back to 0%
- unstake the 100x stSOL from Lido

## Time Arb

Example:

- Epoch just passed, all other LSTs have ran their crank but Marinade hasn't
- I somehow know Marinade will run their crank on the next block
- Right Before that I swap 100 jitoSOL -> 100 mSOL
- Marinade crank runs, increasing the SOL value of mSOL by 4bps, assuming 0 fees, I swap 100 mSOL -> 100.004 jitoSOL
- Pool has just lost 0.004 jitoSOL
