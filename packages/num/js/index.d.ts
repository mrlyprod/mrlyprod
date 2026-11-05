export { default, initSync } from "./pkg/mrly_num_js.js";

export type Color = [number, number, number, number];
export interface Tensor {
    shape: number[];
    data: Uint8Array | Uint16Array | Uint32Array | Int32Array;
}
export interface Cell {
    shape: number[];
    types: Uint8Array | Uint16Array | Uint32Array | Int32Array;
    colors?: Uint8Array;
    tags?: Uint8Array | Uint16Array | Uint32Array | Int32Array;
}
export interface Cell6d {
    cell: Cell;
    projection: string;
    orientation: string;
    start: number;
}
export interface Image {
    shape: [number, number];
    colors: Uint8Array<ArrayBuffer>;
}
export interface ColorData {
    r: number;
    g: number;
    b: number;
    a: number;
}
export interface TensorData {
    shape: number[];
    data: { U8: number[] } | { U16: number[] } | { U32: number[] } | { I32: number[] };
}
export interface CellData {
    types: TensorData;
    colors?: number[][];
    tags?: TensorData;
}
export interface Cell6dData {
    cell: { cell: CellData };
    projection: string;
    orientation: string;
    start: number;
}
export class Rng {
    constructor(seed: number | bigint | string);
    free(): void;
    unit(): number;
    below(n: number): number;
    range(lo: number, hi: number): number;
    boolean(): boolean;
    chance(p: number): boolean;
    sample_indices(length: number, amount: number): Uint32Array;
    choice<T>(items: ArrayLike<T>): T;
    shuffle<T>(items: T[]): void;
}
export declare namespace apollonian {
    export function flat(name: string, cap: number | bigint): Float64Array;
    export function form(u: ArrayLike<number | bigint>, v: ArrayLike<number | bigint>): string;
    export function frame(p: apollonian.Packing): Float64Array;
    export function grow(name: string, cap: number | bigint): apollonian.Packing;
    export function is_ford(c: apollonian.Circle): boolean;
    export function on_line(c: apollonian.Circle): boolean;
    export function reflect(q: apollonian.Circle[], at: number): apollonian.Circle;
    export function root(name: string): apollonian.Circle[];
    export function shadow(p: apollonian.Packing, order: number): apollonian.Shadow;
    export function sound(q: apollonian.Circle[]): boolean;
    export function swap(q: apollonian.Circle[], at: number): apollonian.Circle[];
    export function touches(p: apollonian.Packing): apollonian.Touch[];
    export function CIRCLE_CAP(): number;
    export function CURVATURE_CAP(): bigint;
    export function ORDER_CAP(): number;
    export function ROOTS(): string[];
    export function STRIDE(): number;
    export interface CircleData {
        k: number;
        x: number;
        y: number;
    }
    export class Circle {
        private constructor();
        free(): void;
        static from(data: CircleData): Circle;
        toJSON(): CircleData;
        get k(): bigint;
        set k(value: number | bigint);
        get x(): bigint;
        set x(value: number | bigint);
        get y(): bigint;
        set y(value: number | bigint);
        centre(): [number, number] | undefined;
        is_line(): boolean;
        radius(): number | undefined;
    }
    export interface Packing {
        root: apollonian.CircleData[];
        strip: boolean;
        cap: number;
        circles: apollonian.CircleData[];
        quads: number;
        broken: number;
        strayed: number;
    }
    export interface Shadow {
        order: number;
        reach: number;
        covered: boolean;
        nodes: number;
        touched: number;
        missed: number;
        offford: number;
        bright: bigint;
        want: bigint;
    }
    export interface Touch {
        num: number;
        den: number;
        k: number;
    }
}
export declare namespace blend {
    export function add(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    export function cauchy(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    export function characteristic(coefficients: ([string | number | bigint, string | number | bigint])[]): [string, string][];
    export function decimate(a: (string | number | bigint)[], step: number, offset: number): string[];
    export function delta(a: (string | number | bigint)[]): string[];
    export function growth(coefficients: ([string | number | bigint, string | number | bigint])[]): number;
    export function hadamard(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
    export function recurrence(terms: (string | number | bigint)[]): [string, string][] | undefined;
    export function scale(a: (string | number | bigint)[], factor: string | number | bigint): string[];
    export function shift(a: (string | number | bigint)[], count: number): string[];
    export function sigma(a: (string | number | bigint)[]): string[];
    export function sub(a: (string | number | bigint)[], b: (string | number | bigint)[]): string[];
}
export declare namespace boolean {
    export function is_balanced(code: string | number | bigint, n: number): boolean;
    export function nonlinearity(code: string | number | bigint, n: number): bigint;
    export function sac(code: string | number | bigint, n: number): number;
    export function walsh_spectrum(code: string | number | bigint, n: number): BigInt64Array;
}
export declare namespace chladni {
    export function ladder(cap: number): chladni.Rung[];
    export function lines(field: ArrayLike<number>, side: number): Float64Array;
    export function plate(n: number, m: number, sign: number, size: number): Float64Array;
    export function shake(field: ArrayLike<number>, side: number, grains: ArrayLike<number>, from: number | bigint, steps: number, seed: number | bigint): Float32Array;
    export interface Rung {
        n: number;
        m: number;
        sign: number;
        pitch: number;
    }
}
export declare namespace factor {
    export function aliquot(number: number): number;
    export function coprime(a: number, b: number): boolean;
    export function divisors(number: number | bigint): BigUint64Array;
    export function factorial(number: number): string;
    export function factorize(number: number): [number, number][];
    export function factorize_wide(number: number | bigint): [bigint, number][];
    export function gcd(a: string | number | bigint, b: string | number | bigint): string;
    export function lcm(a: number, b: number): number;
    export function mobius(number: number): number;
    export function mobius_sieve(limit: number): Int8Array;
    export function radical(number: number): number;
    export function reduce(numerator: string | number | bigint, denominator: string | number | bigint): [string, string];
    export function sigma(number: number, power: number): string;
    export function squarefree(number: number): boolean;
    export function totient(number: number): number;
    export function totients(n: number): BigUint64Array;
    export function twisted(number: number, rhythm: ArrayLike<number>): bigint;
}
export declare namespace fft {
    export function convolve(field: ArrayLike<number>, kernel: ArrayLike<number>, size: number): Float64Array;
    export function convolve_with(field: ArrayLike<number>, kernel_re: ArrayLike<number>, kernel_im: ArrayLike<number>, size: number): Float64Array;
    export function embed_kernel(mask: ArrayLike<number>, side: number, size: number): Float64Array;
    export function log_spectrum(field: ArrayLike<number>, size: number): Float64Array;
    export function magnitude_spectrum(field: ArrayLike<number>, size: number): Float64Array;
    export function peak_ring(profile: ArrayLike<number>): number;
    export function peak_wavelength(profile: ArrayLike<number>, size: number): number;
    export function radial_profile(spectrum: ArrayLike<number>, size: number): Float64Array;
    export function transform(field: ArrayLike<number>, size: number): [Float64Array, Float64Array];
}
export declare namespace gauss {
    export function classes(ring: gauss.Ring, bound: number | bigint): [bigint, bigint][];
    export function peak(ring: gauss.Ring, limit: number): [number, number];
    export function shells(ring: gauss.Ring, limit: number): Uint32Array;
    export interface Census {
        points: number;
        primes: number;
        split: number;
        inert: number;
        ramified: number;
        units: number;
        composites: number;
        density: number;
    }
    export type Class = "Zero" | "Unit" | "Ramified" | "Split" | "Inert" | "Composite";
    export const Class: {
        prime(class_: gauss.Class): boolean;
        word(class_: gauss.Class): string;
    };
    export type Ring = "Gaussian" | "Eisenstein";
    export const Ring: {
        associates(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint][];
        canon(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        conjugate(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        count(ring: gauss.Ring, radius: number | bigint): number;
        div_rem(ring: gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [[bigint, bigint], [bigint, bigint]];
        fate(ring: gauss.Ring, n: number | bigint): gauss.Class;
        gaussian_gcd(ring: gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [bigint, bigint];
        inert(ring: gauss.Ring, p: number | bigint): boolean;
        mul(ring: gauss.Ring, arg1: [number | bigint, number | bigint], arg2: [number | bigint, number | bigint]): [bigint, bigint];
        named(name: string): gauss.Ring | undefined;
        nearest(ring: gauss.Ring, x: number, y: number): [bigint, bigint];
        norm(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint;
        place(ring: gauss.Ring, a: number | bigint, b: number | bigint): [number, number];
        ramified(ring: gauss.Ring): bigint;
        reach(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint;
        symmetry(ring: gauss.Ring): number;
        top(ring: gauss.Ring, radius: number | bigint): bigint;
        turn(ring: gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
        units(ring: gauss.Ring): number;
        whole(ring: gauss.Ring, a: number | bigint, b: number | bigint): bigint | undefined;
    };
    export type WindowData = Record<string, unknown>;
    export class Window {
        constructor(ring: gauss.Ring, radius: number | bigint);
        free(): void;
        static from(data: WindowData): Window;
        toJSON(): WindowData;
        census(): gauss.Census;
        class(a: number | bigint, b: number | bigint): gauss.Class;
        holds(a: number | bigint, b: number | bigint): boolean;
        points(): [bigint, bigint][];
        radius(): bigint;
        ring(): gauss.Ring;
    }
}
export declare namespace kummer {
    export function carry(prime: number | bigint, top: number | bigint, bottom: number | bigint): kummer.Carry;
    export function columns(k: number | bigint, prime: number | bigint, top: number | bigint, bottom: number | bigint): kummer.Column[];
    export function count(prime: number | bigint, top: number | bigint, bottom: number | bigint, level: number): string;
    export function member(k: number | bigint, prime: number | bigint, top: number | bigint, bottom: number | bigint): boolean;
    export function valuation(k: number | bigint, prime: number | bigint, top: number | bigint, bottom: number | bigint): number;
    export function witness(prime: number | bigint, top: number | bigint, bottom: number | bigint): [bigint, bigint] | undefined;
    export function MOST(): bigint;
    export function TABLE(): bigint;
    export interface Carry {
        prime: number;
        top: number;
        bottom: number;
        states: [number, number][];
        moves: ((number | undefined)[])[];
        closes: boolean[];
    }
    export interface Column {
        digit: number;
        state: [number, number];
        left: number;
        right: number;
        next?: [number, number];
    }
}
export declare namespace lattice {
    export function coprime_pairs(n: number): bigint;
    export function farey(order: number): lattice.Node[];
    export function grid(n: number): lattice.Node2d[];
    export function new_nodes(n: number): bigint;
    export function pi_estimate(n: number): number;
    export function recovered(n: number, dimension: number): number;
    export function visible_density(dimension: number): number;
    export function zeta_factor(dimension: number): number | undefined;
    export function zeta_whole(s: number): number;
    export interface Node {
        num: number;
        den: number;
        brightness: number;
    }
    export interface Node2d {
        x: lattice.Node;
        y: lattice.Node;
        brightness: number;
    }
}
export declare namespace morse {
    export function boundary(word: ArrayLike<number>): Uint8Array;
    export function difference(a: ArrayLike<number>, b: ArrayLike<number>): Uint8Array;
    export function digits(length: number): Uint8Array;
    export function doubling(length: number): Uint8Array;
    export function faults(a: ArrayLike<number>, b: ArrayLike<number>): number;
    export function fold(grid: ArrayLike<number>, side: number, number: number): morse.Fold;
    export function letter(place: number | bigint): number;
    export function lift(kind: morse.Lift, side: number): Uint8Array;
    export function mask(a: ArrayLike<number>, b: ArrayLike<number>): Uint8Array;
    export function power(tile: ArrayLike<number>, number: number, level: number): Uint8Array;
    export function repeat(tile: ArrayLike<number>, number: number, side: number): Uint8Array;
    export function runs(word: ArrayLike<number>): Uint32Array;
    export function signs(types: ArrayLike<number>): Uint8Array;
    export function stage(rounds: number): Uint8Array;
    export function substitution(length: number): Uint8Array;
    export function upsample(grid: ArrayLike<number>, side: number, scale: number): Uint8Array;
    export function LIFTS(): morse.Lift[];
    export interface Fold {
        tile: number[];
        number: number;
        level: number;
        folds: boolean;
        faults: number;
        first?: [number, number];
    }
    export type Lift = "Parity" | "And" | "Xor" | "Sum";
    export const Lift: {
        all(): morse.Lift[];
        at(lift: morse.Lift, i: number | bigint, j: number | bigint): number;
        formula(lift: morse.Lift): string;
    };
}
export declare namespace prime {
    export function chart(top: number, bins: number): prime.Reading[];
    export function flags(limit: number): boolean[];
    export function goldbach(number: number): number;
    export function goldbach_record(top: number): Uint32Array;
    export function is_prime(number: number): boolean;
    export function pile(number: number | bigint): prime.Pile;
    export function prime_count(n: number): number;
    export function prime_from(number: number): number;
    export function primes(limit: number): Uint32Array;
    export function rectangles(number: number): [number, number][];
    export function splits(number: number): [number, number][];
    export function squares(number: number): [number, number] | undefined;
    export function study(limit: number): prime.Prime[];
    export interface Pile {
        number: number;
        factors: [number, number][];
        prime: boolean;
        rectangles: [number, number][];
    }
    export interface Prime {
        value: number;
        index: number;
        gap: number;
        twin: boolean;
        squares?: [number, number];
    }
    export interface Reading {
        x: number;
        pi: number;
        ratio: number;
        li: number;
    }
    export type SieveData = Record<string, unknown>;
    export class Sieve {
        constructor(limit: number);
        free(): void;
        static from(data: SieveData): Sieve;
        toJSON(): SieveData;
        count(): number;
        done(): boolean;
        finish(): void;
        rank(): number;
        step(): number;
        struck(): number;
        types(): Uint8Array;
    }
}
export declare namespace readings {
    export function arcs(design: readings.Design, cut: number): readings.Arc[];
    export function battery(stream: ArrayLike<number>): readings.Battery;
    export function bias(stream: ArrayLike<number>, exponents: ArrayLike<number>): BigInt64Array;
    export function designs(): readings.Design[];
    export function model(design: readings.Design, cut: number, exponents: ArrayLike<number>): Float64Array;
    export function name(design: readings.Design): string;
    export function near_heads(design: readings.Design, reading: readings.Reading): readings.Head[];
    export function sign(design: readings.Design, n: number | bigint): number;
    export function stream(design: readings.Design, reading: readings.Reading, count: number): Int8Array;
    export function READINGS(): readings.Reading[];
    export interface Arc {
        num: number;
        den: number;
        eta: number;
        amplitude_re: number;
        amplitude_im: number;
    }
    export interface Battery {
        count: number;
        mean: number;
        shift: number[];
        dilation: readings.Dilation[];
        kernel: number[];
        square: number;
    }
    export interface Design {
        base: number;
        mask: number;
        rudin: boolean;
    }
    export const Design: {
        digits(base: number, mask: number): readings.Design;
        rudin(): readings.Design;
        ternary(): readings.Design;
        thue(): readings.Design;
    };
    export interface Dilation {
        scale: number;
        offset: number;
        correlation: number;
    }
    export interface Head {
        kernel: boolean;
        scale: number;
        offset: number;
        level: number;
        shift: number;
        tail: number;
        sign: number;
        exact: boolean;
    }
    export type Reading = "N" | "Square" | "SquarePlusN" | "Cube" | "Primes";
    export const Reading: {
        all(): readings.Reading[];
        formula(reading: readings.Reading): string;
    };
}
export declare namespace series {
    export function basel(n: number): number;
    export function bernoulli(count: number): [string, string][];
    export function beta(s: number, terms: number): number;
    export function binary(limit: number): Uint32Array;
    export function catalan(limit: number): Uint32Array;
    export function chi3(number: number): number;
    export function chi4(number: number): number;
    export function chi8(number: number): number;
    export function dirichlet(s: number, rhythm: ArrayLike<number>, terms: number): number;
    export function e_partial(n: number): number;
    export function euler_gamma_partial(n: number): number;
    export function euler_product(s: number, limit: number): number;
    export function evens(limit: number): Uint32Array;
    export function fibonacci(limit: number): Uint32Array;
    export function harmonic(terms: number): number;
    export function lambda(s: number, terms: number): number;
    export function leibniz(n: number): number;
    export function li(x: number): number;
    export function mertens(n: number): bigint;
    export function odds(limit: number): Uint32Array;
    export function visible(limit: number, dimension: number): string;
    export function wallis_half_pi(n: number): number;
    export function wallis_quarter_pi(factors: number): number;
    export function zeta(s: number, terms: number): number;
    export function APERY(): number;
    export function BASEL(): number;
    export function CATALAN(): number;
    export function EULER(): number;
    export function VISIBLE(): number;
}
export declare namespace sieve {
    export function cells(word: ArrayLike<number | bigint>, dimension: number): string;
    export function exponent(word: ArrayLike<number | bigint>, dimension: number): number;
    export function flat_word(side: number | bigint, levels: number): BigUint64Array;
    export function holes(word: ArrayLike<number | bigint>, dimension: number): string;
    export function limit(word: ArrayLike<number | bigint>, dimension: number): number | undefined;
    export function odd_word(levels: number): BigUint64Array;
    export function parity_product(dimension: number, odd: boolean): number;
    export function punctures(word: ArrayLike<number | bigint>, dimension: number): BigUint64Array;
    export function raster(word: ArrayLike<number | bigint>): [number, Uint8Array];
    export function ratio(word: ArrayLike<number | bigint>, dimension: number): number;
    export function row_fill(profile: ArrayLike<number | bigint>, side: number | bigint): string;
    export function row_law(profile: ArrayLike<number | bigint>): sieve.Row;
    export function row_profile(code: string | number | bigint, dimension: number): BigUint64Array;
    export function row_settle(profile: ArrayLike<number | bigint>, stops: ArrayLike<number>, even: boolean): Float64Array;
    export function side(word: ArrayLike<number | bigint>): string;
    export function solid_limit(): number;
    export function PLANE_LIMIT(): number;
    export interface Row {
        profile: number[];
        drift: [number, number];
        correction: [number, number];
        roots: zeta.Complex[];
        constant: number;
        closed: string;
        reflection?: number;
        mirror: number;
        parity?: [boolean, number];
    }
}
export declare namespace spiral {
    export function diagonal(lattice: spiral.Lattice, side: number, a: number | bigint, b: number | bigint, c: number | bigint): spiral.Diagonal;
    export function level_of(n: number | bigint, base: number | bigint): number;
    export function marks(mark: spiral.Mark, limit: number): Int8Array;
    export function snail(base: number | bigint, top: number | bigint, growth: spiral.Growth): spiral.Snail;
    export interface Diagonal {
        top: number;
        primes: number;
        density: number;
        values: number[];
        cells: [number, number][];
        hit: boolean[];
        hits: number;
        streak: number;
        share: number;
    }
    export type Growth = "Prime" | "Every";
    export const Growth: {
        all(): spiral.Growth[];
    };
    export type Lattice = "Square" | "Hex";
    export const Lattice: {
        all(): spiral.Lattice[];
        count(lattice: spiral.Lattice, side: number): number;
        n(lattice: spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
        radius(lattice: spiral.Lattice, side: number): number;
        ring(lattice: spiral.Lattice, n: number | bigint): bigint;
        ring_of(lattice: spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
        xy(lattice: spiral.Lattice, n: number | bigint): [bigint, bigint];
    };
    export type Mark = "Prime" | "Twin" | "Squarefree" | "Mobius";
    export const Mark: {
        all(): spiral.Mark[];
    };
    export interface Snail {
        base: number;
        tiles: spiral.Tile[];
        primes: number;
        levels: number[];
        area: bigint;
        low: [number, number];
        high: [number, number];
    }
    export interface Tile {
        n: number;
        level: number;
        side: number;
        prime: boolean;
        x: number;
        y: number;
    }
}
export declare namespace zeta {
    export function bump(u: number): number;
    export function corrections(p: number): Float64Array;
    export function kernel(p: number): number;
    export function mellin(s: zeta.Complex): zeta.Complex;
    export function novelty_main(): number;
    export function novelty_wave(gammas: ArrayLike<number>, coef: zeta.Complex[], log_y: number): number;
    export function psi_curve(gammas: ArrayLike<number>, samples: ArrayLike<number>): Float64Array;
    export function psi_formula(x: number, gammas: ArrayLike<number>): number;
    export function psi_gap(x: number, gammas: ArrayLike<number>): number;
    export function psi_stair(x: number): Float64Array;
    export function raise(base: number, exponent: zeta.Complex): zeta.Complex;
    export function sharp_novelty(prefix: ArrayLike<number | bigint>, y: number): number;
    export function smoothed_novelty(phi: ArrayLike<number | bigint>, y: number, main: number): number;
    export function JOIN(): number;
    export interface Complex {
        re: number;
        im: number;
    }
    export const Complex: {
        abs(z: zeta.Complex): number;
        arg(z: zeta.Complex): number;
        default(): zeta.Complex;
        exp(z: zeta.Complex): zeta.Complex;
        ln(z: zeta.Complex): zeta.Complex;
        "new"(re: number, im: number): zeta.Complex;
        turn(angle: number): zeta.Complex;
    };
    export type LineData = Record<string, unknown>;
    export class Line {
        constructor();
        free(): void;
        static from(data: LineData): Line;
        toJSON(): LineData;
        count(t: number): number;
        static default(): zeta.Line;
        exact(t: number): number;
        gram(n: number | bigint): number;
        maclaurin(t: number): zeta.Complex;
        novelty_coefficients(gammas: ArrayLike<number>): zeta.Complex[];
        pair(s: zeta.Complex): [zeta.Complex, zeta.Complex];
        point(t: number): [zeta.Complex, number];
        seam(t0: number, t1: number, steps: number): number;
        siegel(t: number): number;
        theta(t: number): number;
        z(t: number): number;
        zeros(count: number): Float64Array;
    }
}
