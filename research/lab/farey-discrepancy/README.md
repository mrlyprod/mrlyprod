# Farey Discrepancy

- The lit nodes of the scale stack are the Farey nodes: the literal reduced lines of every scale `n <= Q`, the sorted window list and the mediant walk are one set, sized by `sum_{k<=Q} phi(k)`, at `Q = 10, 30, 60, 125`.
- Each node `a/b` is drawn by `floor(Q/b)` of those scales, and scale `n` lights `phi(n)` nodes no earlier scale lit.
- The Franel-Landau meter on those nodes in `(0,1]`: with `rho_1 < ... < rho_m` ascending and `delta_j = rho_j - j/m`, the table of `S2*Q`, `S1/sqrt(Q)` and the local exponents `d ln S / d ln Q` over the seven rungs `Q = 125 .. 8000`.
- The sorted window route cross-checked against the mediant walk at the first four rungs.
- Runs the full page domain: the top rung is `Q = 8000` with 19455782 nodes.

## RUN

- `CARGO_BUILD_JOBS=4 cargo run --release --manifest-path research/lab/Cargo.toml -p farey-discrepancy`
- Under a second, 0.7 GB peak; prints only, writes nothing.

## WITNESSES

- farey.md:69-72 the lit nodes are `F_Q`, sized `sum phi(k)`: 32, 278, 1102, 4796 at `Q = 10, 30, 60, 125`.
- farey.md:187-188 brightness is `floor(Q/b)`, on all 278 lit fractions at `Q = 30` and up to `Q = 125`.
- farey.md:99-105 nodes 4796, 19024, 76116, 304192, 1216588, 4863602, 19455782.
- farey.md:99-105 `S2*Q` 0.5395, 0.5848, 0.6241, 0.6387, 0.6560, 0.6538, 0.6564.
- farey.md:99-105 `S1/sqrt(Q)` 0.2040, 0.1942, 0.1852, 0.1634, 0.1512, 0.1314, 0.1123.
- farey.md:99-105 local exponent of `S2` -0.884, -0.906, -0.967, -0.961, -1.005, -0.994.
- farey.md:107-109 `S2*Q` flat near 0.656 and the local exponent of `S1` between +0.274 and +0.432.
