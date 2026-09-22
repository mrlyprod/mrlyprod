export { default, initSync } from "./pkg/font/mrlyjs_font.js";

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
/** Builds every glyph in font order: uppers, lowers, digits, extras, specials. */
export function all(): Glyph[];
/** Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster. */
export function animate(text: string, pad: number): Anim;
/** Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any. */
export function cycle(write: Anim, merge: ArrayLike<number>[], hold: number): Anim;
/** Builds the ten digit glyphs. */
export function digits(): Glyph[];
/** Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck. */
export function draft(rows: string[]): [number, number][][];
/** Builds the punctuation, symbol and arrow glyphs. */
export function extras(): Glyph[];
/** Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank. */
export function floor(rows: string[]): number;
/** Returns an owned copy of the character's glyph, or None outside the font. */
export function glyph(c: string): Glyph | undefined;
/** Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form. */
export function lower(rows: string[]): string[];
/** Builds the twenty-six lowercase glyphs by rounding the uppers' corners. */
export function lowers(): Glyph[];
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
export function specials(): Glyph[];
/** Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font. */
export function strokes(c: string): [number, number][][];
/** Returns every character in the font, in font order. */
export function supported(): string[];
/** Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has. */
export function trim(rows: string[]): string[];
/** Builds the twenty-six uppercase glyphs. */
export function uppers(): Glyph[];
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
    readonly char: string;
    /** The bitmap rows of '0' and '1' characters. */
    readonly rows: string[];
    /** Returns the number of rows. */
    height(): number;
    /** Returns the cell width of the first row, or 0 for an empty glyph. */
    width(): number;
}
export declare namespace paths {
    /** Returns the character's hand-penned strokes from the pen tables, or None outside the font. */
    export function penned(c: string): [number, number][][] | undefined;
}
export declare namespace pens {
    /** Returns every pen in font order: uppers, lowers, digits, extras, specials. */
    export function all(): [string, string[]][];
}
