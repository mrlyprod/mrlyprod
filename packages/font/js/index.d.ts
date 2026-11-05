export { default, initSync } from "./pkg/mrly_font_js.js";

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
export function all(): Glyph[];
export function animate(text: string, pad: number): Anim;
export function cycle(write: Anim, merge: ArrayLike<number>[], hold: number): Anim;
export function digits(): Glyph[];
export function draft(rows: string[]): [number, number][][];
export function extras(): Glyph[];
export function floor(rows: string[]): number;
export function glyph(c: string): Glyph | undefined;
export function lower(rows: string[]): string[];
export function lowers(): Glyph[];
export function map(): Record<string, string[]>;
export function merge(text: string, pad: number): Uint32Array[];
export function name_of(c: string): string;
export function path(c: string): [number, number][];
export function raster(text: string): Uint8Array[];
export function specials(): Glyph[];
export function strokes(c: string): [number, number][][];
export function supported(): string[];
export function trim(rows: string[]): string[];
export function uppers(): Glyph[];
export function FPS(): number;
export function HOLD(): number;
export interface Anim {
    rows: number;
    cols: number;
    fps: number;
    frames: number[][];
}
export interface GlyphData {
    char: string;
    rows: string[];
}
export class Glyph {
    constructor(char: string, rows: string[]);
    free(): void;
    static from(data: GlyphData): Glyph;
    toJSON(): GlyphData;
    get char(): string;
    set char(value: string);
    get rows(): string[];
    set rows(value: string[]);
    height(): number;
    width(): number;
}
export declare namespace paths {
    export function penned(c: string): [number, number][][] | undefined;
}
export declare namespace pens {
    export function all(): [string, string[]][];
}
