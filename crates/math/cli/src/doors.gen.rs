use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "math.arcs.curves",
        signature: "(code: Code, base: usize, level: usize) -> [math.arcs.Curve]",
        docs: "Walks level `level` of `bang dim 2, base b, code c` in arcs into its curves: first the strands, from each boundary midpoint in turn, across edges before upright ones, then the loops, each from its first unwalked arc.\n\nThe corner of an arc is its cell's lower-left or upper-right for a filled cell, lower-right or upper-left for a deleted one, as in `draw`.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a side past the machine word.",
        uncallable: None,
        door: Some(door_math_arcs_curves),
    },
    Function {
        name: "math.arcs.draw",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Arcs",
        docs: "Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by `crate::bang::factory::create`, and level 0 is one filled cell.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a side past the machine word.",
        uncallable: None,
        door: Some(door_math_arcs_draw),
    },
    Function {
        name: "math.arcs.law",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Law?",
        docs: "Returns the proved loop law of `bang dim 2, base b, code c` at the level, or none where no law is proved: the carpet, base 3 code 495, has `(8^n - 1)/7 - 3^n + n + 1` loops; at base 2 codes 7 and 14 have `3^(n-1) - 2^n + 1`, codes 11 and 13 have `3^(n-1) - 2^(n-1)`, both from level 1 on, code 9 has `2^n - 1`, and the other eleven codes never loop.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a count past 64 bits.",
        uncallable: None,
        door: Some(door_math_arcs_law),
    },
    Function {
        name: "math.arcs.trace",
        signature: "(side: usize, on: [bool]) -> math.arcs.Arcs",
        docs: "Draws a `side x side` grid of filled and deleted cells in arcs and counts its curves by union-find over the edge midpoints, a curve being a loop when no midpoint of it lies on the boundary.\n\n# Errors\n\nErrors on side zero or a cell list that is not `side^2` long.",
        uncallable: None,
        door: Some(door_math_arcs_trace),
    },
    Function {
        name: "math.arrays.essential",
        signature: "(sensors: [u64]) -> [bool]",
        docs: "Returns whether each sensor is essential, in the order given: whether removing it deletes a lag from the coarray, which happens exactly when some lag `t != 0` has all its pairs through it, the one pair of a lag of weight `1` or the two pairs `(s, s - t)` and `(s + t, s)` of a lag of weight `2`.",
        uncallable: None,
        door: Some(door_math_arrays_essential),
    },
    Function {
        name: "math.arrays.fractal",
        signature: "(generator: [u64], base: u64, level: u32) -> [u64]",
        docs: "Returns the fractal array of the generator at the base and level, ascending: every `sum_(i < level) g_i base^i` with each digit `g_i` in the generator.\n\nThe generator is the digit set of a dim 1 digit design, its least digit `0`; at base `2a + 1`, `a` its largest digit, this is the fractal array of Cohen and Eldar, and every base above `a` gives `L^level` distinct sensors, `L` the size of the generator.\n\n# Errors\n\nErrs when the generator is empty, misses `0`, repeats a digit or holds one at or above the base, or when the array passes `MOST` sensors.",
        uncallable: None,
        door: Some(door_math_arrays_fractal),
    },
    Function {
        name: "math.arrays.holes",
        signature: "(sensors: [u64]) -> [i64]",
        docs: "Returns the holes of the coarray, ascending: the lags between `-span` and `span` that no pair of sensors makes.",
        uncallable: None,
        door: Some(door_math_arrays_holes),
    },
    Function {
        name: "math.arrays.law",
        signature: "(generator: [u64], level: u32) -> (u64, u64)?",
        docs: "Returns the essential count and the sensor count that Theorem A of the arrays note proves for the fractal array of the generator at base `2a + 1`, `(u^level, L^level)`, `u` the count of `paired` digits; `None` off the theorem: a level below 2, a generator without `0`, or a generator whose coarray has a hole.",
        uncallable: None,
        door: Some(door_math_arrays_law),
    },
    Function {
        name: "math.arrays.lost",
        signature: "(sensors: [u64], sensor: u64) -> [i64]",
        docs: "Returns the lags that removing the sensor deletes from the coarray, ascending: those every pair of which holds it; empty when the sensor is inessential or absent.",
        uncallable: None,
        door: Some(door_math_arrays_lost),
    },
    Function {
        name: "math.arrays.paired",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "Returns the paired sensors, ascending: each `g` with a partner `h` whose lag `g - h` has weight `1`, the set `U` of condition C1 of Cohen and Eldar; a paired sensor is essential.",
        uncallable: None,
        door: Some(door_math_arrays_paired),
    },
    Function {
        name: "math.arrays.weights",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "Returns the coarray weights of the sensors, `w(t)` for every lag `t` from `-span` to `span` at index `t + span`, `span` the largest sensor less the least: the count of ordered pairs `(p, q)` of sensors with `p - q = t`.",
        uncallable: None,
        door: Some(door_math_arrays_weights),
    },
    Function {
        name: "math.atoms.carpet_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n carpet, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_2d),
    },
    Function {
        name: "math.atoms.carpet_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n carpet, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_3d),
    },
    Function {
        name: "math.atoms.carpet_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a carpet of the given side at any rank, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_nd),
    },
    Function {
        name: "math.atoms.dust_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n dust, on where both coordinates are even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_2d),
    },
    Function {
        name: "math.atoms.dust_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n dust, on where all three coordinates are even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_3d),
    },
    Function {
        name: "math.atoms.dust_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a dust of the given side at any rank, on where every coordinate is even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_nd),
    },
    Function {
        name: "math.atoms.hline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n line, free on axis 1, on along the odd rows.",
        uncallable: None,
        door: Some(door_math_atoms_hline_2d),
    },
    Function {
        name: "math.atoms.htree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tree, free on axis 1, on along the even rows.",
        uncallable: None,
        door: Some(door_math_atoms_htree_2d),
    },
    Function {
        name: "math.atoms.line_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.",
        uncallable: None,
        door: Some(door_math_atoms_line_nd),
    },
    Function {
        name: "math.atoms.net_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n net, on where at least one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_net_2d),
    },
    Function {
        name: "math.atoms.net_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n net, on where at least two coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_net_3d),
    },
    Function {
        name: "math.atoms.net_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.",
        uncallable: None,
        door: Some(door_math_atoms_net_nd),
    },
    Function {
        name: "math.atoms.noise_2d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.",
        uncallable: None,
        door: Some(door_math_atoms_noise_2d),
    },
    Function {
        name: "math.atoms.noise_3d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.",
        uncallable: None,
        door: Some(door_math_atoms_noise_3d),
    },
    Function {
        name: "math.atoms.ones_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tensor of ones.",
        uncallable: None,
        door: Some(door_math_atoms_ones_2d),
    },
    Function {
        name: "math.atoms.ones_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tensor of ones.",
        uncallable: None,
        door: Some(door_math_atoms_ones_3d),
    },
    Function {
        name: "math.atoms.point_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n point, on where both coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_2d),
    },
    Function {
        name: "math.atoms.point_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n point, on where all three coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_3d),
    },
    Function {
        name: "math.atoms.point_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a point of the given side at any rank, on where every coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_nd),
    },
    Function {
        name: "math.atoms.star_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n star, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_2d),
    },
    Function {
        name: "math.atoms.star_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n star, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_3d),
    },
    Function {
        name: "math.atoms.star_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a star of the given side at any rank, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_nd),
    },
    Function {
        name: "math.atoms.tree_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.",
        uncallable: None,
        door: Some(door_math_atoms_tree_nd),
    },
    Function {
        name: "math.atoms.vline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n line, free on axis 0, on along the odd columns.",
        uncallable: None,
        door: Some(door_math_atoms_vline_2d),
    },
    Function {
        name: "math.atoms.void_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n void, on where both coordinates share one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_2d),
    },
    Function {
        name: "math.atoms.void_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n void, on where all three coordinates share one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_3d),
    },
    Function {
        name: "math.atoms.void_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a void of the given side at any rank, on where every coordinate shares one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_nd),
    },
    Function {
        name: "math.atoms.vtree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tree, free on axis 0, on along the even columns.",
        uncallable: None,
        door: Some(door_math_atoms_vtree_2d),
    },
    Function {
        name: "math.atoms.xline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 0, its rods running along x.",
        uncallable: None,
        door: Some(door_math_atoms_xline_3d),
    },
    Function {
        name: "math.atoms.xtree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 0, its beams running along x.",
        uncallable: None,
        door: Some(door_math_atoms_xtree_3d),
    },
    Function {
        name: "math.atoms.yline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 1, its rods running along y.",
        uncallable: None,
        door: Some(door_math_atoms_yline_3d),
    },
    Function {
        name: "math.atoms.ytree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 1, its beams running along y.",
        uncallable: None,
        door: Some(door_math_atoms_ytree_3d),
    },
    Function {
        name: "math.atoms.zeros_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tensor of zeros.",
        uncallable: None,
        door: Some(door_math_atoms_zeros_2d),
    },
    Function {
        name: "math.atoms.zeros_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tensor of zeros.",
        uncallable: None,
        door: Some(door_math_atoms_zeros_3d),
    },
    Function {
        name: "math.atoms.zline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 2, its rods running along z.",
        uncallable: None,
        door: Some(door_math_atoms_zline_3d),
    },
    Function {
        name: "math.atoms.ztree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 2, its beams running along z.",
        uncallable: None,
        door: Some(door_math_atoms_ztree_3d),
    },
    Function {
        name: "math.automaton.Automaton.abscissa",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of `crate::memory::perron`.\n\nThe series converges absolutely on `Re s > alpha`. The bracket of `Automaton::perron` is a bound and not the root, so it is not read here.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_abscissa),
    },
    Function {
        name: "math.automaton.Automaton.base",
        signature: "(self: math.automaton.Automaton) -> u64",
        docs: "Returns the base `q = 2^D`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_base),
    },
    Function {
        name: "math.automaton.Automaton.cofactor",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.\n\nThe scalar `1 - k q^(-s)` becomes the determinant, and the cofactor is carried as `det(I - q^(-s) T) D_(P-1)(s) + 1^T adj(I - q^(-s) T) N(s)` with `N` the ladder numerator, so it is read on the whole `m = 0` pole comb where `zeta_W` itself is singular.\n\n# Errors\n\nErrs when the tolerance is out of reach.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_cofactor),
    },
    Function {
        name: "math.automaton.Automaton.denominator",
        signature: "(self: math.automaton.Automaton) -> [f64]",
        docs: "Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.\n\nThe roots in `x` are the reciprocals of the nonzero eigenvalues of `T`, so the poles of `zeta_W` sit at `s = log_q lambda_i - m + (arg lambda_i + 2 pi j) i / log q`: one comb per distinct eigenvalue, offset by the eigenvalue's argument.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_denominator),
    },
    Function {
        name: "math.automaton.Automaton.matrix",
        signature: "(self: math.automaton.Automaton) -> [[f64]]",
        docs: "Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of `crate::memory::transfer`, entry `(u', u)` counting the letters carrying `u` to `u'`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_matrix),
    },
    Function {
        name: "math.automaton.Automaton.new",
        signature: "(rule: math.memory.Rule) -> math.automaton.Automaton",
        docs: "Builds the ladder of a rule, choosing the peel depth.\n\n# Errors\n\nErrs when the rule accepts no element with a nonzero leading digit, or when its peel overruns the exact integers.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_new),
    },
    Function {
        name: "math.automaton.Automaton.peel",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "Returns the peel depth `P`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_peel),
    },
    Function {
        name: "math.automaton.Automaton.period",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "Returns the pole spacing `2 pi / log q`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_period),
    },
    Function {
        name: "math.automaton.Automaton.perron",
        signature: "(self: math.automaton.Automaton) -> (f64, f64)",
        docs: "Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.\n\nFor any nonnegative `T` and any `v > 0`, `min_u (T v)_u / v_u <= rho <= max_u (T v)_u / v_u`, so the pair brackets the root with no primitivity hypothesis; the guide is `v = (I + T)^m 1` for `m` sixty, or fewer when the entries would overrun the exponent range.\nThe ends carry a relative `1e-14` allowance of the same measured kind as `ROUNDING` and are not a rounding-certified interval, and the bracket is loose exactly when `T` is defective: at code `13` and `k = 2`, a Jordan block of root `1`, the upper end is `64/62`. Only the upper end is load bearing, as the `theta < 1` test that opens the Neumann branch; the root itself is `crate::memory::perron`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_perron),
    },
    Function {
        name: "math.automaton.Automaton.residue",
        signature: "(self: math.automaton.Automaton, w0: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.\n\nAt such a point the resolvent is `adj(I - x T) / det(I - x T)` with `x = q^(-w)`, so the residue is `1^T adj(I - x0 T) N(w0)` over `-x0 log q det'(x0)`: the adjugate is the spectral projector in polynomial form, and no eigenvector is solved for.\nThe head term `det(I - q^(-s) T) D_(P-1)(s)` is dropped, since the determinant vanishes at the pole.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when `w0` is not a simple root of `det(I - q^(-w) T)`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_residue),
    },
    Function {
        name: "math.automaton.Automaton.rule",
        signature: "(self: math.automaton.Automaton) -> math.memory.Rule",
        docs: "Returns the rule.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_rule),
    },
    Function {
        name: "math.automaton.Automaton.states",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "Returns the state count `q^(k-1)`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_states),
    },
    Function {
        name: "math.automaton.Automaton.with_peel",
        signature: "(rule: math.memory.Rule, peel: usize) -> math.automaton.Automaton",
        docs: "Builds the ladder at an explicit peel depth, at least the rule width and at least two.\n\n# Errors\n\nErrs when the rule accepts no such element, and at a peel depth under the rule width or two, or past the exact integers.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_with_peel),
    },
    Function {
        name: "math.automaton.Automaton.zeta",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns `zeta_W(s)` and the bound it is known to.\n\nThe bound is the propagated truncation bound, carried entrywise as a nonnegative vector through `(I - q^(-w) T)^(-1)`, plus `ROUNDING` times the scale the ladder carries, which is a measured allowance and not a proof.\nRight of the abscissa the inverse is majorised by its Neumann series, `sum_i (q^(-Re w) T)^i`, which is entrywise nonnegative and needs no norm and no primitivity; left of it the bound runs through the computed inverse certified by its own residual, `abs(C) (y + norm(y) r/(1-r) 1)` with `r = norm(I - (I - q^(-w) T) C)`, and the module raises rather than return when `r >= 1`.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when a level of the walk sits on a pole of the resolvent.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_zeta),
    },
    Function {
        name: "math.bang.Design.anf",
        signature: "(self: math.bang.Design) -> String",
        docs: "Returns the design's algebraic normal form as a string.",
        uncallable: None,
        door: Some(door_math_bang_design_anf),
    },
    Function {
        name: "math.bang.Design.degree",
        signature: "(self: math.bang.Design) -> i32",
        docs: "Returns the design's algebraic degree, or -1 for the zero design.",
        uncallable: None,
        door: Some(door_math_bang_design_degree),
    },
    Function {
        name: "math.bang.Design.name",
        signature: "(self: math.bang.Design) -> String",
        docs: "Returns the design's name as a line of prose, `bang dim 2, code 7`.\n\n# Errors\n\nErrors when the design's name does not read back.",
        uncallable: None,
        door: Some(door_math_bang_design_name),
    },
    Function {
        name: "math.bang.Design.rule",
        signature: "(self: math.bang.Design) -> [[u8]]",
        docs: "Returns the design's filled corners in sorted order.",
        uncallable: None,
        door: Some(door_math_bang_design_rule),
    },
    Function {
        name: "math.bang.MagicLayer.new",
        signature: "(design: math.name.Bang, number: usize) -> math.bang.MagicLayer",
        docs: "Pins a design to the side number it renders at.",
        uncallable: None,
        door: Some(door_math_bang_magiclayer_new),
    },
    Function {
        name: "math.bang.Universe.all",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "Returns every design in code order.",
        uncallable: None,
        door: Some(door_math_bang_universe_all),
    },
    Function {
        name: "math.bang.Universe.canonical",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "Returns the designs whose codes lead their orbits.",
        uncallable: None,
        door: Some(door_math_bang_universe_canonical),
    },
    Function {
        name: "math.bang.Universe.design",
        signature: "(self: math.bang.Universe, code: Code) -> math.bang.Design",
        docs: "Returns the design at a code with its precomputed orbit facts.",
        uncallable: None,
        door: Some(door_math_bang_universe_design),
    },
    Function {
        name: "math.bang.Universe.distinct",
        signature: "(self: math.bang.Universe) -> usize",
        docs: "Returns the number of distinct orbits.",
        uncallable: None,
        door: Some(door_math_bang_universe_distinct),
    },
    Function {
        name: "math.bang.Universe.new",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "Enumerates every orbit of a dimension from 1 to 4.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_universe_new),
    },
    Function {
        name: "math.bang.bang",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "Builds the universe of a dimension.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_bang),
    },
    Function {
        name: "math.bang.baseq.axis_maps",
        signature: "(base: usize) -> [[usize]]",
        docs: "Returns the distinct rotation and reflection maps of a base-q axis.",
        uncallable: None,
        door: Some(door_math_bang_baseq_axis_maps),
    },
    Function {
        name: "math.bang.baseq.bracelets",
        signature: "(max_base: usize) -> [u128]",
        docs: "Returns the distinct one-dimensional design counts for bases 1 through max_base.\n\n# Errors\n\nErrors when a Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_bracelets),
    },
    Function {
        name: "math.bang.baseq.canonical",
        signature: "(group: [[usize]], code: Code) -> Code",
        docs: "Returns the least code of the design's orbit.\n\n# Errors\n\nErrors on an empty group.",
        uncallable: None,
        door: Some(door_math_bang_baseq_canonical),
    },
    Function {
        name: "math.bang.baseq.carry",
        signature: "(element: [usize], code: Code) -> Code",
        docs: "Carries a code through one group element.",
        uncallable: None,
        door: Some(door_math_bang_baseq_carry),
    },
    Function {
        name: "math.bang.baseq.class_sequence",
        signature: "(max_dimension: usize) -> [u128]",
        docs: "Returns the fill-class counts for dimensions 1 through max_dimension.",
        uncallable: None,
        door: Some(door_math_bang_baseq_class_sequence),
    },
    Function {
        name: "math.bang.baseq.classes",
        signature: "(dimension: usize) -> u128",
        docs: "Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.",
        uncallable: None,
        door: Some(door_math_bang_baseq_classes),
    },
    Function {
        name: "math.bang.baseq.distinct_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Counts base-q designs distinct under symmetry.\n\n# Errors\n\nErrors below base one or dimension one, or when the Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_distinct_designs),
    },
    Function {
        name: "math.bang.baseq.even_fill_is_balanced",
        signature: "(number: usize, dimension: usize, popcount: u128) -> u128",
        docs: "Returns the collapsed fill count at an even side number.\n\n# Errors\n\nErrors at an odd side number.",
        uncallable: None,
        door: Some(door_math_bang_baseq_even_fill_is_balanced),
    },
    Function {
        name: "math.bang.baseq.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize) -> u128",
        docs: "Returns the filled-cell count of a binary design at a side number, folded from its filled corners.",
        uncallable: None,
        door: Some(door_math_bang_baseq_fill_from_corners),
    },
    Function {
        name: "math.bang.baseq.group",
        signature: "(base: usize, dimension: usize) -> [[usize]]",
        docs: "Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.",
        uncallable: None,
        door: Some(door_math_bang_baseq_group),
    },
    Function {
        name: "math.bang.baseq.group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the symmetry group order counted from the enumerated axis maps.",
        uncallable: None,
        door: Some(door_math_bang_baseq_group_order),
    },
    Function {
        name: "math.bang.baseq.orbit",
        signature: "(group: [[usize]], code: Code) -> [Code]",
        docs: "Returns every code a design reaches under the group.",
        uncallable: None,
        door: Some(door_math_bang_baseq_orbit),
    },
    Function {
        name: "math.bang.baseq.predicted_group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the closed-form group order the axis-map count must match.",
        uncallable: None,
        door: Some(door_math_bang_baseq_predicted_group_order),
    },
    Function {
        name: "math.bang.baseq.representatives",
        signature: "(base: usize, dimension: usize) -> [(Code, usize)]",
        docs: "Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.\n\n# Errors\n\nErrors when the cell count passes the walk limit.",
        uncallable: None,
        door: Some(door_math_bang_baseq_representatives),
    },
    Function {
        name: "math.bang.baseq.sequence",
        signature: "(base: usize, max_dimension: usize) -> [u128]",
        docs: "Returns the distinct-design counts for dimensions 1 through max_dimension.\n\n# Errors\n\nErrors below base one, or when a Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_sequence),
    },
    Function {
        name: "math.bang.baseq.total_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the raw design count before symmetry, two to the number of cells.\n\n# Errors\n\nErrors when the cell count reaches the hundred and twenty-eight bits a u128 holds.",
        uncallable: None,
        door: Some(door_math_bang_baseq_total_designs),
    },
    Function {
        name: "math.bang.catalog.Design.all",
        signature: "() -> [math.bang.catalog.Design; 16]",
        docs: "Returns every Design in canonical order.",
        uncallable: None,
        door: Some(door_math_bang_catalog_design_all),
    },
    Function {
        name: "math.bang.catalog.antis",
        signature: "(dimension: usize) -> [math.bang.catalog.Design]",
        docs: "Returns the anti designs for a dimension.",
        uncallable: None,
        door: Some(door_math_bang_catalog_antis),
    },
    Function {
        name: "math.bang.catalog.classics",
        signature: "(dimension: usize) -> [math.bang.catalog.Design]",
        docs: "Returns the classic designs for a dimension.",
        uncallable: None,
        door: Some(door_math_bang_catalog_classics),
    },
    Function {
        name: "math.bang.code.get",
        signature: "(self: Code) -> u128",
        docs: "Returns the bitmask the code carries.",
        uncallable: None,
        door: Some(door_math_bang_code_get),
    },
    Function {
        name: "math.bang.code_to_corners",
        signature: "(code: Code, dimension: usize, base: usize) -> [[u8]]",
        docs: "Unpacks a code into its filled residue corners.\n\n# Errors\n\nErrors when the code names a corner the dimension and base do not hold.",
        uncallable: None,
        door: Some(door_math_bang_code_to_corners),
    },
    Function {
        name: "math.bang.corners",
        signature: "(dimension: usize) -> [[u8]]",
        docs: "Returns the binary corners of a dimension in code order.",
        uncallable: None,
        door: Some(door_math_bang_corners),
    },
    Function {
        name: "math.bang.corners_to_code",
        signature: "(filled: [[u8]], dimension: usize, base: usize) -> Code",
        docs: "Packs filled residue corners back into their code.",
        uncallable: None,
        door: Some(door_math_bang_corners_to_code),
    },
    Function {
        name: "math.bang.factory.create",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "Renders a coded design to a tensor at its side number, dimension, base and fractal level.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_bang_factory_create),
    },
    Function {
        name: "math.bang.factory.create_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "Renders a design straight from its filled residue corners.\n\n# Errors\n\nErrors when a corner does not hold one residue per axis.",
        uncallable: None,
        door: Some(door_math_bang_factory_create_from_corners),
    },
    Function {
        name: "math.bang.factory.create_named",
        signature: "(spec: str, number: usize, level: usize) -> Tensor",
        docs: "Renders a design from its canonical JSON name.\n\n# Errors\n\nErrors for JSON that is not a bang name.",
        uncallable: None,
        door: Some(door_math_bang_factory_create_named),
    },
    Function {
        name: "math.bang.factory.residue_corners",
        signature: "(dimension: usize, base: usize) -> [[u8]]",
        docs: "Returns every base-q residue corner of a dimension in row-major order.",
        uncallable: None,
        door: Some(door_math_bang_factory_residue_corners),
    },
    Function {
        name: "math.bang.factory.total_codes",
        signature: "(dimension: usize, base: usize) -> Code",
        docs: "Returns the code count of a dimension and base, two to the number of corners.\n\n# Errors\n\nErrors when the corner count reaches the hundred and twenty-eight bits a u128 code holds.",
        uncallable: None,
        door: Some(door_math_bang_factory_total_codes),
    },
    Function {
        name: "math.bang.level_cap",
        signature: "(number: usize, dimension: usize, budget: usize) -> usize",
        docs: "Returns the deepest level, never below one, whose grid of number to the dimension per level stays within the cell budget.",
        uncallable: None,
        door: Some(door_math_bang_level_cap),
    },
    Function {
        name: "math.bang.levels_code",
        signature: "(dimension: usize, base: usize, levels: [usize]) -> Code",
        docs: "Returns the code of the design filled wherever a corner's residue sum lands in the levels.",
        uncallable: None,
        door: Some(door_math_bang_levels_code),
    },
    Function {
        name: "math.bang.magic",
        signature: "(layers: [math.bang.MagicLayer]) -> Tensor",
        docs: "Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.\n\nA run of one repeated layer is the ordinary self-similar fractal level.\n\n# Errors\n\nErrors below two layers, across dimensions, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_bang_magic),
    },
    Function {
        name: "math.bang.magic_named",
        signature: "(layers: [(str, usize)]) -> Tensor",
        docs: "Composes JSON-named layers in order.\n\n# Errors\n\nErrors for an object that is not a bang name, or layers that do not compose.",
        uncallable: None,
        door: Some(door_math_bang_magic_named),
    },
    Function {
        name: "math.bang.random_code",
        signature: "(dimension: usize, base: usize, rng: Rng(seed), min_fills: usize) -> Code",
        docs: "Rolls a code uniformly from the nonempty codes of a dimension and base that fill at least min_fills corners.\n\n# Errors\n\nErrors when the corners overflow a code, or min_fills is more than the corners.",
        uncallable: None,
        door: Some(door_math_bang_random_code),
    },
    Function {
        name: "math.bang.sources",
        signature: "(catalog: math.bang.catalog.Catalog, dimension: usize) -> [math.bang.catalog.Source]",
        docs: "Builds the tile sources a catalog names at a dimension.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_sources),
    },
    Function {
        name: "math.bang.symmetries",
        signature: "(dimension: usize) -> [([usize], [u8])]",
        docs: "Returns the full symmetry group as axis permutations paired with flip patterns.",
        uncallable: None,
        door: Some(door_math_bang_symmetries),
    },
    Function {
        name: "math.bang.total_exposure",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "Returns whether no two filled corners of a code sit at Hamming distance one.\n\nSuch a design buries no face at any side and any level, so its surface is six per cell.",
        uncallable: None,
        door: Some(door_math_bang_total_exposure),
    },
    Function {
        name: "math.bang.touches_every_corner",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.",
        uncallable: None,
        door: Some(door_math_bang_touches_every_corner),
    },
    Function {
        name: "math.bang.universe.anf",
        signature: "(code: Code, dimension: usize) -> [u8]",
        docs: "Returns the algebraic normal form coefficients of a code, one per corner.",
        uncallable: None,
        door: Some(door_math_bang_universe_anf),
    },
    Function {
        name: "math.bang.universe.anf_string",
        signature: "(code: Code, dimension: usize) -> String",
        docs: "Formats the algebraic normal form of a code as a sum of monomials.",
        uncallable: None,
        door: Some(door_math_bang_universe_anf_string),
    },
    Function {
        name: "math.bang.universe.apply",
        signature: "(element: ([usize], [u8]), corner: [u8]) -> [u8]",
        docs: "Applies a symmetry element to a corner.",
        uncallable: None,
        door: Some(door_math_bang_universe_apply),
    },
    Function {
        name: "math.bang.universe.corner_index",
        signature: "(corner: [u8]) -> usize",
        docs: "Returns the bit position a binary corner occupies in a code.",
        uncallable: None,
        door: Some(door_math_bang_universe_corner_index),
    },
    Function {
        name: "math.bang.universe.degree",
        signature: "(code: Code, dimension: usize) -> i32",
        docs: "Returns the algebraic degree of a code, or -1 for the zero design.",
        uncallable: None,
        door: Some(door_math_bang_universe_degree),
    },
    Function {
        name: "math.bang.universe.orbit",
        signature: "(code: Code, dimension: usize) -> [Code]",
        docs: "Returns every code a design reaches under the full symmetry group.",
        uncallable: None,
        door: Some(door_math_bang_universe_orbit),
    },
    Function {
        name: "math.bang.universe.permutations",
        signature: "(n: usize) -> [[usize]]",
        docs: "Returns every permutation of 0..n in sorted order.",
        uncallable: None,
        door: Some(door_math_bang_universe_permutations),
    },
    Function {
        name: "math.bang.universe_codes",
        signature: "(dimension: usize) -> [u128]",
        docs: "Returns the canonical design codes of a dimension, computed once and cached for the process.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_universe_codes),
    },
    Function {
        name: "math.bang.word.Schedule.all",
        signature: "() -> [math.bang.word.Schedule; 3]",
        docs: "Returns every Schedule in canonical order.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_all),
    },
    Function {
        name: "math.bang.word.Schedule.frequencies",
        signature: "(self: math.bang.word.Schedule) -> (f64, f64)",
        docs: "Returns the letter frequencies the schedule tends to.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_frequencies),
    },
    Function {
        name: "math.bang.word.Schedule.place",
        signature: "(self: math.bang.word.Schedule, index: usize) -> usize",
        docs: "Returns the letter the schedule takes at the place, zero or one.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_place),
    },
    Function {
        name: "math.bang.word.components",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Counts the 4-connected components of a plane word without drawing it.\n\n# Errors\n\nErrors on an empty word, or when the component count passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_components),
    },
    Function {
        name: "math.bang.word.constant_functional",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "Returns the constant-word component functional of a plane word's letter frequencies,\nin log two units.\n\nIt reads `sum_c f_c log2 comp(A_c)`, the one linear functional exact on constant words,\nwhich on the plane alphabet at side two is `(f_6 + f_9) log 2`. It is a prediction and not\na theorem: at interior frequency it is refuted on 78 of the 105 letter pairs and exact on 27.\n\n# Errors\n\nErrors on an empty word.",
        uncallable: None,
        door: Some(door_math_bang_word_constant_functional),
    },
    Function {
        name: "math.bang.word.dimension",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.\n\n# Errors\n\nErrors on an empty word, or one whose letters are all side one.",
        uncallable: None,
        door: Some(door_math_bang_word_dimension),
    },
    Function {
        name: "math.bang.word.fill",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Returns the filled cells of a word, the product of its letter fills.\n\n# Errors\n\nErrors when the word's fill passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_fill),
    },
    Function {
        name: "math.bang.word.fills",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Lists the filled cells of every letter, the product of which is the word's fill.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_bang_word_fills),
    },
    Function {
        name: "math.bang.word.letter",
        signature: "(layer: math.bang.MagicLayer) -> math.bang.word.Letter",
        docs: "Reads one plane letter: its fill, its runs, the rows and columns that wrap into a\nneighbouring copy, and its own components.\n\n# Errors\n\nErrors for a layer that is not a plane reading.",
        uncallable: None,
        door: Some(door_math_bang_word_letter),
    },
    Function {
        name: "math.bang.word.native",
        signature: "(layers: [math.bang.MagicLayer]) -> bool",
        docs: "Returns whether every letter renders at its own residue base, the native case where a\nperiodic word folds to one residue rule at the product base.",
        uncallable: None,
        door: Some(door_math_bang_word_native),
    },
    Function {
        name: "math.bang.word.period",
        signature: "(layers: [math.bang.MagicLayer]) -> usize",
        docs: "Returns the shortest whole period of the letter list, its own length when no shorter block repeats.",
        uncallable: None,
        door: Some(door_math_bang_word_period),
    },
    Function {
        name: "math.bang.word.prefixes",
        signature: "(layers: [math.bang.MagicLayer]) -> [math.bang.word.Counts]",
        docs: "Folds a plane word letter by letter and returns the counts at every prefix.\n\nEach letter folds the component count by its own geometry: a letter with no wrap-around\ncontact isolates every block, so the count becomes the outer fill times the letter's own\npieces; a connected letter with contacts on one axis merges blocks along that axis alone,\nso the count becomes the outer run count; a connected letter with both contacts leaves the\nblock graph isomorphic to the cell graph and the count unmoved. A letter that is neither\nconnected nor contact-free both splits and merges and is refused.\n\nThe list stops at the last prefix whose counts fit a u128 rather than wrapping.\n\n# Errors\n\nErrors for a letter that neither connects nor stays contact-free.",
        uncallable: None,
        door: Some(door_math_bang_word_prefixes),
    },
    Function {
        name: "math.bang.word.rates",
        signature: "(layers: [math.bang.MagicLayer]) -> [(f64, f64)]",
        docs: "Returns the prefix rates of a plane word in log two units, the component rate\n`(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length.\n\nAt interior letter frequency the two meet: the component exponent is order-blind and equals\nthe fill exponent on every one of the 105 letter pairs but the domino against the full tile.\nThe list stops at the last prefix whose counts fit a u128.\n\n# Errors\n\nErrors for a letter that neither connects nor stays contact-free.",
        uncallable: None,
        door: Some(door_math_bang_word_rates),
    },
    Function {
        name: "math.bang.word.side",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Returns the side of a word, the product of its letter sides.\n\n# Errors\n\nErrors when the word's side passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_side),
    },
    Function {
        name: "math.bang.word.spell",
        signature: "(schedule: math.bang.word.Schedule, pair: (math.bang.MagicLayer, math.bang.MagicLayer), length: usize) -> [math.bang.MagicLayer]",
        docs: "Spells the first letters of a schedule over an ordered pair of letters.",
        uncallable: None,
        door: Some(door_math_bang_word_spell),
    },
    Function {
        name: "math.bang.word.staircase",
        signature: "(depth: usize) -> [math.bang.MagicLayer]",
        docs: "Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,\nthen `magic(3,5)`, then `magic(3,5,7)`, and so on.\n\nThe letter at odd side `2j + 1` occurs `depth - j + 1` times in the first `depth` blocks,\nso the word holds `depth (depth + 1) / 2` letters and its dimension is the occurrence-weighted\naverage of the per-letter dimensions.\n\n# Errors\n\nErrors on an empty block list.",
        uncallable: None,
        door: Some(door_math_bang_word_staircase),
    },
    Function {
        name: "math.bang.word.thue_morse",
        signature: "(index: usize) -> usize",
        docs: "Returns the Thue-Morse letter at the place, the parity of its binary digit sum.",
        uncallable: None,
        door: Some(door_math_bang_word_thue_morse),
    },
    Function {
        name: "math.cell.census.edges",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the distinct unit edges the filled sites carry, the edge graph's branches.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_cell_census_edges),
    },
    Function {
        name: "math.cell.census.exposure",
        signature: "(cell: Cell2d|Cell3d) -> u128",
        docs: "Counts the faces of filled sites open to emptiness or the border.",
        uncallable: None,
        door: Some(door_math_cell_census_exposure),
    },
    Function {
        name: "math.cell.census.fills",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the filled sites of the cell.",
        uncallable: None,
        door: Some(door_math_cell_census_fills),
    },
    Function {
        name: "math.cell.census.vertices",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the distinct corners the filled sites touch, the edge graph's nodes.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_cell_census_vertices),
    },
    Function {
        name: "math.cell.census.voids",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the empty sites of the cell.",
        uncallable: None,
        door: Some(door_math_cell_census_voids),
    },
    Function {
        name: "math.cell.geometry.merge_reps",
        signature: "(cells: [Cell2d|Cell3d], reps: [usize]) -> Cell2d|Cell3d",
        docs: "Merges same-shaped cells into one block laid out by the per-axis repetition counts.\n\n# Errors\n\nErrors when the cells differ in shape or do not fill the arrangement.",
        uncallable: None,
        door: Some(door_math_cell_geometry_merge_reps),
    },
    Function {
        name: "math.cell.geometry.perforate",
        signature: "(mask: Tensor, cell: Cell2d|Cell3d, value: u8) -> Cell2d|Cell3d",
        docs: "Writes the value into the cell wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_cell_geometry_perforate),
    },
    Function {
        name: "math.cell.grow",
        signature: "(pattern: Tensor, level: usize) -> Cell2d|Cell3d",
        docs: "Grows a seed pattern into a cell, deepened to its fractal past level one.\n\n# Errors\n\nErrors when the pattern's rank is not N.",
        uncallable: None,
        door: Some(door_math_cell_grow),
    },
    Function {
        name: "math.cell.models.anti",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Inverts the cell.",
        uncallable: None,
        door: Some(door_math_cell_models_anti),
    },
    Function {
        name: "math.cell.models.binarize",
        signature: "(self: Cell2d|Cell3d, threshold: u8) -> Cell2d|Cell3d",
        docs: "Maps each site to one at or above the threshold, zero below.",
        uncallable: None,
        door: Some(door_math_cell_models_binarize),
    },
    Function {
        name: "math.cell.models.binarize_otsu",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Binarizes the cell at the threshold Otsu's method picks.",
        uncallable: None,
        door: Some(door_math_cell_models_binarize_otsu),
    },
    Function {
        name: "math.cell.models.blur",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, wrap: bool) -> Cell2d|Cell3d",
        docs: "Rounds each site to the mean of its masked neighborhood, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_cell_models_blur),
    },
    Function {
        name: "math.cell.models.combine",
        signature: "(self: Cell2d|Cell3d, other: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Returns the Kronecker product of the two cells.",
        uncallable: None,
        door: Some(door_math_cell_models_combine),
    },
    Function {
        name: "math.cell.models.counting_dtype",
        signature: "(mask: Tensor) -> core.Dtype",
        docs: "Returns the narrowest count dtype that fits the mask's popcount.",
        uncallable: None,
        door: Some(door_math_cell_models_counting_dtype),
    },
    Function {
        name: "math.cell.models.depth",
        signature: "(self: Cell3d) -> usize",
        docs: "Returns the size of axis 0, the cube's leading axis.",
        uncallable: None,
        door: Some(door_math_cell_models_depth),
    },
    Function {
        name: "math.cell.models.dtype_for",
        signature: "(peak: i64) -> core.Dtype",
        docs: "Returns the narrowest unsigned dtype that holds the peak value.",
        uncallable: None,
        door: Some(door_math_cell_models_dtype_for),
    },
    Function {
        name: "math.cell.models.fractal",
        signature: "(self: Cell2d|Cell3d, level: usize) -> Cell2d|Cell3d",
        docs: "Deepens the cell into its level-fold fractal.\n\n# Errors\n\nErrors below level one.",
        uncallable: None,
        door: Some(door_math_cell_models_fractal),
    },
    Function {
        name: "math.cell.models.height",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "Returns the size of the axis before the last.",
        uncallable: None,
        door: Some(door_math_cell_models_height),
    },
    Function {
        name: "math.cell.models.invert",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Swaps filled and empty sites.",
        uncallable: None,
        door: Some(door_math_cell_models_invert),
    },
    Function {
        name: "math.cell.models.layers",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Tags each site with its ring distance from the center.",
        uncallable: None,
        door: Some(door_math_cell_models_layers),
    },
    Function {
        name: "math.cell.models.neighbors",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, target: u8, wrap: bool) -> Cell2d|Cell3d",
        docs: "Tags each site with its count of masked neighbors matching the target, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_cell_models_neighbors),
    },
    Function {
        name: "math.cell.models.new",
        signature: "(types: Tensor) -> Cell2d|Cell3d",
        docs: "Builds a cell from an N-dimensional tensor of types.\n\n# Errors\n\nErrors when the tensor's rank is not N.",
        uncallable: None,
        door: Some(door_math_cell_models_new),
    },
    Function {
        name: "math.cell.models.orient",
        signature: "(self: Cell3d, index: usize) -> Cell3d",
        docs: "Turns the cell into one of the 24 cube orientations.\n\n# Errors\n\nErrors past orientation twenty-three.",
        uncallable: None,
        door: Some(door_math_cell_models_orient),
    },
    Function {
        name: "math.cell.models.pad",
        signature: "(self: Cell2d|Cell3d, count: usize, value: u8) -> Cell2d|Cell3d",
        docs: "Wraps the cell in count layers of the given value on every side.",
        uncallable: None,
        door: Some(door_math_cell_models_pad),
    },
    Function {
        name: "math.cell.models.paint",
        signature: "(self: Cell2d|Cell3d, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "Colors each site by its type through the mapping in the given mode.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_cell_models_paint),
    },
    Function {
        name: "math.cell.models.perforate",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, value: u8) -> Cell2d|Cell3d",
        docs: "Writes the value wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_cell_models_perforate),
    },
    Function {
        name: "math.cell.models.rotate",
        signature: "(self: Cell2d, k: usize) -> Cell2d | (self: Cell3d, k: usize, axes: (usize, usize)) -> Cell3d",
        docs: "Rotates the cell k quarter turns in the plane.\n\n# Errors\n\nErrors for a cell without two axes.",
        uncallable: None,
        door: Some(door_math_cell_models_rotate),
    },
    Function {
        name: "math.cell.models.tile",
        signature: "(self: Cell2d, width: usize, height: usize) -> Cell2d | (self: Cell3d, width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "Repeats the cell into a width-by-height array of copies.\n\n# Errors\n\nErrors for a cell without two axes.",
        uncallable: None,
        door: Some(door_math_cell_models_tile),
    },
    Function {
        name: "math.cell.models.types",
        signature: "(self: Cell2d|Cell3d) -> Tensor",
        docs: "Returns the tensor of types.",
        uncallable: None,
        door: Some(door_math_cell_models_types),
    },
    Function {
        name: "math.cell.models.width",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "Returns the size of the last axis.",
        uncallable: None,
        door: Some(door_math_cell_models_width),
    },
    Function {
        name: "math.cell.paint",
        signature: "(cell: Cell2d|Cell3d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "Colors the cell through the given mapping and mode, defaulting to the standard palette by type.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_cell_paint),
    },
    Function {
        name: "math.cell.serializer.byte_cube",
        signature: "(value: Json) -> [[[u8]]]",
        docs: "Reads a triply nested JSON array into layers of byte rows.\n\n# Errors\n\nErrors when the value is not a cube of bytes.",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_cube),
    },
    Function {
        name: "math.cell.serializer.byte_grid",
        signature: "(value: Json) -> [[u8]]",
        docs: "Reads a nested JSON array into rows of bytes.\n\n# Errors\n\nErrors when the value is not a grid of bytes.",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_grid),
    },
    Function {
        name: "math.cell.serializer.color_grid",
        signature: "(value: Json) -> [[[u8; 4]]]",
        docs: "Reads a nested JSON array into rows of four-channel colors.\n\n# Errors\n\nErrors when the value is not a grid of RGBA quadruples.",
        uncallable: None,
        door: Some(door_math_cell_serializer_color_grid),
    },
    Function {
        name: "math.cell.serializer.count_cube",
        signature: "(value: Json) -> [i64]",
        docs: "Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.\n\n# Errors\n\nErrors when the value is not a cube of counts.",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_cube),
    },
    Function {
        name: "math.cell.serializer.count_grid",
        signature: "(value: Json) -> [i64]",
        docs: "Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.\n\n# Errors\n\nErrors when the value is not a grid of counts.",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_grid),
    },
    Function {
        name: "math.cell.serializer.parse",
        signature: "(text: str) -> Json",
        docs: "Parses JSON text into a value tree.\n\n# Errors\n\nErrors when the text is not JSON.",
        uncallable: None,
        door: Some(door_math_cell_serializer_parse),
    },
    Function {
        name: "math.cell.serializer.tag_layer",
        signature: "(counts: [i64], shape: [usize]) -> Tensor",
        docs: "Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.\n\n# Errors\n\nErrors when the counts do not match the shape.",
        uncallable: None,
        door: Some(door_math_cell_serializer_tag_layer),
    },
    Function {
        name: "math.cell.serializer.types_field",
        signature: "(data: Json) -> Json",
        docs: "Returns the types field of the data.\n\n# Errors\n\nErrors when the types field is missing.",
        uncallable: None,
        door: Some(door_math_cell_serializer_types_field),
    },
    Function {
        name: "math.coprime.gasket",
        signature: "(level: u32) -> u64",
        docs: "Counts the coprime points of the Sierpinski gasket at a level: the pairs `0 <= i, j < 2^level` with `i AND j = 0` and `gcd(i, j) = 1`, the sequence [A396934](https://oeis.org/A396934).\n\nWalks the rows `m = i + j = i OR j`: only odd rows hold a coprime pair, `gcd(i, j) = gcd(i, m)`, the odd submasks `i` of `m` are kept when coprime to `m`, and the swap `(i, j) -> (j, i)` doubles them.\n\n# Errors\n\nFails past `GASKET_LEVELS`.",
        uncallable: None,
        door: Some(door_math_coprime_gasket),
    },
    Function {
        name: "math.counts.Closed.text",
        signature: "(self: math.counts.Closed) -> String",
        docs: "Spells the closed form.",
        uncallable: None,
        door: Some(door_math_counts_closed_text),
    },
    Function {
        name: "math.counts.Exposure.at",
        signature: "(self: math.counts.Exposure, level: u32) -> u128?",
        docs: "Returns the exposed faces of the level-fold Kronecker power, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_exposure_at),
    },
    Function {
        name: "math.counts.Exposure.from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> math.counts.Exposure",
        docs: "Folds the counts from the filled residue corners at a side number, without rendering the tile.\n\nAn adjacent pair sits at positions `i, i + 1` whose residues are `r, r + 1 mod q`, and a\nspanning position pairs residue 0 with the residue of `n - 1`.",
        uncallable: None,
        door: Some(door_math_counts_exposure_from_corners),
    },
    Function {
        name: "math.counts.Exposure.of_tile",
        signature: "(tile: Tensor) -> math.counts.Exposure",
        docs: "Reads the counts off a rendered tile.",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile),
    },
    Function {
        name: "math.counts.Exposure.recurrence",
        signature: "(self: math.counts.Exposure) -> [i128]",
        docs: "Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.\n\nThe roots are `occ` and the distinct nonzero spanning counts, `occ` doubled where a\nspanning count equals it.",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence),
    },
    Function {
        name: "math.counts.Placements.at",
        signature: "(self: math.counts.Placements, level: u32) -> math.counts.Skeleton?",
        docs: "Returns the face counts of the level-fold Kronecker power, or none past a `u128`.",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.of_tile",
        signature: "(tile: Tensor) -> math.counts.Placements",
        docs: "Reads the placement counts off a rendered tile of two to four dimensions.\n\n# Errors\n\nErrors when the tile has more than four dimensions, where the window patterns pass a `u32`.",
        uncallable: Some("math::counts::Placements has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.recurrence",
        signature: "(self: math.counts.Placements, k: usize?) -> [i128]?",
        docs: "Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the `k`-face count obeys from `L = 0`, every face when `k` is none, or none past an `i128`.",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Placements.roots",
        signature: "(self: math.counts.Placements, k: usize?) -> [(u128, u32)]",
        docs: "Returns the roots of the recurrence the `k`-face count obeys, every face when `k` is none, each with the multiplicity the recurrence needs: the spanning counts of the patterns the count reaches, a root repeated once per pattern of that count on one carry chain.",
        uncallable: Some("math::counts::Placements has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.cells",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "The filled cells.",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.edges",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "The distinct unit edges.",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.faces",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "The distinct unit squares, the cells themselves in the plane.",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.Skeleton.vertices",
        signature: "(self: math.counts.Skeleton) -> u128",
        docs: "The distinct corners.",
        uncallable: Some("math::counts::Skeleton has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.counts.centered_hexagonal",
        signature: "(m: usize) -> u128",
        docs: "Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.",
        uncallable: None,
        door: Some(door_math_counts_centered_hexagonal),
    },
    Function {
        name: "math.counts.cut_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the filled triangle count of the code's cut section at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_cut_fills),
    },
    Function {
        name: "math.counts.cut_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the empty triangle count of the code's cut section at the given level.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_cut_voids),
    },
    Function {
        name: "math.counts.dimension",
        signature: "(code: Code, number: usize, base_dimension: usize, base: usize) -> f64",
        docs: "Returns the code's fractal dimension, the log of its one-level fill over the log of number.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_dimension),
    },
    Function {
        name: "math.counts.edges_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term\nrecurrence over four powers, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_edges_of_tile),
    },
    Function {
        name: "math.counts.exposure",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.\n\n# Errors\n\nErrors when the code is out of range, or the exposure passes a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_math_counts_exposure),
    },
    Function {
        name: "math.counts.exposure_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile_2),
    },
    Function {
        name: "math.counts.exposure_recurrence",
        signature: "(tile: Tensor) -> [i128]",
        docs: "Returns the coefficients of the recurrence the tile's exposure obeys.",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence_2),
    },
    Function {
        name: "math.counts.fill",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the filled cell count of the code's fractal at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_fill),
    },
    Function {
        name: "math.counts.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, _dimension: usize, level: u32, base: usize) -> u128",
        docs: "Sums each corner's position products into a base fill and raises it to the level.",
        uncallable: None,
        door: Some(door_math_counts_fill_from_corners),
    },
    Function {
        name: "math.counts.fill_polynomial",
        signature: "(corners: [[u8]], dimension: usize) -> [i128]",
        docs: "Expands the odd-side fill at side `2k - 1`, `sum over the corners of k^(zeros) (k - 1)^(ones)`, into coefficients by rising power of `k`.",
        uncallable: None,
        door: Some(door_math_counts_fill_polynomial),
    },
    Function {
        name: "math.counts.grid",
        signature: "(number: usize, dimension: usize, level: u32) -> u128",
        docs: "Returns the total cells of the grid, number to the dimension, to the level.",
        uncallable: None,
        door: Some(door_math_counts_grid),
    },
    Function {
        name: "math.counts.ladder.cap",
        signature: "(base: usize) -> usize",
        docs: "The widest dimension the exact carry arithmetic reaches at the base.\n\nThe sign is the heaviest reading: it walks the characteristic polynomial at `f_D/q` with the\ndenominators cleared, so its running value climbs like `f_D^(D/2)` and a hundred and twenty-eight\nbits run out at dimension fifteen in base three and dimension eleven in base five.\n\n# Errors\n\nErrors at an even base under three, or past the dimension the ladder reaches.",
        uncallable: None,
        door: Some(door_math_counts_ladder_cap),
    },
    Function {
        name: "math.counts.ladder.carry_matrix",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.\n\nA level of the design adds one base-`q` digit per coordinate, so the height of the central\ndiagonal hyperplane moves by a digit sum `s` and a carry `c -> (c + mD - s)/q` with `m` the\nmiddle digit; the map contracts onto this window from every start.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_carry_matrix),
    },
    Function {
        name: "math.counts.ladder.characteristic",
        signature: "(rows: [[i128]]) -> [i128]",
        docs: "The monic characteristic polynomial of a square integer matrix, highest power first.\n\nFaddeev-LeVerrier in exact integers: every division lands whole, so no fraction is ever needed\nand the answer is the recurrence's coefficients up to sign.\n\n# Errors\n\nErrors when the walk leaves a remainder, or a coefficient passes an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_characteristic),
    },
    Function {
        name: "math.counts.ladder.determinant",
        signature: "(rows: [[i128]]) -> i128",
        docs: "The determinant of a square integer matrix, read off its characteristic polynomial.\n\n# Errors\n\nErrors when the walk leaves a remainder, or a coefficient passes an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_determinant),
    },
    Function {
        name: "math.counts.ladder.digit_polynomial",
        signature: "(base: usize, dimension: usize) -> [i128]",
        docs: "The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.\n\nThe design keeps the cells whose digit vector has at most one coordinate equal to the middle\ndigit, so its weight generating function is `A(t)^(D-1) (A(t) + D t^m)`, with `A` the sum of\nevery digit power but the middle one. At base three that factors as\n`(1 + t^2)^(D-1) (1 + D t + t^2)`, and the sum of the coefficients is the fill.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_digit_polynomial),
    },
    Function {
        name: "math.counts.ladder.even_block",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "The reflection-even block of the carry matrix, of size `ceil(D/2)`.\n\nThe digit polynomial is palindromic, so the reflection `c -> -c` commutes with the carry map\nand splits it; the even half carries the central count and its characteristic polynomial is\nthe recurrence the counts obey.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_even_block),
    },
    Function {
        name: "math.counts.ladder.fill",
        signature: "(base: usize, dimension: usize) -> i128",
        docs: "The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.\n\n# Errors\n\nErrors at an even base under three, a dimension outside two to the ceiling, or a product past an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_fill),
    },
    Function {
        name: "math.counts.ladder.ladder",
        signature: "(base: usize, dimension: usize, levels: usize) -> [i128]",
        docs: "The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.\n\nThe count is the top-left entry of the `L`-th power of the even block. The walk stops early\nwhen the next count would pass a hundred and twenty-eight bits, so the answer runs as far as\nexact integers reach and no further.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_ladder),
    },
    Function {
        name: "math.counts.ladder.perron",
        signature: "(rows: [[i128]]) -> f64",
        docs: "The Perron root of a nonnegative square integer matrix.\n\nThe characteristic polynomial is rescaled by the largest row sum, which bounds the root above,\nthen the largest sign change on the unit interval is hunted on a grid and closed by bisection,\nthe same walk the recurrence growth reader takes.\n\n# Errors\n\nErrors when the block has no positive row sum, or no root inside its bound.",
        uncallable: None,
        door: Some(door_math_counts_ladder_perron),
    },
    Function {
        name: "math.counts.ladder.sign",
        signature: "(base: usize, dimension: usize) -> i32",
        docs: "The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.\n\nThe characteristic polynomial is evaluated at `f_D/q` with the denominators cleared, so the\nanswer is a comparison of whole numbers and never a rounding. The law reads `(-1)^(D+1)`: the\nslice exponent stands above the solid's dimension less one at odd `D` and below it at even `D`.\n\n# Errors\n\nErrors at an even base under three, a dimension outside two to the ceiling, or a term past an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_sign),
    },
    Function {
        name: "math.counts.ladder.spectral_ratio",
        signature: "(base: usize, dimension: usize) -> f64?",
        docs: "The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.\n\nPower iteration finds the leading pair, Hotelling deflation takes it out and a second walk,\nreprojected every sweep so rounding cannot bring the leader back, reads the runner up. At base\nthree the ratio falls to `(D + 2)/(D - 2)`, so no fixed spectral gap survives the dimensions.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_spectral_ratio),
    },
    Function {
        name: "math.counts.ladder.trace",
        signature: "(rows: [[i128]]) -> i128",
        docs: "The trace of a square integer matrix.",
        uncallable: None,
        door: Some(door_math_counts_ladder_trace),
    },
    Function {
        name: "math.counts.limit",
        signature: "(code: Code, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "Returns the fill ratio the code walks toward as the side number grows, reduced.\n\nThe ratio is the corner count over the corner slots, so it is always rational: the\ncarpet holds three corners of four and the sponge four of eight.\n\n# Errors\n\nErrors when the code is out of range, or the limit overflows at this level.",
        uncallable: None,
        door: Some(door_math_counts_limit),
    },
    Function {
        name: "math.counts.odd_power",
        signature: "(dimension: usize) -> [i128]",
        docs: "Expands the whole grid at side `2k - 1`, `(2k - 1)^dimension`, into coefficients by rising power of `k`.",
        uncallable: None,
        door: Some(door_math_counts_odd_power),
    },
    Function {
        name: "math.counts.pairs",
        signature: "(tile: Tensor) -> [(u128, u128)]",
        docs: "Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.\n\nA level deeper, each adjacent pair buries one face per spanning position of the block, and the\nspanning positions of the block multiply level by level, so the exposure closes.",
        uncallable: None,
        door: Some(door_math_counts_pairs),
    },
    Function {
        name: "math.counts.positions",
        signature: "(residue: usize, number: usize, base: usize) -> u128",
        docs: "Counts the indices below number that equal residue modulo base.",
        uncallable: None,
        door: Some(door_math_counts_positions),
    },
    Function {
        name: "math.counts.pro_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the filled triangle count of the code's pro projection at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_pro_fills),
    },
    Function {
        name: "math.counts.pro_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the empty triangle count of the code's pro projection at the given level.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_pro_voids),
    },
    Function {
        name: "math.counts.profile_of_tile",
        signature: "(tile: Tensor, level: u32) -> [u128]",
        docs: "Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.\n\nThe count is the coefficient of the digit polynomial, the level-fold product of the tile's\nweight sums, so no cell of the power is ever built; the tile must be a hypercube.\n\n# Errors\n\nErrors for a tile that is not a hypercube, a level below one, a side past the widest, or counts past a u128.",
        uncallable: None,
        door: Some(door_math_counts_profile_of_tile),
    },
    Function {
        name: "math.counts.ratio",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> f64",
        docs: "Returns the filled fraction of the grid, or 0.0 for an empty grid.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_ratio),
    },
    Function {
        name: "math.counts.rational",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "Returns the exact filled fraction as a fraction of fill over grid, reduced.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_rational),
    },
    Function {
        name: "math.counts.six.grid_triangles",
        signature: "(number: usize, level: u32) -> u128",
        docs: "Returns the triangles of the full hexagon with side number to the level.",
        uncallable: None,
        door: Some(door_math_counts_six_grid_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_boundary",
        signature: "(number: usize) -> u128",
        docs: "Returns the boundary edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_boundary),
    },
    Function {
        name: "math.counts.six.solid_slice_core_edges",
        signature: "(number: usize) -> u128",
        docs: "Returns the core edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_core_nodes",
        signature: "(number: usize) -> u128",
        docs: "Returns the core node count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_nodes),
    },
    Function {
        name: "math.counts.six.solid_slice_edges",
        signature: "(number: usize) -> u128",
        docs: "Returns the distinct triangle-edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_interior",
        signature: "(number: usize) -> u128",
        docs: "Returns the interior edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_interior),
    },
    Function {
        name: "math.counts.six.solid_slice_triangles",
        signature: "(number: usize) -> u128",
        docs: "Returns the triangle count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_vertices",
        signature: "(number: usize) -> u128",
        docs: "Returns the vertex count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_vertices),
    },
    Function {
        name: "math.counts.skeleton",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, level: u32) -> math.counts.Skeleton",
        docs: "Returns the face counts of the code's design at the given level, from the tile's placement counts alone.\n\n# Errors\n\nErrors when the code is out of range, the dimension passes four, or a count passes a `u128`.",
        uncallable: Some("math::counts::Skeleton has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.skeleton_of_tile",
        signature: "(tile: Tensor, level: u32) -> math.counts.Skeleton?",
        docs: "Returns the face counts of the tile's level-fold Kronecker power, or none past a `u128`.\n\n# Errors\n\nErrors when the tile has more than four dimensions.",
        uncallable: Some("math::counts::Skeleton has no Serialize"),
        door: None,
    },
    Function {
        name: "math.counts.skeleton_recurrence",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, k: usize?) -> [i128]",
        docs: "Returns the coefficients of the recurrence the code's `k`-face count obeys along the level, every face and the Euler characteristic when `k` is none.\n\n# Errors\n\nErrors when the code is out of range, the dimension passes four, or a coefficient passes an `i128`.",
        uncallable: None,
        door: Some(door_math_counts_skeleton_recurrence),
    },
    Function {
        name: "math.counts.surface",
        signature: "(code: Code, number: usize, level: u32, base: usize) -> u128",
        docs: "Returns the exposed face count of the code's 3D fractal at the given level.\n\n# Errors\n\nErrors when the code is out of range, or the exposure passes a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_math_counts_surface),
    },
    Function {
        name: "math.counts.void",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the empty cell count, grid minus fill.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_void),
    },
    Function {
        name: "math.design.digits_of",
        signature: "(mask: u32, base: u64) -> [u64]",
        docs: "Returns the digits a bitmask names inside the base, ascending.",
        uncallable: None,
        door: Some(door_math_design_digits_of),
    },
    Function {
        name: "math.design.echo_series",
        signature: "(values: [u64], log_x: [f64], exponent: f64) -> [f64]",
        docs: "Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.",
        uncallable: None,
        door: Some(door_math_design_echo_series),
    },
    Function {
        name: "math.design.elements",
        signature: "(base: u64, digits: [u64], depth: usize) -> [u64]",
        docs: "Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.",
        uncallable: None,
        door: Some(door_math_design_elements),
    },
    Function {
        name: "math.design.log_grid",
        signature: "(values: [u64], samples: usize) -> [f64]",
        docs: "Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.",
        uncallable: None,
        door: Some(door_math_design_log_grid),
    },
    Function {
        name: "math.design.median_floor",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "Returns the running median of the power over a window of the given width, the window clamped at the ends.",
        uncallable: None,
        door: Some(door_math_design_median_floor),
    },
    Function {
        name: "math.design.meter",
        signature: "(mu: [i8]) -> [i64]",
        docs: "Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.",
        uncallable: None,
        door: Some(door_math_design_meter),
    },
    Function {
        name: "math.design.nearest",
        signature: "(value: f64, list: [f64]) -> f64",
        docs: "Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.",
        uncallable: None,
        door: Some(door_math_design_nearest),
    },
    Function {
        name: "math.design.peaks",
        signature: "(gamma: [f64], score: [f64], band: (f64, f64), threshold: f64) -> [usize]",
        docs: "Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.\n\n# Errors\n\nErrs when the axis holds fewer bins than the score.",
        uncallable: None,
        door: Some(door_math_design_peaks),
    },
    Function {
        name: "math.design.pole_lattice",
        signature: "(base: u64, top: f64) -> [f64]",
        docs: "Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.",
        uncallable: None,
        door: Some(door_math_design_pole_lattice),
    },
    Function {
        name: "math.design.resample",
        signature: "(values: [u64], running: [i64], exponent: f64, log_x: [f64]) -> [f64]",
        docs: "Reads the running meter at every point of the log grid and divides by x to the exponent.",
        uncallable: None,
        door: Some(door_math_design_resample),
    },
    Function {
        name: "math.design.score",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "Returns the power over its local median floor, the score a peak is read against.",
        uncallable: None,
        door: Some(door_math_design_score),
    },
    Function {
        name: "math.design.size",
        signature: "(digits: [u64], depth: usize) -> u128",
        docs: "Returns the count of elements the design holds at the depth, the length `elements` returns without building them.",
        uncallable: None,
        door: Some(door_math_design_size),
    },
    Function {
        name: "math.design.spectrum",
        signature: "(log_x: [f64], series: [f64]) -> ([f64], [f64])",
        docs: "Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.\n\n# Errors\n\nErrs when the log grid holds fewer points than the series.",
        uncallable: None,
        door: Some(door_math_design_spectrum),
    },
    Function {
        name: "math.design.upper_rms",
        signature: "(series: [f64]) -> f64",
        docs: "Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.",
        uncallable: None,
        door: Some(door_math_design_upper_rms),
    },
    Function {
        name: "math.dissection.cap",
        signature: "(y: u64) -> u64",
        docs: "Returns `Q = floor(y^(3/5))`, the largest denominator the dissection admits, exact in integers.",
        uncallable: None,
        door: Some(door_math_dissection_cap),
    },
    Function {
        name: "math.dissection.chain_exponent",
        signature: "(base: u64) -> f64",
        docs: "Returns the chain's certificate exponent at one missing digit, `alpha_1 = log_base(z base/(base - 1))`, the same at every missing digit.",
        uncallable: None,
        door: Some(door_math_dissection_chain_exponent),
    },
    Function {
        name: "math.dissection.chain_margin",
        signature: "(base: u64) -> f64",
        docs: "Returns the chain's margin at the bar, the cubic cleared of denominators at `w = base^(1/5)(1 - 1/base)`, positive exactly when the root sits below `w` and `alpha_1 < 1/5`.",
        uncallable: None,
        door: Some(door_math_dissection_chain_margin),
    },
    Function {
        name: "math.dissection.chain_root",
        signature: "(base: u64) -> f64",
        docs: "Returns the root `z > 1` of the digit-uniform chain at one missing digit, `(z-1)^3 = (2/pi)(log base) z + gamma'(z-1) + (2/pi)(z-1)^2/(base z - 1)`, by bisection.",
        uncallable: None,
        door: Some(door_math_dissection_chain_root),
    },
    Function {
        name: "math.dissection.chain_wall",
        signature: "() -> u64",
        docs: "Returns the chain's wall, one past the last base up to `CAP_BASE` whose margin is not positive.",
        uncallable: None,
        door: Some(door_math_dissection_chain_wall),
    },
    Function {
        name: "math.dissection.consecutive",
        signature: "(digits: [u64]) -> bool",
        docs: "Returns whether the digit set keeps two consecutive digits, the hypothesis region C1 reads.",
        uncallable: None,
        door: Some(door_math_dissection_consecutive),
    },
    Function {
        name: "math.dissection.fraction",
        signature: "(a: u64, y: u64, cap: u64) -> (u64, u64)",
        docs: "Returns the last continued-fraction convergent `l/d` of `a/y` whose denominator is at most the cap, the Dirichlet fraction of the dissection.",
        uncallable: None,
        door: Some(door_math_dissection_fraction),
    },
    Function {
        name: "math.dissection.kappa",
        signature: "(base: u64, digits: [u64]) -> (u64, u64)",
        docs: "Returns `kappa_F = (base/phi(base)) #{f in F : gcd(f, base) = 1}/fill`, the main-term constant of the prime count, as a reduced fraction.",
        uncallable: None,
        door: Some(door_math_dissection_kappa),
    },
    Function {
        name: "math.dissection.masses",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "Returns the unshifted masses `c_j = sum_(a < base^j) |hat F_j(a/base^j)|` for `j = 0..=level`, the `l^1` mass region A pays, `c_0 = 1`.",
        uncallable: None,
        door: Some(door_math_dissection_masses),
    },
    Function {
        name: "math.dissection.reach",
        signature: "(base: u64, missing: u64) -> str",
        docs: "Returns how the theorem reaches the set missing one digit: `proof` from the chain's wall, `certificate` at every base from `DIGIT_WALL` below it and at the `CERTIFIED` sets, `none` elsewhere.",
        uncallable: None,
        door: Some(door_math_dissection_reach),
    },
    Function {
        name: "math.dissection.reading",
        signature: "(base: u64, fill: usize, masses: [f64]) -> f64",
        docs: "Returns the `l^1` exponent the top two masses read, `log_base(c_j/(fill c_(j-1)))`: a reading of the growth region A pays, never a certificate.",
        uncallable: None,
        door: Some(door_math_dissection_reading),
    },
    Function {
        name: "math.dissection.regions",
        signature: "(base: u64, level: u32, z: u64) -> [math.dissection.Region]",
        docs: "Returns the region of every frequency `a/y`, `a < y = base^level`, at the cut `Z`, the fraction taken by `fraction` at `cap`.",
        uncallable: None,
        door: Some(door_math_dissection_regions),
    },
    Function {
        name: "math.dissection.tally",
        signature: "(base: u64, digits: [u64], level: usize, samples: usize) -> math.dissection.Tally",
        docs: "Tallies the set below `base^level` on a log grid of the given size, with the Mobius values and the primes sieved to the span.",
        uncallable: None,
        door: Some(door_math_dissection_tally),
    },
    Function {
        name: "math.dissection.weights",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "Returns `|hat F_level(a/base^level)|` at every `a < base^level`, built one digit at a time from `hat F_j(t) = hat F(t) hat F_(j-1)(base t)`.",
        uncallable: None,
        door: Some(door_math_dissection_weights),
    },
    Function {
        name: "math.game.cell",
        signature: "(point: [f64], base: u8, level: u32) -> u64",
        docs: "Returns the index of the level-`k` cell holding a point of `[0, 1)^D`: `floor(x b^k) + floor(y b^k) b^k + ...`, row major with coordinate `0` least significant, the dimension read off the point.\n\nAt level one the index is the corner the point sits in. A coordinate outside the cube is clamped to its nearest cell.\n\n# Errors\n\nErrs under base two, and when the cell count runs past a `u64`.",
        uncallable: None,
        door: Some(door_math_game_cell),
    },
    Function {
        name: "math.game.cells",
        signature: "(base: u8, dimension: u8, level: u32) -> u64",
        docs: "Returns the count of level-`k` cells of the cube, `b^(D k)`.\n\n# Errors\n\nErrs under base two, and when the count runs past a `u64`.",
        uncallable: None,
        door: Some(door_math_game_cells),
    },
    Function {
        name: "math.game.counts",
        signature: "(matrix: [[u64]], start: usize, levels: usize) -> [u64]",
        docs: "Returns the path counts from a state, `e_start M^L 1` for `L = 1 ..= levels`: the filled cells of a mosaic slot or of a slice at each level, and stops early on the level whose count overruns a `u64`.\n\n# Errors\n\nErrs when the matrix is not square, and when the start is not one of its states.",
        uncallable: None,
        door: Some(door_math_game_counts),
    },
    Function {
        name: "math.game.de_bruijn",
        signature: "(m: u32, k: u32) -> [u8]",
        docs: "Returns a de Bruijn cycle of order `k` over `m` letters: `m^k` letters in which every word of length `k` occurs exactly once as a cyclic window, so played as the draw schedule of an `m`-digit design it paints every level-`k` cell once per lap, and from a cold start once in `m^k + k - 1` draws.\n\nThe cycle concatenates the Lyndon words over `{0, ..., m - 1}` whose length divides `k`, in lexicographic order, the construction of Fredricksen, Kessler and Maiorana.\n\n# Errors\n\nErrs when `m` or `k` is zero, when `m` passes the `256` letters a byte holds, and when `m^k` passes `LAP`.",
        uncallable: None,
        door: Some(door_math_game_de_bruijn),
    },
    Function {
        name: "math.game.mosaic_game",
        signature: "(mosaic: math.game.Mosaic) -> math.game.MosaicGame",
        docs: "Reads the game of a mosaic: its substitution matrix, Perron root, dimension and edges.\n\n# Errors\n\nErrs under base two or dimension one, with no mask or more than `256`, at a corner past `b^D` or repeated in its mask, and at a slot past the masks.",
        uncallable: None,
        door: Some(door_math_game_mosaic_game),
    },
    Function {
        name: "math.game.named_games",
        signature: "() -> [math.game.NamedGame]",
        docs: "Returns the named games of the square, corner `c = x + 2 y`, cyclic order `0, 1, 3, 2` anticlockwise with `y` up: the seven one-step games and the width-3 game that forbids a neighbour after a repeat.",
        uncallable: None,
        door: Some(door_math_game_named_games),
    },
    Function {
        name: "math.game.perron",
        signature: "(matrix: [[u64]]) -> f64",
        docs: "Returns the Perron root of a square nonnegative integer matrix, the growth per level of the paths its graph counts.\n\nThe graph is split into its strongly connected components; a component carrying no cycle contributes nothing, and a matrix with no cycle reads exactly `0`.\nOn each cyclic component `I + B` is primitive, so the power iteration on it converges to the Perron vector, and the Collatz-Wielandt ratios of that vector bracket the root; the answer is the largest component root.\n\n# Errors\n\nErrs when the matrix is not square.",
        uncallable: None,
        door: Some(door_math_game_perron),
    },
    Function {
        name: "math.game.reverse",
        signature: "(rule: math.memory.Rule) -> math.memory.Rule",
        docs: "Returns the rule whose windows are the rule's windows read backwards.\n\nA restricted game allows a window of draws in time order, oldest first, and the digits of its point are the draws newest first, so the set a game draws is the dial's set of the reversed rule: `crate::memory::cells` of the reverse are the game's cells, and the two share their Perron root, their transfer matrices being transposes.",
        uncallable: None,
        door: Some(door_math_game_reverse),
    },
    Function {
        name: "math.game.slice_game",
        signature: "(base: u8, digits: [u8], p: i64, q: i64) -> math.game.SliceGame",
        docs: "Reads the game of a plane design, corners `c = x + b y`, cut by the line `x + y = p / q`: its carry states, edges, matrix, Perron root and dimension.\n\n# Errors\n\nErrs under base two, at a corner past `b^2` or repeated, at a zero `q`, and when a height overruns an `i64`.",
        uncallable: None,
        door: Some(door_math_game_slice_game),
    },
    Function {
        name: "math.game.successors",
        signature: "(rule: math.memory.Rule, history: [u8]) -> [u8]",
        docs: "Returns the letters a restricted game may draw next: the corners `c` for which some allowed window, read in time order, ends in the last `k - 1` draws of the history followed by `c`.\n\nA history shorter than `k - 1` fixes only the end of the window, so the letters consistent with some allowed window are returned, and the empty history at width one reads the rule's alphabet.\nA history letter past the rule's corners matches no window.",
        uncallable: None,
        door: Some(door_math_game_successors),
    },
    Function {
        name: "math.game.word_cell",
        signature: "(word: [u8], base: u8, dimension: u8) -> u64",
        docs: "Returns the cell of a word of corners read coarsest digit first, the cell the game lands in after playing the word's digits last letter first.\n\n# Errors\n\nErrs under base two or dimension one, at a corner past `b^D`, and when the cell count runs past a `u64`.",
        uncallable: None,
        door: Some(door_math_game_word_cell),
    },
    Function {
        name: "math.graph.Layout.energy",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the mean net force per node in units of `k` after the last tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.from_network",
        signature: "(network: math.graph.Network, seed: u64) -> math.graph.Layout",
        docs: "Starts a layout from a network's own positions and branches.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ideal",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the ideal branch length `k`.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.moved",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the mean distance a node moved in the last tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.new",
        signature: "(positions: [f64], branches: [(usize, usize)], dim: usize, seed: u64) -> math.graph.Layout",
        docs: "Starts a layout from flat positions, `dim` floats per node, and the branch pairs.\n\n# Errors\n\nErrors when a branch points off the list, or the positions are not dim floats a node.",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.nodes",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "Returns the node count.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.positions",
        signature: "(self: math.graph.Layout) -> [f64]",
        docs: "Returns the positions, `dim` floats per node.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.step",
        signature: "(self: math.graph.Layout, ticks: usize) -> f64",
        docs: "Runs the ticks and returns the energy left: the mean net force per node in units of `k`.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.temperature",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the cap on one node's move in the next tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ticks",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "Returns the ticks stepped so far.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_branch",
        signature: "(self: math.graph.Network, parent: usize, child: usize, radius: f64) -> null",
        docs: "Appends a branch between two node indices.\n\n# Errors\n\nErrors when either index is out of range.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_node",
        signature: "(self: math.graph.Network, position: [f64]) -> usize",
        docs: "Appends a node at the position and returns its index.\n\n# Errors\n\nErrors on a dimension mismatch.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.adjacency",
        signature: "(self: math.graph.Network) -> {usize: [usize]}",
        docs: "Returns the undirected neighbor lists of every node.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_network_adjacency),
    },
    Function {
        name: "math.graph.Network.degree",
        signature: "(self: math.graph.Network) -> [usize]",
        docs: "Returns each node's branch count, indexed like the node list.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_network_degree),
    },
    Function {
        name: "math.graph.Network.new",
        signature: "(dim: usize) -> math.graph.Network",
        docs: "Builds an empty network of the given dimension.",
        uncallable: None,
        door: Some(door_math_graph_network_new),
    },
    Function {
        name: "math.graph.census",
        signature: "(network: math.graph.Network) -> math.graph.Census",
        docs: "Takes the full census of a network.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_census),
    },
    Function {
        name: "math.graph.components",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the connected components of the network.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_components),
    },
    Function {
        name: "math.graph.core_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the network of filled sites joined to their axis neighbors.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_core_graph),
    },
    Function {
        name: "math.graph.edge_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the network of corners and edges outlining every filled site.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_edge_graph),
    },
    Function {
        name: "math.graph.fractal_dimension",
        signature: "(network: math.graph.Network, samples: usize) -> f64",
        docs: "Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.",
        uncallable: None,
        door: Some(door_math_graph_fractal_dimension),
    },
    Function {
        name: "math.graph.junctions",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the nodes of degree three or more.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_junctions),
    },
    Function {
        name: "math.graph.largest_component",
        signature: "(network: math.graph.Network) -> math.graph.Network",
        docs: "Extracts the largest connected piece as a network of its own, branches re-indexed.\n\nTies go to the piece whose lowest node index comes first. An empty network comes back empty.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_largest_component),
    },
    Function {
        name: "math.graph.roles",
        signature: "(network: math.graph.Network) -> [math.graph.Role]",
        docs: "Tags every node by its degree, indexed like the node list.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_roles),
    },
    Function {
        name: "math.graph.tips",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the nodes of degree one.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_tips),
    },
    Function {
        name: "math.graph.total_length",
        signature: "(network: math.graph.Network) -> f64",
        docs: "Sums the straight-line lengths of every branch.",
        uncallable: None,
        door: Some(door_math_graph_total_length),
    },
    Function {
        name: "math.graph.tunnel_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the core graph of the inverted grid, joining empty sites instead.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_tunnel_graph),
    },
    Function {
        name: "math.ladder.Design.abscissa",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "Returns the abscissa `alpha = log_q k`.",
        uncallable: None,
        door: Some(door_math_ladder_design_abscissa),
    },
    Function {
        name: "math.ladder.Design.base",
        signature: "(self: math.ladder.Design) -> u64",
        docs: "Returns the base.",
        uncallable: None,
        door: Some(door_math_ladder_design_base),
    },
    Function {
        name: "math.ladder.Design.digits",
        signature: "(self: math.ladder.Design) -> [u64]",
        docs: "Returns the digit set, ascending.",
        uncallable: None,
        door: Some(door_math_ladder_design_digits),
    },
    Function {
        name: "math.ladder.Design.new",
        signature: "(base: u64, digits: [u64]) -> math.ladder.Design",
        docs: "Builds a design on the base and the digit set, choosing the peel depth.\n\n# Errors\n\nErrs at a base under two, at fewer than two digits, at a digit outside the base, or at an all-zero digit set.",
        uncallable: None,
        door: Some(door_math_ladder_design_new),
    },
    Function {
        name: "math.ladder.Design.peel",
        signature: "(self: math.ladder.Design) -> usize",
        docs: "Returns the peel depth.",
        uncallable: None,
        door: Some(door_math_ladder_design_peel),
    },
    Function {
        name: "math.ladder.Design.period",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "Returns the pole spacing `2 pi / log q`.",
        uncallable: None,
        door: Some(door_math_ladder_design_period),
    },
    Function {
        name: "math.ladder.Design.pole",
        signature: "(self: math.ladder.Design, m: usize, j: i64) -> num.zeta.Complex",
        docs: "Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.",
        uncallable: None,
        door: Some(door_math_ladder_design_pole),
    },
    Function {
        name: "math.ladder.Design.with_peel",
        signature: "(base: u64, digits: [u64], peel: usize) -> math.ladder.Design",
        docs: "Builds a design at an explicit peel depth, at least two.\n\n# Errors\n\nErrs at a bad base or digit set, and at a peel depth under two or past the exact integers.",
        uncallable: None,
        door: Some(door_math_ladder_design_with_peel),
    },
    Function {
        name: "math.ladder.cofactor",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.\n\nThe cofactor is the ladder numerator over the finite polynomial, so it is analytic on `Re s > alpha - 1` and a zero census on it needs no pole-free strip.\nThe blind lattice here is `s = alpha - m + 2 pi i j / log q` for a whole `m >= 1`: the numerator itself is read at `m = 0`, which is what makes the residue available.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when the point sits on the blind lattice.",
        uncallable: None,
        door: Some(door_math_ladder_cofactor),
    },
    Function {
        name: "math.ladder.residue",
        signature: "(design: math.ladder.Design, m: usize, j: i64, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.\n\nAt `m = 0` the factor `1 - k q^(-s)` has derivative `log q`, so the residue is the ladder numerator over `log q`; the column below comes off the same recursion, `(1 - q^m) R_m = sum_(l = 1)^m binom(-s_(m,j), l) q^(-s_(m,j)-l) gamma_l R_(m-l)`.\nThe head term `(1 - k q^(-s)) D_(P-1)(s)` is dropped, since the factor vanishes at the pole; in double precision the factor is not exactly zero, and the residue of the dropped term, about `1e-15` at base 10, is not charged separately and sits inside the carried scale.\n\n# Errors\n\nErrs when the tolerance is out of reach.",
        uncallable: None,
        door: Some(door_math_ladder_residue),
    },
    Function {
        name: "math.ladder.zeta",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns `zeta_F(s)` and the bound it is known to.\n\nThe bound is the propagated truncation bound, which is proved, plus `ROUNDING` times the scale the ladder carries, which is a measured allowance; the acceptance test charges every term the returned bound carries, the Dirichlet polynomial's own scale included, so the returned bound is never above the tolerance asked.\nThe recursion walks `w = s + j` upward and divides by `1 - k q^(-w)`, so it cannot read `s = alpha - m + 2 pi i j / log q` for a whole `m >= 0`: at those points it raises and names the pole. On a full digit set that lattice is `s = 1, 0, -1, -2, ...`, where `zeta_F` is `zeta` and only `s = 1` is singular.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when the point sits on the blind lattice.",
        uncallable: None,
        door: Some(door_math_ladder_zeta),
    },
    Function {
        name: "math.memory.Rule.accepts",
        signature: "(self: math.memory.Rule, word: [usize]) -> bool",
        docs: "Returns whether a word, coarsest digit first, is accepted.",
        uncallable: None,
        door: Some(door_math_memory_rule_accepts),
    },
    Function {
        name: "math.memory.Rule.allowed",
        signature: "(self: math.memory.Rule, window: usize) -> bool",
        docs: "Returns whether the window is allowed, and false for any window out of range.",
        uncallable: None,
        door: Some(door_math_memory_rule_allowed),
    },
    Function {
        name: "math.memory.Rule.alphabet",
        signature: "(self: math.memory.Rule) -> [usize]",
        docs: "Returns the letters that stand in at least one allowed window.",
        uncallable: None,
        door: Some(door_math_memory_rule_alphabet),
    },
    Function {
        name: "math.memory.Rule.codes",
        signature: "(self: math.memory.Rule) -> u128",
        docs: "Returns the count of rules of this shape, `2^(2^(k D))`.",
        uncallable: None,
        door: Some(door_math_memory_rule_codes),
    },
    Function {
        name: "math.memory.Rule.full",
        signature: "(dimension: usize, width: usize) -> math.memory.Rule",
        docs: "Returns the rule that allows every window.\n\n# Errors\n\nErrs when the dimension is not one, two or three, the width is zero, or the window count overruns.",
        uncallable: None,
        door: Some(door_math_memory_rule_full),
    },
    Function {
        name: "math.memory.Rule.letters",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the letter count `2^D`, the digit vectors of the cube's corners.",
        uncallable: None,
        door: Some(door_math_memory_rule_letters),
    },
    Function {
        name: "math.memory.Rule.new",
        signature: "(dimension: usize, width: usize, code: u64) -> math.memory.Rule",
        docs: "Builds a rule from its dimension, its width and its code.\n\n# Errors\n\nErrs when the dimension is not one, two or three, the width is zero, the window count overruns, or the code runs past it.",
        uncallable: None,
        door: Some(door_math_memory_rule_new),
    },
    Function {
        name: "math.memory.Rule.states",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.",
        uncallable: None,
        door: Some(door_math_memory_rule_states),
    },
    Function {
        name: "math.memory.Rule.windows",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the window count `2^(k D)`.",
        uncallable: None,
        door: Some(door_math_memory_rule_windows),
    },
    Function {
        name: "math.memory.allowed_windows",
        signature: "(rule: math.memory.Rule) -> usize",
        docs: "Returns the count of allowed windows `card W`, the bits the code sets inside its window range.",
        uncallable: None,
        door: Some(door_math_memory_allowed_windows),
    },
    Function {
        name: "math.memory.cells",
        signature: "(rule: math.memory.Rule, level: usize) -> [u64]",
        docs: "Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.\n\nThe index is row major, `z side^2 + y side + x` with `side = 2^L`, so at `k = 1` the set is the level-`L` fill of the design of the same code.",
        uncallable: None,
        door: Some(door_math_memory_cells),
    },
    Function {
        name: "math.memory.counts",
        signature: "(rule: math.memory.Rule, levels: usize) -> [u64]",
        docs: "Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.\n\nFor `L < k` every word of that length is accepted, so the count is `2^(D L)`.",
        uncallable: None,
        door: Some(door_math_memory_counts),
    },
    Function {
        name: "math.memory.exponent",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.",
        uncallable: None,
        door: Some(door_math_memory_exponent),
    },
    Function {
        name: "math.memory.kappa",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.\n\nThe window budget `log_2(card W) / k` is what a word of length `mk` could carry if its `m` disjoint windows were free of each other, and `log_2 rho` is what it really carries, so `kappa >= 0` and the gap is the interference the rule pays for.\nIt is zero on every product rule `W = F^k`, where `card W = card F^k` and `rho = card F`, so every `k = 1` rule reads zero; it is `f64::INFINITY` when a window is allowed and nothing survives past the window, and zero when no window is allowed.",
        uncallable: None,
        door: Some(door_math_memory_kappa),
    },
    Function {
        name: "math.memory.perron",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the Perron root of the transfer matrix, the count's growth per level.\n\nThe digraph is split into its strongly connected components first; a component carrying no cycle contributes nothing, and a rule whose components all die past the window returns exactly `0`.\nEach cyclic component is irreducible, so `I + B` is primitive and the power iteration on it converges geometrically; the iteration stops on a sustained Cauchy move of the normalised vector, never on a plateau of one scalar, and the Collatz-Wielandt ratios bracket the root.\nThe estimate is then made exact against the component's characteristic polynomial, taken by integer Faddeev-LeVerrier: an integer root is returned exactly, and otherwise the simple Perron root is bisected to the resolution of an `f64`.\nThe answer is the largest component root.",
        uncallable: None,
        door: Some(door_math_memory_perron),
    },
    Function {
        name: "math.memory.transfer",
        signature: "(rule: math.memory.Rule) -> [[u64]]",
        docs: "Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.\n\nAt `k = 1` the one state is the empty window and the single entry counts the allowed letters.",
        uncallable: None,
        door: Some(door_math_memory_transfer),
    },
    Function {
        name: "math.moire.Field.as_f64",
        signature: "(self: math.moire.Field) -> [f64]",
        docs: "Returns the samples widened to f64.",
        uncallable: None,
        door: Some(door_math_moire_field_as_f64),
    },
    Function {
        name: "math.moire.Field.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Field",
        docs: "Wraps row-major samples of the given side.\n\n# Errors\n\nErrors when the count is not the side squared.",
        uncallable: None,
        door: Some(door_math_moire_field_from_data),
    },
    Function {
        name: "math.moire.Field.max",
        signature: "(self: math.moire.Field) -> f32",
        docs: "Returns the largest sample.",
        uncallable: None,
        door: Some(door_math_moire_field_max),
    },
    Function {
        name: "math.moire.Field.mean",
        signature: "(self: math.moire.Field) -> f64",
        docs: "Returns the mean sample, or zero for an empty field.",
        uncallable: None,
        door: Some(door_math_moire_field_mean),
    },
    Function {
        name: "math.moire.Field.min",
        signature: "(self: math.moire.Field) -> f32",
        docs: "Returns the smallest sample.",
        uncallable: None,
        door: Some(door_math_moire_field_min),
    },
    Function {
        name: "math.moire.Field.new",
        signature: "(size: usize) -> math.moire.Field",
        docs: "Builds a zeroed field of the given side.",
        uncallable: None,
        door: Some(door_math_moire_field_new),
    },
    Function {
        name: "math.moire.Field.normalized",
        signature: "(self: math.moire.Field, symmetric: bool) -> [f32]",
        docs: "Returns the samples scaled into 0..1, symmetric about zero on request.",
        uncallable: None,
        door: Some(door_math_moire_field_normalized),
    },
    Function {
        name: "math.moire.Layer.new",
        signature: "(spec: math.moire.Spec, number: usize) -> math.moire.Layer",
        docs: "Builds a layer at level 1 on a 512-pixel square lattice.",
        uncallable: None,
        door: Some(door_math_moire_layer_new),
    },
    Function {
        name: "math.moire.Preset.carpet",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The carpet stack: every base-three corner but the centre, summed over odd scales.",
        uncallable: None,
        door: Some(door_math_moire_preset_carpet),
    },
    Function {
        name: "math.moire.Preset.field",
        signature: "(self: math.moire.Preset, size: usize) -> math.moire.Field",
        docs: "Samples the preset into a square field of the given side.\n\n# Errors\n\nErrors when the preset's code is out of range.",
        uncallable: Some("math::moire::Preset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.moire.Preset.heatmap",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The parity heatmap: odd scales of the low corner summed on the square lattice.",
        uncallable: None,
        door: Some(door_math_moire_preset_heatmap),
    },
    Function {
        name: "math.moire.Preset.hive",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The hive: the parity heatmap sampled on the hexagonal lattice.",
        uncallable: None,
        door: Some(door_math_moire_preset_hive),
    },
    Function {
        name: "math.moire.Preset.weave",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The parity weave: the same odd scales folded to their parity instead of summed.",
        uncallable: None,
        door: Some(door_math_moire_preset_weave),
    },
    Function {
        name: "math.moire.Spec.new",
        signature: "(code: u128, base: usize, dimension: usize) -> math.moire.Spec",
        docs: "Builds a spec from a code, base and dimension.",
        uncallable: None,
        door: Some(door_math_moire_spec_new),
    },
    Function {
        name: "math.moire.Volume.at",
        signature: "(self: math.moire.Volume, x: usize, y: usize, z: usize) -> f32",
        docs: "Reads the sample at a voxel.",
        uncallable: None,
        door: Some(door_math_moire_volume_at),
    },
    Function {
        name: "math.moire.Volume.count",
        signature: "(self: math.moire.Volume, level: f32) -> usize",
        docs: "Counts the samples at or above the level.",
        uncallable: None,
        door: Some(door_math_moire_volume_count),
    },
    Function {
        name: "math.moire.Volume.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Volume",
        docs: "Wraps x-major samples of the side.\n\n# Errors\n\nErrors when the count is not the side cubed.",
        uncallable: None,
        door: Some(door_math_moire_volume_from_data),
    },
    Function {
        name: "math.moire.Volume.max",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "Returns the largest sample.",
        uncallable: None,
        door: Some(door_math_moire_volume_max),
    },
    Function {
        name: "math.moire.Volume.min",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "Returns the smallest sample.",
        uncallable: None,
        door: Some(door_math_moire_volume_min),
    },
    Function {
        name: "math.moire.Volume.new",
        signature: "(size: usize) -> math.moire.Volume",
        docs: "Builds a zeroed volume of the side.",
        uncallable: None,
        door: Some(door_math_moire_volume_new),
    },
    Function {
        name: "math.moire.Volume.plane",
        signature: "(self: math.moire.Volume, frame: math.moire.Frame, out: usize) -> ([f32], [u8])",
        docs: "Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.",
        uncallable: None,
        door: Some(door_math_moire_volume_plane),
    },
    Function {
        name: "math.moire.Volume.sample",
        signature: "(self: math.moire.Volume, p: [f64; 3]) -> f32?",
        docs: "Reads the voxel a point of the unit cube falls in, or zero outside it.",
        uncallable: None,
        door: Some(door_math_moire_volume_sample),
    },
    Function {
        name: "math.moire.Volume.solid",
        signature: "(self: math.moire.Volume, level: f32) -> Tensor",
        docs: "Thresholds into a byte tensor: one where a sample reaches the level, zero below.",
        uncallable: None,
        door: Some(door_math_moire_volume_solid),
    },
    Function {
        name: "math.moire.all",
        signature: "(limit: usize) -> [math.moire.Preset]",
        docs: "Returns every preset stacked up to the given scale.",
        uncallable: None,
        door: Some(door_math_moire_all),
    },
    Function {
        name: "math.moire.frame",
        signature: "(normal: [f64; 3], offset: f64) -> math.moire.Frame",
        docs: "Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.\n\n# Errors\n\nErrors on a zero normal.",
        uncallable: None,
        door: Some(door_math_moire_frame),
    },
    Function {
        name: "math.moire.layer",
        signature: "(params: math.moire.Layer) -> [bool]",
        docs: "Samples a design over the pixel grid into a boolean mask.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_layer),
    },
    Function {
        name: "math.moire.named",
        signature: "(name: str, limit: usize) -> math.moire.Preset",
        docs: "Returns the preset the name picks.\n\n# Errors\n\nErrors for a name that is not heatmap, weave, hive or carpet.",
        uncallable: None,
        door: Some(door_math_moire_named),
    },
    Function {
        name: "math.moire.pairs.correlation",
        signature: "(m: usize, n: usize) -> f64",
        docs: "Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.\n\nA layer at scale n is lit where the row and the column of the n by n grid are not both odd.\nThe correlation is exactly zero when the two odd scales are coprime, and zero by convention below scale two, where a layer is constant.",
        uncallable: None,
        door: Some(door_math_moire_pairs_correlation),
    },
    Function {
        name: "math.moire.pairs.sampled",
        signature: "(m: usize, n: usize) -> f64",
        docs: "Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.\n\n# Errors\n\nErrors when either scale is zero.",
        uncallable: None,
        door: Some(door_math_moire_pairs_sampled),
    },
    Function {
        name: "math.moire.pairs.witness",
        signature: "(scale: usize) -> math.moire.pairs.Witness",
        docs: "Puts an odd scale of three or more on trial against every earlier odd scale.\n\n# Errors\n\nErrors for a scale that is not odd and three or more.",
        uncallable: None,
        door: Some(door_math_moire_pairs_witness),
    },
    Function {
        name: "math.moire.render",
        signature: "(field: math.moire.Field, colorizer: core.Colorizer, levels: usize, symmetric: bool, invert: bool, scale: usize) -> Image",
        docs: "Quantizes a field into colored levels and renders them as an image, each sample a scale by scale block.\n\n# Errors\n\nErrors at scale zero.",
        uncallable: None,
        door: Some(door_math_moire_render),
    },
    Function {
        name: "math.moire.sample.axes",
        signature: "(size: usize, lattice: math.moire.Lattice, row: usize) -> ([f64], [f64])",
        docs: "Returns the two lattice coordinates of each pixel centre along a row.",
        uncallable: None,
        door: Some(door_math_moire_sample_axes),
    },
    Function {
        name: "math.moire.sample.membership",
        signature: "(code: u128, base: usize, dimension: usize) -> [bool]",
        docs: "Unpacks a code into its residue-corner truth table.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_moire_sample_membership),
    },
    Function {
        name: "math.moire.sample.pack",
        signature: "(residues: [usize], base: usize) -> usize",
        docs: "Folds residues into a base-q index of the truth table.",
        uncallable: None,
        door: Some(door_math_moire_sample_pack),
    },
    Function {
        name: "math.moire.stack",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "Layers one design at several side numbers into a field under the chosen combine.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_stack),
    },
    Function {
        name: "math.moire.stack_codes",
        signature: "(specs: [math.moire.Spec], number: usize, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "Sums layers of several designs at one side number into a field.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_stack_codes),
    },
    Function {
        name: "math.moire.volume",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, size: usize) -> math.moire.Volume",
        docs: "Layers one cube design at several side numbers into a volume under the chosen combine.\n\n# Errors\n\nErrors at size zero, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_volume),
    },
    Function {
        name: "math.name.Bang.cells",
        signature: "(self: math.name.Bang) -> u32",
        docs: "Returns the number of digits the code addresses.\n\n# Errors\n\nErrors below dimension one, below base two, or past the u128 code space.",
        uncallable: None,
        door: Some(door_math_name_bang_cells),
    },
    Function {
        name: "math.name.Bang.checked",
        signature: "(self: math.name.Bang) -> math.name.Bang",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_bang_checked),
    },
    Function {
        name: "math.name.Bang.from_file",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_bang_from_file),
    },
    Function {
        name: "math.name.Bang.from_json",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_bang_from_json),
    },
    Function {
        name: "math.name.Bang.from_url",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_bang_from_url),
    },
    Function {
        name: "math.name.Bang.new",
        signature: "(code: u128, dim: usize, base: usize) -> math.name.Bang",
        docs: "Pins a code to its dimension and base on the square lattice.",
        uncallable: None,
        door: Some(door_math_name_bang_new),
    },
    Function {
        name: "math.name.Bang.to_file",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_file),
    },
    Function {
        name: "math.name.Bang.to_id",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_bang_to_id),
    },
    Function {
        name: "math.name.Bang.to_json",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_bang_to_json),
    },
    Function {
        name: "math.name.Bang.to_prose",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_prose),
    },
    Function {
        name: "math.name.Bang.to_url",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_url),
    },
    Function {
        name: "math.name.Lattice.default",
        signature: "() -> math.name.Lattice",
        docs: "Returns the default Lattice.",
        uncallable: None,
        door: Some(door_math_name_lattice_default),
    },
    Function {
        name: "math.name.Lattice.is_square",
        signature: "(self: math.name.Lattice) -> bool",
        docs: "Returns whether this is the square lattice.",
        uncallable: None,
        door: Some(door_math_name_lattice_is_square),
    },
    Function {
        name: "math.name.Lattice.units",
        signature: "(self: math.name.Lattice) -> usize",
        docs: "Returns the number of unit directions a twist may pick from.",
        uncallable: None,
        door: Some(door_math_name_lattice_units),
    },
    Function {
        name: "math.name.Sequence.checked",
        signature: "(self: math.name.Sequence) -> math.name.Sequence",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_sequence_checked),
    },
    Function {
        name: "math.name.Sequence.design",
        signature: "(self: math.name.Sequence) -> math.name.Bang",
        docs: "Returns the design pinned to its dimension and base.",
        uncallable: None,
        door: Some(door_math_name_sequence_design),
    },
    Function {
        name: "math.name.Sequence.from_file",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_file),
    },
    Function {
        name: "math.name.Sequence.from_json",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_json),
    },
    Function {
        name: "math.name.Sequence.from_url",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_url),
    },
    Function {
        name: "math.name.Sequence.new",
        signature: "(code: u128, dim: usize, base: usize, measure: str, axis: str) -> math.name.Sequence",
        docs: "Pins a design's reading to its measure and axis.",
        uncallable: None,
        door: Some(door_math_name_sequence_new),
    },
    Function {
        name: "math.name.Sequence.to_file",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_file),
    },
    Function {
        name: "math.name.Sequence.to_id",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_id),
    },
    Function {
        name: "math.name.Sequence.to_json",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_json),
    },
    Function {
        name: "math.name.Sequence.to_prose",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_prose),
    },
    Function {
        name: "math.name.Sequence.to_url",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_url),
    },
    Function {
        name: "math.name.Word.bases",
        signature: "(self: math.name.Word) -> [usize]",
        docs: "Returns the base of every letter, 2 where the name says nothing.",
        uncallable: None,
        door: Some(door_math_name_word_bases),
    },
    Function {
        name: "math.name.Word.checked",
        signature: "(self: math.name.Word) -> math.name.Word",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_word_checked),
    },
    Function {
        name: "math.name.Word.from_file",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_word_from_file),
    },
    Function {
        name: "math.name.Word.from_json",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_word_from_json),
    },
    Function {
        name: "math.name.Word.from_url",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_word_from_url),
    },
    Function {
        name: "math.name.Word.letters",
        signature: "(self: math.name.Word) -> [math.name.Bang]",
        docs: "Returns every letter as a design pinned to the word's dimension and its own base.",
        uncallable: None,
        door: Some(door_math_name_word_letters),
    },
    Function {
        name: "math.name.Word.new",
        signature: "(dim: usize, letters: [(u128, usize)]) -> math.name.Word",
        docs: "Pins an ordered letter list at base 2.\n\n# Errors\n\nErrors below two letters, or outside the code range.",
        uncallable: None,
        door: Some(door_math_name_word_new),
    },
    Function {
        name: "math.name.Word.to_file",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_file),
    },
    Function {
        name: "math.name.Word.to_id",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_word_to_id),
    },
    Function {
        name: "math.name.Word.to_json",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_word_to_json),
    },
    Function {
        name: "math.name.Word.to_prose",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_prose),
    },
    Function {
        name: "math.name.Word.to_url",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_url),
    },
    Function {
        name: "math.name.of",
        signature: "(code: Code, dim: usize, base: usize) -> String?",
        docs: "Returns the word of the first named design a code draws in the plane or the cube, or None for a code no named design draws.",
        uncallable: None,
        door: Some(door_math_name_of),
    },
    Function {
        name: "math.name.words",
        signature: "(dim: usize, base: usize) -> [(String, String)]",
        docs: "Lists every classic then anti design of the plane or the cube as its decimal code and its word, empty for any other dimension.",
        uncallable: None,
        door: Some(door_math_name_words),
    },
    Function {
        name: "math.nodal.figures",
        signature: "(code: u64, number: usize, level: usize, count: usize) -> math.nodal.Figures",
        docs: "Rings the plate cut as the design `math::two::create(code, number, level, 0, 2)`: its cells joined to their four neighbours, free at the rim and at every hole, the combinatorial Laplacian `D - A`. Returns the lowest `count` modes, `count` clamped to the cells and to 64, each in a canonical basis: eigenvalues within `1e-8` form a cluster, split by the design's transpose, left-right and up-down mirrors in that order and then by the diagonal `x + sqrt(2) y`, each figure projected exactly onto its mirror parities and signed so its first largest cell is positive. A figure's domains are the 4-connected runs of one strict sign, cells within `1e-9` of its peak left out; its bound is the 1-based index of the last member of its cluster, the last cluster closed before the cut.\n\n# Errors\n\nErrs on what `holds` refuses, or when the solver does not settle.",
        uncallable: None,
        door: Some(door_math_nodal_figures),
    },
    Function {
        name: "math.nodal.holds",
        signature: "(code: u64, number: usize, level: usize) -> bool",
        docs: "Answers whether a 2D base-2 design cuts a plate the figures can ring: one 4-connected piece of 16 to 10,000 cells on a side of at most 128. The cells are counted before the design is built.\n\n# Errors\n\nErrs when the code is not a 2D base-2 code, 0 to 15.",
        uncallable: None,
        door: Some(door_math_nodal_holds),
    },
    Function {
        name: "math.press.Press.add",
        signature: "(self: math.press.Press, number: u128, weight: i128) -> null",
        docs: "Adds a weighted number to its usage bucket.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.press.Press.new",
        signature: "(dimension: usize, base: usize) -> math.press.Press",
        docs: "Builds an empty press over every design of the dimension and base.\n\n# Errors\n\nErrors past twenty corners, where the bucket table would leave a million rows.",
        uncallable: None,
        door: Some(door_math_press_press_new),
    },
    Function {
        name: "math.press.Press.total",
        signature: "(self: math.press.Press, code: Code) -> i128",
        docs: "Returns the total weight the design at a code has collected.",
        uncallable: None,
        door: Some(door_math_press_press_total),
    },
    Function {
        name: "math.press.Press.totals",
        signature: "(self: math.press.Press) -> [i128]",
        docs: "Returns every design's total in code order by one subset-sum transform.",
        uncallable: None,
        door: Some(door_math_press_press_totals),
    },
    Function {
        name: "math.press.containing",
        signature: "(number: u128, dimension: usize, base: usize) -> u128",
        docs: "Returns the number of designs of the dimension and base that contain the number.\n\nA design contains the number exactly when it fills every used corner, so the count\nis two to the free corners, and the average membership over all designs is one over\ntwo to the distinct-vector count.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_containing),
    },
    Function {
        name: "math.press.coordinates",
        signature: "(number: u128, dimension: usize, base: usize) -> [u128]",
        docs: "Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_coordinates),
    },
    Function {
        name: "math.press.count_below",
        signature: "(code: Code, dimension: usize, base: usize, limit: u128) -> u128",
        docs: "Counts the members of a design below the limit.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_count_below),
    },
    Function {
        name: "math.press.distinct",
        signature: "(number: u128, dimension: usize, base: usize) -> u32",
        docs: "Returns the count of distinct digit vectors the number uses.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_distinct),
    },
    Function {
        name: "math.press.interleave",
        signature: "(coords: [u128], base: usize) -> u128",
        docs: "Weaves dimension coordinates back into their single interleaved number.\n\n# Errors\n\nErrors on an empty coordinate list, past a hundred and twenty-seven corners, or when the woven number passes a u128.",
        uncallable: None,
        door: Some(door_math_press_interleave),
    },
    Function {
        name: "math.press.layer_table",
        signature: "(layer: math.bang.MagicLayer) -> [bool]",
        docs: "Returns the allowed digit table of one magic layer, one flag per cell of its tile.\n\nA cell is allowed when its coordinate residues form a filled corner, which is the\ntile the layer renders read as a digit alphabet.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_layer_table),
    },
    Function {
        name: "math.press.member",
        signature: "(code: Code, number: u128, dimension: usize, base: usize) -> bool",
        docs: "Returns whether every digit vector of the number lies in the design.\n\nThis is the scalar membership rule of the sequence press: the number's base\n`base` to the dimension digits, read as residue corners, must all be filled.\nAt dimension one it is the classic restricted-digit set.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_member),
    },
    Function {
        name: "math.press.members",
        signature: "(code: Code, dimension: usize, base: usize, count: usize) -> [u128]",
        docs: "Returns the first members of a design in ascending order.\n\nStops early where the next member would pass a hundred and twenty-eight bits.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_members),
    },
    Function {
        name: "math.press.profile",
        signature: "(code: Code, dimension: usize, base: usize, level: usize) -> [u128]",
        docs: "Returns the diagonal slice profile of one design pressed to a fractal level.\n\n# Errors\n\nErrors below level one, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_press_profile),
    },
    Function {
        name: "math.press.usage",
        signature: "(number: u128, dimension: usize, base: usize) -> Code",
        docs: "Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.\n\nA number is read in base `base` to the power of the dimension, so each digit is one\nresidue corner of the design cube, and zero uses exactly the zero corner.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_usage),
    },
    Function {
        name: "math.press.word_count",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Counts the members of a magic word from its layer fills, without enumeration.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_count),
    },
    Function {
        name: "math.press.word_member",
        signature: "(layers: [math.bang.MagicLayer], number: u128) -> bool",
        docs: "Returns whether the number lies in the magic word's composed design.\n\nThe number is read in the word's mixed radix, one digit per layer with the first\nlayer most significant, and every digit must land on an allowed cell of its tile.\nA number past the word's domain is an error.\n\n# Errors\n\nErrors on a code out of range, or a number past the word's domain.",
        uncallable: None,
        door: Some(door_math_press_word_member),
    },
    Function {
        name: "math.press.word_members",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Enumerates every member of the magic word in ascending order.\n\nThe member count is the product of the layer fills, so measure with `word_count`\nbefore pressing a word too rich to hold.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_members),
    },
    Function {
        name: "math.press.word_profile",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Returns the diagonal slice profile of a magic word by the substitution product.\n\nThe profile of a tile lists, per coordinate sum, its filled cells, and the profile\nof a Kronecker word is the product of its layer profiles with strides, so no cell\nof the composed design is ever enumerated.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_profile),
    },
    Function {
        name: "math.radix.Base.class",
        signature: "(self: math.radix.Base, z: (i64, i64)) -> usize",
        docs: "Returns the index in the canonical residue system of the class of a point.\n\n# Errors\n\nErrs when the residue system is short, or when the point lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_base_class),
    },
    Function {
        name: "math.radix.Base.congruent",
        signature: "(self: math.radix.Base, z: (i64, i64), w: (i64, i64)) -> bool",
        docs: "Returns whether two points are congruent modulo the base.",
        uncallable: None,
        door: Some(door_math_radix_base_congruent),
    },
    Function {
        name: "math.radix.Base.group",
        signature: "(self: math.radix.Base) -> [[usize]]",
        docs: "Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.\n\nMultiplying every digit by a unit `v` carries the attractor of a design to `v` times that attractor over the same base, so the unit group acts; conjugation carries the base to its conjugate and joins the group exactly when that is an associate.\n\nThe list is the Burnside multiset, one entry per abstract group element, and not the order of the permutation group it induces: the action need not be faithful, so entries repeat, and the acting image is the deduplicated list.\n\n# Errors\n\nErrs when the residue system is short, or when an image lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_base_group),
    },
    Function {
        name: "math.radix.Base.mirrored",
        signature: "(self: math.radix.Base) -> bool",
        docs: "Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.",
        uncallable: None,
        door: Some(door_math_radix_base_mirrored),
    },
    Function {
        name: "math.radix.Base.new",
        signature: "(ring: num.gauss.Ring, value: (i64, i64)) -> math.radix.Base",
        docs: "Fixes a base in a ring.\n\n# Errors\n\nErrs below norm two.",
        uncallable: None,
        door: Some(door_math_radix_base_new),
    },
    Function {
        name: "math.radix.Base.norm",
        signature: "(self: math.radix.Base) -> u64",
        docs: "Returns the norm `q` of the base: the count of residue classes and the square of the scale.",
        uncallable: None,
        door: Some(door_math_radix_base_norm),
    },
    Function {
        name: "math.radix.Base.power",
        signature: "(self: math.radix.Base, level: usize) -> (i64, i64)",
        docs: "Returns the base raised to a level.",
        uncallable: None,
        door: Some(door_math_radix_base_power),
    },
    Function {
        name: "math.radix.Base.residues",
        signature: "(self: math.radix.Base) -> [(i64, i64)]",
        docs: "Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.\n\nThe system is built greedily over every point of norm at most `q` sorted by norm and then by argument, which reaches every class because the covering radius of the lattice `b R` is `|b| / sqrt(2)` on the square lattice and `|b| / sqrt(3)` on the hexagonal.\n\n# Errors\n\nErrs when the greedy sweep finds fewer classes than the norm.",
        uncallable: None,
        door: Some(door_math_radix_base_residues),
    },
    Function {
        name: "math.radix.Base.ring",
        signature: "(self: math.radix.Base) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_math_radix_base_ring),
    },
    Function {
        name: "math.radix.Base.value",
        signature: "(self: math.radix.Base) -> (i64, i64)",
        docs: "Returns the base element.",
        uncallable: None,
        door: Some(door_math_radix_base_value),
    },
    Function {
        name: "math.radix.Radix.base",
        signature: "(self: math.radix.Radix) -> math.radix.Base",
        docs: "Returns the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_base),
    },
    Function {
        name: "math.radix.Radix.canonical",
        signature: "(self: math.radix.Radix) -> bool",
        docs: "Returns whether every digit is the canonical representative of its class.\n\n# Errors\n\nErrs when the residue system is short.",
        uncallable: None,
        door: Some(door_math_radix_radix_canonical),
    },
    Function {
        name: "math.radix.Radix.code",
        signature: "(self: math.radix.Radix) -> u128",
        docs: "Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.\n\n# Errors\n\nErrs when the residue system is short, or when a digit lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_radix_code),
    },
    Function {
        name: "math.radix.Radix.digits",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "Returns the digits.",
        uncallable: None,
        door: Some(door_math_radix_radix_digits),
    },
    Function {
        name: "math.radix.Radix.dimension",
        signature: "(self: math.radix.Radix) -> f64",
        docs: "Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_dimension),
    },
    Function {
        name: "math.radix.Radix.distinct",
        signature: "(self: math.radix.Radix, level: usize) -> usize",
        docs: "Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.",
        uncallable: None,
        door: Some(door_math_radix_radix_distinct),
    },
    Function {
        name: "math.radix.Radix.fill",
        signature: "(self: math.radix.Radix, level: usize) -> u128",
        docs: "Returns the count of words of a level, `|F|^L`.",
        uncallable: None,
        door: Some(door_math_radix_radix_fill),
    },
    Function {
        name: "math.radix.Radix.from_code",
        signature: "(base: math.radix.Base, code: u128) -> math.radix.Radix",
        docs: "Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.\n\n# Errors\n\nErrs when the class count passes 128, and when the code runs past the classes.",
        uncallable: None,
        door: Some(door_math_radix_radix_from_code),
    },
    Function {
        name: "math.radix.Radix.new",
        signature: "(base: math.radix.Base, digits: [(i64, i64)], twists: [(i64, i64)]) -> math.radix.Radix",
        docs: "Builds a design from a base, a digit list and a unit twist per digit.\n\nPairwise incongruent digits are the hypothesis of the untwisted fill law: they are what recovers the last digit from the word read modulo the base, so a repeated class is refused here rather than silently gluing words.\n\n# Errors\n\nErrs on a length mismatch, on a twist whose norm is not one, and on two digits congruent modulo the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_new),
    },
    Function {
        name: "math.radix.Radix.plane",
        signature: "(self: math.radix.Radix, level: usize) -> [(f64, f64)]",
        docs: "Returns the level-`L` points in the plane, the scaled words divided by `b^L`.",
        uncallable: None,
        door: Some(door_math_radix_radix_plane),
    },
    Function {
        name: "math.radix.Radix.ring",
        signature: "(self: math.radix.Radix) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_math_radix_radix_ring),
    },
    Function {
        name: "math.radix.Radix.size",
        signature: "(self: math.radix.Radix) -> usize",
        docs: "Returns the digit count `|F|`.",
        uncallable: None,
        door: Some(door_math_radix_radix_size),
    },
    Function {
        name: "math.radix.Radix.twists",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "Returns the twists.",
        uncallable: None,
        door: Some(door_math_radix_radix_twists),
    },
    Function {
        name: "math.radix.Radix.with_twists",
        signature: "(self: math.radix.Radix, units: [usize]) -> math.radix.Radix",
        docs: "Returns the design with the twists named by their index in the unit list, the units in turning order from one.\n\n# Errors\n\nErrs at a twist index past the unit list.",
        uncallable: None,
        door: Some(door_math_radix_radix_with_twists),
    },
    Function {
        name: "math.radix.Radix.words",
        signature: "(self: math.radix.Radix, level: usize) -> [(i64, i64)]",
        docs: "Returns the level-`L` points in exact ring coordinates scaled by `b^L`.\n\nThe place map of digit `d` is `phi_d(x) = (u_d x + d) / b`, a word `d_1 ... d_L` lands on `phi_(d_1)(... phi_(d_L)(0))`, and that point times `b^L` is `sum_(j=1..L) (prod_(i<j) u_(d_i)) d_j b^(L-j)`, which stays in the ring. Words come in digit-lexicographic order, the first digit slowest.",
        uncallable: None,
        door: Some(door_math_radix_radix_words),
    },
    Function {
        name: "math.radix.flowsnake",
        signature: "() -> math.radix.Radix",
        docs: "Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_flowsnake),
    },
    Function {
        name: "math.radix.gasket",
        signature: "() -> math.radix.Radix",
        docs: "Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_gasket),
    },
    Function {
        name: "math.radix.koch",
        signature: "() -> math.radix.Radix",
        docs: "Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.\n\nThe digits are not the canonical residues: `2` and `-1` share a class and the canonical system holds `-1`, so the code alone does not name this design.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_koch),
    },
    Function {
        name: "math.radix.terdragon",
        signature: "() -> math.radix.Radix",
        docs: "Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.\n\nThe untwisted code misses the curve: reading the L-system `F -> F + F - F` at `120` degrees as a turtle, three segments to a level, and normalising by the endpoint gives the segment starts word for word only under this twist.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_terdragon),
    },
    Function {
        name: "math.radix.tile",
        signature: "(m: u64, code: u128) -> math.radix.Radix",
        docs: "Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.\n\nBit `y m + x` of the code is the cell at row `y` and column `x` of the `m` by `m` tile, the column the real part and the row the imaginary part, so the level-`L` words scaled by `m^L` are exactly the filled cells of the level-`L` tile read as `(column, row)`.\n\n# Errors\n\nErrs below a side of two, at a tile of 128 cells or more, and at a code running past the cells.",
        uncallable: None,
        door: Some(door_math_radix_tile),
    },
    Function {
        name: "math.radix.twindragon",
        signature: "() -> math.radix.Radix",
        docs: "Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_twindragon),
    },
    Function {
        name: "math.roulette.Nodes.default",
        signature: "() -> math.roulette.Nodes",
        docs: "Returns the default Nodes.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_default),
    },
    Function {
        name: "math.roulette.Nodes.pair",
        signature: "(self: math.roulette.Nodes, i: usize, j: usize) -> usize",
        docs: "How often the curves `i` and `j` cross, either order, and zero when they are one curve.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_pair),
    },
    Function {
        name: "math.roulette.Nodes.paired",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every crossing of two curves.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_paired),
    },
    Function {
        name: "math.roulette.Nodes.selved",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every self crossing.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_selved),
    },
    Function {
        name: "math.roulette.Nodes.total",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_total),
    },
    Function {
        name: "math.roulette.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize, tol: f64) -> math.roulette.Nodes",
        docs: "Counts the nodes of the roulette the pencils draw on the track: `mrly::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.\n\n# Errors\n\nErrors on an empty pencil list, under three samples, a tolerance outside zero to one, or a trace with no extent.",
        uncallable: None,
        door: Some(door_math_roulette_nodes),
    },
    Function {
        name: "math.roulette.side",
        signature: "(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> i32",
        docs: "Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.",
        uncallable: None,
        door: Some(door_math_roulette_side),
    },
    Function {
        name: "math.roulette.spread",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [math.spirograph.Pencil]",
        docs: "One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrly::math::spirograph::distinct` and `mrly::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.",
        uncallable: None,
        door: Some(door_math_roulette_spread),
    },
    Function {
        name: "math.rules.render",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> Tensor",
        docs: "Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.\n\n# Errors\n\nErrors when the number, the dimension or the base is below one.",
        uncallable: None,
        door: Some(door_math_rules_render),
    },
    Function {
        name: "math.rules.tree_axes",
        signature: "(dimension: usize, free_axis: usize) -> [usize]",
        docs: "Returns every axis but the free one.",
        uncallable: None,
        door: Some(door_math_rules_tree_axes),
    },
    Function {
        name: "math.shape.Frac.minus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact difference.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_minus),
    },
    Function {
        name: "math.shape.Frac.new",
        signature: "(num: i64, den: i64) -> math.shape.Frac",
        docs: "Builds the reduced fraction num over den.\n\n# Errors\n\nErrors on a zero denominator.",
        uncallable: None,
        door: Some(door_math_shape_frac_new),
    },
    Function {
        name: "math.shape.Frac.plus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact sum.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_plus),
    },
    Function {
        name: "math.shape.Frac.times",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact product.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_times),
    },
    Function {
        name: "math.shape.Frac.whole",
        signature: "(num: i64) -> math.shape.Frac",
        docs: "Wraps an integer as a fraction over one.",
        uncallable: None,
        door: Some(door_math_shape_frac_whole),
    },
    Function {
        name: "math.shape.Region.flip",
        signature: "(self: math.shape.Region) -> math.shape.Region",
        docs: "Swaps In and Out, keeping Cut.",
        uncallable: None,
        door: Some(door_math_shape_region_flip),
    },
    Function {
        name: "math.shape.census",
        signature: "(shape: math.shape.Shape, types: Tensor) -> math.shape.ShapeCensus",
        docs: "Tallies the design's cells and filled cells per region of the shape.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_census),
    },
    Function {
        name: "math.shape.classify",
        signature: "(shape: math.shape.Shape, side: usize, index: [usize]) -> math.shape.Region",
        docs: "Places one lattice cell relative to the shape, exactly, with no floats.\n\nThe cell at the index occupies the closed box from the index to the index plus one on each axis, and the shape's unit-box coordinates are scaled by the side.\n\nA polytope is judged wall by wall: Out means some wall excludes the cell, In means every wall contains it, and Cut means neither - so a cell that touches each wall's feasible side separately reads Cut even when the wall intersection misses it, a conservative call that never mislabels In or Out.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_classify),
    },
    Function {
        name: "math.shape.crop",
        signature: "(types: Tensor, shape: math.shape.Shape, keep_cut: bool) -> Tensor",
        docs: "Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_crop),
    },
    Function {
        name: "math.shape.crossing_shell",
        signature: "(radius: u64, number: u64, level: u32) -> [(u64, u64)]",
        docs: "Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.\n\nA box `X` of side `number^level` is crossed when its near corner lies in the closed disc and its far corner outside it, `|X| <= radius / number^level < |X + 1|`, so the level is the whole grid's crossing shell read at that real radius. The columns telescope, the low row of one column being the high row of the next, so the walk is one pass over `floor(radius / number^level) + 1` columns and every comparison is on integers. The order runs along the arc, the first coordinate rising while the second falls, which is why a parent's children are contiguous among its own.",
        uncallable: None,
        door: Some(door_math_shape_crossing_shell),
    },
    Function {
        name: "math.shape.crossing_tree",
        signature: "(radius: u64, number: u64, keep: [bool]) -> math.shape.Shell",
        docs: "Builds the whole crossing tree of one radius, pruned by the seats the design keeps.\n\nThe depth is the least level whose grid holds the circle inside one box, so the tree is rooted there and its leaves are the `2 * radius + 1` crossed cells. `keep` reads the design's level-one tile row-major, one flag per seat, and a box is live when every seat from the root down to it is kept, so the live leaves are exactly the crossed cells the design fills.",
        uncallable: None,
        door: Some(door_math_shape_crossing_tree),
    },
    Function {
        name: "math.shape.fit",
        signature: "(shape: math.shape.Shape, extents: [usize]) -> math.shape.Shape",
        docs: "Centres a unit-box shape on the short side of a lattice of the extents and keeps it to that centred cube, so `regions`, `crop` and `census` read it on any lattice; a cube lattice keeps the shape as it is.\n\nThe cube sits half the spare cells in, rounded down, on each long axis.\n\n# Errors\n\nErrors on no extents, a zero extent, a shape of another dimension, or a scale past an i64.",
        uncallable: None,
        door: Some(door_math_shape_fit),
    },
    Function {
        name: "math.shape.named",
        signature: "(name: str, dimension: usize, radius: math.shape.Frac) -> math.shape.Shape",
        docs: "Builds a named shape of the dimension, centered at one half on every axis.\n\nRegular hexagons and equilateral triangles have irrational walls in the grid frame, so they are excluded on purpose rather than approximated.\n\n# Errors\n\nErrors for an unknown name, or one whose walls are irrational on the grid.",
        uncallable: None,
        door: Some(door_math_shape_named),
    },
    Function {
        name: "math.shape.radial_census",
        signature: "(types: Tensor, centre: [i64], r_max: u64) -> [math.shape.RadialCounts]",
        docs: "Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.\n\nCell centres sit at the index plus one half, so every length is doubled and the centre is given in those doubled units: the lattice corner is zero on each axis and the grid centre is the side. Entry r of the result holds the filled cells whose centre lies within radius r, the filled cells whose whole cell lies within it, and the filled cells the sphere of radius r crosses; squared distances are compared as integers and never as floats.\n\nThe tallies are prefix sums over one pass of the grid: each cell lands in the smallest radius that sees it, the smallest that swallows it and the smallest that clears it, and the columns are summed afterwards.",
        uncallable: None,
        door: Some(door_math_shape_radial_census),
    },
    Function {
        name: "math.shape.reach",
        signature: "(name: str, dimension: usize, den: u32) -> u32?",
        docs: "Returns the least radius, in den-ths up to one, at which the named shape covers the whole unit box, or none when it does not by one.\n\n# Errors\n\nErrors for an unknown name or a zero den.",
        uncallable: None,
        door: Some(door_math_shape_reach),
    },
    Function {
        name: "math.shape.refine",
        signature: "(types: Tensor, shape: math.shape.Shape, base: usize, extra: usize, keep_cut: bool) -> Tensor",
        docs: "Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.\n\n# Errors\n\nErrors below base one, or past the cell ceiling.",
        uncallable: None,
        door: Some(door_math_shape_refine),
    },
    Function {
        name: "math.shape.regions",
        signature: "(shape: math.shape.Shape, dims: [usize]) -> Tensor",
        docs: "Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_regions),
    },
    Function {
        name: "math.shape.shapes",
        signature: "(dimension: usize) -> [String]",
        docs: "Lists the named shapes of a dimension.",
        uncallable: None,
        door: Some(door_math_shape_shapes),
    },
    Function {
        name: "math.six.anti",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "Swaps every fill triangle for a void and back.",
        uncallable: None,
        door: Some(door_math_six_anti),
    },
    Function {
        name: "math.six.binarize",
        signature: "(self: Cell6d, threshold: u8) -> Cell6d",
        docs: "Maps each triangle to one at or above the threshold, zero below.",
        uncallable: None,
        door: Some(door_math_six_binarize),
    },
    Function {
        name: "math.six.binarize_otsu",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "Binarizes the triangles at the threshold Otsu's method picks.",
        uncallable: None,
        door: Some(door_math_six_binarize_otsu),
    },
    Function {
        name: "math.six.blank",
        signature: "(radius: usize, orient: math.six.Orientation, fill: u8, void: u8) -> Cell2d",
        docs: "Builds a hexagon of the given radius, fill inside and void outside.\n\n# Errors\n\nErrors below radius one.",
        uncallable: None,
        door: Some(door_math_six_blank),
    },
    Function {
        name: "math.six.blur",
        signature: "(self: Cell6d, mask: Tensor, wrap: bool) -> Cell6d",
        docs: "Rounds each triangle to the mean of its masked neighborhood, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_six_blur),
    },
    Function {
        name: "math.six.census",
        signature: "(cell: Cell6d, include_grid: bool) -> math.six.Census",
        docs: "Tallies a cell's triangles, corners and edges, counting the backdrop only on request.",
        uncallable: None,
        door: Some(door_math_six_census),
    },
    Function {
        name: "math.six.components",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the connected pieces of the fill, triangles joined across shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_components),
    },
    Function {
        name: "math.six.copies",
        signature: "(rings: usize) -> usize",
        docs: "Returns the hexagons in a disc of the given rings about one centre: 1, 7, 19, 37 and on.\n\n# Errors\n\nErrors when the count passes the machine word.",
        uncallable: None,
        door: Some(door_math_six_copies),
    },
    Function {
        name: "math.six.cut",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Slices a cube through its center across the main diagonal into a hexagon.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_cut),
    },
    Function {
        name: "math.six.cut_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and slices its central hexagon.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_cut_design),
    },
    Function {
        name: "math.six.disc",
        signature: "(cell: Cell6d, rings: usize) -> Cell6d",
        docs: "Tessellates a hexagonal cell over the disc of the given rings about one centre, keeping its projection and orientation and taking the start parity of the uncropped sheet the disc sits in; no rings is the cell itself.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_disc),
    },
    Function {
        name: "math.six.east",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the east-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_east),
    },
    Function {
        name: "math.six.euler",
        signature: "(cell: Cell6d, include_grid: bool) -> i64",
        docs: "Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.",
        uncallable: None,
        door: Some(door_math_six_euler),
    },
    Function {
        name: "math.six.fills",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the filled triangles of the cell.",
        uncallable: None,
        door: Some(door_math_six_fills),
    },
    Function {
        name: "math.six.fills_only",
        signature: "(cell: Cell6d) -> math.six.Census",
        docs: "Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.",
        uncallable: None,
        door: Some(door_math_six_fills_only),
    },
    Function {
        name: "math.six.framed",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.\n\nA renderer reads a sheet's orientation off its frame, so a tall sheet of wide hexagons would draw every triangle on its side; the spare columns or rows are backdrop and reach neither the census nor the picture.\n\n# Errors\n\nErrors when the backdrop does not take the cell's triangles.",
        uncallable: None,
        door: Some(door_math_six_framed),
    },
    Function {
        name: "math.six.from_json",
        signature: "(text: str) -> Cell6d",
        docs: "Parses a cell from JSON, defaulting any missing projection metadata.\n\n# Errors\n\nErrors when the text is not a hex cell.",
        uncallable: None,
        door: Some(door_math_six_from_json),
    },
    Function {
        name: "math.six.giant",
        signature: "(cell: Cell6d) -> usize",
        docs: "Returns the triangle count of the fill's largest connected piece.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_giant),
    },
    Function {
        name: "math.six.giant_network",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Returns the largest connected piece of the filled-triangle network as a network of its own.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_giant_network),
    },
    Function {
        name: "math.six.height",
        signature: "(self: Cell6d) -> usize",
        docs: "Returns the grid height in triangles.",
        uncallable: None,
        door: Some(door_math_six_height),
    },
    Function {
        name: "math.six.holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_holes),
    },
    Function {
        name: "math.six.image",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize) -> Image",
        docs: "Rasters a cell's triangles to an image at the given scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrors when nothing renders or when the scale is zero.",
        uncallable: None,
        door: Some(door_math_six_image),
    },
    Function {
        name: "math.six.is_cube",
        signature: "(cell: Cell3d) -> bool",
        docs: "Returns whether the cell's three sides are equal.",
        uncallable: None,
        door: Some(door_math_six_is_cube),
    },
    Function {
        name: "math.six.is_hex",
        signature: "(cell: Cell2d) -> bool",
        docs: "Returns whether the cell's width, height and parity frame a hexagon.",
        uncallable: None,
        door: Some(door_math_six_is_hex),
    },
    Function {
        name: "math.six.iso",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Projects a cube into the isometric hexagon of top, left and right faces.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_iso),
    },
    Function {
        name: "math.six.iso_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and projects it isometrically.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_iso_design),
    },
    Function {
        name: "math.six.new",
        signature: "(cell: Cell2d, projection: math.six.Projection, orientation: math.six.Orientation, start: u8) -> Cell6d",
        docs: "Builds a cell from its four parts.",
        uncallable: None,
        door: Some(door_math_six_new),
    },
    Function {
        name: "math.six.north",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the north-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_north),
    },
    Function {
        name: "math.six.orientation",
        signature: "(width: usize, height: usize) -> math.six.Orientation",
        docs: "Returns the orientation a hexagon's width and height imply.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_orientation),
    },
    Function {
        name: "math.six.pad",
        signature: "(cell: Cell6d, k: usize, value: u8) -> Cell6d",
        docs: "Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_pad),
    },
    Function {
        name: "math.six.paint",
        signature: "(cell: Cell6d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell6d",
        docs: "Colors each triangle by its type through the custom or default mapping in the given or type mode.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_six_paint),
    },
    Function {
        name: "math.six.perforate",
        signature: "(self: Cell6d, mask: Tensor, value: u8) -> Cell6d",
        docs: "Writes the value wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_six_perforate),
    },
    Function {
        name: "math.six.pro",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Projects a cube's three facing sides into a hexagon of fills and voids.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_pro),
    },
    Function {
        name: "math.six.pro_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and projects its facing sides.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_pro_design),
    },
    Function {
        name: "math.six.radial",
        signature: "(cell: Cell6d, radius: usize) -> Cell2d",
        docs: "Tessellates a hexagonal cell over the disc mask of the given radius.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_radial),
    },
    Function {
        name: "math.six.radial_crop",
        signature: "(cell: Cell2d, radius: usize, size: (usize, usize)) -> Cell2d",
        docs: "Crops the interlocking overhang off a disc tiled at the given radius and tile size.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_radial_crop),
    },
    Function {
        name: "math.six.radial_mask",
        signature: "(radius: usize, orient: math.six.Orientation) -> Tensor",
        docs: "Builds the disc mask of cells within hex distance radius of the center.",
        uncallable: None,
        door: Some(door_math_six_radial_mask),
    },
    Function {
        name: "math.six.raster",
        signature: "(cell: Cell6d, size: usize) -> [f32]",
        docs: "Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.\n\n# Errors\n\nErrors at size zero, for a cell that is not a hexagon, or when nothing is filled.",
        uncallable: None,
        door: Some(door_math_six_raster),
    },
    Function {
        name: "math.six.rect_image",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> Image",
        docs: "Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to an image.\n\n# Errors\n\nErrors for a cell that is not a hexagon or when the scale is zero.",
        uncallable: None,
        door: Some(door_math_six_rect_image),
    },
    Function {
        name: "math.six.rect_svg",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> String",
        docs: "Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.\n\n# Errors\n\nErrors for a cell that is not a hexagon, or when nothing renders.",
        uncallable: None,
        door: Some(door_math_six_rect_svg),
    },
    Function {
        name: "math.six.rim_holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the void regions the rim never reaches, the second route to the hole count.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_rim_holes),
    },
    Function {
        name: "math.six.skin",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.\n\nThe other two projections already speak in fills and voids and come back untouched.",
        uncallable: None,
        door: Some(door_math_six_skin),
    },
    Function {
        name: "math.six.slice_core_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of filled triangles joined by shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_core_graph),
    },
    Function {
        name: "math.six.slice_dual_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of fill and void triangles joined by shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_dual_graph),
    },
    Function {
        name: "math.six.slice_edge_graph",
        signature: "(cell: Cell6d, value: u8?) -> math.graph.Network",
        docs: "Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_edge_graph),
    },
    Function {
        name: "math.six.slice_tunnel_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of void triangles joined by shared edges, the pore network of the slice.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_tunnel_graph),
    },
    Function {
        name: "math.six.south",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the south-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_south),
    },
    Function {
        name: "math.six.spectral_exponent",
        signature: "(cell: Cell6d, window: f64) -> f64",
        docs: "Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.\n\n# Errors\n\nErrors when the giant piece is too small to fit an exponent.",
        uncallable: None,
        door: Some(door_math_six_spectral_exponent),
    },
    Function {
        name: "math.six.star.Branch.constant",
        signature: "(self: math.six.star.Branch) -> f64",
        docs: "The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.",
        uncallable: None,
        door: Some(door_math_six_star_branch_constant),
    },
    Function {
        name: "math.six.star.Branch.name",
        signature: "(self: math.six.star.Branch) -> String",
        docs: "The name of the branch.",
        uncallable: None,
        door: Some(door_math_six_star_branch_name),
    },
    Function {
        name: "math.six.star.Branch.of",
        signature: "(layers: usize) -> math.six.star.Branch",
        docs: "The branch of a layer count.",
        uncallable: None,
        door: Some(door_math_six_star_branch_of),
    },
    Function {
        name: "math.six.star.Branch.residual",
        signature: "(self: math.six.star.Branch) -> f64?",
        docs: "The exact `1/L^2` coefficient at even `L`, absent at odd `L`.",
        uncallable: None,
        door: Some(door_math_six_star_branch_residual),
    },
    Function {
        name: "math.six.star.Share.reduced",
        signature: "(self: math.six.star.Share) -> (i64, i64)",
        docs: "The share in lowest terms, numerator then denominator.",
        uncallable: None,
        door: Some(door_math_six_star_share_reduced),
    },
    Function {
        name: "math.six.star.Share.value",
        signature: "(self: math.six.star.Share) -> f64",
        docs: "The share as a real number.",
        uncallable: None,
        door: Some(door_math_six_star_share_value),
    },
    Function {
        name: "math.six.star.Star.arm",
        signature: "(self: math.six.star.Star, number: usize, half: usize) -> math.six.star.Share",
        docs: "The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.\n\nAt `W = 0` the band is the arm itself, the `2n` cells of the diameter, whose share is the\nclosed form of `arm_law`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_arm),
    },
    Function {
        name: "math.six.star.Star.cell",
        signature: "(self: math.six.star.Star, number: usize, x: i64, z: i64) -> bool?",
        docs: "The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.\n\nThe answer is absent where the plane `x + y + z = 6n - 2` leaves the cube, which is what draws\nthe hexagon's own edge.",
        uncallable: None,
        door: Some(door_math_six_star_star_cell),
    },
    Function {
        name: "math.six.star.Star.excesses",
        signature: "(self: math.six.star.Star, layers: usize, half: usize) -> [f64]",
        docs: "The per-layer excess of the star band over the hexagon across the first `L` odd layers.\n\n# Errors\n\nErrors below one layer, or at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_excesses),
    },
    Function {
        name: "math.six.star.Star.hexagon",
        signature: "(self: math.six.star.Star, number: usize) -> math.six.star.Share",
        docs: "The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.\n\nThe rows are the `2n` even heights `z`, and each runs over the `x` the plane leaves inside the\ncube. The fill depends on `x` only through `x mod 8`, so a row is counted in eight steps and\nthe whole layer in `O(n)`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_hexagon),
    },
    Function {
        name: "math.six.star.Star.new",
        signature: "(code: u128) -> math.six.star.Star",
        docs: "Reads the star of a base-2 space code, the carpet being `23`.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_six_star_star_new),
    },
    Function {
        name: "math.six.star.arm_law",
        signature: "(number: usize) -> math.six.star.Share",
        docs: "The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.\n\nThe arm `x = y` of the cut is a diameter of `2n` cells with no width to choose, and on it the two\nblock parities agree, so the space rule collapses to `floor(x/4)` even and the inked count is\n`f(3n) - f(n) = n + chi_8(n)` with `f(m) = 4 floor(m/8) + min(m mod 8, 4)`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_arm_law),
    },
    Function {
        name: "math.six.star.chi8",
        signature: "(number: usize) -> i64",
        docs: "The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.",
        uncallable: None,
        door: Some(door_math_six_star_chi8),
    },
    Function {
        name: "math.six.star.constant",
        signature: "() -> f64",
        docs: "The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.\n\nEvery limb is a closed form the subtraction of the two ink laws hands over: the character part is\n`L(1, chi_8)/2` by the class number formula for `Q(sqrt 2)`, the square tail is `-G/8` with `G`\nCatalan's constant, and the harmonic tail carries `-gamma/4 - (ln 2)/2`.",
        uncallable: None,
        door: Some(door_math_six_star_constant),
    },
    Function {
        name: "math.six.star.decay",
        signature: "(excesses: [f64], layers: usize) -> math.six.star.Decay",
        docs: "The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.\n\nAt `W = 0` and even `L` the reading is `L * excess_L = -(ln L)/4 + C + O(1/L^2)`, so the slope\nsettles on `-1/4` and the residual on the branch's exact `1/L^2` coefficient. The sliding window\ncarries a residue trap: the per-layer excess opens on a term in the ink law's character whose sum\nvanishes at even `L` alone, so a window from `L/2` to `L` cancels it only when `L` is divisible by\nfour. At `L = 2 mod 4` the half is odd and the window converges somewhere else entirely, so the\nslope is refused there rather than reported wrong.\n\n# Errors\n\nErrors below two layers, or past the excesses read.",
        uncallable: None,
        door: Some(door_math_six_star_decay),
    },
    Function {
        name: "math.six.star.width_law",
        signature: "(half: usize) -> f64",
        docs: "The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.\n\n`b` is the block parity of the band edge, one when `floor(K/2)` is even and zero when it is odd.\nThe band's per-layer excess is `kappa chi + (m + q chi_8(n))/n + chi/(8 n^2)`, and summing the\n`1/n` term over the first `L` odd layers hands this coefficient as `m/2`, so it is a theorem and\nnot a fit. At even `W` it reads `-(W + 2b)/(8(W + 1))` and at odd `W` it reads\n`-(W - 1 + 2b)/(8W)`, since `x - y` is even on the cut and an odd width is never a new band:\n`W = 1` returns the arm's `-1/4` and `W = 3` returns `-1/6`. As `W` grows it tends to `-1/8`.",
        uncallable: None,
        door: Some(door_math_six_star_width_law),
    },
    Function {
        name: "math.six.svg",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize, start: usize?) -> String",
        docs: "Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrors when nothing renders.",
        uncallable: None,
        door: Some(door_math_six_svg),
    },
    Function {
        name: "math.six.tessellate",
        signature: "(cell: Cell6d, mask: Tensor) -> Cell2d",
        docs: "Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tessellate),
    },
    Function {
        name: "math.six.tile",
        signature: "(cell: Cell6d, width: usize, height: usize) -> Cell2d",
        docs: "Tessellates a hexagonal cell over a full width-by-height mask.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tile),
    },
    Function {
        name: "math.six.tile_cell",
        signature: "(cell: Cell6d, width: usize, height: usize, crop: bool) -> Cell6d",
        docs: "Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.\n\nThe sheet keeps the tile's projection and orientation. A crop slides the triangle grid by the interlocking step on both axes, so the start parity flips whenever that step is odd; without the flip every triangle in the sheet points the wrong way.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tile_cell),
    },
    Function {
        name: "math.six.tile_crop",
        signature: "(cell: Cell2d, size: (usize, usize)) -> Cell2d",
        docs: "Crops one interlocking step off each side of a sheet tiled at the given size.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_tile_crop),
    },
    Function {
        name: "math.six.tile_step",
        signature: "(size: (usize, usize)) -> (usize, usize)",
        docs: "Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_tile_step),
    },
    Function {
        name: "math.six.to_json",
        signature: "(cell: Cell6d) -> String",
        docs: "Serializes a cell and its projection metadata to JSON.",
        uncallable: None,
        door: Some(door_math_six_to_json),
    },
    Function {
        name: "math.six.triangles",
        signature: "(cell: Cell6d, start: usize?) -> [([(i64, i64); 3], [u8; 4])]",
        docs: "Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_triangles),
    },
    Function {
        name: "math.six.west",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the west-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_west),
    },
    Function {
        name: "math.six.width",
        signature: "(self: Cell6d) -> usize",
        docs: "Returns the grid width in triangles.",
        uncallable: None,
        door: Some(door_math_six_width),
    },
    Function {
        name: "math.spectrum.clusters",
        signature: "(eigenvalues: [f64], tolerance: f64) -> [(f64, usize)]",
        docs: "Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.\n\n# Errors\n\nErrors on a NaN.",
        uncallable: None,
        door: Some(door_math_spectrum_clusters),
    },
    Function {
        name: "math.spectrum.laplacian",
        signature: "(network: math.graph.Network, normalised: bool) -> [[f64]]",
        docs: "Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.\n\nSelf-loops are ignored and the degrees are the adjacency row sums. Errors when a\nnode carries no branch and the normalised form is asked for.\n\n# Errors\n\nErrors when a node carries no branch under the normalised reading.",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian),
    },
    Function {
        name: "math.spectrum.laplacian_spectrum",
        signature: "(network: math.graph.Network, normalised: bool) -> [f64]",
        docs: "Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.\n\n# Errors\n\nErrors when a node carries no branch, or the spectrum is not a number.",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian_spectrum),
    },
    Function {
        name: "math.spectrum.multiplicity",
        signature: "(eigenvalues: [f64], value: f64, tolerance: f64) -> usize",
        docs: "Counts the eigenvalues within the tolerance of a value.",
        uncallable: None,
        door: Some(door_math_spectrum_multiplicity),
    },
    Function {
        name: "math.spectrum.spectral_exponent",
        signature: "(eigenvalues: [f64], window: f64) -> f64?",
        docs: "Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_exponent),
    },
    Function {
        name: "math.spectrum.spectral_fit",
        signature: "(eigenvalues: [f64], window: f64) -> (f64, f64, usize)?",
        docs: "Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.\n\nThe points come from `spectral_points` and the first `max(floor(window * total), 3)` of\nthem are fitted by least squares. Returns `None` when fewer than two points remain.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_fit),
    },
    Function {
        name: "math.spectrum.spectral_points",
        signature: "(eigenvalues: [f64]) -> [(f64, f64)]",
        docs: "Builds the integrated density of states as points, each an eigenvalue and its rank fraction.\n\nThe eigenvalues are clamped at zero and sorted, exactly equal neighbours collapse to one\npoint at the run's last index over the total count, and points at or below `1e-12` drop.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_points),
    },
    Function {
        name: "math.spectrum.symmetric_eigenvalues",
        signature: "(matrix: [[f64]]) -> [f64]",
        docs: "Returns the eigenvalues of a dense real symmetric matrix in ascending order.\n\nThe matrix is reduced to tridiagonal form by Householder reflections and then\ndiagonalised by implicit QL with Wilkinson shifts. Errors on a matrix that is\nnot square or whose entries disagree across the diagonal by more than `1e-12`.\n\n# Errors\n\nErrors on a matrix that is not square or not symmetric, or a spectrum that is not a number.",
        uncallable: None,
        door: Some(door_math_spectrum_symmetric_eigenvalues),
    },
    Function {
        name: "math.spin.Blend.fold",
        signature: "(self: math.spin.Blend, values: [f32]) -> f32",
        docs: "Merges one site's copies into the blended value.",
        uncallable: None,
        door: Some(door_math_spin_blend_fold),
    },
    Function {
        name: "math.spin.Blend.named",
        signature: "(name: str) -> math.spin.Blend?",
        docs: "Reads a blend by name: mean, sum, union, meet, parity or difference.",
        uncallable: None,
        door: Some(door_math_spin_blend_named),
    },
    Function {
        name: "math.spin.arcs",
        signature: "(data: [f32], size: usize, radius: f64) -> [(f64, f64, f32)]",
        docs: "The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_arcs),
    },
    Function {
        name: "math.spin.harmonics",
        signature: "(data: [f32], size: usize, rings: usize, orders: usize) -> [f64]",
        docs: "The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_harmonics),
    },
    Function {
        name: "math.spin.mass",
        signature: "(profile: [f32], size: usize) -> f64",
        docs: "The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.",
        uncallable: None,
        door: Some(door_math_spin_mass),
    },
    Function {
        name: "math.spin.mass_within",
        signature: "(profile: [f32], size: usize, radius: f64) -> f64",
        docs: "The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.",
        uncallable: None,
        door: Some(door_math_spin_mass_within),
    },
    Function {
        name: "math.spin.petals",
        signature: "(copies: usize, order: usize) -> usize",
        docs: "The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.",
        uncallable: None,
        door: Some(door_math_spin_petals),
    },
    Function {
        name: "math.spin.profile",
        signature: "(data: [f32], size: usize, steps: usize) -> [f32]",
        docs: "The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_profile),
    },
    Function {
        name: "math.spin.radial",
        signature: "(data: [f32], size: usize, out: usize, copies: usize, step: f64, blend: math.spin.Blend, samples: usize) -> [f32]",
        docs: "Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_radial),
    },
    Function {
        name: "math.spin.reach",
        signature: "(size: usize) -> f64",
        docs: "The radius of the corner circle of a square raster of the side, the last radius a profile reads.",
        uncallable: None,
        door: Some(door_math_spin_reach),
    },
    Function {
        name: "math.spin.ring",
        signature: "(data: [f32], size: usize, radius: f64) -> f64",
        docs: "The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_ring),
    },
    Function {
        name: "math.spin.turns",
        signature: "(power: [f64]) -> usize",
        docs: "The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.",
        uncallable: None,
        door: Some(door_math_spin_turns),
    },
    Function {
        name: "math.spin.wheel",
        signature: "(profile: [f32], size: usize) -> [f32]",
        docs: "The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.",
        uncallable: None,
        door: Some(door_math_spin_wheel),
    },
    Function {
        name: "math.spirograph.Seats.default",
        signature: "() -> math.spirograph.Seats",
        docs: "Returns the default Seats.",
        uncallable: None,
        door: Some(door_math_spirograph_seats_default),
    },
    Function {
        name: "math.spirograph.cell",
        signature: "(width: usize, height: usize, reach: f64) -> f64",
        docs: "The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.",
        uncallable: None,
        door: Some(door_math_spirograph_cell),
    },
    Function {
        name: "math.spirograph.cover",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool, samples: usize, side: usize) -> math.spirograph.Cover",
        docs: "The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.\n\n# Errors\n\nErrors on a line or a polygon track, a raster side out of range, under two samples, or past the point cap.",
        uncallable: None,
        door: Some(door_math_spirograph_cover),
    },
    Function {
        name: "math.spirograph.disc",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> math.spirograph.Disc",
        docs: "The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.\n\n# Errors\n\nErrors on a line or a polygon track.",
        uncallable: None,
        door: Some(door_math_spirograph_disc),
    },
    Function {
        name: "math.spirograph.distinct",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> usize",
        docs: "How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.",
        uncallable: None,
        door: Some(door_math_spirograph_distinct),
    },
    Function {
        name: "math.spirograph.frame",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> [f64; 4]",
        docs: "The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.",
        uncallable: None,
        door: Some(door_math_spirograph_frame),
    },
    Function {
        name: "math.spirograph.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> u64?",
        docs: "The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.",
        uncallable: None,
        door: Some(door_math_spirograph_nodes),
    },
    Function {
        name: "math.spirograph.pencils",
        signature: "(types: [u8], width: usize, height: usize, mode: str, reach: f64, jitter: f64, seed: u32) -> [math.spirograph.Pencil]",
        docs: "Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.\n\n# Errors\n\nErrors on a grid that is not width by height, a reach or jitter out of range, or a mode that is not fill, void, both or corners.",
        uncallable: None,
        door: Some(door_math_spirograph_pencils),
    },
    Function {
        name: "math.spirograph.point",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil, s: f64) -> (f64, f64)",
        docs: "Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.",
        uncallable: None,
        door: Some(door_math_spirograph_point),
    },
    Function {
        name: "math.spirograph.pose",
        signature: "(track: math.spirograph.Track, s: f64) -> (f64, f64)",
        docs: "The wheel's centre after `s` of path length.",
        uncallable: None,
        door: Some(door_math_spirograph_pose),
    },
    Function {
        name: "math.spirograph.representatives",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [usize]",
        docs: "One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.",
        uncallable: None,
        door: Some(door_math_spirograph_representatives),
    },
    Function {
        name: "math.spirograph.seats",
        signature: "(pencils: [math.spirograph.Pencil]) -> math.spirograph.Seats",
        docs: "Counts the pencils by kind.",
        uncallable: None,
        door: Some(door_math_spirograph_seats),
    },
    Function {
        name: "math.spirograph.signed_area",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil) -> f64?",
        docs: "The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.",
        uncallable: None,
        door: Some(door_math_spirograph_signed_area),
    },
    Function {
        name: "math.spirograph.trace",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize) -> [f32]",
        docs: "Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.\n\n# Errors\n\nErrors under two samples, or past the point cap.",
        uncallable: None,
        door: Some(door_math_spirograph_trace),
    },
    Function {
        name: "math.spirograph.track",
        signature: "(kind: str, ring: usize, wheel: usize, sides: usize, laps: usize) -> math.spirograph.Track",
        docs: "Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.\n\n# Errors\n\nErrors on a radius or lap count out of range, a wheel that does not fit its ring, or a kind that is not line, in, out, polyin or polyout.",
        uncallable: None,
        door: Some(door_math_spirograph_track),
    },
    Function {
        name: "math.spirograph.turn",
        signature: "(track: math.spirograph.Track, s: f64) -> f64",
        docs: "The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.",
        uncallable: None,
        door: Some(door_math_spirograph_turn),
    },
    Function {
        name: "math.sumset.Pair.clean",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "Whether the pair is clean, `3^k > d(k, m)` and `4^m > d(k, m)`, so that `S meet [0, d] = A_k + B_m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_clean),
    },
    Function {
        name: "math.sumset.Pair.copy",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "Whether the pair is a gap copy, `2 4^m < 3^k + 5`: `A_k + B_m` is then two disjoint translates of `A_(k-1) + B_m` and its energy is twice theirs.",
        uncallable: None,
        door: Some(door_math_sumset_pair_copy),
    },
    Function {
        name: "math.sumset.Pair.energy",
        signature: "(self: math.sumset.Pair) -> u128",
        docs: "The additive energy `E(k, m) = sum_x r(x)^2`, `r(x)` the number of ways `x = a + b` with `a` in `A_k` and `b` in `B_m`.\n\nSummed as `sum_t 2^(z_3(t) + z_4(t))` over the `3^m` integers `t` with an `m`-digit base-4 string in `{-1, 0, 1}`, `z_4(t)` its zero digits and `z_3(t)` the zero digits of the `k`-digit balanced ternary expansion of `t`, a `t` past `(3^k - 1)/2` adding nothing: a difference of two digit strings from `{0, 1}` is a digit string from `{-1, 0, 1}`, which fixes `t` in either base, and each zero difference arises twice. The cost is `3^m` strings.",
        uncallable: None,
        door: Some(door_math_sumset_pair_energy),
    },
    Function {
        name: "math.sumset.Pair.four",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "The base-4 level `m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_four),
    },
    Function {
        name: "math.sumset.Pair.gap",
        signature: "(self: math.sumset.Pair) -> (u64, u64)?",
        docs: "The first and the last integer of the open interval `(d(k, m), min(3^k, 4^m))`, which `S` misses, or `None` when it holds none.\n\nA sum with `a < 3^k` and `b < 4^m` is at most `d(k, m)`, and a sum with `a >= 3^k` or `b >= 4^m` is at least `min(3^k, 4^m)`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_gap),
    },
    Function {
        name: "math.sumset.Pair.largest",
        signature: "(self: math.sumset.Pair) -> u64",
        docs: "The largest element `d(k, m) = (3^k - 1)/2 + (4^m - 1)/3` of `A_k + B_m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_largest),
    },
    Function {
        name: "math.sumset.Pair.new",
        signature: "(three: u32, four: u32) -> math.sumset.Pair",
        docs: "Names the pair `(k, m)`.\n\n# Errors\n\nErrs when either level is zero or past `WIDEST`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_new),
    },
    Function {
        name: "math.sumset.Pair.ratio",
        signature: "(self: math.sumset.Pair, energy: u128) -> f64",
        docs: "The energy ratio `Q(k, m) = E(k, m) (d + 1)/4^(k+m)` of the energy `Pair::energy` returns, the energy against its flat value, at least `1`; `card(A_k + B_m) >= (d + 1)/Q` by Cauchy-Schwarz.",
        uncallable: None,
        door: Some(door_math_sumset_pair_ratio),
    },
    Function {
        name: "math.sumset.Pair.scale",
        signature: "(self: math.sumset.Pair) -> f64",
        docs: "The scaling `tau = 4^m/3^k`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_scale),
    },
    Function {
        name: "math.sumset.Pair.three",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "The base-3 level `k`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_three),
    },
    Function {
        name: "math.sumset.Sumset.contains",
        signature: "(self: math.sumset.Sumset, x: u64) -> bool?",
        docs: "Whether `x` is in `S`, or `None` past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.count",
        signature: "(self: math.sumset.Sumset, x: u64) -> u64?",
        docs: "Counts `card(S meet [1, x])`, or `None` past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.density",
        signature: "(self: math.sumset.Sumset, x: u64) -> f64?",
        docs: "Reads the density `D(x) = card(S meet [1, x])/x`, or `None` at zero and past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.extremes",
        signature: "(self: math.sumset.Sumset, edges: [u64]) -> [(f64, f64)?]",
        docs: "Reads the least and the greatest `D(x)` over each window `[edges[i], edges[i + 1])`, `None` for an empty window.\n\n# Errors\n\nErrs when the edges decrease, start at zero or run past `top + 1`.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.fills",
        signature: "(self: math.sumset.Sumset, low: u64, high: u64, cells: usize) -> [f64]",
        docs: "Reads the share of members in each of `cells` equal runs of the integers `[low, high)`, the strip of `S` a page draws.\n\nRun `i` is `[low + floor(i n/cells), low + floor((i + 1) n/cells))` with `n = high - low`.\n\n# Errors\n\nErrs when the window is empty or runs past the top, or when `cells` is zero or more than the integers in the window.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.level",
        signature: "(self: math.sumset.Sumset) -> u32",
        docs: "The level the array was built to.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.new",
        signature: "(level: u32) -> math.sumset.Sumset",
        docs: "Builds `S meet [0, 3^level]`.\n\n# Errors\n\nErrs when the level is zero or past `DEEPEST`.",
        uncallable: Some("math::sumset::Sumset has no Serialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.top",
        signature: "(self: math.sumset.Sumset) -> u64",
        docs: "The largest integer the array holds, `3^level`.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.pairs",
        signature: "(level: u32) -> [math.sumset.Pair]",
        docs: "Lists the pairs of the census: every `(k, m)` with `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, by `d`.\n\n# Errors\n\nErrs when the level is zero or past `WIDEST`.",
        uncallable: None,
        door: Some(door_math_sumset_pairs),
    },
    Function {
        name: "math.three.Vec3.cross",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> math.three.Vec3",
        docs: "Returns the cross product, perpendicular to both vectors.",
        uncallable: None,
        door: Some(door_math_three_vec3_cross),
    },
    Function {
        name: "math.three.Vec3.dot",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> f32",
        docs: "Returns the dot product of the two vectors.",
        uncallable: None,
        door: Some(door_math_three_vec3_dot),
    },
    Function {
        name: "math.three.Vec3.new",
        signature: "(x: f32, y: f32, z: f32) -> math.three.Vec3",
        docs: "Builds a vector from its components.",
        uncallable: None,
        door: Some(door_math_three_vec3_new),
    },
    Function {
        name: "math.three.Vec3.scale",
        signature: "(self: math.three.Vec3, s: f32) -> math.three.Vec3",
        docs: "Multiplies every component by the scalar.",
        uncallable: None,
        door: Some(door_math_three_vec3_scale),
    },
    Function {
        name: "math.three.carpet",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_carpet),
    },
    Function {
        name: "math.three.census",
        signature: "(cell: Cell3d) -> math.three.Census",
        docs: "Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cube.",
        uncallable: None,
        door: Some(door_math_three_census),
    },
    Function {
        name: "math.three.core_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of filled sites joined to their axis neighbors.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_core_graph),
    },
    Function {
        name: "math.three.create",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell3d",
        docs: "Builds the cube the universe code names, deepened to the given fractal level.\n\n# Errors\n\nErrors when the code is out of range.",
        uncallable: None,
        door: Some(door_math_three_create),
    },
    Function {
        name: "math.three.diagonal_slice",
        signature: "(code: Code, number: usize, level: usize, base: usize, height: usize) -> [[u32; 3]]",
        docs: "Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_three_diagonal_slice),
    },
    Function {
        name: "math.three.diagonal_svg",
        signature: "(code: Code, number: usize, level: usize, base: usize, heights: [usize], scale: usize) -> String",
        docs: "Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.\n\nThe frame is tight around the projected points and carries no background, so the drawing sits on\nwhatever the page is.\n\n# Errors\n\nErrors when the code is out of range, the level is below one, or nothing renders.",
        uncallable: None,
        door: Some(door_math_three_diagonal_svg),
    },
    Function {
        name: "math.three.dust",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the dust cube, filled where every coordinate is even, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_dust),
    },
    Function {
        name: "math.three.edge_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of corners and edges outlining every filled site.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_edge_graph),
    },
    Function {
        name: "math.three.euler",
        signature: "(cell: Cell3d) -> i64",
        docs: "Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.\n\nOne for a solid block, and one less than twice the genus below zero for a tunnelled one.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cube.",
        uncallable: None,
        door: Some(door_math_three_euler),
    },
    Function {
        name: "math.three.extrude",
        signature: "(cell: Cell2d, axis: usize, depth: usize) -> Cell3d",
        docs: "Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.\n\nThis is the inverse of slice: every slice of the lift on that axis is the flat cell again.\n\n# Errors\n\nErrors when the axis is not 0, 1 or 2, or the depth is below one.",
        uncallable: None,
        door: Some(door_math_three_extrude),
    },
    Function {
        name: "math.three.extrude_cube",
        signature: "(cell: Cell3d, depth: usize) -> Cell3d",
        docs: "Repeats every plane of a cube depth times along its leading axis, colors and tags with it.\n\n# Errors\n\nErrors when the depth is below one.",
        uncallable: None,
        door: Some(door_math_three_extrude_cube),
    },
    Function {
        name: "math.three.faces",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.\n\nSurface counts only the faces open to void; this counts every face of the complex.",
        uncallable: None,
        door: Some(door_math_three_faces),
    },
    Function {
        name: "math.three.fills",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of filled sites.",
        uncallable: None,
        door: Some(door_math_three_fills),
    },
    Function {
        name: "math.three.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, base: usize) -> Cell3d",
        docs: "Builds a cube from its corner patterns, deepened to the given fractal level.\n\n# Errors\n\nErrors when a corner does not fit the dimension and base.",
        uncallable: None,
        door: Some(door_math_three_from_corners),
    },
    Function {
        name: "math.three.from_json",
        signature: "(text: str) -> Cell3d",
        docs: "Parses a cell from its JSON, colors and tags included.\n\n# Errors\n\nErrors when the text is not a cube.",
        uncallable: None,
        door: Some(door_math_three_from_json),
    },
    Function {
        name: "math.three.from_strings",
        signature: "(data: [[String]]) -> Cell3d",
        docs: "Builds a cube from one string of digits per row, grouped plane by plane.\n\n# Errors\n\nErrors when the planes are empty, ragged, or hold anything but digits.",
        uncallable: None,
        door: Some(door_math_three_from_strings),
    },
    Function {
        name: "math.three.hidden",
        signature: "(cell: Cell3d) -> u128",
        docs: "Returns the count of faces buried between two filled sites, six per site less the exposed surface.",
        uncallable: None,
        door: Some(door_math_three_hidden),
    },
    Function {
        name: "math.three.level_set",
        signature: "(number: usize, levels: [usize], level: usize, base: usize) -> Cell3d",
        docs: "Builds the cube filled wherever the residue sum lands in the levels, at the given level.\n\nCarpet, net and void are the three presets of this one engine: the levels are all a\nsymmetric cube design ever names.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_level_set),
    },
    Function {
        name: "math.three.magic",
        signature: "(cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "Folds two or more cells into one by chained Kronecker combination.\n\n# Errors\n\nErrors when the cells differ in shape.",
        uncallable: None,
        door: Some(door_math_three_magic),
    },
    Function {
        name: "math.three.manhattan_layers",
        signature: "(cell: Cell3d) -> Cell3d",
        docs: "Tags every site with its Manhattan distance from the cube's center, the diamond shells.\n\nThe cell's own layers count Chebyshev shells, which are boxes; these are octahedra.",
        uncallable: None,
        door: Some(door_math_three_manhattan_layers),
    },
    Function {
        name: "math.three.merge",
        signature: "(cells: [Cell3d], width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "Merges the cells into one cube arranged width by height by depth.\n\n# Errors\n\nErrors when the cells differ in shape or do not fill the arrangement.",
        uncallable: None,
        door: Some(door_math_three_merge),
    },
    Function {
        name: "math.three.mosaic",
        signature: "(mask: Tensor, cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "Builds a cell by placing at each mask site the cell its value indexes.\n\n# Errors\n\nErrors when the mask and the cells do not fit one another.",
        uncallable: None,
        door: Some(door_math_three_mosaic),
    },
    Function {
        name: "math.three.named",
        signature: "(design: math.bang.catalog.Design, number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube the name picks, deepened to the given fractal level.\n\n# Errors\n\nErrors for a design that is not 3d, or a level below one.",
        uncallable: None,
        door: Some(door_math_three_named),
    },
    Function {
        name: "math.three.net",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the net cube, filled where at least two coordinates are odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_net),
    },
    Function {
        name: "math.three.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell3d",
        docs: "Builds a cube whose every site turns on with probability density, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_noise),
    },
    Function {
        name: "math.three.ones",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the solid cube at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ones),
    },
    Function {
        name: "math.three.orientations",
        signature: "() -> [(usize, usize, usize)]",
        docs: "Returns the 24 rotation triples that reach each distinct cube orientation.",
        uncallable: None,
        door: Some(door_math_three_orientations),
    },
    Function {
        name: "math.three.point",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the point cube, filled where every coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_point),
    },
    Function {
        name: "math.three.profile",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> [u128]",
        docs: "Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.\n\nThe count is the coefficient of the digit polynomial, the product over the scales of the\npattern's weight sums, so no cell of the solid is ever built.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_three_profile),
    },
    Function {
        name: "math.three.project",
        signature: "(point: [u32; 3]) -> (f64, f64)",
        docs: "Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.",
        uncallable: None,
        door: Some(door_math_three_project),
    },
    Function {
        name: "math.three.quads",
        signature: "(cell: Cell3d) -> [math.three.Quad]",
        docs: "Returns one outward quad per exposed face, scaled into the unit box.",
        uncallable: None,
        door: Some(door_math_three_quads),
    },
    Function {
        name: "math.three.shadow",
        signature: "(point: [u32; 3]) -> (i64, i64)",
        docs: "Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.",
        uncallable: None,
        door: Some(door_math_three_shadow),
    },
    Function {
        name: "math.three.slice",
        signature: "(cell: Cell3d, axis: usize, index: usize) -> Cell2d",
        docs: "Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.\n\n# Errors\n\nErrors when the axis or the index is past the cube.",
        uncallable: None,
        door: Some(door_math_three_slice),
    },
    Function {
        name: "math.three.special",
        signature: "(mask: Tensor, cell: Cell3d) -> Cell3d",
        docs: "Orients a copy of the cell by each mask value and merges them in the mask's shape.\n\n# Errors\n\nErrors when the mask is not 3d, or holds a value past twenty-three.",
        uncallable: None,
        door: Some(door_math_three_special),
    },
    Function {
        name: "math.three.sponge.deep",
        signature: "(radius: f64) -> f64",
        docs: "The volume `Deep(radius)` of the points of one arm's quarter beyond the tubes of both walls it touches, at every radius, to double precision.\n\nWith `line = min(radius, 1/6)`, the quarter is `[0, line]^2` across the arm times its length,\nand every point of `Deep` sits, through both of its wall coordinates, in one and the same hole\nof the wall carpet that the line `across = line` cuts: a crossing hole. The volume is the sum\nover crossing holes of one arcsine integral, `2^(m-1)` holes of side `3^(-m-1)` at every level\n`m` for `line = 1/6`, none at all for `1/12`, whose triple `1/4 = 0.0202...` has no digit `1`.\nIt vanishes from `sqrt(10)/18` on, where the arms are swallowed. Each integral is elementary, but its\nexpanded arcsine form cancels at deep levels, so it is summed by 16-point Gauss-Legendre on each smooth\npiece in the angle `asin(R/radius)`, `R` the depth that a wall point needs.",
        uncallable: None,
        door: Some(door_math_three_sponge_deep),
    },
    Function {
        name: "math.three.sponge.dimension",
        signature: "() -> f64",
        docs: "The Minkowski dimension of the Menger sponge, `log(20)/log(3)`, the similarity dimension of its 20 maps of ratio `1/3`.",
        uncallable: None,
        door: Some(door_math_three_sponge_dimension),
    },
    Function {
        name: "math.three.sponge.distance",
        signature: "(point: [f64; 3]) -> f64",
        docs: "The Euclidean distance from a point of the unit cube to the Menger sponge, exact to the last binary place.\n\nThe point descends the base-3 digits while its digit triple holds at most one `1`, each step\ndividing the distance by 3, since inside a retained subcube the distance to the sponge is the\ndistance to that subcube's copy of it. Once it lands in the plus of seven removed cubes, the\ndistance is the one to the twelve edges of the centre cube or to the four carpets walling the\narm it sits in. Coordinates outside `[0, 1]` are clamped; the window centre `(1/2, 1/2, 0)`\nreads `1/6` and the centre of the cube `sqrt(2)/6`.",
        uncallable: None,
        door: Some(door_math_three_sponge_distance),
    },
    Function {
        name: "math.three.sponge.exact",
        signature: "(radius: f64) -> f64",
        docs: "The volume `T(radius)` of the points of the plus within `radius` of the sponge at every radius, `Deep` included, to double precision.\n\nOn `(0, 1/6]` it is the closed form of `tube` less `24 Deep`. On `[1/6, sqrt(2)/6]` the arms\nhold `2/9 - 24 Deep` and the centre cube `pi r^2 - 8 sqrt(2) r^3 - 4 r^2 acos(1/(6r)) +\n(2/3 + 16 c^2) c` with `c = sqrt(r^2 - 1/36)`, the edge cylinders overlapping along the faces;\nfrom `sqrt(2)/6` on the plus is swallowed, `7/27`.",
        uncallable: None,
        door: Some(door_math_three_sponge_exact),
    },
    Function {
        name: "math.three.sponge.profile",
        signature: "(radius: f64) -> f64?",
        docs: "The periodic function `p` of Kombrink, Pearse and Winter at `radius`: the reading's limit profile, unchanged when the radius is multiplied by 3, or `None` on the phases `(1/6, sqrt(2)/6]` where `T` has no closed form.\n\nThe radius is first carried by powers of 3 into `(sqrt(2)/18, sqrt(2)/6]`; on\n`(sqrt(2)/18, 1/6]` it is `r^(D-3) (20/27 + sum_(l = 0..40) (27/20)^l T(r/3^l))`, and the\nsponge is Minkowski measurable exactly when this is constant. It is not: the certificate puts\n`p(1/12)` in `[2.122718, 2.122723]` and `p(1/6)` in `[2.135019, 2.136794]`.",
        uncallable: None,
        door: Some(door_math_three_sponge_profile),
    },
    Function {
        name: "math.three.sponge.reading",
        signature: "(radius: f64) -> f64?",
        docs: "The Minkowski reading `radius^(D-3)` times the volume inside the cube, the number whose limit as the radius shrinks would be the sponge's Minkowski content.",
        uncallable: None,
        door: Some(door_math_three_sponge_reading),
    },
    Function {
        name: "math.three.sponge.tube",
        signature: "(radius: f64) -> f64?",
        docs: "The volume `T(radius)` of the points of the plus of seven removed level-1 cubes within `radius` of the sponge, or `None` on `(1/6, sqrt(2)/6)`, where no closed form is known.\n\nOn `(0, 1/6]` it is `(pi + 8) r^2 - 8 sqrt(2) r^3 + 48 (V1 - A1)`, the centre cube's edge\ncylinders plus the arcsine hole sums of the 24 wall carpets, `V1` over half a wall and `A1`\nover the strip along a shared edge, both summed level by level and closed by their geometric\ntails. The one term left out is `24 Deep`, `Deep` the corner volume beyond both wall tubes,\nat most `24 x 3.842e-5 < 9.23e-4` at `1/6` and `5.81e-9` at `1/12` by the paper's bound on\n`Deep`, so this is the upper edge of the certified enclosure:\n`T(1/6) = (pi + 8)/36 - sqrt(2)/27`. From `sqrt(2)/6` on the plus is swallowed, `7/27`.",
        uncallable: None,
        door: Some(door_math_three_sponge_tube),
    },
    Function {
        name: "math.three.sponge.volume",
        signature: "(radius: f64) -> f64?",
        docs: "The volume of the points of the unit cube within `radius` of the sponge, `sum_k (20/27)^k T(3^k radius)`, or `None` when some `3^k radius` falls where `T` has no closed form.\n\nThe cube is the plus and 20 retained subcubes, each holding a copy of the whole picture at a\nthird of the scale, so the volume is `T(radius)` plus `20/27` of the volume at three times the\nradius, and the sum stops at the first radius past `sqrt(2)/6`, where the volume is the cube.",
        uncallable: None,
        door: Some(door_math_three_sponge_volume),
    },
    Function {
        name: "math.three.star",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the star cube, filled where exactly one coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_star),
    },
    Function {
        name: "math.three.support",
        signature: "(counts: [u128]) -> (usize, usize)?",
        docs: "Returns the first and last height a profile fills, or none when the design is empty.",
        uncallable: None,
        door: Some(door_math_three_support),
    },
    Function {
        name: "math.three.surface",
        signature: "(cell: Cell3d) -> u128",
        docs: "Returns the count of filled faces exposed to void or the outside.",
        uncallable: None,
        door: Some(door_math_three_surface),
    },
    Function {
        name: "math.three.text",
        signature: "(cell: Cell3d, glyphs: {u8: String}?) -> [String]",
        docs: "Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.",
        uncallable: None,
        door: Some(door_math_three_text),
    },
    Function {
        name: "math.three.to_json",
        signature: "(cell: Cell3d) -> String",
        docs: "Serializes the cell's shape and types to JSON, with colors and tags when present.",
        uncallable: None,
        door: Some(door_math_three_to_json),
    },
    Function {
        name: "math.three.to_obj",
        signature: "(cell: Cell3d) -> String",
        docs: "Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.",
        uncallable: None,
        door: Some(door_math_three_to_obj),
    },
    Function {
        name: "math.three.to_strings",
        signature: "(cell: Cell3d) -> [[String]]",
        docs: "Unrolls the cube into one string of digits per row, grouped plane by plane.",
        uncallable: None,
        door: Some(door_math_three_to_strings),
    },
    Function {
        name: "math.three.tunnel_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of empty sites joined to their axis neighbors.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_tunnel_graph),
    },
    Function {
        name: "math.three.void",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_void),
    },
    Function {
        name: "math.three.voids",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of empty sites.",
        uncallable: None,
        door: Some(door_math_three_voids),
    },
    Function {
        name: "math.three.volume",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the filled-site count, the cube's volume.",
        uncallable: None,
        door: Some(door_math_three_volume),
    },
    Function {
        name: "math.three.wires",
        signature: "(cell: Cell3d) -> [[math.three.Vec3; 2]]",
        docs: "Returns the cell's edge-graph segments, scaled into the unit box.",
        uncallable: None,
        door: Some(door_math_three_wires),
    },
    Function {
        name: "math.three.xline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the x axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_xline),
    },
    Function {
        name: "math.three.xtree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the x axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_xtree),
    },
    Function {
        name: "math.three.yline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the y axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_yline),
    },
    Function {
        name: "math.three.ytree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the y axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ytree),
    },
    Function {
        name: "math.three.zeros",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the all-void cube at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_zeros),
    },
    Function {
        name: "math.three.zline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the z axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_zline),
    },
    Function {
        name: "math.three.ztree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the z axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ztree),
    },
    Function {
        name: "math.tourbillon.eyes",
        signature: "(qmax: usize) -> [math.tourbillon.Eye]",
        docs: "The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.\n\nThe odd carpet is unchanged by a quarter turn, so two layers whose angles agree modulo ninety stand on one exact lattice of nodes and the stack stops moving there.",
        uncallable: None,
        door: Some(door_math_tourbillon_eyes),
    },
    Function {
        name: "math.tourbillon.field",
        signature: "(top: usize, size: usize, schedule: str, increment: f64, set: str, weights: str, mode: str, blend: str, seed: u32) -> [f32]",
        docs: "Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.\n\nThe schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers, the weights are plain, mobius or harmonic, the render mode is cells, edges or corners and the blend is mean, sum, union, meet, parity or difference. The weights are scaled to average magnitude one and apply only under the linear blends, so mean is paper coverage and sum is the inked layer count. Sites outside the disc are NaN.\n\n# Errors\n\nErrors at a raster or top out of range, or for a schedule, set, weights or blend name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_field),
    },
    Function {
        name: "math.tourbillon.layers",
        signature: "(top: usize, schedule: str, increment: f64, set: str, weights: str, seed: u32) -> [math.tourbillon.Layer]",
        docs: "The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.\n\nThe schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers and the weights are plain, mobius or harmonic. The weights are rescaled so their magnitudes sum to the layer count, so the mean blend stays paper coverage and the sum blend stays the inked layer count.\n\n# Errors\n\nErrors at an even top or one out of range, or for a schedule, set or weights name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_layers),
    },
    Function {
        name: "math.tourbillon.period",
        signature: "(increment: f64) -> usize?",
        docs: "The least whole number of increments that closes a quarter turn, none once the count passes the cap.",
        uncallable: None,
        door: Some(door_math_tourbillon_period),
    },
    Function {
        name: "math.tourbillon.sharing",
        signature: "(list: [math.tourbillon.Layer]) -> (usize, usize)",
        docs: "The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.",
        uncallable: None,
        door: Some(door_math_tourbillon_sharing),
    },
    Function {
        name: "math.tourbillon.stack",
        signature: "(list: [math.tourbillon.Layer], size: usize, mode: str, blend: math.spin.Blend) -> [f32]",
        docs: "Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.\n\nThe render mode is cells, the lit squares of the carpet, edges, the boundaries between its cells, or corners, the vertices of its lit squares. The weights ride on the layers under the linear blends only. Sites outside the disc are NaN.\n\n# Errors\n\nErrors for a mode that is not cells, edges or corners, or a raster past the site budget.",
        uncallable: None,
        door: Some(door_math_tourbillon_stack),
    },
    Function {
        name: "math.tourbillon.stats",
        signature: "(field: [f32], size: usize, top: usize, schedule: str, increment: f64, set: str, weights: str, blend: str, seed: u32) -> math.tourbillon.Stats",
        docs: "Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.\n\nThe centre is computed from the layers at the exact half-half point, never sampled off the raster, so no turn of the layers moves it.\n\n# Errors\n\nErrors when the field is not size by size, or a name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_stats),
    },
    Function {
        name: "math.two.capacity",
        signature: "(cell: Cell2d) -> usize",
        docs: "Returns the payload bytes the cell's filled sites can hold, its length header paid for.",
        uncallable: None,
        door: Some(door_math_two_capacity),
    },
    Function {
        name: "math.two.carpet",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_carpet),
    },
    Function {
        name: "math.two.census",
        signature: "(cell: Cell2d) -> math.two.Census",
        docs: "Takes the cell's full census in one reading.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cell.",
        uncallable: None,
        door: Some(door_math_two_census),
    },
    Function {
        name: "math.two.create",
        signature: "(code: Code, number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the design a universe code names, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors when the code is out of range.",
        uncallable: None,
        door: Some(door_math_two_create),
    },
    Function {
        name: "math.two.dust",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the dust fractal, its seed on at every even-even site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_dust),
    },
    Function {
        name: "math.two.embed",
        signature: "(cell: Cell2d, payload: [u8]) -> Cell2d",
        docs: "Writes the payload over the cell's filled sites, repeating it until every site is spoken for.\n\n# Errors\n\nErrors when the payload needs more sites than the cell carries.",
        uncallable: None,
        door: Some(door_math_two_embed),
    },
    Function {
        name: "math.two.euler",
        signature: "(cell: Cell2d) -> i64",
        docs: "Returns the Euler characteristic of the filled sites, vertices less edges plus faces.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cell.",
        uncallable: None,
        door: Some(door_math_two_euler),
    },
    Function {
        name: "math.two.extract",
        signature: "(carrier: Cell2d, carried: Cell2d) -> [u8]",
        docs: "Reads the payload back, the plain cell naming the sites the carried one wrote over.\n\n# Errors\n\nErrors when the two cells differ in shape, or the header does not fit.",
        uncallable: None,
        door: Some(door_math_two_extract),
    },
    Function {
        name: "math.two.fills",
        signature: "(cell: Cell2d) -> usize",
        docs: "Counts the filled sites of the cell.",
        uncallable: None,
        door: Some(door_math_two_fills),
    },
    Function {
        name: "math.two.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors when a corner does not fit the dimension and base.",
        uncallable: None,
        door: Some(door_math_two_from_corners),
    },
    Function {
        name: "math.two.from_json",
        signature: "(text: str) -> Cell2d",
        docs: "Restores a cell from its JSON string, colors and tags included.\n\n# Errors\n\nErrors when the text is not a flat cell.",
        uncallable: None,
        door: Some(door_math_two_from_json),
    },
    Function {
        name: "math.two.from_strings",
        signature: "(rows: [String]) -> Cell2d",
        docs: "Builds a cell from rows of digits, the inverse of the text rendering.\n\n# Errors\n\nErrors when a row is empty, ragged, or holds anything but decimal digits.",
        uncallable: None,
        door: Some(door_math_two_from_strings),
    },
    Function {
        name: "math.two.hline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the hline fractal, its seed striped along odd rows, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_hline),
    },
    Function {
        name: "math.two.htree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the htree fractal, its seed striped along even rows, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_htree),
    },
    Function {
        name: "math.two.image",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> Image",
        docs: "Renders the cell to an image at the given pixel scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrs on a scale below one.",
        uncallable: None,
        door: Some(door_math_two_image),
    },
    Function {
        name: "math.two.level_set",
        signature: "(number: usize, levels: [usize], level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the level-set design, filling every residue corner whose digits sum to a named level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_level_set),
    },
    Function {
        name: "math.two.mask",
        signature: "(mask: Tensor, shape: [usize]) -> Tensor",
        docs: "Tiles the mask over the shape and crops it, the perforation pattern itself.\n\n# Errors\n\nErrors when the mask or the shape is not two-axis, or the mask is empty.",
        uncallable: None,
        door: Some(door_math_two_mask),
    },
    Function {
        name: "math.two.merge",
        signature: "(cells: [Cell2d], width: usize, height: usize) -> Cell2d",
        docs: "Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.\n\n# Errors\n\nErrors on an empty list, a wrong cell count, or cells of different shapes.",
        uncallable: None,
        door: Some(door_math_two_merge),
    },
    Function {
        name: "math.two.named",
        signature: "(design: math.bang.catalog.Design, number: usize, level: usize, rotation: usize) -> Cell2d",
        docs: "Builds the design the name picks, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors for a design that is not 2d, or a level below one.",
        uncallable: None,
        door: Some(door_math_two_named),
    },
    Function {
        name: "math.two.net",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_net),
    },
    Function {
        name: "math.two.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell2d",
        docs: "Builds a random cell, each seed site drawn on with probability density, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_noise),
    },
    Function {
        name: "math.two.ones",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds an all-filled cell of the given size and level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_ones),
    },
    Function {
        name: "math.two.payload.frame",
        signature: "() -> Tensor",
        docs: "The five by five mask a carried mosaic lays its four tiles out under.",
        uncallable: None,
        door: Some(door_math_two_payload_frame),
    },
    Function {
        name: "math.two.perimeter",
        signature: "(cell: Cell2d) -> u128",
        docs: "Counts the faces of filled sites open to emptiness or the border.",
        uncallable: None,
        door: Some(door_math_two_perimeter),
    },
    Function {
        name: "math.two.point",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the point fractal, its seed on at every odd-odd site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_point),
    },
    Function {
        name: "math.two.read",
        signature: "(sheet: Cell2d, carrier: Cell2d) -> [u8]",
        docs: "Reads the payload back from a framed sheet, the plain fourth cell naming the sites.\n\n# Errors\n\nErrors when the sheet is not five carriers across and down.",
        uncallable: None,
        door: Some(door_math_two_read),
    },
    Function {
        name: "math.two.sheet",
        signature: "(cells: [Cell2d; 4], payload: [u8]) -> Cell2d",
        docs: "Builds the framed sheet of four same-sized cells, the fourth carrying the payload.\n\n# Errors\n\nErrors when the payload needs more sites than the carrier holds.",
        uncallable: None,
        door: Some(door_math_two_sheet),
    },
    Function {
        name: "math.two.special",
        signature: "(mask: Tensor, cell: Cell2d) -> Cell2d",
        docs: "Tiles quarter-turned copies of the cell as the 2d mask directs.\n\n# Errors\n\nErrors when the mask is not 2d, or holds a rotation past three.",
        uncallable: None,
        door: Some(door_math_two_special),
    },
    Function {
        name: "math.two.star",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_star),
    },
    Function {
        name: "math.two.svg",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> String",
        docs: "Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.",
        uncallable: None,
        door: Some(door_math_two_svg),
    },
    Function {
        name: "math.two.text",
        signature: "(cell: Cell2d, glyphs: {u8: String}?) -> [String]",
        docs: "Renders the cell as rows of glyphs, or of digits where no glyph is mapped.",
        uncallable: None,
        door: Some(door_math_two_text),
    },
    Function {
        name: "math.two.to_3d",
        signature: "(cell: Cell2d) -> Cell3d",
        docs: "Lifts the flat cell into a cube one site deep, colors and tags with it.",
        uncallable: None,
        door: Some(door_math_two_to_3d),
    },
    Function {
        name: "math.two.to_json",
        signature: "(cell: Cell2d) -> String",
        docs: "Serializes the cell to a JSON string of its types, with colors and tags when present.",
        uncallable: None,
        door: Some(door_math_two_to_json),
    },
    Function {
        name: "math.two.vline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the vline fractal, its seed striped along odd columns, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_vline),
    },
    Function {
        name: "math.two.void",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_void),
    },
    Function {
        name: "math.two.voids",
        signature: "(cell: Cell2d) -> usize",
        docs: "Counts the empty sites of the cell.",
        uncallable: None,
        door: Some(door_math_two_voids),
    },
    Function {
        name: "math.two.vtree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the vtree fractal, its seed striped along even columns, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_vtree),
    },
    Function {
        name: "math.two.zeros",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds an all-empty cell of the given size and level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_zeros),
    },
];

fn door_math_arcs_curves(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::curves(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_draw(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::draw(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::arcs::law(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<bool> = take!(name, 1, &args[1]);
    match mrly_math::arcs::trace(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_essential(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::essential(&a0)))
}

fn door_math_arrays_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::arrays::fractal(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::holes(&a0)))
}

fn door_math_arrays_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::arrays::law(&a0, a1)))
}

fn door_math_arrays_lost(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::arrays::lost(&a0, a1)))
}

fn door_math_arrays_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::paired(&a0)))
}

fn door_math_arrays_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::arrays::weights(&a0)))
}

fn door_math_atoms_carpet_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::carpet_2d(a0)))
}

fn door_math_atoms_carpet_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::carpet_3d(a0)))
}

fn door_math_atoms_carpet_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::carpet_nd(a0, a1)))
}

fn door_math_atoms_dust_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::dust_2d(a0)))
}

fn door_math_atoms_dust_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::dust_3d(a0)))
}

fn door_math_atoms_dust_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::dust_nd(a0, a1)))
}

fn door_math_atoms_hline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::hline_2d(a0)))
}

fn door_math_atoms_htree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::htree_2d(a0)))
}

fn door_math_atoms_line_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::atoms::line_nd(a0, a1, a2)))
}

fn door_math_atoms_net_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::net_2d(a0)))
}

fn door_math_atoms_net_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::net_3d(a0)))
}

fn door_math_atoms_net_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::net_nd(a0, a1)))
}

fn door_math_atoms_noise_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly_math::atoms::noise_2d(a0, a1, &mut a2)))
}

fn door_math_atoms_noise_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly_math::atoms::noise_3d(a0, a1, &mut a2)))
}

fn door_math_atoms_ones_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ones_2d(a0)))
}

fn door_math_atoms_ones_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ones_3d(a0)))
}

fn door_math_atoms_point_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::point_2d(a0)))
}

fn door_math_atoms_point_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::point_3d(a0)))
}

fn door_math_atoms_point_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::point_nd(a0, a1)))
}

fn door_math_atoms_star_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::star_2d(a0)))
}

fn door_math_atoms_star_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::star_3d(a0)))
}

fn door_math_atoms_star_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::star_nd(a0, a1)))
}

fn door_math_atoms_tree_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::atoms::tree_nd(a0, a1, a2)))
}

fn door_math_atoms_vline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::vline_2d(a0)))
}

fn door_math_atoms_void_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::void_2d(a0)))
}

fn door_math_atoms_void_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::void_3d(a0)))
}

fn door_math_atoms_void_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::atoms::void_nd(a0, a1)))
}

fn door_math_atoms_vtree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::vtree_2d(a0)))
}

fn door_math_atoms_xline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::xline_3d(a0)))
}

fn door_math_atoms_xtree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::xtree_3d(a0)))
}

fn door_math_atoms_yline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::yline_3d(a0)))
}

fn door_math_atoms_ytree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ytree_3d(a0)))
}

fn door_math_atoms_zeros_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zeros_2d(a0)))
}

fn door_math_atoms_zeros_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zeros_3d(a0)))
}

fn door_math_atoms_zline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::zline_3d(a0)))
}

fn door_math_atoms_ztree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::atoms::ztree_3d(a0)))
}

fn door_math_automaton_automaton_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::abscissa(&a0)))
}

fn door_math_automaton_automaton_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::base(&a0)))
}

fn door_math_automaton_automaton_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_denominator(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::denominator(&a0)))
}

fn door_math_automaton_automaton_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::matrix(&a0)))
}

fn door_math_automaton_automaton_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    match mrly_math::automaton::Automaton::new(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::peel(&a0)))
}

fn door_math_automaton_automaton_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::period(&a0)))
}

fn door_math_automaton_automaton_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::perron(&a0)))
}

fn door_math_automaton_automaton_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::residue(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::rule(&a0)))
}

fn door_math_automaton_automaton_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::automaton::Automaton::states(&a0)))
}

fn door_math_automaton_automaton_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::automaton::Automaton::with_peel(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::automaton::Automaton::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::anf(&a0)))
}

fn door_math_bang_design_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::degree(&a0)))
}

fn door_math_bang_design_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    match mrly_math::bang::Design::name(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Design::rule(&a0)))
}

fn door_math_bang_magiclayer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::MagicLayer::new(a0, a1)))
}

fn door_math_bang_universe_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::all(&a0)))
}

fn door_math_bang_universe_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::canonical(&a0)))
}

fn door_math_bang_universe_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(give!(mrly_math::bang::Universe::design(&a0, a1)))
}

fn door_math_bang_universe_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::Universe::distinct(&a0)))
}

fn door_math_bang_universe_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::Universe::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_bang(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::bang(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_axis_maps(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::baseq::axis_maps(a0)))
}

fn door_math_bang_baseq_bracelets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::baseq::bracelets(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    match mrly_math::bang::baseq::canonical(&a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly_math::bang::baseq::carry(&a0, a1).get().to_string(),
    ))
}

fn door_math_bang_baseq_class_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::baseq::class_sequence(a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::bang::baseq::classes(a0).to_string(),
    ))
}

fn door_math_bang_baseq_distinct_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::distinct_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_even_fill_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u128 = big(name, 2, &args[2])?;
    match mrly_math::bang::baseq::even_fill_is_balanced(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::baseq::fill_from_corners(&a0, a1, a2).to_string(),
    ))
}

fn door_math_bang_baseq_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::baseq::group(a0, a1)))
}

fn door_math_bang_baseq_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::baseq::orbit(&a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_predicted_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::predicted_group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::representatives(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.get().to_string()),
                        give!(parts1.1),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::sequence(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_total_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::baseq::total_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_catalog_design_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::bang::catalog::Design::all()))
}

fn door_math_bang_catalog_antis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::catalog::antis(a0)))
}

fn door_math_bang_catalog_classics(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::catalog::classics(a0)))
}

fn door_math_bang_code_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::Code = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::bang::Code::get(a0).to_string()))
}

fn door_math_bang_code_to_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::bang::code_to_corners(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::corners(a0)))
}

fn door_math_bang_corners_to_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::corners_to_code(&a0, a1, a2)
            .get()
            .to_string(),
    ))
}

fn door_math_bang_factory_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::bang::factory::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::bang::factory::create_from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::bang::factory::create_named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_residue_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::factory::residue_corners(a0, a1)))
}

fn door_math_bang_factory_total_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::factory::total_codes(a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_level_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::bang::level_cap(a0, a1, a2)))
}

fn door_math_bang_levels_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::bang::levels_code(a0, a1, &a2).get().to_string(),
    ))
}

fn door_math_bang_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::magic(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_magic_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(String, usize)> = take!(name, 0, &args[0]);
    let b0: Vec<(&str, usize)> = a0.iter().map(|item| (item.0.as_str(), item.1)).collect();
    match mrly_math::bang::magic_named(&b0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_random_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::bang::random_code(a0, a1, &mut a2, a3) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_sources(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::catalog::Catalog = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::bang::sources(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_symmetries(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::symmetries(a0)))
}

fn door_math_bang_total_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::total_exposure(a0, a1)))
}

fn door_math_bang_touches_every_corner(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::touches_every_corner(a0, a1)))
}

fn door_math_bang_universe_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::anf(a0, a1)))
}

fn door_math_bang_universe_anf_string(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::anf_string(a0, a1)))
}

fn door_math_bang_universe_apply(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: (Vec<usize>, Vec<u8>) = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::apply(&a0, &a1)))
}

fn door_math_bang_universe_corner_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::universe::corner_index(&a0)))
}

fn door_math_bang_universe_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::universe::degree(a0, a1)))
}

fn door_math_bang_universe_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::bang::universe::orbit(a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_universe_permutations(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::universe::permutations(a0)))
}

fn door_math_bang_universe_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::universe_codes(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_schedule_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::bang::word::Schedule::all()))
}

fn door_math_bang_word_schedule_frequencies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::Schedule::frequencies(a0)))
}

fn door_math_bang_word_schedule_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::bang::word::Schedule::place(a0, a1)))
}

fn door_math_bang_word_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::components(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_constant_functional(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::constant_functional(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::dimension(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::fill(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::fills(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly_math::bang::word::letter(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_native(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::native(&a0)))
}

fn door_math_bang_word_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::period(&a0)))
}

fn door_math_bang_word_prefixes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::prefixes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_rates(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::rates(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::bang::word::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_spell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: (mrly_math::bang::MagicLayer, mrly_math::bang::MagicLayer) = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::bang::word::spell(a0, a1, a2)))
}

fn door_math_bang_word_staircase(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::bang::word::staircase(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_thue_morse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::bang::word::thue_morse(a0)))
}

fn door_math_cell_census_edges(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::edges::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::edges::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_exposure(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly_math::cell::census::exposure::<2>(&a0).to_string(),
            ))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly_math::cell::census::exposure::<3>(&a0).to_string(),
            ))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_fills(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::fills::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::fills::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_vertices(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::vertices::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::cell::census::vertices::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_voids(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::voids::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::census::voids::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_merge_reps(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly_math::cell::geometry::merge_reps::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly_math::cell::geometry::merge_reps::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 1)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::geometry::perforate::<2>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::geometry::perforate::<3>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_grow(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::grow::<2>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::grow::<3>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_anti(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::anti(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::anti(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::binarize(
                a0, a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::binarize(
                a0, a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize_otsu(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::binarize_otsu(
                a0
            )))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::binarize_otsu(
                a0
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_blur(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_combine(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::combine(
                &a0, &a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::combine(
                &a0, &a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_counting_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::counting_dtype(&a0)))
}

fn door_math_cell_models_depth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::CellNd::<3>::depth(&a0)))
}

fn door_math_cell_models_dtype_for(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::cell::models::dtype_for(a0)))
}

fn door_math_cell_models_fractal(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<2>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<3>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_height(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::height(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::height(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_invert(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::invert(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::invert(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_layers(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::layers(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::layers(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_neighbors(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<2>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<3>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_new(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            match mrly_math::cell::models::CellNd::<2>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            match mrly_math::cell::models::CellNd::<3>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_orient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::cell::models::CellNd::<3>::orient(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_models_pad(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::pad(a0, a1, a2)))
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::pad(a0, a1, a2)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly_core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::models::CellNd::<2>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly_core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::models::CellNd::<3>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_rotate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly_math::cell::models::CellNd::<2>::rotate(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: (usize, usize) = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<3>::rotate(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_tile(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            match mrly_math::cell::models::CellNd::<2>::tile(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            let a3: usize = take!(name, 3, &args[3]);
            match mrly_math::cell::models::CellNd::<3>::tile(a0, a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_types(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::types(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::types(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_width(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<2>::width(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly_math::cell::models::CellNd::<3>::width(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::paint::<2>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly_math::cell::paint::<3>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_serializer_byte_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::byte_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_byte_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::byte_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_color_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::color_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::count_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::count_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_tag_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::cell::serializer::tag_layer(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_types_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly_math::cell::serializer::types_field(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_coprime_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_math::coprime::gasket(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_closed_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::counts::Closed = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::counts::Closed::text(&a0)))
}

fn door_math_counts_exposure_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::counts::Exposure = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::Exposure::at(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::counts::Exposure::from_corners(
        &a0, a1, a2, a3
    )))
}

fn door_math_counts_exposure_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::counts::Exposure::of_tile(&a0)))
}

fn door_math_counts_exposure_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::counts::Exposure = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::Exposure::recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_centered_hexagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::counts::centered_hexagonal(a0).to_string(),
    ))
}

fn door_math_counts_cut_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::cut_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_cut_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::cut_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::dimension(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_edges_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::edges_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::exposure(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_exposure_of_tile_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly_math::counts::exposure_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_recurrence_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::exposure_recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::fill(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    Ok(Value::String(
        mrly_math::counts::fill_from_corners(&a0, a1, a2, a3, a4).to_string(),
    ))
}

fn door_math_counts_fill_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::fill_polynomial(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::counts::grid(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_ladder_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::ladder::cap(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_carry_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::carry_matrix(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::characteristic(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_determinant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::determinant(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_digit_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::digit_polynomial(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_even_block(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::even_block(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::fill(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::counts::ladder::ladder(a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly_math::counts::ladder::perron(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::sign(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_spectral_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::counts::ladder::spectral_ratio(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    Ok(Value::String(
        mrly_math::counts::ladder::trace(&a0).to_string(),
    ))
}

fn door_math_counts_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::limit(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_odd_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::odd_power(a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::counts::pairs(&a0) {
            list0.push({
                let parts1 = item0;
                Value::Array(vec![
                    Value::String(parts1.0.to_string()),
                    Value::String(parts1.1.to_string()),
                ])
            });
        }
        Value::Array(list0)
    })
}

fn door_math_counts_positions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly_math::counts::positions(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_pro_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::pro_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_pro_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::counts::pro_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_profile_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::counts::profile_of_tile(&a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::ratio(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_rational(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::rational(a0, a1, a2, a3, a4) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_grid_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_math::counts::six::grid_triangles(a0, a1).to_string(),
    ))
}

fn door_math_counts_six_solid_slice_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_boundary(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_core_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_core_nodes(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_interior(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_interior(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_triangles(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_vertices(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::counts::six::solid_slice_vertices(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_skeleton_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrly_math::counts::skeleton_recurrence(a0, a1, a2, a3, a4) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::counts::surface(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::counts::void(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_digits_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::digits_of(a0, a1)))
}

fn door_math_design_echo_series(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::design::echo_series(&a0, &a1, a2)))
}

fn door_math_design_elements(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::design::elements(a0, &a1, a2)))
}

fn door_math_design_log_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::log_grid(&a0, a1)))
}

fn door_math_design_median_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::median_floor(&a0, a1)))
}

fn door_math_design_meter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::design::meter(&a0)))
}

fn door_math_design_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::nearest(a0, &a1)))
}

fn door_math_design_peaks(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: (f64, f64) = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::design::peaks(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_pole_lattice(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::pole_lattice(a0, a1)))
}

fn door_math_design_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: Vec<f64> = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::design::resample(&a0, &a1, a2, &a3)))
}

fn door_math_design_score(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::design::score(&a0, a1)))
}

fn door_math_design_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(mrly_math::design::size(&a0, a1).to_string()))
}

fn door_math_design_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    match mrly_math::design::spectrum(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_upper_rms(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::design::upper_rms(&a0)))
}

fn door_math_dissection_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::cap(a0)))
}

fn door_math_dissection_chain_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_exponent(a0)))
}

fn door_math_dissection_chain_margin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_margin(a0)))
}

fn door_math_dissection_chain_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::chain_root(a0)))
}

fn door_math_dissection_chain_wall(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::dissection::chain_wall()))
}

fn door_math_dissection_consecutive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::dissection::consecutive(&a0)))
}

fn door_math_dissection_fraction(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::fraction(a0, a1, a2)))
}

fn door_math_dissection_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::dissection::kappa(a0, &a1)))
}

fn door_math_dissection_masses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::masses(a0, &a1, a2)))
}

fn door_math_dissection_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::dissection::reach(a0, a1)))
}

fn door_math_dissection_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::reading(a0, a1, &a2)))
}

fn door_math_dissection_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::regions(a0, a1, a2)))
}

fn door_math_dissection_tally(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::dissection::tally(a0, &a1, a2, a3)))
}

fn door_math_dissection_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::dissection::weights(a0, &a1, a2)))
}

fn door_math_game_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::game::cell(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::game::cells(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u64>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::game::counts(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_de_bruijn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::game::de_bruijn(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_mosaic_game(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::game::Mosaic = take!(name, 0, &args[0]);
    match mrly_math::game::mosaic_game(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_named_games(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::game::named_games()))
}

fn door_math_game_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<u64>> = take!(name, 0, &args[0]);
    match mrly_math::game::perron(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_reverse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::game::reverse(&a0)))
}

fn door_math_game_slice_game(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    match mrly_math::game::slice_game(a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_game_successors(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::game::successors(&a0, &a1)))
}

fn door_math_game_word_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::game::word_cell(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_adjacency(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::Network::adjacency(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::Network::degree(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::graph::Network::new(a0)))
}

fn door_math_graph_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::edge_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_fractal_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::graph::fractal_dimension(&a0, a1)))
}

fn door_math_graph_junctions(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::junctions(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_largest_component(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::largest_component(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_roles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::roles(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_tips(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    match mrly_math::graph::tips(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_total_length(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::graph::total_length(&a0)))
}

fn door_math_graph_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_math::graph::tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::abscissa(&a0)))
}

fn door_math_ladder_design_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::base(&a0)))
}

fn door_math_ladder_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::digits(&a0)))
}

fn door_math_ladder_design_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    match mrly_math::ladder::Design::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::peel(&a0)))
}

fn door_math_ladder_design_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::ladder::Design::period(&a0)))
}

fn door_math_ladder_design_pole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::ladder::Design::pole(&a0, a1, a2)))
}

fn door_math_ladder_design_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::ladder::Design::with_peel(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::ladder::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::ladder::residue(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::ladder::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_accepts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::Rule::accepts(&a0, &a1)))
}

fn door_math_memory_rule_allowed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::Rule::allowed(&a0, a1)))
}

fn door_math_memory_rule_alphabet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::alphabet(&a0)))
}

fn door_math_memory_rule_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::memory::Rule::codes(&a0).to_string(),
    ))
}

fn door_math_memory_rule_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::memory::Rule::full(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::letters(&a0)))
}

fn door_math_memory_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_math::memory::Rule::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::states(&a0)))
}

fn door_math_memory_rule_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::Rule::windows(&a0)))
}

fn door_math_memory_allowed_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::allowed_windows(&a0)))
}

fn door_math_memory_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::cells(&a0, a1)))
}

fn door_math_memory_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::memory::counts(&a0, a1)))
}

fn door_math_memory_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::exponent(&a0)))
}

fn door_math_memory_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::kappa(&a0)))
}

fn door_math_memory_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::perron(&a0)))
}

fn door_math_memory_transfer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::memory::transfer(&a0)))
}

fn door_math_moire_field_as_f64(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::as_f64(&a0)))
}

fn door_math_moire_field_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::Field::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_field_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::max(&a0)))
}

fn door_math_moire_field_mean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::mean(&a0)))
}

fn door_math_moire_field_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::min(&a0)))
}

fn door_math_moire_field_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Field::new(a0)))
}

fn door_math_moire_field_normalized(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Field::normalized(&a0, a1)))
}

fn door_math_moire_layer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Layer::new(a0, a1)))
}

fn door_math_moire_preset_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::carpet(a0)))
}

fn door_math_moire_preset_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::heatmap(a0)))
}

fn door_math_moire_preset_hive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::hive(a0)))
}

fn door_math_moire_preset_weave(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Preset::weave(a0)))
}

fn door_math_moire_spec_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::Spec::new(a0, a1, a2)))
}

fn door_math_moire_volume_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::moire::Volume::at(&a0, a1, a2, a3)))
}

fn door_math_moire_volume_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::count(&a0, a1)))
}

fn door_math_moire_volume_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::Volume::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::max(&a0)))
}

fn door_math_moire_volume_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::min(&a0)))
}

fn door_math_moire_volume_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::Volume::new(a0)))
}

fn door_math_moire_volume_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: mrly_math::moire::Frame = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::Volume::plane(&a0, &a1, a2)))
}

fn door_math_moire_volume_sample(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: [f64; 3] = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::sample(&a0, a1)))
}

fn door_math_moire_volume_solid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::Volume::solid(&a0, a1)))
}

fn door_math_moire_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::moire::all(a0)))
}

fn door_math_moire_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::moire::frame(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::moire::Layer = take!(name, 0, &args[0]);
    match mrly_math::moire::layer(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::named(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_correlation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::pairs::correlation(a0, a1)))
}

fn door_math_moire_pairs_sampled(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::moire::pairs::sampled(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::moire::pairs::witness(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly_math::moire::Field = take!(name, 0, &args[0]);
    let a1: mrly_core::Colorizer = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly_math::moire::render(&a0, &a1, a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::moire::Lattice = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::moire::sample::axes(a0, a1, a2)))
}

fn door_math_moire_sample_membership(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::moire::sample::membership(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_pack(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::moire::sample::pack(&a0, a1)))
}

fn door_math_moire_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly_math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::moire::Lattice = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    let a6: Vec<f64> = take!(name, 6, &args[6]);
    match mrly_math::moire::stack(a0, &a1, a2, a3, a4, a5, &a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_stack_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<mrly_math::moire::Spec> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly_math::moire::Lattice = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: Vec<f64> = take!(name, 5, &args[5]);
    match mrly_math::moire::stack_codes(&a0, a1, a2, a3, a4, &a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly_math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::moire::volume(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match mrly_math::name::Bang::cells(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::name::Bang::new(a0, a1, a2)))
}

fn door_math_name_bang_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Bang as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_bang_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Bang as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_bang_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Bang = take!(name, 0, &args[0]);
    match <mrly_math::name::Bang as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_lattice_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::name::Lattice::default()))
}

fn door_math_name_lattice_is_square(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Lattice::is_square(&a0)))
}

fn door_math_name_lattice_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Lattice::units(a0)))
}

fn door_math_name_sequence_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Sequence::design(&a0)))
}

fn door_math_name_sequence_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    Ok(give!(mrly_math::name::Sequence::new(
        a0,
        a1,
        a2,
        a3.as_str(),
        a4.as_str()
    )))
}

fn door_math_name_sequence_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Sequence as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_sequence_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Sequence as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_sequence_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly_math::name::Sequence as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_bases(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Word::bases(&a0)))
}

fn door_math_name_word_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::name::Word::letters(&a0)))
}

fn door_math_name_word_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<(u128, usize)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push({
                let parts1 = parts(name, 1, item0, 2)?;
                (big(name, 1, &parts1[0])?, take!(name, 1, &parts1[1]))
            });
        }
        list0
    };
    match mrly_math::name::Word::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Word as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_word_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_math::name::Word as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_word_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::name::Word = take!(name, 0, &args[0]);
    match <mrly_math::name::Word as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::name::of(a0, a1, a2)))
}

fn door_math_name_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::name::words(a0, a1)))
}

fn door_math_nodal_figures(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::nodal::figures(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_nodal_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::nodal::holds(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::press::Press::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::press::Press = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly_math::press::Press::total(&a0, a1).to_string(),
    ))
}

fn door_math_press_press_totals(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::press::Press = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_math::press::Press::totals(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_press_containing(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::containing(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_coordinates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::coordinates(a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_count_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u128 = big(name, 3, &args[3])?;
    match mrly_math::press::count_below(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::distinct(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_interleave(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::press::interleave(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_layer_table(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly_math::press::layer_table(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: u128 = big(name, 1, &args[1])?;
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::members(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::press::profile(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_usage(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::press::usage(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_count(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::press::word_member(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_members(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly_math::press::word_profile(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly_math::radix::Base::class(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_congruent(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::radix::Base::congruent(a0, a1, a2)))
}

fn door_math_radix_base_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    match mrly_math::radix::Base::group(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_mirrored(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::mirrored(a0)))
}

fn door_math_radix_base_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly_math::radix::Base::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::norm(a0)))
}

fn door_math_radix_base_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Base::power(a0, a1)))
}

fn door_math_radix_base_residues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    match mrly_math::radix::Base::residues(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::ring(a0)))
}

fn door_math_radix_base_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Base::value(a0)))
}

fn door_math_radix_radix_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::base(&a0)))
}

fn door_math_radix_radix_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    match mrly_math::radix::Radix::canonical(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    match mrly_math::radix::Radix::code(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::digits(&a0)))
}

fn door_math_radix_radix_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::dimension(&a0)))
}

fn door_math_radix_radix_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::distinct(&a0, a1)))
}

fn door_math_radix_radix_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_math::radix::Radix::fill(&a0, a1).to_string(),
    ))
}

fn door_math_radix_radix_from_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::radix::Radix::from_code(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::radix::Base = take!(name, 0, &args[0]);
    let a1: Vec<(i64, i64)> = take!(name, 1, &args[1]);
    let a2: Vec<(i64, i64)> = take!(name, 2, &args[2]);
    match mrly_math::radix::Radix::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::plane(&a0, a1)))
}

fn door_math_radix_radix_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::ring(&a0)))
}

fn door_math_radix_radix_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::size(&a0)))
}

fn door_math_radix_radix_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::radix::Radix::twists(&a0)))
}

fn door_math_radix_radix_with_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::radix::Radix::with_twists(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::radix::Radix::words(&a0, a1)))
}

fn door_math_radix_flowsnake(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::flowsnake() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::gasket() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_koch(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::koch() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_terdragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::terdragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly_math::radix::tile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_twindragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_math::radix::twindragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_nodes_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::roulette::Nodes::default()))
}

fn door_math_roulette_nodes_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::Nodes::pair(&a0, a1, a2)))
}

fn door_math_roulette_nodes_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::paired(&a0)))
}

fn door_math_roulette_nodes_selved(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::selved(&a0)))
}

fn door_math_roulette_nodes_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::roulette::Nodes::total(&a0)))
}

fn door_math_roulette_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly_math::roulette::nodes(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: [f64; 2] = take!(name, 0, &args[0]);
    let a1: [f64; 2] = take!(name, 1, &args[1]);
    let a2: [f64; 2] = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::side(a0, a1, a2)))
}

fn door_math_roulette_spread(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::roulette::spread(&a0, &a1, a2)))
}

fn door_math_rules_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::rules::render(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_rules_tree_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::rules::tree_axes(a0, a1)))
}

fn door_math_shape_frac_minus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::minus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_plus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::plus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_times(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Frac = take!(name, 1, &args[1]);
    match mrly_math::shape::Frac::times(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::Frac::whole(a0)))
}

fn door_math_shape_region_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::shape::Region = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::Region::flip(a0)))
}

fn door_math_shape_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    match mrly_math::shape::census(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_classify(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrly_math::shape::classify(&a0, a1, &a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Shape = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_math::shape::crop(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crossing_shell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::crossing_shell(a0, a1, a2)))
}

fn door_math_shape_crossing_tree(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: Vec<bool> = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::crossing_tree(a0, a1, &a2)))
}

fn door_math_shape_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::shape::fit(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly_math::shape::Frac = take!(name, 2, &args[2]);
    match mrly_math::shape::named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_radial_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::shape::radial_census(&a0, &a1, a2)))
}

fn door_math_shape_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly_math::shape::reach(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_refine(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::shape::Shape = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    match mrly_math::shape::refine(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::shape::regions(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_shapes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::shape::shapes(a0)))
}

fn door_math_six_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::anti(a0)))
}

fn door_math_six_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::Cell6d::binarize(a0, a1)))
}

fn door_math_six_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::binarize_otsu(a0)))
}

fn door_math_six_blank(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Orientation = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    match mrly_math::six::blank(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_math::six::Cell6d::blur(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::census(&a0, a1)))
}

fn door_math_six_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_copies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::six::copies(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::cut(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::cut_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::disc(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_east(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::east(a0, a1)))
}

fn door_math_six_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::euler(&a0, a1)))
}

fn door_math_six_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::fills(&a0)))
}

fn door_math_six_fills_only(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::fills_only(&a0)))
}

fn door_math_six_framed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::framed(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::six::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::giant(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant_network(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::giant_network(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::height(&a0)))
}

fn door_math_six_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::image(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_is_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::is_cube(&a0)))
}

fn door_math_six_is_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::is_hex(&a0)))
}

fn door_math_six_iso(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::iso(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_iso_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::iso_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Projection = take!(name, 1, &args[1]);
    let a2: mrly_math::six::Orientation = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::six::Cell6d::new(a0, a1, a2, a3)))
}

fn door_math_six_north(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::north(a0, a1)))
}

fn door_math_six_orientation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::orientation(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::six::pad(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) {
        None
    } else {
        Some(
            {
                let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect(),
        )
    };
    let a2: Option<mrly_core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() {
        None
    } else {
        Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
    };
    match mrly_math::six::paint(a0, a1.as_ref(), a2, a3.as_mut()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_math::six::Cell6d::perforate(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::six::pro(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::six::pro_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::radial(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly_math::six::radial_crop(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::six::Orientation = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::radial_mask(a0, a1)))
}

fn door_math_six_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::raster(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly_math::six::rect_image(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly_math::six::rect_svg(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rim_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::rim_holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_skin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::skin(&a0)))
}

fn door_math_six_slice_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_dual_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_dual_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<u8> = take!(name, 1, &args[1]);
    match mrly_math::six::slice_edge_graph(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly_math::six::slice_tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_south(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::south(a0, a1)))
}

fn door_math_six_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::six::spectral_exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_branch_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::constant(a0)))
}

fn door_math_six_star_branch_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::name(a0)))
}

fn door_math_six_star_branch_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::of(a0)))
}

fn door_math_six_star_branch_residual(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Branch::residual(a0)))
}

fn door_math_six_star_share_reduced(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Share::reduced(a0)))
}

fn door_math_six_star_share_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::Share::value(a0)))
}

fn door_math_six_star_star_arm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::star::Star::arm(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    Ok(give!(mrly_math::six::star::Star::cell(&a0, a1, a2, a3)))
}

fn door_math_six_star_star_excesses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::star::Star::excesses(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_hexagon(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::star::Star::hexagon(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u128 = big(name, 0, &args[0])?;
    match mrly_math::six::star::Star::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_arm_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_math::six::star::arm_law(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::chi8(a0)))
}

fn door_math_six_star_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::six::star::constant()))
}

fn door_math_six_star_decay(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::six::star::decay(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_width_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::star::width_law(a0)))
}

fn door_math_six_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrly_math::six::svg(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    match mrly_math::six::tessellate(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::six::tile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly_math::six::tile_cell(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: (usize, usize) = take!(name, 1, &args[1]);
    match mrly_math::six::tile_crop(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: (usize, usize) = take!(name, 0, &args[0]);
    match mrly_math::six::tile_step(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::to_json(&a0)))
}

fn door_math_six_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<usize> = take!(name, 1, &args[1]);
    match mrly_math::six::triangles(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_west(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::six::west(a0, a1)))
}

fn door_math_six_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::six::Cell6d::width(&a0)))
}

fn door_math_spectrum_clusters(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly_math::spectrum::clusters(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_math::spectrum::laplacian(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_math::spectrum::laplacian_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_multiplicity(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spectrum::multiplicity(&a0, a1, a2)))
}

fn door_math_spectrum_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spectrum::spectral_exponent(&a0, a1)))
}

fn door_math_spectrum_spectral_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spectrum::spectral_fit(&a0, a1)))
}

fn door_math_spectrum_spectral_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spectrum::spectral_points(&a0)))
}

fn door_math_spectrum_symmetric_eigenvalues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<f64>> = take!(name, 0, &args[0]);
    match mrly_math::spectrum::symmetric_eigenvalues(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_blend_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spin::Blend = take!(name, 0, &args[0]);
    let a1: Vec<f32> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::Blend::fold(a0, &a1)))
}

fn door_math_spin_blend_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::Blend::named(a0.as_str())))
}

fn door_math_spin_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::spin::arcs(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_harmonics(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::spin::harmonics(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_mass(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::mass(&a0, a1)))
}

fn door_math_spin_mass_within(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spin::mass_within(&a0, a1, a2)))
}

fn door_math_spin_petals(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::petals(a0, a1)))
}

fn door_math_spin_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::spin::profile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: mrly_math::spin::Blend = take!(name, 5, &args[5]);
    let a6: usize = take!(name, 6, &args[6]);
    match mrly_math::spin::radial(&a0, a1, a2, a3, a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::reach(a0)))
}

fn door_math_spin_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_math::spin::ring(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_turns(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spin::turns(&a0)))
}

fn door_math_spin_wheel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spin::wheel(&a0, a1)))
}

fn door_math_spirograph_seats_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::spirograph::Seats::default()))
}

fn door_math_spirograph_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::cell(a0, a1, a2)))
}

fn door_math_spirograph_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::spirograph::cover(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    match mrly_math::spirograph::disc(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::distinct(&a0, &a1, a2)))
}

fn door_math_spirograph_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::frame(&a0, &a1)))
}

fn door_math_spirograph_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::nodes(&a0, &a1, a2)))
}

fn door_math_spirograph_pencils(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: f64 = take!(name, 5, &args[5]);
    let a6: u32 = take!(name, 6, &args[6]);
    match mrly_math::spirograph::pencils(&a0, a1, a2, a3.as_str(), a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly_math::spirograph::Pencil = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::point(&a0, &a1, a2)))
}

fn door_math_spirograph_pose(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::pose(&a0, a1)))
}

fn door_math_spirograph_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::spirograph::representatives(&a0, &a1, a2)))
}

fn door_math_spirograph_seats(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::spirograph::Pencil> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::spirograph::seats(&a0)))
}

fn door_math_spirograph_signed_area(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly_math::spirograph::Pencil = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::signed_area(&a0, &a1)))
}

fn door_math_spirograph_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly_math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::spirograph::trace(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_track(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::spirograph::track(a0.as_str(), a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::spirograph::turn(&a0, a1)))
}

fn door_math_sumset_pair_clean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::clean(&a0)))
}

fn door_math_sumset_pair_copy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::copy(&a0)))
}

fn door_math_sumset_pair_energy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly_math::sumset::Pair::energy(&a0).to_string(),
    ))
}

fn door_math_sumset_pair_four(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::four(&a0)))
}

fn door_math_sumset_pair_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::gap(&a0)))
}

fn door_math_sumset_pair_largest(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::largest(&a0)))
}

fn door_math_sumset_pair_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_math::sumset::Pair::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_sumset_pair_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(give!(mrly_math::sumset::Pair::ratio(&a0, a1)))
}

fn door_math_sumset_pair_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::scale(&a0)))
}

fn door_math_sumset_pair_three(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::sumset::Pair::three(&a0)))
}

fn door_math_sumset_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_math::sumset::pairs(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_vec3_cross(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly_math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::cross(a0, a1)))
}

fn door_math_three_vec3_dot(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly_math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::dot(a0, a1)))
}

fn door_math_three_vec3_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f32 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    let a2: f32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_math::three::Vec3::new(a0, a1, a2)))
}

fn door_math_three_vec3_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::Vec3::scale(a0, a1)))
}

fn door_math_three_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::three::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_core_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::core_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::core_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::create(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::three::diagonal_slice(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Vec<usize> = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly_math::three::diagonal_svg(a0, a1, a2, a3, &a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_edge_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::edge_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::edge_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly_math::three::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::extrude(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::extrude_cube(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_faces(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::faces(&a0)))
}

fn door_math_three_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::fills(&a0)))
}

fn door_math_three_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::from_corners(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::three::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<String>> = take!(name, 0, &args[0]);
    match mrly_math::three::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_hidden(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::three::hidden(&a0).to_string()))
}

fn door_math_three_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::level_set(a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_magic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            match mrly_math::three::magic::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            match mrly_math::three::magic::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_manhattan_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::manhattan_layers(a0)))
}

fn door_math_three_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::merge(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_mosaic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 1)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 1, &args[1]);
            match mrly_math::three::mosaic::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly_math::cell::models::CellNd<3>> = take!(name, 1, &args[1]);
            match mrly_math::three::mosaic::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::bang::catalog::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::named(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly_core::Rng::new(seed(name, 3, &args[3])?);
    match mrly_math::three::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_orientations(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::three::orientations()))
}

fn door_math_three_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::three::profile(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_project(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::project(a0)))
}

fn door_math_three_quads(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::quads(&a0)))
}

fn door_math_three_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::shadow(a0)))
}

fn door_math_three_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::three::slice(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    match mrly_math::three::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_sponge_deep(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::deep(a0)))
}

fn door_math_three_sponge_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::three::sponge::dimension()))
}

fn door_math_three_sponge_distance(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::distance(a0)))
}

fn door_math_three_sponge_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::exact(a0)))
}

fn door_math_three_sponge_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::profile(a0)))
}

fn door_math_three_sponge_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::reading(a0)))
}

fn door_math_three_sponge_tube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::tube(a0)))
}

fn door_math_three_sponge_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::sponge::volume(a0)))
}

fn door_math_three_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_support(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    Ok(give!(mrly_math::three::support(&a0)))
}

fn door_math_three_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::three::surface(&a0).to_string()))
}

fn door_math_three_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::three::text(&a0, a1.as_ref())))
}

fn door_math_three_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_json(&a0)))
}

fn door_math_three_to_obj(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_obj(&a0)))
}

fn door_math_three_to_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::to_strings(&a0)))
}

fn door_math_three_tunnel_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly_math::three::tunnel_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly_math::three::tunnel_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::voids(&a0)))
}

fn door_math_three_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::volume(&a0)))
}

fn door_math_three_wires(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::three::wires(&a0)))
}

fn door_math_three_xline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::xline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_xtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::xtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_yline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::yline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ytree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ytree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::zline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ztree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::three::ztree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_eyes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::eyes(a0)))
}

fn door_math_tourbillon_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrly_math::tourbillon::field(
        a0,
        a1,
        a2.as_str(),
        a3,
        a4.as_str(),
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: String = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: u32 = take!(name, 5, &args[5]);
    match mrly_math::tourbillon::layers(a0, a1.as_str(), a2, a3.as_str(), a4.as_str(), a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::period(a0)))
}

fn door_math_tourbillon_sharing(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::tourbillon::Layer> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::tourbillon::sharing(&a0)))
}

fn door_math_tourbillon_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly_math::tourbillon::Layer> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: mrly_math::spin::Blend = take!(name, 3, &args[3]);
    match mrly_math::tourbillon::stack(&a0, a1, a2.as_str(), a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_stats(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrly_math::tourbillon::stats(
        &a0,
        a1,
        a2,
        a3.as_str(),
        a4,
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_capacity(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::capacity(&a0)))
}

fn door_math_two_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly_math::two::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_embed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly_math::two::embed(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly_math::two::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_extract(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::extract(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::fills(&a0)))
}

fn door_math_two_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_math::two::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    match mrly_math::two::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_hline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::hline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_htree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::htree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::two::Shape = take!(name, 4, &args[4]);
    match mrly_math::two::image(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly_math::two::level_set(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_math::two::mask(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_math::two::merge(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_math::bang::catalog::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_math::two::named(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly_core::Rng::new(seed(name, 3, &args[3])?);
    match mrly_math::two::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_payload_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_math::two::payload::frame()))
}

fn door_math_two_perimeter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_math::two::perimeter(&a0).to_string()))
}

fn door_math_two_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::read(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_sheet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_math::cell::models::CellNd<2>; 4] = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly_math::two::sheet(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly_math::two::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly_core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly_math::two::Shape = take!(name, 4, &args[4]);
    Ok(give!(mrly_math::two::svg(&a0, a1, a2, a3, a4)))
}

fn door_math_two_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly_math::two::text(&a0, a1.as_ref())))
}

fn door_math_two_to_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::to_3d(&a0)))
}

fn door_math_two_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::to_json(&a0)))
}

fn door_math_two_vline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::vline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_math::two::voids(&a0)))
}

fn door_math_two_vtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::vtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_math::two::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}
