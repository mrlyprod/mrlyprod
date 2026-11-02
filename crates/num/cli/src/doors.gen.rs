use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "num.apollonian.Circle.centre",
        signature: "(self: num.apollonian.Circle) -> (f64, f64)?",
        docs: "The centre, none on a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_centre),
    },
    Function {
        name: "num.apollonian.Circle.is_line",
        signature: "(self: num.apollonian.Circle) -> bool",
        docs: "Whether the circle is a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_is_line),
    },
    Function {
        name: "num.apollonian.Circle.radius",
        signature: "(self: num.apollonian.Circle) -> f64?",
        docs: "The radius, none on a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_radius),
    },
    Function {
        name: "num.apollonian.flat",
        signature: "(name: str, cap: i64) -> [f64]",
        docs: "Grows the named packing and lays every circle of positive curvature as `x, y, r, k` in one flat run: the root's own circles first in root order, then the grown ones in curvature order.\n\n# Errors\n\nErrs as `grow` does.",
        uncallable: None,
        door: Some(door_num_apollonian_flat),
    },
    Function {
        name: "num.apollonian.form",
        signature: "(u: [i64; 4], v: [i64; 4]) -> i128",
        docs: "The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.",
        uncallable: None,
        door: Some(door_num_apollonian_form),
    },
    Function {
        name: "num.apollonian.frame",
        signature: "(p: num.apollonian.Packing) -> [f64; 4]",
        docs: "The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.",
        uncallable: None,
        door: Some(door_num_apollonian_frame),
    },
    Function {
        name: "num.apollonian.grow",
        signature: "(name: str, cap: i64) -> num.apollonian.Packing",
        docs: "Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.\n\n# Errors\n\nErrs at a name outside `ROOTS`, and at a curvature cap outside two through `CURVATURE_CAP`.",
        uncallable: None,
        door: Some(door_num_apollonian_grow),
    },
    Function {
        name: "num.apollonian.is_ford",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.",
        uncallable: None,
        door: Some(door_num_apollonian_is_ford),
    },
    Function {
        name: "num.apollonian.on_line",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.",
        uncallable: None,
        door: Some(door_num_apollonian_on_line),
    },
    Function {
        name: "num.apollonian.reflect",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> num.apollonian.Circle",
        docs: "Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.",
        uncallable: None,
        door: Some(door_num_apollonian_reflect),
    },
    Function {
        name: "num.apollonian.root",
        signature: "(name: str) -> [num.apollonian.Circle; 4]",
        docs: "The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.\n\n# Errors\n\nErrs at a name outside `ROOTS`.",
        uncallable: None,
        door: Some(door_num_apollonian_root),
    },
    Function {
        name: "num.apollonian.shadow",
        signature: "(p: num.apollonian.Packing, order: usize) -> num.apollonian.Shadow",
        docs: "Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.\n\n# Errors\n\nErrs at an order outside one through `ORDER_CAP`.",
        uncallable: None,
        door: Some(door_num_apollonian_shadow),
    },
    Function {
        name: "num.apollonian.sound",
        signature: "(q: [num.apollonian.Circle; 4]) -> bool",
        docs: "Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.",
        uncallable: None,
        door: Some(door_num_apollonian_sound),
    },
    Function {
        name: "num.apollonian.swap",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> [num.apollonian.Circle; 4]",
        docs: "The quadruple with the circle at the seat replaced by its reflection.",
        uncallable: None,
        door: Some(door_num_apollonian_swap),
    },
    Function {
        name: "num.apollonian.touches",
        signature: "(p: num.apollonian.Packing) -> [num.apollonian.Touch]",
        docs: "The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.",
        uncallable: None,
        door: Some(door_num_apollonian_touches),
    },
    Function {
        name: "num.blend.add",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Adds two sequences term by term over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_add),
    },
    Function {
        name: "num.blend.cauchy",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Convolves two sequences, keeping the exact prefix their shared length affords.\n\n# Errors\n\nErrs when a term passes a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_blend_cauchy),
    },
    Function {
        name: "num.blend.characteristic",
        signature: "(coefficients: [(i128, i128)]) -> [(i128, i128)]",
        docs: "Returns the monic characteristic polynomial of a recurrence, highest power first.",
        uncallable: None,
        door: Some(door_num_blend_characteristic),
    },
    Function {
        name: "num.blend.decimate",
        signature: "(a: [i128], step: usize, offset: usize) -> [i128]",
        docs: "Keeps every step-th term from the offset onward.\n\n# Errors\n\nErrs at a step of zero.",
        uncallable: None,
        door: Some(door_num_blend_decimate),
    },
    Function {
        name: "num.blend.delta",
        signature: "(a: [i128]) -> [i128]",
        docs: "Returns the first differences of a sequence, one term shorter.",
        uncallable: None,
        door: Some(door_num_blend_delta),
    },
    Function {
        name: "num.blend.growth",
        signature: "(coefficients: [(i128, i128)]) -> f64",
        docs: "Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.\n\nA simple root lands at machine precision; a repeated root lands to a few decimals only.",
        uncallable: None,
        door: Some(door_num_blend_growth),
    },
    Function {
        name: "num.blend.hadamard",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Multiplies two sequences term by term over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_hadamard),
    },
    Function {
        name: "num.blend.recurrence",
        signature: "(terms: [i128]) -> [(i128, i128)]?",
        docs: "Finds the smallest linear constant-coefficient recurrence that fits every supplied term.\n\nThe coefficients come back as reduced fractions with the newest term first, so a\nresult of one and one twice is the Fibonacci rule. The hunt runs modulo three\nprimes, rebuilds the rationals by remainder reconstruction, and verifies the rule\non every term before answering; too few terms to trust an order returns nothing,\nand the zero sequence returns the empty rule.",
        uncallable: None,
        door: Some(door_num_blend_recurrence),
    },
    Function {
        name: "num.blend.scale",
        signature: "(a: [i128], factor: i128) -> [i128]",
        docs: "Multiplies every term of a sequence by the factor.",
        uncallable: None,
        door: Some(door_num_blend_scale),
    },
    Function {
        name: "num.blend.shift",
        signature: "(a: [i128], count: usize) -> [i128]",
        docs: "Drops the first terms of a sequence.",
        uncallable: None,
        door: Some(door_num_blend_shift),
    },
    Function {
        name: "num.blend.sigma",
        signature: "(a: [i128]) -> [i128]",
        docs: "Returns the partial sums of a sequence.\n\n# Errors\n\nErrs when a partial sum passes a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_blend_sigma),
    },
    Function {
        name: "num.blend.sub",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Subtracts the second sequence from the first over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_sub),
    },
    Function {
        name: "num.boolean.is_balanced",
        signature: "(code: u128, n: usize) -> bool",
        docs: "Reports whether the packed function outputs one on exactly half of its inputs.",
        uncallable: None,
        door: Some(door_num_boolean_is_balanced),
    },
    Function {
        name: "num.boolean.nonlinearity",
        signature: "(code: u128, n: usize) -> i64",
        docs: "Returns how far the packed function sits from every affine function, zero when it is one.",
        uncallable: None,
        door: Some(door_num_boolean_nonlinearity),
    },
    Function {
        name: "num.boolean.sac",
        signature: "(code: u128, n: usize) -> f64",
        docs: "Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.",
        uncallable: None,
        door: Some(door_num_boolean_sac),
    },
    Function {
        name: "num.boolean.walsh_spectrum",
        signature: "(code: u128, n: usize) -> [i64]",
        docs: "Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.",
        uncallable: None,
        door: Some(door_num_boolean_walsh_spectrum),
    },
    Function {
        name: "num.chladni.ladder",
        signature: "(cap: usize) -> [num.chladni.Rung]",
        docs: "Lists every distinct plate figure with `0 <= n <= m <= cap` but `(0, 0)`, both signs when `n < m` and the sign 1 when `n == m`, by pitch, then `n`, then the sign 1 before -1.\n\n# Errors\n\nErrs when the cap is over 64.",
        uncallable: None,
        door: Some(door_num_chladni_ladder),
    },
    Function {
        name: "num.chladni.lines",
        signature: "(field: [f64], side: usize) -> [f64]",
        docs: "Traces the zero set of a plate raster by marching squares on the lattice of cell centres, its holes and its rim ghosted to the mean of their plate neighbours so the lines run to every free edge: segments `x0, y0, x1, y1` in `[0, 1]` back to back, pieces inside a NaN cell dropped, a saddle joined by the mean of its four corners.\n\n# Errors\n\nErrs when the side is 0, or the field is not the side squared or holds an infinity.",
        uncallable: None,
        door: Some(door_num_chladni_lines),
    },
    Function {
        name: "num.chladni.plate",
        signature: "(n: usize, m: usize, sign: i32, size: usize) -> [f64]",
        docs: "Samples the Chladni figure `cos(n pi x) cos(m pi y) + sign cos(m pi x) cos(n pi y)` of a free square plate at the cell centres `(i + 1/2) / size` of a row-major `size` by `size` raster, `x` along a row, scaled so the largest magnitude is 1, or all zero when no centre rises past `1e-12`; when `n == m` it is the single product for either sign.\n\n# Errors\n\nErrs when the sign is not 1 or -1, the size is 0 or over 1024, or a wave number is over 64.",
        uncallable: None,
        door: Some(door_num_chladni_plate),
    },
    Function {
        name: "num.chladni.shake",
        signature: "(field: [f64], side: usize, grains: [f32], from: u64, steps: usize, seed: u64) -> [f32]",
        docs: "Shakes sand on a plate raster: grains are `x, y` pairs in `[0, 1)` over the row-major `side` by `side` field, a NaN pair an empty slot that stays NaN, a NaN cell a hole. Each step moves a grain a random length `kick |f| + floor` in a random direction plus a drift `-0.3 f grad f / max|grad f|^2`, with `kick = 0.6 / max|grad f|` read off the raster by finite differences and a fixed kick on a flat field, the field read bilinearly with its holes and its rim ghosted to the mean of their plate neighbours. The rim reflects; a grain on a hole falls through and becomes NaN. Step `from + i` of grain `j` draws from a counter keyed by `(seed, from + i, j)`, so six steps are three and three.\n\n# Errors\n\nErrs when the side is 0, the field is not the side squared or holds an infinity, the grains hold an odd count or over 400,000 pairs, or the steps are over 64.",
        uncallable: None,
        door: Some(door_num_chladni_shake),
    },
    Function {
        name: "num.factor.aliquot",
        signature: "(number: usize) -> usize",
        docs: "Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.",
        uncallable: None,
        door: Some(door_num_factor_aliquot),
    },
    Function {
        name: "num.factor.coprime",
        signature: "(a: usize, b: usize) -> bool",
        docs: "Returns whether two numbers share no divisor above one.",
        uncallable: None,
        door: Some(door_num_factor_coprime),
    },
    Function {
        name: "num.factor.divisors",
        signature: "(number: u64) -> [u64]",
        docs: "Builds every divisor of a wide number from its factorization, ascending, empty for zero.\n\nThe width is the one the factorization needs, so a caller on the pointer width casts at its own call.",
        uncallable: None,
        door: Some(door_num_factor_divisors),
    },
    Function {
        name: "num.factor.factorial",
        signature: "(number: usize) -> u128",
        docs: "Returns the factorial of the number, the product of one through it, erring past thirty-four.",
        uncallable: None,
        door: Some(door_num_factor_factorial),
    },
    Function {
        name: "num.factor.factorize",
        signature: "(number: usize) -> [(usize, u32)]",
        docs: "Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_factor_factorize),
    },
    Function {
        name: "num.factor.factorize_wide",
        signature: "(number: u64) -> [(u64, u32)]",
        docs: "Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_factor_factorize_wide),
    },
    Function {
        name: "num.factor.gcd",
        signature: "(a: u128, b: u128) -> u128",
        docs: "Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.",
        uncallable: None,
        door: Some(door_num_factor_gcd),
    },
    Function {
        name: "num.factor.lcm",
        signature: "(a: usize, b: usize) -> usize",
        docs: "Returns the least common multiple of two numbers, zero when either side is zero.\n\nWraps silently once the multiple passes the pointer width.",
        uncallable: None,
        door: Some(door_num_factor_lcm),
    },
    Function {
        name: "num.factor.mobius",
        signature: "(number: usize) -> i8",
        docs: "Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.\n\nThe argument is the pointer width, so a wide value narrows on wasm32 above two to the thirty-two.",
        uncallable: None,
        door: Some(door_num_factor_mobius),
    },
    Function {
        name: "num.factor.mobius_sieve",
        signature: "(limit: usize) -> [i8]",
        docs: "Sieves the Mobius values of zero through the limit in one pass.",
        uncallable: None,
        door: Some(door_num_factor_mobius_sieve),
    },
    Function {
        name: "num.factor.radical",
        signature: "(number: usize) -> usize",
        docs: "Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.",
        uncallable: None,
        door: Some(door_num_factor_radical),
    },
    Function {
        name: "num.factor.reduce",
        signature: "(numerator: u128, denominator: u128) -> (u128, u128)",
        docs: "Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.",
        uncallable: None,
        door: Some(door_num_factor_reduce),
    },
    Function {
        name: "num.factor.sigma",
        signature: "(number: usize, power: u32) -> u128",
        docs: "Returns the sum of every divisor of the number raised to the power, so power zero counts them.\n\nStays exact for a power of two or below, above which the sum can wrap past a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_factor_sigma),
    },
    Function {
        name: "num.factor.squarefree",
        signature: "(number: usize) -> bool",
        docs: "Returns whether no prime squares into the number, true for one and false for zero.",
        uncallable: None,
        door: Some(door_num_factor_squarefree),
    },
    Function {
        name: "num.factor.totient",
        signature: "(number: usize) -> usize",
        docs: "Returns the Euler totient of the number from its factorization, zero for zero and one for one.",
        uncallable: None,
        door: Some(door_num_factor_totient),
    },
    Function {
        name: "num.factor.totients",
        signature: "(n: usize) -> [u64]",
        docs: "Sieves the Euler totients of zero through n in one pass, the run beside the single value.",
        uncallable: None,
        door: Some(door_num_factor_totients),
    },
    Function {
        name: "num.factor.twisted",
        signature: "(number: usize, rhythm: [i8]) -> i64",
        docs: "Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.",
        uncallable: None,
        door: Some(door_num_factor_twisted),
    },
    Function {
        name: "num.fft.convolve",
        signature: "(field: [f64], kernel: [f64], size: usize) -> [f64]",
        docs: "Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.\n\n# Errors\n\nErrs when the field or the kernel is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_convolve),
    },
    Function {
        name: "num.fft.convolve_with",
        signature: "(field: [f64], kernel_re: [f64], kernel_im: [f64], size: usize) -> [f64]",
        docs: "Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.",
        uncallable: None,
        door: Some(door_num_fft_convolve_with),
    },
    Function {
        name: "num.fft.embed_kernel",
        signature: "(mask: [u8], side: usize, size: usize) -> [f64]",
        docs: "Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.\n\n# Errors\n\nErrs at an even mask side, at a side past the field, and when the mask is not the side squared.",
        uncallable: None,
        door: Some(door_num_fft_embed_kernel),
    },
    Function {
        name: "num.fft.log_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_log_spectrum),
    },
    Function {
        name: "num.fft.magnitude_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_magnitude_spectrum),
    },
    Function {
        name: "num.fft.peak_ring",
        signature: "(profile: [f64]) -> usize",
        docs: "Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.",
        uncallable: None,
        door: Some(door_num_fft_peak_ring),
    },
    Function {
        name: "num.fft.peak_wavelength",
        signature: "(profile: [f64], size: usize) -> f64",
        docs: "Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.",
        uncallable: None,
        door: Some(door_num_fft_peak_wavelength),
    },
    Function {
        name: "num.fft.radial_profile",
        signature: "(spectrum: [f64], size: usize) -> [f64]",
        docs: "Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.\n\n# Errors\n\nErrs when the spectrum is not the size squared.",
        uncallable: None,
        door: Some(door_num_fft_radial_profile),
    },
    Function {
        name: "num.fft.transform",
        signature: "(field: [f64], size: usize) -> ([f64], [f64])",
        docs: "Transforms a real size-square field forward by fft2, returning the real and imaginary parts.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_transform),
    },
    Function {
        name: "num.gauss.Class.prime",
        signature: "(self: num.gauss.Class) -> bool",
        docs: "Returns whether the class is prime.",
        uncallable: None,
        door: Some(door_num_gauss_class_prime),
    },
    Function {
        name: "num.gauss.Class.word",
        signature: "(self: num.gauss.Class) -> String",
        docs: "Returns the class as a word.",
        uncallable: None,
        door: Some(door_num_gauss_class_word),
    },
    Function {
        name: "num.gauss.Ring.associates",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> [(i64, i64)]",
        docs: "Returns the unit multiples of a point, the point first, turning anticlockwise.",
        uncallable: None,
        door: Some(door_num_gauss_ring_associates),
    },
    Function {
        name: "num.gauss.Ring.canon",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin.",
        uncallable: None,
        door: Some(door_num_gauss_ring_canon),
    },
    Function {
        name: "num.gauss.Ring.conjugate",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the conjugate: the mirror image in the real axis.",
        uncallable: None,
        door: Some(door_num_gauss_ring_conjugate),
    },
    Function {
        name: "num.gauss.Ring.count",
        signature: "(self: num.gauss.Ring, radius: u64) -> usize",
        docs: "Returns the count of points within the reach: the square or the hexagon.",
        uncallable: None,
        door: Some(door_num_gauss_ring_count),
    },
    Function {
        name: "num.gauss.Ring.div_rem",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> ((i64, i64), (i64, i64))",
        docs: "Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.",
        uncallable: None,
        door: Some(door_num_gauss_ring_div_rem),
    },
    Function {
        name: "num.gauss.Ring.fate",
        signature: "(self: num.gauss.Ring, n: u64) -> num.gauss.Class",
        docs: "Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.",
        uncallable: None,
        door: Some(door_num_gauss_ring_fate),
    },
    Function {
        name: "num.gauss.Ring.gaussian_gcd",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> (i64, i64)",
        docs: "Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.",
        uncallable: None,
        door: Some(door_num_gauss_ring_gaussian_gcd),
    },
    Function {
        name: "num.gauss.Ring.inert",
        signature: "(self: num.gauss.Ring, p: u64) -> bool",
        docs: "Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.",
        uncallable: None,
        door: Some(door_num_gauss_ring_inert),
    },
    Function {
        name: "num.gauss.Ring.mul",
        signature: "(self: num.gauss.Ring, arg1: (i64, i64), arg2: (i64, i64)) -> (i64, i64)",
        docs: "Returns the product of two points.",
        uncallable: None,
        door: Some(door_num_gauss_ring_mul),
    },
    Function {
        name: "num.gauss.Ring.named",
        signature: "(name: str) -> num.gauss.Ring?",
        docs: "Reads a ring from its name.",
        uncallable: None,
        door: Some(door_num_gauss_ring_named),
    },
    Function {
        name: "num.gauss.Ring.nearest",
        signature: "(self: num.gauss.Ring, x: f64, y: f64) -> (i64, i64)",
        docs: "Returns the point nearest a place in the plane.",
        uncallable: None,
        door: Some(door_num_gauss_ring_nearest),
    },
    Function {
        name: "num.gauss.Ring.norm",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "Returns the norm of a point: its squared length.",
        uncallable: None,
        door: Some(door_num_gauss_ring_norm),
    },
    Function {
        name: "num.gauss.Ring.place",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (f64, f64)",
        docs: "Returns the place of a point in the plane, x right and y up, one unit between neighbours.",
        uncallable: None,
        door: Some(door_num_gauss_ring_place),
    },
    Function {
        name: "num.gauss.Ring.ramified",
        signature: "(self: num.gauss.Ring) -> u64",
        docs: "Returns the one rational prime that ramifies: 2 or 3.",
        uncallable: None,
        door: Some(door_num_gauss_ring_ramified),
    },
    Function {
        name: "num.gauss.Ring.reach",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.",
        uncallable: None,
        door: Some(door_num_gauss_ring_reach),
    },
    Function {
        name: "num.gauss.Ring.symmetry",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.",
        uncallable: None,
        door: Some(door_num_gauss_ring_symmetry),
    },
    Function {
        name: "num.gauss.Ring.top",
        signature: "(self: num.gauss.Ring, radius: u64) -> u64",
        docs: "Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.",
        uncallable: None,
        door: Some(door_num_gauss_ring_top),
    },
    Function {
        name: "num.gauss.Ring.turn",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.",
        uncallable: None,
        door: Some(door_num_gauss_ring_turn),
    },
    Function {
        name: "num.gauss.Ring.units",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "Returns the count of units: 4 or 6.",
        uncallable: None,
        door: Some(door_num_gauss_ring_units),
    },
    Function {
        name: "num.gauss.Ring.whole",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64?",
        docs: "Returns the whole number an associate of the point lies on, when one lies on the positive real axis.",
        uncallable: None,
        door: Some(door_num_gauss_ring_whole),
    },
    Function {
        name: "num.gauss.Window.census",
        signature: "(self: num.gauss.Window) -> num.gauss.Census",
        docs: "Counts every class inside.",
        uncallable: None,
        door: Some(door_num_gauss_window_census),
    },
    Function {
        name: "num.gauss.Window.class",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> num.gauss.Class",
        docs: "Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.",
        uncallable: None,
        door: Some(door_num_gauss_window_class),
    },
    Function {
        name: "num.gauss.Window.holds",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> bool",
        docs: "Returns whether a point lies inside.",
        uncallable: None,
        door: Some(door_num_gauss_window_holds),
    },
    Function {
        name: "num.gauss.Window.new",
        signature: "(ring: num.gauss.Ring, radius: u64) -> num.gauss.Window",
        docs: "Opens the window of a ring out to a reach, sieving every norm inside it.",
        uncallable: None,
        door: Some(door_num_gauss_window_new),
    },
    Function {
        name: "num.gauss.Window.points",
        signature: "(self: num.gauss.Window) -> [(i64, i64)]",
        docs: "Lists every point inside, row by row from the bottom left of the bounding square.",
        uncallable: None,
        door: Some(door_num_gauss_window_points),
    },
    Function {
        name: "num.gauss.Window.radius",
        signature: "(self: num.gauss.Window) -> u64",
        docs: "Returns the reach.",
        uncallable: None,
        door: Some(door_num_gauss_window_radius),
    },
    Function {
        name: "num.gauss.Window.ring",
        signature: "(self: num.gauss.Window) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_num_gauss_window_ring),
    },
    Function {
        name: "num.gauss.classes",
        signature: "(ring: num.gauss.Ring, bound: u64) -> [(i64, i64)]",
        docs: "Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.",
        uncallable: None,
        door: Some(door_num_gauss_classes),
    },
    Function {
        name: "num.gauss.peak",
        signature: "(ring: num.gauss.Ring, limit: usize) -> (usize, u32)",
        docs: "Returns the norm from one through the limit with the most points and that count, the earliest on a tie.",
        uncallable: None,
        door: Some(door_num_gauss_peak),
    },
    Function {
        name: "num.gauss.shells",
        signature: "(ring: num.gauss.Ring, limit: usize) -> [u32]",
        docs: "Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.",
        uncallable: None,
        door: Some(door_num_gauss_shells),
    },
    Function {
        name: "num.kummer.carry",
        signature: "(prime: u64, top: u64, bottom: u64) -> num.kummer.Carry",
        docs: "Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom.\n\nAt `(3, 1)` and `p = 7` state `0` keeps the digits `0..2` to itself and `4` to state `1`, and state `1` keeps `0, 1` to state `0` and `3, 4` to itself.\n\n# Errors\n\nErrs when `p` is not a prime, when `1 <= b < a <= MOST` fails, or when the table passes `TABLE` moves.",
        uncallable: None,
        door: Some(door_num_kummer_carry),
    },
    Function {
        name: "num.kummer.columns",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> [num.kummer.Column]",
        docs: "Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_columns),
    },
    Function {
        name: "num.kummer.count",
        signature: "(prime: u64, top: u64, bottom: u64, level: u32) -> u128",
        docs: "Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close.\n\nAt every prime `p > a` every state keeps `(p + 1)/2` digits and every state closes, so the count is `((p + 1)/2)^level`; at `p <= a` it can fail, as at `(3, 1)` and `p = 3`.\n\n# Errors\n\nErrs as `carry` does, and when `p^level` passes a `u128`.",
        uncallable: None,
        door: Some(door_num_kummer_count),
    },
    Function {
        name: "num.kummer.member",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> bool",
        docs: "Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_member),
    },
    Function {
        name: "num.kummer.valuation",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> u32",
        docs: "Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_valuation),
    },
    Function {
        name: "num.kummer.witness",
        signature: "(prime: u64, top: u64, bottom: u64) -> (u64, u64)?",
        docs: "Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`.\n\n# Errors\n\nErrs as `carry` does.",
        uncallable: None,
        door: Some(door_num_kummer_witness),
    },
    Function {
        name: "num.lattice.coprime_pairs",
        signature: "(n: usize) -> u64",
        docs: "Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.",
        uncallable: None,
        door: Some(door_num_lattice_coprime_pairs),
    },
    Function {
        name: "num.lattice.farey",
        signature: "(order: usize) -> [num.lattice.Node]",
        docs: "Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.",
        uncallable: None,
        door: Some(door_num_lattice_farey),
    },
    Function {
        name: "num.lattice.grid",
        signature: "(n: usize) -> [num.lattice.Node2d]",
        docs: "Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.",
        uncallable: None,
        door: Some(door_num_lattice_grid),
    },
    Function {
        name: "num.lattice.new_nodes",
        signature: "(n: usize) -> u64",
        docs: "Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.",
        uncallable: None,
        door: Some(door_num_lattice_new_nodes),
    },
    Function {
        name: "num.lattice.pi_estimate",
        signature: "(n: usize) -> f64",
        docs: "Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.",
        uncallable: None,
        door: Some(door_num_lattice_pi_estimate),
    },
    Function {
        name: "num.lattice.recovered",
        signature: "(n: usize, dimension: u32) -> f64",
        docs: "Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.\n\n# Errors\n\nErrs at a zero dimension.",
        uncallable: None,
        door: Some(door_num_lattice_recovered),
    },
    Function {
        name: "num.lattice.visible_density",
        signature: "(dimension: u32) -> f64",
        docs: "The density the visible count of a window in the dimension walks to, one over zeta of the dimension.\n\n# Errors\n\nErrs at a dimension of one or below.",
        uncallable: None,
        door: Some(door_num_lattice_visible_density),
    },
    Function {
        name: "num.lattice.zeta_factor",
        signature: "(dimension: u32) -> f64?",
        docs: "The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.",
        uncallable: None,
        door: Some(door_num_lattice_zeta_factor),
    },
    Function {
        name: "num.lattice.zeta_whole",
        signature: "(s: u32) -> f64",
        docs: "The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.\n\n# Errors\n\nErrs at a whole argument of one or below, where the sum does not converge.",
        uncallable: None,
        door: Some(door_num_lattice_zeta_whole),
    },
    Function {
        name: "num.morse.Lift.all",
        signature: "() -> [num.morse.Lift; 4]",
        docs: "Returns every Lift in canonical order.",
        uncallable: None,
        door: Some(door_num_morse_lift_all),
    },
    Function {
        name: "num.morse.Lift.at",
        signature: "(self: num.morse.Lift, i: u64, j: u64) -> u8",
        docs: "Returns the sign at a site, zero for plus one and one for minus one.",
        uncallable: None,
        door: Some(door_num_morse_lift_at),
    },
    Function {
        name: "num.morse.Lift.formula",
        signature: "(self: num.morse.Lift) -> String",
        docs: "Returns the lift's formula, written the way the page prints it.",
        uncallable: None,
        door: Some(door_num_morse_lift_formula),
    },
    Function {
        name: "num.morse.boundary",
        signature: "(word: [u8]) -> [u8]",
        docs: "Returns the run-boundary word, one wherever a letter differs from the next.",
        uncallable: None,
        door: Some(door_num_morse_boundary),
    },
    Function {
        name: "num.morse.difference",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "Exclusive-ors two grids of the same length, site by site.",
        uncallable: None,
        door: Some(door_num_morse_difference),
    },
    Function {
        name: "num.morse.digits",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the first letters of the Thue-Morse word by the digit rule.",
        uncallable: None,
        door: Some(door_num_morse_digits),
    },
    Function {
        name: "num.morse.doubling",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.",
        uncallable: None,
        door: Some(door_num_morse_doubling),
    },
    Function {
        name: "num.morse.faults",
        signature: "(a: [u8], b: [u8]) -> usize",
        docs: "Counts the sites where two grids of the same length differ.",
        uncallable: None,
        door: Some(door_num_morse_faults),
    },
    Function {
        name: "num.morse.fold",
        signature: "(grid: [u8], side: usize, number: usize) -> num.morse.Fold",
        docs: "Tests a grid against the Kronecker power of its own corner tile.\n\nThe corner tile is the only candidate worth testing. If the grid is `T` folded `L` times then\nits corner block is `T` with every bit flipped by `(L - 1) t00`, and folding that block `L`\ntimes flips the grid by `L (L - 1) t00`, which is even. So a grid folds if and only if it\nfolds from its corner block, and no search over tiles is needed.\n\n# Errors\n\nErrs when the grid is not the side squared, below a tile side of two, and when the side is no power of the tile side.",
        uncallable: None,
        door: Some(door_num_morse_fold),
    },
    Function {
        name: "num.morse.letter",
        signature: "(place: u64) -> u8",
        docs: "Returns the Thue-Morse letter at the place, the parity of its binary digit sum.",
        uncallable: None,
        door: Some(door_num_morse_letter),
    },
    Function {
        name: "num.morse.lift",
        signature: "(kind: num.morse.Lift, side: usize) -> [u8]",
        docs: "Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.",
        uncallable: None,
        door: Some(door_num_morse_lift),
    },
    Function {
        name: "num.morse.mask",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "Keeps the sites lit in both grids of the same length, site by site.",
        uncallable: None,
        door: Some(door_num_morse_mask),
    },
    Function {
        name: "num.morse.power",
        signature: "(tile: [u8], number: usize, level: usize) -> [u8]",
        docs: "Folds a tile of the side into its Kronecker power at the level, one bit per site.\n\nThe bits ride the exclusive or, so the power is the digit rule of the tile: a site takes the\nexclusive or of the tile's bits at its base-side digit pairs.\n\n# Errors\n\nErrs when the tile is not the side squared, or when the level passes what a machine holds.",
        uncallable: None,
        door: Some(door_num_morse_power),
    },
    Function {
        name: "num.morse.repeat",
        signature: "(tile: [u8], number: usize, side: usize) -> [u8]",
        docs: "Repeats a tile until it fills a grid of the side.",
        uncallable: None,
        door: Some(door_num_morse_repeat),
    },
    Function {
        name: "num.morse.runs",
        signature: "(word: [u8]) -> [usize]",
        docs: "Returns the lengths of the maximal blocks of one repeated letter, in order.",
        uncallable: None,
        door: Some(door_num_morse_runs),
    },
    Function {
        name: "num.morse.signs",
        signature: "(types: [u8]) -> [u8]",
        docs: "Reads a design plus-minus: a filled cell is plus, zero, and a void is minus, one.",
        uncallable: None,
        door: Some(door_num_morse_signs),
    },
    Function {
        name: "num.morse.stage",
        signature: "(rounds: usize) -> [u8]",
        docs: "Returns the substitution stage after the rounds, a word of length two to the rounds.",
        uncallable: None,
        door: Some(door_num_morse_stage),
    },
    Function {
        name: "num.morse.substitution",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.\n\nThe seed is the single letter 0, and the rounds double the length until it covers the ask.",
        uncallable: None,
        door: Some(door_num_morse_substitution),
    },
    Function {
        name: "num.morse.upsample",
        signature: "(grid: [u8], side: usize, scale: usize) -> [u8]",
        docs: "Blows a grid up by the scale, every site becoming a scale-by-scale block.",
        uncallable: None,
        door: Some(door_num_morse_upsample),
    },
    Function {
        name: "num.prime.Sieve.count",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of numbers marked prime so far.",
        uncallable: None,
        door: Some(door_num_prime_sieve_count),
    },
    Function {
        name: "num.prime.Sieve.done",
        signature: "(self: num.prime.Sieve) -> bool",
        docs: "Returns whether every number is settled.",
        uncallable: None,
        door: Some(door_num_prime_sieve_done),
    },
    Function {
        name: "num.prime.Sieve.finish",
        signature: "(self: num.prime.Sieve) -> null",
        docs: "Runs the sieve to the end.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.new",
        signature: "(limit: usize) -> num.prime.Sieve",
        docs: "Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.",
        uncallable: None,
        door: Some(door_num_prime_sieve_new),
    },
    Function {
        name: "num.prime.Sieve.rank",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of primes used so far.",
        uncallable: None,
        door: Some(door_num_prime_sieve_rank),
    },
    Function {
        name: "num.prime.Sieve.step",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.\n\nThe strike mark saturates at 255, so it is exact through the 254th prime.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.struck",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of numbers the last step struck.",
        uncallable: None,
        door: Some(door_num_prime_sieve_struck),
    },
    Function {
        name: "num.prime.Sieve.types",
        signature: "(self: num.prime.Sieve) -> [u8]",
        docs: "Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.",
        uncallable: None,
        door: Some(door_num_prime_sieve_types),
    },
    Function {
        name: "num.prime.chart",
        signature: "(top: usize, bins: usize) -> [num.prime.Reading]",
        docs: "Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.",
        uncallable: None,
        door: Some(door_num_prime_chart),
    },
    Function {
        name: "num.prime.flags",
        signature: "(limit: usize) -> [bool]",
        docs: "Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.\n\nThe flag is the prime mark, never the strike rank, so it is exact at any limit.",
        uncallable: None,
        door: Some(door_num_prime_flags),
    },
    Function {
        name: "num.prime.goldbach",
        signature: "(number: usize) -> usize",
        docs: "Returns the count of unordered pairs of primes summing to the number, zero below four.",
        uncallable: None,
        door: Some(door_num_prime_goldbach),
    },
    Function {
        name: "num.prime.goldbach_record",
        signature: "(top: usize) -> [usize]",
        docs: "Returns the count of prime pairs at every even number from four up to the top, one entry per even number.",
        uncallable: None,
        door: Some(door_num_prime_goldbach_record),
    },
    Function {
        name: "num.prime.is_prime",
        signature: "(number: usize) -> bool",
        docs: "Returns whether the number is prime, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_prime_is_prime),
    },
    Function {
        name: "num.prime.pile",
        signature: "(number: u64) -> num.prime.Pile",
        docs: "Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.",
        uncallable: None,
        door: Some(door_num_prime_pile),
    },
    Function {
        name: "num.prime.prime_count",
        signature: "(n: usize) -> usize",
        docs: "Returns the count of primes at or below n.",
        uncallable: None,
        door: Some(door_num_prime_prime_count),
    },
    Function {
        name: "num.prime.prime_from",
        signature: "(number: usize) -> usize",
        docs: "Returns the smallest prime at or above the number.",
        uncallable: None,
        door: Some(door_num_prime_prime_from),
    },
    Function {
        name: "num.prime.primes",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the primes up to the limit, the finished sieve read as a list.",
        uncallable: None,
        door: Some(door_num_prime_primes),
    },
    Function {
        name: "num.prime.rectangles",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.",
        uncallable: None,
        door: Some(door_num_prime_rectangles),
    },
    Function {
        name: "num.prime.splits",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.",
        uncallable: None,
        door: Some(door_num_prime_splits),
    },
    Function {
        name: "num.prime.squares",
        signature: "(number: usize) -> (usize, usize)?",
        docs: "Returns the smallest pair of positive sides whose squares sum to the number, when one exists.",
        uncallable: None,
        door: Some(door_num_prime_squares),
    },
    Function {
        name: "num.prime.study",
        signature: "(limit: usize) -> [num.prime.Prime]",
        docs: "Returns one prime object for every prime up to and including the limit.",
        uncallable: None,
        door: Some(door_num_prime_study),
    },
    Function {
        name: "num.readings.Design.digits",
        signature: "(base: u8, mask: u8) -> num.readings.Design",
        docs: "Builds the design `b:D` from its base and its digit mask.",
        uncallable: None,
        door: Some(door_num_readings_design_digits),
    },
    Function {
        name: "num.readings.Design.rudin",
        signature: "() -> num.readings.Design",
        docs: "Returns Rudin-Shapiro.",
        uncallable: None,
        door: Some(door_num_readings_design_rudin),
    },
    Function {
        name: "num.readings.Design.ternary",
        signature: "() -> num.readings.Design",
        docs: "Returns the ternary parity, `3:2`, the parity of the ternary digits equal to 2.",
        uncallable: None,
        door: Some(door_num_readings_design_ternary),
    },
    Function {
        name: "num.readings.Design.thue",
        signature: "() -> num.readings.Design",
        docs: "Returns Thue-Morse, `2:1`.",
        uncallable: None,
        door: Some(door_num_readings_design_thue),
    },
    Function {
        name: "num.readings.Reading.all",
        signature: "() -> [num.readings.Reading; 5]",
        docs: "Returns every Reading in canonical order.",
        uncallable: None,
        door: Some(door_num_readings_reading_all),
    },
    Function {
        name: "num.readings.Reading.formula",
        signature: "(self: num.readings.Reading) -> String",
        docs: "Returns the reading's formula, written the way the page prints it.",
        uncallable: None,
        door: Some(door_num_readings_reading_formula),
    },
    Function {
        name: "num.readings.arcs",
        signature: "(design: num.readings.Design, cut: f64) -> [num.readings.Arc]",
        docs: "Lists a design's major arcs along the squares below the cut, best first, each with its amplitude.\n\nThe cut is held under `1/2`, where the trees stop converging; the lab's model keeps the arcs below `1/4`. An arc whose Gauss sum vanishes keeps a zero amplitude. Rudin-Shapiro, which is no digit design, has none. For base 6 the limit `K` is a series over the 2-adic and 3-adic valuations, cut at 14 terms.",
        uncallable: None,
        door: Some(door_num_readings_arcs),
    },
    Function {
        name: "num.readings.battery",
        signature: "(stream: [i8]) -> num.readings.Battery",
        docs: "Runs the battery on a stream of signs.\n\nEvery statistic runs over the same `N = min(floor(len / 9), len - 64)` values of `n`, the largest that keeps every pair inside the stream, so a stream of `9N` signs feeds a battery at `N`; a stream below 72 signs gives `N = 0` and zeros. A pair whose sides have no variance reads zero.",
        uncallable: None,
        door: Some(door_num_readings_battery),
    },
    Function {
        name: "num.readings.bias",
        signature: "(stream: [i8], exponents: [u32]) -> [i64]",
        docs: "Measures the bias `S(N) = sum_(n<N) y(n)` of a stream at `N = 2^e` for each exponent; a checkpoint past the stream's end sums the whole stream.",
        uncallable: None,
        door: Some(door_num_readings_bias),
    },
    Function {
        name: "num.readings.designs",
        signature: "() -> [num.readings.Design]",
        docs: "Lists the 57 digit designs, base by base and mask by mask, then Rudin-Shapiro.",
        uncallable: None,
        door: Some(door_num_readings_designs),
    },
    Function {
        name: "num.readings.model",
        signature: "(design: num.readings.Design, cut: f64, exponents: [u32]) -> [f64]",
        docs: "Predicts the bias `S(N) = sum_(n<N) x(n^2)` at `N = 2^e` for each exponent, from the arcs below the cut: `1 + Re sum_gamma A(gamma) U(gamma; N^2)`.\n\nExponents above 63 are read as 63. Rudin-Shapiro has no arcs and reads the constant one.",
        uncallable: None,
        door: Some(door_num_readings_model),
    },
    Function {
        name: "num.readings.name",
        signature: "(design: num.readings.Design) -> String",
        docs: "Returns the design's name in the `b:D` form, the digits of `D` ascending, or `rudin-shapiro`.",
        uncallable: None,
        door: Some(door_num_readings_name),
    },
    Function {
        name: "num.readings.near_heads",
        signature: "(design: num.readings.Design, reading: num.readings.Reading) -> [num.readings.Head]",
        docs: "Lists the battery pairs that are near-heads of a polynomial reading at the design's base, the 44 dilation pairs first and the two kernel pairs after; the primes have none.",
        uncallable: None,
        door: Some(door_num_readings_near_heads),
    },
    Function {
        name: "num.readings.sign",
        signature: "(design: num.readings.Design, n: u64) -> i8",
        docs: "Returns the design's sign at a number, plus or minus one.",
        uncallable: None,
        door: Some(door_num_readings_sign),
    },
    Function {
        name: "num.readings.stream",
        signature: "(design: num.readings.Design, reading: num.readings.Reading, count: u32) -> [i8]",
        docs: "Builds a reading's first signs: `y(n) = x(P(n))` for `n < count`, or `x(p_n)` along the first `count` primes from a segmented sieve.",
        uncallable: None,
        door: Some(door_num_readings_stream),
    },
    Function {
        name: "num.series.basel",
        signature: "(n: usize) -> f64",
        docs: "Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.",
        uncallable: None,
        door: Some(door_num_series_basel),
    },
    Function {
        name: "num.series.bernoulli",
        signature: "(count: usize) -> [(i128, i128)]",
        docs: "Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.\n\n# Errors\n\nErrs past a count of thirty-two, where the fractions overrun a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_series_bernoulli),
    },
    Function {
        name: "num.series.beta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.",
        uncallable: None,
        door: Some(door_num_series_beta),
    },
    Function {
        name: "num.series.binary",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the powers of two up to the limit.",
        uncallable: None,
        door: Some(door_num_series_binary),
    },
    Function {
        name: "num.series.catalan",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the distinct Catalan numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_catalan),
    },
    Function {
        name: "num.series.chi3",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-three rhythm of the number: zero, one, minus one.",
        uncallable: None,
        door: Some(door_num_series_chi3),
    },
    Function {
        name: "num.series.chi4",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-four rhythm of the number: zero, one, zero, minus one.",
        uncallable: None,
        door: Some(door_num_series_chi4),
    },
    Function {
        name: "num.series.chi8",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.",
        uncallable: None,
        door: Some(door_num_series_chi8),
    },
    Function {
        name: "num.series.dirichlet",
        signature: "(s: f64, rhythm: [i8], terms: usize) -> f64",
        docs: "Returns the L-series partial sum with a periodic rhythm painted on the terms.",
        uncallable: None,
        door: Some(door_num_series_dirichlet),
    },
    Function {
        name: "num.series.e_partial",
        signature: "(n: usize) -> f64",
        docs: "Returns one plus one over n raised to the n, walking to the natural base.",
        uncallable: None,
        door: Some(door_num_series_e_partial),
    },
    Function {
        name: "num.series.euler_gamma_partial",
        signature: "(n: usize) -> f64",
        docs: "Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.",
        uncallable: None,
        door: Some(door_num_series_euler_gamma_partial),
    },
    Function {
        name: "num.series.euler_product",
        signature: "(s: f64, limit: usize) -> f64",
        docs: "Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.",
        uncallable: None,
        door: Some(door_num_series_euler_product),
    },
    Function {
        name: "num.series.evens",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the even numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_evens),
    },
    Function {
        name: "num.series.fibonacci",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the distinct Fibonacci numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_fibonacci),
    },
    Function {
        name: "num.series.harmonic",
        signature: "(terms: usize) -> f64",
        docs: "Returns the partial harmonic sum, the reciprocals of one through the term count.",
        uncallable: None,
        door: Some(door_num_series_harmonic),
    },
    Function {
        name: "num.series.lambda",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the Dirichlet lambda value, one minus two to the minus s times zeta.\n\n# Errors\n\nErrs at an s of one or below, and at a NaN, where zeta does not converge.",
        uncallable: None,
        door: Some(door_num_series_lambda),
    },
    Function {
        name: "num.series.leibniz",
        signature: "(n: usize) -> f64",
        docs: "Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.",
        uncallable: None,
        door: Some(door_num_series_leibniz),
    },
    Function {
        name: "num.series.li",
        signature: "(x: f64) -> f64",
        docs: "Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.",
        uncallable: None,
        door: Some(door_num_series_li),
    },
    Function {
        name: "num.series.mertens",
        signature: "(n: usize) -> i64",
        docs: "Returns the Mertens function at n, the Mobius values of one through n summed.",
        uncallable: None,
        door: Some(door_num_series_mertens),
    },
    Function {
        name: "num.series.odds",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the odd numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_odds),
    },
    Function {
        name: "num.series.visible",
        signature: "(limit: usize, dimension: u32) -> u128",
        docs: "Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.\n\nThe count wraps once the limit to the dimension passes a signed hundred and twenty-eight bits.\n\n# Errors\n\nErrs at a zero dimension.",
        uncallable: None,
        door: Some(door_num_series_visible),
    },
    Function {
        name: "num.series.wallis_half_pi",
        signature: "(n: usize) -> f64",
        docs: "Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.",
        uncallable: None,
        door: Some(door_num_series_wallis_half_pi),
    },
    Function {
        name: "num.series.wallis_quarter_pi",
        signature: "(factors: usize) -> f64",
        docs: "Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.",
        uncallable: None,
        door: Some(door_num_series_wallis_quarter_pi),
    },
    Function {
        name: "num.series.zeta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.\n\n# Errors\n\nErrs at an s of one or below, and at a NaN, where the sum does not converge.",
        uncallable: None,
        door: Some(door_num_series_zeta),
    },
    Function {
        name: "num.sieve.cells",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the product overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_cells),
    },
    Function {
        name: "num.sieve.exponent",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.\n\nChanging the letter is not enough: the alternating word 3, 5, 3, 5 changes at every step and freezes at log 192 / log 15, its ratio falling to nothing like a fixed-ratio schedule. The hypothesis that buys a positive area is strictly increasing odd letters, under which sum s_k^(-d) converges.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_exponent),
    },
    Function {
        name: "num.sieve.flat_word",
        signature: "(side: u64, levels: usize) -> [u64]",
        docs: "Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.",
        uncallable: None,
        door: Some(door_num_sieve_flat_word),
    },
    Function {
        name: "num.sieve.holes",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "Returns the punctures the word makes, one per surviving cell at every level.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the count overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_holes),
    },
    Function {
        name: "num.sieve.limit",
        signature: "(word: [u64], dimension: u32) -> f64?",
        docs: "Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.\n\nA run of consecutive odd letters from a, of two letters or more, names the schedule a, a + 2, a + 4 and on: its letters are strictly increasing, sum s_k^(-d) converges, and the limit is positive, pi over four in the plane and the closed form above in the cube divided by the head the run skips. Two or more copies of one letter name the fixed-ratio schedule, whose limit is zero; that is what repetition costs, and an alternating word such as 3, 5, 3, 5 changes at every step and still loses the whole measure. Every other word, and every word of one letter, names no schedule and reads None.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_limit),
    },
    Function {
        name: "num.sieve.odd_word",
        signature: "(levels: usize) -> [u64]",
        docs: "Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.",
        uncallable: None,
        door: Some(door_num_sieve_odd_word),
    },
    Function {
        name: "num.sieve.parity_product",
        signature: "(dimension: u32, odd: bool) -> f64",
        docs: "Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.\n\nAt odd `N` the sum of `(-1)^(x_1 + ... + x_dim)` over the box is 1, so the two designs fill `(N^dim -+ 1)/2` and the row word's constant is this product. The logarithm is summed over odd `N` from 3 to 999, smallest term first, and the tail past 1001 is the series `sum_j (-+1)^j / j sum N^(-dim j)`, each inner sum by Euler-Maclaurin through the fourth Bernoulli term, as for the solid limit.\n\n# Errors\n\nErrs below two dimensions, where the product runs off to 0 or to infinity.",
        uncallable: None,
        door: Some(door_num_sieve_parity_product),
    },
    Function {
        name: "num.sieve.punctures",
        signature: "(word: [u64], dimension: u32) -> [u64]",
        docs: "Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the word's side overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_punctures),
    },
    Function {
        name: "num.sieve.raster",
        signature: "(word: [u64]) -> (usize, [u8])",
        docs: "Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_raster),
    },
    Function {
        name: "num.sieve.ratio",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_ratio),
    },
    Function {
        name: "num.sieve.row_fill",
        signature: "(profile: [u64], side: u64) -> u128",
        docs: "Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds, or when the fill overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_row_fill),
    },
    Function {
        name: "num.sieve.row_law",
        signature: "(profile: [u64]) -> num.sieve.Row",
        docs: "Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply.\n\nThe fill polynomial factors as `w n^(m_0) (n-1)^(m_1)` times a core with integer coefficients; its rational roots are found exactly and divided out, and what is left is a constant, an irreducible quadratic or, at dim 3, an irreducible cubic, whose roots are taken in floating point. Each `Gamma(2 - r)` is read against one `Gamma(3/2)` as a ratio, by the recurrence up past 16 and Stirling's series there, so the large terms cancel before they round.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds.",
        uncallable: None,
        door: Some(door_num_sieve_row_law),
    },
    Function {
        name: "num.sieve.row_profile",
        signature: "(code: u128, dimension: usize) -> [u64]",
        docs: "Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`.\n\n# Errors\n\nErrs outside dim 1 to 3, or on a code out of range for the dimension.",
        uncallable: None,
        door: Some(door_num_sieve_row_profile),
    },
    Function {
        name: "num.sieve.row_settle",
        signature: "(profile: [u64], stops: [usize], even: bool) -> [f64]",
        docs: "Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`.\n\nEach letter adds `log(1 + (2^dim fill - w side^dim) / (w side^dim))`, the excess taken in exact integers; at an even side the excess is zero and the even word carries no drift, so its walk stands at 1.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds, or on stops that are not rising whole numbers from 1.",
        uncallable: None,
        door: Some(door_num_sieve_row_settle),
    },
    Function {
        name: "num.sieve.side",
        signature: "(word: [u64]) -> u128",
        docs: "Returns the side of the word, the product of its letters' sides.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the product overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_side),
    },
    Function {
        name: "num.sieve.solid_limit",
        signature: "() -> f64",
        docs: "Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.\n\nWrite m = 2k + 1 and factor m^3 - 1 = (m - 1)(m - w)(m - w^2) at w = exp(2 pi i / 3), the three cube roots of one; dividing by m^3 = 8 (k + 1/2)^3 turns the k-th factor into k (k + (1 - w)/2) (k + (1 - w^2)/2) / (k + 1/2)^3, so the product is a ratio of three rising shifts over one repeated three times.\n\nThe three shifts a1 = 0, a2 = (1 - w)/2 and a3 = (1 - w^2)/2 sum to (2 - w - w^2)/2 = 3/2, exactly three times the shift b = 1/2 below, and prod_{k=1..N} (k + a) = Gamma(N + 1 + a) / Gamma(1 + a) with Gamma(N + 1 + a) / Gamma(N + 1 + b) ~ N^(a - b), so the N-dependent factor tends to N^(a1 + a2 + a3 - 3b) = 1 and the limit is Gamma(1 + b)^3 / (Gamma(1 + a1) Gamma(1 + a2) Gamma(1 + a3)).\n\nThat is Gamma(3/2)^3 / (Gamma(1) Gamma((3 - w)/2) Gamma((3 - w^2)/2)) = pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2), the two gamma values being conjugates; the same Weierstrass product gives the identity prod_{k >= 1} (1 - a^3/k^3) = 1 / (Gamma(1 - a) Gamma(1 - a w) Gamma(1 - a w^2)) the ratio form of this limit leans on, at a = 1/2 against prod_{n >= 2} (1 - n^-3) = cosh(pi sqrt(3)/2) / (3 pi).\n\nThe value is read from the logarithm rather than from that gamma ratio, which cancels four digits away: log P = sum over odd n from 3 to 999 of log(1 - n^-3), taken from its smallest term, minus sum_{j = 1..3} (1/j) sum_{n odd >= 1001} n^(-3j), each inner sum by Euler-Maclaurin through the fourth Bernoulli term. Every piece carries one sign, so nothing cancels and the exponential lands within a few ulps.",
        uncallable: None,
        door: Some(door_num_sieve_solid_limit),
    },
    Function {
        name: "num.spiral.Growth.all",
        signature: "() -> [num.spiral.Growth; 2]",
        docs: "Returns every Growth in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_growth_all),
    },
    Function {
        name: "num.spiral.Lattice.all",
        signature: "() -> [num.spiral.Lattice; 2]",
        docs: "Returns every Lattice in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_all),
    },
    Function {
        name: "num.spiral.Lattice.count",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_count),
    },
    Function {
        name: "num.spiral.Lattice.n",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "Returns the number at a cell, one at the origin.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_n),
    },
    Function {
        name: "num.spiral.Lattice.radius",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "Returns the outermost ring of a sheet the odd side wide, half the side rounded down.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_radius),
    },
    Function {
        name: "num.spiral.Lattice.ring",
        signature: "(self: num.spiral.Lattice, n: u64) -> u64",
        docs: "Returns the ring a number sits on, zero for one.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring),
    },
    Function {
        name: "num.spiral.Lattice.ring_of",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring_of),
    },
    Function {
        name: "num.spiral.Lattice.xy",
        signature: "(self: num.spiral.Lattice, n: u64) -> (i64, i64)",
        docs: "Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_xy),
    },
    Function {
        name: "num.spiral.Mark.all",
        signature: "() -> [num.spiral.Mark; 4]",
        docs: "Returns every Mark in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_mark_all),
    },
    Function {
        name: "num.spiral.diagonal",
        signature: "(lattice: num.spiral.Lattice, side: usize, a: i64, b: i64, c: i64) -> num.spiral.Diagonal",
        docs: "Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.",
        uncallable: None,
        door: Some(door_num_spiral_diagonal),
    },
    Function {
        name: "num.spiral.level_of",
        signature: "(n: u64, base: u64) -> u32",
        docs: "Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.",
        uncallable: None,
        door: Some(door_num_spiral_level_of),
    },
    Function {
        name: "num.spiral.marks",
        signature: "(mark: num.spiral.Mark, limit: usize) -> [i8]",
        docs: "Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.",
        uncallable: None,
        door: Some(door_num_spiral_marks),
    },
    Function {
        name: "num.spiral.snail",
        signature: "(base: u64, top: u64, growth: num.spiral.Growth) -> num.spiral.Snail",
        docs: "Winds one to the top on the square spiral and lays a square tile on every cell, the snail.\n\nThe tile of n has side base to the level of n when n grows and side one when it does not, and its lower-left corner is the corner of the tile before it plus one unit step of the winding scaled by the side of that earlier tile. Tiles overlap wherever the growth outruns the winding; a base below two is read as two and a top below one as one.",
        uncallable: None,
        door: Some(door_num_spiral_snail),
    },
    Function {
        name: "num.zeta.Complex.abs",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "Returns the modulus.",
        uncallable: None,
        door: Some(door_num_zeta_complex_abs),
    },
    Function {
        name: "num.zeta.Complex.arg",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "Returns the principal argument.",
        uncallable: None,
        door: Some(door_num_zeta_complex_arg),
    },
    Function {
        name: "num.zeta.Complex.default",
        signature: "() -> num.zeta.Complex",
        docs: "Returns the default Complex.",
        uncallable: None,
        door: Some(door_num_zeta_complex_default),
    },
    Function {
        name: "num.zeta.Complex.exp",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the exponential.",
        uncallable: None,
        door: Some(door_num_zeta_complex_exp),
    },
    Function {
        name: "num.zeta.Complex.ln",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the principal logarithm.",
        uncallable: None,
        door: Some(door_num_zeta_complex_ln),
    },
    Function {
        name: "num.zeta.Complex.new",
        signature: "(re: f64, im: f64) -> num.zeta.Complex",
        docs: "Builds a complex number from its parts.",
        uncallable: None,
        door: Some(door_num_zeta_complex_new),
    },
    Function {
        name: "num.zeta.Complex.turn",
        signature: "(angle: f64) -> num.zeta.Complex",
        docs: "Returns a unit complex number at the given angle.",
        uncallable: None,
        door: Some(door_num_zeta_complex_turn),
    },
    Function {
        name: "num.zeta.Line.count",
        signature: "(self: num.zeta.Line, t: f64) -> usize",
        docs: "Counts the zeros on the line below t: the sign changes of Z on ten steps a Gram interval, the Riemann-Siegel main sum turned from step to step and its remainder read only where it could flip the sign.",
        uncallable: None,
        door: Some(door_num_zeta_line_count),
    },
    Function {
        name: "num.zeta.Line.default",
        signature: "() -> num.zeta.Line",
        docs: "Returns the default Line.",
        uncallable: None,
        door: Some(door_num_zeta_line_default),
    },
    Function {
        name: "num.zeta.Line.exact",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.",
        uncallable: None,
        door: Some(door_num_zeta_line_exact),
    },
    Function {
        name: "num.zeta.Line.gram",
        signature: "(self: num.zeta.Line, n: i64) -> f64",
        docs: "Returns the n-th Gram point, where theta is n pi, by Newton from the right.",
        uncallable: None,
        door: Some(door_num_zeta_line_gram),
    },
    Function {
        name: "num.zeta.Line.maclaurin",
        signature: "(self: num.zeta.Line, t: f64) -> num.zeta.Complex",
        docs: "Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.",
        uncallable: None,
        door: Some(door_num_zeta_line_maclaurin),
    },
    Function {
        name: "num.zeta.Line.new",
        signature: "() -> num.zeta.Line",
        docs: "Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.",
        uncallable: None,
        door: Some(door_num_zeta_line_new),
    },
    Function {
        name: "num.zeta.Line.novelty_coefficients",
        signature: "(self: num.zeta.Line, gammas: [f64]) -> [num.zeta.Complex]",
        docs: "Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.",
        uncallable: None,
        door: Some(door_num_zeta_line_novelty_coefficients),
    },
    Function {
        name: "num.zeta.Line.pair",
        signature: "(self: num.zeta.Line, s: num.zeta.Complex) -> (num.zeta.Complex, num.zeta.Complex)",
        docs: "Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.",
        uncallable: None,
        door: Some(door_num_zeta_line_pair),
    },
    Function {
        name: "num.zeta.Line.point",
        signature: "(self: num.zeta.Line, t: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns zeta on the line and Z(t) together, from the engine that serves the t.",
        uncallable: None,
        door: Some(door_num_zeta_line_point),
    },
    Function {
        name: "num.zeta.Line.seam",
        signature: "(self: num.zeta.Line, t0: f64, t1: f64, steps: usize) -> f64",
        docs: "Returns the largest gap between the two engines over the t range on a grid.",
        uncallable: None,
        door: Some(door_num_zeta_line_seam),
    },
    Function {
        name: "num.zeta.Line.siegel",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.",
        uncallable: None,
        door: Some(door_num_zeta_line_siegel),
    },
    Function {
        name: "num.zeta.Line.theta",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.",
        uncallable: None,
        door: Some(door_num_zeta_line_theta),
    },
    Function {
        name: "num.zeta.Line.z",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.",
        uncallable: None,
        door: Some(door_num_zeta_line_z),
    },
    Function {
        name: "num.zeta.Line.zeros",
        signature: "(self: num.zeta.Line, count: usize) -> [f64]",
        docs: "Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.",
        uncallable: None,
        door: Some(door_num_zeta_line_zeros),
    },
    Function {
        name: "num.zeta.bump",
        signature: "(u: f64) -> f64",
        docs: "The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.",
        uncallable: None,
        door: Some(door_num_zeta_bump),
    },
    Function {
        name: "num.zeta.corrections",
        signature: "(p: f64) -> [f64; 4]",
        docs: "Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.",
        uncallable: None,
        door: Some(door_num_zeta_corrections),
    },
    Function {
        name: "num.zeta.kernel",
        signature: "(p: f64) -> f64",
        docs: "Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.",
        uncallable: None,
        door: Some(door_num_zeta_kernel),
    },
    Function {
        name: "num.zeta.mellin",
        signature: "(s: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.",
        uncallable: None,
        door: Some(door_num_zeta_mellin),
    },
    Function {
        name: "num.zeta.novelty_main",
        signature: "() -> f64",
        docs: "Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.",
        uncallable: None,
        door: Some(door_num_zeta_novelty_main),
    },
    Function {
        name: "num.zeta.novelty_wave",
        signature: "(gammas: [f64], coef: [num.zeta.Complex], log_y: f64) -> f64",
        docs: "Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.",
        uncallable: None,
        door: Some(door_num_zeta_novelty_wave),
    },
    Function {
        name: "num.zeta.psi_curve",
        signature: "(gammas: [f64], samples: [f64]) -> [f64]",
        docs: "Returns the explicit formula over the zeros at the given ordinates at every sample x.",
        uncallable: None,
        door: Some(door_num_zeta_psi_curve),
    },
    Function {
        name: "num.zeta.psi_formula",
        signature: "(x: f64, gammas: [f64]) -> f64",
        docs: "Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.",
        uncallable: None,
        door: Some(door_num_zeta_psi_formula),
    },
    Function {
        name: "num.zeta.psi_gap",
        signature: "(x: usize, gammas: [f64]) -> f64",
        docs: "Returns how far the explicit formula over the zeros at the given ordinates lands from the staircase at the whole number x.",
        uncallable: None,
        door: Some(door_num_zeta_psi_gap),
    },
    Function {
        name: "num.zeta.psi_stair",
        signature: "(x: usize) -> [f64]",
        docs: "Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.",
        uncallable: None,
        door: Some(door_num_zeta_psi_stair),
    },
    Function {
        name: "num.zeta.raise",
        signature: "(base: f64, exponent: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns a positive real base raised to a complex exponent.",
        uncallable: None,
        door: Some(door_num_zeta_raise),
    },
    Function {
        name: "num.zeta.sharp_novelty",
        signature: "(prefix: [u64], y: f64) -> f64",
        docs: "Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.",
        uncallable: None,
        door: Some(door_num_zeta_sharp_novelty),
    },
    Function {
        name: "num.zeta.smoothed_novelty",
        signature: "(phi: [u64], y: f64, main: f64) -> f64",
        docs: "Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.",
        uncallable: None,
        door: Some(door_num_zeta_smoothed_novelty),
    },
];

fn door_num_apollonian_circle_centre(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::centre(a0)))
}

fn door_num_apollonian_circle_is_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::is_line(a0)))
}

fn door_num_apollonian_circle_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::Circle::radius(a0)))
}

fn door_num_apollonian_flat(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_num::apollonian::flat(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_form(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [i64; 4] = take!(name, 0, &args[0]);
    let a1: [i64; 4] = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly_num::apollonian::form(a0, a1).to_string(),
    ))
}

fn door_num_apollonian_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::frame(&a0)))
}

fn door_num_apollonian_grow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly_num::apollonian::grow(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_is_ford(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::is_ford(a0)))
}

fn door_num_apollonian_on_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::on_line(a0)))
}

fn door_num_apollonian_reflect(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::apollonian::reflect(&a0, a1)))
}

fn door_num_apollonian_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_num::apollonian::root(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::apollonian::shadow(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_sound(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::sound(&a0)))
}

fn door_num_apollonian_swap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly_num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::apollonian::swap(&a0, a1)))
}

fn door_num_apollonian_touches(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::apollonian::touches(&a0)))
}

fn door_num_blend_add(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::add(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_cauchy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    match mrly_num::blend::cauchy(&a0, &a1) {
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

fn door_num_blend_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::characteristic(&a0) {
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

fn door_num_blend_decimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::blend::decimate(&a0, a1, a2) {
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

fn door_num_blend_delta(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::delta(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_growth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok(give!(mrly_num::blend::growth(&a0)))
}

fn door_num_blend_hadamard(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::hadamard(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok(match mrly_num::blend::recurrence(&a0) {
        Some(item0) => {
            let mut list1 = Vec::new();
            for item1 in item0 {
                list1.push({
                    let parts2 = item1;
                    Value::Array(vec![
                        Value::String(parts2.0.to_string()),
                        Value::String(parts2.1.to_string()),
                    ])
                });
            }
            Value::Array(list1)
        }
        None => Value::Null,
    })
}

fn door_num_blend_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: i128 = small(name, 1, &args[1])?;
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::scale(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_shift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::shift(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    match mrly_num::blend::sigma(&a0) {
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

fn door_num_blend_sub(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_num::blend::sub(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_boolean_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::is_balanced(a0, a1)))
}

fn door_num_boolean_nonlinearity(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::nonlinearity(a0, a1)))
}

fn door_num_boolean_sac(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::sac(a0, a1)))
}

fn door_num_boolean_walsh_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::boolean::walsh_spectrum(a0, a1)))
}

fn door_num_chladni_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::chladni::ladder(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_lines(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::chladni::lines(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_plate(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_num::chladni::plate(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_shake(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f32> = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: u64 = take!(name, 5, &args[5]);
    match mrly_num::chladni::shake(&a0, a1, &a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_aliquot(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::aliquot(a0)))
}

fn door_num_factor_coprime(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::coprime(a0, a1)))
}

fn door_num_factor_divisors(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::divisors(a0)))
}

fn door_num_factor_factorial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::factor::factorial(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_factorize(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::factorize(a0)))
}

fn door_num_factor_factorize_wide(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::factorize_wide(a0)))
}

fn door_num_factor_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(Value::String(mrly_num::factor::gcd(a0, a1).to_string()))
}

fn door_num_factor_lcm(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::lcm(a0, a1)))
}

fn door_num_factor_mobius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::mobius(a0)))
}

fn door_num_factor_mobius_sieve(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::mobius_sieve(a0)))
}

fn door_num_factor_radical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::radical(a0)))
}

fn door_num_factor_reduce(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok({
        let parts0 = mrly_num::factor::reduce(a0, a1);
        Value::Array(vec![
            Value::String(parts0.0.to_string()),
            Value::String(parts0.1.to_string()),
        ])
    })
}

fn door_num_factor_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(mrly_num::factor::sigma(a0, a1).to_string()))
}

fn door_num_factor_squarefree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::squarefree(a0)))
}

fn door_num_factor_totient(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::totient(a0)))
}

fn door_num_factor_totients(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::factor::totients(a0)))
}

fn door_num_factor_twisted(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::factor::twisted(a0, &a1)))
}

fn door_num_fft_convolve(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::fft::convolve(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_convolve_with(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_num::fft::convolve_with(&a0, &a1, &a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_embed_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::fft::embed_kernel(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_log_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::log_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_magnitude_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::magnitude_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_peak_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::fft::peak_ring(&a0)))
}

fn door_num_fft_peak_wavelength(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::fft::peak_wavelength(&a0, a1)))
}

fn door_num_fft_radial_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::radial_profile(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_transform(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::fft::transform(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_gauss_class_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Class::prime(a0)))
}

fn door_num_gauss_class_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Class::word(a0)))
}

fn door_num_gauss_ring_associates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::associates(a0, a1, a2)))
}

fn door_num_gauss_ring_canon(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::canon(a0, a1, a2)))
}

fn door_num_gauss_ring_conjugate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::conjugate(a0, a1, a2)))
}

fn door_num_gauss_ring_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::count(a0, a1)))
}

fn door_num_gauss_ring_div_rem(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::div_rem(a0, a1, a2)))
}

fn door_num_gauss_ring_fate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::fate(a0, a1)))
}

fn door_num_gauss_ring_gaussian_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::gaussian_gcd(a0, a1, a2)))
}

fn door_num_gauss_ring_inert(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::inert(a0, a1)))
}

fn door_num_gauss_ring_mul(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::mul(a0, a1, a2)))
}

fn door_num_gauss_ring_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::named(a0.as_str())))
}

fn door_num_gauss_ring_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::nearest(a0, a1, a2)))
}

fn door_num_gauss_ring_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::norm(a0, a1, a2)))
}

fn door_num_gauss_ring_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::place(a0, a1, a2)))
}

fn door_num_gauss_ring_ramified(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::ramified(a0)))
}

fn door_num_gauss_ring_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::reach(a0, a1, a2)))
}

fn door_num_gauss_ring_symmetry(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::symmetry(a0)))
}

fn door_num_gauss_ring_top(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Ring::top(a0, a1)))
}

fn door_num_gauss_ring_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::turn(a0, a1, a2)))
}

fn door_num_gauss_ring_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Ring::units(a0)))
}

fn door_num_gauss_ring_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Ring::whole(a0, a1, a2)))
}

fn door_num_gauss_window_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::census(&a0)))
}

fn door_num_gauss_window_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Window::class(&a0, a1, a2)))
}

fn door_num_gauss_window_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::gauss::Window::holds(&a0, a1, a2)))
}

fn door_num_gauss_window_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::Window::new(a0, a1)))
}

fn door_num_gauss_window_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::points(&a0)))
}

fn door_num_gauss_window_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::radius(&a0)))
}

fn door_num_gauss_window_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::gauss::Window::ring(&a0)))
}

fn door_num_gauss_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::classes(a0, a1)))
}

fn door_num_gauss_peak(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::peak(a0, a1)))
}

fn door_num_gauss_shells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::gauss::shells(a0, a1)))
}

fn door_num_kummer_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_num::kummer::carry(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_columns(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::columns(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    match mrly_num::kummer::count(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_valuation(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly_num::kummer::valuation(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly_num::kummer::witness(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_coprime_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::coprime_pairs(a0)))
}

fn door_num_lattice_farey(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::farey(a0)))
}

fn door_num_lattice_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::grid(a0)))
}

fn door_num_lattice_new_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::new_nodes(a0)))
}

fn door_num_lattice_pi_estimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::pi_estimate(a0)))
}

fn door_num_lattice_recovered(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::lattice::recovered(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_visible_density(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_num::lattice::visible_density(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_zeta_factor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::lattice::zeta_factor(a0)))
}

fn door_num_lattice_zeta_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly_num::lattice::zeta_whole(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_lift_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::morse::Lift::all()))
}

fn door_num_morse_lift_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::Lift::at(a0, a1, a2)))
}

fn door_num_morse_lift_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::Lift::formula(a0)))
}

fn door_num_morse_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::boundary(&a0)))
}

fn door_num_morse_difference(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::difference(&a0, &a1)))
}

fn door_num_morse_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::digits(a0)))
}

fn door_num_morse_doubling(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::doubling(a0)))
}

fn door_num_morse_faults(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::faults(&a0, &a1)))
}

fn door_num_morse_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::morse::fold(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::letter(a0)))
}

fn door_num_morse_lift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::morse::Lift = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::lift(a0, a1)))
}

fn door_num_morse_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::morse::mask(&a0, &a1)))
}

fn door_num_morse_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_num::morse::power(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_repeat(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::repeat(&a0, a1, a2)))
}

fn door_num_morse_runs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::runs(&a0)))
}

fn door_num_morse_signs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::signs(&a0)))
}

fn door_num_morse_stage(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::stage(a0)))
}

fn door_num_morse_substitution(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::morse::substitution(a0)))
}

fn door_num_morse_upsample(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::morse::upsample(&a0, a1, a2)))
}

fn door_num_prime_sieve_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::count(&a0)))
}

fn door_num_prime_sieve_done(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::done(&a0)))
}

fn door_num_prime_sieve_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::new(a0)))
}

fn door_num_prime_sieve_rank(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::rank(&a0)))
}

fn door_num_prime_sieve_struck(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::struck(&a0)))
}

fn door_num_prime_sieve_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::Sieve::types(&a0)))
}

fn door_num_prime_chart(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::prime::chart(a0, a1)))
}

fn door_num_prime_flags(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::flags(a0)))
}

fn door_num_prime_goldbach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::goldbach(a0)))
}

fn door_num_prime_goldbach_record(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::goldbach_record(a0)))
}

fn door_num_prime_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::is_prime(a0)))
}

fn door_num_prime_pile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::pile(a0)))
}

fn door_num_prime_prime_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::prime_count(a0)))
}

fn door_num_prime_prime_from(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::prime_from(a0)))
}

fn door_num_prime_primes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::primes(a0)))
}

fn door_num_prime_rectangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::rectangles(a0)))
}

fn door_num_prime_splits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::splits(a0)))
}

fn door_num_prime_squares(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::squares(a0)))
}

fn door_num_prime_study(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::prime::study(a0)))
}

fn door_num_readings_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::Design::digits(a0, a1)))
}

fn door_num_readings_design_rudin(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::rudin()))
}

fn door_num_readings_design_ternary(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::ternary()))
}

fn door_num_readings_design_thue(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Design::thue()))
}

fn door_num_readings_reading_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::Reading::all()))
}

fn door_num_readings_reading_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::readings::Reading = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::Reading::formula(a0)))
}

fn door_num_readings_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::arcs(a0, a1)))
}

fn door_num_readings_battery(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::battery(&a0)))
}

fn door_num_readings_bias(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    let a1: Vec<u32> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::bias(&a0, &a1)))
}

fn door_num_readings_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::readings::designs()))
}

fn door_num_readings_model(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: Vec<u32> = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::readings::model(a0, a1, &a2)))
}

fn door_num_readings_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::readings::name(a0)))
}

fn door_num_readings_near_heads(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::readings::Reading = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::near_heads(a0, a1)))
}

fn door_num_readings_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::readings::sign(a0, a1)))
}

fn door_num_readings_stream(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::readings::Design = take!(name, 0, &args[0]);
    let a1: mrly_num::readings::Reading = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::readings::stream(a0, a1, a2)))
}

fn door_num_series_basel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::basel(a0)))
}

fn door_num_series_bernoulli(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_num::series::bernoulli(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.to_string()),
                        Value::String(parts1.1.to_string()),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_beta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::series::beta(a0, a1)))
}

fn door_num_series_binary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::binary(a0)))
}

fn door_num_series_catalan(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::catalan(a0)))
}

fn door_num_series_chi3(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi3(a0)))
}

fn door_num_series_chi4(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi4(a0)))
}

fn door_num_series_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::chi8(a0)))
}

fn door_num_series_dirichlet(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::series::dirichlet(a0, &a1, a2)))
}

fn door_num_series_e_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::e_partial(a0)))
}

fn door_num_series_euler_gamma_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::euler_gamma_partial(a0)))
}

fn door_num_series_euler_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::series::euler_product(a0, a1)))
}

fn door_num_series_evens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::evens(a0)))
}

fn door_num_series_fibonacci(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::fibonacci(a0)))
}

fn door_num_series_harmonic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::harmonic(a0)))
}

fn door_num_series_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::series::lambda(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_leibniz(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::leibniz(a0)))
}

fn door_num_series_li(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::li(a0)))
}

fn door_num_series_mertens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::mertens(a0)))
}

fn door_num_series_odds(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::odds(a0)))
}

fn door_num_series_visible(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::series::visible(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_wallis_half_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::wallis_half_pi(a0)))
}

fn door_num_series_wallis_quarter_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::series::wallis_quarter_pi(a0)))
}

fn door_num_series_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::series::zeta(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::cells(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_flat_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::sieve::flat_word(a0, a1)))
}

fn door_num_sieve_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::holes(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::limit(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_odd_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::sieve::odd_word(a0)))
}

fn door_num_sieve_parity_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly_num::sieve::parity_product(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_punctures(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::punctures(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::raster(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly_num::sieve::ratio(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    match mrly_num::sieve::row_fill(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::row_law(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_num::sieve::row_profile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_settle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_num::sieve::row_settle(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly_num::sieve::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_solid_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::sieve::solid_limit()))
}

fn door_num_spiral_growth_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Growth::all()))
}

fn door_num_spiral_lattice_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Lattice::all()))
}

fn door_num_spiral_lattice_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::count(a0, a1)))
}

fn door_num_spiral_lattice_n(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::Lattice::n(a0, a1, a2)))
}

fn door_num_spiral_lattice_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::radius(a0, a1)))
}

fn door_num_spiral_lattice_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::ring(a0, a1)))
}

fn door_num_spiral_lattice_ring_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::Lattice::ring_of(a0, a1, a2)))
}

fn door_num_spiral_lattice_xy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::Lattice::xy(a0, a1)))
}

fn door_num_spiral_mark_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::spiral::Mark::all()))
}

fn door_num_spiral_diagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    let a4: i64 = take!(name, 4, &args[4]);
    Ok(give!(mrly_num::spiral::diagonal(a0, a1, a2, a3, a4)))
}

fn door_num_spiral_level_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::level_of(a0, a1)))
}

fn door_num_spiral_marks(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::spiral::Mark = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::spiral::marks(a0, a1)))
}

fn door_num_spiral_snail(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: mrly_num::spiral::Growth = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::spiral::snail(a0, a1, a2)))
}

fn door_num_zeta_complex_abs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::abs(a0)))
}

fn door_num_zeta_complex_arg(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::arg(a0)))
}

fn door_num_zeta_complex_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Complex::default()))
}

fn door_num_zeta_complex_exp(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::exp(a0)))
}

fn door_num_zeta_complex_ln(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::ln(a0)))
}

fn door_num_zeta_complex_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Complex::new(a0, a1)))
}

fn door_num_zeta_complex_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::Complex::turn(a0)))
}

fn door_num_zeta_line_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::count(&a0, a1)))
}

fn door_num_zeta_line_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Line::default()))
}

fn door_num_zeta_line_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::exact(&a0, a1)))
}

fn door_num_zeta_line_gram(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::gram(&a0, a1)))
}

fn door_num_zeta_line_maclaurin(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::maclaurin(&a0, a1)))
}

fn door_num_zeta_line_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::Line::new()))
}

fn door_num_zeta_line_novelty_coefficients(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::novelty_coefficients(&a0, &a1)))
}

fn door_num_zeta_line_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::pair(&a0, a1)))
}

fn door_num_zeta_line_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::point(&a0, a1)))
}

fn door_num_zeta_line_seam(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_num::zeta::Line::seam(&a0, a1, a2, a3)))
}

fn door_num_zeta_line_siegel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::siegel(&a0, a1)))
}

fn door_num_zeta_line_theta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::theta(&a0, a1)))
}

fn door_num_zeta_line_z(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::z(&a0, a1)))
}

fn door_num_zeta_line_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_num::zeta::Line = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::Line::zeros(&a0, a1)))
}

fn door_num_zeta_bump(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::bump(a0)))
}

fn door_num_zeta_corrections(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::corrections(a0)))
}

fn door_num_zeta_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::kernel(a0)))
}

fn door_num_zeta_mellin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::mellin(a0)))
}

fn door_num_zeta_novelty_main(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_num::zeta::novelty_main()))
}

fn door_num_zeta_novelty_wave(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<mrly_num::zeta::Complex> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::zeta::novelty_wave(&a0, &a1, a2)))
}

fn door_num_zeta_psi_curve(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_curve(&a0, &a1)))
}

fn door_num_zeta_psi_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_formula(a0, &a1)))
}

fn door_num_zeta_psi_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::psi_gap(a0, &a1)))
}

fn door_num_zeta_psi_stair(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_num::zeta::psi_stair(a0)))
}

fn door_num_zeta_raise(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: mrly_num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::raise(a0, a1)))
}

fn door_num_zeta_sharp_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_num::zeta::sharp_novelty(&a0, a1)))
}

fn door_num_zeta_smoothed_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_num::zeta::smoothed_novelty(&a0, a1, a2)))
}
