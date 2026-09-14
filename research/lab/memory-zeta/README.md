# memory-zeta

- The Dirichlet series of a memory rule, `zeta_W(s) = sum_(n in S_W) n^(-s)` over the positive integers whose minimal base-2 string the rule accepts, computed in arbitrary precision as an independent control on the double-precision matrix ladder of `mrlynum::automaton`.
- The rule is the golden one, code `7` at width `2`, forbidding the window `11`, so `S_W` is the fibbinary integers without zero; `A = [[1,1],[1,0]]`, `det(I - x A) = 1 - x - x^2`, and the abscissa is `log_2 phi`.
- The peel: with `E_j(w)` the vector whose entry `u` sums `n^(-w)` over the accepted words of exactly `j` digits ending in state `u`, and `G_P = sum_(j >= P) E_j`, splitting a word on its last digit gives `(I - 2^(-w) A^T) G_P(w) = E_P(w) + sum_(l >= 1) binom(-w,l) 2^(-w-l) Gamma_l G_P(w+l)`, and `zeta_W(w) = 1^T D_(P-1)(w) + 1^T G_P(w)` for the Dirichlet polynomial over the accepted words of at most `P-1` digits.
- The residue at a simple root `x_0` of the determinant is `1^T adj(I - x_0 A^T) N(w_0)` over `-x_0 log 2 det'(x_0)`, for `N` the right side of that identity, so no eigenvector is solved for.
- This is a control and not a hot loop: mpmath at `dps = 40`, one rule, ten evaluations, a closed-form `2 x 2` solve at every level. It shares the mathematics of the crate and none of its arithmetic, running a different peel depth, `P = 14` against the crate's `12`, and a different shift schedule, every argument carried to `Re w = 60` before the head polynomial is summed.
- It carries no bound of its own. At `dps = 40` the seed error is under `2^(-780)` and the printed digits are taken to be exact against `f64`.

## RUN

- `uv run python research/lab/memory-zeta/peel.py` from the repo root, `3.2` seconds. Prints only, writes nothing.

## WITNESSES

- The page lines of [beneath](../../beneath.md), `### The memory zeta`.
- `zeta_W(3) = 1.154012963277642016659466`, `zeta_W(2) = 1.415825532884777929125692`, `zeta_W(0.8) = 9.536379694275011510923898` and `zeta_W(1.2 + 9i) = 1.906409024243906069557735 - 0.453243424778265833646527i`, each met by `mrlynum::automaton` inside its own bound, the largest gap `3.0e-15` against a bound of `9.54e-13`.
- The six residues at `m = 0`, `j = 0, 1, 2` on both combs. On the comb at `Re s = log_2 phi` the gaps against the crate are `6.0e-16`, `2.6e-15` and `9.3e-16` against bounds near `1.3e-13`; on the comb at `Re s = -log_2 phi`, where the crate reads through its left-of-abscissa branch and bounds near `4.1e-9`, the gaps are `1.1e-12`, `3.7e-12` and `4.3e-12`.
- That second comb is the reason this study exists: every crate number on it is produced by one branch and is met here from outside it.
