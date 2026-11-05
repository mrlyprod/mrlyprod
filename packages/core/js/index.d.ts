export { default, initSync } from "./pkg/mrly_core_js.js";

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
export function hex_fit(pixels: ArrayLike<number>[], width: number, height: number, vertical: boolean, filter: Filter): [number, number, Uint8Array[]];
export function hex_size(width: number, height: number, vertical: boolean): [number, number];
export function resample(pixels: ArrayLike<number>[], width: number, height: number, out_w: number, out_h: number, filter: Filter): Uint8Array[];
export function HEX_RATIO(): number;
export type Colorizer = { Bins: { background: ColorData; ramp: ColorData[] } };
export const Colorizer: {
    default(): Colorizer;
    diverge(): Colorizer;
    fire(): Colorizer;
    gradient_bins(background: Color, colors: Color[], shades: number): Colorizer;
    heat(): Colorizer;
};
export type Dtype = "U8" | "U16" | "U32" | "I32";
export const Dtype: {
    max(dtype: Dtype): bigint;
};
export type Filter = "Nearest" | "Linear" | "Box";
export type Mode = "Type" | "Tag" | "Index" | "Enumerate" | "Random" | "Row" | "Column" | "Depth";
export declare namespace cell {
    export function anti(cell: Cell): Cell;
    export function binarize(cell: Cell, threshold: number): Cell;
    export function binarize_otsu(cell: Cell): Cell;
    export function blur(cell: Cell, mask: Tensor, wrap: boolean): Cell;
    export function color_at(cell: Cell, flat: number): Uint8Array;
    export function combine(cell: Cell, other: Cell): Cell;
    export function fractal(cell: Cell, level: number): Cell;
    export function invert(cell: Cell): Cell;
    export function layers(cell: Cell, dtype: Dtype): Cell;
    export function magic(cells: Cell[]): Cell;
    export function mapping(): Record<string, Color[]>;
    export function merge(cells: Cell[], reps: ArrayLike<number>): Cell;
    export function moore(dimension: number): Tensor;
    export function mosaic(mask: Tensor, cells: Cell[]): Cell;
    export function neighbors(cell: Cell, mask: Tensor, target: number, wrap: boolean, dtype: Dtype): Cell;
    export function new_(types: Tensor): Cell;
    export function pad(cell: Cell, count: number, value: number): Cell;
    export function paint(cell: Cell, mapping: Record<string, Color[]>, mode: Mode, rng?: Rng): Cell;
    export function perforate(cell: Cell, mask: Tensor, value: number): Cell;
    export function remap(cell: Cell, map: ArrayLike<number>, shape: ArrayLike<number>): Cell;
    export function rgba(cell: Cell): Uint8Array;
    export function rot90_map(shape: ArrayLike<number>, k: number, axes: [number, number]): Uint32Array;
    export function rotate(cell: Cell, k: number, axes: [number, number]): Cell;
    export function shape(cell: Cell): Uint32Array;
    export function size(cell: Cell): number;
    export function tile(cell: Cell, reps: ArrayLike<number>): Cell;
    export function tile_map(shape: ArrayLike<number>, reps: ArrayLike<number>): Uint32Array;
}
export declare namespace colors {
    export function alpha(color: Color, level: number): Color;
    export function board(dark: boolean): Uint8Array;
    export function css(color: Color): string;
    export function from_hex(hex: string): Color;
    export function gradient(colors: Color[], steps: number): Color[];
    export function ink(dark: boolean): Uint8Array;
    export function invert(color: Color): Color;
    export function lightness(color: Color, level: number): Color;
    export function luma_types(pixels: ArrayLike<number>[], width: number, height: number, level: number): Tensor;
    export function mix(color_1: Color, color_2: Color, ratio: number): Color;
    export function named(name: string): Color;
    export function random(alpha: boolean, rng: Rng): Color;
    export function rgb(r: number, g: number, b: number): Color;
    export function rgba(r: number, g: number, b: number, a: number): Color;
    export function shades(hue: Color): Color[];
    export function snap(pixels: ArrayLike<number>[], palette: Color[]): Uint8Array[];
    export function to_hex(color: Color): string;
    export function ALPHA(): Color;
    export function BLACK(): Color;
    export function BLUE(): Color;
    export function BROWN(): Color;
    export function CYAN(): Color;
    export function DARK(): colors.Theme;
    export function GRAY(): Color;
    export function GREEN(): Color;
    export function INDIGO(): Color;
    export function LIGHT(): colors.Theme;
    export function MINT(): Color;
    export function NAMES(): string[];
    export function ORANGE(): Color;
    export function PALETTE(): Color[];
    export function PINK(): Color;
    export function PURPLE(): Color;
    export function RED(): Color;
    export function TEAL(): Color;
    export function WHITE(): Color;
    export function YELLOW(): Color;
    export interface ThemeData {
        ground: ColorData;
        bg: ColorData;
        panel: ColorData;
        deep: ColorData;
        line: ColorData;
        fg: ColorData;
        dim: ColorData;
        accent: ColorData;
        on_accent: ColorData;
        red: ColorData;
        orange: ColorData;
        yellow: ColorData;
        green: ColorData;
        mint: ColorData;
        teal: ColorData;
        cyan: ColorData;
        blue: ColorData;
        indigo: ColorData;
        purple: ColorData;
        pink: ColorData;
        brown: ColorData;
        gray: ColorData;
    }
    export class Theme {
        private constructor();
        free(): void;
        static from(data: ThemeData): Theme;
        toJSON(): ThemeData;
        get ground(): Color;
        set ground(value: Color);
        get bg(): Color;
        set bg(value: Color);
        get panel(): Color;
        set panel(value: Color);
        get deep(): Color;
        set deep(value: Color);
        get line(): Color;
        set line(value: Color);
        get fg(): Color;
        set fg(value: Color);
        get dim(): Color;
        set dim(value: Color);
        get accent(): Color;
        set accent(value: Color);
        get on_accent(): Color;
        set on_accent(value: Color);
        get red(): Color;
        set red(value: Color);
        get orange(): Color;
        set orange(value: Color);
        get yellow(): Color;
        set yellow(value: Color);
        get green(): Color;
        set green(value: Color);
        get mint(): Color;
        set mint(value: Color);
        get teal(): Color;
        set teal(value: Color);
        get cyan(): Color;
        set cyan(value: Color);
        get blue(): Color;
        set blue(value: Color);
        get indigo(): Color;
        set indigo(value: Color);
        get purple(): Color;
        set purple(value: Color);
        get pink(): Color;
        set pink(value: Color);
        get brown(): Color;
        set brown(value: Color);
        get gray(): Color;
        set gray(value: Color);
        hues(): Color[];
        inks(): Color[];
    }
}
export declare namespace error {
    export function parse(text: string): any;
}
export declare namespace image {
    export function blur(pixels: ArrayLike<number>[], width: number, height: number, radius: number): Uint8Array[];
    export function new_(width: number, height: number, colors: ArrayLike<number>[]): Image;
    export function resample(image: Image, width: number, height: number, filter: Filter): Image;
    export function scale(image: Image, scale: number): Image;
}
export declare namespace name {
    export namespace sha {
        export function hex(bytes: ArrayLike<number>): string;
        export function sha256(bytes: ArrayLike<number>): Uint8Array;
        export function short(bytes: ArrayLike<number>): string;
    }
}
export declare namespace paint {
    export function apply(paint: paint.Paint, cell: Cell, rng: Rng): void;
    export function coat(cell: Cell, paint: paint.Paint, mask: Tensor | undefined, rng: Rng): void;
    export function paint(cell: Cell, config: paint.Config, mask: Tensor | undefined, rng: Rng): paint.Paint;
    export function prime(paint: paint.Paint, cell: Cell, mask: Tensor | undefined, rng: Rng): paint.Paint;
    export function random_edition(editions: paint.Edition[] | undefined, rng: Rng): paint.Edition;
    export function reroll(paint: paint.Paint, rng: Rng): paint.Paint;
    export function setup(paint: paint.Paint, config: paint.Config, rng: Rng): paint.Paint;
    export function tag(cell: Cell, edition: paint.Edition, target: paint.Target, mask?: Tensor): number;
    export interface Config {
        editions?: paint.Edition[];
        primaries?: paint.Ink[];
        target?: paint.Target;
    }
    export const Config: {
        default(): paint.Config;
    };
    export type Edition = "Simple" | "Index" | "Layers" | "Neighbors" | "Rows" | "Columns" | "Random";
    export const Edition: {
        all(): paint.Edition[];
        mode(edition: paint.Edition): Mode | undefined;
    };
    export type Ink = "Black" | "White" | "Red" | "Orange" | "Yellow" | "Green" | "Mint" | "Teal" | "Cyan" | "Blue" | "Indigo" | "Purple" | "Pink" | "Brown" | "Gray";
    export const Ink: {
        all(): paint.Ink[];
        color(ink: paint.Ink): Color;
    };
    export interface PaintData {
        edition: paint.Edition;
        scheme: paint.Scheme;
        target: paint.Target;
        primary: paint.Ink;
        secondary: paint.Ink[];
        shades: number[];
    }
    export class Paint {
        constructor(edition: paint.Edition);
        free(): void;
        static from(data: PaintData): Paint;
        toJSON(): PaintData;
        get edition(): paint.Edition;
        set edition(value: paint.Edition);
        get scheme(): paint.Scheme;
        set scheme(value: paint.Scheme);
        get target(): paint.Target;
        set target(value: paint.Target);
        get primary(): paint.Ink;
        set primary(value: paint.Ink);
        get secondary(): paint.Ink[];
        set secondary(value: paint.Ink[]);
        get shades(): Uint32Array;
        set shades(value: ArrayLike<number>);
        is_simple(): boolean;
    }
    export type Scheme = "Multicolor" | "Multitone";
    export const Scheme: {
        all(): paint.Scheme[];
    };
    export type Target = "Fill" | "Void";
    export const Target: {
        all(): paint.Target[];
    };
}
export declare namespace ramp {
    export function color(colorizer: Colorizer, value: number, max: number): Color;
    export function colors(colorizer: Colorizer, values: ArrayLike<number>, max: number): Uint8Array[];
}
export declare namespace rng {
    export function below(rng: Rng, n: number): number;
    export function boolean(rng: Rng): boolean;
    export function chance(rng: Rng, p: number): boolean;
    export function new_(seed: number | bigint): Rng;
    export function range(rng: Rng, lo: number | bigint, hi: number | bigint): bigint;
    export function sample_indices(rng: Rng, length: number, amount: number): Uint32Array;
    export function unit(rng: Rng): number;
}
export declare namespace tensor {
    export function at(tensor: Tensor, flat: number): bigint;
    export function binarize(tensor: Tensor, threshold: number): Tensor;
    export function binarize_otsu(tensor: Tensor): Tensor;
    export function blur(tensor: Tensor, mask: Tensor, wrap: boolean): Tensor;
    export function bytes(tensor: Tensor): Uint8Array;
    export function count(tensor: Tensor, value: number): number;
    export function dtype(tensor: Tensor): Dtype;
    export function exposed(tensor: Tensor): string;
    export function filled(shape: ArrayLike<number>, value: number | bigint, dtype: Dtype): Tensor;
    export function flip(tensor: Tensor, axis: number): Tensor;
    export function fractal(tensor: Tensor, level: number): Tensor;
    export function full(shape: ArrayLike<number>, value: number): Tensor;
    export function get(tensor: Tensor, multi: ArrayLike<number>): number;
    export function i32(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    export function i32s(tensor: Tensor): Int32Array;
    export function index(tensor: Tensor, multi: ArrayLike<number>): number;
    export function invert(tensor: Tensor): Tensor;
    export function kron(tensor: Tensor, other: Tensor): Tensor;
    export function layers(tensor: Tensor, dtype: Dtype): Tensor;
    export function neighbors(tensor: Tensor, mask: Tensor, target: number, wrap: boolean, dtype: Dtype): Tensor;
    export function new_(shape: ArrayLike<number>): Tensor;
    export function of(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    export function otsu_threshold(tensor: Tensor): number;
    export function pad(tensor: Tensor, count: number, value: number): Tensor;
    export function perforate(tensor: Tensor, mask: Tensor, value: number): Tensor;
    export function put(tensor: Tensor, flat: number, value: number | bigint): void;
    export function rot90(tensor: Tensor, k: number, axes: [number, number]): Tensor;
    export function set(tensor: Tensor, multi: ArrayLike<number>, value: number): void;
    export function size(tensor: Tensor): number;
    export function slice(tensor: Tensor, axis: number, index: number): Tensor;
    export function sum(tensor: Tensor): bigint;
    export function tile(tensor: Tensor, reps: ArrayLike<number>): Tensor;
    export function transpose(tensor: Tensor, a: number, b: number): Tensor;
    export function typed(shape: ArrayLike<number>, dtype: Dtype): Tensor;
    export function u16(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    export function u16s(tensor: Tensor): Uint16Array;
    export function u32(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
    export function u32s(tensor: Tensor): Uint32Array;
    export function u8(data: ArrayLike<number>, shape: ArrayLike<number>): Tensor;
}
