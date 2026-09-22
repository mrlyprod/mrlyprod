export { default, initSync } from "./pkg/math/mrlyjs_math.js";

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
export declare namespace atoms {
    /** Builds an n by n carpet, on where at most one coordinate is odd. */
    export function carpet_2d(n: number): Tensor;
    /** Builds an n by n by n carpet, on where at most one coordinate is odd. */
    export function carpet_3d(n: number): Tensor;
    /** Builds a carpet of the given side at any rank, on where at most one coordinate is odd. */
    export function carpet_nd(n: number, rank: number): Tensor;
    /** Builds an n by n dust, on where both coordinates are even. */
    export function dust_2d(n: number): Tensor;
    /** Builds an n by n by n dust, on where all three coordinates are even. */
    export function dust_3d(n: number): Tensor;
    /** Builds a dust of the given side at any rank, on where every coordinate is even. */
    export function dust_nd(n: number, rank: number): Tensor;
    /** Builds an n by n line, free on axis 1, on along the odd rows. */
    export function hline_2d(n: number): Tensor;
    /** Builds an n by n tree, free on axis 1, on along the even rows. */
    export function htree_2d(n: number): Tensor;
    /** Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none. */
    export function line_nd(n: number, rank: number, axis: number): Tensor;
    /** Builds an n by n net, on where at least one coordinate is odd. */
    export function net_2d(n: number): Tensor;
    /** Builds an n by n by n net, on where at least two coordinates are odd. */
    export function net_3d(n: number): Tensor;
    /** Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank. */
    export function net_nd(n: number, rank: number): Tensor;
    /** Builds an n by n tensor where each cell turns on with probability density, drawn from the stream. */
    export function noise_2d(n: number, density: number, rng: Rng): Tensor;
    /** Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream. */
    export function noise_3d(n: number, density: number, rng: Rng): Tensor;
    /** Builds an n by n tensor of ones. */
    export function ones_2d(n: number): Tensor;
    /** Builds an n by n by n tensor of ones. */
    export function ones_3d(n: number): Tensor;
    /** Builds an n by n point, on where both coordinates are odd. */
    export function point_2d(n: number): Tensor;
    /** Builds an n by n by n point, on where all three coordinates are odd. */
    export function point_3d(n: number): Tensor;
    /** Builds a point of the given side at any rank, on where every coordinate is odd. */
    export function point_nd(n: number, rank: number): Tensor;
    /** Builds an n by n star, on where exactly one coordinate is odd. */
    export function star_2d(n: number): Tensor;
    /** Builds an n by n by n star, on where exactly one coordinate is odd. */
    export function star_3d(n: number): Tensor;
    /** Builds a star of the given side at any rank, on where exactly one coordinate is odd. */
    export function star_nd(n: number, rank: number): Tensor;
    /** Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none. */
    export function tree_nd(n: number, rank: number, axis: number): Tensor;
    /** Builds an n by n line, free on axis 0, on along the odd columns. */
    export function vline_2d(n: number): Tensor;
    /** Builds an n by n void, on where both coordinates share one parity. */
    export function void_2d(n: number): Tensor;
    /** Builds an n by n by n void, on where all three coordinates share one parity. */
    export function void_3d(n: number): Tensor;
    /** Builds a void of the given side at any rank, on where every coordinate shares one parity. */
    export function void_nd(n: number, rank: number): Tensor;
    /** Builds an n by n tree, free on axis 0, on along the even columns. */
    export function vtree_2d(n: number): Tensor;
    /** Builds an n by n by n line, free on axis 0, its rods running along x. */
    export function xline_3d(n: number): Tensor;
    /** Builds an n by n by n tree, free on axis 0, its beams running along x. */
    export function xtree_3d(n: number): Tensor;
    /** Builds an n by n by n line, free on axis 1, its rods running along y. */
    export function yline_3d(n: number): Tensor;
    /** Builds an n by n by n tree, free on axis 1, its beams running along y. */
    export function ytree_3d(n: number): Tensor;
    /** Builds an n by n tensor of zeros. */
    export function zeros_2d(n: number): Tensor;
    /** Builds an n by n by n tensor of zeros. */
    export function zeros_3d(n: number): Tensor;
    /** Builds an n by n by n line, free on axis 2, its rods running along z. */
    export function zline_3d(n: number): Tensor;
    /** Builds an n by n by n tree, free on axis 2, its beams running along z. */
    export function ztree_3d(n: number): Tensor;
}
export declare namespace bang {
    /** Builds the universe of a dimension. */
    export function bang(dimension: number): bang.Universe;
    /** Unpacks a code into its filled residue corners. */
    export function code_to_corners(code: string | number | bigint, dimension: number, base: number): Uint8Array[];
    /** Returns the binary corners of a dimension in code order. */
    export function corners(dimension: number): Uint8Array[];
    /** Packs filled residue corners back into their code. */
    export function corners_to_code(filled: ArrayLike<number>[], dimension: number, base: number): string;
    /** Returns the code of the design filled wherever a corner's residue sum lands in the levels. */
    export function levels_code(dimension: number, base: number, levels: ArrayLike<number>): string;
    /** Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost. */
    export function magic(layers: bang.MagicLayer[]): Tensor;
    /** Composes JSON-named layers in order. */
    export function magic_named(layers: [string, number][]): Tensor;
    /** Builds the tile sources a catalog names at a dimension. */
    export function sources(catalog: gen.recipe.Catalog, dimension: number): gen.recipe.Source[];
    /** Returns the full symmetry group as axis permutations paired with flip patterns. */
    export function symmetries(dimension: number): [Uint32Array, Uint8Array][];
    /** Returns whether no two filled corners of a code sit at Hamming distance one. */
    export function total_exposure(code: string | number | bigint, dimension: number): boolean;
    /** Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side. */
    export function touches_every_corner(code: string | number | bigint, dimension: number): boolean;
    /** Returns the canonical design codes of a dimension, computed once and cached for the process. */
    export function universe_codes(dimension: number): string[];
    export interface DesignData {
        /** The design's code. */
        i: bigint;
        /** The design's dimension. */
        dimension: number;
        /** Whether this code is the smallest in its orbit. */
        canonical: boolean;
        /** The smallest code in the orbit. */
        class_rep: bigint;
        /** The number of codes in the orbit. */
        orbit_size: number;
    }
    /** A single design with its place in the orbit structure. */
    export class Design {
        private constructor();
        free(): void;
        /** Reads the Design from its plain data. */
        static from(data: DesignData): Design;
        /** Writes the Design as plain data. */
        toJSON(): DesignData;
        /** The design's code. */
        get i(): string;
        set i(value: string | number | bigint);
        /** The design's dimension. */
        get dimension(): number;
        set dimension(value: number);
        /** Whether this code is the smallest in its orbit. */
        get canonical(): boolean;
        set canonical(value: boolean);
        /** The smallest code in the orbit. */
        get class_rep(): string;
        set class_rep(value: string | number | bigint);
        /** The number of codes in the orbit. */
        get orbit_size(): number;
        set orbit_size(value: number);
        /** Returns the design's algebraic normal form as a string. */
        anf(): string;
        /** Returns the design's algebraic degree, or -1 for the zero design. */
        degree(): number;
        /** Returns the design's name as a line of prose, `bang dim 2, code 7`. */
        name(): string;
        /** Returns the design's filled corners in sorted order. */
        rule(): Uint8Array[];
    }
    /** One ordered layer of a magic composition: a coded design at its own side number. */
    export interface MagicLayer {
        /** The layer's coded design. */
        design: name.BangData;
        /** The layer's side number. */
        number: number;
    }
    export const MagicLayer: {
        /** Pins a design to the side number it renders at. */
        "new"(design: name.Bang, number: number): bang.MagicLayer;
    };
    export type UniverseData = Record<string, unknown>;
    /** The complete enumeration of one dimension's designs and orbits. */
    export class Universe {
        /** Enumerates every orbit of a dimension from 1 to 4. */
        constructor(dimension: number);
        free(): void;
        /** Reads the Universe from its plain data. */
        static from(data: UniverseData): Universe;
        /** Writes the Universe as plain data. */
        toJSON(): UniverseData;
        /** The universe's dimension. */
        get dimension(): number;
        set dimension(value: number);
        /** The number of codes in the universe. */
        get total(): number;
        set total(value: number);
        /** Returns every design in code order. */
        all(): bang.Design[];
        /** Returns the designs whose codes lead their orbits. */
        canonical(): bang.Design[];
        /** Returns the design at a code with its precomputed orbit facts. */
        design(code: string | number | bigint): bang.Design;
        /** Returns the number of distinct orbits. */
        distinct(): number;
    }
    export namespace baseq {
        /** Returns the distinct rotation and reflection maps of a base-q axis. */
        export function axis_maps(base: number): Uint32Array[];
        /** Returns the distinct one-dimensional design counts for bases 1 through max_base. */
        export function bracelets(max_base: number): string[];
        /** Returns the least code of the design's orbit. */
        export function canonical(group: ArrayLike<number>[], code: string | number | bigint): string;
        /** Carries a code through one group element. */
        export function carry(element: ArrayLike<number>, code: string | number | bigint): string;
        /** Returns the fill-class counts for dimensions 1 through max_dimension. */
        export function class_sequence(max_dimension: number): string[];
        /** Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension. */
        export function classes(dimension: number): string;
        /** Counts base-q designs distinct under symmetry. */
        export function distinct_designs(base: number, dimension: number): string;
        /** Returns the collapsed fill count at an even side number. */
        export function even_fill_is_balanced(number: number, dimension: number, popcount: string | number | bigint): string;
        /** Returns the filled-cell count of a binary design at a side number, folded from its filled corners. */
        export function fill_from_corners(filled: ArrayLike<number>[], number: number, dimension: number): string;
        /** Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`. */
        export function group(base: number, dimension: number): Uint32Array[];
        /** Returns the symmetry group order counted from the enumerated axis maps. */
        export function group_order(base: number, dimension: number): string;
        /** Returns every code a design reaches under the group. */
        export function orbit(group: ArrayLike<number>[], code: string | number | bigint): string[];
        /** Returns the closed-form group order the axis-map count must match. */
        export function predicted_group_order(base: number, dimension: number): string;
        /** Walks every code of a base and dimension and returns each orbit's least code with the orbit's size. */
        export function representatives(base: number, dimension: number): [string, number][];
        /** Returns the distinct-design counts for dimensions 1 through max_dimension. */
        export function sequence(base: number, max_dimension: number): string[];
        /** Returns the raw design count before symmetry, two to the number of cells. */
        export function total_designs(base: number, dimension: number): string;
        /** The most cells a code walk visits, so that the walk stays within `2^20` codes. */
        export function WALK_LIMIT(): number;
    }
    export namespace catalog {
        /** Returns the anti designs for a dimension. */
        export function antis(dimension: number): gen.recipe.Design[];
        /** The five antis of the plane, the complements of the five classics in order. */
        export function ANTIS_2D(): gen.recipe.Design[];
        /** The six antis of the cube: point, dust, the three lines and the star. */
        export function ANTIS_3D(): gen.recipe.Design[];
    }
    export namespace code {
        /** Returns the bitmask the code carries. */
        export function get(code: string | number | bigint): string;
    }
    export namespace factory {
        /** Renders a coded design to a tensor at its side number, dimension, base and fractal level. */
        export function create(code: string | number | bigint, number: number, dimension: number, base: number, level: number): Tensor;
        /** Renders a design straight from its filled residue corners. */
        export function create_from_corners(filled: ArrayLike<number>[], number: number, dimension: number, base: number, level: number): Tensor;
        /** Renders a design from its canonical JSON name. */
        export function create_named(spec: string, number: number, level: number): Tensor;
        /** Returns every base-q residue corner of a dimension in row-major order. */
        export function residue_corners(dimension: number, base: number): Uint8Array[];
        /** Returns the code count of a dimension and base, two to the number of corners. */
        export function total_codes(dimension: number, base: number): string;
    }
    export namespace universe {
        /** Returns the algebraic normal form coefficients of a code, one per corner. */
        export function anf(code: string | number | bigint, dimension: number): Uint8Array;
        /** Formats the algebraic normal form of a code as a sum of monomials. */
        export function anf_string(code: string | number | bigint, dimension: number): string;
        /** Applies a symmetry element to a corner. */
        export function apply(element: [ArrayLike<number>, ArrayLike<number>], corner: ArrayLike<number>): Uint8Array;
        /** Returns the bit position a binary corner occupies in a code. */
        export function corner_index(corner: ArrayLike<number>): number;
        /** Returns the algebraic degree of a code, or -1 for the zero design. */
        export function degree(code: string | number | bigint, dimension: number): number;
        /** Returns every code a design reaches under the full symmetry group. */
        export function orbit(code: string | number | bigint, dimension: number): string[];
        /** Returns every permutation of 0..n in sorted order. */
        export function permutations(n: number): Uint32Array[];
    }
    export namespace word {
        /** Counts the 4-connected components of a plane word without drawing it. */
        export function components(layers: bang.MagicLayer[]): string;
        /** Returns the constant-word component functional of a plane word's letter frequencies, */
        export function constant_functional(layers: bang.MagicLayer[]): number;
        /** Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides. */
        export function dimension(layers: bang.MagicLayer[]): number;
        /** Returns the filled cells of a word, the product of its letter fills. */
        export function fill(layers: bang.MagicLayer[]): string;
        /** Lists the filled cells of every letter, the product of which is the word's fill. */
        export function fills(layers: bang.MagicLayer[]): string[];
        /** Reads one plane letter: its fill, its runs, the rows and columns that wrap into a */
        export function letter(layer: bang.MagicLayer): bang.word.Letter;
        /** Returns whether every letter renders at its own residue base, the native case where a */
        export function native(layers: bang.MagicLayer[]): boolean;
        /** Returns the shortest whole period of the letter list, its own length when no shorter block repeats. */
        export function period(layers: bang.MagicLayer[]): number;
        /** Folds a plane word letter by letter and returns the counts at every prefix. */
        export function prefixes(layers: bang.MagicLayer[]): bang.word.Counts[];
        /** Returns the prefix rates of a plane word in log two units, the component rate */
        export function rates(layers: bang.MagicLayer[]): [number, number][];
        /** Returns the side of a word, the product of its letter sides. */
        export function side(layers: bang.MagicLayer[]): string;
        /** Spells the first letters of a schedule over an ordered pair of letters. */
        export function spell(schedule: bang.word.Schedule, pair: [bang.MagicLayer, bang.MagicLayer], length: number): bang.MagicLayer[];
        /** Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`, */
        export function staircase(depth: number): bang.MagicLayer[];
        /** Returns the Thue-Morse letter at the place, the parity of its binary digit sum. */
        export function thue_morse(index: number): number;
        /** The counts a word carries at one prefix length. */
        export interface Counts {
            /** The side, the product of the prefix's letter sides. */
            side: bigint;
            /** The filled cells, the product of the prefix's letter fills. */
            fill: bigint;
            /** The 4-connected components. */
            components: bigint;
            /** The maximal horizontal runs of filled cells. */
            runs_h: bigint;
            /** The maximal vertical runs of filled cells. */
            runs_v: bigint;
        }
        /** The plane geometry of one letter, the numbers a word's counts fold through. */
        export interface Letter {
            /** The count of filled cells. */
            fill: bigint;
            /** The count of maximal horizontal runs of filled cells. */
            runs_h: bigint;
            /** The count of maximal vertical runs of filled cells. */
            runs_v: bigint;
            /** The count of rows whose first and last cells are both filled. */
            touch_h: bigint;
            /** The count of columns whose first and last cells are both filled. */
            touch_v: bigint;
            /** The count of 4-connected components. */
            components: bigint;
        }
        /** The named infinite schedules over an ordered pair of letters. */
        export type Schedule = "ThueMorse" | "Periodic" | "Constant";
        export const Schedule: {
            /** Returns every Schedule in canonical order. */
            all(): bang.word.Schedule[];
            /** Returns the letter frequencies the schedule tends to. */
            frequencies(schedule: bang.word.Schedule): [number, number];
            /** Returns the letter the schedule takes at the place, zero or one. */
            place(schedule: bang.word.Schedule, index: number): number;
        };
    }
}
export declare namespace cell {
    /** Grows a seed pattern into a cell, deepened to its fractal past level one. */
    export function grow(pattern: Tensor, level: number): Cell;
    /** Colors the cell through the given mapping and mode, defaulting to the standard palette by type. */
    export function paint(cell: Cell, custom?: Record<string, Color[]>, mode?: core.Mode, rng?: Rng): Cell;
    export namespace census {
        /** Counts the distinct unit edges the filled sites carry, the edge graph's branches. */
        export function edges(cell: Cell): number;
        /** Counts the faces of filled sites open to emptiness or the border. */
        export function exposure(cell: Cell): string;
        /** Counts the filled sites of the cell. */
        export function fills(cell: Cell): number;
        /** Counts the distinct corners the filled sites touch, the edge graph's nodes. */
        export function vertices(cell: Cell): number;
        /** Counts the empty sites of the cell. */
        export function voids(cell: Cell): number;
    }
    export namespace geometry {
        /** Merges same-shaped cells into one block laid out by the per-axis repetition counts. */
        export function merge_reps(cells: Cell[], reps: ArrayLike<number>): Cell;
        /** Writes the value into the cell wherever the tiled mask is nonzero. */
        export function perforate(mask: Tensor, cell: Cell, value: number): Cell;
    }
    export namespace models {
        /** Inverts the cell. */
        export function anti(cell: Cell): Cell;
        /** Maps each site to one at or above the threshold, zero below. */
        export function binarize(cell: Cell, threshold: number): Cell;
        /** Binarizes the cell at the threshold Otsu's method picks. */
        export function binarize_otsu(cell: Cell): Cell;
        /** Rounds each site to the mean of its masked neighborhood, wrapping on request. */
        export function blur(cell: Cell, mask: Tensor, wrap: boolean): Cell;
        /** Returns the Kronecker product of the two cells. */
        export function combine(cell: Cell, other: Cell): Cell;
        /** Returns the narrowest count dtype that fits the mask's popcount. */
        export function counting_dtype(mask: Tensor): core.Dtype;
        /** Returns the size of axis 0, the cube's leading axis. */
        export function depth(cell: Cell): number;
        /** Returns the narrowest unsigned dtype that holds the peak value. */
        export function dtype_for(peak: number | bigint): core.Dtype;
        /** Deepens the cell into its level-fold fractal. */
        export function fractal(cell: Cell, level: number): Cell;
        /** Returns the size of the axis before the last. */
        export function height(cell: Cell): number;
        /** Swaps filled and empty sites. */
        export function invert(cell: Cell): Cell;
        /** Tags each site with its ring distance from the center. */
        export function layers(cell: Cell): Cell;
        /** Tags each site with its count of masked neighbors matching the target, wrapping on request. */
        export function neighbors(cell: Cell, mask: Tensor, target: number, wrap: boolean): Cell;
        /** Builds a cell from an N-dimensional tensor of types. */
        export function new_(types: Tensor): Cell;
        /** Turns the cell into one of the 24 cube orientations. */
        export function orient(cell: Cell, index: number): Cell;
        /** Wraps the cell in count layers of the given value on every side. */
        export function pad(cell: Cell, count: number, value: number): Cell;
        /** Colors each site by its type through the mapping in the given mode. */
        export function paint(cell: Cell, mapping: Record<string, Color[]>, mode: core.Mode, rng?: Rng): Cell;
        /** Writes the value wherever the tiled mask is nonzero. */
        export function perforate(cell: Cell, mask: Tensor, value: number): Cell;
        /** Rotates the cell k quarter turns in the plane. */
        export function rotate(cell: Cell, k: number, axes?: [number, number]): Cell;
        /** Repeats the cell into a width-by-height array of copies. */
        export function tile(cell: Cell, width: number, height: number, depth?: number): Cell;
        /** Returns the tensor of types. */
        export function types(cell: Cell): Tensor;
        /** Returns the size of the last axis. */
        export function width(cell: Cell): number;
    }
    export namespace serializer {
        /** Reads a triply nested JSON array into layers of byte rows. */
        export function byte_cube(value: any): Uint8Array[][];
        /** Reads a nested JSON array into rows of bytes. */
        export function byte_grid(value: any): Uint8Array[];
        /** Reads a nested JSON array into rows of four-channel colors. */
        export function color_grid(value: any): Uint8Array[][];
        /** Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits. */
        export function count_cube(value: any): BigInt64Array;
        /** Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits. */
        export function count_grid(value: any): BigInt64Array;
        /** Parses JSON text into a value tree. */
        export function parse(text: string): any;
        /** Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them. */
        export function tag_layer(counts: ArrayLike<number | bigint>, shape: ArrayLike<number>): Tensor;
        /** Returns the types field of the data. */
        export function types_field(data: any): any;
    }
}
export declare namespace core {
    /** A rule that turns counter values into colors. */
    export type Colorizer = { Bins: { background: ColorData; ramp: ColorData[] } };
    /** The element widths a tensor can hold. */
    export type Dtype = "U8" | "U16" | "U32" | "I32";
    /** The ways paint picks a color within a type's palette. */
    export type Mode = "Type" | "Tag" | "Index" | "Enumerate" | "Random" | "Row" | "Column" | "Depth";
}
export declare namespace counts {
    /** Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1. */
    export function centered_hexagonal(m: number): string;
    /** Returns the filled triangle count of the code's cut section at the given level, without rendering it. */
    export function cut_fills(code: string | number | bigint, number: number, level: number): string;
    /** Returns the empty triangle count of the code's cut section at the given level. */
    export function cut_voids(code: string | number | bigint, number: number, level: number): string;
    /** Returns the code's fractal dimension, the log of its one-level fill over the log of number. */
    export function dimension(code: string | number | bigint, number: number, base_dimension: number, base: number): number;
    /** Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term */
    export function edges_of_tile(tile: Tensor, level: number): string | undefined;
    /** Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners. */
    export function exposure(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    /** Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128. */
    export function exposure_of_tile(tile: Tensor, level: number): string | undefined;
    /** Returns the coefficients of the recurrence the tile's exposure obeys. */
    export function exposure_recurrence(tile: Tensor): string[];
    /** Returns the filled cell count of the code's fractal at the given level, without rendering it. */
    export function fill(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    /** Sums each corner's position products into a base fill and raises it to the level. */
    export function fill_from_corners(filled: ArrayLike<number>[], number: number, _dimension: number, level: number, base: number): string;
    /** Returns the total cells of the grid, number to the dimension, to the level. */
    export function grid(number: number, dimension: number, level: number): string;
    /** Returns the fill ratio the code walks toward as the side number grows, reduced. */
    export function limit(code: string | number | bigint, dimension: number, level: number, base: number): [string, string];
    /** Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled. */
    export function pairs(tile: Tensor): [string, string][];
    /** Counts the indices below number that equal residue modulo base. */
    export function positions(residue: number, number: number, base: number): string;
    /** Returns the filled triangle count of the code's pro projection at the given level, without rendering it. */
    export function pro_fills(code: string | number | bigint, number: number, level: number): string;
    /** Returns the empty triangle count of the code's pro projection at the given level. */
    export function pro_voids(code: string | number | bigint, number: number, level: number): string;
    /** Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`. */
    export function profile_of_tile(tile: Tensor, level: number): string[];
    /** Returns the filled fraction of the grid, or 0.0 for an empty grid. */
    export function ratio(code: string | number | bigint, number: number, dimension: number, level: number, base: number): number;
    /** Returns the exact filled fraction as a fraction of fill over grid, reduced. */
    export function rational(code: string | number | bigint, number: number, dimension: number, level: number, base: number): [string, string];
    /** Returns the exposed face count of the code's 3D fractal at the given level. */
    export function surface(code: string | number | bigint, number: number, level: number, base: number): string;
    /** Returns the empty cell count, grid minus fill. */
    export function void_(code: string | number | bigint, number: number, dimension: number, level: number, base: number): string;
    export interface ExposureData {
        /** The filled cells of the tile. */
        occupancy: bigint;
        /** The exposed faces of the tile. */
        exposed: bigint;
        /** Per axis, the adjacent filled pairs and the spanning positions. */
        axes: [bigint, bigint][];
    }
    /** The counts the exposure recurrence runs on: the filled cells and exposed faces of the tile, and per axis its adjacent pairs and spanning positions. */
    export class Exposure {
        private constructor();
        free(): void;
        /** Reads the Exposure from its plain data. */
        static from(data: ExposureData): Exposure;
        /** Writes the Exposure as plain data. */
        toJSON(): ExposureData;
        /** The filled cells of the tile. */
        get occupancy(): string;
        set occupancy(value: string | number | bigint);
        /** The exposed faces of the tile. */
        get exposed(): string;
        set exposed(value: string | number | bigint);
        /** Per axis, the adjacent filled pairs and the spanning positions. */
        get axes(): [string, string][];
        set axes(value: ([string | number | bigint, string | number | bigint])[]);
        /** Returns the exposed faces of the level-fold Kronecker power, or none past a u128. */
        at(level: number): string | undefined;
        /** Folds the counts from the filled residue corners at a side number, without rendering the tile. */
        static from_corners(filled: ArrayLike<number>[], number: number, dimension: number, base: number): counts.Exposure;
        /** Reads the counts off a rendered tile. */
        static of_tile(tile: Tensor): counts.Exposure;
        /** Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys. */
        recurrence(): string[];
    }
    export namespace diagonal {
        /** The widest side a profile spans. */
        export function WIDEST(): number;
    }
    export namespace ladder {
        /** The widest dimension the exact carry arithmetic reaches at the base. */
        export function cap(base: number): number;
        /** The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out. */
        export function carry_matrix(base: number, dimension: number): string[][];
        /** The monic characteristic polynomial of a square integer matrix, highest power first. */
        export function characteristic(rows: ((string | number | bigint)[])[]): string[];
        /** The determinant of a square integer matrix, read off its characteristic polynomial. */
        export function determinant(rows: ((string | number | bigint)[])[]): string;
        /** The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first. */
        export function digit_polynomial(base: number, dimension: number): string[];
        /** The reflection-even block of the carry matrix, of size `ceil(D/2)`. */
        export function even_block(base: number, dimension: number): string[][];
        /** The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`. */
        export function fill(base: number, dimension: number): string;
        /** The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`. */
        export function ladder(base: number, dimension: number, levels: number): string[];
        /** The Perron root of a nonnegative square integer matrix. */
        export function perron(rows: ((string | number | bigint)[])[]): number;
        /** The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers. */
        export function sign(base: number, dimension: number): number;
        /** The Perron root over the modulus of the second eigenvalue, or none where the block is one wide. */
        export function spectral_ratio(base: number, dimension: number): number | undefined;
        /** The trace of a square integer matrix. */
        export function trace(rows: ((string | number | bigint)[])[]): string;
    }
    export namespace six {
        /** Returns the triangles of the full hexagon with side number to the level. */
        export function grid_triangles(number: number, level: number): string;
        /** Returns the boundary edge count of the solid slice, defined for odd number. */
        export function solid_slice_boundary(number: number): string;
        /** Returns the core edge count of the solid slice, defined for odd number. */
        export function solid_slice_core_edges(number: number): string;
        /** Returns the core node count of the solid slice, defined for odd number. */
        export function solid_slice_core_nodes(number: number): string;
        /** Returns the distinct triangle-edge count of the solid slice, defined for odd number. */
        export function solid_slice_edges(number: number): string;
        /** Returns the interior edge count of the solid slice, defined for odd number. */
        export function solid_slice_interior(number: number): string;
        /** Returns the triangle count of the solid slice, defined for odd number. */
        export function solid_slice_triangles(number: number): string;
        /** Returns the vertex count of the solid slice, defined for odd number. */
        export function solid_slice_vertices(number: number): string;
    }
}
export declare namespace gen {
    export namespace recipe {
        /** The pool of sources a tile may draw from. */
        export type Catalog = "Classics" | "Universe" | { Codes: bigint[] } | { Designs: gen.recipe.Design[] };
        /** The named designs a source can point at: the four classics and their four antis. */
        export type Design = "Carpet" | "Net" | "Htree" | "Vtree" | "Void" | "Xtree" | "Ytree" | "Ztree" | "Point" | "Dust" | "Hline" | "Vline" | "Star" | "Xline" | "Yline" | "Zline";
        /** The origin of one tile layer, a one-field json object. */
        export type Source = { design: gen.recipe.Design } | { code: bigint };
    }
}
export declare namespace graph {
    /** Takes the full census of a network. */
    export function census(network: graph.Network): graph.Census;
    /** Counts the connected components of the network. */
    export function components(network: graph.Network): number;
    /** Extracts the network of filled sites joined to their axis neighbors. */
    export function core_graph(grid: Tensor): graph.Network;
    /** Extracts the network of corners and edges outlining every filled site. */
    export function edge_graph(grid: Tensor): graph.Network;
    /** Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample. */
    export function fractal_dimension(network: graph.Network, samples: number): number;
    /** Counts the nodes of degree three or more. */
    export function junctions(network: graph.Network): number;
    /** Extracts the largest connected piece as a network of its own, branches re-indexed. */
    export function largest_component(network: graph.Network): graph.Network;
    /** Tags every node by its degree, indexed like the node list. */
    export function roles(network: graph.Network): graph.Role[];
    /** Counts the nodes of degree one. */
    export function tips(network: graph.Network): number;
    /** Sums the straight-line lengths of every branch. */
    export function total_length(network: graph.Network): number;
    /** Extracts the core graph of the inverted grid, joining empty sites instead. */
    export function tunnel_graph(grid: Tensor): graph.Network;
    /** A link between two nodes. */
    export interface Branch {
        /** The index of the node the branch leaves. */
        parent: number;
        /** The index of the node the branch reaches. */
        child: number;
        /** The thickness of the branch. */
        radius: number;
    }
    /** The measurements of one network. */
    export interface Census {
        /** The node count. */
        nodes: number;
        /** The branch count. */
        branches: number;
        /** The count of degree-one nodes. */
        tips: number;
        /** The count of nodes of degree three or more. */
        junctions: number;
        /** The connected component count. */
        components: number;
        /** The summed branch length. */
        total_length: number;
        /** The box-counting dimension estimate. */
        fractal_dimension: number;
    }
    export type LayoutData = Record<string, unknown>;
    /** A force-directed layout: every node repels every other, every branch pulls its ends together, and a cooling cap on the move per tick lets the lattice settle. */
    export class Layout {
        /** Starts a layout from flat positions, `dim` floats per node, and the branch pairs. */
        constructor(positions: ArrayLike<number>, branches: [number, number][], dim: number, seed: number | bigint);
        free(): void;
        /** Returns the mean net force per node in units of `k` after the last tick. */
        energy(): number;
        /** Starts a layout from a network's own positions and branches. */
        static from_network(network: graph.Network, seed: number | bigint): graph.Layout;
        /** Returns the ideal branch length `k`. */
        ideal(): number;
        /** Returns the mean distance a node moved in the last tick. */
        moved(): number;
        /** Returns the node count. */
        nodes(): number;
        /** Returns the positions, `dim` floats per node. */
        positions(): Float64Array;
        /** Runs the ticks and returns the energy left: the mean net force per node in units of `k`. */
        step(ticks: number): number;
        /** Returns the cap on one node's move in the next tick. */
        temperature(): number;
        /** Returns the ticks stepped so far. */
        ticks(): number;
    }
    export interface NetworkData {
        /** The dimension every position must match. */
        dim: number;
        /** The nodes in insertion order. */
        nodes: graph.Node[];
        /** The branches in insertion order. */
        branches: graph.Branch[];
    }
    /** A spatial graph of nodes and branches. */
    export class Network {
        /** Builds an empty network of the given dimension. */
        constructor(dim: number);
        free(): void;
        /** Reads the Network from its plain data. */
        static from(data: NetworkData): Network;
        /** Writes the Network as plain data. */
        toJSON(): NetworkData;
        /** The dimension every position must match. */
        get dim(): number;
        set dim(value: number);
        /** The nodes in insertion order. */
        get nodes(): graph.Node[];
        set nodes(value: graph.Node[]);
        /** The branches in insertion order. */
        get branches(): graph.Branch[];
        set branches(value: graph.Branch[]);
        /** Appends a branch between two node indices. */
        add_branch(parent: number, child: number, radius: number): void;
        /** Appends a node at the position and returns its index. */
        add_node(position: ArrayLike<number>): number;
        /** Returns the undirected neighbor lists of every node. */
        adjacency(): Record<string, Uint32Array>;
        /** Returns each node's branch count, indexed like the node list. */
        degree(): Uint32Array;
    }
    /** A point of the network. */
    export interface Node {
        /** The coordinates, one per dimension. */
        position: number[];
        /** The node's place in the network's list. */
        index: number;
    }
    /** What a node's degree makes it. */
    export type Role = "Alone" | "Tip" | "Through" | "Junction";
}
export declare namespace moire {
    /** Returns every preset stacked up to the given scale. */
    export function all(limit: number): moire.Preset[];
    /** Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal. */
    export function frame(normal: ArrayLike<number>, offset: number): moire.Frame;
    /** Samples a design over the pixel grid into a boolean mask. */
    export function layer(params: moire.Layer): boolean[];
    /** Returns the preset the name picks. */
    export function named(name: string, limit: number): moire.Preset;
    /** Quantizes a field into colored levels and encodes PNG bytes. */
    export function render(field: moire.Field, colorizer: core.Colorizer, levels: number, symmetric: boolean, invert: boolean, scale: number): Uint8Array;
    /** Layers one design at several side numbers into a field under the chosen combine. */
    export function stack(spec: moire.Spec, numbers: ArrayLike<number>, combine: moire.Combine, level: number, lattice: moire.Lattice, size: number, slices: ArrayLike<number>): moire.Field;
    /** Sums layers of several designs at one side number into a field. */
    export function stack_codes(specs: moire.Spec[], number: number, level: number, lattice: moire.Lattice, size: number, slices: ArrayLike<number>): moire.Field;
    /** Layers one cube design at several side numbers into a volume under the chosen combine. */
    export function volume(spec: moire.Spec, numbers: ArrayLike<number>, combine: moire.Combine, level: number, size: number): moire.Volume;
    /** The way stacked layers merge. */
    export type Combine = "Sum" | "And" | "Xor";
    export interface FieldData {
        /** The samples in row-major order. */
        data: number[];
        /** The side length in samples. */
        size: number;
    }
    /** A square grid of f32 samples. */
    export class Field {
        /** Builds a zeroed field of the given side. */
        constructor(size: number);
        free(): void;
        /** Reads the Field from its plain data. */
        static from(data: FieldData): Field;
        /** Writes the Field as plain data. */
        toJSON(): FieldData;
        /** The samples in row-major order. */
        get data(): Float32Array;
        set data(value: ArrayLike<number>);
        /** The side length in samples. */
        get size(): number;
        set size(value: number);
        /** Returns the samples widened to f64. */
        as_f64(): Float64Array;
        /** Wraps row-major samples of the given side. */
        static from_data(data: ArrayLike<number>, size: number): moire.Field;
        /** Returns the largest sample. */
        max(): number;
        /** Returns the mean sample, or zero for an empty field. */
        mean(): number;
        /** Returns the smallest sample. */
        min(): number;
        /** Returns the samples scaled into 0..1, symmetric about zero on request. */
        normalized(symmetric: boolean): Float32Array;
    }
    /** A plane through the unit box, framed for sampling: its centre, its two in-plane axes and the width of the square window that holds the whole section, all in the box `[-1, 1]^3`. */
    export interface Frame {
        /** The point of the plane the window is centred on. */
        centre: number[];
        /** The unit axis the window's columns run along. */
        u: number[];
        /** The unit axis the window's rows run along. */
        v: number[];
        /** The unit normal. */
        normal: number[];
        /** The side of the square window. */
        width: number;
    }
    /** The sampling lattice of a moire field. */
    export type Lattice = "Square" | "Hex";
    /** The recipe for one moire layer. */
    export interface Layer {
        /** The design to sample. */
        spec: moire.Spec;
        /** The side number of the residue grid. */
        number: number;
        /** The fractal depth. */
        level: number;
        /** The sampling lattice. */
        lattice: moire.Lattice;
        /** The output side in pixels. */
        size: number;
        /** The 0..1 positions fixing the axes beyond the first two. */
        slices: number[];
    }
    export const Layer: {
        /** Builds a layer at level 1 on a 512-pixel square lattice. */
        "new"(spec: moire.Spec, number: number): moire.Layer;
    };
    export interface PresetData {
        /** The name the recipe answers to. */
        name: string;
        /** The design sampled at every scale. */
        spec: moire.Spec;
        /** The side numbers stacked. */
        numbers: number[];
        /** The way the layers merge. */
        combine: moire.Combine;
        /** The fractal depth of each layer. */
        level: number;
        /** The lattice the layers are sampled on. */
        lattice: moire.Lattice;
    }
    /** One named moire recipe: the design, the scales it stacks and the lattice it samples. */
    export class Preset {
        private constructor();
        free(): void;
        /** Writes the Preset as plain data. */
        toJSON(): PresetData;
        /** The name the recipe answers to. */
        readonly name: string;
        /** The design sampled at every scale. */
        get spec(): moire.Spec;
        set spec(value: moire.Spec);
        /** The side numbers stacked. */
        get numbers(): Uint32Array;
        set numbers(value: ArrayLike<number>);
        /** The way the layers merge. */
        get combine(): moire.Combine;
        set combine(value: moire.Combine);
        /** The fractal depth of each layer. */
        get level(): number;
        set level(value: number);
        /** The lattice the layers are sampled on. */
        get lattice(): moire.Lattice;
        set lattice(value: moire.Lattice);
        /** The carpet stack: every base-three corner but the centre, summed over odd scales. */
        static carpet(limit: number): moire.Preset;
        /** Samples the preset into a square field of the given side. */
        field(size: number): moire.Field;
        /** The parity heatmap: odd scales of the low corner summed on the square lattice. */
        static heatmap(limit: number): moire.Preset;
        /** The hive: the parity heatmap sampled on the hexagonal lattice. */
        static hive(limit: number): moire.Preset;
        /** The parity weave: the same odd scales folded to their parity instead of summed. */
        static weave(limit: number): moire.Preset;
    }
    /** The identity of a design: its code, base and dimension. */
    export interface Spec {
        /** The design code. */
        code: bigint;
        /** The residue base. */
        base: number;
        /** The design dimension. */
        dimension: number;
    }
    export const Spec: {
        /** Builds a spec from a code, base and dimension. */
        "new"(code: string | number | bigint, base: number, dimension: number): moire.Spec;
    };
    export interface VolumeData {
        /** The samples, x-major, then y, then z. */
        data: number[];
        /** The side in samples. */
        size: number;
    }
    /** A cubic grid of f32 samples, x-major. */
    export class Volume {
        /** Builds a zeroed volume of the side. */
        constructor(size: number);
        free(): void;
        /** Reads the Volume from its plain data. */
        static from(data: VolumeData): Volume;
        /** Writes the Volume as plain data. */
        toJSON(): VolumeData;
        /** The samples, x-major, then y, then z. */
        get data(): Float32Array;
        set data(value: ArrayLike<number>);
        /** The side in samples. */
        get size(): number;
        set size(value: number);
        /** Reads the sample at a voxel. */
        at(x: number, y: number, z: number): number;
        /** Counts the samples at or above the level. */
        count(level: number): number;
        /** Wraps x-major samples of the side. */
        static from_data(data: ArrayLike<number>, size: number): moire.Volume;
        /** Returns the largest sample. */
        max(): number;
        /** Returns the smallest sample. */
        min(): number;
        /** Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube. */
        plane(frame: moire.Frame, out: number): [Float32Array, Uint8Array];
        /** Reads the voxel a point of the unit cube falls in, or zero outside it. */
        sample(p: ArrayLike<number>): number | undefined;
        /** Thresholds into a byte tensor: one where a sample reaches the level, zero below. */
        solid(level: number): Tensor;
    }
    export namespace pairs {
        /** Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid. */
        export function correlation(m: number, n: number): number;
        /** Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated. */
        export function sampled(m: number, n: number): number;
        /** Puts an odd scale of three or more on trial against every earlier odd scale. */
        export function witness(scale: number): moire.pairs.Witness;
        /** The witness row of an odd scale: its correlation with every earlier odd scale from three, and the verdict the row gives. */
        export interface Witness {
            /** The scale on trial. */
            scale: number;
            /** The earlier odd scales, three up to the scale less two. */
            scales: number[];
            /** The exact correlation with each earlier scale. */
            row: number[];
            /** The largest correlation in the row, zero for an empty row. */
            max: number;
            /** The earlier scale carrying the largest correlation, zero when the row is clear. */
            at: number;
            /** Whether the row is exactly clear, which is the scale being prime. */
            prime: boolean;
        }
    }
    export namespace sample {
        /** Returns the two lattice coordinates of each pixel centre along a row. */
        export function axes(size: number, lattice: moire.Lattice, row: number): [Float64Array, Float64Array];
        /** Unpacks a code into its residue-corner truth table. */
        export function membership(code: string | number | bigint, base: number, dimension: number): boolean[];
        /** Folds residues into a base-q index of the truth table. */
        export function pack(residues: ArrayLike<number>, base: number): number;
    }
}
export declare namespace name {
    export interface BangData {
        /** The kind word. */
        kind: string;
        /** The number of axes. */
        dim: number;
        /** The lattice, square unless said. */
        lattice: name.Lattice;
        /** The digits per axis, 2 unless said. */
        base: number;
        /** The design as a number. */
        code: bigint;
        /** One unit index per filled digit, absent when nothing turns. */
        twist?: number[];
    }
    /** A design code pinned to its dimension, lattice and base, with one unit index per filled digit when it twists. */
    export class Bang {
        /** Pins a code to its dimension and base on the square lattice. */
        constructor(code: string | number | bigint, dim: number, base: number);
        free(): void;
        /** Reads the Bang from its plain data. */
        static from(data: BangData): Bang;
        /** Writes the Bang as plain data. */
        toJSON(): BangData;
        /** The number of axes. */
        get dim(): number;
        set dim(value: number);
        /** The lattice, square unless said. */
        get lattice(): name.Lattice;
        set lattice(value: name.Lattice);
        /** The digits per axis, 2 unless said. */
        get base(): number;
        set base(value: number);
        /** The design as a number. */
        get code(): string;
        set code(value: string | number | bigint);
        /** One unit index per filled digit, absent when nothing turns. */
        get twist(): Uint32Array | undefined;
        set twist(value: ArrayLike<number> | undefined);
        /** Returns the number of digits the code addresses. */
        cells(): number;
        /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
        checked(): name.Bang;
        /** Reads a filename back into the value, or an error. */
        static from_file(text: string): name.Bang;
        /** Reads a JSON object into its canonical value, or an error naming the broken key. */
        static from_json(text: string): name.Bang;
        /** Reads a path and query string back into the value, or an error. */
        static from_url(text: string): name.Bang;
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
    /** The lattice the cells sit on. */
    export type Lattice = "square" | "hex";
    export const Lattice: {
        /** Returns the default Lattice. */
        default(): name.Lattice;
        /** Returns whether this is the square lattice. */
        is_square(lattice: name.Lattice): boolean;
        /** Returns the number of unit directions a twist may pick from. */
        units(lattice: name.Lattice): number;
    };
    export interface SequenceData {
        /** The kind word. */
        kind: string;
        /** The number of axes. */
        dim: number;
        /** The digits per axis, 2 unless said. */
        base: number;
        /** The design as a number. */
        code: bigint;
        /** The reading taken. */
        measure: string;
        /** The index the reading runs along. */
        axis: string;
    }
    /** A design sequence's address: the design, the reading taken off it and the index it runs along. */
    export class Sequence {
        /** Pins a design's reading to its measure and axis. */
        constructor(code: string | number | bigint, dim: number, base: number, measure: string, axis: string);
        free(): void;
        /** Reads the Sequence from its plain data. */
        static from(data: SequenceData): Sequence;
        /** Writes the Sequence as plain data. */
        toJSON(): SequenceData;
        /** The number of axes. */
        get dim(): number;
        set dim(value: number);
        /** The digits per axis, 2 unless said. */
        get base(): number;
        set base(value: number);
        /** The design as a number. */
        get code(): string;
        set code(value: string | number | bigint);
        /** The reading taken. */
        get measure(): string;
        set measure(value: string);
        /** The index the reading runs along. */
        get axis(): string;
        set axis(value: string);
        /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
        checked(): name.Sequence;
        /** Returns the design pinned to its dimension and base. */
        design(): name.Bang;
        /** Reads a filename back into the value, or an error. */
        static from_file(text: string): name.Sequence;
        /** Reads a JSON object into its canonical value, or an error naming the broken key. */
        static from_json(text: string): name.Sequence;
        /** Reads a path and query string back into the value, or an error. */
        static from_url(text: string): name.Sequence;
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
    export interface WordData {
        /** The kind word. */
        kind: string;
        /** The number of axes every letter shares. */
        dim: number;
        /** The codes of the letters in order. */
        magic: bigint[];
        /** The side each letter renders at. */
        side: number[];
        /** The base of each letter, absent when every letter is base 2. */
        base?: number[];
    }
    /** A magic word: an ordered list of design letters, first letter outermost, each at its own side. */
    export class Word {
        /** Pins an ordered letter list at base 2. */
        constructor(dim: number, letters: ([string | number | bigint, number])[]);
        free(): void;
        /** Reads the Word from its plain data. */
        static from(data: WordData): Word;
        /** Writes the Word as plain data. */
        toJSON(): WordData;
        /** The number of axes every letter shares. */
        get dim(): number;
        set dim(value: number);
        /** The codes of the letters in order. */
        get magic(): string[];
        set magic(value: (string | number | bigint)[]);
        /** The side each letter renders at. */
        get side(): Uint32Array;
        set side(value: ArrayLike<number>);
        /** The base of each letter, absent when every letter is base 2. */
        get base(): Uint32Array | undefined;
        set base(value: ArrayLike<number> | undefined);
        /** Returns the base of every letter, 2 where the name says nothing. */
        bases(): Uint32Array;
        /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
        checked(): name.Word;
        /** Reads a filename back into the value, or an error. */
        static from_file(text: string): name.Word;
        /** Reads a JSON object into its canonical value, or an error naming the broken key. */
        static from_json(text: string): name.Word;
        /** Reads a path and query string back into the value, or an error. */
        static from_url(text: string): name.Word;
        /** Returns every letter as a design pinned to the word's dimension and its own base. */
        letters(): name.Bang[];
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
}
export declare namespace press {
    /** Returns the number of designs of the dimension and base that contain the number. */
    export function containing(number: string | number | bigint, dimension: number, base: number): string;
    /** Splits a number into its dimension coordinates, one base digit peeled per axis in parallel. */
    export function coordinates(number: string | number | bigint, dimension: number, base: number): string[];
    /** Counts the members of a design below the limit. */
    export function count_below(code: string | number | bigint, dimension: number, base: number, limit: string | number | bigint): string;
    /** Returns the count of distinct digit vectors the number uses. */
    export function distinct(number: string | number | bigint, dimension: number, base: number): number;
    /** Weaves dimension coordinates back into their single interleaved number. */
    export function interleave(coords: (string | number | bigint)[], base: number): string;
    /** Returns the allowed digit table of one magic layer, one flag per cell of its tile. */
    export function layer_table(layer: bang.MagicLayer): boolean[];
    /** Returns whether every digit vector of the number lies in the design. */
    export function member(code: string | number | bigint, number: string | number | bigint, dimension: number, base: number): boolean;
    /** Returns the first members of a design in ascending order. */
    export function members(code: string | number | bigint, dimension: number, base: number, count: number): string[];
    /** Returns the diagonal slice profile of one design pressed to a fractal level. */
    export function profile(code: string | number | bigint, dimension: number, base: number, level: number): string[];
    /** Returns the corner-usage mask of a number, one bit per digit vector its expansion uses. */
    export function usage(number: string | number | bigint, dimension: number, base: number): string;
    /** Counts the members of a magic word from its layer fills, without enumeration. */
    export function word_count(layers: bang.MagicLayer[]): string;
    /** Returns whether the number lies in the magic word's composed design. */
    export function word_member(layers: bang.MagicLayer[], number: string | number | bigint): boolean;
    /** Enumerates every member of the magic word in ascending order. */
    export function word_members(layers: bang.MagicLayer[]): string[];
    /** Returns the diagonal slice profile of a magic word by the substitution product. */
    export function word_profile(layers: bang.MagicLayer[]): string[];
    /** The largest corner count the tally press accepts, keeping its table a million rows. */
    export function CORNERS(): number;
    export type PressData = Record<string, unknown>;
    /** The tally press: one pass over the integers weighs every design of a universe at once. */
    export class Press {
        /** Builds an empty press over every design of the dimension and base. */
        constructor(dimension: number, base: number);
        free(): void;
        /** Reads the Press from its plain data. */
        static from(data: PressData): Press;
        /** Writes the Press as plain data. */
        toJSON(): PressData;
        /** The design dimension of the universe. */
        get dimension(): number;
        set dimension(value: number);
        /** The numeral base of the universe. */
        get base(): number;
        set base(value: number);
        /** Adds a weighted number to its usage bucket. */
        add(number: string | number | bigint, weight: string | number | bigint): void;
        /** Returns the total weight the design at a code has collected. */
        total(code: string | number | bigint): string;
        /** Returns every design's total in code order by one subset-sum transform. */
        totals(): string[];
    }
}
export declare namespace roulette {
    /** Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders. */
    export function nodes(track: spirograph.Track, pencils: spirograph.Pencil[], samples: number, tol: number): roulette.Nodes;
    /** Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one. */
    export function side(a: ArrayLike<number>, b: ArrayLike<number>, c: ArrayLike<number>): number;
    /** One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve. */
    export function spread(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): spirograph.Pencil[];
    export interface NodesData {
        /** The curves counted, in the order the pencils came in. */
        curves: number;
        /** How often each curve crosses itself, curve by curve. */
        selves: number[];
        /** How often each pair of curves crosses, the lower curve first, in lexicographic order. */
        pairs: number[];
        /** The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet. */
        most: number;
        /** The nodes more than one crossing clusters at. */
        crowded: number;
        /** The distinct points the crossings sit at, one for every cluster. */
        points: number;
        /** The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded. */
        branches: number;
        /** The segment pairs that meet without crossing: collinear or end to end. */
        touches: number;
    }
    /** Every crossing of a traced roulette: the curves against themselves, the curves against one another, and how crowded the worst node is. */
    export class Nodes {
        private constructor();
        free(): void;
        /** Reads the Nodes from its plain data. */
        static from(data: NodesData): Nodes;
        /** Writes the Nodes as plain data. */
        toJSON(): NodesData;
        /** The curves counted, in the order the pencils came in. */
        get curves(): number;
        set curves(value: number);
        /** How often each curve crosses itself, curve by curve. */
        get selves(): Uint32Array;
        set selves(value: ArrayLike<number>);
        /** How often each pair of curves crosses, the lower curve first, in lexicographic order. */
        get pairs(): Uint32Array;
        set pairs(value: ArrayLike<number>);
        /** The most crossings one node carries: one at a plain double point, and `n(n - 1)/2` where `n` branches meet. */
        get most(): number;
        set most(value: number);
        /** The nodes more than one crossing clusters at. */
        get crowded(): number;
        set crowded(value: number);
        /** The distinct points the crossings sit at, one for every cluster. */
        get points(): number;
        set points(value: number);
        /** The branches through every node added up, which is the edge count of the picture as a plane graph, `n` at a node where `n` branches meet and `2` times `points` when no node is crowded. */
        get branches(): number;
        set branches(value: number);
        /** The segment pairs that meet without crossing: collinear or end to end. */
        get touches(): number;
        set touches(value: number);
        /** Returns the default Nodes. */
        static default(): roulette.Nodes;
        /** How often the curves `i` and `j` cross, either order, and zero when they are one curve. */
        pair(i: number, j: number): number;
        /** Every crossing of two curves. */
        paired(): number;
        /** Every self crossing. */
        selved(): number;
        /** Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero. */
        total(): number;
    }
}
export declare namespace rules {
    /** Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list. */
    export function render(filled: ArrayLike<number>[], number: number, dimension: number, base: number): Tensor;
    /** Returns every axis but the free one. */
    export function tree_axes(dimension: number, free_axis: number): Uint32Array;
    /** The default residue base. */
    export function BASE(): number;
}
export declare namespace shape {
    /** Tallies the design's cells and filled cells per region of the shape. */
    export function census(shape: shape.Shape, types: Tensor): shape.ShapeCensus;
    /** Places one lattice cell relative to the shape, exactly, with no floats. */
    export function classify(shape: shape.Shape, side: number, index: ArrayLike<number>): shape.Region;
    /** Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti. */
    export function crop(types: Tensor, shape: shape.Shape, keep_cut: boolean): Tensor;
    /** Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order. */
    export function crossing_shell(radius: number | bigint, number: number | bigint, level: number): [bigint, bigint][];
    /** Builds the whole crossing tree of one radius, pruned by the seats the design keeps. */
    export function crossing_tree(radius: number | bigint, number: number | bigint, keep: boolean[]): shape.Shell;
    /** Builds a named shape of the dimension, centered at one half on every axis. */
    export function named(name: string, dimension: number, radius: shape.Frac): shape.Shape;
    /** Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic. */
    export function radial_census(types: Tensor, centre: ArrayLike<number | bigint>, r_max: number | bigint): shape.RadialCounts[];
    /** Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes. */
    export function refine(types: Tensor, shape: shape.Shape, base: number, extra: number, keep_cut: boolean): Tensor;
    /** Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side. */
    export function regions(shape: shape.Shape, dims: ArrayLike<number>): Tensor;
    /** Lists the named shapes of a dimension. */
    export function shapes(dimension: number): string[];
    /** The refine output ceiling in cells. */
    export function REFINE_LIMIT(): number;
    export interface FracData {
        /** The numerator, carrying the sign. */
        num: number;
        /** The denominator, always positive. */
        den: number;
    }
    /** An exact rational number with a positive, reduced denominator. */
    export class Frac {
        /** Builds the reduced fraction num over den. */
        constructor(num: number | bigint, den: number | bigint);
        free(): void;
        /** Reads the Frac from its plain data. */
        static from(data: FracData): Frac;
        /** Writes the Frac as plain data. */
        toJSON(): FracData;
        /** The numerator, carrying the sign. */
        get num(): bigint;
        set num(value: number | bigint);
        /** The denominator, always positive. */
        get den(): bigint;
        set den(value: number | bigint);
        /** Returns the exact difference. */
        minus(other: shape.Frac): shape.Frac;
        /** Returns the exact sum. */
        plus(other: shape.Frac): shape.Frac;
        /** Returns the exact product. */
        times(other: shape.Frac): shape.Frac;
        /** Wraps an integer as a fraction over one. */
        static whole(num: number | bigint): shape.Frac;
    }
    /** A closed half-space: the points x with normal dot x at most offset. */
    export interface Half {
        /** The integer outward normal. */
        normal: number[];
        /** The rational offset the linear form stays under. */
        offset: shape.FracData;
    }
    /** The tallies of one design against a single integer radius about a centre. */
    export interface RadialCounts {
        /** The filled cells whose own centre lies within the radius. */
        seen: number;
        /** The filled cells lying wholly within the radius. */
        inside: number;
        /** The filled cells the sphere of the radius crosses. */
        cut: number;
    }
    /** Where one lattice cell sits relative to a shape. */
    export type Region = "Out" | "Cut" | "In";
    export const Region: {
        /** Swaps In and Out, keeping Cut. */
        flip(region: shape.Region): shape.Region;
    };
    /** An exact region of the unit box, scaled onto the lattice by the side. */
    export type Shape = { Ball: { center: shape.FracData[]; radius: shape.FracData } } | { Polytope: { walls: shape.Half[] } } | { Anti: string };
    /** The per-region tallies of a shape over a design, indexed Out, Cut, In. */
    export interface ShapeCensus {
        /** The cell count of each region. */
        cells: number[];
        /** The filled-cell count of each region. */
        filled: number[];
    }
    /** The rooted tree of the boxes one circle crosses, level by level. */
    export interface Shell {
        /** The radius in cells. */
        radius: number;
        /** The side of a box measured in the boxes one level below it. */
        number: number;
        /** The boxes of level `j`, the crossed cells at `0` and the single root last, each level in the arc's own order. */
        levels: shape.ShellBox[][];
        /** The boxes whose parent is missing from the level above, which the crossing identity forbids. */
        orphans: number;
    }
    /** One box of a crossing shell: where it sits, the seat it takes in its parent and whether the design keeps its path. */
    export interface ShellBox {
        /** The box's first coordinate in its own level's grid. */
        x: number;
        /** The box's second coordinate in its own level's grid. */
        y: number;
        /** The seat the box takes in its parent, row-major over the side, and the side squared at the root. */
        seat: number;
        /** The parent's place in the level above, and `usize::MAX` at the root or when the level above holds no such box. */
        parent: number;
        /** Whether every seat from the root down to this box is one the design keeps. */
        live: boolean;
    }
}
export declare namespace six {
    /** Swaps every fill triangle for a void and back. */
    export function anti(cell: Cell6d): Cell6d;
    /** Maps each triangle to one at or above the threshold, zero below. */
    export function binarize(cell: Cell6d, threshold: number): Cell6d;
    /** Binarizes the triangles at the threshold Otsu's method picks. */
    export function binarize_otsu(cell: Cell6d): Cell6d;
    /** Builds a hexagon of the given radius, fill inside and void outside. */
    export function blank(radius: number, orient: six.Orientation, fill: number, void_: number): Cell;
    /** Rounds each triangle to the mean of its masked neighborhood, wrapping on request. */
    export function blur(cell: Cell6d, mask: Tensor, wrap: boolean): Cell6d;
    /** Tallies a cell's triangles, corners and edges, counting the backdrop only on request. */
    export function census(cell: Cell6d, include_grid: boolean): six.Census;
    /** Counts the connected pieces of the fill, triangles joined across shared edges. */
    export function components(cell: Cell6d): number;
    /** Slices a cube through its center across the main diagonal into a hexagon. */
    export function cut(cell: Cell): Cell6d;
    /** Builds the coded 3d design and slices its central hexagon. */
    export function cut_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    /** The three corners of the east-pointing triangle at the grid column and row. */
    export function east(x: number | bigint, y: number | bigint): [bigint, bigint][];
    /** Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request. */
    export function euler(cell: Cell6d, include_grid: boolean): bigint;
    /** Counts the filled triangles of the cell. */
    export function fills(cell: Cell6d): number;
    /** Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh. */
    export function fills_only(cell: Cell6d): six.Census;
    /** Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood. */
    export function framed(cell: Cell6d): Cell6d;
    /** Parses a cell from JSON, defaulting any missing projection metadata. */
    export function from_json(text: string): Cell6d;
    /** Returns the triangle count of the fill's largest connected piece. */
    export function giant(cell: Cell6d): number;
    /** Returns the largest connected piece of the filled-triangle network as a network of its own. */
    export function giant_network(cell: Cell6d): graph.Network;
    /** Returns the grid height in triangles. */
    export function height(cell: Cell6d): number;
    /** Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh. */
    export function holes(cell: Cell6d): number;
    /** Returns whether the cell's three sides are equal. */
    export function is_cube(cell: Cell): boolean;
    /** Returns whether the cell's width, height and parity frame a hexagon. */
    export function is_hex(cell: Cell): boolean;
    /** Projects a cube into the isometric hexagon of top, left and right faces. */
    export function iso(cell: Cell): Cell6d;
    /** Builds the coded 3d design and projects it isometrically. */
    export function iso_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    /** Builds a cell from its four parts. */
    export function new_(cell: Cell, projection: six.Projection, orientation: six.Orientation, start: number): Cell6d;
    /** The three corners of the north-pointing triangle at the grid column and row. */
    export function north(x: number | bigint, y: number | bigint): [bigint, bigint][];
    /** Returns the orientation a hexagon's width and height imply. */
    export function orientation(width: number, height: number): six.Orientation;
    /** Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along. */
    export function pad(cell: Cell6d, k: number, value: number): Cell6d;
    /** Colors each triangle by its type through the custom or default mapping in the given or type mode. */
    export function paint(cell: Cell6d, custom?: Record<string, Color[]>, mode?: core.Mode, rng?: Rng): Cell6d;
    /** Writes the value wherever the tiled mask is nonzero. */
    export function perforate(cell: Cell6d, mask: Tensor, value: number): Cell6d;
    /** Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given. */
    export function png(cell: Cell6d, scale: number, outline: Color | undefined, width: number): Uint8Array;
    /** Projects a cube's three facing sides into a hexagon of fills and voids. */
    export function pro(cell: Cell): Cell6d;
    /** Builds the coded 3d design and projects its facing sides. */
    export function pro_design(code: string | number | bigint, number: number, level: number, base: number): Cell6d;
    /** Tessellates a hexagonal cell over the disc mask of the given radius. */
    export function radial(cell: Cell6d, radius: number): Cell;
    /** Crops the interlocking overhang off a disc tiled at the given radius and tile size. */
    export function radial_crop(cell: Cell, radius: number, size: [number, number]): Cell;
    /** Builds the disc mask of cells within hex distance radius of the center. */
    export function radial_mask(radius: number, orient: six.Orientation): Tensor;
    /** Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere. */
    export function raster(cell: Cell6d, size: number): Float32Array;
    /** Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes. */
    export function rect_png(cell: Cell6d, scale: number, start?: number): Uint8Array;
    /** Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string. */
    export function rect_svg(cell: Cell6d, scale: number, start?: number): string;
    /** Counts the void regions the rim never reaches, the second route to the hole count. */
    export function rim_holes(cell: Cell6d): number;
    /** Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure. */
    export function skin(cell: Cell6d): Cell6d;
    /** Builds the network of filled triangles joined by shared edges. */
    export function slice_core_graph(cell: Cell6d): graph.Network;
    /** Builds the network of fill and void triangles joined by shared edges. */
    export function slice_dual_graph(cell: Cell6d): graph.Network;
    /** Builds the corner-and-edge network of the triangles matching the value, or of every fill and void. */
    export function slice_edge_graph(cell: Cell6d, value?: number): graph.Network;
    /** Builds the network of void triangles joined by shared edges, the pore network of the slice. */
    export function slice_tunnel_graph(cell: Cell6d): graph.Network;
    /** The three corners of the south-pointing triangle at the grid column and row. */
    export function south(x: number | bigint, y: number | bigint): [bigint, bigint][];
    /** Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states. */
    export function spectral_exponent(cell: Cell6d, window: number): number;
    /** Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given. */
    export function svg(cell: Cell6d, scale: number, outline: Color | undefined, width: number, start?: number): string;
    /** Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included. */
    export function tessellate(cell: Cell6d, mask: Tensor): Cell;
    /** Tessellates a hexagonal cell over a full width-by-height mask. */
    export function tile(cell: Cell6d, width: number, height: number): Cell;
    /** Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request. */
    export function tile_cell(cell: Cell6d, width: number, height: number, crop: boolean): Cell6d;
    /** Crops one interlocking step off each side of a sheet tiled at the given size. */
    export function tile_crop(cell: Cell, size: [number, number]): Cell;
    /** Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped. */
    export function tile_step(size: [number, number]): [number, number];
    /** Serializes a cell and its projection metadata to JSON. */
    export function to_json(cell: Cell6d): string;
    /** Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override. */
    export function triangles(cell: Cell6d, start?: number): [[bigint, bigint][], Uint8Array][];
    /** The three corners of the west-pointing triangle at the grid column and row. */
    export function west(x: number | bigint, y: number | bigint): [bigint, bigint][];
    /** Returns the grid width in triangles. */
    export function width(cell: Cell6d): number;
    /** The triangle code for a filled site. */
    export function FILL(): number;
    /** The triangle code for the backdrop outside the figure. */
    export function GRID(): number;
    /** The triangle code for a cube's left face in the iso view. */
    export function LEFT(): number;
    /** The triangle code for a cube's right face in the iso view. */
    export function RIGHT(): number;
    /** The triangle code for a cube's top face in the iso view. */
    export function UP(): number;
    /** The triangle code for an empty site. */
    export function VOID(): number;
    /** The tally of a triangle mesh. */
    export interface Census {
        /** The count of tallied triangles. */
        triangles: number;
        /** The count of filled triangles. */
        fills: number;
        /** The count of void triangles. */
        voids: number;
        /** The count of backdrop triangles. */
        grids: number;
        /** The count of distinct corners. */
        vertices: number;
        /** The count of distinct edges. */
        edges: number;
        /** The count of edges touching one triangle. */
        boundary_edges: number;
        /** The count of edges shared by two triangles. */
        interior_edges: number;
        /** The Euler characteristic of the mesh. */
        euler: number;
    }
    /** The two ways a hexagon can point. */
    export type Orientation = "Horizontal" | "Vertical";
    /** The three ways a cube flattens to a hexagon. */
    export type Projection = "Iso" | "Pro" | "Cut";
    export namespace star {
        /** The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`. */
        export function arm_law(number: number): six.star.Share;
        /** The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`. */
        export function chi8(number: number): bigint;
        /** The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`. */
        export function constant(): number;
        /** The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`. */
        export function decay(excesses: ArrayLike<number>, layers: number): six.star.Decay;
        /** The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`. */
        export function width_law(half: number): number;
        /** The three classes of layer count the `1/L^2` term of the decay reads. */
        export type Branch = "Zero" | "Two" | "Odd";
        export const Branch: {
            /** The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`. */
            constant(branch: six.star.Branch): number;
            /** The name of the branch. */
            name(branch: six.star.Branch): string;
            /** The branch of a layer count. */
            of(layers: number): six.star.Branch;
            /** The exact `1/L^2` coefficient at even `L`, absent at odd `L`. */
            residual(branch: six.star.Branch): number | undefined;
        };
        /** The `L`-layer reading of the ghost star's decay in the cell frame. */
        export interface Decay {
            /** The layer count `L`. */
            layers: number;
            /** The mean excess of the star over the background across the first `L` odd layers. */
            excess: number;
            /** That excess times `L`. */
            scaled: number;
            /** The scaled excess plus `(ln L)/4`, which settles on the branch constant. */
            logged: number;
            /** The miss of the settled value against the branch constant. */
            miss: number;
            /** The miss times `L`, the reading odd `L` leaves behind. */
            linear: number;
            /** The miss times `L` squared, the reading even `L` leaves behind. */
            residual: number;
            /** The slope of the scaled excess against `ln L`, read from `L/2` to `L`, absent unless `L` is divisible by four. */
            slope?: number;
        }
        export interface ShareData {
            /** The count of inked cells. */
            inked: number;
            /** The count of cells read. */
            cells: number;
        }
        /** The exact reading of a cut layer: how many cells were inked out of how many were read. */
        export class Share {
            private constructor();
            free(): void;
            /** Reads the Share from its plain data. */
            static from(data: ShareData): Share;
            /** Writes the Share as plain data. */
            toJSON(): ShareData;
            /** The count of inked cells. */
            get inked(): bigint;
            set inked(value: number | bigint);
            /** The count of cells read. */
            get cells(): bigint;
            set cells(value: number | bigint);
            /** The share in lowest terms, numerator then denominator. */
            reduced(): [bigint, bigint];
            /** The share as a real number. */
            value(): number;
        }
        export type StarData = Record<string, unknown>;
        /** The ghost star of a coded cube's hexagonal cut stack, read in the cell frame. */
        export class Star {
            /** Reads the star of a base-2 space code, the carpet being `23`. */
            constructor(code: string | number | bigint);
            free(): void;
            /** Reads the Star from its plain data. */
            static from(data: StarData): Star;
            /** Writes the Star as plain data. */
            toJSON(): StarData;
            /** The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`. */
            arm(number: number, half: number): six.star.Share;
            /** The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`. */
            cell(number: number, x: number | bigint, z: number | bigint): boolean | undefined;
            /** The per-layer excess of the star band over the hexagon across the first `L` odd layers. */
            excesses(layers: number, half: number): Float64Array;
            /** The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against. */
            hexagon(number: number): six.star.Share;
        }
    }
}
export declare namespace spectrum {
    /** Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size. */
    export function clusters(eigenvalues: ArrayLike<number>, tolerance: number): [number, number][];
    /** Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`. */
    export function laplacian(network: graph.Network, normalised: boolean): Float64Array[];
    /** Returns the ascending Laplacian spectrum of a network, combinatorial or normalised. */
    export function laplacian_spectrum(network: graph.Network, normalised: boolean): Float64Array;
    /** Counts the eigenvalues within the tolerance of a value. */
    export function multiplicity(eigenvalues: ArrayLike<number>, value: number, tolerance: number): number;
    /** Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window. */
    export function spectral_exponent(eigenvalues: ArrayLike<number>, window: number): number | undefined;
    /** Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count. */
    export function spectral_fit(eigenvalues: ArrayLike<number>, window: number): [number, number, number] | undefined;
    /** Builds the integrated density of states as points, each an eigenvalue and its rank fraction. */
    export function spectral_points(eigenvalues: ArrayLike<number>): [number, number][];
    /** Returns the eigenvalues of a dense real symmetric matrix in ascending order. */
    export function symmetric_eigenvalues(matrix: ArrayLike<number>[]): Float64Array;
}
export declare namespace spin {
    /** The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside. */
    export function arcs(data: ArrayLike<number>, size: number, radius: number): [number, number, number][];
    /** The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs. */
    export function harmonics(data: ArrayLike<number>, size: number, rings: number, orders: number): Float64Array;
    /** The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from. */
    export function mass(profile: ArrayLike<number>, size: number): number;
    /** The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from. */
    export function mass_within(profile: ArrayLike<number>, size: number, radius: number): number;
    /** The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple. */
    export function petals(copies: number, order: number): number;
    /** The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle. */
    export function profile(data: ArrayLike<number>, size: number, steps: number): Float32Array;
    /** Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points. */
    export function radial(data: ArrayLike<number>, size: number, out: number, copies: number, step: number, blend: spin.Blend, samples: number): Float32Array;
    /** The radius of the corner circle of a square raster of the side, the last radius a profile reads. */
    export function reach(size: number): number;
    /** The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero. */
    export function ring(data: ArrayLike<number>, size: number, radius: number): number;
    /** The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does. */
    export function turns(power: ArrayLike<number>): number;
    /** The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius. */
    export function wheel(profile: ArrayLike<number>, size: number): Float32Array;
    /** The way radial copies merge: their mean, their sum, their union, their meet, their parity or what the first keeps that no other has. */
    export type Blend = "Mean" | "Sum" | "Union" | "Meet" | "Parity" | "Difference";
    export const Blend: {
        /** Merges one site's copies into the blended value. */
        fold(blend: spin.Blend, values: ArrayLike<number>): number;
        /** Reads a blend by name: mean, sum, union, meet, parity or difference. */
        named(name: string): spin.Blend | undefined;
    };
}
export declare namespace spirograph {
    /** The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel. */
    export function cell(width: number, height: number, reach: number): number;
    /** The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area. */
    export function cover(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean, samples: number, side: number): spirograph.Cover;
    /** The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall. */
    export function disc(track: spirograph.Track, pencils: spirograph.Pencil[]): spirograph.Disc;
    /** How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter. */
    export function distinct(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): number;
    /** The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further. */
    export function frame(track: spirograph.Track, pencils: spirograph.Pencil[]): Float64Array;
    /** The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach. */
    export function nodes(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): bigint | undefined;
    /** Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded. */
    export function pencils(types: ArrayLike<number>, width: number, height: number, mode: string, reach: number, jitter: number, seed: number): spirograph.Pencil[];
    /** Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel. */
    export function point(track: spirograph.Track, pencil: spirograph.Pencil, s: number): [number, number];
    /** The wheel's centre after `s` of path length. */
    export function pose(track: spirograph.Track, s: number): [number, number];
    /** One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter. */
    export function representatives(track: spirograph.Track, pencils: spirograph.Pencil[], exact: boolean): Uint32Array;
    /** Counts the pencils by kind. */
    export function seats(pencils: spirograph.Pencil[]): spirograph.Seats;
    /** The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close. */
    export function signed_area(track: spirograph.Track, pencil: spirograph.Pencil): number | undefined;
    /** Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y. */
    export function trace(track: spirograph.Track, pencils: spirograph.Pencil[], samples: number): Float32Array;
    /** Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps. */
    export function track(kind: string, ring: number, wheel: number, sides: number, laps: number): spirograph.Track;
    /** The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius. */
    export function turn(track: spirograph.Track, s: number): number;
    /** The most laps of a line or a polygon. */
    export function LAPS_CAP(): number;
    /** The most pencils a wheel seats. */
    export function PENCIL_CAP(): number;
    /** The most points one trace returns. */
    export function POINT_CAP(): number;
    /** The largest radius of a ring or a wheel. */
    export function RADIUS_CAP(): number;
    /** The largest raster side a cover rasters. */
    export function RASTER_CAP(): number;
    /** The fewest and the most sides of a polygon track. */
    export function SIDES(): [number, number];
    /** The shape between the walls on a raster, with its numbers. */
    export interface Cover {
        /** The raster row by row, four codes: zero outside the disc, one the flood from the raster's edge, two the flood from the centre, three the shape, the walls and their pockets included. */
        mask: number[];
        /** The raster's side in pixels. */
        side: number;
        /** The shape's share of the disc's pixels, the walls counted in. */
        covered: number;
        /** The centre flood's share of the disc's pixels. */
        hole: number;
        /** The share of the disc's pixels the polylines themselves mark, the boundary error `covered` carries and loses as the raster grows. */
        wall: number;
        /** The mean signed winding number of the disc's pixel centres, read by scanline off the polylines. */
        winding: number;
        /** The closed form `winding` converges to: the distinct curves' signed areas summed and divided by the disc's area. */
        areas: number;
    }
    /** The disc a circle roulette sits in, in the track's units. */
    export interface Disc {
        /** The centre's abscissa in the track's units, the centre of the ring. */
        x: number;
        /** The centre's ordinate in the track's units. */
        y: number;
        /** The radius no curve leaves, in the track's units: `rho + d` with `rho` the centre circle's radius and `d = r abs(p)` the outermost seat, which is `(a - b)/b + abs(p)` wheel radii inside and `(a + b)/b + abs(p)` outside. */
        radius: number;
        /** The radius no curve enters, in the track's units: the least of `abs(rho - d)` over the seats, which is not `rho` less the outermost seat when the seats straddle `rho`, and `rho` itself when no pencil is seated. */
        hole: number;
    }
    /** What a pencil sits on: a filled cell, an empty cell, or a corner of a filled cell. */
    export type Kind = "Fill" | "Void" | "Corner";
    /** A pencil on the wheel: its seat in units of the wheel's radius with the tile centre at the origin, the exact seat it came from, and its kind. */
    export interface Pencil {
        /** The seat's abscissa, in wheel radii. */
        x: number;
        /** The seat's ordinate, up the page, in wheel radii. */
        y: number;
        /** The exact seat, twice the cell coordinates from the tile centre, before any jitter. */
        seat: [number, number];
        /** What the pencil sits on. */
        kind: spirograph.Kind;
    }
    /** One piece of the centre path: a straight run, or a turn about a point. */
    export type Piece = { Run: { from: [number, number]; to: [number, number] } } | { Turn: { about: [number, number]; radius: number; from: number; to: number } };
    /** The mass of a byte grid taken as a wheel: how many pencils of each kind it seats. */
    export interface Seats {
        /** The pencils on filled cells. */
        fills: number;
        /** The pencils on empty cells. */
        voids: number;
        /** The pencils on corners. */
        corners: number;
    }
    export const Seats: {
        /** Returns the default Seats. */
        default(): spirograph.Seats;
    };
    /** A track and the path the wheel's centre takes along it: the wheel of radius `wheel` rolls without slipping, on the left of the track when `side` is minus one and on the right when it is plus one, and turns by `side` times the centre's path length over the wheel's radius. */
    export interface Track {
        /** The kind: `line`, `in`, `out`, `polyin` or `polyout`. */
        kind: string;
        /** The wheel's radius. */
        wheel: number;
        /** The pieces of the centre path, in order. */
        pieces: spirograph.Piece[];
        /** The length of the centre path. */
        total: number;
        /** Minus one inside the track, plus one outside it. */
        side: number;
        /** The track itself as a polyline, closed when `closed` says so. */
        outline: [number, number][];
        /** Whether the outline closes on itself. */
        closed: boolean;
        /** The ring's radius over the wheel's in lowest terms on a circle, zero over zero elsewhere. */
        ratio: [number, number];
        /** How many times the centre goes round: the ratio's denominator on a circle, the laps on a polygon, one on a line. */
        orbits: number;
        /** The rotation order of the whole picture: the ratio's numerator on a circle, none elsewhere. */
        fold: number;
    }
}
export declare namespace three {
    /** Builds the Menger sponge, filled where at most one coordinate is odd, at the given level. */
    export function carpet(number: number, level: number): Cell;
    /** Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading. */
    export function census(cell: Cell): three.Census;
    /** Extracts the network of filled sites joined to their axis neighbors. */
    export function core_graph(cell: Cell): graph.Network;
    /** Builds the cube the universe code names, deepened to the given fractal level. */
    export function create(code: string | number | bigint, number: number, level: number, base: number): Cell;
    /** Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples. */
    export function diagonal_slice(code: string | number | bigint, number: number, level: number, base: number, height: number): Uint32Array[];
    /** Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner. */
    export function diagonal_svg(code: string | number | bigint, number: number, level: number, base: number, heights: ArrayLike<number>, scale: number): string;
    /** Builds the dust cube, filled where every coordinate is even, at the given level. */
    export function dust(number: number, level: number): Cell;
    /** Extracts the network of corners and edges outlining every filled site. */
    export function edge_graph(cell: Cell): graph.Network;
    /** Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites. */
    export function euler(cell: Cell): bigint;
    /** Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it. */
    export function extrude(cell: Cell, axis: number, depth: number): Cell;
    /** Repeats every plane of a cube depth times along its leading axis, colors and tags with it. */
    export function extrude_cube(cell: Cell, depth: number): Cell;
    /** Returns the count of unit faces the filled sites touch, a face shared by two sites counted once. */
    export function faces(cell: Cell): number;
    /** Returns the count of filled sites. */
    export function fills(cell: Cell): number;
    /** Builds a cube from its corner patterns, deepened to the given fractal level. */
    export function from_corners(corners: ArrayLike<number>[], number: number, level: number, base: number): Cell;
    /** Parses a cell from its JSON, colors and tags included. */
    export function from_json(text: string): Cell;
    /** Builds a cube from one string of digits per row, grouped plane by plane. */
    export function from_strings(data: string[][]): Cell;
    /** Returns the count of faces buried between two filled sites, six per site less the exposed surface. */
    export function hidden(cell: Cell): string;
    /** Builds the cube filled wherever the residue sum lands in the levels, at the given level. */
    export function level_set(number: number, levels: ArrayLike<number>, level: number, base: number): Cell;
    /** Folds two or more cells into one by chained Kronecker combination. */
    export function magic(cells: Cell[]): Cell;
    /** Tags every site with its Manhattan distance from the cube's center, the diamond shells. */
    export function manhattan_layers(cell: Cell): Cell;
    /** Merges the cells into one cube arranged width by height by depth. */
    export function merge(cells: Cell[], width: number, height: number, depth: number): Cell;
    /** Builds a cell by placing at each mask site the cell its value indexes. */
    export function mosaic(mask: Tensor, cells: Cell[]): Cell;
    /** Builds the cube the name picks, deepened to the given fractal level. */
    export function named(design: gen.recipe.Design, number: number, level: number): Cell;
    /** Builds the net cube, filled where at least two coordinates are odd, at the given level. */
    export function net(number: number, level: number): Cell;
    /** Builds a cube whose every site turns on with probability density, at the given level. */
    export function noise(number: number, level: number, density: number, rng: Rng): Cell;
    /** Builds the solid cube at the given size and level. */
    export function ones(number: number, level: number): Cell;
    /** Returns the 24 rotation triples that reach each distinct cube orientation. */
    export function orientations(): [number, number, number][];
    /** Builds the point cube, filled where every coordinate is odd, at the given level. */
    export function point(number: number, level: number): Cell;
    /** Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`. */
    export function profile(code: string | number | bigint, number: number, level: number, base: number): string[];
    /** Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`. */
    export function project(point: ArrayLike<number>): [number, number];
    /** Returns one outward quad per exposed face, scaled into the unit box. */
    export function quads(cell: Cell): three.Quad[];
    /** Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped. */
    export function shadow(point: ArrayLike<number>): [bigint, bigint];
    /** Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it. */
    export function slice(cell: Cell, axis: number, index: number): Cell;
    /** Orients a copy of the cell by each mask value and merges them in the mask's shape. */
    export function special(mask: Tensor, cell: Cell): Cell;
    /** Builds the star cube, filled where exactly one coordinate is odd, at the given level. */
    export function star(number: number, level: number): Cell;
    /** Returns the first and last height a profile fills, or none when the design is empty. */
    export function support(counts: (string | number | bigint)[]): [number, number] | undefined;
    /** Returns the count of filled faces exposed to void or the outside. */
    export function surface(cell: Cell): string;
    /** Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped. */
    export function text(cell: Cell, glyphs?: Record<string, string>): string[];
    /** Serializes the cell's shape and types to JSON, with colors and tags when present. */
    export function to_json(cell: Cell): string;
    /** Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner. */
    export function to_obj(cell: Cell): string;
    /** Unrolls the cube into one string of digits per row, grouped plane by plane. */
    export function to_strings(cell: Cell): string[][];
    /** Extracts the network of empty sites joined to their axis neighbors. */
    export function tunnel_graph(cell: Cell): graph.Network;
    /** Builds the checkerboard cube, filled where all coordinate parities agree, at the given level. */
    export function void_(number: number, level: number): Cell;
    /** Returns the count of empty sites. */
    export function voids(cell: Cell): number;
    /** Returns the filled-site count, the cube's volume. */
    export function volume(cell: Cell): number;
    /** Returns the cell's edge-graph segments, scaled into the unit box. */
    export function wires(cell: Cell): three.Vec3[][];
    /** Builds the cube of rods along the x axis at the given size and level. */
    export function xline(number: number, level: number): Cell;
    /** Builds the cube of beams along the x axis at the given size and level. */
    export function xtree(number: number, level: number): Cell;
    /** Builds the cube of rods along the y axis at the given size and level. */
    export function yline(number: number, level: number): Cell;
    /** Builds the cube of beams along the y axis at the given size and level. */
    export function ytree(number: number, level: number): Cell;
    /** Builds the all-void cube at the given size and level. */
    export function zeros(number: number, level: number): Cell;
    /** Builds the cube of rods along the z axis at the given size and level. */
    export function zline(number: number, level: number): Cell;
    /** Builds the cube of beams along the z axis at the given size and level. */
    export function ztree(number: number, level: number): Cell;
    /** The tally of a cube's sites. */
    export interface Census {
        /** The count of filled sites. */
        fills: number;
        /** The count of empty sites. */
        voids: number;
        /** The count of exposed faces. */
        surface: bigint;
        /** The count of corners the filled sites touch. */
        vertices: number;
        /** The count of unit edges the filled sites touch. */
        edges: number;
        /** The count of unit faces the filled sites touch, shared ones counted once. */
        faces: number;
        /** The Euler characteristic of the filled complex. */
        euler: number;
    }
    /** An outward face of a filled site: its normal and four corners. */
    export interface Quad {
        /** The outward unit normal. */
        normal: three.Vec3Data;
        /** The four corners in winding order. */
        verts: three.Vec3Data[];
    }
    export interface Vec3Data {
        /** The x component. */
        x: number;
        /** The y component. */
        y: number;
        /** The z component. */
        z: number;
    }
    /** A three-component vector of f32. */
    export class Vec3 {
        /** Builds a vector from its components. */
        constructor(x: number, y: number, z: number);
        free(): void;
        /** Reads the Vec3 from its plain data. */
        static from(data: Vec3Data): Vec3;
        /** Writes the Vec3 as plain data. */
        toJSON(): Vec3Data;
        /** The x component. */
        get x(): number;
        set x(value: number);
        /** The y component. */
        get y(): number;
        set y(value: number);
        /** The z component. */
        get z(): number;
        set z(value: number);
        /** Returns the cross product, perpendicular to both vectors. */
        cross(o: three.Vec3): three.Vec3;
        /** Returns the dot product of the two vectors. */
        dot(o: three.Vec3): number;
        /** Multiplies every component by the scalar. */
        scale(s: number): three.Vec3;
    }
}
export declare namespace tourbillon {
    /** The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle. */
    export function eyes(qmax: number): tourbillon.Eye[];
    /** Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer. */
    export function field(top: number, size: number, schedule: string, increment: number, set: string, weights: string, mode: string, blend: string, seed: number): Float32Array;
    /** The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle. */
    export function layers(top: number, schedule: string, increment: number, set: string, weights: string, seed: number): tourbillon.Layer[];
    /** The least whole number of increments that closes a quarter turn, none once the count passes the cap. */
    export function period(increment: number): number | undefined;
    /** The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one. */
    export function sharing(list: tourbillon.Layer[]): [number, number];
    /** Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site. */
    export function stack(list: tourbillon.Layer[], size: number, mode: string, blend: spin.Blend): Float32Array;
    /** Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites. */
    export function stats(field: ArrayLike<number>, size: number, top: number, schedule: string, increment: number, set: string, weights: string, blend: string, seed: number): tourbillon.Stats;
    /** One angle of the quarter-turn lattice: the turn in degrees and the ninety a over q that names it. */
    export interface Eye {
        /** The turn in degrees. */
        angle: number;
        /** The numerator, ninety times a. */
        numer: number;
        /** The denominator q. */
        denom: number;
    }
    /** One carpet of a stack: the odd scale it is drawn at, the weight the linear blends carry it at and the turn it takes about the centre, in degrees. */
    export interface Layer {
        /** The odd scale, the number of cells across the carpet. */
        scale: number;
        /** The weight, scaled so the magnitudes average one. */
        weight: number;
        /** The turn about the centre, in degrees. */
        degrees: number;
    }
    /** The readings of a spun stack: its layers, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers, the sites inside the disc and the brightest three. */
    export interface Stats {
        /** The count of layers in the stack. */
        layers: number;
        /** The first eight scales. */
        scales: number[];
        /** The first eight angles, in degrees. */
        angles: number[];
        /** The mean over the disc. */
        mean: number;
        /** The RMS contrast over the disc. */
        rms: number;
        /** The RMS contrast times the root of the layer count. */
        faded: number;
        /** The exact value at the centre, computed and never sampled. */
        centre: number;
        /** Whether the blend carries the weights. */
        weighted: boolean;
        /** The smallest value over the disc. */
        low: number;
        /** The largest value over the disc. */
        high: number;
        /** The count of sites inside the disc. */
        inside: number;
        /** The brightest three sites, each as its unit coordinates and its value. */
        peaks: number[][];
        /** The least whole number of increments that closes a quarter turn, none past the cap. */
        period?: number;
        /** The count of distinct angle classes the layers fall in, read a quarter turn apart. */
        classes: number;
        /** The count of layer pairs sharing an angle class. */
        pairs: number;
    }
}
export declare namespace two {
    /** Returns the payload bytes the cell's filled sites can hold, its length header paid for. */
    export function capacity(cell: Cell): number;
    /** Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level. */
    export function carpet(number: number, level: number): Cell;
    /** Takes the cell's full census in one reading. */
    export function census(cell: Cell): two.Census;
    /** Builds the design a universe code names, deepened to the level and rotated by quarter-turns. */
    export function create(code: string | number | bigint, number: number, level: number, rotation: number, base: number): Cell;
    /** Builds the dust fractal, its seed on at every even-even site, deepened to the level. */
    export function dust(number: number, level: number): Cell;
    /** Writes the payload over the cell's filled sites, repeating it until every site is spoken for. */
    export function embed(cell: Cell, payload: ArrayLike<number>): Cell;
    /** Returns the Euler characteristic of the filled sites, vertices less edges plus faces. */
    export function euler(cell: Cell): bigint;
    /** Reads the payload back, the plain cell naming the sites the carried one wrote over. */
    export function extract(carrier: Cell, carried: Cell): Uint8Array;
    /** Counts the filled sites of the cell. */
    export function fills(cell: Cell): number;
    /** Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns. */
    export function from_corners(corners: ArrayLike<number>[], number: number, level: number, rotation: number, base: number): Cell;
    /** Restores a cell from its JSON string, colors and tags included. */
    export function from_json(text: string): Cell;
    /** Builds a cell from rows of digits, the inverse of the text rendering. */
    export function from_strings(rows: string[]): Cell;
    /** Builds the hline fractal, its seed striped along odd rows, deepened to the level. */
    export function hline(number: number, level: number): Cell;
    /** Builds the htree fractal, its seed striped along even rows, deepened to the level. */
    export function htree(number: number, level: number): Cell;
    /** Builds the level-set design, filling every residue corner whose digits sum to a named level. */
    export function level_set(number: number, levels: ArrayLike<number>, level: number, rotation: number, base: number): Cell;
    /** Tiles the mask over the shape and crops it, the perforation pattern itself. */
    export function mask(mask: Tensor, shape: ArrayLike<number>): Tensor;
    /** Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept. */
    export function merge(cells: Cell[], width: number, height: number): Cell;
    /** Builds the design the name picks, deepened to the level and rotated by quarter-turns. */
    export function named(design: gen.recipe.Design, number: number, level: number, rotation: number): Cell;
    /** Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level. */
    export function net(number: number, level: number): Cell;
    /** Builds a random cell, each seed site drawn on with probability density, deepened to the level. */
    export function noise(number: number, level: number, density: number, rng: Rng): Cell;
    /** Builds an all-filled cell of the given size and level. */
    export function ones(number: number, level: number): Cell;
    /** Counts the faces of filled sites open to emptiness or the border. */
    export function perimeter(cell: Cell): string;
    /** Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given. */
    export function png(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: two.Shape): Uint8Array;
    /** Builds the point fractal, its seed on at every odd-odd site, deepened to the level. */
    export function point(number: number, level: number): Cell;
    /** Reads the payload back from a framed sheet, the plain fourth cell naming the sites. */
    export function read(sheet: Cell, carrier: Cell): Uint8Array;
    /** Builds the framed sheet of four same-sized cells, the fourth carrying the payload. */
    export function sheet(cells: Cell[], payload: ArrayLike<number>): Cell;
    /** Tiles quarter-turned copies of the cell as the 2d mask directs. */
    export function special(mask: Tensor, cell: Cell): Cell;
    /** Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level. */
    export function star(number: number, level: number): Cell;
    /** Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given. */
    export function svg(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: two.Shape): string;
    /** Renders the cell as rows of glyphs, or of digits where no glyph is mapped. */
    export function text(cell: Cell, glyphs?: Record<string, string>): string[];
    /** Lifts the flat cell into a cube one site deep, colors and tags with it. */
    export function to_3d(cell: Cell): Cell;
    /** Serializes the cell to a JSON string of its types, with colors and tags when present. */
    export function to_json(cell: Cell): string;
    /** Builds the vline fractal, its seed striped along odd columns, deepened to the level. */
    export function vline(number: number, level: number): Cell;
    /** Builds the void fractal, its seed a checkerboard on even parity, deepened to the level. */
    export function void_(number: number, level: number): Cell;
    /** Counts the empty sites of the cell. */
    export function voids(cell: Cell): number;
    /** Builds the vtree fractal, its seed striped along even columns, deepened to the level. */
    export function vtree(number: number, level: number): Cell;
    /** Builds an all-empty cell of the given size and level. */
    export function zeros(number: number, level: number): Cell;
    /** One reading of a cell: its sites, its outline and its topology. */
    export interface Census {
        /** The count of filled sites. */
        fills: number;
        /** The count of empty sites. */
        voids: number;
        /** The count of filled faces open to emptiness or the border. */
        perimeter: bigint;
        /** The count of distinct corners the filled sites touch. */
        vertices: number;
        /** The count of distinct unit edges the filled sites carry. */
        edges: number;
        /** The Euler characteristic, vertices less edges plus filled sites. */
        euler: number;
    }
    /** The outline a flat cell's sites are drawn with. */
    export type Shape = "Square" | "Circle" | "Diamond";
    export namespace payload {
        /** The five by five mask a carried mosaic lays its four tiles out under. */
        export function frame(): Tensor;
    }
}
