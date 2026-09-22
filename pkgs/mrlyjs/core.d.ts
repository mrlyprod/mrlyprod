export { default, initSync } from "./pkg/core/mrlyjs_core.js";

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
/** Squashes rgba pixels to the hex aspect, returning the new width, height and pixels. */
export function hex_fit(pixels: ArrayLike<number>[], width: number, height: number, vertical: boolean, filter: Filter): [number, number, Uint8Array[]];
/** Returns the size a hex rendering wears, the named axis squashed by the triangle ratio. */
export function hex_size(width: number, height: number, vertical: boolean): [number, number];
/** Resamples rgba pixels to a new size. */
export function resample(pixels: ArrayLike<number>[], width: number, height: number, out_w: number, out_h: number, filter: Filter): Uint8Array[];
/** Decodes a png to its width, height, and rgba colors. */
export function unpng(bytes: ArrayLike<number>): [number, number, Uint8Array[]];
/** The height of an equilateral triangle over its side, the squash a hex rendering wears. */
export function HEX_RATIO(): number;
/** The eight bytes every png file starts with. */
export function PNG_MAGIC(): Uint8Array;
/** A rule that turns counter values into colors. */
export type Colorizer = { Bins: { background: ColorData; ramp: ColorData[] } };
export const Colorizer: {
    /** Builds the blue-to-red diverging ramp around a white middle. */
    diverge(): Colorizer;
    /** Builds the black-through-ember fire ramp: black, dark red, orange, light yellow. */
    fire(): Colorizer;
    /** Builds a binned colorizer from a gradient through the given stops. */
    gradient_bins(background: Color, colors: Color[], shades: number): Colorizer;
    /** Builds the white-to-black heat ramp. */
    heat(): Colorizer;
};
/** The element widths a tensor can hold. */
export type Dtype = "U8" | "U16" | "U32" | "I32";
export const Dtype: {
    /** Returns the largest value the width can hold. */
    max(dtype: Dtype): bigint;
};
/** The way a resampling weighs the source pixels it reads. */
export type Filter = "Nearest" | "Linear" | "Box";
export interface ImageData {
    /** The width in pixels. */
    width: number;
    /** The height in pixels. */
    height: number;
    /** The palette index of every pixel, row by row. */
    rows: number[][];
    /** The colors the rows index. */
    palette: ColorData[];
}
/** A paletted image: rows of palette indices and the palette they point into, hex strings in json. */
export class Image {
    /** Builds an image from its four parts. */
    constructor(width: number, height: number, rows: ArrayLike<number>[], palette: Color[]);
    free(): void;
    /** Reads the Image from its plain data. */
    static from(data: ImageData): Image;
    /** Writes the Image as plain data. */
    toJSON(): ImageData;
    /** The width in pixels. */
    readonly width: number;
    /** The height in pixels. */
    readonly height: number;
    /** The palette index of every pixel, row by row. */
    readonly rows: Uint32Array[];
    /** The colors the rows index. */
    readonly palette: Color[];
    /** Returns the flat rgba pixels, transparent wherever an index misses the palette. */
    colors(): Uint8Array[];
    /** Builds a paletted image from raw rgba pixels, growing the palette as new colors appear. */
    static from_pixels(width: number, height: number, pixels: ArrayLike<number>[]): Image;
    /** Encodes the image as a png at the given scale. */
    png(scale: number): Uint8Array;
    /** Resamples the image to a new size, its palette rebuilt from the blended pixels. */
    resample(width: number, height: number, filter: Filter): Image;
}
/** The ways paint picks a color within a type's palette. */
export type Mode = "Type" | "Tag" | "Index" | "Enumerate" | "Random" | "Row" | "Column" | "Depth";
export declare namespace cell {
    /** Flips every type to one minus itself, same as invert. */
    export function anti(cell: Cell): Cell;
    /** Maps every type to one at or above the threshold and zero below, dropping colors. */
    export function binarize(cell: Cell, threshold: number): Cell;
    /** Binarizes the types at Otsu's threshold, dropping colors. */
    export function binarize_otsu(cell: Cell): Cell;
    /** Replaces every type with the rounded mean of its masked neighborhood, dropping colors. */
    export function blur(cell: Cell, mask: Tensor, wrap: boolean): Cell;
    /** Returns the painted color at a flat index, or transparent while unpainted or past the end. */
    export function color_at(cell: Cell, flat: number): Uint8Array;
    /** Builds the Kronecker product of the two cells' types. */
    export function combine(cell: Cell, other: Cell): Cell;
    /** Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags. */
    export function fractal(cell: Cell, level: number): Cell;
    /** Flips every type to one minus itself. */
    export function invert(cell: Cell): Cell;
    /** Tags every cell with its concentric shell distance from the center. */
    export function layers(cell: Cell, dtype: Dtype): Cell;
    /** Folds at least two cells into one by chained Kronecker products. */
    export function magic(cells: Cell[]): Cell;
    /** Returns the default mapping of the first six types to white, black, alpha, red, green, and blue. */
    export function mapping(): Record<string, Color[]>;
    /** Stitches same-shaped cells into one grid of reps blocks per axis. */
    export function merge(cells: Cell[], reps: ArrayLike<number>): Cell;
    /** Builds the 3-wide Moore mask of the dimension, every site on but the center. */
    export function moore(dimension: number): Tensor;
    /** Lays the cell each mask entry indexes into that entry's place and merges the lot. */
    export function mosaic(mask: Tensor, cells: Cell[]): Cell;
    /** Tags every cell with its count of target-valued neighbors under the mask. */
    export function neighbors(cell: Cell, mask: Tensor, target: number, wrap: boolean, dtype: Dtype): Cell;
    /** Wraps a tensor of types in a bare cell, colorless and tagless. */
    export function new_(types: Tensor): Cell;
    /** Wraps the cell in count layers of value on every side, dropping colors. */
    export function pad(cell: Cell, count: number, value: number): Cell;
    /** Colors every mapped cell, picking within each type's palette by the mode. */
    export function paint(cell: Cell, mapping: Record<string, Color[]>, mode: Mode, rng?: Rng): Cell;
    /** Stamps value wherever the tiled mask is on, dropping colors. */
    export function perforate(cell: Cell, mask: Tensor, value: number): Cell;
    /** Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map. */
    export function remap(cell: Cell, map: ArrayLike<number>, shape: ArrayLike<number>): Cell;
    /** Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else. */
    export function rgba(cell: Cell): Uint8Array;
    /** Builds the flat source index of every destination cell after k quarter turns in the plane of the axes. */
    export function rot90_map(shape: ArrayLike<number>, k: number, axes: [number, number]): Uint32Array;
    /** Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along. */
    export function rotate(cell: Cell, k: number, axes: [number, number]): Cell;
    /** Returns the shape of the type tensor. */
    export function shape(cell: Cell): Uint32Array;
    /** Returns the number of cells. */
    export function size(cell: Cell): number;
    /** Repeats the cell reps times along each axis, carrying colors and tags along. */
    export function tile(cell: Cell, reps: ArrayLike<number>): Cell;
    /** Builds the flat source index of every destination cell after tiling reps copies per axis. */
    export function tile_map(shape: ArrayLike<number>, reps: ArrayLike<number>): Uint32Array;
}
export declare namespace codec {
    /** Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block. */
    export function gif(frames: ArrayLike<number>[], palette: ArrayLike<number>[], width: number, height: number, scale: number, delay: number): Uint8Array;
    /** Encodes rgba colors as a png, drawing each source pixel as a scale by scale block. */
    export function png(colors: ArrayLike<number>[], width: number, height: number, scale: number): Uint8Array;
}
export declare namespace colors {
    /** Returns the color with its alpha set to level. */
    export function alpha(color: Color, level: number): Color;
    /** Returns the ground rgba of the dark or the light theme. */
    export function board(dark: boolean): Uint8Array;
    /** Formats the color as a css rgb or rgba call. */
    export function css(color: Color): string;
    /** Parses a #RRGGBB or #RRGGBBAA code, hash optional. */
    export function from_hex(hex: string): Color;
    /** Builds a gradient of steps colors sweeping evenly through the given stops. */
    export function gradient(colors: Color[], steps: number): Color[];
    /** Returns the foreground rgba of the dark or the light theme. */
    export function ink(dark: boolean): Uint8Array;
    /** Returns the color with every channel flipped and the alpha kept. */
    export function invert(color: Color): Color;
    /** Returns the color scaled toward black below level 50 and toward white above. */
    export function lightness(color: Color, level: number): Color;
    /** Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level. */
    export function luma_types(pixels: ArrayLike<number>[], width: number, height: number, level: number): Tensor;
    /** Blends two colors linearly by ratio. */
    export function mix(color_1: Color, color_2: Color, ratio: number): Color;
    /** Returns the palette color a name spells. */
    export function named(name: string): Color;
    /** Draws a color from the stream, opaque unless alpha is asked for. */
    export function random(alpha: boolean, rng: Rng): Color;
    /** Builds an opaque color. */
    export function rgb(r: number, g: number, b: number): Color;
    /** Builds a color with an explicit alpha. */
    export function rgba(r: number, g: number, b: number, a: number): Color;
    /** Returns a hue one shade lighter, itself, and one shade darker. */
    export function shades(hue: Color): Color[];
    /** Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie. */
    export function snap(pixels: ArrayLike<number>[], palette: Color[]): Uint8Array[];
    /** Formats the color as lowercase hex, appending the alpha pair only when not opaque. */
    export function to_hex(color: Color): string;
    /** The fully transparent color. */
    export function ALPHA(): Color;
    /** The palette black, #000000. */
    export function BLACK(): Color;
    /** The palette blue, #008cff. */
    export function BLUE(): Color;
    /** The palette brown, #b18462. */
    export function BROWN(): Color;
    /** The palette cyan, #1ec9f3. */
    export function CYAN(): Color;
    /** The dark theme. */
    export function DARK(): colors.Theme;
    /** The palette gray, #8e8e93. */
    export function GRAY(): Color;
    /** The palette green, #32cc58. */
    export function GREEN(): Color;
    /** The palette indigo, #6768fa. */
    export function INDIGO(): Color;
    /** The light theme. */
    export function LIGHT(): colors.Theme;
    /** The palette mint, #00d1bb. */
    export function MINT(): Color;
    /** The fifteen names, in palette order. */
    export function NAMES(): string[];
    /** The palette orange, #ff8f2c. */
    export function ORANGE(): Color;
    /** The fifteen colors, in name order. */
    export function PALETTE(): Color[];
    /** The palette pink, #ff325a. */
    export function PINK(): Color;
    /** The palette purple, #d332e9. */
    export function PURPLE(): Color;
    /** The palette red, #ff3d40. */
    export function RED(): Color;
    /** The palette teal, #00cad8. */
    export function TEAL(): Color;
    /** The palette white, #ffffff. */
    export function WHITE(): Color;
    /** The palette yellow, #ffd100. */
    export function YELLOW(): Color;
    export interface ThemeData {
        /** The ground every figure is painted on. */
        ground: ColorData;
        /** The page background, one step off the ground. */
        bg: ColorData;
        /** The raised panel. */
        panel: ColorData;
        /** The sunken well. */
        deep: ColorData;
        /** The hairline between things. */
        line: ColorData;
        /** The foreground, the strongest tone. */
        fg: ColorData;
        /** The dimmed foreground, for anything secondary. */
        dim: ColorData;
        /** The interactive accent. */
        accent: ColorData;
        /** The tone written on the accent. */
        on_accent: ColorData;
        /** The red ink. */
        red: ColorData;
        /** The orange ink. */
        orange: ColorData;
        /** The yellow ink. */
        yellow: ColorData;
        /** The green ink. */
        green: ColorData;
        /** The mint ink. */
        mint: ColorData;
        /** The teal ink. */
        teal: ColorData;
        /** The cyan ink. */
        cyan: ColorData;
        /** The blue ink. */
        blue: ColorData;
        /** The indigo ink. */
        indigo: ColorData;
        /** The purple ink. */
        purple: ColorData;
        /** The pink ink. */
        pink: ColorData;
        /** The brown ink. */
        brown: ColorData;
        /** The gray ink. */
        gray: ColorData;
    }
    /** One theme: the surfaces of a dark or a light ground and the thirteen inks, the same on both. */
    export class Theme {
        private constructor();
        free(): void;
        /** Reads the Theme from its plain data. */
        static from(data: ThemeData): Theme;
        /** Writes the Theme as plain data. */
        toJSON(): ThemeData;
        /** The ground every figure is painted on. */
        readonly ground: Color;
        /** The page background, one step off the ground. */
        readonly bg: Color;
        /** The raised panel. */
        readonly panel: Color;
        /** The sunken well. */
        readonly deep: Color;
        /** The hairline between things. */
        readonly line: Color;
        /** The foreground, the strongest tone. */
        readonly fg: Color;
        /** The dimmed foreground, for anything secondary. */
        readonly dim: Color;
        /** The interactive accent. */
        readonly accent: Color;
        /** The tone written on the accent. */
        readonly on_accent: Color;
        /** The red ink. */
        readonly red: Color;
        /** The orange ink. */
        readonly orange: Color;
        /** The yellow ink. */
        readonly yellow: Color;
        /** The green ink. */
        readonly green: Color;
        /** The mint ink. */
        readonly mint: Color;
        /** The teal ink. */
        readonly teal: Color;
        /** The cyan ink. */
        readonly cyan: Color;
        /** The blue ink. */
        readonly blue: Color;
        /** The indigo ink. */
        readonly indigo: Color;
        /** The purple ink. */
        readonly purple: Color;
        /** The pink ink. */
        readonly pink: Color;
        /** The brown ink. */
        readonly brown: Color;
        /** The gray ink. */
        readonly gray: Color;
        /** The thirteen inks in name order. */
        hues(): Color[];
        /** The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo. */
        inks(): Color[];
    }
}
export declare namespace error {
    /** Parses JSON text into a value. */
    export function parse(text: string): any;
}
export declare namespace image {
    /** Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window. */
    export function blur(pixels: ArrayLike<number>[], width: number, height: number, radius: number): Uint8Array[];
}
export declare namespace paint {
    /** Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream. */
    export function apply(paint: paint.Paint, cell: Cell, rng: Rng): void;
    /** Replays a stored paint onto a cell, tagging first and applying it from the stream. */
    export function coat(cell: Cell, paint: paint.Paint, mask: Tensor | undefined, rng: Rng): void;
    /** Draws a random paint under the config, applies it to the cell, and returns the recipe. */
    export function paint(cell: Cell, config: paint.Config, mask: Tensor | undefined, rng: Rng): paint.Paint;
    /** Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count. */
    export function prime(paint: paint.Paint, cell: Cell, mask: Tensor | undefined, rng: Rng): paint.Paint;
    /** Draws a random edition from the allowed list, or from all seven. */
    export function random_edition(editions: paint.Edition[] | undefined, rng: Rng): paint.Edition;
    /** Redraws the paint's secondary inks and shades under its scheme. */
    export function reroll(paint: paint.Paint, rng: Rng): paint.Paint;
    /** Draws the paint's scheme, target and primary under the config, then rerolls the rest. */
    export function setup(paint: paint.Paint, config: paint.Config, rng: Rng): paint.Paint;
    /** Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side. */
    export function tag(cell: Cell, edition: paint.Edition, target: paint.Target, mask?: Tensor): number;
    /** The constraints a caller may put on a random paint. */
    export interface Config {
        /** The editions allowed, or None for all seven. */
        editions?: paint.Edition[];
        /** The primary inks allowed, or None for black and white. */
        primaries?: paint.Ink[];
        /** The forced target, or None for a coin flip. */
        target?: paint.Target;
    }
    /** The seven ways a paint distributes its colors over a cell. */
    export type Edition = "Simple" | "Index" | "Layers" | "Neighbors" | "Rows" | "Columns" | "Random";
    export const Edition: {
        /** Returns every Edition in canonical order. */
        all(): paint.Edition[];
        /** Returns the cell-painting mode this edition renders with, or None for Random, which scatters. */
        mode(edition: paint.Edition): Mode | undefined;
    };
    /** The fifteen named inks a paint draws from. */
    export type Ink = "Black" | "White" | "Red" | "Orange" | "Yellow" | "Green" | "Mint" | "Teal" | "Cyan" | "Blue" | "Indigo" | "Purple" | "Pink" | "Brown" | "Gray";
    export const Ink: {
        /** Returns every Ink in canonical order. */
        all(): paint.Ink[];
        /** Returns the ink's color. */
        color(ink: paint.Ink): Color;
    };
    export interface PaintData {
        /** The coloring edition. */
        edition: paint.Edition;
        /** The secondary color scheme. */
        scheme: paint.Scheme;
        /** The side the primary ink lands on. */
        target: paint.Target;
        /** The primary ink. */
        primary: paint.Ink;
        /** The secondary inks. */
        secondary: paint.Ink[];
        /** The shade indices of a multitone ramp. */
        shades: number[];
    }
    /** A complete coloring recipe for one cell. */
    export class Paint {
        /** Builds a black-primary, fill-target, multicolor paint for an edition. */
        constructor(edition: paint.Edition);
        free(): void;
        /** Reads the Paint from its plain data. */
        static from(data: PaintData): Paint;
        /** Writes the Paint as plain data. */
        toJSON(): PaintData;
        /** The coloring edition. */
        readonly edition: paint.Edition;
        /** The secondary color scheme. */
        readonly scheme: paint.Scheme;
        /** The side the primary ink lands on. */
        readonly target: paint.Target;
        /** The primary ink. */
        readonly primary: paint.Ink;
        /** The secondary inks. */
        readonly secondary: paint.Ink[];
        /** The shade indices of a multitone ramp. */
        readonly shades: Uint32Array;
        /** Returns true for the Simple edition. */
        is_simple(): boolean;
    }
    /** The two ways secondary colors are drawn. */
    export type Scheme = "Multicolor" | "Multitone";
    export const Scheme: {
        /** Returns every Scheme in canonical order. */
        all(): paint.Scheme[];
    };
    /** The side of the figure the primary ink lands on. */
    export type Target = "Fill" | "Void";
    export const Target: {
        /** Returns every Target in canonical order. */
        all(): paint.Target[];
    };
}
export declare namespace ramp {
    /** Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up. */
    export function color(colorizer: Colorizer, value: number, max: number): Color;
    /** Maps a slice of values to rgba pixels against the range maximum. */
    export function colors(colorizer: Colorizer, values: ArrayLike<number>, max: number): Uint8Array[];
}
export declare namespace rng {
    /** Draws an integer below n, or zero when n is zero. */
    export function below(rng: Rng, n: number): number;
    /** Draws a fair coin flip. */
    export function boolean(rng: Rng): boolean;
    /** Returns true with probability p. */
    export function chance(rng: Rng, p: number): boolean;
    /** Builds the stream from a seed. */
    export function new_(seed: number | bigint): Rng;
    /** Draws an integer between lo and hi inclusive, or lo when hi is not above lo. */
    export function range(rng: Rng, lo: number | bigint, hi: number | bigint): bigint;
    /** Draws amount distinct indices below length, or every index when amount is larger. */
    export function sample_indices(rng: Rng, length: number, amount: number): Uint32Array;
    /** Draws a float at or above zero and below one. */
    export function unit(rng: Rng): number;
}
export declare namespace tensor {
    /** Returns the element at a flat index, which must be below the size like a slice index. */
    export function at(tensor: Tensor, flat: number): bigint;
    /** Maps every element to one at or above the threshold, zero below. */
    export function binarize(tensor: Tensor, threshold: number): Tensor;
    /** Binarizes at one above the Otsu threshold. */
    export function binarize_otsu(tensor: Tensor): Tensor;
    /** Averages every position over its masked neighborhood, rounded. */
    export function blur(tensor: Tensor, mask: Tensor, wrap: boolean): Tensor;
    /** Returns the elements as bytes. */
    export function bytes(tensor: Tensor): Uint8Array;
    /** Counts the cells holding one value. */
    export function count(tensor: Tensor, value: number): number;
    /** Returns the element width. */
    export function dtype(tensor: Tensor): Dtype;
    /** Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d. */
    export function exposed(tensor: Tensor): string;
    /** Builds a tensor of the shape and width filled with one value. */
    export function filled(shape: ArrayLike<number>, value: number | bigint, dtype: Dtype): Tensor;
    /** Reverses the tensor along one axis. */
    export function flip(tensor: Tensor, axis: number): Tensor;
    /** Folds the tensor into its level-fold Kronecker power. */
    export function fractal(tensor: Tensor, level: number): Tensor;
    /** Builds a u8 tensor filled with one value. */
    export function full(shape: ArrayLike<number>, value: number): Tensor;
    /** Returns the byte at a multi-index. */
    export function get(tensor: Tensor, multi: ArrayLike<number>): number;
    /** Wraps an i32 vector as a tensor of the shape. */
    export function i32(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    /** Returns the elements as i32s. */
    export function i32s(tensor: Tensor): Int32Array;
    /** Folds a multi-index into its flat index. */
    export function index(tensor: Tensor, multi: ArrayLike<number>): number;
    /** Flips every element between zero and one. */
    export function invert(tensor: Tensor): Tensor;
    /** Builds the Kronecker product of the two tensors. */
    export function kron(tensor: Tensor, other: Tensor): Tensor;
    /** Numbers every position by its concentric ring out from the center. */
    export function layers(tensor: Tensor, dtype: Dtype): Tensor;
    /** Counts each position's masked neighbors holding the target bit. */
    export function neighbors(tensor: Tensor, mask: Tensor, target: number, wrap: boolean, dtype: Dtype): Tensor;
    /** Builds a zeroed u8 tensor of the shape. */
    export function new_(shape: ArrayLike<number>): Tensor;
    /** Wraps a byte vector as a tensor of the shape. */
    export function of(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    /** Returns the Otsu threshold splitting the histogram at greatest variance. */
    export function otsu_threshold(tensor: Tensor): number;
    /** Wraps the tensor in a count-thick border of one value. */
    export function pad(tensor: Tensor, count: number, value: number): Tensor;
    /** Stamps the value wherever the tiled mask is nonzero. */
    export function perforate(tensor: Tensor, mask: Tensor, value: number): Tensor;
    /** Writes the element at a flat index, which must be below the size like a slice index. */
    export function put(tensor: Tensor, flat: number, value: number | bigint): void;
    /** Rotates the tensor k quarter turns in the plane of two axes. */
    export function rot90(tensor: Tensor, k: number, axes: [number, number]): Tensor;
    /** Writes the byte at a multi-index. */
    export function set(tensor: Tensor, multi: ArrayLike<number>, value: number): void;
    /** Returns the number of elements. */
    export function size(tensor: Tensor): number;
    /** Drops one axis by fixing it at an index. */
    export function slice(tensor: Tensor, axis: number, index: number): Tensor;
    /** Returns the sum of all elements. */
    export function sum(tensor: Tensor): bigint;
    /** Repeats the tensor the given number of times along each axis. */
    export function tile(tensor: Tensor, reps: ArrayLike<number>): Tensor;
    /** Swaps two axes. */
    export function transpose(tensor: Tensor, a: number, b: number): Tensor;
    /** Builds a zeroed tensor of the shape and width. */
    export function typed(shape: ArrayLike<number>, dtype: Dtype): Tensor;
    /** Wraps a u16 vector as a tensor of the shape. */
    export function u16(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    /** Returns the elements as u16s. */
    export function u16s(tensor: Tensor): Uint16Array;
    /** Wraps a u32 vector as a tensor of the shape. */
    export function u32(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    /** Returns the elements as u32s. */
    export function u32s(tensor: Tensor): Uint32Array;
    /** Wraps a u8 vector as a tensor of the shape, the same door as [`Tensor::of`]. */
    export function u8(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
}
