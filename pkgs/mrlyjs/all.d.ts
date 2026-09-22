export { default, initSync } from "./pkg/all/mrlyjs_all.js";

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
export declare namespace core {
    /** Squashes rgba pixels to the hex aspect, returning the new width, height and pixels. */
    export function hex_fit(pixels: ArrayLike<number>[], width: number, height: number, vertical: boolean, filter: core.Filter): [number, number, Uint8Array[]];
    /** Returns the size a hex rendering wears, the named axis squashed by the triangle ratio. */
    export function hex_size(width: number, height: number, vertical: boolean): [number, number];
    /** Resamples rgba pixels to a new size. */
    export function resample(pixels: ArrayLike<number>[], width: number, height: number, out_w: number, out_h: number, filter: core.Filter): Uint8Array[];
    /** Decodes a png to its width, height, and rgba colors. */
    export function unpng(bytes: ArrayLike<number>): [number, number, Uint8Array[]];
    /** The height of an equilateral triangle over its side, the squash a hex rendering wears. */
    export function HEX_RATIO(): number;
    /** The eight bytes every png file starts with. */
    export function PNG_MAGIC(): Uint8Array;
    /** A rule that turns counter values into colors. */
    export type Colorizer = { Bins: { background: ColorData; ramp: ColorData[] } };
    export const Colorizer: {
        /** Returns the default Colorizer. */
        default(): core.Colorizer;
        /** Builds the blue-to-red diverging ramp around a white middle. */
        diverge(): core.Colorizer;
        /** Builds the black-through-ember fire ramp: black, dark red, orange, light yellow. */
        fire(): core.Colorizer;
        /** Builds a binned colorizer from a gradient through the given stops. */
        gradient_bins(background: Color, colors: Color[], shades: number): core.Colorizer;
        /** Builds the white-to-black heat ramp. */
        heat(): core.Colorizer;
    };
    /** The element widths a tensor can hold. */
    export type Dtype = "U8" | "U16" | "U32" | "I32";
    export const Dtype: {
        /** Returns the largest value the width can hold. */
        max(dtype: core.Dtype): bigint;
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
        get width(): number;
        set width(value: number);
        /** The height in pixels. */
        get height(): number;
        set height(value: number);
        /** The palette index of every pixel, row by row. */
        get rows(): Uint32Array[];
        set rows(value: ArrayLike<number>[]);
        /** The colors the rows index. */
        get palette(): Color[];
        set palette(value: Color[]);
        /** Returns the flat rgba pixels, transparent wherever an index misses the palette. */
        colors(): Uint8Array[];
        /** Builds a paletted image from raw rgba pixels, growing the palette as new colors appear. */
        static from_pixels(width: number, height: number, pixels: ArrayLike<number>[]): core.Image;
        /** Encodes the image as a png at the given scale. */
        png(scale: number): Uint8Array;
        /** Resamples the image to a new size, its palette rebuilt from the blended pixels. */
        resample(width: number, height: number, filter: core.Filter): core.Image;
    }
    /** The ways paint picks a color within a type's palette. */
    export type Mode = "Type" | "Tag" | "Index" | "Enumerate" | "Random" | "Row" | "Column" | "Depth";
    export namespace cell {
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
        export function layers(cell: Cell, dtype: core.Dtype): Cell;
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
        export function neighbors(cell: Cell, mask: Tensor, target: number, wrap: boolean, dtype: core.Dtype): Cell;
        /** Wraps a tensor of types in a bare cell, colorless and tagless. */
        export function new_(types: Tensor): Cell;
        /** Wraps the cell in count layers of value on every side, dropping colors. */
        export function pad(cell: Cell, count: number, value: number): Cell;
        /** Colors every mapped cell, picking within each type's palette by the mode. */
        export function paint(cell: Cell, mapping: Record<string, Color[]>, mode: core.Mode, rng?: Rng): Cell;
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
    export namespace codec {
        /** Encodes indexed frames as an animated gif89a, each source pixel a scale by scale block. */
        export function gif(frames: ArrayLike<number>[], palette: ArrayLike<number>[], width: number, height: number, scale: number, delay: number): Uint8Array;
        /** Encodes rgba colors as a png, drawing each source pixel as a scale by scale block. */
        export function png(colors: ArrayLike<number>[], width: number, height: number, scale: number): Uint8Array;
    }
    export namespace colors {
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
        export function DARK(): core.colors.Theme;
        /** The palette gray, #8e8e93. */
        export function GRAY(): Color;
        /** The palette green, #32cc58. */
        export function GREEN(): Color;
        /** The palette indigo, #6768fa. */
        export function INDIGO(): Color;
        /** The light theme. */
        export function LIGHT(): core.colors.Theme;
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
            get ground(): Color;
            set ground(value: Color);
            /** The page background, one step off the ground. */
            get bg(): Color;
            set bg(value: Color);
            /** The raised panel. */
            get panel(): Color;
            set panel(value: Color);
            /** The sunken well. */
            get deep(): Color;
            set deep(value: Color);
            /** The hairline between things. */
            get line(): Color;
            set line(value: Color);
            /** The foreground, the strongest tone. */
            get fg(): Color;
            set fg(value: Color);
            /** The dimmed foreground, for anything secondary. */
            get dim(): Color;
            set dim(value: Color);
            /** The interactive accent. */
            get accent(): Color;
            set accent(value: Color);
            /** The tone written on the accent. */
            get on_accent(): Color;
            set on_accent(value: Color);
            /** The red ink. */
            get red(): Color;
            set red(value: Color);
            /** The orange ink. */
            get orange(): Color;
            set orange(value: Color);
            /** The yellow ink. */
            get yellow(): Color;
            set yellow(value: Color);
            /** The green ink. */
            get green(): Color;
            set green(value: Color);
            /** The mint ink. */
            get mint(): Color;
            set mint(value: Color);
            /** The teal ink. */
            get teal(): Color;
            set teal(value: Color);
            /** The cyan ink. */
            get cyan(): Color;
            set cyan(value: Color);
            /** The blue ink. */
            get blue(): Color;
            set blue(value: Color);
            /** The indigo ink. */
            get indigo(): Color;
            set indigo(value: Color);
            /** The purple ink. */
            get purple(): Color;
            set purple(value: Color);
            /** The pink ink. */
            get pink(): Color;
            set pink(value: Color);
            /** The brown ink. */
            get brown(): Color;
            set brown(value: Color);
            /** The gray ink. */
            get gray(): Color;
            set gray(value: Color);
            /** The thirteen inks in name order. */
            hues(): Color[];
            /** The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo. */
            inks(): Color[];
        }
    }
    export namespace error {
        /** Parses JSON text into a value. */
        export function parse(text: string): any;
    }
    export namespace image {
        /** Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window. */
        export function blur(pixels: ArrayLike<number>[], width: number, height: number, radius: number): Uint8Array[];
    }
    export namespace paint {
        /** Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream. */
        export function apply(paint: core.paint.Paint, cell: Cell, rng: Rng): void;
        /** Replays a stored paint onto a cell, tagging first and applying it from the stream. */
        export function coat(cell: Cell, paint: core.paint.Paint, mask: Tensor | undefined, rng: Rng): void;
        /** Draws a random paint under the config, applies it to the cell, and returns the recipe. */
        export function paint(cell: Cell, config: core.paint.Config, mask: Tensor | undefined, rng: Rng): core.paint.Paint;
        /** Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count. */
        export function prime(paint: core.paint.Paint, cell: Cell, mask: Tensor | undefined, rng: Rng): core.paint.Paint;
        /** Draws a random edition from the allowed list, or from all seven. */
        export function random_edition(editions: core.paint.Edition[] | undefined, rng: Rng): core.paint.Edition;
        /** Redraws the paint's secondary inks and shades under its scheme. */
        export function reroll(paint: core.paint.Paint, rng: Rng): core.paint.Paint;
        /** Draws the paint's scheme, target and primary under the config, then rerolls the rest. */
        export function setup(paint: core.paint.Paint, config: core.paint.Config, rng: Rng): core.paint.Paint;
        /** Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side. */
        export function tag(cell: Cell, edition: core.paint.Edition, target: core.paint.Target, mask?: Tensor): number;
        /** The constraints a caller may put on a random paint. */
        export interface Config {
            /** The editions allowed, or None for all seven. */
            editions?: core.paint.Edition[];
            /** The primary inks allowed, or None for black and white. */
            primaries?: core.paint.Ink[];
            /** The forced target, or None for a coin flip. */
            target?: core.paint.Target;
        }
        export const Config: {
            /** Returns the default Config. */
            default(): core.paint.Config;
        };
        /** The seven ways a paint distributes its colors over a cell. */
        export type Edition = "Simple" | "Index" | "Layers" | "Neighbors" | "Rows" | "Columns" | "Random";
        export const Edition: {
            /** Returns every Edition in canonical order. */
            all(): core.paint.Edition[];
            /** Returns the cell-painting mode this edition renders with, or None for Random, which scatters. */
            mode(edition: core.paint.Edition): core.Mode | undefined;
        };
        /** The fifteen named inks a paint draws from. */
        export type Ink = "Black" | "White" | "Red" | "Orange" | "Yellow" | "Green" | "Mint" | "Teal" | "Cyan" | "Blue" | "Indigo" | "Purple" | "Pink" | "Brown" | "Gray";
        export const Ink: {
            /** Returns every Ink in canonical order. */
            all(): core.paint.Ink[];
            /** Returns the ink's color. */
            color(ink: core.paint.Ink): Color;
        };
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
            /** Builds a black-primary, fill-target, multicolor paint for an edition. */
            constructor(edition: core.paint.Edition);
            free(): void;
            /** Reads the Paint from its plain data. */
            static from(data: PaintData): Paint;
            /** Writes the Paint as plain data. */
            toJSON(): PaintData;
            /** The coloring edition. */
            get edition(): core.paint.Edition;
            set edition(value: core.paint.Edition);
            /** The secondary color scheme. */
            get scheme(): core.paint.Scheme;
            set scheme(value: core.paint.Scheme);
            /** The side the primary ink lands on. */
            get target(): core.paint.Target;
            set target(value: core.paint.Target);
            /** The primary ink. */
            get primary(): core.paint.Ink;
            set primary(value: core.paint.Ink);
            /** The secondary inks. */
            get secondary(): core.paint.Ink[];
            set secondary(value: core.paint.Ink[]);
            /** The shade indices of a multitone ramp. */
            get shades(): Uint32Array;
            set shades(value: ArrayLike<number>);
            /** Returns true for the Simple edition. */
            is_simple(): boolean;
        }
        /** The two ways secondary colors are drawn. */
        export type Scheme = "Multicolor" | "Multitone";
        export const Scheme: {
            /** Returns every Scheme in canonical order. */
            all(): core.paint.Scheme[];
        };
        /** The side of the figure the primary ink lands on. */
        export type Target = "Fill" | "Void";
        export const Target: {
            /** Returns every Target in canonical order. */
            all(): core.paint.Target[];
        };
    }
    export namespace ramp {
        /** Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up. */
        export function color(colorizer: core.Colorizer, value: number, max: number): Color;
        /** Maps a slice of values to rgba pixels against the range maximum. */
        export function colors(colorizer: core.Colorizer, values: ArrayLike<number>, max: number): Uint8Array[];
    }
    export namespace rng {
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
    export namespace tensor {
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
        export function dtype(tensor: Tensor): core.Dtype;
        /** Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d. */
        export function exposed(tensor: Tensor): string;
        /** Builds a tensor of the shape and width filled with one value. */
        export function filled(shape: ArrayLike<number>, value: number | bigint, dtype: core.Dtype): Tensor;
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
        export function layers(tensor: Tensor, dtype: core.Dtype): Tensor;
        /** Counts each position's masked neighbors holding the target bit. */
        export function neighbors(tensor: Tensor, mask: Tensor, target: number, wrap: boolean, dtype: core.Dtype): Tensor;
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
        export function typed(shape: ArrayLike<number>, dtype: core.Dtype): Tensor;
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
}
export declare namespace font {
    /** Builds every glyph in font order: uppers, lowers, digits, extras, specials. */
    export function all(): font.Glyph[];
    /** Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster. */
    export function animate(text: string, pad: number): font.Anim;
    /** Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any. */
    export function cycle(write: font.Anim, merge: ArrayLike<number>[], hold: number): font.Anim;
    /** Builds the ten digit glyphs. */
    export function digits(): font.Glyph[];
    /** Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck. */
    export function draft(rows: string[]): [number, number][][];
    /** Builds the punctuation, symbol and arrow glyphs. */
    export function extras(): font.Glyph[];
    /** Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank. */
    export function floor(rows: string[]): number;
    /** Returns an owned copy of the character's glyph, or None outside the font. */
    export function glyph(c: string): font.Glyph | undefined;
    /** Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form. */
    export function lower(rows: string[]): string[];
    /** Builds the twenty-six lowercase glyphs by rounding the uppers' corners. */
    export function lowers(): font.Glyph[];
    /** Returns the whole font as a map from character to bitmap rows. */
    export function map(): Record<string, string[]>;
    /** Folds the written text's glyphs, frame by frame, into one centered stack. */
    export function merge(text: string, pad: number): Uint32Array[];
    /** Returns the character's Unicode name, or a U+ code point label for a character outside the font. */
    export function name_of(c: string): string;
    /** Flattens the character's strokes into one cell-by-cell drawing order. */
    export function path(c: string): [number, number][];
    /** Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart. */
    export function raster(text: string): Uint8Array[];
    /** Builds the four seven-row glyphs: dollar, at, copyright and registered. */
    export function specials(): font.Glyph[];
    /** Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font. */
    export function strokes(c: string): [number, number][][];
    /** Returns every character in the font, in font order. */
    export function supported(): string[];
    /** Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has. */
    export function trim(rows: string[]): string[];
    /** Builds the twenty-six uppercase glyphs. */
    export function uppers(): font.Glyph[];
    /** The playback rate of every animation, in frames per second. */
    export function FPS(): number;
    /** The default number of frames a cycle rests between movements. */
    export function HOLD(): number;
    /** A frame-by-frame animation over a fixed board. */
    export interface Anim {
        /** The board height in cells. */
        rows: number;
        /** The board width in cells. */
        cols: number;
        /** The playback rate in frames per second. */
        fps: number;
        /** The frames, each a sorted list of lit row-major cell indices. */
        frames: number[][];
    }
    export interface GlyphData {
        /** The character the glyph draws. */
        char: string;
        /** The bitmap rows of '0' and '1' characters. */
        rows: string[];
    }
    /** One character's pixel bitmap. */
    export class Glyph {
        /** Builds a glyph from its character and rows. */
        constructor(char: string, rows: string[]);
        free(): void;
        /** Reads the Glyph from its plain data. */
        static from(data: GlyphData): Glyph;
        /** Writes the Glyph as plain data. */
        toJSON(): GlyphData;
        /** The character the glyph draws. */
        get char(): string;
        set char(value: string);
        /** The bitmap rows of '0' and '1' characters. */
        get rows(): string[];
        set rows(value: string[]);
        /** Returns the number of rows. */
        height(): number;
        /** Returns the cell width of the first row, or 0 for an empty glyph. */
        width(): number;
    }
    export namespace paths {
        /** Returns the character's hand-penned strokes from the pen tables, or None outside the font. */
        export function penned(c: string): [number, number][][] | undefined;
    }
    export namespace pens {
        /** Returns every pen in font order: uppers, lowers, digits, extras, specials. */
        export function all(): [string, string[]][];
    }
}
export declare namespace gen {
    /** Draws one seeded artwork and returns its PNG bytes: a random flat tile under the default recipe */
    export function background(seed: number | bigint, width: number, height: number): Uint8Array;
    /** Returns the plane's bang code of a classic design, or None for one outside the plane. */
    export function classic_code(design: gen.recipe.Design): string | undefined;
    /** Returns the bang code of a named design in a dimension, or None where it has no design. */
    export function classic_code_nd(design: gen.recipe.Design, dimension: number): string | undefined;
    /** Draws a hex key of the given length from the stream. */
    export function hex_key(length: number, rng: Rng): string;
    /** Draws one of the four flat classics from the stream: carpet, net, vertical tree or void. */
    export function random_design(rng: Rng): gen.recipe.Design;
    /** Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3. */
    export function random_rotation(design: gen.recipe.Design, rng: Rng): number;
    /** Builds the mask of a mosaic tile: the two trees of the side, two where they cross. */
    export function tree_mask(n: number): Tensor;
    /** The five construction families a tile can belong to. */
    export type Group = "General" | "Fractal" | "Magic" | "Special" | "Mosaic";
    export const Group: {
        /** Returns every Group in canonical order. */
        all(): gen.Group[];
    };
    /** The parity filter over candidate sizes. */
    export type Parity = "Evens" | "Odds" | "Both";
    export const Parity: {
        /** Returns every Parity in canonical order. */
        all(): gen.Parity[];
        /** Returns true when the number passes the filter. */
        keep(parity: gen.Parity, n: number): boolean;
    };
    export interface TileData {
        /** The construction family. */
        group: gen.Group;
        /** The base factor of the construction. */
        factor: number;
        /** The origin of each layer. */
        sources: gen.recipe.Source[];
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
        constructor(group: gen.Group);
        free(): void;
        /** Reads the Tile from its plain data. */
        static from(data: TileData): Tile;
        /** Writes the Tile as plain data. */
        toJSON(): TileData;
        /** The construction family. */
        get group(): gen.Group;
        set group(value: gen.Group);
        /** The base factor of the construction. */
        get factor(): number;
        set factor(value: number);
        /** The origin of each layer. */
        get sources(): gen.recipe.Source[];
        set sources(value: gen.recipe.Source[]);
        /** The grid size of each source. */
        get numbers(): Uint32Array;
        set numbers(value: ArrayLike<number>);
        /** The fractal level of each source. */
        get levels(): Uint32Array;
        set levels(value: ArrayLike<number>);
        /** The quarter-turn rotation of each source. */
        get rotations(): Uint32Array;
        set rotations(value: ArrayLike<number>);
        /** Whether the finished tile inverts. */
        get invert(): boolean;
        set invert(value: boolean);
        /** Whether the finished tile flips. */
        get flip(): boolean;
        set flip(value: boolean);
        /** The tile's width in cells. */
        get width(): number;
        set width(value: number);
        /** The tile's height in cells. */
        get height(): number;
        set height(value: number);
        /** Checks that the slots, numbers and sizes agree. */
        check(): void;
        /** Returns whether the recipe is a magic tile of one repeated source at one repeated number, */
        degenerate(): boolean;
        /** Returns the larger of width and height. */
        max_size(): number;
        /** Recomputes the factor and side length the group and numbers imply, zero when they overflow. */
        resize(): void;
        /** Sets the tile's width and height. */
        size(width: number, height: number): gen.Tile;
    }
    export namespace build {
        /** Builds the flat cell the tile describes. */
        export function build_2d(tile: gen.Tile): Cell;
        /** Builds the cube the tile describes. */
        export function build_3d(tile: gen.Tile): Cell;
        /** Builds the tile's cube and flattens it through its projection. */
        export function build_6d(hex: gen.build.HexTile): Cell6d;
        /** Draws a random flat tile from the stream, rotations from the four quarter-turns. */
        export function create_2d(config: gen.draw.ConfigNd, rng: Rng): gen.Tile;
        /** Draws a cube tile from the config with cube orientations drawn from the stream. */
        export function create_3d(config: gen.draw.ConfigNd, rng: Rng): gen.Tile;
        /** Draws a cube tile from the config under a projection drawn from the stream. */
        export function create_6d(config: gen.draw.ConfigNd, rng: Rng): gen.build.HexTile;
        /** Draws a random flat tile up to the given size under the default config. */
        export function random_tile_2d(max_size: number, rng: Rng): gen.Tile;
        /** Draws a random cube tile up to the given size. */
        export function random_tile_3d(max_size: number, rng: Rng): gen.Tile;
        /** Draws a random cube tile up to the given size under a random projection. */
        export function random_tile_6d(max_size: number, rng: Rng): gen.build.HexTile;
        export type Config2d = gen.draw.ConfigNd;
        export const Config2d: {
            /** Returns the default Config2d. */
            default(): gen.draw.ConfigNd;
        };
        export type Config3d = gen.draw.ConfigNd;
        export const Config3d: {
            /** Returns the default Config3d. */
            default(): gen.draw.ConfigNd;
        };
        /** A cube tile paired with the projection that flattens it. */
        export interface HexTile {
            /** The projection that flattens the tile. */
            projection: math.six.Projection;
            /** The cube tile underneath. */
            tile: gen.TileData;
        }
    }
    export namespace draw {
        /** The constraints a random tile is drawn under. */
        export interface ConfigNd {
            /** The tile groups allowed. */
            groups: gen.Group[];
            /** The catalog the sources are drawn from. */
            catalog: gen.recipe.Catalog;
            /** The smallest allowed side. */
            min_size: number;
            /** The largest allowed side. */
            max_size: number;
            /** The parity the sizes must keep. */
            parity: gen.Parity;
            /** The forced inversion flag, or None to flip a coin. */
            invert?: boolean;
        }
    }
    export namespace name {
        /** One value for every slot of a tile, or one value per slot. */
        export type Slots = number | number[];
        export const Slots: {
            /** Returns the default Slots. */
            default(): gen.name.Slots;
        };
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
            side: gen.name.Slots;
            /** The power a fractal tile is raised to, absent at one. */
            level?: number;
            /** The quarter turns of each slot, absent when nothing turns. */
            turn: gen.name.Slots;
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
            get code(): string | undefined;
            set code(value: string | number | bigint | undefined);
            /** The mask code of a special tile. */
            get special(): string | undefined;
            set special(value: string | number | bigint | undefined);
            /** The letters of a magic tile, first letter outermost. */
            get magic(): string[];
            set magic(value: (string | number | bigint)[]);
            /** The three codes of a mosaic tile. */
            get mosaic(): string[];
            set mosaic(value: (string | number | bigint)[]);
            /** The side of the mask of a special or mosaic tile. */
            get factor(): number | undefined;
            set factor(value: number | undefined);
            /** The side each slot renders at, one per letter for a magic tile. */
            get side(): gen.name.Slots;
            set side(value: gen.name.Slots);
            /** The power a fractal tile is raised to, absent at one. */
            get level(): number | undefined;
            set level(value: number | undefined);
            /** The quarter turns of each slot, absent when nothing turns. */
            get turn(): gen.name.Slots;
            set turn(value: gen.name.Slots);
            /** Whether a special tile flips its mask. */
            get flip(): boolean;
            set flip(value: boolean);
            /** Whether the finished tile inverts. */
            get invert(): boolean;
            set invert(value: boolean);
            /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
            checked(): gen.name.Tile;
            /** Reads a filename back into the value, or an error. */
            static from_file(text: string): gen.name.Tile;
            /** Reads a JSON object into its canonical value, or an error naming the broken key. */
            static from_json(text: string): gen.name.Tile;
            /** Reads a path and query string back into the value, or an error. */
            static from_url(text: string): gen.name.Tile;
            /** Folds a recipe to its name. */
            static of(recipe: gen.Tile): gen.name.Tile;
            /** Builds the recipe the name folds, resized and checked. */
            recipe(): gen.Tile;
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
    export namespace recipe {
        /** Returns the classic designs for a dimension. */
        export function classics(dimension: number): gen.recipe.Design[];
        /** Returns every flat size in the range that passes the parity filter. */
        export function generals(min_size: number, max_size: number, parity: gen.Parity): Uint32Array;
        /** Returns every factor list of depth two and beyond whose product lands in the size range. */
        export function nestings(min_size: number, max_size: number, parity: gen.Parity): Uint32Array[];
        /** Returns every factor and level whose power lands in the size range. */
        export function powers(min_size: number, max_size: number, parity: gen.Parity): [number, number][];
        /** Returns every count-long factor list whose product lands in the size range. */
        export function products(min_size: number, max_size: number, count: number, parity: gen.Parity): Uint32Array[];
        /** Returns the side a factor raised to a level makes, or None when no usize holds it. */
        export function size(number: number | bigint, level: number | bigint): number | undefined;
        /** The five classic designs of the plane. */
        export function CLASSICS_2D(): gen.recipe.Design[];
        /** The six classic designs of the cube. */
        export function CLASSICS_3D(): gen.recipe.Design[];
        /** The deepest fractal level a tile may take. */
        export function MAX_LEVEL(): number;
        /** The largest side, number or factor a tile may take. */
        export function MAX_SIDE(): number;
        /** The most slots a magic tile may take. */
        export function MAX_SLOTS(): number;
        /** The smallest side, number or factor a tile may take. */
        export function MIN_SIDE(): number;
        /** The pool of sources a tile may draw from. */
        export type Catalog = "Classics" | "Universe" | { Codes: bigint[] } | { Designs: gen.recipe.Design[] };
        /** The named designs a source can point at: the four classics and their four antis. */
        export type Design = "Carpet" | "Net" | "Htree" | "Vtree" | "Void" | "Xtree" | "Ytree" | "Ztree" | "Point" | "Dust" | "Hline" | "Vline" | "Star" | "Xline" | "Yline" | "Zline";
        export const Design: {
            /** Returns every Design in canonical order. */
            all(): gen.recipe.Design[];
        };
        /** The origin of one tile layer, a one-field json object. */
        export type Source = { design: gen.recipe.Design } | { code: bigint };
    }
    export namespace variation {
        /** Draws a variation's seed from the stream, then the variation itself on that seed, with a */
        export function create(config: gen.variation.Config, rng: Rng): gen.variation.Variation;
        /** Builds the variation's base cell and draws its paint from the stream, painting the base */
        export function generate(variation: gen.variation.Variation, config: gen.variation.Config, rng: Rng): gen.variation.Variation;
        /** Renders every file of the variation to PNG at the given scale, scattering a Random edition */
        export function render(variation: gen.variation.Variation, scale: number, rng: Rng): gen.variation.Variation;
        /** The settings an artwork is drawn under. */
        export interface Config {
            /** The constraints the tile is drawn under. */
            tile: gen.draw.ConfigNd;
            /** The constraints the paint is drawn under. */
            paint: core.paint.Config;
            /** The width and height repetition pairs to render. */
            files: [number, number][];
        }
        export const Config: {
            /** Returns the default Config. */
            default(): gen.variation.Config;
        };
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
            get width(): number;
            set width(value: number);
            /** The count of tile repetitions down. */
            get height(): number;
            set height(value: number);
            /** The encoded PNG bytes, empty until rendered and left out of the json. */
            get png(): Uint8Array;
            set png(value: ArrayLike<number>);
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
            tile: gen.TileData;
            /** The mask tile, present only under the Neighbors edition. */
            mask?: gen.TileData;
            /** The paint, set by generate. */
            paint?: core.paint.PaintData;
            /** The renderings, filled by render. */
            files: gen.variation.FileData[];
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
            get key(): string;
            set key(value: string);
            /** The seed the variation is drawn under. */
            get seed(): bigint;
            set seed(value: number | bigint);
            /** The paint edition. */
            get edition(): core.paint.Edition;
            set edition(value: core.paint.Edition);
            /** The primary inks, when the config fixes them. */
            get primaries(): core.paint.Ink[] | undefined;
            set primaries(value: core.paint.Ink[] | undefined);
            /** The tile recipe. */
            get tile(): gen.Tile;
            set tile(value: gen.Tile);
            /** The mask tile, present only under the Neighbors edition. */
            get mask(): gen.Tile | undefined;
            set mask(value: gen.Tile | undefined);
            /** The paint, set by generate. */
            get paint(): core.paint.Paint | undefined;
            set paint(value: core.paint.Paint | undefined);
            /** The built base cell, set by generate and left out of the json. */
            get base(): Cell | undefined;
            set base(value: Cell | undefined);
            /** The renderings, filled by render. */
            get files(): gen.variation.File[];
            set files(value: gen.variation.File[]);
            /** Returns whether the edition paints the whole tiled canvas. */
            is_cover(): boolean;
            /** Returns whether the edition paints the base cell before tiling. */
            is_prime(): boolean;
        }
    }
}
export declare namespace life {
    /** Returns whether a rule is affine, its algebraic degree at most one. */
    export function affine(rule: number): boolean;
    /** Runs a seed under a config until it fixes, loops or times out, recording every generation. */
    export function animate(seed: Cell, config: life.Config): life.Life;
    /** Returns the mean fraction of sites changed between consecutive grids. */
    export function churn(grids: Cell[]): number;
    /** Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`. */
    export function corner_bits(rule: number): Uint8Array;
    /** Returns the sequence up to max_neighbors, keeping zeros and ones only on request. */
    export function counts(seq: life.Source, max_neighbors: number, include_zeros: boolean, include_ones: boolean): Uint32Array;
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
    export function next_grid(cell: Cell, birth: ArrayLike<number>, survive: ArrayLike<number>, mask: Tensor, boundary: life.Boundary): Cell;
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
        all(): life.Boundary[];
        /** Returns whether the edges wrap. */
        wrap(boundary: life.Boundary): boolean;
    };
    export interface ConfigData {
        /** The neighborhood mask. */
        mask: { cell: CellData };
        /** The neighbor counts that create a cell. */
        birth: life.CountsData;
        /** The neighbor counts that keep a cell. */
        survive: life.CountsData;
        /** The edge policy. */
        boundary: life.Boundary;
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
        constructor(mask: Cell, birth: life.Counts, survive: life.Counts);
        free(): void;
        /** Reads the Config from its plain data. */
        static from(data: ConfigData): Config;
        /** Writes the Config as plain data. */
        toJSON(): ConfigData;
        /** The neighborhood mask. */
        get mask(): Cell;
        set mask(value: Cell);
        /** The neighbor counts that create a cell. */
        get birth(): life.Counts;
        set birth(value: life.Counts);
        /** The neighbor counts that keep a cell. */
        get survive(): life.Counts;
        set survive(value: life.Counts);
        /** The edge policy. */
        get boundary(): life.Boundary;
        set boundary(value: life.Boundary);
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
    export type CountsData = { List: number[] } | { Drawn: { seq: life.SourceData; zeros: boolean; ones: boolean } };
    /** The neighbor counts one side of a rule fires on. */
    export class Counts {
        private constructor();
        free(): void;
        /** Reads the Counts from its plain data. */
        static from(data: CountsData): Counts;
        /** Writes the Counts as plain data. */
        toJSON(): CountsData;
        /** Builds the counts a sequence lays down, keeping zeros and ones on request. */
        static drawn(seq: life.Source, zeros: boolean, ones: boolean): life.Counts;
        /** Spells the counts outright. */
        static list(counts: ArrayLike<number>): life.Counts;
        /** Returns the counts, a drawn side resolved against the mask's neighbor budget. */
        values(budget: number): Uint32Array;
    }
    /** The ending of a life run. */
    export type Fate = "Dead" | "Alive" | "Loop" | "Timeout";
    export const Fate: {
        /** Returns every Fate in canonical order. */
        all(): life.Fate[];
    };
    export interface LifeData {
        /** Every generation in order. */
        grids: { cell: CellData }[];
        /** The run's ending. */
        fate: life.Fate;
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
        get fate(): life.Fate;
        set fate(value: life.Fate);
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
        birth: life.CountsData;
        /** The neighbor counts that keep a cell, listed or drawn from a sequence. */
        survive: life.CountsData;
        /** Whether the edge wraps, false unless said. */
        wrap: boolean;
    }
    /** A life rule: the birth and survival counts and whether the edge wraps. */
    export class Rule {
        /** Builds a rule from its counts and edge policy, listed counts folded to a sorted set. */
        constructor(birth: life.Counts, survive: life.Counts, wrap: boolean);
        free(): void;
        /** Reads the Rule from its plain data. */
        static from(data: RuleData): Rule;
        /** Writes the Rule as plain data. */
        toJSON(): RuleData;
        /** The neighbor counts that create a cell, listed or drawn from a sequence. */
        get birth(): life.Counts;
        set birth(value: life.Counts);
        /** The neighbor counts that keep a cell, listed or drawn from a sequence. */
        get survive(): life.Counts;
        set survive(value: life.Counts);
        /** Whether the edge wraps, false unless said. */
        get wrap(): boolean;
        set wrap(value: boolean);
        /** Returns the edge policy the rule runs under. */
        boundary(): life.Boundary;
        /** Folds a decoded value to its canonical form, or an error for one outside the kind. */
        checked(): life.Rule;
        /** Builds a life config running this rule over a neighborhood mask. */
        config(mask: Cell): life.Config;
        /** Reads a filename back into the value, or an error. */
        static from_file(text: string): life.Rule;
        /** Reads a JSON object into its canonical value, or an error naming the broken key. */
        static from_json(text: string): life.Rule;
        /** Reads a path and query string back into the value, or an error. */
        static from_url(text: string): life.Rule;
        /** Reads the rule out of a life config. */
        static of(config: life.Config): life.Rule;
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
        static all(): life.Source[];
        /** Returns the seventeen mrly design families: the grid, the four classics and their antis. */
        static designs(): life.Source[];
        /** Returns whether the sequence is a seeded random draw. */
        is_random(): boolean;
        /** Returns the sequence's parseable name, the one string that regenerates it. */
        name(): string;
        /** Returns the six number sequences, the random one listed under seed zero. */
        static numbers(): life.Source[];
        /** Returns the sequence's OEIS id, or None off the encyclopedia. */
        oeis(): string | undefined;
        /** Parses a sequence name back to its source. */
        static parse(name: string): life.Source;
        /** Reads a canonical name off the front of the text, returning the tail left over. */
        static read(text: string): [life.Source, string] | undefined;
    }
    export namespace elementary {
        /** Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell. */
        export function output(rule: number, l: number, c: number, r: number): number;
    }
    export namespace render {
        /** Renders one grid to white-on-black PNG bytes at a pixel scale. */
        export function frame(grid: Cell, scale: number): Uint8Array;
    }
    export namespace source {
        /** Generates the sequence's values up to the limit. */
        export function sequence(seq: life.Source, limit: number): Uint32Array;
    }
}
export declare namespace math {
    export namespace atoms {
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
    export namespace bang {
        /** Builds the universe of a dimension. */
        export function bang(dimension: number): math.bang.Universe;
        /** Unpacks a code into its filled residue corners. */
        export function code_to_corners(code: string | number | bigint, dimension: number, base: number): Uint8Array[];
        /** Returns the binary corners of a dimension in code order. */
        export function corners(dimension: number): Uint8Array[];
        /** Packs filled residue corners back into their code. */
        export function corners_to_code(filled: ArrayLike<number>[], dimension: number, base: number): string;
        /** Returns the code of the design filled wherever a corner's residue sum lands in the levels. */
        export function levels_code(dimension: number, base: number, levels: ArrayLike<number>): string;
        /** Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost. */
        export function magic(layers: math.bang.MagicLayer[]): Tensor;
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
            design: math.name.BangData;
            /** The layer's side number. */
            number: number;
        }
        export const MagicLayer: {
            /** Pins a design to the side number it renders at. */
            "new"(design: math.name.Bang, number: number): math.bang.MagicLayer;
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
            all(): math.bang.Design[];
            /** Returns the designs whose codes lead their orbits. */
            canonical(): math.bang.Design[];
            /** Returns the design at a code with its precomputed orbit facts. */
            design(code: string | number | bigint): math.bang.Design;
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
            export function components(layers: math.bang.MagicLayer[]): string;
            /** Returns the constant-word component functional of a plane word's letter frequencies, */
            export function constant_functional(layers: math.bang.MagicLayer[]): number;
            /** Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides. */
            export function dimension(layers: math.bang.MagicLayer[]): number;
            /** Returns the filled cells of a word, the product of its letter fills. */
            export function fill(layers: math.bang.MagicLayer[]): string;
            /** Lists the filled cells of every letter, the product of which is the word's fill. */
            export function fills(layers: math.bang.MagicLayer[]): string[];
            /** Reads one plane letter: its fill, its runs, the rows and columns that wrap into a */
            export function letter(layer: math.bang.MagicLayer): math.bang.word.Letter;
            /** Returns whether every letter renders at its own residue base, the native case where a */
            export function native(layers: math.bang.MagicLayer[]): boolean;
            /** Returns the shortest whole period of the letter list, its own length when no shorter block repeats. */
            export function period(layers: math.bang.MagicLayer[]): number;
            /** Folds a plane word letter by letter and returns the counts at every prefix. */
            export function prefixes(layers: math.bang.MagicLayer[]): math.bang.word.Counts[];
            /** Returns the prefix rates of a plane word in log two units, the component rate */
            export function rates(layers: math.bang.MagicLayer[]): [number, number][];
            /** Returns the side of a word, the product of its letter sides. */
            export function side(layers: math.bang.MagicLayer[]): string;
            /** Spells the first letters of a schedule over an ordered pair of letters. */
            export function spell(schedule: math.bang.word.Schedule, pair: [math.bang.MagicLayer, math.bang.MagicLayer], length: number): math.bang.MagicLayer[];
            /** Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`, */
            export function staircase(depth: number): math.bang.MagicLayer[];
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
                all(): math.bang.word.Schedule[];
                /** Returns the letter frequencies the schedule tends to. */
                frequencies(schedule: math.bang.word.Schedule): [number, number];
                /** Returns the letter the schedule takes at the place, zero or one. */
                place(schedule: math.bang.word.Schedule, index: number): number;
            };
        }
    }
    export namespace cell {
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
    export namespace counts {
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
            static from_corners(filled: ArrayLike<number>[], number: number, dimension: number, base: number): math.counts.Exposure;
            /** Reads the counts off a rendered tile. */
            static of_tile(tile: Tensor): math.counts.Exposure;
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
    export namespace graph {
        /** Takes the full census of a network. */
        export function census(network: math.graph.Network): math.graph.Census;
        /** Counts the connected components of the network. */
        export function components(network: math.graph.Network): number;
        /** Extracts the network of filled sites joined to their axis neighbors. */
        export function core_graph(grid: Tensor): math.graph.Network;
        /** Extracts the network of corners and edges outlining every filled site. */
        export function edge_graph(grid: Tensor): math.graph.Network;
        /** Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample. */
        export function fractal_dimension(network: math.graph.Network, samples: number): number;
        /** Counts the nodes of degree three or more. */
        export function junctions(network: math.graph.Network): number;
        /** Extracts the largest connected piece as a network of its own, branches re-indexed. */
        export function largest_component(network: math.graph.Network): math.graph.Network;
        /** Tags every node by its degree, indexed like the node list. */
        export function roles(network: math.graph.Network): math.graph.Role[];
        /** Counts the nodes of degree one. */
        export function tips(network: math.graph.Network): number;
        /** Sums the straight-line lengths of every branch. */
        export function total_length(network: math.graph.Network): number;
        /** Extracts the core graph of the inverted grid, joining empty sites instead. */
        export function tunnel_graph(grid: Tensor): math.graph.Network;
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
            static from_network(network: math.graph.Network, seed: number | bigint): math.graph.Layout;
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
            nodes: math.graph.Node[];
            /** The branches in insertion order. */
            branches: math.graph.Branch[];
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
            get nodes(): math.graph.Node[];
            set nodes(value: math.graph.Node[]);
            /** The branches in insertion order. */
            get branches(): math.graph.Branch[];
            set branches(value: math.graph.Branch[]);
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
    export namespace moire {
        /** Returns every preset stacked up to the given scale. */
        export function all(limit: number): math.moire.Preset[];
        /** Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal. */
        export function frame(normal: ArrayLike<number>, offset: number): math.moire.Frame;
        /** Samples a design over the pixel grid into a boolean mask. */
        export function layer(params: math.moire.Layer): boolean[];
        /** Returns the preset the name picks. */
        export function named(name: string, limit: number): math.moire.Preset;
        /** Quantizes a field into colored levels and encodes PNG bytes. */
        export function render(field: math.moire.Field, colorizer: core.Colorizer, levels: number, symmetric: boolean, invert: boolean, scale: number): Uint8Array;
        /** Layers one design at several side numbers into a field under the chosen combine. */
        export function stack(spec: math.moire.Spec, numbers: ArrayLike<number>, combine: math.moire.Combine, level: number, lattice: math.moire.Lattice, size: number, slices: ArrayLike<number>): math.moire.Field;
        /** Sums layers of several designs at one side number into a field. */
        export function stack_codes(specs: math.moire.Spec[], number: number, level: number, lattice: math.moire.Lattice, size: number, slices: ArrayLike<number>): math.moire.Field;
        /** Layers one cube design at several side numbers into a volume under the chosen combine. */
        export function volume(spec: math.moire.Spec, numbers: ArrayLike<number>, combine: math.moire.Combine, level: number, size: number): math.moire.Volume;
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
            static from_data(data: ArrayLike<number>, size: number): math.moire.Field;
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
            spec: math.moire.Spec;
            /** The side number of the residue grid. */
            number: number;
            /** The fractal depth. */
            level: number;
            /** The sampling lattice. */
            lattice: math.moire.Lattice;
            /** The output side in pixels. */
            size: number;
            /** The 0..1 positions fixing the axes beyond the first two. */
            slices: number[];
        }
        export const Layer: {
            /** Builds a layer at level 1 on a 512-pixel square lattice. */
            "new"(spec: math.moire.Spec, number: number): math.moire.Layer;
        };
        export interface PresetData {
            /** The name the recipe answers to. */
            name: string;
            /** The design sampled at every scale. */
            spec: math.moire.Spec;
            /** The side numbers stacked. */
            numbers: number[];
            /** The way the layers merge. */
            combine: math.moire.Combine;
            /** The fractal depth of each layer. */
            level: number;
            /** The lattice the layers are sampled on. */
            lattice: math.moire.Lattice;
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
            get spec(): math.moire.Spec;
            set spec(value: math.moire.Spec);
            /** The side numbers stacked. */
            get numbers(): Uint32Array;
            set numbers(value: ArrayLike<number>);
            /** The way the layers merge. */
            get combine(): math.moire.Combine;
            set combine(value: math.moire.Combine);
            /** The fractal depth of each layer. */
            get level(): number;
            set level(value: number);
            /** The lattice the layers are sampled on. */
            get lattice(): math.moire.Lattice;
            set lattice(value: math.moire.Lattice);
            /** The carpet stack: every base-three corner but the centre, summed over odd scales. */
            static carpet(limit: number): math.moire.Preset;
            /** Samples the preset into a square field of the given side. */
            field(size: number): math.moire.Field;
            /** The parity heatmap: odd scales of the low corner summed on the square lattice. */
            static heatmap(limit: number): math.moire.Preset;
            /** The hive: the parity heatmap sampled on the hexagonal lattice. */
            static hive(limit: number): math.moire.Preset;
            /** The parity weave: the same odd scales folded to their parity instead of summed. */
            static weave(limit: number): math.moire.Preset;
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
            "new"(code: string | number | bigint, base: number, dimension: number): math.moire.Spec;
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
            static from_data(data: ArrayLike<number>, size: number): math.moire.Volume;
            /** Returns the largest sample. */
            max(): number;
            /** Returns the smallest sample. */
            min(): number;
            /** Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube. */
            plane(frame: math.moire.Frame, out: number): [Float32Array, Uint8Array];
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
            export function witness(scale: number): math.moire.pairs.Witness;
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
            export function axes(size: number, lattice: math.moire.Lattice, row: number): [Float64Array, Float64Array];
            /** Unpacks a code into its residue-corner truth table. */
            export function membership(code: string | number | bigint, base: number, dimension: number): boolean[];
            /** Folds residues into a base-q index of the truth table. */
            export function pack(residues: ArrayLike<number>, base: number): number;
        }
    }
    export namespace name {
        export interface BangData {
            /** The kind word. */
            kind: string;
            /** The number of axes. */
            dim: number;
            /** The lattice, square unless said. */
            lattice: math.name.Lattice;
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
            get lattice(): math.name.Lattice;
            set lattice(value: math.name.Lattice);
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
            checked(): math.name.Bang;
            /** Reads a filename back into the value, or an error. */
            static from_file(text: string): math.name.Bang;
            /** Reads a JSON object into its canonical value, or an error naming the broken key. */
            static from_json(text: string): math.name.Bang;
            /** Reads a path and query string back into the value, or an error. */
            static from_url(text: string): math.name.Bang;
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
            default(): math.name.Lattice;
            /** Returns whether this is the square lattice. */
            is_square(lattice: math.name.Lattice): boolean;
            /** Returns the number of unit directions a twist may pick from. */
            units(lattice: math.name.Lattice): number;
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
            checked(): math.name.Sequence;
            /** Returns the design pinned to its dimension and base. */
            design(): math.name.Bang;
            /** Reads a filename back into the value, or an error. */
            static from_file(text: string): math.name.Sequence;
            /** Reads a JSON object into its canonical value, or an error naming the broken key. */
            static from_json(text: string): math.name.Sequence;
            /** Reads a path and query string back into the value, or an error. */
            static from_url(text: string): math.name.Sequence;
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
            checked(): math.name.Word;
            /** Reads a filename back into the value, or an error. */
            static from_file(text: string): math.name.Word;
            /** Reads a JSON object into its canonical value, or an error naming the broken key. */
            static from_json(text: string): math.name.Word;
            /** Reads a path and query string back into the value, or an error. */
            static from_url(text: string): math.name.Word;
            /** Returns every letter as a design pinned to the word's dimension and its own base. */
            letters(): math.name.Bang[];
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
    export namespace press {
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
        export function layer_table(layer: math.bang.MagicLayer): boolean[];
        /** Returns whether every digit vector of the number lies in the design. */
        export function member(code: string | number | bigint, number: string | number | bigint, dimension: number, base: number): boolean;
        /** Returns the first members of a design in ascending order. */
        export function members(code: string | number | bigint, dimension: number, base: number, count: number): string[];
        /** Returns the diagonal slice profile of one design pressed to a fractal level. */
        export function profile(code: string | number | bigint, dimension: number, base: number, level: number): string[];
        /** Returns the corner-usage mask of a number, one bit per digit vector its expansion uses. */
        export function usage(number: string | number | bigint, dimension: number, base: number): string;
        /** Counts the members of a magic word from its layer fills, without enumeration. */
        export function word_count(layers: math.bang.MagicLayer[]): string;
        /** Returns whether the number lies in the magic word's composed design. */
        export function word_member(layers: math.bang.MagicLayer[], number: string | number | bigint): boolean;
        /** Enumerates every member of the magic word in ascending order. */
        export function word_members(layers: math.bang.MagicLayer[]): string[];
        /** Returns the diagonal slice profile of a magic word by the substitution product. */
        export function word_profile(layers: math.bang.MagicLayer[]): string[];
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
    export namespace roulette {
        /** Counts the nodes of the roulette the pencils draw on the track: `mrlyrs::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders. */
        export function nodes(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], samples: number, tol: number): math.roulette.Nodes;
        /** Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one. */
        export function side(a: ArrayLike<number>, b: ArrayLike<number>, c: ArrayLike<number>): number;
        /** One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrlyrs::math::spirograph::distinct` and `mrlyrs::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve. */
        export function spread(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], exact: boolean): math.spirograph.Pencil[];
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
            static default(): math.roulette.Nodes;
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
    export namespace rules {
        /** Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list. */
        export function render(filled: ArrayLike<number>[], number: number, dimension: number, base: number): Tensor;
        /** Returns every axis but the free one. */
        export function tree_axes(dimension: number, free_axis: number): Uint32Array;
        /** The default residue base. */
        export function BASE(): number;
    }
    export namespace shape {
        /** Tallies the design's cells and filled cells per region of the shape. */
        export function census(shape: math.shape.Shape, types: Tensor): math.shape.ShapeCensus;
        /** Places one lattice cell relative to the shape, exactly, with no floats. */
        export function classify(shape: math.shape.Shape, side: number, index: ArrayLike<number>): math.shape.Region;
        /** Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti. */
        export function crop(types: Tensor, shape: math.shape.Shape, keep_cut: boolean): Tensor;
        /** Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order. */
        export function crossing_shell(radius: number | bigint, number: number | bigint, level: number): [bigint, bigint][];
        /** Builds the whole crossing tree of one radius, pruned by the seats the design keeps. */
        export function crossing_tree(radius: number | bigint, number: number | bigint, keep: boolean[]): math.shape.Shell;
        /** Builds a named shape of the dimension, centered at one half on every axis. */
        export function named(name: string, dimension: number, radius: math.shape.Frac): math.shape.Shape;
        /** Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic. */
        export function radial_census(types: Tensor, centre: ArrayLike<number | bigint>, r_max: number | bigint): math.shape.RadialCounts[];
        /** Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes. */
        export function refine(types: Tensor, shape: math.shape.Shape, base: number, extra: number, keep_cut: boolean): Tensor;
        /** Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side. */
        export function regions(shape: math.shape.Shape, dims: ArrayLike<number>): Tensor;
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
            minus(other: math.shape.Frac): math.shape.Frac;
            /** Returns the exact sum. */
            plus(other: math.shape.Frac): math.shape.Frac;
            /** Returns the exact product. */
            times(other: math.shape.Frac): math.shape.Frac;
            /** Wraps an integer as a fraction over one. */
            static whole(num: number | bigint): math.shape.Frac;
        }
        /** A closed half-space: the points x with normal dot x at most offset. */
        export interface Half {
            /** The integer outward normal. */
            normal: number[];
            /** The rational offset the linear form stays under. */
            offset: math.shape.FracData;
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
            flip(region: math.shape.Region): math.shape.Region;
        };
        /** An exact region of the unit box, scaled onto the lattice by the side. */
        export type Shape = { Ball: { center: math.shape.FracData[]; radius: math.shape.FracData } } | { Polytope: { walls: math.shape.Half[] } } | { Anti: string };
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
            levels: math.shape.ShellBox[][];
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
    export namespace six {
        /** Swaps every fill triangle for a void and back. */
        export function anti(cell: Cell6d): Cell6d;
        /** Maps each triangle to one at or above the threshold, zero below. */
        export function binarize(cell: Cell6d, threshold: number): Cell6d;
        /** Binarizes the triangles at the threshold Otsu's method picks. */
        export function binarize_otsu(cell: Cell6d): Cell6d;
        /** Builds a hexagon of the given radius, fill inside and void outside. */
        export function blank(radius: number, orient: math.six.Orientation, fill: number, void_: number): Cell;
        /** Rounds each triangle to the mean of its masked neighborhood, wrapping on request. */
        export function blur(cell: Cell6d, mask: Tensor, wrap: boolean): Cell6d;
        /** Tallies a cell's triangles, corners and edges, counting the backdrop only on request. */
        export function census(cell: Cell6d, include_grid: boolean): math.six.Census;
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
        export function fills_only(cell: Cell6d): math.six.Census;
        /** Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood. */
        export function framed(cell: Cell6d): Cell6d;
        /** Parses a cell from JSON, defaulting any missing projection metadata. */
        export function from_json(text: string): Cell6d;
        /** Returns the triangle count of the fill's largest connected piece. */
        export function giant(cell: Cell6d): number;
        /** Returns the largest connected piece of the filled-triangle network as a network of its own. */
        export function giant_network(cell: Cell6d): math.graph.Network;
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
        export function new_(cell: Cell, projection: math.six.Projection, orientation: math.six.Orientation, start: number): Cell6d;
        /** The three corners of the north-pointing triangle at the grid column and row. */
        export function north(x: number | bigint, y: number | bigint): [bigint, bigint][];
        /** Returns the orientation a hexagon's width and height imply. */
        export function orientation(width: number, height: number): math.six.Orientation;
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
        export function radial_mask(radius: number, orient: math.six.Orientation): Tensor;
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
        export function slice_core_graph(cell: Cell6d): math.graph.Network;
        /** Builds the network of fill and void triangles joined by shared edges. */
        export function slice_dual_graph(cell: Cell6d): math.graph.Network;
        /** Builds the corner-and-edge network of the triangles matching the value, or of every fill and void. */
        export function slice_edge_graph(cell: Cell6d, value?: number): math.graph.Network;
        /** Builds the network of void triangles joined by shared edges, the pore network of the slice. */
        export function slice_tunnel_graph(cell: Cell6d): math.graph.Network;
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
            export function arm_law(number: number): math.six.star.Share;
            /** The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`. */
            export function chi8(number: number): bigint;
            /** The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`. */
            export function constant(): number;
            /** The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`. */
            export function decay(excesses: ArrayLike<number>, layers: number): math.six.star.Decay;
            /** The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`. */
            export function width_law(half: number): number;
            /** The three classes of layer count the `1/L^2` term of the decay reads. */
            export type Branch = "Zero" | "Two" | "Odd";
            export const Branch: {
                /** The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`. */
                constant(branch: math.six.star.Branch): number;
                /** The name of the branch. */
                name(branch: math.six.star.Branch): string;
                /** The branch of a layer count. */
                of(layers: number): math.six.star.Branch;
                /** The exact `1/L^2` coefficient at even `L`, absent at odd `L`. */
                residual(branch: math.six.star.Branch): number | undefined;
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
                arm(number: number, half: number): math.six.star.Share;
                /** The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`. */
                cell(number: number, x: number | bigint, z: number | bigint): boolean | undefined;
                /** The per-layer excess of the star band over the hexagon across the first `L` odd layers. */
                excesses(layers: number, half: number): Float64Array;
                /** The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against. */
                hexagon(number: number): math.six.star.Share;
            }
        }
    }
    export namespace spectrum {
        /** Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size. */
        export function clusters(eigenvalues: ArrayLike<number>, tolerance: number): [number, number][];
        /** Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`. */
        export function laplacian(network: math.graph.Network, normalised: boolean): Float64Array[];
        /** Returns the ascending Laplacian spectrum of a network, combinatorial or normalised. */
        export function laplacian_spectrum(network: math.graph.Network, normalised: boolean): Float64Array;
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
    export namespace spin {
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
        export function radial(data: ArrayLike<number>, size: number, out: number, copies: number, step: number, blend: math.spin.Blend, samples: number): Float32Array;
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
            fold(blend: math.spin.Blend, values: ArrayLike<number>): number;
            /** Reads a blend by name: mean, sum, union, meet, parity or difference. */
            named(name: string): math.spin.Blend | undefined;
        };
    }
    export namespace spirograph {
        /** The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel. */
        export function cell(width: number, height: number, reach: number): number;
        /** The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area. */
        export function cover(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], exact: boolean, samples: number, side: number): math.spirograph.Cover;
        /** The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall. */
        export function disc(track: math.spirograph.Track, pencils: math.spirograph.Pencil[]): math.spirograph.Disc;
        /** How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter. */
        export function distinct(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], exact: boolean): number;
        /** The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further. */
        export function frame(track: math.spirograph.Track, pencils: math.spirograph.Pencil[]): Float64Array;
        /** The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach. */
        export function nodes(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], exact: boolean): bigint | undefined;
        /** Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded. */
        export function pencils(types: ArrayLike<number>, width: number, height: number, mode: string, reach: number, jitter: number, seed: number): math.spirograph.Pencil[];
        /** Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel. */
        export function point(track: math.spirograph.Track, pencil: math.spirograph.Pencil, s: number): [number, number];
        /** The wheel's centre after `s` of path length. */
        export function pose(track: math.spirograph.Track, s: number): [number, number];
        /** One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter. */
        export function representatives(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], exact: boolean): Uint32Array;
        /** Counts the pencils by kind. */
        export function seats(pencils: math.spirograph.Pencil[]): math.spirograph.Seats;
        /** The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close. */
        export function signed_area(track: math.spirograph.Track, pencil: math.spirograph.Pencil): number | undefined;
        /** Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y. */
        export function trace(track: math.spirograph.Track, pencils: math.spirograph.Pencil[], samples: number): Float32Array;
        /** Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps. */
        export function track(kind: string, ring: number, wheel: number, sides: number, laps: number): math.spirograph.Track;
        /** The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius. */
        export function turn(track: math.spirograph.Track, s: number): number;
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
            kind: math.spirograph.Kind;
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
            default(): math.spirograph.Seats;
        };
        /** A track and the path the wheel's centre takes along it: the wheel of radius `wheel` rolls without slipping, on the left of the track when `side` is minus one and on the right when it is plus one, and turns by `side` times the centre's path length over the wheel's radius. */
        export interface Track {
            /** The kind: `line`, `in`, `out`, `polyin` or `polyout`. */
            kind: string;
            /** The wheel's radius. */
            wheel: number;
            /** The pieces of the centre path, in order. */
            pieces: math.spirograph.Piece[];
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
    export namespace three {
        /** Builds the Menger sponge, filled where at most one coordinate is odd, at the given level. */
        export function carpet(number: number, level: number): Cell;
        /** Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading. */
        export function census(cell: Cell): math.three.Census;
        /** Extracts the network of filled sites joined to their axis neighbors. */
        export function core_graph(cell: Cell): math.graph.Network;
        /** Builds the cube the universe code names, deepened to the given fractal level. */
        export function create(code: string | number | bigint, number: number, level: number, base: number): Cell;
        /** Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples. */
        export function diagonal_slice(code: string | number | bigint, number: number, level: number, base: number, height: number): Uint32Array[];
        /** Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner. */
        export function diagonal_svg(code: string | number | bigint, number: number, level: number, base: number, heights: ArrayLike<number>, scale: number): string;
        /** Builds the dust cube, filled where every coordinate is even, at the given level. */
        export function dust(number: number, level: number): Cell;
        /** Extracts the network of corners and edges outlining every filled site. */
        export function edge_graph(cell: Cell): math.graph.Network;
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
        export function quads(cell: Cell): math.three.Quad[];
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
        export function tunnel_graph(cell: Cell): math.graph.Network;
        /** Builds the checkerboard cube, filled where all coordinate parities agree, at the given level. */
        export function void_(number: number, level: number): Cell;
        /** Returns the count of empty sites. */
        export function voids(cell: Cell): number;
        /** Returns the filled-site count, the cube's volume. */
        export function volume(cell: Cell): number;
        /** Returns the cell's edge-graph segments, scaled into the unit box. */
        export function wires(cell: Cell): math.three.Vec3[][];
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
            normal: math.three.Vec3Data;
            /** The four corners in winding order. */
            verts: math.three.Vec3Data[];
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
            cross(o: math.three.Vec3): math.three.Vec3;
            /** Returns the dot product of the two vectors. */
            dot(o: math.three.Vec3): number;
            /** Multiplies every component by the scalar. */
            scale(s: number): math.three.Vec3;
        }
    }
    export namespace tourbillon {
        /** The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle. */
        export function eyes(qmax: number): math.tourbillon.Eye[];
        /** Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer. */
        export function field(top: number, size: number, schedule: string, increment: number, set: string, weights: string, mode: string, blend: string, seed: number): Float32Array;
        /** The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle. */
        export function layers(top: number, schedule: string, increment: number, set: string, weights: string, seed: number): math.tourbillon.Layer[];
        /** The least whole number of increments that closes a quarter turn, none once the count passes the cap. */
        export function period(increment: number): number | undefined;
        /** The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one. */
        export function sharing(list: math.tourbillon.Layer[]): [number, number];
        /** Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site. */
        export function stack(list: math.tourbillon.Layer[], size: number, mode: string, blend: math.spin.Blend): Float32Array;
        /** Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites. */
        export function stats(field: ArrayLike<number>, size: number, top: number, schedule: string, increment: number, set: string, weights: string, blend: string, seed: number): math.tourbillon.Stats;
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
    export namespace two {
        /** Returns the payload bytes the cell's filled sites can hold, its length header paid for. */
        export function capacity(cell: Cell): number;
        /** Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level. */
        export function carpet(number: number, level: number): Cell;
        /** Takes the cell's full census in one reading. */
        export function census(cell: Cell): math.two.Census;
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
        export function png(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: math.two.Shape): Uint8Array;
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
        export function svg(cell: Cell, scale: number, outline: Color | undefined, width: number, shape: math.two.Shape): string;
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
}
export declare namespace num {
    export namespace apollonian {
        /** The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves. */
        export function form(u: ArrayLike<number | bigint>, v: ArrayLike<number | bigint>): string;
        /** The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing. */
        export function frame(p: num.apollonian.Packing): Float64Array;
        /** Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period. */
        export function grow(name: string, cap: number | bigint): num.apollonian.Packing;
        /** Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`. */
        export function is_ford(c: num.apollonian.Circle): boolean;
        /** Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`. */
        export function on_line(c: num.apollonian.Circle): boolean;
        /** Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root. */
        export function reflect(q: num.apollonian.Circle[], at: number): num.apollonian.Circle;
        /** The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures. */
        export function root(name: string): num.apollonian.Circle[];
        /** Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero. */
        export function shadow(p: num.apollonian.Packing, order: number): num.apollonian.Shadow;
        /** Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`. */
        export function sound(q: num.apollonian.Circle[]): boolean;
        /** The quadruple with the circle at the seat replaced by its reflection. */
        export function swap(q: num.apollonian.Circle[], at: number): num.apollonian.Circle[];
        /** The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip. */
        export function touches(p: num.apollonian.Packing): num.apollonian.Touch[];
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
            get k(): bigint;
            set k(value: number | bigint);
            /** The curvature times the centre's abscissa. */
            get x(): bigint;
            set x(value: number | bigint);
            /** The curvature times the centre's ordinate. */
            get y(): bigint;
            set y(value: number | bigint);
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
            root: num.apollonian.CircleData[];
            /** Whether the root carries a line, so the packing is the strip and the growth keeps one period. */
            strip: boolean;
            /** The curvature the growth stopped at. */
            cap: number;
            /** The circles the growth made, the root excluded, in curvature order. */
            circles: num.apollonian.CircleData[];
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
    export namespace automaton {
        /** The relative rounding allowance the double-precision matrix ladder charges against the scale it carries. */
        export function ROUNDING(): number;
        export type AutomatonData = Record<string, unknown>;
        /** A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from. */
        export class Automaton {
            /** Builds the ladder of a rule, choosing the peel depth. */
            constructor(rule: num.memory.Rule);
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
            cofactor(s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
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
            residue(w0: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
            /** Returns the rule. */
            rule(): num.memory.Rule;
            /** Returns the state count `q^(k-1)`. */
            states(): number;
            /** Builds the ladder at an explicit peel depth, at least the rule width and at least two. */
            static with_peel(rule: num.memory.Rule, peel: number): num.automaton.Automaton;
            /** Returns `zeta_W(s)` and the bound it is known to. */
            zeta(s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
        }
    }
    export namespace blend {
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
    export namespace boolean {
        /** Reports whether the packed function outputs one on exactly half of its inputs. */
        export function is_balanced(code: string | number | bigint, n: number): boolean;
        /** Returns how far the packed function sits from every affine function, zero when it is one. */
        export function nonlinearity(code: string | number | bigint, n: number): bigint;
        /** Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche. */
        export function sac(code: string | number | bigint, n: number): number;
        /** Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code. */
        export function walsh_spectrum(code: string | number | bigint, n: number): BigInt64Array;
    }
    export namespace design {
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
    export namespace factor {
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
    export namespace fft {
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
    export namespace gauss {
        /** Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates. */
        export function classes(ring: num.gauss.Ring, bound: number | bigint): [bigint, bigint][];
        /** Returns the norm from one through the limit with the most points and that count, the earliest on a tie. */
        export function peak(ring: num.gauss.Ring, limit: number): [number, number];
        /** Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice. */
        export function shells(ring: num.gauss.Ring, limit: number): Uint32Array;
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
            prime(class_: num.gauss.Class): boolean;
            /** Returns the class as a word. */
            word(class_: num.gauss.Class): string;
        };
        /** The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice. */
        export type Ring = "Gaussian" | "Eisenstein";
        export const Ring: {
            /** Returns the unit multiples of a point, the point first, turning anticlockwise. */
            associates(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint][];
            /** Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin. */
            canon(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
            /** Returns the conjugate: the mirror image in the real axis. */
            conjugate(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
            /** Returns the count of points within the reach: the square or the hexagon. */
            count(ring: num.gauss.Ring, radius: number | bigint): number;
            /** Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`. */
            div_rem(ring: num.gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [[bigint, bigint], [bigint, bigint]];
            /** Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise. */
            fate(ring: num.gauss.Ring, n: number | bigint): num.gauss.Class;
            /** Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins. */
            gaussian_gcd(ring: num.gauss.Ring, z: [number | bigint, number | bigint], w: [number | bigint, number | bigint]): [bigint, bigint];
            /** Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3. */
            inert(ring: num.gauss.Ring, p: number | bigint): boolean;
            /** Returns the product of two points. */
            mul(ring: num.gauss.Ring, arg1: [number | bigint, number | bigint], arg2: [number | bigint, number | bigint]): [bigint, bigint];
            /** Reads a ring from its name. */
            named(name: string): num.gauss.Ring | undefined;
            /** Returns the point nearest a place in the plane. */
            nearest(ring: num.gauss.Ring, x: number, y: number): [bigint, bigint];
            /** Returns the norm of a point: its squared length. */
            norm(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): bigint;
            /** Returns the place of a point in the plane, x right and y up, one unit between neighbours. */
            place(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): [number, number];
            /** Returns the one rational prime that ramifies: 2 or 3. */
            ramified(ring: num.gauss.Ring): bigint;
            /** Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance. */
            reach(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): bigint;
            /** Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12. */
            symmetry(ring: num.gauss.Ring): number;
            /** Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's. */
            top(ring: num.gauss.Ring, radius: number | bigint): bigint;
            /** Returns the point turned anticlockwise by one unit: a quarter turn or a sixth. */
            turn(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): [bigint, bigint];
            /** Returns the count of units: 4 or 6. */
            units(ring: num.gauss.Ring): number;
            /** Returns the whole number an associate of the point lies on, when one lies on the positive real axis. */
            whole(ring: num.gauss.Ring, a: number | bigint, b: number | bigint): bigint | undefined;
        };
        export type WindowData = Record<string, unknown>;
        /** The symmetric window of one ring: every point within a reach, with the norms sieved once. */
        export class Window {
            /** Opens the window of a ring out to a reach, sieving every norm inside it. */
            constructor(ring: num.gauss.Ring, radius: number | bigint);
            free(): void;
            /** Reads the Window from its plain data. */
            static from(data: WindowData): Window;
            /** Writes the Window as plain data. */
            toJSON(): WindowData;
            /** Counts every class inside. */
            census(): num.gauss.Census;
            /** Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime. */
            class(a: number | bigint, b: number | bigint): num.gauss.Class;
            /** Returns whether a point lies inside. */
            holds(a: number | bigint, b: number | bigint): boolean;
            /** Lists every point inside, row by row from the bottom left of the bounding square. */
            points(): [bigint, bigint][];
            /** Returns the reach. */
            radius(): bigint;
            /** Returns the ring. */
            ring(): num.gauss.Ring;
        }
    }
    export namespace ladder {
        /** Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to. */
        export function cofactor(design: num.ladder.Design, s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
        /** Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to. */
        export function residue(design: num.ladder.Design, m: number, j: number | bigint, tolerance: number): [num.zeta.Complex, number];
        /** Returns `zeta_F(s)` and the bound it is known to. */
        export function zeta(design: num.ladder.Design, s: num.zeta.Complex, tolerance: number): [num.zeta.Complex, number];
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
            pole(m: number, j: number | bigint): num.zeta.Complex;
            /** Builds a design at an explicit peel depth, at least two. */
            static with_peel(base: number | bigint, digits: ArrayLike<number | bigint>, peel: number): num.ladder.Design;
        }
    }
    export namespace lattice {
        /** Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one. */
        export function coprime_pairs(n: number): bigint;
        /** Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending. */
        export function farey(order: number): num.lattice.Node[];
        /** Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes. */
        export function grid(n: number): num.lattice.Node2d[];
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
            x: num.lattice.Node;
            /** The vertical node. */
            y: num.lattice.Node;
            /** The product of the two axis brightnesses. */
            brightness: number;
        }
    }
    export namespace memory {
        /** Returns the count of allowed windows `card W`, the bits the code sets inside its window range. */
        export function allowed_windows(rule: num.memory.Rule): number;
        /** Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first. */
        export function cells(rule: num.memory.Rule, level: number): BigUint64Array;
        /** Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`. */
        export function counts(rule: num.memory.Rule, levels: number): BigUint64Array;
        /** Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count. */
        export function exponent(rule: num.memory.Rule): number;
        /** Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory. */
        export function kappa(rule: num.memory.Rule): number;
        /** Returns the Perron root of the transfer matrix, the count's growth per level. */
        export function perron(rule: num.memory.Rule): number;
        /** Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed. */
        export function transfer(rule: num.memory.Rule): BigUint64Array[];
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
            get dimension(): number;
            set dimension(value: number);
            /** The window width `k`, at least one. */
            get width(): number;
            set width(value: number);
            /** The window code, bit `w` set when window `w` is allowed. */
            get code(): bigint;
            set code(value: number | bigint);
            /** Returns whether a word, coarsest digit first, is accepted. */
            accepts(word: ArrayLike<number>): boolean;
            /** Returns whether the window is allowed, and false for any window out of range. */
            allowed(window: number): boolean;
            /** Returns the letters that stand in at least one allowed window. */
            alphabet(): Uint32Array;
            /** Returns the count of rules of this shape, `2^(2^(k D))`. */
            codes(): string;
            /** Returns the rule that allows every window. */
            static full(dimension: number, width: number): num.memory.Rule;
            /** Returns the letter count `2^D`, the digit vectors of the cube's corners. */
            letters(): number;
            /** Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on. */
            states(): number;
            /** Returns the window count `2^(k D)`. */
            windows(): number;
        }
    }
    export namespace morse {
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
        export function fold(grid: ArrayLike<number>, side: number, number: number): num.morse.Fold;
        /** Returns the Thue-Morse letter at the place, the parity of its binary digit sum. */
        export function letter(place: number | bigint): number;
        /** Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one. */
        export function lift(kind: num.morse.Lift, side: number): Uint8Array;
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
        export function LIFTS(): num.morse.Lift[];
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
            all(): num.morse.Lift[];
            /** Returns the sign at a site, zero for plus one and one for minus one. */
            at(lift: num.morse.Lift, i: number | bigint, j: number | bigint): number;
            /** Returns the lift's formula, written the way the page prints it. */
            formula(lift: num.morse.Lift): string;
        };
    }
    export namespace prime {
        /** Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last. */
        export function chart(top: number, bins: number): num.prime.Reading[];
        /** Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag. */
        export function flags(limit: number): boolean[];
        /** Returns the count of unordered pairs of primes summing to the number, zero below four. */
        export function goldbach(number: number): number;
        /** Returns the count of prime pairs at every even number from four up to the top, one entry per even number. */
        export function goldbach_record(top: number): Uint32Array;
        /** Returns whether the number is prime, by trial division on the six-step wheel. */
        export function is_prime(number: number): boolean;
        /** Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization. */
        export function pile(number: number | bigint): num.prime.Pile;
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
        export function study(limit: number): num.prime.Prime[];
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
    export namespace radix {
        /** Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`. */
        export function flowsnake(): num.radix.Radix;
        /** Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`. */
        export function gasket(): num.radix.Radix;
        /** Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`. */
        export function koch(): num.radix.Radix;
        /** Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`. */
        export function terdragon(): num.radix.Radix;
        /** Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`. */
        export function tile(m: number | bigint, code: string | number | bigint): num.radix.Radix;
        /** Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`. */
        export function twindragon(): num.radix.Radix;
        export type BaseData = Record<string, unknown>;
        /** The base of a radix design: a ring and an element of norm at least two, the scale every word is read against. */
        export class Base {
            /** Fixes a base in a ring. */
            constructor(ring: num.gauss.Ring, value: [number | bigint, number | bigint]);
            free(): void;
            /** Reads the Base from its plain data. */
            static from(data: BaseData): Base;
            /** Writes the Base as plain data. */
            toJSON(): BaseData;
            /** Returns the index in the canonical residue system of the class of a point. */
            class(z: [number | bigint, number | bigint]): number;
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
            ring(): num.gauss.Ring;
            /** Returns the base element. */
            value(): [bigint, bigint];
        }
        export type RadixData = Record<string, unknown>;
        /** A radix design: a digit set inside one ring, placed by a base with a unit twist per digit. */
        export class Radix {
            /** Builds a design from a base, a digit list and a unit twist per digit. */
            constructor(base: num.radix.Base, digits: ([number | bigint, number | bigint])[], twists: ([number | bigint, number | bigint])[]);
            free(): void;
            /** Reads the Radix from its plain data. */
            static from(data: RadixData): Radix;
            /** Writes the Radix as plain data. */
            toJSON(): RadixData;
            /** Returns the base. */
            base(): num.radix.Base;
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
            static from_code(base: num.radix.Base, code: string | number | bigint): num.radix.Radix;
            /** Returns the level-`L` points in the plane, the scaled words divided by `b^L`. */
            plane(level: number): [number, number][];
            /** Returns the ring. */
            ring(): num.gauss.Ring;
            /** Returns the digit count `|F|`. */
            size(): number;
            /** Returns the twists. */
            twists(): [bigint, bigint][];
            /** Returns the design with the twists named by their index in the unit list, the units in turning order from one. */
            with_twists(units: ArrayLike<number>): num.radix.Radix;
            /** Returns the level-`L` points in exact ring coordinates scaled by `b^L`. */
            words(level: number): [bigint, bigint][];
        }
    }
    export namespace series {
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
    export namespace sieve {
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
    export namespace spiral {
        /** Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak. */
        export function diagonal(lattice: num.spiral.Lattice, side: number, a: number | bigint, b: number | bigint, c: number | bigint): num.spiral.Diagonal;
        /** Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself. */
        export function level_of(n: number | bigint, base: number | bigint): number;
        /** Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero. */
        export function marks(mark: num.spiral.Mark, limit: number): Int8Array;
        /** Winds one to the top on the square spiral and lays a square tile on every cell, the snail. */
        export function snail(base: number | bigint, top: number | bigint, growth: num.spiral.Growth): num.spiral.Snail;
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
            all(): num.spiral.Growth[];
        };
        /** The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right. */
        export type Lattice = "Square" | "Hex";
        export const Lattice: {
            /** Returns every Lattice in canonical order. */
            all(): num.spiral.Lattice[];
            /** Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across. */
            count(lattice: num.spiral.Lattice, side: number): number;
            /** Returns the number at a cell, one at the origin. */
            n(lattice: num.spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
            /** Returns the outermost ring of a sheet the odd side wide, half the side rounded down. */
            radius(lattice: num.spiral.Lattice, side: number): number;
            /** Returns the ring a number sits on, zero for one. */
            ring(lattice: num.spiral.Lattice, n: number | bigint): bigint;
            /** Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon. */
            ring_of(lattice: num.spiral.Lattice, x: number | bigint, y: number | bigint): bigint;
            /** Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon. */
            xy(lattice: num.spiral.Lattice, n: number | bigint): [bigint, bigint];
        };
        /** What a cell is painted for. */
        export type Mark = "Prime" | "Twin" | "Squarefree" | "Mobius";
        export const Mark: {
            /** Returns every Mark in canonical order. */
            all(): num.spiral.Mark[];
        };
        /** The snail: every tile of the winding, the tallies, the area drawn and the box filled. */
        export interface Snail {
            /** The base every tile side is a power of. */
            base: number;
            /** Every tile, in the order one, two, three and on. */
            tiles: num.spiral.Tile[];
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
    export namespace zeta {
        /** The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2. */
        export function bump(u: number): number;
        /** Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step. */
        export function corrections(p: number): Float64Array;
        /** Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points. */
        export function kernel(p: number): number;
        /** Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule. */
        export function mellin(s: num.zeta.Complex): num.zeta.Complex;
        /** Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two. */
        export function novelty_main(): number;
        /** Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict. */
        export function novelty_wave(gammas: ArrayLike<number>, coef: num.zeta.Complex[], log_y: number): number;
        /** Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two. */
        export function psi_formula(x: number, gammas: ArrayLike<number>): number;
        /** Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each. */
        export function psi_stair(x: number): Float64Array;
        /** Returns a positive real base raised to a complex exponent. */
        export function raise(base: number, exponent: num.zeta.Complex): num.zeta.Complex;
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
            get re(): number;
            set re(value: number);
            /** The imaginary part. */
            get im(): number;
            set im(value: number);
            /** Returns the modulus. */
            abs(): number;
            /** Returns the principal argument. */
            arg(): number;
            /** Returns the default Complex. */
            static default(): num.zeta.Complex;
            /** Returns the exponential. */
            exp(): num.zeta.Complex;
            /** Returns the principal logarithm. */
            ln(): num.zeta.Complex;
            /** Returns a unit complex number at the given angle. */
            static turn(angle: number): num.zeta.Complex;
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
            /** Returns the default Line. */
            static default(): num.zeta.Line;
            /** Returns Z(t) from the Euler-Maclaurin value turned onto the real axis. */
            exact(t: number): number;
            /** Returns the n-th Gram point, where theta is n pi, by Newton from the right. */
            gram(n: number | bigint): number;
            /** Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections. */
            maclaurin(t: number): num.zeta.Complex;
            /** Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump. */
            novelty_coefficients(gammas: ArrayLike<number>): num.zeta.Complex[];
            /** Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s. */
            pair(s: num.zeta.Complex): [num.zeta.Complex, num.zeta.Complex];
            /** Returns zeta on the line and Z(t) together, from the engine that serves the t. */
            point(t: number): [num.zeta.Complex, number];
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
}
