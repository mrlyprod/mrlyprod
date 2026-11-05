export { default, initSync } from "./pkg/mrly_math_js.js";

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
export declare namespace arcs {
    export function curves(code: string | number | bigint, base: number, level: number): arcs.Curve[];
    export function draw(code: string | number | bigint, base: number, level: number): arcs.Arcs;
    export function law(code: string | number | bigint, base: number, level: number): arcs.Law | undefined;
    export function trace(side: number, on: boolean[]): arcs.Arcs;
    export interface Arcs {
        side: number;
        cells: number[];
        loops: number;
        strands: number;
    }
    export interface Curve {
        arcs: number[];
        dir: number[];
        closed: boolean;
    }
    export interface Law {
        formula: string;
        loops: number;
    }
}
export declare namespace arrays {
    export function essential(sensors: ArrayLike<number | bigint>): boolean[];
    export function fractal(generator: ArrayLike<number | bigint>, base: number | bigint, level: number): BigUint64Array;
    export function holes(sensors: ArrayLike<number | bigint>): BigInt64Array;
    export function law(generator: ArrayLike<number | bigint>, level: number): [bigint, bigint] | undefined;
    export function lost(sensors: ArrayLike<number | bigint>, sensor: number | bigint): BigInt64Array;
    export function paired(sensors: ArrayLike<number | bigint>): BigUint64Array;
    export function weights(sensors: ArrayLike<number | bigint>): BigUint64Array;
    export function MOST(): bigint;
}
export declare namespace atoms {
    export function carpet_2d(n: number): Tensor;
    export function carpet_3d(n: number): Tensor;
    export function carpet_nd(n: number, rank: number): Tensor;
    export function dust_2d(n: number): Tensor;
    export function dust_3d(n: number): Tensor;
    export function dust_nd(n: number, rank: number): Tensor;
    export function hline_2d(n: number): Tensor;
    export function htree_2d(n: number): Tensor;
    export function line_nd(n: number, rank: number, axis: number): Tensor;
    export function net_2d(n: number): Tensor;
    export function net_3d(n: number): Tensor;
    export function net_nd(n: number, rank: number): Tensor;
    export function noise_2d(n: number, density: number, rng: Rng): Tensor;
    export function noise_3d(n: number, density: number, rng: Rng): Tensor;
    export function ones_2d(n: number): Tensor;
    export function ones_3d(n: number): Tensor;
    export function point_2d(n: number): Tensor;
    export function point_3d(n: number): Tensor;
    export function point_nd(n: number, rank: number): Tensor;
    export function star_2d(n: number): Tensor;
    export function star_3d(n: number): Tensor;
    export function star_nd(n: number, rank: number): Tensor;
    export function tree_nd(n: number, rank: number, axis: number): Tensor;
    export function vline_2d(n: number): Tensor;
    export function void_2d(n: number): Tensor;
    export function void_3d(n: number): Tensor;
    export function void_nd(n: number, rank: number): Tensor;
    export function vtree_2d(n: number): Tensor;
    export function xline_3d(n: number): Tensor;
    export function xtree_3d(n: number): Tensor;
    export function yline_3d(n: number): Tensor;
    export function ytree_3d(n: number): Tensor;
    export function zeros_2d(n: number): Tensor;
    export function zeros_3d(n: number): Tensor;
    export function zline_3d(n: number): Tensor;
    export function ztree_3d(n: number): Tensor;
}
export declare namespace automaton {
    export function ROUNDING(): number;
    export type AutomatonData = Record<string, unknown>;
    export class Automaton {
        constructor(rule: memory.Rule);
        free(): void;
        static from(data: AutomatonData): Automaton;
        toJSON(): AutomatonData;
        abscissa(): number;
        base(): bigint;
        cofactor(s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
        denominator(): Float64Array;
        matrix(): Float64Array[];
        peel(): number;
        period(): number;
        perron(): [number, number];
        residue(w0: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
        rule(): memory.Rule;
        states(): number;
        static with_peel(rule: memory.Rule, peel: number): automaton.Automaton;
        zeta(s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
    }
}
export declare namespace bang {
    export function bang(dimension: number): bang.Universe;
    export function code_to_corners(code: string | number | bigint, dimension: number, base: number): Uint8Array[];
    export function corners(dimension: number): Uint8Array[];
    export function corners_to_code(filled: ArrayLike<number>[], dimension: number, base: number): string;
    export function level_cap(number: number, dimension: number, budget: number): number;
    export function levels_code(dimension: number, base: number, levels: ArrayLike<number>): string;
    export function magic(layers: bang.MagicLayer[]): Tensor;
    export function magic_named(layers: [string, number][]): Tensor;
    export function random_code(dimension: number, base: number, rng: Rng, min_fills: number): string;
    export function sources(catalog: bang.catalog.Catalog, dimension: number): bang.catalog.Source[];
    export function symmetries(dimension: number): [Uint32Array, Uint8Array][];
    export function total_exposure(code: string | number | bigint, dimension: number): boolean;
    export function touches_every_corner(code: string | number | bigint, dimension: number): boolean;
    export function universe_codes(dimension: number): string[];
    export interface DesignData {
        i: bigint;
        dimension: number;
        canonical: boolean;
        class_rep: bigint;
        orbit_size: number;
    }
    export class Design {
        private constructor();
        free(): void;
        static from(data: DesignData): Design;
        toJSON(): DesignData;
        get i(): string;
        set i(value: string | number | bigint);
        get dimension(): number;
        set dimension(value: number);
        get canonical(): boolean;
        set canonical(value: boolean);
        get class_rep(): string;
        set class_rep(value: string | number | bigint);
        get orbit_size(): number;
        set orbit_size(value: number);
        anf(): string;
        degree(): number;
        name(): string;
        rule(): Uint8Array[];
    }
    export interface MagicLayer {
        design: name.BangData;
        number: number;
    }
    export const MagicLayer: {
        "new"(design: name.Bang, number: number): bang.MagicLayer;
    };
    export type UniverseData = Record<string, unknown>;
    export class Universe {
        constructor(dimension: number);
        free(): void;
        static from(data: UniverseData): Universe;
        toJSON(): UniverseData;
        get dimension(): number;
        set dimension(value: number);
        get total(): number;
        set total(value: number);
        all(): bang.Design[];
        canonical(): bang.Design[];
        design(code: string | number | bigint): bang.Design;
        distinct(): number;
    }
    export namespace baseq {
        export function axis_maps(base: number): Uint32Array[];
        export function bracelets(max_base: number): string[];
        export function canonical(group: ArrayLike<number>[], code: string | number | bigint): string;
        export function carry(element: ArrayLike<number>, code: string | number | bigint): string;
        export function class_sequence(max_dimension: number): string[];
        export function classes(dimension: number): string;
        export function distinct_designs(base: number, dimension: number): string;
        export function even_fill_is_balanced(number: number, dimension: number, popcount: string | number | bigint): string;
        export function fill_from_corners(filled: ArrayLike<number>[], number: number, dimension: number): string;
        export function group(base: number, dimension: number): Uint32Array[];
        export function group_order(base: number, dimension: number): string;
        export function orbit(group: ArrayLike<number>[], code: string | number | bigint): string[];
        export function predicted_group_order(base: number, dimension: number): string;
        export function representatives(base: number, dimension: number): [string, number][];
        export function sequence(base: number, max_dimension: number): string[];
        export function total_designs(base: number, dimension: number): string;
        export function WALK_LIMIT(): number;
    }
    export namespace catalog {
        export function antis(dimension: number): bang.catalog.Design[];
        export function classics(dimension: number): bang.catalog.Design[];
        export function ANTIS_2D(): bang.catalog.Design[];
        export function ANTIS_3D(): bang.catalog.Design[];
        export function CLASSICS_2D(): bang.catalog.Design[];
        export function CLASSICS_3D(): bang.catalog.Design[];
        export type Catalog = "Classics" | "Universe" | { Codes: bigint[] } | { Designs: bang.catalog.Design[] };
        export type Design = "Carpet" | "Net" | "Htree" | "Vtree" | "Void" | "Xtree" | "Ytree" | "Ztree" | "Point" | "Dust" | "Hline" | "Vline" | "Star" | "Xline" | "Yline" | "Zline";
        export const Design: {
            all(): bang.catalog.Design[];
        };
        export type Source = { design: bang.catalog.Design } | { code: bigint };
    }
    export namespace code {
        export function get(code: string | number | bigint): string;
    }
    export namespace factory {
        export function create(code: string | number | bigint, number: number, dimension: number, base: number, level: number): Tensor;
        export function create_from_corners(filled: ArrayLike<number>[], number: number, dimension: number, base: number, level: number): Tensor;
        export function create_named(spec: string, number: number, level: number): Tensor;
        export function residue_corners(dimension: number, base: number): Uint8Array[];
        export function total_codes(dimension: number, base: number): string;
    }
    export namespace universe {
        export function anf(code: string | number | bigint, dimension: number): Uint8Array;
        export function anf_string(code: string | number | bigint, dimension: number): string;
        export function apply(element: [ArrayLike<number>, ArrayLike<number>], corner: ArrayLike<number>): Uint8Array;
        export function corner_index(corner: ArrayLike<number>): number;
        export function degree(code: string | number | bigint, dimension: number): number;
        export function orbit(code: string | number | bigint, dimension: number): string[];
        export function permutations(n: number): Uint32Array[];
    }
    export namespace word {
        export function components(layers: bang.MagicLayer[]): string;
        export function constant_functional(layers: bang.MagicLayer[]): number;
        export function dimension(layers: bang.MagicLayer[]): number;
        export function fill(layers: bang.MagicLayer[]): string;
        export function fills(layers: bang.MagicLayer[]): string[];
        export function letter(layer: bang.MagicLayer): bang.word.Letter;
        export function native(layers: bang.MagicLayer[]): boolean;
        export function period(layers: bang.MagicLayer[]): number;
        export function prefixes(layers: bang.MagicLayer[]): bang.word.Counts[];
        export function rates(layers: bang.MagicLayer[]): [number, number][];
        export function side(layers: bang.MagicLayer[]): string;
        export function spell(schedule: bang.word.Schedule, pair: [bang.MagicLayer, bang.MagicLayer], length: number): bang.MagicLayer[];
        export function staircase(depth: number): bang.MagicLayer[];
        export function thue_morse(index: number): number;
        export interface Counts {
            side: bigint;
            fill: bigint;
            components: bigint;
            runs_h: bigint;
            runs_v: bigint;
        }
        export interface Letter {
            fill: bigint;
            runs_h: bigint;
            runs_v: bigint;
            touch_h: bigint;
            touch_v: bigint;
            components: bigint;
        }
        export type Schedule = "ThueMorse" | "Periodic" | "Constant";
        export const Schedule: {
            all(): bang.word.Schedule[];
            frequencies(schedule: bang.word.Schedule): [number, number];
            place(schedule: bang.word.Schedule, index: number): number;
        };
    }
}
export declare namespace cell {
    export function grow(pattern: Tensor, level: number): Cell;
    export function paint(cell: Cell, custom?: Record<string, Color[]>, mode?: core.Mode, rng?: Rng): Cell;
    export namespace census {
        export function edges(cell: Cell): number;
        export function exposure(cell: Cell): string;
        export function fills(cell: Cell): number;
        export function vertices(cell: Cell): number;
        export function voids(cell: Cell): number;
    }
    export namespace geometry {
        export function merge_reps(cells: Cell[], reps: ArrayLike<number>): Cell;
        export function perforate(mask: Tensor, cell: Cell, value: number): Cell;
    }
    export namespace models {
        export function anti(cell: Cell): Cell;
        export function binarize(cell: Cell, threshold: number): Cell;
        export function binarize_otsu(cell: Cell): Cell;
        export function blur(cell: Cell, mask: Tensor, wrap: boolean): Cell;
        export function combine(cell: Cell, other: Cell): Cell;
        export function counting_dtype(mask: Tensor): core.Dtype;
        export function depth(cell: Cell): number;
        export function dtype_for(peak: number | bigint): core.Dtype;
        export function fractal(cell: Cell, level: number): Cell;
        export function height(cell: Cell): number;
        export function invert(cell: Cell): Cell;
        export function layers(cell: Cell): Cell;
        export function neighbors(cell: Cell, mask: Tensor, target: number, wrap: boolean): Cell;
        export function new_(types: Tensor): Cell;
        export function orient(cell: Cell, index: number): Cell;
        export function pad(cell: Cell, count: number, value: number): Cell;
        export function paint(cell: Cell, mapping: Record<string, Color[]>, mode: core.Mode, rng?: Rng): Cell;
        export function perforate(cell: Cell, mask: Tensor, value: number): Cell;
        export function rotate(cell: Cell, k: number, axes?: [number, number]): Cell;
        export function tile(cell: Cell, width: number, height: number, depth?: number): Cell;
        export function types(cell: Cell): Tensor;
        export function width(cell: Cell): number;
    }
    export namespace serializer {
        export function byte_cube(value: any): Uint8Array[][];
        export function byte_grid(value: any): Uint8Array[];
        export function color_grid(value: any): Uint8Array[][];
        export function count_cube(value: any): BigInt64Array;
        export function count_grid(value: any): BigInt64Array;
        export function parse(text: string): any;
        export function tag_layer(counts: ArrayLike<number | bigint>, shape: ArrayLike<number>): Tensor;
        export function types_field(data: any): any;
    }
}
export declare namespace coprime {
    export function gasket(level: number): bigint;
    export function GASKET_LEVELS(): number;
}
export declare namespace core {
    export type Colorizer = { Bins: { background: ColorData; ramp: ColorData[] } };
    export type Dtype = "U8" | "U16" | "U32" | "I32";
    export type Mode = "Type" | "Tag" | "Index" | "Enumerate" | "Random" | "Row" | "Column" | "Depth";
}
export declare namespace counts {
    export function centered_hexagonal(m: number): string;
    export function cut_fills(code: string | number | bigint, number: number, level: number): string;
    export function cut_voids(code: string | number | bigint, number: number, level: number): string;
    export function dimension(code: string | number | bigint, number: number, base_dimension: number, base: number): number;
    export function edges_of_tile(tile: Tensor, level: number): string | undefined;
    export function exposure(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    export function exposure_of_tile(tile: Tensor, level: number): string | undefined;
    export function exposure_recurrence(tile: Tensor): string[];
    export function fill(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    export function fill_from_corners(filled: ArrayLike<number>[], number: number, _dimension: number, level: number, base: number): string;
    export function fill_polynomial(corners: ArrayLike<number>[], dimension: number): string[];
    export function grid(number: number, dimension: number, level: number): string;
    export function limit(code: string | number | bigint, dimension: number, level: number, base: number): [string, string];
    export function odd_power(dimension: number): string[];
    export function pairs(tile: Tensor): [string, string][];
    export function positions(residue: number, number: number, base: number): string;
    export function pro_fills(code: string | number | bigint, number: number, level: number): string;
    export function pro_voids(code: string | number | bigint, number: number, level: number): string;
    export function profile_of_tile(tile: Tensor, level: number): string[];
    export function ratio(code: string | number | bigint, number: number, dimension: number, level: number, base: number): number;
    export function rational(code: string | number | bigint, number: number, dimension: number, level: number, base: number): [string, string];
    export function skeleton(code: string | number | bigint, number: number, dimension: number, base: number, level: number): counts.Skeleton;
    export function skeleton_of_tile(tile: Tensor, level: number): counts.Skeleton | undefined;
    export function skeleton_recurrence(code: string | number | bigint, number: number, dimension: number, base: number, k?: number): string[];
    export function surface(code: string | number | bigint, number: number, level: number, base: number): string;
    export function void_(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    export type ClosedData = { Power: bigint } | { Difference: [bigint, bigint] } | { Polynomial: bigint[] } | { Recurrence: bigint[] };
    export class Closed {
        private constructor();
        free(): void;
        static from(data: ClosedData): Closed;
        toJSON(): ClosedData;
        text(): string;
    }
    export interface ExposureData {
        occupancy: bigint;
        exposed: bigint;
        axes: [bigint, bigint][];
    }
    export class Exposure {
        private constructor();
        free(): void;
        static from(data: ExposureData): Exposure;
        toJSON(): ExposureData;
        get occupancy(): string;
        set occupancy(value: string | number | bigint);
        get exposed(): string;
        set exposed(value: string | number | bigint);
        get axes(): [string, string][];
        set axes(value: ([string | number | bigint, string | number | bigint])[]);
        at(level: number): string | undefined;
        static from_corners(filled: ArrayLike<number>[], number: number, dimension: number, base: number): counts.Exposure;
        static of_tile(tile: Tensor): counts.Exposure;
        recurrence(): string[];
    }
    export type PlacementsData = Record<string, unknown>;
    export class Placements {
        private constructor();
        free(): void;
        at(level: number): counts.Skeleton | undefined;
        static of_tile(tile: Tensor): counts.Placements;
        recurrence(k?: number): string[] | undefined;
        roots(k?: number): [string, number][];
    }
    export interface SkeletonData {
        counts: bigint[];
        euler: bigint;
    }
    export class Skeleton {
        private constructor();
        free(): void;
        get counts(): string[];
        set counts(value: (string | number | bigint)[]);
        get euler(): string;
        set euler(value: string | number | bigint);
        cells(): string;
        edges(): string;
        faces(): string;
        vertices(): string;
    }
    export namespace diagonal {
        export function WIDEST(): number;
    }
    export namespace ladder {
        export function cap(base: number): number;
        export function carry_matrix(base: number, dimension: number): string[][];
        export function characteristic(rows: ((string | number | bigint)[])[]): string[];
        export function determinant(rows: ((string | number | bigint)[])[]): string;
        export function digit_polynomial(base: number, dimension: number): string[];
        export function even_block(base: number, dimension: number): string[][];
        export function fill(base: number, dimension: number): string;
        export function ladder(base: number, dimension: number, levels: number): string[];
        export function perron(rows: ((string | number | bigint)[])[]): number;
        export function sign(base: number, dimension: number): number;
        export function spectral_ratio(base: number, dimension: number): number | undefined;
        export function trace(rows: ((string | number | bigint)[])[]): string;
    }
    export namespace six {
        export function grid_triangles(number: number, level: number): string;
        export function solid_slice_boundary(number: number): string;
        export function solid_slice_core_edges(number: number): string;
        export function solid_slice_core_nodes(number: number): string;
        export function solid_slice_edges(number: number): string;
        export function solid_slice_interior(number: number): string;
        export function solid_slice_triangles(number: number): string;
        export function solid_slice_vertices(number: number): string;
    }
}
export declare namespace design {
    export function digits_of(mask: number, base: number | bigint): BigUint64Array;
    export function echo_series(values: ArrayLike<number | bigint>, log_x: ArrayLike<number>, exponent: number): Float64Array;
    export function elements(base: number | bigint, digits: ArrayLike<number | bigint>, depth: number): BigUint64Array;
    export function log_grid(values: ArrayLike<number | bigint>, samples: number): Float64Array;
    export function median_floor(power: ArrayLike<number>, width: number): Float64Array;
    export function meter(mu: ArrayLike<number>): BigInt64Array;
    export function nearest(value: number, list: ArrayLike<number>): number;
    export function peaks(gamma: ArrayLike<number>, score: ArrayLike<number>, band: [number, number], threshold: number): Uint32Array;
    export function pole_lattice(base: number | bigint, top: number): Float64Array;
    export function resample(values: ArrayLike<number | bigint>, running: ArrayLike<number | bigint>, exponent: number, log_x: ArrayLike<number>): Float64Array;
    export function score(power: ArrayLike<number>, width: number): Float64Array;
    export function size(digits: ArrayLike<number | bigint>, depth: number): string;
    export function spectrum(log_x: ArrayLike<number>, series: ArrayLike<number>): [Float64Array, Float64Array];
    export function upper_rms(series: ArrayLike<number>): number;
    export function ZETA_ORDINATES(): Float64Array;
}
export declare namespace dissection {
    export function cap(y: number | bigint): bigint;
    export function chain_exponent(base: number | bigint): number;
    export function chain_margin(base: number | bigint): number;
    export function chain_root(base: number | bigint): number;
    export function chain_wall(): bigint;
    export function consecutive(digits: ArrayLike<number | bigint>): boolean;
    export function fraction(a: number | bigint, y: number | bigint, cap: number | bigint): [bigint, bigint];
    export function kappa(base: number | bigint, digits: ArrayLike<number | bigint>): [bigint, bigint];
    export function masses(base: number | bigint, digits: ArrayLike<number | bigint>, level: number): Float64Array;
    export function reach(base: number | bigint, missing: number | bigint): string;
    export function reading(base: number | bigint, fill: number, masses: ArrayLike<number>): number;
    export function regions(base: number | bigint, level: number, z: number | bigint): dissection.Region[];
    export function tally(base: number | bigint, digits: ArrayLike<number | bigint>, level: number, samples: number): dissection.Tally;
    export function weights(base: number | bigint, digits: ArrayLike<number | bigint>, level: number): Float64Array;
    export function BAR_A(): number;
    export function BAR_B(): number;
    export function CAP_BASE(): bigint;
    export function CERTIFIED(): [bigint, bigint][];
    export function DIGIT_WALL(): bigint;
    export function FIRST_BELOW(): bigint;
    export function GAMMA(): number;
    export function WINDOW_WALL(): bigint;
    export type Region = "A" | "B" | "C1" | "C2";
    export interface Tally {
        log_x: number[];
        count: number[];
        meter: number[];
        primes: number[];
    }
}
export declare namespace game {
    export function cell(point: ArrayLike<number>, base: number, level: number): bigint;
    export function cells(base: number, dimension: number, level: number): bigint;
    export function counts(matrix: (ArrayLike<number | bigint>)[], start: number, levels: number): BigUint64Array;
    export function de_bruijn(m: number, k: number): Uint8Array;
    export function mosaic_game(mosaic: game.Mosaic): game.MosaicGame;
    export function named_games(): game.NamedGame[];
    export function perron(matrix: (ArrayLike<number | bigint>)[]): number;
    export function reverse(rule: memory.Rule): memory.Rule;
    export function slice_game(base: number, digits: ArrayLike<number>, p: number | bigint, q: number | bigint): game.SliceGame;
    export function successors(rule: memory.Rule, history: ArrayLike<number>): Uint8Array;
    export function word_cell(word: ArrayLike<number>, base: number, dimension: number): bigint;
    export function LAP(): bigint;
    export interface Mosaic {
        base: number;
        dimension: number;
        masks: [number, number][][];
    }
    export interface MosaicGame {
        matrix: number[][];
        rho: number;
        dimension: number;
        edges: [number, number, number][];
    }
    export interface NamedGame {
        name: string;
        dimension: number;
        width: number;
        code: bigint;
    }
    export interface SliceGame {
        states: [number, number][];
        start: number;
        edges: [number, number, number][];
        matrix: number[][];
        rho: number;
        dimension: number;
    }
}
export declare namespace graph {
    export function census(network: graph.Network): graph.Census;
    export function components(network: graph.Network): number;
    export function core_graph(grid: Tensor): graph.Network;
    export function edge_graph(grid: Tensor): graph.Network;
    export function fractal_dimension(network: graph.Network, samples: number): number;
    export function junctions(network: graph.Network): number;
    export function largest_component(network: graph.Network): graph.Network;
    export function roles(network: graph.Network): graph.Role[];
    export function tips(network: graph.Network): number;
    export function total_length(network: graph.Network): number;
    export function tunnel_graph(grid: Tensor): graph.Network;
    export interface Branch {
        parent: number;
        child: number;
        radius: number;
    }
    export interface Census {
        nodes: number;
        branches: number;
        tips: number;
        junctions: number;
        components: number;
        total_length: number;
        fractal_dimension: number;
    }
    export type LayoutData = Record<string, unknown>;
    export class Layout {
        constructor(positions: ArrayLike<number>, branches: [number, number][], dim: number, seed: number | bigint);
        free(): void;
        energy(): number;
        static from_network(network: graph.Network, seed: number | bigint): graph.Layout;
        ideal(): number;
        moved(): number;
        nodes(): number;
        positions(): Float64Array;
        step(ticks: number): number;
        temperature(): number;
        ticks(): number;
    }
    export interface NetworkData {
        dim: number;
        nodes: graph.Node[];
        branches: graph.Branch[];
    }
    export class Network {
        constructor(dim: number);
        free(): void;
        static from(data: NetworkData): Network;
        toJSON(): NetworkData;
        get dim(): number;
        set dim(value: number);
        get nodes(): graph.Node[];
        set nodes(value: graph.Node[]);
        get branches(): graph.Branch[];
        set branches(value: graph.Branch[]);
        add_branch(parent: number, child: number, radius: number): void;
        add_node(position: ArrayLike<number>): number;
        adjacency(): Record<string, Uint32Array>;
        degree(): Uint32Array;
    }
    export interface Node {
        position: number[];
        index: number;
    }
    export type Role = "Alone" | "Tip" | "Through" | "Junction";
}
export declare namespace ladder {
    export function cofactor(design: ladder.Design, s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
    export function residue(design: ladder.Design, m: number, j: number | bigint, tolerance: number): [num.zeta.Complex, number];
    export function zeta(design: ladder.Design, s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
    export function ROUNDING(): number;
    export type DesignData = Record<string, unknown>;
    export class Design {
        constructor(base: number | bigint, digits: ArrayLike<number | bigint>);
        free(): void;
        static from(data: DesignData): Design;
        toJSON(): DesignData;
        abscissa(): number;
        base(): bigint;
        digits(): BigUint64Array;
        peel(): number;
        period(): number;
        pole(m: number, j: number | bigint): num.zeta.Complex;
        static with_peel(base: number | bigint, digits: ArrayLike<number | bigint>, peel: number): ladder.Design;
    }
}
export declare namespace memory {
    export function allowed_windows(rule: memory.Rule): number;
    export function cells(rule: memory.Rule, level: number): BigUint64Array;
    export function counts(rule: memory.Rule, levels: number): BigUint64Array;
    export function exponent(rule: memory.Rule): number;
    export function kappa(rule: memory.Rule): number;
    export function perron(rule: memory.Rule): number;
    export function transfer(rule: memory.Rule): BigUint64Array[];
    export function SPAN(): number;
    export function SWEEPS(): number;
    export function TOLERANCE(): number;
    export interface RuleData {
        dimension: number;
        width: number;
        code: number;
    }
    export class Rule {
        constructor(dimension: number, width: number, code: number | bigint);
        free(): void;
        static from(data: RuleData): Rule;
        toJSON(): RuleData;
        get dimension(): number;
        set dimension(value: number);
        get width(): number;
        set width(value: number);
        get code(): bigint;
        set code(value: number | bigint);
        accepts(word: ArrayLike<number>): boolean;
        allowed(window: number): boolean;
        alphabet(): Uint32Array;
        codes(): string;
        static full(dimension: number, width: number): memory.Rule;
        letters(): number;
        states(): number;
        windows(): number;
    }
}
export declare namespace moire {
    export function all(limit: number): moire.Preset[];
    export function frame(normal: ArrayLike<number>, offset: number): moire.Frame;
    export function layer(params: moire.Layer): boolean[];
    export function named(name: string, limit: number): moire.Preset;
    export function render(field: moire.Field, colorizer: core.Colorizer, levels: number, symmetric: boolean, invert: boolean, scale: number): Image;
    export function stack(spec: moire.Spec, numbers: ArrayLike<number>, combine: moire.Combine, level: number, lattice: moire.Lattice, size: number, slices: ArrayLike<number>): moire.Field;
    export function stack_codes(specs: moire.Spec[], number: number, level: number, lattice: moire.Lattice, size: number, slices: ArrayLike<number>): moire.Field;
    export function volume(spec: moire.Spec, numbers: ArrayLike<number>, combine: moire.Combine, level: number, size: number): moire.Volume;
    export type Combine = "Sum" | "And" | "Xor";
    export interface FieldData {
        data: number[];
        size: number;
    }
    export class Field {
        constructor(size: number);
        free(): void;
        static from(data: FieldData): Field;
        toJSON(): FieldData;
        get data(): Float32Array;
        set data(value: ArrayLike<number>);
        get size(): number;
        set size(value: number);
        as_f64(): Float64Array;
        static from_data(data: ArrayLike<number>, size: number): moire.Field;
        max(): number;
        mean(): number;
        min(): number;
        normalized(symmetric: boolean): Float32Array;
    }
    export interface Frame {
        centre: number[];
        u: number[];
        v: number[];
        normal: number[];
        width: number;
    }
    export type Lattice = "Square" | "Hex";
    export interface Layer {
        spec: moire.Spec;
        number: number;
        level: number;
        lattice: moire.Lattice;
        size: number;
        slices: number[];
    }
    export const Layer: {
        "new"(spec: moire.Spec, number: number): moire.Layer;
    };
    export interface PresetData {
        name: string;
        spec: moire.Spec;
        numbers: number[];
        combine: moire.Combine;
        level: number;
        lattice: moire.Lattice;
    }
    export class Preset {
        private constructor();
        free(): void;
        toJSON(): PresetData;
        readonly name: string;
        get spec(): moire.Spec;
        set spec(value: moire.Spec);
        get numbers(): Uint32Array;
        set numbers(value: ArrayLike<number>);
        get combine(): moire.Combine;
        set combine(value: moire.Combine);
        get level(): number;
        set level(value: number);
        get lattice(): moire.Lattice;
        set lattice(value: moire.Lattice);
        static carpet(limit: number): moire.Preset;
        field(size: number): moire.Field;
        static heatmap(limit: number): moire.Preset;
        static hive(limit: number): moire.Preset;
        static weave(limit: number): moire.Preset;
    }
    export interface Spec {
        code: bigint;
        base: number;
        dimension: number;
    }
    export const Spec: {
        "new"(code: string | number | bigint, base: number, dimension: number): moire.Spec;
    };
    export interface VolumeData {
        data: number[];
        size: number;
    }
    export class Volume {
        constructor(size: number);
        free(): void;
        static from(data: VolumeData): Volume;
        toJSON(): VolumeData;
        get data(): Float32Array;
        set data(value: ArrayLike<number>);
        get size(): number;
        set size(value: number);
        at(x: number, y: number, z: number): number;
        count(level: number): number;
        static from_data(data: ArrayLike<number>, size: number): moire.Volume;
        max(): number;
        min(): number;
        plane(frame: moire.Frame, out: number): [Float32Array, Uint8Array];
        sample(p: ArrayLike<number>): number | undefined;
        solid(level: number): Tensor;
    }
    export namespace pairs {
        export function correlation(m: number, n: number): number;
        export function sampled(m: number, n: number): number;
        export function witness(scale: number): moire.pairs.Witness;
        export interface Witness {
            scale: number;
            scales: number[];
            row: number[];
            max: number;
            at: number;
            prime: boolean;
        }
    }
    export namespace sample {
        export function axes(size: number, lattice: moire.Lattice, row: number): [Float64Array, Float64Array];
        export function membership(code: string | number | bigint, base: number, dimension: number): boolean[];
        export function pack(residues: ArrayLike<number>, base: number): number;
    }
}
export declare namespace name {
    export function of(code: string | number | bigint, dim: number, base: number): string | undefined;
    export function words(dim: number, base: number): [string, string][];
    export interface BangData {
        kind: string;
        dim: number;
        lattice: name.Lattice;
        base: number;
        code: bigint;
        twist?: number[];
    }
    export class Bang {
        constructor(code: string | number | bigint, dim: number, base: number);
        free(): void;
        static from(data: BangData): Bang;
        toJSON(): BangData;
        get dim(): number;
        set dim(value: number);
        get lattice(): name.Lattice;
        set lattice(value: name.Lattice);
        get base(): number;
        set base(value: number);
        get code(): string;
        set code(value: string | number | bigint);
        get twist(): Uint32Array | undefined;
        set twist(value: ArrayLike<number> | undefined);
        cells(): number;
        checked(): name.Bang;
        static from_file(text: string): name.Bang;
        static from_json(text: string): name.Bang;
        static from_url(text: string): name.Bang;
        to_file(): string;
        to_id(): string;
        to_json(): string;
        to_prose(): string;
        to_url(): string;
    }
    export type Lattice = "square" | "hex";
    export const Lattice: {
        default(): name.Lattice;
        is_square(lattice: name.Lattice): boolean;
        units(lattice: name.Lattice): number;
    };
    export interface SequenceData {
        kind: string;
        dim: number;
        base: number;
        code: bigint;
        measure: string;
        axis: string;
    }
    export class Sequence {
        constructor(code: string | number | bigint, dim: number, base: number, measure: string, axis: string);
        free(): void;
        static from(data: SequenceData): Sequence;
        toJSON(): SequenceData;
        get dim(): number;
        set dim(value: number);
        get base(): number;
        set base(value: number);
        get code(): string;
        set code(value: string | number | bigint);
        get measure(): string;
        set measure(value: string);
        get axis(): string;
        set axis(value: string);
        checked(): name.Sequence;
        design(): name.Bang;
        static from_file(text: string): name.Sequence;
        static from_json(text: string): name.Sequence;
        static from_url(text: string): name.Sequence;
        to_file(): string;
        to_id(): string;
        to_json(): string;
        to_prose(): string;
        to_url(): string;
    }
    export interface WordData {
        kind: string;
        dim: number;
        magic: bigint[];
        side: number[];
        base?: number[];
    }
    export class Word {
        constructor(dim: number, letters: ([string | number | bigint, number])[]);
        free(): void;
        static from(data: WordData): Word;
        toJSON(): WordData;
        get dim(): number;
        set dim(value: number);
        get magic(): string[];
        set magic(value: (string | number | bigint)[]);
        get side(): Uint32Array;
        set side(value: ArrayLike<number>);
        get base(): Uint32Array | undefined;
        set base(value: ArrayLike<number> | undefined);
        bases(): Uint32Array;
        checked(): name.Word;
        static from_file(text: string): name.Word;
        static from_json(text: string): name.Word;
        static from_url(text: string): name.Word;
        letters(): name.Bang[];
        to_file(): string;
        to_id(): string;
        to_json(): string;
        to_prose(): string;
        to_url(): string;
    }
}
export declare namespace nodal {
    export function figures(code: number | bigint, number: number, level: number, count: number): nodal.Figures;
    export function holds(code: number | bigint, number: number, level: number): boolean;
    export interface Figures {
        side: number;
        values: number[];
        domains: number[];
        bound: number[];
        fields: number[];
    }
}
export declare namespace num {
    export namespace gauss {
        export type Ring = "Gaussian" | "Eisenstein";
    }
    export namespace zeta {
        export interface Complex {
            re: number;
            im: number;
        }
    }
}
export declare namespace press {
    export function containing(number: string | number | bigint, dimension: number, base: number): string;
    export function coordinates(number: string | number | bigint, dimension: number, base: number): string[];
    export function count_below(code: string | number | bigint, dimension: number, base: number, limit: string | number | bigint): string;
    export function distinct(number: string | number | bigint, dimension: number, base: number): number;
    export function interleave(coords: (string | number | bigint)[], base: number): string;
    export function layer_table(layer: bang.MagicLayer): boolean[];
    export function member(code: string | number | bigint, number: string | number | bigint, dimension: number, base: number): boolean;
    export function members(code: string | number | bigint, dimension: number, base: number, count: number): string[];
    export function profile(code: string | number | bigint, dimension: number, base: number, level: number): string[];
    export function usage(number: string | number | bigint, dimension: number, base: number): string;
    export function word_count(layers: bang.MagicLayer[]): string;
    export function word_member(layers: bang.MagicLayer[], number: string | number | bigint): boolean;
    export function word_members(layers: bang.MagicLayer[]): string[];
    export function word_profile(layers: bang.MagicLayer[]): string[];
    export function CORNERS(): number;
    export type PressData = Record<string, unknown>;
    export class Press {
        constructor(dimension: number, base: number);
        free(): void;
        static from(data: PressData): Press;
        toJSON(): PressData;
        get dimension(): number;
        set dimension(value: number);
        get base(): number;
        set base(value: number);
        add(number: string | number | bigint, weight: string | number | bigint): void;
        total(code: string | number | bigint): string;
        totals(): string[];
    }
}
export declare namespace radix {
    export function flowsnake(): radix.Radix;
    export function gasket(): radix.Radix;
    export function koch(): radix.Radix;
    export function terdragon(): radix.Radix;
    export function tile(m: number | bigint, code: string | number | bigint): radix.Radix;
    export function twindragon(): radix.Radix;
    export type BaseData = Record<string, unknown>;
    export class Base {
        constructor(ring: num.gauss.Ring, value: [number | bigint, number | bigint]);
        free(): void;
        static from(data: BaseData): Base;
        toJSON(): BaseData;
        class(z: [number | bigint, number | bigint]): number;
        congruent(z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): boolean;
        group(): Uint32Array[];
        mirrored(): boolean;
        norm(): bigint;
        power(level: number): [bigint, bigint];
        residues(): [bigint, bigint][];
        ring(): num.gauss.Ring;
        value(): [bigint, bigint];
    }
    export type RadixData = Record<string, unknown>;
    export class Radix {
        constructor(base: radix.Base, digits: ([number | bigint, number | bigint])[], twists: ([number | bigint, number | bigint])[]);
        free(): void;
        static from(data: RadixData): Radix;
        toJSON(): RadixData;
        base(): radix.Base;
        canonical(): boolean;
        code(): string;
        digits(): [bigint, bigint][];
        dimension(): number;
        distinct(level: number): number;
        fill(level: number): string;
        static from_code(base: radix.Base, code: string | number | bigint): radix.Radix;
        plane(level: number): [number, number][];
        ring(): num.gauss.Ring;
        size(): number;
        twists(): [bigint, bigint][];
        with_twists(units: ArrayLike<number>): radix.Radix;
        words(level: number): [bigint, bigint][];
    }
}
export declare namespace roulette {
    export function nodes(track: spirograph.Track, pencils: spirograph.Pencil[], samples: number, tol: number): roulette.Nodes;
    export function side(a: ArrayLike<number>, b: ArrayLike<number>, c: ArrayLike<number>): number;
    export function spread(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): spirograph.Pencil[];
    export interface NodesData {
        curves: number;
        selves: number[];
        pairs: number[];
        most: number;
        crowded: number;
        points: number;
        branches: number;
        touches: number;
    }
    export class Nodes {
        private constructor();
        free(): void;
        static from(data: NodesData): Nodes;
        toJSON(): NodesData;
        get curves(): number;
        set curves(value: number);
        get selves(): Uint32Array;
        set selves(value: ArrayLike<number>);
        get pairs(): Uint32Array;
        set pairs(value: ArrayLike<number>);
        get most(): number;
        set most(value: number);
        get crowded(): number;
        set crowded(value: number);
        get points(): number;
        set points(value: number);
        get branches(): number;
        set branches(value: number);
        get touches(): number;
        set touches(value: number);
        static default(): roulette.Nodes;
        pair(i: number, j: number): number;
        paired(): number;
        selved(): number;
        total(): number;
    }
}
export declare namespace rules {
    export function render(filled: ArrayLike<number>[], number: number, dimension: number, base: number): Tensor;
    export function tree_axes(dimension: number, free_axis: number): Uint32Array;
    export function BASE(): number;
}
export declare namespace shape {
    export function census(shape: shape.Shape, types: Tensor): shape.ShapeCensus;
    export function classify(shape: shape.Shape, side: number, index: ArrayLike<number>): shape.Region;
    export function crop(types: Tensor, shape: shape.Shape, keep_cut: boolean): Tensor;
    export function crossing_shell(radius: number | bigint, number: number | bigint, level: number): [bigint, bigint][];
    export function crossing_tree(radius: number | bigint, number: number | bigint, keep: boolean[]): shape.Shell;
    export function fit(shape: shape.Shape, extents: ArrayLike<number>): shape.Shape;
    export function named(name: string, dimension: number, radius: shape.Frac): shape.Shape;
    export function radial_census(types: Tensor, centre: ArrayLike<number | bigint>, r_max: number | bigint): shape.RadialCounts[];
    export function reach(name: string, dimension: number, den: number): number | undefined;
    export function refine(types: Tensor, shape: shape.Shape, base: number, extra: number, keep_cut: boolean): Tensor;
    export function regions(shape: shape.Shape, dims: ArrayLike<number>): Tensor;
    export function shapes(dimension: number): string[];
    export function REFINE_LIMIT(): number;
    export interface FracData {
        num: number;
        den: number;
    }
    export class Frac {
        constructor(num: number | bigint, den: number | bigint);
        free(): void;
        static from(data: FracData): Frac;
        toJSON(): FracData;
        get num(): bigint;
        set num(value: number | bigint);
        get den(): bigint;
        set den(value: number | bigint);
        minus(other: shape.Frac): shape.Frac;
        plus(other: shape.Frac): shape.Frac;
        times(other: shape.Frac): shape.Frac;
        static whole(num: number | bigint): shape.Frac;
    }
    export interface Half {
        normal: number[];
        offset: shape.FracData;
    }
    export interface RadialCounts {
        seen: number;
        inside: number;
        cut: number;
    }
    export type Region = "Out" | "Cut" | "In";
    export const Region: {
        flip(region: shape.Region): shape.Region;
    };
    export type Shape = { Ball: { center: shape.FracData[]; radius: shape.FracData } } | { Polytope: { walls: shape.Half[] } } | { Anti: shape.Shape } | { Window: { inner: shape.Shape; low: shape.FracData[]; high: shape.FracData[] } };
    export interface ShapeCensus {
        cells: number[];
        filled: number[];
    }
    export interface Shell {
        radius: number;
        number: number;
        levels: shape.ShellBox[][];
        orphans: number;
    }
    export interface ShellBox {
        x: number;
        y: number;
        seat: number;
        parent: number;
        live: boolean;
    }
}
export declare namespace six {
    export function anti(cell: Cell6d): Cell6d;
    export function binarize(cell: Cell6d, threshold: number): Cell6d;
    export function binarize_otsu(cell: Cell6d): Cell6d;
    export function blank(radius: number, orient: six.Orientation, fill: number, void_: number): Cell;
    export function blur(cell: Cell6d, mask: Tensor, wrap: boolean): Cell6d;
    export function census(cell: Cell6d, include_grid: boolean): six.Census;
    export function components(cell: Cell6d): number;
    export function copies(rings: number): number;
    export function cut(cell: Cell): Cell6d;
    export function cut_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    export function disc(cell: Cell6d, rings: number): Cell6d;
    export function east(x: number | bigint, y: number | bigint): [bigint, bigint][];
    export function euler(cell: Cell6d, include_grid: boolean): bigint;
    export function fills(cell: Cell6d): number;
    export function fills_only(cell: Cell6d): six.Census;
    export function framed(cell: Cell6d): Cell6d;
    export function from_json(text: string): Cell6d;
    export function giant(cell: Cell6d): number;
    export function giant_network(cell: Cell6d): graph.Network;
    export function height(cell: Cell6d): number;
    export function holes(cell: Cell6d): number;
    export function image(cell: Cell6d, scale: number, outline: Color | undefined, width: number): Image;
    export function is_cube(cell: Cell): boolean;
    export function is_hex(cell: Cell): boolean;
    export function iso(cell: Cell): Cell6d;
    export function iso_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    export function new_(cell: Cell, projection: six.Projection, orientation: six.Orientation, start: number): Cell6d;
    export function north(x: number | bigint, y: number | bigint): [bigint, bigint][];
    export function orientation(width: number, height: number): six.Orientation;
    export function pad(cell: Cell6d, k: number, value: number): Cell6d;
    export function paint(cell: Cell6d, custom?: Record<string, Color[]>, mode?: core.Mode, rng?: Rng): Cell6d;
    export function perforate(cell: Cell6d, mask: Tensor, value: number): Cell6d;
    export function pro(cell: Cell): Cell6d;
    export function pro_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    export function radial(cell: Cell6d, radius: number): Cell;
    export function radial_crop(cell: Cell, radius: number, size: [number, number]): Cell;
    export function radial_mask(radius: number, orient: six.Orientation): Tensor;
    export function raster(cell: Cell6d, size: number): Float32Array;
    export function rect_image(cell: Cell6d, scale: number, start?: number): Image;
    export function rect_svg(cell: Cell6d, scale: number, start?: number): string;
    export function rim_holes(cell: Cell6d): number;
    export function skin(cell: Cell6d): Cell6d;
    export function slice_core_graph(cell: Cell6d): graph.Network;
    export function slice_dual_graph(cell: Cell6d): graph.Network;
    export function slice_edge_graph(cell: Cell6d, value?: number): graph.Network;
    export function slice_tunnel_graph(cell: Cell6d): graph.Network;
    export function south(x: number | bigint, y: number | bigint): [bigint, bigint][];
    export function spectral_exponent(cell: Cell6d, window: number): number;
    export function svg(cell: Cell6d, scale: number, outline: Color | undefined, width: number, start?: number): string;
    export function tessellate(cell: Cell6d, mask: Tensor): Cell;
    export function tile(cell: Cell6d, width: number, height: number): Cell;
    export function tile_cell(cell: Cell6d, width: number, height: number, crop: boolean): Cell6d;
    export function tile_crop(cell: Cell, size: [number, number]): Cell;
    export function tile_step(size: [number, number]): [number, number];
    export function to_json(cell: Cell6d): string;
    export function triangles(cell: Cell6d, start?: number): [[bigint, bigint][], Uint8Array][];
    export function west(x: number | bigint, y: number | bigint): [bigint, bigint][];
    export function width(cell: Cell6d): number;
    export function FILL(): number;
    export function GRID(): number;
    export function LEFT(): number;
    export function RIGHT(): number;
    export function UP(): number;
    export function VOID(): number;
    export interface Census {
        triangles: number;
        fills: number;
        voids: number;
        grids: number;
        vertices: number;
        edges: number;
        boundary_edges: number;
        interior_edges: number;
        euler: number;
    }
    export type Orientation = "Horizontal" | "Vertical";
    export type Projection = "Iso" | "Pro" | "Cut";
    export namespace star {
        export function arm_law(number: number): six.star.Share;
        export function chi8(number: number): bigint;
        export function constant(): number;
        export function decay(excesses: ArrayLike<number>, layers: number): six.star.Decay;
        export function width_law(half: number): number;
        export type Branch = "Zero" | "Two" | "Odd";
        export const Branch: {
            constant(branch: six.star.Branch): number;
            name(branch: six.star.Branch): string;
            of(layers: number): six.star.Branch;
            residual(branch: six.star.Branch): number | undefined;
        };
        export interface Decay {
            layers: number;
            excess: number;
            scaled: number;
            logged: number;
            miss: number;
            linear: number;
            residual: number;
            slope?: number;
        }
        export interface ShareData {
            inked: number;
            cells: number;
        }
        export class Share {
            private constructor();
            free(): void;
            static from(data: ShareData): Share;
            toJSON(): ShareData;
            get inked(): bigint;
            set inked(value: number | bigint);
            get cells(): bigint;
            set cells(value: number | bigint);
            reduced(): [bigint, bigint];
            value(): number;
        }
        export type StarData = Record<string, unknown>;
        export class Star {
            constructor(code: string | number | bigint);
            free(): void;
            static from(data: StarData): Star;
            toJSON(): StarData;
            arm(number: number, half: number): six.star.Share;
            cell(number: number, x: number | bigint, z: number | bigint): boolean | undefined;
            excesses(layers: number, half: number): Float64Array;
            hexagon(number: number): six.star.Share;
        }
    }
}
export declare namespace spectrum {
    export function clusters(eigenvalues: ArrayLike<number>, tolerance: number): [number, number][];
    export function laplacian(network: graph.Network, normalised: boolean): Float64Array[];
    export function laplacian_spectrum(network: graph.Network, normalised: boolean): Float64Array;
    export function multiplicity(eigenvalues: ArrayLike<number>, value: number, tolerance: number): number;
    export function spectral_exponent(eigenvalues: ArrayLike<number>, window: number): number | undefined;
    export function spectral_fit(eigenvalues: ArrayLike<number>, window: number): [number, number, number] | undefined;
    export function spectral_points(eigenvalues: ArrayLike<number>): [number, number][];
    export function symmetric_eigenvalues(matrix: ArrayLike<number>[]): Float64Array;
}
export declare namespace spin {
    export function arcs(data: ArrayLike<number>, size: number, radius: number): [number, number, number][];
    export function harmonics(data: ArrayLike<number>, size: number, rings: number, orders: number): Float64Array;
    export function mass(profile: ArrayLike<number>, size: number): number;
    export function mass_within(profile: ArrayLike<number>, size: number, radius: number): number;
    export function petals(copies: number, order: number): number;
    export function profile(data: ArrayLike<number>, size: number, steps: number): Float32Array;
    export function radial(data: ArrayLike<number>, size: number, out: number, copies: number, step: number, blend: spin.Blend, samples: number): Float32Array;
    export function reach(size: number): number;
    export function ring(data: ArrayLike<number>, size: number, radius: number): number;
    export function turns(power: ArrayLike<number>): number;
    export function wheel(profile: ArrayLike<number>, size: number): Float32Array;
    export type Blend = "Mean" | "Sum" | "Union" | "Meet" | "Parity" | "Difference";
    export const Blend: {
        fold(blend: spin.Blend, values: ArrayLike<number>): number;
        named(name: string): spin.Blend | undefined;
    };
}
export declare namespace spirograph {
    export function cell(width: number, height: number, reach: number): number;
    export function cover(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean, samples: number, side: number): spirograph.Cover;
    export function disc(track: spirograph.Track, pencils: spirograph.Pencil[]): spirograph.Disc;
    export function distinct(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): number;
    export function frame(track: spirograph.Track, pencils: spirograph.Pencil[]): Float64Array;
    export function nodes(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): bigint | undefined;
    export function pencils(types: ArrayLike<number>, width: number, height: number, mode: string, reach: number, jitter: number, seed: number): spirograph.Pencil[];
    export function point(track: spirograph.Track, pencil: spirograph.Pencil, s: number): [number, number];
    export function pose(track: spirograph.Track, s: number): [number, number];
    export function representatives(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): Uint32Array;
    export function seats(pencils: spirograph.Pencil[]): spirograph.Seats;
    export function signed_area(track: spirograph.Track, pencil: spirograph.Pencil): number | undefined;
    export function trace(track: spirograph.Track, pencils: spirograph.Pencil[], samples: number): Float32Array;
    export function track(kind: string, ring: number, wheel: number, sides: number, laps: number): spirograph.Track;
    export function turn(track: spirograph.Track, s: number): number;
    export function LAPS_CAP(): number;
    export function PENCIL_CAP(): number;
    export function POINT_CAP(): number;
    export function RADIUS_CAP(): number;
    export function RASTER_CAP(): number;
    export function SIDES(): [number, number];
    export interface Cover {
        mask: number[];
        side: number;
        covered: number;
        hole: number;
        wall: number;
        winding: number;
        areas: number;
    }
    export interface Disc {
        x: number;
        y: number;
        radius: number;
        hole: number;
    }
    export type Kind = "Fill" | "Void" | "Corner";
    export interface Pencil {
        x: number;
        y: number;
        seat: [number, number];
        kind: spirograph.Kind;
    }
    export type Piece = { Run: { from: [number, number]; to: [number, number] } } | { Turn: { about: [number, number]; radius: number; from: number; to: number } };
    export interface Seats {
        fills: number;
        voids: number;
        corners: number;
    }
    export const Seats: {
        default(): spirograph.Seats;
    };
    export interface Track {
        kind: string;
        wheel: number;
        pieces: spirograph.Piece[];
        total: number;
        side: number;
        outline: [number, number][];
        closed: boolean;
        ratio: [number, number];
        orbits: number;
        fold: number;
    }
}
export declare namespace sumset {
    export function pairs(level: number): sumset.Pair[];
    export function DEEPEST(): number;
    export function WIDEST(): number;
    export type PairData = Record<string, unknown>;
    export class Pair {
        constructor(three: number, four: number);
        free(): void;
        static from(data: PairData): Pair;
        toJSON(): PairData;
        clean(): boolean;
        copy(): boolean;
        energy(): string;
        four(): number;
        gap(): [bigint, bigint] | undefined;
        largest(): bigint;
        ratio(energy: string | number | bigint): number;
        scale(): number;
        three(): number;
    }
    export type SumsetData = Record<string, unknown>;
    export class Sumset {
        constructor(level: number);
        free(): void;
        contains(x: number | bigint): boolean | undefined;
        count(x: number | bigint): bigint | undefined;
        density(x: number | bigint): number | undefined;
        extremes(edges: ArrayLike<number | bigint>): ([number, number] | undefined)[];
        fills(low: number | bigint, high: number | bigint, cells: number): Float64Array;
        level(): number;
        top(): bigint;
    }
}
export declare namespace three {
    export function carpet(number: number, level: number): Cell;
    export function census(cell: Cell): three.Census;
    export function core_graph(cell: Cell): graph.Network;
    export function create(code: string | number | bigint, number: number, level: number, base: number): Cell;
    export function diagonal_slice(code: string | number | bigint, number: number, level: number, base: number, height: number): Uint32Array[];
    export function diagonal_svg(code: string | number | bigint, number: number, level: number, base: number, heights: ArrayLike<number>, scale: number): string;
    export function dust(number: number, level: number): Cell;
    export function edge_graph(cell: Cell): graph.Network;
    export function euler(cell: Cell): bigint;
    export function extrude(cell: Cell, axis: number, depth: number): Cell;
    export function extrude_cube(cell: Cell, depth: number): Cell;
    export function faces(cell: Cell): number;
    export function fills(cell: Cell): number;
    export function from_corners(corners: ArrayLike<number>[], number: number, level: number, base: number): Cell;
    export function from_json(text: string): Cell;
    export function from_strings(data: string[][]): Cell;
    export function hidden(cell: Cell): string;
    export function level_set(number: number, levels: ArrayLike<number>, level: number, base: number): Cell;
    export function magic(cells: Cell[]): Cell;
    export function manhattan_layers(cell: Cell): Cell;
    export function merge(cells: Cell[], width: number, height: number, depth: number): Cell;
    export function mosaic(mask: Tensor, cells: Cell[]): Cell;
    export function named(design: bang.catalog.Design, number: number, level: number): Cell;
    export function net(number: number, level: number): Cell;
    export function noise(number: number, level: number, density: number, rng: Rng): Cell;
    export function ones(number: number, level: number): Cell;
    export function orientations(): [number, number, number][];
    export function point(number: number, level: number): Cell;
    export function profile(code: string | number | bigint, number: number, level: number, base: number): string[];
    export function project(point: ArrayLike<number>): [number, number];
    export function quads(cell: Cell): three.Quad[];
    export function shadow(point: ArrayLike<number>): [bigint, bigint];
    export function slice(cell: Cell, axis: number, index: number): Cell;
    export function special(mask: Tensor, cell: Cell): Cell;
    export function star(number: number, level: number): Cell;
    export function support(counts: (string | number | bigint)[]): [number, number] | undefined;
    export function surface(cell: Cell): string;
    export function text(cell: Cell, glyphs?: Record<string, string>): string[];
    export function to_json(cell: Cell): string;
    export function to_obj(cell: Cell): string;
    export function to_strings(cell: Cell): string[][];
    export function tunnel_graph(cell: Cell): graph.Network;
    export function void_(number: number, level: number): Cell;
    export function voids(cell: Cell): number;
    export function volume(cell: Cell): number;
    export function wires(cell: Cell): three.Vec3[][];
    export function xline(number: number, level: number): Cell;
    export function xtree(number: number, level: number): Cell;
    export function yline(number: number, level: number): Cell;
    export function ytree(number: number, level: number): Cell;
    export function zeros(number: number, level: number): Cell;
    export function zline(number: number, level: number): Cell;
    export function ztree(number: number, level: number): Cell;
    export interface Census {
        fills: number;
        voids: number;
        surface: bigint;
        vertices: number;
        edges: number;
        faces: number;
        euler: number;
    }
    export interface Quad {
        normal: three.Vec3Data;
        verts: three.Vec3Data[];
    }
    export interface Vec3Data {
        x: number;
        y: number;
        z: number;
    }
    export class Vec3 {
        constructor(x: number, y: number, z: number);
        free(): void;
        static from(data: Vec3Data): Vec3;
        toJSON(): Vec3Data;
        get x(): number;
        set x(value: number);
        get y(): number;
        set y(value: number);
        get z(): number;
        set z(value: number);
        cross(o: three.Vec3): three.Vec3;
        dot(o: three.Vec3): number;
        scale(s: number): three.Vec3;
    }
    export namespace sponge {
        export function deep(radius: number): number;
        export function dimension(): number;
        export function distance(point: ArrayLike<number>): number;
        export function exact(radius: number): number;
        export function profile(radius: number): number | undefined;
        export function reading(radius: number): number | undefined;
        export function tube(radius: number): number | undefined;
        export function volume(radius: number): number | undefined;
        export function COVER(): number;
        export function EDGE(): number;
    }
}
export declare namespace tourbillon {
    export function eyes(qmax: number): tourbillon.Eye[];
    export function field(top: number, size: number, schedule: string, increment: number, set: string, weights: string, mode: string, blend: string, seed: number): Float32Array;
    export function layers(top: number, schedule: string, increment: number, set: string, weights: string, seed: number): tourbillon.Layer[];
    export function period(increment: number): number | undefined;
    export function sharing(list: tourbillon.Layer[]): [number, number];
    export function stack(list: tourbillon.Layer[], size: number, mode: string, blend: spin.Blend): Float32Array;
    export function stats(field: ArrayLike<number>, size: number, top: number, schedule: string, increment: number, set: string, weights: string, blend: string, seed: number): tourbillon.Stats;
    export interface Eye {
        angle: number;
        numer: number;
        denom: number;
    }
    export interface Layer {
        scale: number;
        weight: number;
        degrees: number;
    }
    export interface Stats {
        layers: number;
        scales: number[];
        angles: number[];
        mean: number;
        rms: number;
        faded: number;
        centre: number;
        weighted: boolean;
        low: number;
        high: number;
        inside: number;
        peaks: number[][];
        period?: number;
        classes: number;
        pairs: number;
    }
}
export declare namespace two {
    export function capacity(cell: Cell): number;
    export function carpet(number: number, level: number): Cell;
    export function census(cell: Cell): two.Census;
    export function create(code: string | number | bigint, number: number, level: number, rotation: number, base: number): Cell;
    export function dust(number: number, level: number): Cell;
    export function embed(cell: Cell, payload: ArrayLike<number>): Cell;
    export function euler(cell: Cell): bigint;
    export function extract(carrier: Cell, carried: Cell): Uint8Array;
    export function fills(cell: Cell): number;
    export function from_corners(corners: ArrayLike<number>[], number: number, level: number, rotation: number, base: number): Cell;
    export function from_json(text: string): Cell;
    export function from_strings(rows: string[]): Cell;
    export function hline(number: number, level: number): Cell;
    export function htree(number: number, level: number): Cell;
    export function image(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: two.Shape): Image;
    export function level_set(number: number, levels: ArrayLike<number>, level: number, rotation: number, base: number): Cell;
    export function mask(mask: Tensor, shape: ArrayLike<number>): Tensor;
    export function merge(cells: Cell[], width: number, height: number): Cell;
    export function named(design: bang.catalog.Design, number: number, level: number, rotation: number): Cell;
    export function net(number: number, level: number): Cell;
    export function noise(number: number, level: number, density: number, rng: Rng): Cell;
    export function ones(number: number, level: number): Cell;
    export function perimeter(cell: Cell): string;
    export function point(number: number, level: number): Cell;
    export function read(sheet: Cell, carrier: Cell): Uint8Array;
    export function sheet(cells: Cell[], payload: ArrayLike<number>): Cell;
    export function special(mask: Tensor, cell: Cell): Cell;
    export function star(number: number, level: number): Cell;
    export function svg(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: two.Shape): string;
    export function text(cell: Cell, glyphs?: Record<string, string>): string[];
    export function to_3d(cell: Cell): Cell;
    export function to_json(cell: Cell): string;
    export function vline(number: number, level: number): Cell;
    export function void_(number: number, level: number): Cell;
    export function voids(cell: Cell): number;
    export function vtree(number: number, level: number): Cell;
    export function zeros(number: number, level: number): Cell;
    export interface Census {
        fills: number;
        voids: number;
        perimeter: bigint;
        vertices: number;
        edges: number;
        euler: number;
    }
    export type Shape = "Square" | "Circle" | "Diamond";
    export namespace payload {
        export function frame(): Tensor;
    }
}
