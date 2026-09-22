export { default, initSync } from "./pkg/num/mrlyjs_num.js";

/** An rgba color as four bytes. */
export type Color = [number, number, number, number];
/** A tensor: its shape and its flat data as a typed array of its dtype. */
export interface Tensor {
    shape: number[];
    data: Uint8Array | Uint16Array | Uint32Array | Int32Array;
}
/** A cell: the shape, the type bytes, and the flat rgba colors and the tags when present. */
export interface Cell {
    shape: number[];
    types: Uint8Array | Uint16Array | Uint32Array | Int32Array;
    colors?: Uint8Array;
    tags?: Uint8Array | Uint16Array | Uint32Array | Int32Array;
}
/** A hex cell: a flat cell with its projection, orientation and start row. */
export interface Cell6d {
    cell: Cell;
    projection: "Iso" | "Pro" | "Cut";
    orientation: "Horizontal" | "Vertical";
    start: number;
}
/** A color inside plain data, serde's form. */
export interface ColorData {
    r: number;
    g: number;
    b: number;
    a: number;
}
/** A tensor inside plain data, serde's form. */
export interface TensorData {
    shape: number[];
    data: { U8: number[] } | { U16: number[] } | { U32: number[] } | { I32: number[] };
}
/** A cell inside plain data, serde's form. */
export interface CellData {
    types: TensorData;
    colors?: number[][];
    tags?: TensorData;
}
/** A hex cell inside plain data, serde's form. */
export interface Cell6dData {
    cell: { cell: CellData };
    projection: "Iso" | "Pro" | "Cut";
    orientation: "Horizontal" | "Vertical";
    start: number;
}
/** A seeded random stream, opened from a number or a bigint seed. */
export class Rng {
    constructor(seed: number | bigint | string);
    free(): void;
    /** Draws a float at or above zero and below one. */
    unit(): number;
    /** Draws an integer below n, or zero when n is zero. */
    below(n: number): number;
    /** Draws an integer between lo and hi inclusive, or lo when hi is not above lo. */
    range(lo: number, hi: number): number;
    /** Draws a fair coin flip. */
    boolean(): boolean;
    /** Returns true with probability p. */
    chance(p: number): boolean;
    /** Draws amount distinct indices below length, or every index when amount is larger. */
    sample_indices(length: number, amount: number): Uint32Array;
}
export declare namespace apollonian {
    /** The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves. */
    export function form(u: ArrayLike<number | bigint>, v: ArrayLike<number | bigint>): string;
    /** The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing. */
    export function frame(p: apollonian.Packing): Float64Array;
    /** Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period. */
    export function grow(name: string, cap: number | bigint): apollonian.Packing;
    /** Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`. */
    export function is_ford(c: apollonian.Circle): boolean;
    /** Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`. */
    export function on_line(c: apollonian.Circle): boolean;
    /** Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root. */
    export function reflect(q: apollonian.Circle[], at: number): apollonian.Circle;
    /** The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures. */
    export function root(name: string): apollonian.Circle[];
    /** Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero. */
    export function shadow(p: apollonian.Packing, order: number): apollonian.Shadow;
    /** Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`. */
    export function sound(q: apollonian.Circle[]): boolean;
    /** The quadruple with the circle at the seat replaced by its reflection. */
    export function swap(q: apollonian.Circle[], at: number): apollonian.Circle[];
    /** The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip. */
    export function touches(p: apollonian.Packing): apollonian.Touch[];
    /** The most circles one growth makes before it gives up. */
    export function CIRCLE_CAP(): number;
    /** The largest curvature a packing is grown to. */
    export function CURVATURE_CAP(): bigint;
    /** The deepest the Farey stack is read against a packing. */
    export function ORDER_CAP(): number;
    /** The root quadruples on offer: the strip first, then the bounded packings named by their curvatures. */
    export function ROOTS(): string[];
    export interface CircleData {
        /** The curvature. */
        k: number;
        /** The curvature times the centre's abscissa. */
        x: number;
        /** The curvature times the centre's ordinate. */
        y: number;
    }
    /** A circle in the integer coordinates `(k, k x, k y)`: a line is `k = 0` with `(k x, k y)` its outward unit normal, and the curvature is negative on the circle that contains a bounded packing. */
    export class Circle {
        private constructor();
        free(): void;
        /** Reads the Circle from its plain data. */
        static from(data: CircleData): Circle;
        /** Writes the Circle as plain data. */
        toJSON(): CircleData;
        /** The curvature. */
        readonly k: bigint;
        /** The curvature times the centre's abscissa. */
        readonly x: bigint;
        /** The curvature times the centre's ordinate. */
        readonly y: bigint;
        /** The centre, none on a line. */
        centre(): [number, number] | undefined;
        /** Whether the circle is a line. */
        is_line(): boolean;
        /** The radius, none on a line. */
        radius(): number | undefined;
    }
    /** A packing grown from its root in exact integers. */
    export interface Packing {
        /** The root quadruple the growth started from. */
        root: apollonian.CircleData[];
        /** Whether the root carries a line, so the packing is the strip and the growth keeps one period. */
        strip: boolean;
        /** The curvature the growth stopped at. */
        cap: number;
        /** The circles the growth made, the root excluded, in curvature order. */
        circles: apollonian.CircleData[];
        /** The quadruples the growth made, the root counted. */
        quads: number;
        /** The quadruples that failed one of the six invariants. */
        broken: number;
        /** The circles centred outside the open period, which the strip must have none of. */
        strayed: number;
    }
    /** The Farey stack read against the packing's tangency points. */
    export interface Shadow {
        /** The depth the stack is read at. */
        order: number;
        /** The curvature a circle of denominator the order carries, twice the order squared. */
        reach: number;
        /** Whether the packing was grown far enough to carry every node of that depth. */
        covered: boolean;
        /** The stack's nodes inside the open period. */
        nodes: number;
        /** The packing's tangency points of denominator at most the order. */
        touched: number;
        /** The nodes no tangency point rests on, plus the tangency points no node lights. */
        missed: number;
        /** The circles below the reach tangent to the line that are not Ford circles. */
        offford: number;
        /** The brightness of the period summed node by node, the node `0/1` on the period's edge counted. */
        bright: bigint;
        /** The closed form that brightness lands on, `Q(Q + 1)/2`. */
        want: bigint;
    }
    /** A tangency point on the line `y = 0`: the reduced fraction the circle rests at and the curvature it carries. */
    export interface Touch {
        /** The numerator of the reduced fraction. */
        num: number;
        /** The denominator of the reduced fraction. */
        den: number;
        /** The curvature of the circle resting there, which the Ford identification forces to be `2 den^2`. */
        k: number;
    }
}
export declare namespace automaton {
    /** The relative rounding allowance the double-precision matrix ladder charges against the scale it carries. */
    export function ROUNDING(): number;
    export type AutomatonData = Record<string, unknown>;
    /** A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from. */
    export class Automaton {
        /** Builds the ladder of a rule, choosing the peel depth. */
        constructor(rule: memory.Rule);
        free(): void;
        /** Reads the Automaton from its plain data. */
        static from(data: AutomatonData): Automaton;
        /** Writes the Automaton as plain data. */
        toJSON(): AutomatonData;
        /** Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`]. */
        abscissa(): number;
        /** Returns the base `q = 2^D`. */
        base(): bigint;
        /** Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to. */
        cofactor(s: zeta.Complex, tolerance: number): [zeta.Complex, number];
        /** Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`. */
        denominator(): Float64Array;
        /** Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`. */
        matrix(): Float64Array[];
        /** Returns the peel depth `P`. */
        peel(): number;
        /** Returns the pole spacing `2 pi / log q`. */
        period(): number;
        /** Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with. */
        perron(): [number, number];
        /** Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to. */
        residue(w0: zeta.Complex, tolerance: number): [zeta.Complex, number];
        /** Returns the rule. */
        rule(): memory.Rule;
        /** Returns the state count `q^(k-1)`. */
        states(): number;
        /** Builds the ladder at an explicit peel depth, at least the rule width and at least two. */
        static with_peel(rule: memory.Rule, peel: number): automaton.Automaton;
        /** Returns `zeta_W(s)` and the bound it is known to. */
        zeta(s: zeta.Complex, tolerance: number): [zeta.Complex, number];
    }
}
export declare namespace blend {
    /** Adds two sequences term by term over their shared length. */
    export function add(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    /** Convolves two sequences, keeping the exact prefix their shared length affords. */
    export function cauchy(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    /** Returns the monic characteristic polynomial of a recurrence, highest power first. */
    export function characteristic(coefficients: ([string | number | bigint, string | number | bigint])[]): [string, string][];
    /** Keeps every step-th term from the offset onward. */
    export function decimate(a: (string | number | bigint)[], step: number, offset: number): string[];
    /** Returns the first differences of a sequence, one term shorter. */
    export function delta(a: (string | number | bigint)[]): string[];
    /** Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands. */
    export function growth(coefficients: ([string | number | bigint, string | number | bigint])[]): number;
    /** Multiplies two sequences term by term over their shared length. */
    export function hadamard(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    /** Finds the smallest linear constant-coefficient recurrence that fits every supplied term. */
    export function recurrence(terms: (string | number | bigint)[]): [string, string][] | undefined;
    /** Multiplies every term of a sequence by the factor. */
    export function scale(a: (string | number | bigint)[], factor: string | number | bigint): string[];
    /** Drops the first terms of a sequence. */
    export function shift(a: (string | number | bigint)[], count: number): string[];
    /** Returns the partial sums of a sequence. */
    export function sigma(a: (string | number | bigint)[]): string[];
    /** Subtracts the second sequence from the first over their shared length. */
    export function sub(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
}
export declare namespace boolean {
    /** Reports whether the packed function outputs one on exactly half of its inputs. */
    export function is_balanced(code: string | number | bigint, n: number): boolean;
    /** Returns how far the packed function sits from every affine function, zero when it is one. */
    export function nonlinearity(code: string | number | bigint, n: number): bigint;
    /** Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche. */
    export function sac(code: string | number | bigint, n: number): number;
    /** Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code. */
    export function walsh_spectrum(code: string | number | bigint, n: number): BigInt64Array;
}
export declare namespace design {
    /** Returns the digits a bitmask names inside the base, ascending. */
    export function digits_of(mask: number, base: number | bigint): BigUint64Array;
    /** Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element. */
    export function echo_series(values: ArrayLike<number | bigint>, log_x: ArrayLike<number>, exponent: number): Float64Array;
    /** Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero. */
    export function elements(base: number | bigint, digits: ArrayLike<number | bigint>, depth: number): BigUint64Array;
    /** Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last. */
    export function log_grid(values: ArrayLike<number | bigint>, samples: number): Float64Array;
    /** Returns the running median of the power over a window of the given width, the window clamped at the ends. */
    export function median_floor(power: ArrayLike<number>, width: number): Float64Array;
    /** Returns the running design Mobius meter, the partial sums of the Mobius values along the elements. */
    export function meter(mu: ArrayLike<number>): BigInt64Array;
    /** Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty. */
    export function nearest(value: number, list: ArrayLike<number>): number;
    /** Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first. */
    export function peaks(gamma: ArrayLike<number>, score: ArrayLike<number>, band: [number, number], threshold: number): Uint32Array;
    /** Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries. */
    export function pole_lattice(base: number | bigint, top: number): Float64Array;
    /** Reads the running meter at every point of the log grid and divides by x to the exponent. */
    export function resample(values: ArrayLike<number | bigint>, running: ArrayLike<number | bigint>, exponent: number, log_x: ArrayLike<number>): Float64Array;
    /** Returns the power over its local median floor, the score a peak is read against. */
    export function score(power: ArrayLike<number>, width: number): Float64Array;
    /** Returns the count of elements the design holds at the depth, the length [`elements`] returns without building them. */
    export function size(digits: ArrayLike<number | bigint>, depth: number): string;
    /** Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range. */
    export function spectrum(log_x: ArrayLike<number>, series: ArrayLike<number>): [Float64Array, Float64Array];
    /** Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at. */
    export function upper_rms(series: ArrayLike<number>): number;
    /** The ordinates of the first fourteen nontrivial zeros of the Riemann zeta function, the imaginary parts of the zeros on the critical line in ascending order. */
    export function ZETA_ORDINATES(): Float64Array;
}
export declare namespace factor {
    /** Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one. */
    export function aliquot(number: number): number;
    /** Returns whether two numbers share no divisor above one. */
    export function coprime(a: number, b: number): boolean;
    /** Builds every divisor of a wide number from its factorization, ascending, empty for zero. */
    export function divisors(number: number | bigint): BigUint64Array;
    /** Returns the factorial of the number, the product of one through it, erring past thirty-four. */
    export function factorial(number: number): string;
    /** Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel. */
    export function factorize(number: number): [number, number][];
    /** Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel. */
    export function factorize_wide(number: number | bigint): [bigint, number][];
    /** Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes. */
    export function gcd(a: string | number | bigint, b: string | number | bigint): string;
    /** Returns the least common multiple of two numbers, zero when either side is zero. */
    export function lcm(a: number, b: number): number;
    /** Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes. */
    export function mobius(number: number): number;
    /** Sieves the Mobius values of zero through the limit in one pass. */
    export function mobius_sieve(limit: number): Int8Array;
    /** Returns the radical of the number, the product of its distinct primes, zero for zero and one for one. */
    export function radical(number: number): number;
    /** Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one. */
    export function reduce(numerator: string | number | bigint, denominator: string | number | bigint): [string, string];
    /** Returns the sum of every divisor of the number raised to the power, so power zero counts them. */
    export function sigma(number: number, power: number): string;
    /** Returns whether no prime squares into the number, true for one and false for zero. */
    export function squarefree(number: number): boolean;
    /** Returns the Euler totient of the number from its factorization, zero for zero and one for one. */
    export function totient(number: number): number;
    /** Sieves the Euler totients of zero through n in one pass, the run beside the single value. */
    export function totients(n: number): BigUint64Array;
    /** Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm. */
    export function twisted(number: number, rhythm: ArrayLike<number>): bigint;
}
export declare namespace fft {
    /** Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways. */
    export function convolve(field: ArrayLike<number>, kernel: ArrayLike<number>, size: number): Float64Array;
    /** Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared. */
    export function convolve_with(field: ArrayLike<number>, kernel_re: ArrayLike<number>, kernel_im: ArrayLike<number>, size: number): Float64Array;
    /** Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts. */
    export function embed_kernel(mask: ArrayLike<number>, side: number, size: number): Float64Array;
    /** Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre. */
    export function log_spectrum(field: ArrayLike<number>, size: number): Float64Array;
    /** Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre. */
    export function magnitude_spectrum(field: ArrayLike<number>, size: number): Float64Array;
    /** Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0. */
    export function peak_ring(profile: ArrayLike<number>): number;
    /** Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0. */
    export function peak_wavelength(profile: ArrayLike<number>, size: number): number;
    /** Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field. */
    export function radial_profile(spectrum: ArrayLike<number>, size: number): Float64Array;
    /** Transforms a real size-square field forward by fft2, returning the real and imaginary parts. */
    export function transform(field: ArrayLike<number>, size: number): [Float64Array, Float64Array];
}
export declare namespace gauss {
    /** Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates. */
    export function classes(ring: gauss.Ring, bound: number | bigint): [bigint, bigint][];
    /** Returns the norm from one through the limit with the most points and that count, the earliest on a tie. */
    export function peak(ring: gauss.Ring, limit: number): [number, number];
    /** Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice. */
    export function shells(ring: gauss.Ring, limit: number): Uint32Array;
    /** The tallies of a window: every class counted and the share of primes. */
    export interface Census {
        /** The count of points. */
        points: number;
        /** The count of primes. */
        primes: number;
        /** The split primes. */
        split: number;
        /** The inert primes. */
        inert: number;
        /** The ramified primes. */
        ramified: number;
        /** The units. */
        units: number;
        /** The composites. */
        composites: number;
        /** The primes over the points. */
        density: number;
    }
    /** What a point of the ring is. */
    export type Class = "Zero" | "Unit" | "Ramified" | "Split" | "Inert" | "Composite";
    export const Class: {
        /** Returns whether the class is prime. */
        prime(class_: gauss.Class): boolean;
        /** Returns the class as a word. */
        word(class_: gauss.Class): string;
    };
    /** The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice. */
    export type Ring = "Gaussian" | "Eisenstein";
    export const Ring: {
        /** Returns the unit multiples of a point, the point first, turning anticlockwise. */
        associates(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint][];
        /** Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin. */
        canon(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        /** Returns the conjugate: the mirror image in the real axis. */
        conjugate(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        /** Returns the count of points within the reach: the square or the hexagon. */
        count(ring: gauss.Ring, radius: number | bigint): number;
        /** Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`. */
        div_rem(ring: gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [[bigint, bigint], [bigint, bigint]];
        /** Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise. */
        fate(ring: gauss.Ring, n: number | bigint): gauss.Class;
        /** Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins. */
        gaussian_gcd(ring: gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [bigint, bigint];
        /** Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3. */
        inert(ring: gauss.Ring, p: number | bigint): boolean;
        /** Returns the product of two points. */
        mul(ring: gauss.Ring, arg1: [number | bigint, number | bigint], arg2: [number | bigint, number | bigint]): [bigint, bigint];
        /** Reads a ring from its name. */
        named(name: string): gauss.Ring | undefined;
        /** Returns the point nearest a place in the plane. */
        nearest(ring: gauss.Ring, x: number, y: number): [bigint, bigint];
        /** Returns the norm of a point: its squared length. */
        norm(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint;
        /** Returns the place of a point in the plane, x right and y up, one unit between neighbours. */
        place(ring: gauss.Ring, a: number | bigint, b: number | bigint): [number, number];
        /** Returns the one rational prime that ramifies: 2 or 3. */
        ramified(ring: gauss.Ring): bigint;
        /** Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance. */
        reach(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint;
        /** Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12. */
        symmetry(ring: gauss.Ring): number;
        /** Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's. */
        top(ring: gauss.Ring, radius: number | bigint): bigint;
        /** Returns the point turned anticlockwise by one unit: a quarter turn or a sixth. */
        turn(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        /** Returns the count of units: 4 or 6. */
        units(ring: gauss.Ring): number;
        /** Returns the whole number an associate of the point lies on, when one lies on the positive real axis. */
        whole(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint | undefined;
    };
    export type WindowData = Record<string, unknown>;
    /** The symmetric window of one ring: every point within a reach, with the norms sieved once. */
    export class Window {
        /** Opens the window of a ring out to a reach, sieving every norm inside it. */
        constructor(ring: gauss.Ring, radius: number | bigint);
        free(): void;
        /** Reads the Window from its plain data. */
        static from(data: WindowData): Window;
        /** Writes the Window as plain data. */
        toJSON(): WindowData;
        /** Counts every class inside. */
        census(): gauss.Census;
        /** Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime. */
        class_(a: number | bigint, b: number | bigint): gauss.Class;
        /** Returns whether a point lies inside. */
        holds(a: number | bigint, b: number | bigint): boolean;
        /** Lists every point inside, row by row from the bottom left of the bounding square. */
        points(): [bigint, bigint][];
        /** Returns the reach. */
        radius(): bigint;
        /** Returns the ring. */
        ring(): gauss.Ring;
    }
}
export declare namespace ladder {
    /** Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to. */
    export function cofactor(design: ladder.Design, s: zeta.Complex, tolerance: number): [zeta.Complex, number];
    /** Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to. */
    export function residue(design: ladder.Design, m: number, j: number | bigint, tolerance: number): [zeta.Complex, number];
    /** Returns `zeta_F(s)` and the bound it is known to. */
    export function zeta(design: ladder.Design, s: zeta.Complex, tolerance: number): [zeta.Complex, number];
    /** The relative rounding allowance the double-precision ladder charges against the scale it carries. */
    export function ROUNDING(): number;
    export type DesignData = Record<string, unknown>;
    /** A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at. */
    export class Design {
        /** Builds a design on the base and the digit set, choosing the peel depth. */
        constructor(base: number | bigint, digits: ArrayLike<number | bigint>);
        free(): void;
        /** Reads the Design from its plain data. */
        static from(data: DesignData): Design;
        /** Writes the Design as plain data. */
        toJSON(): DesignData;
        /** Returns the abscissa `alpha = log_q k`. */
        abscissa(): number;
        /** Returns the base. */
        base(): bigint;
        /** Returns the digit set, ascending. */
        digits(): BigUint64Array;
        /** Returns the peel depth. */
        peel(): number;
        /** Returns the pole spacing `2 pi / log q`. */
        period(): number;
        /** Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`. */
        pole(m: number, j: number | bigint): zeta.Complex;
        /** Builds a design at an explicit peel depth, at least two. */
        static with_peel(base: number | bigint, digits: ArrayLike<number | bigint>, peel: number): ladder.Design;
    }
}
export declare namespace lattice {
    /** Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one. */
    export function coprime_pairs(n: number): bigint;
    /** Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending. */
    export function farey(order: number): lattice.Node[];
    /** Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes. */
    export function grid(n: number): lattice.Node2d[];
    /** Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after. */
    export function new_nodes(n: number): bigint;
    /** Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared. */
    export function pi_estimate(n: number): number;
    /** Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one. */
    export function recovered(n: number, dimension: number): number;
    /** The density the visible count of a window in the dimension walks to, one over zeta of the dimension. */
    export function visible_density(dimension: number): number;
    /** The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve. */
    export function zeta_factor(dimension: number): number | undefined;
    /** The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one. */
    export function zeta_whole(s: number): number;
    /** A visible node: a reduced fraction and the brightness a stack of scales one through the window gives it. */
    export interface Node {
        /** The numerator, coprime to the denominator. */
        num: number;
        /** The denominator. */
        den: number;
        /** The count of scales putting a line here: the floor of the window over the denominator. */
        brightness: number;
    }
    /** A grid crossing of two visible nodes, its brightness the separable product. */
    export interface Node2d {
        /** The horizontal node. */
        x: lattice.Node;
        /** The vertical node. */
        y: lattice.Node;
        /** The product of the two axis brightnesses. */
        brightness: number;
    }
}
export declare namespace memory {
    /** Returns the count of allowed windows `card W`, the bits the code sets inside its window range. */
    export function allowed_windows(rule: memory.Rule): number;
    /** Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first. */
    export function cells(rule: memory.Rule, level: number): BigUint64Array;
    /** Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`. */
    export function counts(rule: memory.Rule, levels: number): BigUint64Array;
    /** Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count. */
    export function exponent(rule: memory.Rule): number;
    /** Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory. */
    export function kappa(rule: memory.Rule): number;
    /** Returns the Perron root of the transfer matrix, the count's growth per level. */
    export function perron(rule: memory.Rule): number;
    /** Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed. */
    export function transfer(rule: memory.Rule): BigUint64Array[];
    /** The largest digit span a rule may read, so its code fits a `u64`. */
    export function SPAN(): number;
    /** The sweep cap of the power iteration. */
    export function SWEEPS(): number;
    /** The absolute `l^1` move of the normalised iterate that stops the power iteration, counted only when it holds over three consecutive sweeps. */
    export function TOLERANCE(): number;
    export interface RuleData {
        /** The dimension `D`, one to three. */
        dimension: number;
        /** The window width `k`, at least one. */
        width: number;
        /** The window code, bit `w` set when window `w` is allowed. */
        code: number;
    }
    /** A rule on `k` consecutive digits of a design word. */
    export class Rule {
        /** Builds a rule from its dimension, its width and its code. */
        constructor(dimension: number, width: number, code: number | bigint);
        free(): void;
        /** Reads the Rule from its plain data. */
        static from(data: RuleData): Rule;
        /** Writes the Rule as plain data. */
        toJSON(): RuleData;
        /** The dimension `D`, one to three. */
        readonly dimension: number;
        /** The window width `k`, at least one. */
        readonly width: number;
        /** The window code, bit `w` set when window `w` is allowed. */
        readonly code: bigint;
        /** Returns whether a word, coarsest digit first, is accepted. */
        accepts(word: ArrayLike<number>): boolean;
        /** Returns whether the window is allowed, and false for any window out of range. */
        allowed(window: number): boolean;
        /** Returns the letters that stand in at least one allowed window. */
        alphabet(): Uint32Array;
        /** Returns the count of rules of this shape, `2^(2^(k D))`. */
        codes(): string;
        /** Returns the rule that allows every window. */
        static full(dimension: number, width: number): memory.Rule;
        /** Returns the letter count `2^D`, the digit vectors of the cube's corners. */
        letters(): number;
        /** Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on. */
        states(): number;
        /** Returns the window count `2^(k D)`. */
        windows(): number;
    }
}
export declare namespace morse {
    /** Returns the run-boundary word, one wherever a letter differs from the next. */
    export function boundary(word: ArrayLike<number>): Uint8Array;
    /** Exclusive-ors two grids of the same length, site by site. */
    export function difference(a: ArrayLike<number>, b: ArrayLike<number>): Uint8Array;
    /** Builds the first letters of the Thue-Morse word by the digit rule. */
    export function digits(length: number): Uint8Array;
    /** Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1. */
    export function doubling(length: number): Uint8Array;
    /** Counts the sites where two grids of the same length differ. */
    export function faults(a: ArrayLike<number>, b: ArrayLike<number>): number;
    /** Tests a grid against the Kronecker power of its own corner tile. */
    export function fold(grid: ArrayLike<number>, side: number, number: number): morse.Fold;
    /** Returns the Thue-Morse letter at the place, the parity of its binary digit sum. */
    export function letter(place: number | bigint): number;
    /** Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one. */
    export function lift(kind: morse.Lift, side: number): Uint8Array;
    /** Folds a tile of the side into its Kronecker power at the level, one bit per site. */
    export function power(tile: ArrayLike<number>, number: number, level: number): Uint8Array;
    /** Repeats a tile until it fills a grid of the side. */
    export function repeat(tile: ArrayLike<number>, number: number, side: number): Uint8Array;
    /** Returns the lengths of the maximal blocks of one repeated letter, in order. */
    export function runs(word: ArrayLike<number>): Uint32Array;
    /** Returns the substitution stage after the rounds, a word of length two to the rounds. */
    export function stage(rounds: number): Uint8Array;
    /** Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`. */
    export function substitution(length: number): Uint8Array;
    /** Blows a grid up by the scale, every site becoming a scale-by-scale block. */
    export function upsample(grid: ArrayLike<number>, side: number, scale: number): Uint8Array;
    /** Lists the lifts in the order the gallery draws them. */
    export function LIFTS(): morse.Lift[];
    /** The verdict on whether a grid is the Kronecker power of its own corner tile. */
    export interface Fold {
        /** The corner tile the test folds, row major. */
        tile: number[];
        /** The side of the tile. */
        number: number;
        /** The count of tile factors the side asks for. */
        level: number;
        /** Whether the grid is that tile's Kronecker power. */
        folds: boolean;
        /** The count of sites where the grid and the power differ. */
        faults: number;
        /** The first differing site in row-major order, when there is one. */
        first?: [number, number];
    }
    /** The four ways the word lifts from a line to the plane, one sign at every site. */
    export type Lift = "Parity" | "And" | "Xor" | "Sum";
    export const Lift: {
        /** Returns every Lift in canonical order. */
        all(): morse.Lift[];
        /** Returns the sign at a site, zero for plus one and one for minus one. */
        at(lift: morse.Lift, i: number | bigint, j: number | bigint): number;
        /** Returns the lift's formula, written the way the page prints it. */
        formula(lift: morse.Lift): string;
    };
}
export declare namespace prime {
    /** Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last. */
    export function chart(top: number, bins: number): prime.Reading[];
    /** Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag. */
    export function flags(limit: number): boolean[];
    /** Returns the count of unordered pairs of primes summing to the number, zero below four. */
    export function goldbach(number: number): number;
    /** Returns the count of prime pairs at every even number from four up to the top, one entry per even number. */
    export function goldbach_record(top: number): Uint32Array;
    /** Returns whether the number is prime, by trial division on the six-step wheel. */
    export function is_prime(number: number): boolean;
    /** Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization. */
    export function pile(number: number | bigint): prime.Pile;
    /** Returns the count of primes at or below n. */
    export function prime_count(n: number): number;
    /** Returns the smallest prime at or above the number. */
    export function prime_from(number: number): number;
    /** Returns the primes up to the limit, the finished sieve read as a list. */
    export function primes(limit: number): Uint32Array;
    /** Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root. */
    export function rectangles(number: number): [number, number][];
    /** Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending. */
    export function splits(number: number): [number, number][];
    /** Returns the smallest pair of positive sides whose squares sum to the number, when one exists. */
    export function squares(number: number): [number, number] | undefined;
    /** Returns one prime object for every prime up to and including the limit. */
    export function study(limit: number): prime.Prime[];
    /** A number as a pile of stones: its prime factors, whether it is prime, and every rectangle the stones make. */
    export interface Pile {
        /** The count of stones. */
        number: number;
        /** The prime and exponent pairs, ascending. */
        factors: [number, number][];
        /** Whether the stones make a single row and nothing else. */
        prime: boolean;
        /** Every rectangle as a pair of sides, the shorter first, ascending. */
        rectangles: [number, number][];
    }
    /** The prime object: one prime with its rank, the step behind it and the shapes it makes. */
    export interface Prime {
        /** The prime itself. */
        value: number;
        /** The one-based rank in the prime sequence, so two has index one. */
        index: number;
        /** The distance from the previous prime, zero for two. */
        gap: number;
        /** The twin flag: whether a prime sits exactly two away on either side, false for two. */
        twin: boolean;
        /** The two positive sides whose squares sum to the prime, when they exist. */
        squares?: [number, number];
    }
    /** One reading of the prime count against its two smooth guesses. */
    export interface Reading {
        /** The point on the number line. */
        x: number;
        /** The count of primes up to it. */
        pi: number;
        /** The guess x over ln x. */
        ratio: number;
        /** The logarithmic integral. */
        li: number;
    }
    export type SieveData = Record<string, unknown>;
    /** The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it. */
    export class Sieve {
        /** Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside. */
        constructor(limit: number);
        free(): void;
        /** Reads the Sieve from its plain data. */
        static from(data: SieveData): Sieve;
        /** Writes the Sieve as plain data. */
        toJSON(): SieveData;
        /** Returns the count of numbers marked prime so far. */
        count(): number;
        /** Returns whether every number is settled. */
        done(): boolean;
        /** Runs the sieve to the end. */
        finish(): void;
        /** Returns the count of primes used so far. */
        rank(): number;
        /** Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done. */
        step(): number;
        /** Returns the count of numbers the last step struck. */
        struck(): number;
        /** Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it. */
        types(): Uint8Array;
    }
}
export declare namespace radix {
    /** Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`. */
    export function flowsnake(): radix.Radix;
    /** Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`. */
    export function gasket(): radix.Radix;
    /** Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`. */
    export function koch(): radix.Radix;
    /** Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`. */
    export function terdragon(): radix.Radix;
    /** Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`. */
    export function tile(m: number | bigint, code: string | number | bigint): radix.Radix;
    /** Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`. */
    export function twindragon(): radix.Radix;
    export type BaseData = Record<string, unknown>;
    /** The base of a radix design: a ring and an element of norm at least two, the scale every word is read against. */
    export class Base {
        /** Fixes a base in a ring. */
        constructor(ring: gauss.Ring, value: [number | bigint, number | bigint]);
        free(): void;
        /** Reads the Base from its plain data. */
        static from(data: BaseData): Base;
        /** Writes the Base as plain data. */
        toJSON(): BaseData;
        /** Returns the index in the canonical residue system of the class of a point. */
        class_(z: [number | bigint, number | bigint]): number;
        /** Returns whether two points are congruent modulo the base. */
        congruent(z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): boolean;
        /** Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base. */
        group(): Uint32Array[];
        /** Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group. */
        mirrored(): boolean;
        /** Returns the norm `q` of the base: the count of residue classes and the square of the scale. */
        norm(): bigint;
        /** Returns the base raised to a level. */
        power(level: number): [bigint, bigint];
        /** Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`. */
        residues(): [bigint, bigint][];
        /** Returns the ring. */
        ring(): gauss.Ring;
        /** Returns the base element. */
        value(): [bigint, bigint];
    }
    export type RadixData = Record<string, unknown>;
    /** A radix design: a digit set inside one ring, placed by a base with a unit twist per digit. */
    export class Radix {
        /** Builds a design from a base, a digit list and a unit twist per digit. */
        constructor(base: radix.Base, digits: ([number | bigint, number | bigint])[], twists: ([number | bigint, number | bigint])[]);
        free(): void;
        /** Reads the Radix from its plain data. */
        static from(data: RadixData): Radix;
        /** Writes the Radix as plain data. */
        toJSON(): RadixData;
        /** Returns the base. */
        base(): radix.Base;
        /** Returns whether every digit is the canonical representative of its class. */
        canonical(): boolean;
        /** Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives. */
        code(): string;
        /** Returns the digits. */
        digits(): [bigint, bigint][];
        /** Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base. */
        dimension(): number;
        /** Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point. */
        distinct(level: number): number;
        /** Returns the count of words of a level, `|F|^L`. */
        fill(level: number): string;
        /** Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`. */
        static from_code(base: radix.Base, code: string | number | bigint): radix.Radix;
        /** Returns the level-`L` points in the plane, the scaled words divided by `b^L`. */
        plane(level: number): [number, number][];
        /** Returns the ring. */
        ring(): gauss.Ring;
        /** Returns the digit count `|F|`. */
        size(): number;
        /** Returns the twists. */
        twists(): [bigint, bigint][];
        /** Returns the design with the twists named by their index in the unit list, the units in turning order from one. */
        with_twists(units: ArrayLike<number>): radix.Radix;
        /** Returns the level-`L` points in exact ring coordinates scaled by `b^L`. */
        words(level: number): [bigint, bigint][];
    }
}
export declare namespace series {
    /** Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six. */
    export function basel(n: number): number;
    /** Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention. */
    export function bernoulli(count: number): [string, string][];
    /** Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums. */
    export function beta(s: number, terms: number): number;
    /** Returns the powers of two up to the limit. */
    export function binary(limit: number): Uint32Array;
    /** Returns the distinct Catalan numbers up to the limit. */
    export function catalan(limit: number): Uint32Array;
    /** Returns the mod-three rhythm of the number: zero, one, minus one. */
    export function chi3(number: number): number;
    /** Returns the mod-four rhythm of the number: zero, one, zero, minus one. */
    export function chi4(number: number): number;
    /** Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens. */
    export function chi8(number: number): number;
    /** Returns the L-series partial sum with a periodic rhythm painted on the terms. */
    export function dirichlet(s: number, rhythm: ArrayLike<number>, terms: number): number;
    /** Returns one plus one over n raised to the n, walking to the natural base. */
    export function e_partial(n: number): number;
    /** Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant. */
    export function euler_gamma_partial(n: number): number;
    /** Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit. */
    export function euler_product(s: number, limit: number): number;
    /** Returns the even numbers up to the limit. */
    export function evens(limit: number): Uint32Array;
    /** Returns the distinct Fibonacci numbers up to the limit. */
    export function fibonacci(limit: number): Uint32Array;
    /** Returns the partial harmonic sum, the reciprocals of one through the term count. */
    export function harmonic(terms: number): number;
    /** Returns the Dirichlet lambda value, one minus two to the minus s times zeta. */
    export function lambda(s: number, terms: number): number;
    /** Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four. */
    export function leibniz(n: number): number;
    /** Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x. */
    export function li(x: number): number;
    /** Returns the Mertens function at n, the Mobius values of one through n summed. */
    export function mertens(n: number): bigint;
    /** Returns the odd numbers up to the limit. */
    export function odds(limit: number): Uint32Array;
    /** Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion. */
    export function visible(limit: number, dimension: number): string;
    /** Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two. */
    export function wallis_half_pi(n: number): number;
    /** Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four. */
    export function wallis_quarter_pi(factors: number): number;
    /** Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail. */
    export function zeta(s: number, terms: number): number;
    /** The Apery constant, the value zeta takes at three. */
    export function APERY(): number;
    /** The Basel constant, pi squared over six, the value zeta takes at two. */
    export function BASEL(): number;
    /** The Catalan constant, the value the Dirichlet beta function takes at two. */
    export function CATALAN(): number;
    /** The Euler constant, the limit of the harmonic sum less the logarithm. */
    export function EULER(): number;
    /** The visible density, six over pi squared, the share of lattice pairs that are coprime. */
    export function VISIBLE(): number;
}
export declare namespace sieve {
    /** Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter. */
    export function cells(word: ArrayLike<number | bigint>, dimension: number): string;
    /** Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters. */
    export function exponent(word: ArrayLike<number | bigint>, dimension: number): number;
    /** Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet. */
    export function flat_word(side: number | bigint, levels: number): BigUint64Array;
    /** Returns the punctures the word makes, one per surviving cell at every level. */
    export function holes(word: ArrayLike<number | bigint>, dimension: number): string;
    /** Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all. */
    export function limit(word: ArrayLike<number | bigint>, dimension: number): number | undefined;
    /** Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels. */
    export function odd_word(levels: number): BigUint64Array;
    /** Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list. */
    export function punctures(word: ArrayLike<number | bigint>, dimension: number): BigUint64Array;
    /** Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out. */
    export function raster(word: ArrayLike<number | bigint>): [number, Uint8Array];
    /** Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills. */
    export function ratio(word: ArrayLike<number | bigint>, dimension: number): number;
    /** Returns the side of the word, the product of its letters' sides. */
    export function side(word: ArrayLike<number | bigint>): string;
    /** Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form. */
    export function solid_limit(): number;
    /** The limit of the plane Wallis sieve's surviving area, pi over four. */
    export function PLANE_LIMIT(): number;
}
export declare namespace spiral {
    /** Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak. */
    export function diagonal(lattice: spiral.Lattice, side: number, a: number | bigint, b: number | bigint, c: number | bigint): spiral.Diagonal;
    /** Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself. */
    export function level_of(n: number | bigint, base: number | bigint): number;
    /** Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero. */
    export function marks(mark: spiral.Mark, limit: number): Int8Array;
    /** Winds one to the top on the square spiral and lays a square tile on every cell, the snail. */
    export function snail(base: number | bigint, top: number | bigint, growth: spiral.Growth): spiral.Snail;
    /** The readout of one quadratic a k^2 + b k + c across a sheet: where it lands and how often on a prime. */
    export interface Diagonal {
        /** The count of numbers the sheet holds. */
        top: number;
        /** The count of primes among them. */
        primes: number;
        /** The primes as a share of the numbers. */
        density: number;
        /** The values of the quadratic inside the sheet, k counting up from zero. */
        values: number[];
        /** The cell of each value. */
        cells: [number, number][];
        /** Whether each value is prime. */
        hit: boolean[];
        /** The count of values that are prime. */
        hits: number;
        /** The count of primes before the first composite. */
        streak: number;
        /** The hits as a share of the values, zero when the quadratic misses the sheet. */
        share: number;
    }
    /** Which cells of the square winding grow into a tile. */
    export type Growth = "Prime" | "Every";
    export const Growth: {
        /** Returns every Growth in canonical order. */
        all(): spiral.Growth[];
    };
    /** The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right. */
    export type Lattice = "Square" | "Hex";
    export const Lattice: {
        /** Returns every Lattice in canonical order. */
        all(): spiral.Lattice[];
        /** Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across. */
        count(lattice: spiral.Lattice, side: number): number;
        /** Returns the number at a cell, one at the origin. */
        n(lattice: spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
        /** Returns the outermost ring of a sheet the odd side wide, half the side rounded down. */
        radius(lattice: spiral.Lattice, side: number): number;
        /** Returns the ring a number sits on, zero for one. */
        ring(lattice: spiral.Lattice, n: number | bigint): bigint;
        /** Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon. */
        ring_of(lattice: spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
        /** Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon. */
        xy(lattice: spiral.Lattice, n: number | bigint): [bigint, bigint];
    };
    /** What a cell is painted for. */
    export type Mark = "Prime" | "Twin" | "Squarefree" | "Mobius";
    export const Mark: {
        /** Returns every Mark in canonical order. */
        all(): spiral.Mark[];
    };
    /** The snail: every tile of the winding, the tallies, the area drawn and the box filled. */
    export interface Snail {
        /** The base every tile side is a power of. */
        base: number;
        /** Every tile, in the order one, two, three and on. */
        tiles: spiral.Tile[];
        /** The count of primes at or below the top. */
        primes: number;
        /** The count of tiles at each level, from zero up. */
        levels: number[];
        /** The sum of the tile areas, a tile counted once wherever it overlaps another. */
        area: bigint;
        /** The lower-left corner of the box the tiles fill. */
        low: [number, number];
        /** The upper-right corner of the box the tiles fill. */
        high: [number, number];
    }
    /** One tile of the snail: the number it stands for, its level, the side of its square, whether the number is prime, and the lower-left corner it is laid at. */
    export interface Tile {
        /** The number the tile stands for. */
        n: number;
        /** The level the design is grown to. */
        level: number;
        /** The side of the tile, the base raised to the level. */
        side: number;
        /** Whether the number is prime. */
        prime: boolean;
        /** The x of the lower-left corner. */
        x: number;
        /** The y of the lower-left corner. */
        y: number;
    }
}
export declare namespace zeta {
    /** The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2. */
    export function bump(u: number): number;
    /** Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step. */
    export function corrections(p: number): Float64Array;
    /** Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points. */
    export function kernel(p: number): number;
    /** Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule. */
    export function mellin(s: zeta.Complex): zeta.Complex;
    /** Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two. */
    export function novelty_main(): number;
    /** Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict. */
    export function novelty_wave(gammas: ArrayLike<number>, coef: zeta.Complex[], log_y: number): number;
    /** Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two. */
    export function psi_formula(x: number, gammas: ArrayLike<number>): number;
    /** Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each. */
    export function psi_stair(x: number): Float64Array;
    /** Returns a positive real base raised to a complex exponent. */
    export function raise(base: number, exponent: zeta.Complex): zeta.Complex;
    /** Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y. */
    export function sharp_novelty(prefix: ArrayLike<number | bigint>, y: number): number;
    /** Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y. */
    export function smoothed_novelty(phi: ArrayLike<number | bigint>, y: number, main: number): number;
    /** The t where the walk hands over from Euler-Maclaurin to Riemann-Siegel. */
    export function JOIN(): number;
    export interface ComplexData {
        /** The real part. */
        re: number;
        /** The imaginary part. */
        im: number;
    }
    /** A complex number: a real and an imaginary part. */
    export class Complex {
        /** Builds a complex number from its parts. */
        constructor(re: number, im: number);
        free(): void;
        /** Reads the Complex from its plain data. */
        static from(data: ComplexData): Complex;
        /** Writes the Complex as plain data. */
        toJSON(): ComplexData;
        /** The real part. */
        readonly re: number;
        /** The imaginary part. */
        readonly im: number;
        /** Returns the modulus. */
        abs(): number;
        /** Returns the principal argument. */
        arg(): number;
        /** Returns the exponential. */
        exp(): zeta.Complex;
        /** Returns the principal logarithm. */
        ln(): zeta.Complex;
        /** Returns a unit complex number at the given angle. */
        static turn(angle: number): zeta.Complex;
    }
    export type LineData = Record<string, unknown>;
    /** The critical line: the Bernoulli numbers and the Euler-Maclaurin weights the two engines share, built once. */
    export class Line {
        /** Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights. */
        constructor();
        free(): void;
        /** Reads the Line from its plain data. */
        static from(data: LineData): Line;
        /** Writes the Line as plain data. */
        toJSON(): LineData;
        /** Counts the zeros on the line below t. */
        count(t: number): number;
        /** Returns Z(t) from the Euler-Maclaurin value turned onto the real axis. */
        exact(t: number): number;
        /** Returns the n-th Gram point, where theta is n pi, by Newton from the right. */
        gram(n: number | bigint): number;
        /** Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections. */
        maclaurin(t: number): zeta.Complex;
        /** Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump. */
        novelty_coefficients(gammas: ArrayLike<number>): zeta.Complex[];
        /** Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s. */
        pair(s: zeta.Complex): [zeta.Complex, zeta.Complex];
        /** Returns zeta on the line and Z(t) together, from the engine that serves the t. */
        point(t: number): [zeta.Complex, number];
        /** Returns the largest gap between the two engines over the t range on a grid. */
        seam(t0: number, t1: number, steps: number): number;
        /** Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections. */
        siegel(t: number): number;
        /** Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten. */
        theta(t: number): number;
        /** Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above. */
        z(t: number): number;
        /** Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth. */
        zeros(count: number): Float64Array;
    }
}
