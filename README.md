# Ed25519

Please check the latest version on [GitHub](https://github.com/leo-leesco/Crypto-TD5).

## Build

`cargo build` produces `keygen`, `sign` and `verify` in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`keygen` expects a filename, named `prefix`, and writes to `prefix.sk` and `prefix.pk`.

`sign` and `verify` expect :

- the path to `prefix.sk`
- the path to `datafile`
- the path to `sigfile`

`verify` prints `ACCEPT` or `REJECT` to `stdout`.

## Theoretical framework

### Curve used

Since we implement `Ed25519`, we work in $\mathbb F_p$ where $p=2^{255}-19$.
## Design choices

Note that both operations only implicitly depend on the curve form used through `add` and `double`, but not in the form of the curves used.

### 2-D scalar multiplication : Straus' algorithm

We implement `add` and `double` which allows for `multiexp`.
```pseudo
in: a=0b a_0…a_k
    b=0b b_0…b_k
    P,Q
out: [a]P+[b]Q
(T00,T10,T01,T11)=(0,P,Q,P+Q)
R=0
for i=k..=1 :
  R*=2
  R+=Ta_ib_i
return R
```

### Montgomery ladder-style scalar multiplication

We make use of `multiexp` to reproduce the "Montgomery" `ladder`. More precisely, we work on the ground work laid by [Montgomery curves and their arithmetic](https://inria.hal.science/hal-01483768v1) :
```pseudo
in: k,P
out: [k]P
s=k
Q=P
while 2|s :
  s//=2
  Q*=2
r=floor(s/phi)
return [r]Q+[s-r]Q
```
> phi is the golden ratio

This is indeed in constant time as long as the number of digits used by s is indeed constant.
