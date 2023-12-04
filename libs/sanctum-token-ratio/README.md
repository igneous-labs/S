# sanctum-token-ratio

Utils for workting with applying ratios to token amounts that can work both onchain and offchain

## Math

### "Inverting" a floor division

Many stake pool convert LST amount to SOL amount by taking `sol_amount = lst_amount * pool_sol / lst_supply`.

Let y = sol_amount, x = lst_amount, n = pool_sol, d = lst_supply.

Given y, n, d, find a suitable value of x

```
y = floor(nx/d)
dy <= nx < d(y + 1)

LHS:
dy/n <= x

RHS:
x < d(y + 1) / n
x < dy/n + d/n

d/n <= 1,
x = ceil(dy/n) is a possible candidate, error at most 1 if RHS doesnt hold

When RHS doesn't hold:
Let r = dy%n, p be unknown

           | r/n |  d/n   | p |
floor(dy/n)     dy/n           ceil(dy/n)

p = 1 - r/n - d/n
  = (n-r-d)/n

Round to floor if r/n <= p , else ceil

If r/n <= p:
r <= n-r-d
2r <= n - d
```

### "Inverting" a fee charge

Many stake pools charge a percentage fee on stuff by taking `fee_amount = amount * fee_numerator / fee_denominator, output_amount = amount - fee_amount`.

Let y = output_amount, x = amount, n = fee_numerator, d = fee_denominator

Given y, n, d, find a suitable value of x

```
y = x - floor(nx/d)

floor(nx/d) = x - y
x-y <= nx/d < x-y+1

LHS:
x-y <= nx/d
dx-dy <= nx
dx - nx <= dy
x(d-n) <= dy
x <= dy/(d-n)

RHS:
nx/d < x-y+1
nx < dx-dy+d
dy-d < dx-nx
dy-d < x(d-n)
dy/(d-n) - d/(d-n) < x

d/(d-n) >= 1,
x = floor(dy/(d-n)) is always a possible candidate
```
