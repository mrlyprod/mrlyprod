# mrlybang-density-classes

- A mrlybang code is a set `P` of parity corners in `{0,1}^3`; at base `q` it fills the design `F_q(P) = { v in {0..q-1}^3 : v mod 2 in P }`.
- Computes the exact rational part `delta * zeta(3)` of the coprime density of every one of the 255 nonempty codes at every base, as a Mobius bracket times Euler factors.
- Checks the even-base band `(8/7)(1 - t/|P|)` at even `q <= 40` and the same band in `D = 2`, and the odd-base self-similarity `k_e(q) = k(q/e)` at odd `q <= 75`, both domains the widest the pages claim.
- Sorts every code by its mod-2 difference span `H`, groups the codes by weight enumerator, takes the design lattice rank and index at `q = 2, 3, 4, 6, 8, 10`, and finds the codes whose even value equals their odd limit.
- Measures the visible fraction by exact enumeration of all `k^n` level-`n` points, against the limit and against the exact finite-level walk factor `1 - (1/8) Sum_t lambda_t^n`; every enumerated case is the level the page measured.

## RUN

```
uv run python research/lab/mrlybang-density-classes/classes.py
```

Whole run is under half a minute.

## WITNESSES

- `coprime.md:227` `coprime.md:230` - lattice index in `{1, 2, 4, 8}` at even `q = 4, 6, 8, 10`; `151` codes full rank at `q = 2`, index `1` or `2`, the smallest of size `4`.
- `coprime.md:228` `coprime.md:229` - nine band values `0, 4/7, 16/21, 6/7, 32/35, 20/21, 48/49, 1, 8/7`, zero exceptions to `(8/7)(1 - t/|P|)`; carpet `0.712853` and `0.709137`, net `0.951771`, against `0.713063` and `0.950751`.
- `coprime.md:231` - the `D = 2` parity carpet at every even base, `delta * zeta(2) = 8/9`, `delta = 0.5403796460924681 = 16/(3 pi^2)`.
- `coprime.md:233` `coprime.md:234` - zero exceptions to `k_e(q) = k(q/e)`; `63` weight classes, `6` of them mixing regimes, `4` spanning against not and `2` corrected against no-limit.
- `coprime.md:235` - twins `0.707501` and `0.711167` at `q = 4`, splitting at `q = 3` into `0.698204` against `153/182` and `0.783436` against `51/52`.
- `coprime.md:237` - `7` codes of lattice rank `<= 1` at `q = 3`, `37` codes with a coordinate pinned odd, `{110,100}` measuring `0.972222 0.994084 0.999979` against the trichotomy's `0.987319`.
- `coprime.md:238` `coprime.md:239` `coprime.md:241` `coprime.md:245` - class counts `149`, `43`, `63`; the subgroup codes `1 3 5 9 15 17 33 51 65 85 105 129 153 165 195 255`, exactly the parity-stable ones; `239` codes with two values, where the page says `240` and counts the empty code.
- `coprime.md:240` - void `q = 3, n = 8` measures `0.381949` against limit `0.438808` and finite level `0.380043`; tree `0.451821` against `0.452521`, tree `q = 5` `0.468340` against `0.468560`.
- `coprime.md:242` - `{111}` at `q = 5` is exactly `0` at every even level and `0.925781 0.952881 0.957167` against `0.958419`; axes `0.987338 / 0.739563` against `0.987319 / 0.740489`.
- `coprime.md:250` to `coprime.md:259` - the family table `513/520 27/26 99/182 48/91` at `q = 3` up to `200981/201096 1331/1330 9559/16758 16456/28861` at `q = 11`; carpet `0.98654 0.99562 0.99810 0.98959 0.99943` with `k(q) = 20, 81, 208, 425, 756`; net `B(9) = 297/304 = 1 - 7/304` and no odd `q <= 81` below `1`, the page's `20/425` and `1/208` being the carpet's `k`, not the net's.
- `DISCOVERIES.md:33` `DISCOVERIES.md:34` - the band, the lattice indices, the `D = 2` constant, the enumerations `0.712853, 0.951771, 0.709137`; the trichotomy counts at odd `q <= 75`, the axes pair, the `{111}` zeros.
