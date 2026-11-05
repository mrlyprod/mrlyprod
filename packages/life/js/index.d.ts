export { default, initSync } from "./pkg/mrly_life_js.js";

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
    projection: "Iso" | "Pro" | "Cut";
    orientation: "Horizontal" | "Vertical";
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
    projection: "Iso" | "Pro" | "Cut";
    orientation: "Horizontal" | "Vertical";
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
export function affine(rule: number): boolean;
export function animate(seed: Cell, config: Config): Life;
export function churn(grids: Cell[]): number;
export function corner_bits(rule: number): Uint8Array;
export function counts(seq: Source, max_neighbors: number, include_zeros: boolean, include_ones: boolean): Uint32Array;
export function crop(grids: Cell[]): Cell[];
export function cube_orbit(rule: number): Uint8Array;
export function design_mask(dimension: number, code: string | number | bigint, number: number, level: number): Tensor;
export function entropy(grid: Cell): bigint;
export function frames(grids: Cell[], scale: number): Image[];
export function gasket(rule: number): string | undefined;
export function genus(rule: number): string;
export function heatmap(grids: Cell[], scale: number): Image[];
export function history(row: ArrayLike<number>, rule: number, steps: number, wrap: boolean): Tensor;
export function lambda(rule: number): number;
export function lattice_index(mask: Tensor): number;
export function mask_offsets(mask: Tensor): BigInt64Array[];
export function moore(): Cell;
export function next_grid(cell: Cell, birth: ArrayLike<number>, survive: ArrayLike<number>, mask: Tensor, boundary: Boundary): Cell;
export function npn_class(rule: number): Uint8Array;
export function outer_totalistic(rule: number): [Uint32Array, Uint32Array] | undefined;
export function popcount(rule: number): number;
export function reversible(rule: number): boolean;
export function rule_degree(rule: number): number;
export function rule_name(rule: number): string;
export function single_seed(rule: number, steps: number): Tensor;
export function step(row: ArrayLike<number>, rule: number, wrap: boolean): Uint8Array;
export function surjective(rule: number): boolean;
export function tessellate(grids: Cell[], min_canvas: number): Cell[];
export function wolfram_class(rule: number): Uint8Array;
export type Boundary = "Constant" | "Wrap";
export const Boundary: {
    all(): Boundary[];
    wrap(boundary: Boundary): boolean;
};
export interface ConfigData {
    mask: { cell: CellData };
    birth: CountsData;
    survive: CountsData;
    boundary: Boundary;
    max_generations: number;
    grid_size: number;
    padding: number;
}
export class Config {
    constructor(mask: Cell, birth: Counts, survive: Counts);
    free(): void;
    static from(data: ConfigData): Config;
    toJSON(): ConfigData;
    get mask(): Cell;
    set mask(value: Cell);
    get birth(): Counts;
    set birth(value: Counts);
    get survive(): Counts;
    set survive(value: Counts);
    get boundary(): Boundary;
    set boundary(value: Boundary);
    get max_generations(): number;
    set max_generations(value: number);
    get grid_size(): number;
    set grid_size(value: number);
    get padding(): number;
    set padding(value: number);
    budget(): number;
    counts(): [Uint32Array, Uint32Array];
}
export type CountsData = { List: number[] } | { Drawn: { seq: SourceData; zeros: boolean; ones: boolean } };
export class Counts {
    private constructor();
    free(): void;
    static from(data: CountsData): Counts;
    toJSON(): CountsData;
    static drawn(seq: Source, zeros: boolean, ones: boolean): Counts;
    static list(counts: ArrayLike<number>): Counts;
    values(budget: number): Uint32Array;
}
export type Fate = "Dead" | "Alive" | "Loop" | "Timeout";
export const Fate: {
    all(): Fate[];
};
export interface LifeData {
    grids: { cell: CellData }[];
    fate: Fate;
    count: number;
    loop_length: number;
}
export class Life {
    private constructor();
    free(): void;
    static from(data: LifeData): Life;
    toJSON(): LifeData;
    get grids(): Cell[];
    set grids(value: Cell[]);
    get fate(): Fate;
    set fate(value: Fate);
    get count(): number;
    set count(value: number);
    get loop_length(): number;
    set loop_length(value: number);
    last(): Cell | undefined;
}
export interface RuleData {
    kind: string;
    birth: CountsData;
    survive: CountsData;
    wrap: boolean;
}
export class Rule {
    constructor(birth: Counts, survive: Counts, wrap: boolean);
    free(): void;
    static from(data: RuleData): Rule;
    toJSON(): RuleData;
    get birth(): Counts;
    set birth(value: Counts);
    get survive(): Counts;
    set survive(value: Counts);
    get wrap(): boolean;
    set wrap(value: boolean);
    boundary(): Boundary;
    checked(): Rule;
    config(mask: Cell): Config;
    static from_file(text: string): Rule;
    static from_json(text: string): Rule;
    static from_url(text: string): Rule;
    static of(config: Config): Rule;
    to_file(): string;
    to_id(): string;
    to_json(): string;
    to_prose(): string;
    to_url(): string;
}
export type SourceData = "Evens" | "Odds" | { Random: number } | "Primes" | "Binary" | "Fibonacci" | "GridSquares" | "CarpetFills" | "CarpetVoids" | "NetFills" | "NetVoids" | "TreeFills" | "TreeVoids" | "VoidFills" | "VoidVoids" | "PointFills" | "PointVoids" | "DustFills" | "DustVoids" | "LineFills" | "LineVoids" | "StarFills" | "StarVoids" | { CodeFills: bigint } | { CodeVoids: bigint };
export class Source {
    private constructor();
    free(): void;
    static from(data: SourceData): Source;
    toJSON(): SourceData;
    static all(): Source[];
    static designs(): Source[];
    is_random(): boolean;
    name(): string;
    static numbers(): Source[];
    oeis(): string | undefined;
    static parse(name: string): Source;
    static read(text: string): [Source, string] | undefined;
}
export declare namespace elementary {
    export function output(rule: number, l: number, c: number, r: number): number;
}
export declare namespace recipe {
    export function grammar(): recipe.Grammar;
    export function name(recipe: recipe.Recipe): string;
    export function parse(name: string): recipe.Recipe;
    export interface Grammar {
        shapes: string[];
        sides: number[];
        levels: number[];
        tilings: number[];
        modes: string[];
        mask_sides: number[];
        mask_levels: number[];
        copies: string[];
        sources: string[];
        rolls: string[];
    }
    export type Mode = "Island" | "Wallpaper";
    export const Mode: {
        all(): recipe.Mode[];
    };
    export interface Recipe {
        seed: string;
        side: number;
        level: number;
        tiling: number;
        mode: recipe.Mode;
        mask: string;
        birth: string;
        survive: string;
        x: number;
        cap: number;
    }
    export type RunData = Record<string, unknown>;
    export class Run {
        constructor(recipe: recipe.Recipe);
        free(): void;
        background(): number;
        birth(): Uint32Array;
        budget(): number;
        cap(): number;
        flips(): boolean;
        frame(): Cell;
        generation(): number;
        mask(): Tensor;
        origin(): BigInt64Array;
        population(): number;
        reach(): number;
        side(): number;
        step(): recipe.Stop | undefined;
        stop(): recipe.Stop | undefined;
        survive(): Uint32Array;
    }
    export interface Stop {
        kind: string;
        generation: number;
        period: number;
        dr: number;
        dc: number;
        wall_by: number;
    }
}
export declare namespace render {
    export function frame(grid: Cell, scale: number): Image;
}
export declare namespace source {
    export function sequence(seq: Source, limit: number): Uint32Array;
}
