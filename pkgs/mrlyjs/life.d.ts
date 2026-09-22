export { default, initSync } from "./pkg/life/mrlyjs_life.js";

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
    /** Draws one item of the array, the same draw as Rust's choice. */
    choice<T>(items: ArrayLike<T>): T;
    /** Shuffles the array in place, the same permutation as Rust's shuffle. */
    shuffle<T>(items: T[]): void;
}
/** Returns whether a rule is affine, its algebraic degree at most one. */
export function affine(rule: number): boolean;
/** Runs a seed under a config until it fixes, loops or times out, recording every generation. */
export function animate(seed: Cell, config: Config): Life;
/** Returns the mean fraction of sites changed between consecutive grids. */
export function churn(grids: Cell[]): number;
/** Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`. */
export function corner_bits(rule: number): Uint8Array;
/** Returns the sequence up to max_neighbors, keeping zeros and ones only on request. */
export function counts(seq: Source, max_neighbors: number, include_zeros: boolean, include_ones: boolean): Uint32Array;
/** Crops a frame sequence to the centred square bounding every cell ever alive. */
export function crop(grids: Cell[]): Cell[];
/** Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order. */
export function cube_orbit(rule: number): Uint8Array;
/** Builds the base-2 design mask a code names at an odd side grown to the given Kronecker */
export function design_mask(dimension: number, code: string | number | bigint, number: number, level: number): Tensor;
/** Returns the grid's binary Shannon entropy in millibits. */
export function entropy(grid: Cell): bigint;
/** Renders grids to white-on-black PNG bytes at a pixel scale. */
export function frames(grids: Cell[], scale: number): Uint8Array[];
/** Returns the base-2 plane design a rule's single seed draws, or None when it draws none. */
export function gasket(rule: number): string | undefined;
/** Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`. */
export function genus(rule: number): string;
/** Renders a whole run's cumulative-visit heatmap frames with the heat ramp. */
export function heatmap(grids: Cell[], scale: number): Uint8Array[];
/** Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation. */
export function history(row: ArrayLike<number>, rule: number, steps: number, wrap: boolean): Tensor;
/** Returns Langton's lambda, the popcount over eight. */
export function lambda(rule: number): number;
/** Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension. */
export function lattice_index(mask: Tensor): number;
/** Returns the offsets a mask's filled sites take from its centre, the centre itself dropped. */
export function mask_offsets(mask: Tensor): BigInt64Array[];
/** Builds the 3 by 3 Moore mask, every site on but the center. */
export function moore(): Cell;
/** Renders grids into one looping black-on-white gif, the delay in hundredths of a second. */
export function movie(grids: Cell[], scale: number, delay: number): Uint8Array;
/** Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary. */
export function next_grid(cell: Cell, birth: ArrayLike<number>, survive: ArrayLike<number>, mask: Tensor, boundary: Boundary): Cell;
/** Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order. */
export function npn_class(rule: number): Uint8Array;
/** Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone. */
export function outer_totalistic(rule: number): [Uint32Array, Uint32Array] | undefined;
/** Returns the count of neighbourhoods a rule sends to one. */
export function popcount(rule: number): number;
/** Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core. */
export function reversible(rule: number): boolean;
/** Returns the GF(2) algebraic degree of a rule, minus one for the zero rule. */
export function rule_degree(rule: number): number;
/** Returns the design name a rule carries, `bang dim 3, code <rule>`. */
export function rule_name(rule: number): string;
/** Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window. */
export function single_seed(rule: number, steps: number): Tensor;
/** Advances one row one generation, a constant-0 boundary unless the edges wrap. */
export function step(row: ArrayLike<number>, rule: number, wrap: boolean): Uint8Array;
/** Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set. */
export function surjective(rule: number): boolean;
/** Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there. */
export function tessellate(grids: Cell[], min_canvas: number): Cell[];
/** Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order. */
export function wolfram_class(rule: number): Uint8Array;
/** The edge policy of a life grid. */
export type Boundary = "Constant" | "Wrap";
export const Boundary: {
    /** Returns every Boundary in canonical order. */
    all(): Boundary[];
    /** Returns whether the edges wrap. */
    wrap(boundary: Boundary): boolean;
};
export interface ConfigData {
    /** The neighborhood mask. */
    mask: { cell: CellData };
    /** The neighbor counts that create a cell. */
    birth: CountsData;
    /** The neighbor counts that keep a cell. */
    survive: CountsData;
    /** The edge policy. */
    boundary: Boundary;
    /** The generation cap. */
    max_generations: number;
    /** The tiling factor applied to the seed. */
    grid_size: number;
    /** The dead border added around the seed. */
    padding: number;
}
/** The rulebook of a life run. */
export class Config {
    /** Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding. */
    constructor(mask: Cell, birth: Counts, survive: Counts);
    free(): void;
    /** Reads the Config from its plain data. */
    static from(data: ConfigData): Config;
    /** Writes the Config as plain data. */
    toJSON(): ConfigData;
    /** The neighborhood mask. */
    get mask(): Cell;
    set mask(value: Cell);
    /** The neighbor counts that create a cell. */
    get birth(): Counts;
    set birth(value: Counts);
    /** The neighbor counts that keep a cell. */
    get survive(): Counts;
    set survive(value: Counts);
    /** The edge policy. */
    get boundary(): Boundary;
    set boundary(value: Boundary);
    /** The generation cap. */
    get max_generations(): number;
    set max_generations(value: number);
    /** The tiling factor applied to the seed. */
    get grid_size(): number;
    set grid_size(value: number);
    /** The dead border added around the seed. */
    get padding(): number;
    set padding(value: number);
    /** Returns the largest neighbor count the mask can reach. */
    budget(): number;
    /** Resolves the birth and survive counts against the mask's budget. */
    counts(): [Uint32Array, Uint32Array];
}
export type CountsData = { List: number[] } | { Drawn: { seq: SourceData; zeros: boolean; ones: boolean } };
/** The neighbor counts one side of a rule fires on. */
export class Counts {
    private constructor();
    free(): void;
    /** Reads the Counts from its plain data. */
    static from(data: CountsData): Counts;
    /** Writes the Counts as plain data. */
    toJSON(): CountsData;
    /** Builds the counts a sequence lays down, keeping zeros and ones on request. */
    static drawn(seq: Source, zeros: boolean, ones: boolean): Counts;
    /** Spells the counts outright. */
    static list(counts: ArrayLike<number>): Counts;
    /** Returns the counts, a drawn side resolved against the mask's neighbor budget. */
    values(budget: number): Uint32Array;
}
/** The ending of a life run. */
export type Fate = "Dead" | "Alive" | "Loop" | "Timeout";
export const Fate: {
    /** Returns every Fate in canonical order. */
    all(): Fate[];
};
export interface LifeData {
    /** Every generation in order. */
    grids: { cell: CellData }[];
    /** The run's ending. */
    fate: Fate;
    /** The number of recorded generations. */
    count: number;
    /** The cycle length when the fate is a loop, else zero. */
    loop_length: number;
}
/** The recorded run of one seed. */
export class Life {
    private constructor();
    free(): void;
    /** Reads the Life from its plain data. */
    static from(data: LifeData): Life;
    /** Writes the Life as plain data. */
    toJSON(): LifeData;
    /** Every generation in order. */
    get grids(): Cell[];
    set grids(value: Cell[]);
    /** The run's ending. */
    get fate(): Fate;
    set fate(value: Fate);
    /** The number of recorded generations. */
    get count(): number;
    set count(value: number);
    /** The cycle length when the fate is a loop, else zero. */
    get loop_length(): number;
    set loop_length(value: number);
    /** Returns the final grid, or None when the run is empty. */
    last(): Cell | undefined;
}
export interface RuleData {
    /** The kind word. */
    kind: string;
    /** The neighbor counts that create a cell, listed or drawn from a sequence. */
    birth: CountsData;
    /** The neighbor counts that keep a cell, listed or drawn from a sequence. */
    survive: CountsData;
    /** Whether the edge wraps, false unless said. */
    wrap: boolean;
}
/** A life rule: the birth and survival counts and whether the edge wraps. */
export class Rule {
    /** Builds a rule from its counts and edge policy, listed counts folded to a sorted set. */
    constructor(birth: Counts, survive: Counts, wrap: boolean);
    free(): void;
    /** Reads the Rule from its plain data. */
    static from(data: RuleData): Rule;
    /** Writes the Rule as plain data. */
    toJSON(): RuleData;
    /** The neighbor counts that create a cell, listed or drawn from a sequence. */
    get birth(): Counts;
    set birth(value: Counts);
    /** The neighbor counts that keep a cell, listed or drawn from a sequence. */
    get survive(): Counts;
    set survive(value: Counts);
    /** Whether the edge wraps, false unless said. */
    get wrap(): boolean;
    set wrap(value: boolean);
    /** Returns the edge policy the rule runs under. */
    boundary(): Boundary;
    /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
    checked(): Rule;
    /** Builds a life config running this rule over a neighborhood mask. */
    config(mask: Cell): Config;
    /** Reads a filename back into the value, or an error. */
    static from_file(text: string): Rule;
    /** Reads a JSON object into its canonical value, or an error naming the broken key. */
    static from_json(text: string): Rule;
    /** Reads a path and query string back into the value, or an error. */
    static from_url(text: string): Rule;
    /** Reads the rule out of a life config. */
    static of(config: Config): Rule;
    /** Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back. */
    to_file(): string;
    /** Prints the first eight hex digits of the sha256 of the canonical JSON. */
    to_id(): string;
    /** Prints the canonical JSON object. */
    to_json(): string;
    /** Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back. */
    to_mrly(): string;
    /** Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back. */
    to_url(): string;
}
export type SourceData = "Evens" | "Odds" | { Random: number } | "Primes" | "Binary" | "Fibonacci" | "GridSquares" | "CarpetFills" | "CarpetVoids" | "NetFills" | "NetVoids" | "TreeFills" | "TreeVoids" | "VoidFills" | "VoidVoids" | "PointFills" | "PointVoids" | "DustFills" | "DustVoids" | "LineFills" | "LineVoids" | "StarFills" | "StarVoids" | { CodeFills: bigint } | { CodeVoids: bigint };
/** A named source of neighbor-count values. */
export class Source {
    private constructor();
    free(): void;
    /** Reads the Source from its plain data. */
    static from(data: SourceData): Source;
    /** Writes the Source as plain data. */
    toJSON(): SourceData;
    /** Returns every fixed sequence, the seeded and coded families excluded. */
    static all(): Source[];
    /** Returns the seventeen mrly design families: the grid, the four classics and their antis. */
    static designs(): Source[];
    /** Returns whether the sequence is a seeded random draw. */
    is_random(): boolean;
    /** Returns the sequence's parseable name, the one string that regenerates it. */
    name(): string;
    /** Returns the six number sequences, the random one listed under seed zero. */
    static numbers(): Source[];
    /** Returns the sequence's OEIS id, or None off the encyclopedia. */
    oeis(): string | undefined;
    /** Parses a sequence name back to its source. */
    static parse(name: string): Source;
    /** Reads a canonical name off the front of the text, returning the tail left over. */
    static read(text: string): [Source, string] | undefined;
}
export declare namespace elementary {
    /** Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell. */
    export function output(rule: number, l: number, c: number, r: number): number;
}
export declare namespace render {
    /** Renders one grid to white-on-black PNG bytes at a pixel scale. */
    export function frame(grid: Cell, scale: number): Uint8Array;
}
export declare namespace source {
    /** Generates the sequence's values up to the limit. */
    export function sequence(seq: Source, limit: number): Uint32Array;
}
