export { default, initSync } from "./pkg/mrly_tile_js.js";

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
export function background(seed: number | bigint, width: number, height: number): Image;
export function classic_code(design: recipe.Design): string | undefined;
export function classic_code_nd(design: recipe.Design, dimension: number): string | undefined;
export function hex_key(length: number, rng: Rng): string;
export function random_design(rng: Rng): recipe.Design;
export function random_rotation(design: recipe.Design, rng: Rng): number;
export function tree_mask(n: number): Tensor;
export type Group = "General" | "Fractal" | "Magic" | "Special" | "Mosaic";
export const Group: {
    all(): Group[];
};
export type Parity = "Evens" | "Odds" | "Both";
export const Parity: {
    all(): Parity[];
    keep(parity: Parity, n: number): boolean;
};
export interface TileData {
    group: Group;
    factor: number;
    sources: recipe.Source[];
    numbers: number[];
    levels: number[];
    rotations: number[];
    invert: boolean;
    flip: boolean;
    width: number;
    height: number;
}
export class Tile {
    constructor(group: Group);
    free(): void;
    static from(data: TileData): Tile;
    toJSON(): TileData;
    get group(): Group;
    set group(value: Group);
    get factor(): number;
    set factor(value: number);
    get sources(): recipe.Source[];
    set sources(value: recipe.Source[]);
    get numbers(): Uint32Array;
    set numbers(value: ArrayLike<number>);
    get levels(): Uint32Array;
    set levels(value: ArrayLike<number>);
    get rotations(): Uint32Array;
    set rotations(value: ArrayLike<number>);
    get invert(): boolean;
    set invert(value: boolean);
    get flip(): boolean;
    set flip(value: boolean);
    get width(): number;
    set width(value: number);
    get height(): number;
    set height(value: number);
    check(): void;
    degenerate(): boolean;
    max_size(): number;
    resize(): void;
    size(width: number, height: number): Tile;
}
export declare namespace build {
    export function build_2d(tile: Tile): Cell;
    export function build_3d(tile: Tile): Cell;
    export function build_6d(hex: build.HexTile): Cell6d;
    export function create_2d(config: draw.ConfigNd, rng: Rng): Tile;
    export function create_3d(config: draw.ConfigNd, rng: Rng): Tile;
    export function create_6d(config: draw.ConfigNd, rng: Rng): build.HexTile;
    export function random_tile_2d(max_size: number, rng: Rng): Tile;
    export function random_tile_3d(max_size: number, rng: Rng): Tile;
    export function random_tile_6d(max_size: number, rng: Rng): build.HexTile;
    export type Config2d = draw.ConfigNd;
    export const Config2d: {
        default(): draw.ConfigNd;
    };
    export type Config3d = draw.ConfigNd;
    export const Config3d: {
        default(): draw.ConfigNd;
    };
    export interface HexTile {
        projection: math.six.Projection;
        tile: TileData;
    }
}
export declare namespace core {
    export namespace paint {
        export interface Config {
            editions?: core.paint.Edition[];
            primaries?: core.paint.Ink[];
            target?: core.paint.Target;
        }
        export type Edition = "Simple" | "Index" | "Layers" | "Neighbors" | "Rows" | "Columns" | "Random";
        export type Ink = "Black" | "White" | "Red" | "Orange" | "Yellow" | "Green" | "Mint" | "Teal" | "Cyan" | "Blue" | "Indigo" | "Purple" | "Pink" | "Brown" | "Gray";
        export interface PaintData {
            edition: core.paint.Edition;
            scheme: core.paint.Scheme;
            target: core.paint.Target;
            primary: core.paint.Ink;
            secondary: core.paint.Ink[];
            shades: number[];
        }
        export class Paint {
            private constructor();
            free(): void;
            static from(data: PaintData): Paint;
            toJSON(): PaintData;
            get edition(): core.paint.Edition;
            set edition(value: core.paint.Edition);
            get scheme(): core.paint.Scheme;
            set scheme(value: core.paint.Scheme);
            get target(): core.paint.Target;
            set target(value: core.paint.Target);
            get primary(): core.paint.Ink;
            set primary(value: core.paint.Ink);
            get secondary(): core.paint.Ink[];
            set secondary(value: core.paint.Ink[]);
            get shades(): Uint32Array;
            set shades(value: ArrayLike<number>);
        }
        export type Scheme = "Multicolor" | "Multitone";
        export type Target = "Fill" | "Void";
    }
}
export declare namespace draw {
    export interface ConfigNd {
        groups: Group[];
        catalog: recipe.Catalog;
        min_size: number;
        max_size: number;
        parity: Parity;
        invert?: boolean;
    }
}
export declare namespace math {
    export namespace six {
        export type Projection = "Iso" | "Pro" | "Cut";
    }
}
export declare namespace name {
    export type Slots = number | number[];
    export const Slots: {
        default(): name.Slots;
    };
    export interface TileData {
        kind: string;
        code?: bigint;
        special?: bigint;
        magic: bigint[];
        mosaic: bigint[];
        factor?: number;
        side: name.Slots;
        level?: number;
        turn: name.Slots;
        flip: boolean;
        invert: boolean;
    }
    export class Tile {
        private constructor();
        free(): void;
        static from(data: TileData): Tile;
        toJSON(): TileData;
        get code(): string | undefined;
        set code(value: string | number | bigint | undefined);
        get special(): string | undefined;
        set special(value: string | number | bigint | undefined);
        get magic(): string[];
        set magic(value: (string | number | bigint)[]);
        get mosaic(): string[];
        set mosaic(value: (string | number | bigint)[]);
        get factor(): number | undefined;
        set factor(value: number | undefined);
        get side(): name.Slots;
        set side(value: name.Slots);
        get level(): number | undefined;
        set level(value: number | undefined);
        get turn(): name.Slots;
        set turn(value: name.Slots);
        get flip(): boolean;
        set flip(value: boolean);
        get invert(): boolean;
        set invert(value: boolean);
        checked(): name.Tile;
        static from_file(text: string): name.Tile;
        static from_json(text: string): name.Tile;
        static from_url(text: string): name.Tile;
        static of(recipe: Tile): name.Tile;
        recipe(): Tile;
        to_file(): string;
        to_id(): string;
        to_json(): string;
        to_prose(): string;
        to_url(): string;
    }
}
export declare namespace recipe {
    export function classics(dimension: number): recipe.Design[];
    export function generals(min_size: number, max_size: number, parity: Parity): Uint32Array;
    export function nestings(min_size: number, max_size: number, parity: Parity): Uint32Array[];
    export function powers(min_size: number, max_size: number, parity: Parity): [number, number][];
    export function products(min_size: number, max_size: number, count: number, parity: Parity): Uint32Array[];
    export function size(number: number | bigint, level: number | bigint): number | undefined;
    export function CLASSICS_2D(): recipe.Design[];
    export function CLASSICS_3D(): recipe.Design[];
    export function MAX_LEVEL(): number;
    export function MAX_SIDE(): number;
    export function MAX_SLOTS(): number;
    export function MIN_SIDE(): number;
    export type Catalog = "Classics" | "Universe" | { Codes: bigint[] } | { Designs: recipe.Design[] };
    export type Design = "Carpet" | "Net" | "Htree" | "Vtree" | "Void" | "Xtree" | "Ytree" | "Ztree" | "Point" | "Dust" | "Hline" | "Vline" | "Star" | "Xline" | "Yline" | "Zline";
    export const Design: {
        all(): recipe.Design[];
    };
    export type Source = { design: recipe.Design } | { code: bigint };
}
export declare namespace variation {
    export function create(config: variation.Config, rng: Rng): variation.Variation;
    export function generate(variation: variation.Variation, config: variation.Config, rng: Rng): variation.Variation;
    export function render(variation: variation.Variation, scale: number, rng: Rng): variation.Variation;
    export interface Config {
        tile: draw.ConfigNd;
        paint: core.paint.Config;
        files: [number, number][];
    }
    export const Config: {
        default(): variation.Config;
    };
    export interface FileData {
        width: number;
        height: number;
    }
    export class File {
        constructor(width: number, height: number);
        free(): void;
        static from(data: FileData): File;
        toJSON(): FileData;
        get width(): number;
        set width(value: number);
        get height(): number;
        set height(value: number);
        get image(): Image | undefined;
        set image(value: Image | undefined);
    }
    export interface VariationData {
        key: string;
        seed: number;
        edition: core.paint.Edition;
        primaries?: core.paint.Ink[];
        tile: TileData;
        mask?: TileData;
        paint?: core.paint.PaintData;
        files: variation.FileData[];
    }
    export class Variation {
        private constructor();
        free(): void;
        static from(data: VariationData): Variation;
        toJSON(): VariationData;
        get key(): string;
        set key(value: string);
        get seed(): bigint;
        set seed(value: number | bigint);
        get edition(): core.paint.Edition;
        set edition(value: core.paint.Edition);
        get primaries(): core.paint.Ink[] | undefined;
        set primaries(value: core.paint.Ink[] | undefined);
        get tile(): Tile;
        set tile(value: Tile);
        get mask(): Tile | undefined;
        set mask(value: Tile | undefined);
        get base(): Cell | undefined;
        set base(value: Cell | undefined);
        get files(): variation.File[];
        set files(value: variation.File[]);
        is_cover(): boolean;
        is_prime(): boolean;
    }
}
