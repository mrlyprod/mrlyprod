---
title: Progression-free digit designs
lead: A digit design with `0` among its digits is free of three-term progressions exactly when its digit set is and the signed set `F + F - 2F` closes no carry walk, carries living in `{-1, 0, 1}`, and three levels decide; the no-carry case is the digit lift of a progression-free subset of `Z_b`; the census to base `40` by the criterion against brute force finds the half-base construction beaten at bases `19`, `35`, `37` and `39`, every winner a cyclic lift and a carry that leaves `0` never paying, and the dimension `log 2/log 3` of the Stanley sequence first passed at base `37` by ten digits; the planar block of Elsholtz, Hunter, Proske and Sauermann is run as a discrete block problem whose potential is a peeling order, every exact grid optimum bounding every planar block from above, so no planar block has area past `19/49` and the route cannot bring the constant below `2.3381`; the grid search tracks `7/24 + 1/m` to `m = 48` and every grid set found above `7/24` dies under dilation, while the odd grids give `r_3(Z_5^n) >= 10^(n/2)/(27 n + 1)`, below the record `35^(1/3)` for `F_5^n`, and constants `4.36, 6.40, 7.48` at `Z_7^n, Z_11^n, Z_13^n`, above the product base and above `0.54 m`.
figure: research-progressions
slug: progressions
---

# Progression-free digit designs

A digit design with `0` among its digits is free of three-term progressions exactly when its digit set is and the signed set `F + F - 2F` closes no carry walk, carries living in `{-1, 0, 1}`, and three levels decide; the no-carry case is the digit lift of a progression-free subset of `Z_b`; the census to base `40` by the criterion against brute force finds the half-base construction beaten at bases `19`, `35`, `37` and `39`, every winner a cyclic lift and a carry that leaves `0` never paying, and the dimension `log 2/log 3` of the Stanley sequence first passed at base `37` by ten digits; the planar block of Elsholtz, Hunter, Proske and Sauermann is run as a discrete block problem whose potential is a peeling order, every exact grid optimum bounding every planar block from above, so no planar block has area past `19/49` and the route cannot bring the constant below `2.3381`; the grid search tracks `7/24 + 1/m` to `m = 48` and every grid set found above `7/24` dies under dilation, while the odd grids give `r_3(Z_5^n) >= 10^(n/2)/(27 n + 1)`, below the record `35^(1/3)` for `F_5^n`, and constants `4.36, 6.40, 7.48` at `Z_7^n, Z_11^n, Z_13^n`, above the product base and above `0.54 m`.

- [notes/progressions.md](notes/progressions.md): Progression-free digit designs
- [claims/carry-criterion.md](claims/carry-criterion.md): The carry criterion
- [claims/block-area.md](claims/block-area.md): The block area
- [refs.md](refs.md): the references this topic cites
- [lab/rs/carry-census/](lab/rs/carry-census/): The numbers of progressions
- [lab/py/block-search/](lab/py/block-search/): The discrete block problem
- [lab/rs/block-exact/](lab/rs/block-exact/): The exact grid optimum
- [figures/research-progressions.ts](figures/research-progressions.ts): research-progressions
