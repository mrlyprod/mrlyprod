# smith-window

- Builds the even carry core at base 3, odd `D = 2R + 1`, through its symbol rather than its entries: with `P = (1 + t^2)^(D-1)(1 + D t + t^2)` and `H_x = x_0 t^R + sum_(j >= 1) x_j (t^(R+j) + t^(R-j))`, the row at `c'` of `M_even x` is the coefficient of `t^(3 nu + 1)` in `H_x P` at `nu = R - c'`, and `H_x P` is palindromic about `3R + 1`, so the `R + 1` rows are every exponent `== 1 mod 3` in `[0, 6R + 2]`.
- Reads the second 2-adic Smith layer off the same symbol: a mod-2 kernel vector's mod-4 obstruction is `((H_x P)[3 nu + 1] mod 4)/2`, it lifts exactly when that vector lies in the image of the mod-2 symbol on the same coefficient box, and `V_2` is the set of family coordinates `c(z)` that pass.
- Builds the kernel family `H_i = x^s (1 + x^3)^i (1 + x)^(2^b)` with `b = ceil(log_2(3R - 1))` and `i = i_0 + 2j`, `j = 0..K`, and extracts from `V_2` its minimal-degree element `g_D` and its top degree `C_D`.
- Checks seven statements per row. `V_2 = g_D F_2[z]_(<= C_D - deg g_D)`, a divisor-plus-ceiling window. The generator `g_D = z^m c_t(z^(2^e))` with `c_t` the `F_2` Fibonacci polynomials `c_0 = 1`, `c_1 = 1 + y`, `c_t = y c_(t-1) + c_(t-2)` and `2t + 1 = J(k)`, `k + e = b - 1`. The ceiling `C_D = K - 2J(e-1)` when `k` is even and `C_D = K` when `k` is odd, with `L_2 = C_D - deg g_D + 1`. The family shift law `H^(j+1) = psi H^(j) - 2 Z_j` with `psi = t^3 + 2 + t^(-3)` and `Z_j = H^(j) + (t^3 H^(j) AND t^(-3) H^(j))`, together with `Z_j` palindromic and inside the box, `A(Z_j)` in the image, and `ob(X_(j+1)) = Lambda ob(X_j) + A(Z_j)` for `Lambda = S + S^(-1)` folded at the centre. The slot tent identity. The reach law. And the corrector law `C_D - deg g_D = min(K - deg g_D, floor(reach/3))` with `reach = R - jmax`, `jmax` the least index whose columns of the mod-2 symbol already span the generator's obstruction, now read from the closed form rather than from the span test.
- The slot tent. Write `g = |2R - 2^(b-1) - 1|`, `e = min{e >= 1 : J(e) >= (g+1)/2}`, `k = b - 1 - e`, and give the slot its length `N = J(e) - J(e-1)`, its offset `u = (g+1)/2 - J(e-1) - 1` and its position `p = u` above the octave centre `R = 2^(b-2)`, `p = N - 1 - u` below it. The tent identity is `min(p, N - 1 - p) = C_D - deg g_D`: Law E's window length is the distance to the nearer end of the slot, in the slot's own coordinate.
- The reach law. With `w = min(p, N - 1 - p)`, `reach = 3w + 2 [e even] + [k odd] (1 + (p mod 2))`, and `p == R mod 2` whenever `e >= 3`, so the parity term is the parity of `R`. One row per odd octave escapes it, the `e = 1` row above centre, `D = 4^m + 3`, where `reach = 5` for `m >= 2` and `reach = 3` at `D = 7`.
- What the reach law buys. Off the escaping rows `floor(reach/3) = w + [k odd and e even]`, so `min(K - deg g_D, floor(reach/3)) = w = C_D - deg g_D` at every row and the branch of the `min` is a slot statistic and not a measurement: the floor binds strictly iff `k` is even and `e >= 2`, the cap binds strictly iff `k` is odd and `e` is even or `D = 4^m + 3`, and they tie otherwise. Lemma W's `psi`-orbit gives `C_D - deg g_D >= min(K - deg g_D, floor(reach/3))`, so the reach law turns the lower half of the ceiling law into a consequence and leaves one open half: that `z^(w+1) g_D` does not lift.
- Censuses which branch of that `min` binds, and asserts that the floor-strict rows are exactly the rows with `C_D < K`. Prints the residue classes of `R` mod 4 and mod 8 in the swept range.
- Runs the same obstruction with the corrector taken out of the coefficient box, where the image has corank exactly 1 and every family obstruction meets it, so the unboxed layer-2 window is the whole mod-2 kernel.

## RUN

`uv run python research/lab/smith-window/smith_window.py`

About seventy seconds: 799 rows of the default domain `D = 5..1601` plus the unboxed sweep over odd `D = 5..601`, 299 rows. Two integer arguments set a different odd range; `1603 2401` is 400 rows and about eight minutes.

## WITNESSES

- DISCOVERIES, the 2-adic Smith cascade: the layer-2 window structure `V_2 = g_D F_2[z]_(<= C_D - deg g_D)`, the generator `z^m c_t(z^(2^e))` and the ceiling `C_D`, at 1199/1199 rows of odd `D = 5..2401`.
- The family shift law and the exact intertwiner `ob(X_(j+1)) = Lambda ob(X_j) + A(Z_j)`, at the same 1199 rows.
- The slot tent identity and the reach law, at the same 1199 rows and at the far rows `D = 4099` and `D = 16387` of the escaping family; and the corrector law read from the closed form, with the branch census: over all 1199 the floor binds strictly at 448, the cap at 424, and they tie at 327; over the default 799 the split is 216, 258, 325, and over the 800 out-of-sample rows it is 340, 284, 176.
- The tent identity and the closed-form corrector law are pure slot arithmetic once Law E is granted, and hold at 999998/999998 rows of odd `D = 5..2000001`.
- The unboxed image of corank exactly 1 and the collapse `L_2 = L_1`, at 299/299 rows of odd `D = 5..601`.

## NOTE

- The slot, the window bounds and the closed forms are the shelf lane's arithmetic line for line. What is independent here is the object side: the kernel family, the mod-4 symbol, the obstruction, the extraction of `V_2`, and the reach law.
- The reach law was read off the 399 rows `D = 5..801` and never adjusted after. Its out-of-sample support is the 800 rows `D = 803..2401`, swept cold, plus the single rows `D = 4099` and `D = 16387`, and the escaping family `D = 4^m + 3` was visible inside the fit at `D = 7, 19, 67, 259` and predicted at `D = 1027`, `D = 4099` and `D = 16387` before any sweep reached them.
- The `min` in the corrector law was chosen after the `5..401` overshoot, so `5..401` is in sample for it too; with the reach law that `min` is no longer a choice but a computation, and the branch census is predicted from `(e, k)` at every row swept.
- Dropping the `2` from `psi` breaks the shift law at `D = 29, 31, 47, 115, 251`; the pair `(psi, Z_j)` has to move together.
- The unboxed sweep bounds what an untruncated argument can see, and nothing more: corank 1 is the cokernel dimension the one-class window lemma already records.
- Nothing here is fitted. Every printed count is an equality test between two exactly computed objects, and a failure prints its `D`.
