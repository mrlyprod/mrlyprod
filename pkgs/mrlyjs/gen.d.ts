export { default, initSync } from "./pkg/gen/mrlyjs_gen.js";

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
/** Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe */
export function background(seed: number | bigint, width: number, height: number): Uint8Array;
/** Returns the plane's bang code of a classic design, or None for one outside the plane. */
export function classic_code(design: recipe.Design): string | undefined;
/** Returns the bang code of a named design in a dimension, or None where it has no design. */
export function classic_code_nd(design: recipe.Design, dimension: number): string | undefined;
/** Draws a hex key of the given length from the stream. */
export function hex_key(length: number, rng: Rng): string;
/** Draws one named design from the stream. */
export function random_design(rng: Rng): recipe.Design;
/** Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3. */
export function random_rotation(design: recipe.Design, rng: Rng): number;
/** Builds the mask of a mosaic tile: the two trees of the side, two where they cross. */
export function tree_mask(n: number): Tensor;
/** The five construction families a tile can belong to. */
export type Group = "General" | "Fractal" | "Magic" | "Special" | "Mosaic";
export const Group: {
    /** Returns every Group in canonical order. */
    all(): Group[];
};
/** The parity filter over candidate sizes. */
export type Parity = "Evens" | "Odds" | "Both";
export const Parity: {
    /** Returns every Parity in canonical order. */
    all(): Parity[];
    /** Returns true when the number passes the filter. */
    keep(parity: Parity, n: number): boolean;
};
export interface TileData {
    /** The construction family. */
    group: Group;
    /** The base factor of the construction. */
    factor: number;
    /** The origin of each layer. */
    sources: recipe.Source[];
    /** The grid size of each source. */
    numbers: number[];
    /** The fractal level of each source. */
    levels: number[];
    /** The quarter-turn rotation of each source. */
    rotations: number[];
    /** Whether the finished tile inverts. */
    invert: boolean;
    /** Whether the finished tile flips. */
    flip: boolean;
    /** The tile's width in cells. */
    width: number;
    /** The tile's height in cells. */
    height: number;
}
/** A complete recipe for one tile. */
export class Tile {
    /** Builds an empty tile in a group. */
    constructor(group: Group);
    free(): void;
    /** Reads the Tile from its plain data. */
    static from(data: TileData): Tile;
    /** Writes the Tile as plain data. */
    toJSON(): TileData;
    /** The construction family. */
    readonly group: Group;
    /** The base factor of the construction. */
    readonly factor: number;
    /** The origin of each layer. */
    readonly sources: recipe.Source[];
    /** The grid size of each source. */
    readonly numbers: Uint32Array;
    /** The fractal level of each source. */
    readonly levels: Uint32Array;
    /** The quarter-turn rotation of each source. */
    readonly rotations: Uint32Array;
    /** Whether the finished tile inverts. */
    readonly invert: boolean;
    /** Whether the finished tile flips. */
    readonly flip: boolean;
    /** The tile's width in cells. */
    readonly width: number;
    /** The tile's height in cells. */
    readonly height: number;
    /** Checks that the slots, numbers and sizes agree. */
    check(): void;
    /** Returns whether the recipe is a magic tile of one repeated source at one repeated number, */
    degenerate(): boolean;
    /** Returns the larger of width and height. */
    max_size(): number;
    /** Recomputes the factor and side length the group and numbers imply, zero when they overflow. */
    resize(): void;
    /** Sets the tile's width and height. */
    size(width: number, height: number): Tile;
}
export declare namespace build {
    /** Builds the flat cell the tile describes. */
    export function build_2d(tile: Tile): Cell;
    /** Builds the cube the tile describes. */
    export function build_3d(tile: Tile): Cell;
    /** Builds the tile's cube and flattens it through its projection. */
    export function build_6d(hex: build.HexTile): Cell6d;
    /** Draws a random flat tile from the stream, rotations from the four quarter-turns. */
    export function create_2d(config: draw.ConfigNd, rng: Rng): Tile;
    /** Draws a cube tile from the config with cube orientations drawn from the stream. */
    export function create_3d(config: draw.ConfigNd, rng: Rng): Tile;
    /** Draws a cube tile from the config under a projection drawn from the stream. */
    export function create_6d(config: draw.ConfigNd, rng: Rng): build.HexTile;
    /** Draws a random flat tile up to the given size under the default config. */
    export function random_tile_2d(max_size: number, rng: Rng): Tile;
    /** Draws a random cube tile up to the given size. */
    export function random_tile_3d(max_size: number, rng: Rng): Tile;
    /** Draws a random cube tile up to the given size under a random projection. */
    export function random_tile_6d(max_size: number, rng: Rng): build.HexTile;
    /** A cube tile paired with the projection that flattens it. */
    export interface HexTile {
        /** The projection that flattens the tile. */
        projection: math.six.Projection;
        /** The cube tile underneath. */
        tile: TileData;
    }
}
export declare namespace core {
    export namespace paint {
        /** The constraints a caller may put on a random paint. */
        export interface Config {
            /** The editions allowed, or None for all seven. */
            editions?: core.paint.Edition[];
            /** The primary inks allowed, or None for black and white. */
            primaries?: core.paint.Ink[];
            /** The forced target, or None for a coin flip. */
            target?: core.paint.Target;
        }
        /** The seven ways a paint distributes its colors over a cell. */
        export type Edition = "Simple" | "Index" | "Layers" | "Neighbors" | "Rows" | "Columns" | "Random";
        /** The fifteen named inks a paint draws from. */
        export type Ink = "Black" | "White" | "Red" | "Orange" | "Yellow" | "Green" | "Mint" | "Teal" | "Cyan" | "Blue" | "Indigo" | "Purple" | "Pink" | "Brown" | "Gray";
        export interface PaintData {
            /** The coloring edition. */
            edition: core.paint.Edition;
            /** The secondary color scheme. */
            scheme: core.paint.Scheme;
            /** The side the primary ink lands on. */
            target: core.paint.Target;
            /** The primary ink. */
            primary: core.paint.Ink;
            /** The secondary inks. */
            secondary: core.paint.Ink[];
            /** The shade indices of a multitone ramp. */
            shades: number[];
        }
        /** A complete coloring recipe for one cell. */
        export class Paint {
            private constructor();
            free(): void;
            /** Reads the Paint from its plain data. */
            static from(data: PaintData): Paint;
            /** Writes the Paint as plain data. */
            toJSON(): PaintData;
            /** The coloring edition. */
            readonly edition: core.paint.Edition;
            /** The secondary color scheme. */
            readonly scheme: core.paint.Scheme;
            /** The side the primary ink lands on. */
            readonly target: core.paint.Target;
            /** The primary ink. */
            readonly primary: core.paint.Ink;
            /** The secondary inks. */
            readonly secondary: core.paint.Ink[];
            /** The shade indices of a multitone ramp. */
            readonly shades: Uint32Array;
        }
        /** The two ways secondary colors are drawn. */
        export type Scheme = "Multicolor" | "Multitone";
        /** The side of the figure the primary ink lands on. */
        export type Target = "Fill" | "Void";
    }
}
export declare namespace draw {
    /** The constraints a random tile is drawn under. */
    export interface ConfigNd {
        /** The tile groups allowed. */
        groups: Group[];
        /** The catalog the sources are drawn from. */
        catalog: recipe.Catalog;
        /** The smallest allowed side. */
        min_size: number;
        /** The largest allowed side. */
        max_size: number;
        /** The parity the sizes must keep. */
        parity: Parity;
        /** The forced inversion flag, or None to flip a coin. */
        invert?: boolean;
    }
}
export declare namespace math {
    export namespace six {
        /** The three ways a cube flattens to a hexagon. */
        export type Projection = "Iso" | "Pro" | "Cut";
    }
}
export declare namespace name {
    /** One value for every slot of a tile, or one value per slot. */
    export type Slots = number | number[];
    export interface TileData {
        /** The kind word. */
        kind: string;
        /** The one design of a flat or fractal tile. */
        code?: bigint;
        /** The mask code of a special tile. */
        special?: bigint;
        /** The letters of a magic tile, first letter outermost. */
        magic: bigint[];
        /** The three codes of a mosaic tile. */
        mosaic: bigint[];
        /** The side of the mask of a special or mosaic tile. */
        factor?: number;
        /** The side each slot renders at, one per letter for a magic tile. */
        side: name.Slots;
        /** The power a fractal tile is raised to, absent at one. */
        level?: number;
        /** The quarter turns of each slot, absent when nothing turns. */
        turn: name.Slots;
        /** Whether a special tile flips its mask. */
        flip: boolean;
        /** Whether the finished tile inverts. */
        invert: boolean;
    }
    /** A tile recipe folded to its one canonical object. */
    export class Tile {
        private constructor();
        free(): void;
        /** Reads the Tile from its plain data. */
        static from(data: TileData): Tile;
        /** Writes the Tile as plain data. */
        toJSON(): TileData;
        /** The one design of a flat or fractal tile. */
        readonly code: string | undefined;
        /** The mask code of a special tile. */
        readonly special: string | undefined;
        /** The letters of a magic tile, first letter outermost. */
        readonly magic: string[];
        /** The three codes of a mosaic tile. */
        readonly mosaic: string[];
        /** The side of the mask of a special or mosaic tile. */
        readonly factor: number | undefined;
        /** The side each slot renders at, one per letter for a magic tile. */
        readonly side: name.Slots;
        /** The power a fractal tile is raised to, absent at one. */
        readonly level: number | undefined;
        /** The quarter turns of each slot, absent when nothing turns. */
        readonly turn: name.Slots;
        /** Whether a special tile flips its mask. */
        readonly flip: boolean;
        /** Whether the finished tile inverts. */
        readonly invert: boolean;
        /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
        checked(): name.Tile;
        /** Reads a filename back into the value, or an error. */
        static from_file(text: string): name.Tile;
        /** Reads a JSON object into its canonical value, or an error naming the broken key. */
        static from_json(text: string): name.Tile;
        /** Reads a path and query string back into the value, or an error. */
        static from_url(text: string): name.Tile;
        /** Folds a recipe to its name. */
        static of(recipe: Tile): name.Tile;
        /** Builds the recipe the name folds, resized and checked. */
        recipe(): Tile;
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
export declare namespace recipe {
    /** Returns the classic designs for a dimension. */
    export function classics(dimension: number): recipe.Design[];
    /** Returns every flat size in the range that passes the parity filter. */
    export function generals(min_size: number, max_size: number, parity: Parity): Uint32Array;
    /** Returns every factor list of depth two and beyond whose product lands in the size range. */
    export function nestings(min_size: number, max_size: number, parity: Parity): Uint32Array[];
    /** Returns every factor and level whose power lands in the size range. */
    export function powers(min_size: number, max_size: number, parity: Parity): [number, number][];
    /** Returns every count-long factor list whose product lands in the size range. */
    export function products(min_size: number, max_size: number, count: number, parity: Parity): Uint32Array[];
    /** Returns the side a factor raised to a level makes, or None when no usize holds it. */
    export function size(number: number | bigint, level: number | bigint): number | undefined;
    /** The five classic designs of the plane. */
    export function CLASSICS_2D(): recipe.Design[];
    /** The six classic designs of the cube. */
    export function CLASSICS_3D(): recipe.Design[];
    /** The deepest fractal level a tile may take. */
    export function MAX_LEVEL(): number;
    /** The largest side, number or factor a tile may take. */
    export function MAX_SIDE(): number;
    /** The most slots a magic tile may take. */
    export function MAX_SLOTS(): number;
    /** The smallest side, number or factor a tile may take. */
    export function MIN_SIDE(): number;
    /** The pool of sources a tile may draw from. */
    export type Catalog = "Classics" | "Universe" | { Codes: bigint[] } | { Designs: recipe.Design[] };
    /** The named designs a source can point at: the four classics and their four antis. */
    export type Design = "Carpet" | "Net" | "Htree" | "Vtree" | "Void" | "Xtree" | "Ytree" | "Ztree" | "Point" | "Dust" | "Hline" | "Vline" | "Star" | "Xline" | "Yline" | "Zline";
    export const Design: {
        /** Returns every Design in canonical order. */
        all(): recipe.Design[];
    };
    /** The origin of one tile layer, a one-field json object. */
    export type Source = { design: recipe.Design } | { code: bigint };
}
export declare namespace variation {
    /** Draws a variation's seed from the stream, then the variation itself on that seed, with a */
    export function create(config: variation.Config, rng: Rng): variation.Variation;
    /** Builds the variation's base cell and draws its paint from the stream, painting the base */
    export function generate(variation: variation.Variation, config: variation.Config, rng: Rng): variation.Variation;
    /** Renders every file of the variation to PNG at the given scale, scattering a Random edition */
    export function render(variation: variation.Variation, scale: number, rng: Rng): variation.Variation;
    /** The settings an artwork is drawn under. */
    export interface Config {
        /** The constraints the tile is drawn under. */
        tile: draw.ConfigNd;
        /** The constraints the paint is drawn under. */
        paint: core.paint.Config;
        /** The width and height repetition pairs to render. */
        files: [number, number][];
    }
    export interface FileData {
        /** The count of tile repetitions across. */
        width: number;
        /** The count of tile repetitions down. */
        height: number;
    }
    /** One rendering of an artwork, sized in tile repetitions. */
    export class File {
        /** Builds a file of the given repetition counts with no PNG bytes. */
        constructor(width: number, height: number);
        free(): void;
        /** Reads the File from its plain data. */
        static from(data: FileData): File;
        /** Writes the File as plain data. */
        toJSON(): FileData;
        /** The count of tile repetitions across. */
        readonly width: number;
        /** The count of tile repetitions down. */
        readonly height: number;
        /** The encoded PNG bytes, empty until rendered and left out of the json. */
        readonly png: Uint8Array;
    }
    export interface VariationData {
        /** The random hex identifier. */
        key: string;
        /** The seed the variation is drawn under. */
        seed: number;
        /** The paint edition. */
        edition: core.paint.Edition;
        /** The primary inks, when the config fixes them. */
        primaries?: core.paint.Ink[];
        /** The tile recipe. */
        tile: TileData;
        /** The mask tile, present only under the Neighbors edition. */
        mask?: TileData;
        /** The paint, set by generate. */
        paint?: core.paint.PaintData;
        /** The renderings, filled by render. */
        files: variation.FileData[];
    }
    /** One seeded artwork, from tile recipe to rendered files. */
    export class Variation {
        private constructor();
        free(): void;
        /** Reads the Variation from its plain data. */
        static from(data: VariationData): Variation;
        /** Writes the Variation as plain data. */
        toJSON(): VariationData;
        /** The random hex identifier. */
        readonly key: string;
        /** The seed the variation is drawn under. */
        readonly seed: bigint;
        /** The paint edition. */
        readonly edition: core.paint.Edition;
        /** The primary inks, when the config fixes them. */
        readonly primaries: core.paint.Ink[] | undefined;
        /** The tile recipe. */
        readonly tile: Tile;
        /** The mask tile, present only under the Neighbors edition. */
        readonly mask: Tile | undefined;
        /** The built base cell, set by generate and left out of the json. */
        readonly base: Cell | undefined;
        /** The renderings, filled by render. */
        readonly files: variation.File[];
        /** Returns whether the edition paints the whole tiled canvas. */
        is_cover(): boolean;
        /** Returns whether the edition paints the base cell before tiling. */
        is_prime(): boolean;
    }
}
