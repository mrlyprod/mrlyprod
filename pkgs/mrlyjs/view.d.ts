export type Color = [number, number, number, number];
export type Point = [number, number];
export type Box = [number, number, number, number];

export interface Pixels {
    shape: [number, number];
    colors: Uint8Array | Uint8ClampedArray;
}

export interface Frame {
    x: number;
    y: number;
    w: number;
    h: number;
    inset(px: number): Frame;
    cell(n: number): number;
    at(u: number, v: number): Point;
    center(): Point;
    square(): Frame;
    radius(): number;
    cols(n: number): Frame[];
    rows(n: number): Frame[];
}

export function frame(x: number, y: number, w: number, h: number): Frame;

export interface Pen {
    readonly width: number;
    readonly height: number;
    frame(margin: number): Frame;
    area(margin: number): Frame;
    rect(x: number, y: number, w: number, h: number, c: Color): void;
    round_rect(x: number, y: number, w: number, h: number, r: number, c: Color): void;
    disc(cx: number, cy: number, r: number, c: Color): void;
    ring(cx: number, cy: number, r: number, thick: number, c: Color): void;
    segment(a: Point, b: Point, thick: number, c: Color): void;
    polyline(pts: Point[], thick: number, c: Color): void;
    triangle(a: Point, b: Point, c: Point, color: Color): void;
    polygon(pts: Point[], c: Color): void;
    arc(center: Point, r: number, angles: [number, number], thick: number, c: Color): void;
    image(x: number, y: number, w: number, h: number, pixels: Pixels): void;
}

export interface Raster extends Pen {
    pixels(): { shape: [number, number]; colors: Uint8ClampedArray };
}

export type Png = (pixels: { shape: [number, number]; colors: Uint8Array }) => Uint8Array | ArrayBuffer | string | Promise<Uint8Array | ArrayBuffer | string>;

export interface Svg extends Pen {
    text(options?: { png?: Png }): Promise<string>;
}

export function raster(width: number, height: number, ground?: Color): Raster;
export function svg(width: number, height: number, ground?: Color): Svg;
export function canvas(ctx: CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D, width: number, height: number, ground?: Color): Pen;

export type Swatch = string | ArrayLike<number> | { r: number; g: number; b: number; a?: number };
export type Palette = Record<string, Swatch> | object;

export class Ramp {
    constructor(stops: Color[], ground?: Color);
    stops: Color[];
    ground: Color;
    at(t: number): Color;
}

export interface Ramps {
    new (stops: Color[]): Ramp;
    heat(): Ramp;
    fire(): Ramp;
    diverge(): Ramp;
    tone(a: Color, b: Color): Ramp;
}

export interface Ink {
    readonly ground: Color;
    readonly panel: Color;
    readonly line: Color;
    readonly fg: Color;
    readonly dim: Color;
    readonly red: Color;
    readonly orange: Color;
    readonly yellow: Color;
    readonly green: Color;
    readonly mint: Color;
    readonly teal: Color;
    readonly cyan: Color;
    readonly blue: Color;
    readonly indigo: Color;
    readonly purple: Color;
    readonly pink: Color;
    readonly brown: Color;
    readonly gray: Color;
    readonly inks: Color[];
    mix(a: Color, b: Color, t: number): Color;
    fade(c: Color, alpha: number): Color;
    Ramp: Ramps;
}

export function ink(palette: Palette): Ink;
export function mix(a: Color, b: Color, t: number): Color;
export function fade(c: Color, alpha: number): Color;

export interface Cell {
    shape: number[];
    types: ArrayLike<number>;
}

export namespace grid {
    export class Grid {
        constructor(frame: Frame, cols: number, rows: number, gap: number);
        frame: Frame;
        cols: number;
        rows: number;
        gap: number;
        cell(col: number, row: number): Box;
        fill(pen: Pen, col: number, row: number, color: Color): void;
        paint(pen: Pen, cells: Cell, ink: (type: number) => Color | null | undefined): void;
        carpet(pen: Pen, mask: boolean[][], color: Color): void;
    }
    export const LOGO: readonly string[];
    export function mask(rows: readonly string[], level: number): boolean[][];
    export function carpet(pen: Pen, frame: Frame, mask: boolean[][], gap: number, color: Color): void;
}

export import Grid = grid.Grid;

export namespace hex {
    export function draw(pen: Pen, frame: Frame, cell: { cell: Cell; start: number }, gap: number, ink: (type: number) => Color | null | undefined): void;
    export function count(n: number): number;
    export function row_len(n: number, row: number): number;
    export function hexagon(pen: Pen, frame: Frame, n: number, gap: number, ink: (row: number, col: number, up: number) => Color | null | undefined): void;
}

export namespace iso {
    export interface Vec3 {
        x: number;
        y: number;
        z: number;
    }
    export interface Quad {
        normal: Vec3;
        verts: Vec3[];
    }
    export function project(x: number, y: number, z: number): Point;
    export function draw(pen: Pen, frame: Frame, quads: Quad[], shade: [Color, Color, Color], edge?: Color | null): void;
}

export namespace plot {
    export function bars(pen: Pen, frame: Frame, values: ArrayLike<number>, gap: number, color: Color): void;
    export function dots(pen: Pen, pts: Point[], r: number, color: Color): void;
    export function rings(pen: Pen, center: Point, radii: number[], thick: number, color: Color): void;
    export function staircase(pen: Pen, frame: Frame, values: ArrayLike<number>, thick: number, color: Color): void;
    export function curve(pen: Pen, frame: Frame, xs: ArrayLike<number>, ys: ArrayLike<number>, thick: number, color: Color): void;
    export function axis(pen: Pen, frame: Frame, color: Color): void;
    export function baseline(pen: Pen, frame: Frame, color: Color): void;
}

export namespace field {
    export function draw(pen: Pen, frame: Frame, width: number, height: number, values: ArrayLike<number>, ramp: Ramp): void;
    export function draw_range(pen: Pen, frame: Frame, width: number, height: number, values: ArrayLike<number>, span: [number, number], ramp: Ramp): void;
    export function sample(pen: Pen, frame: Frame, resolution: number, f: (u: number, v: number) => number, ramp: Ramp): void;
}
