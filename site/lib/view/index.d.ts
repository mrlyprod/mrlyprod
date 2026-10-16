/** An rgba color as four whole bytes. */
export type Color = [number, number, number, number];
/** A point in pen space: x to the right, y down, in pixels. */
export type Point = [number, number];
/** A box as its left, its top, its width and its height. */
export type Box = [number, number, number, number];

/** A field of pixels that image paints, nearest sampled, and that a renderer may flatten to a PNG. */
export interface Pixels {
    /** The height then the width in pixels. */
    shape: [number, number];
    /** The rgba bytes, row by row, four a pixel. */
    colors: Uint8Array | Uint8ClampedArray;
    /** The coverage of each pixel, one a pixel and clamped to the unit interval; absent means every pixel at one. */
    cover?: ArrayLike<number>;
}

/** A box of whole pixels holding one color and one coverage a pixel, painted as one image: the pen's Board.blend. */
export interface Patch extends Pixels {
    /** The rgba bytes, zero until blended. */
    colors: Uint8ClampedArray;
    /** The coverage a pixel, zero until blended, so an untouched pixel leaves the ground alone. */
    cover: Float64Array;
    /** Sets the pixel at board coordinates px, py to the color at the coverage, one at default; a pixel outside the box is ignored and a second blend replaces the first. */
    blend(px: number, py: number, color: Color, cover?: number): void;
    /** Paints the whole patch onto the pen at the box it was made for. */
    paint(pen: Pen): void;
}

/** A rectangle of pen space that a figure lays itself out in. */
export interface Frame {
    /** The left edge in pixels. */
    x: number;
    /** The top edge in pixels. */
    y: number;
    /** The width in pixels. */
    w: number;
    /** The height in pixels. */
    h: number;
    /** Shrinks the frame by px on every side. */
    inset(px: number): Frame;
    /** Returns the width of one of n columns. */
    cell(n: number): number;
    /** Maps unit coordinates, zero at the top left and one at the bottom right, to pixels. */
    at(u: number, v: number): Point;
    /** Returns the middle of the frame. */
    center(): Point;
    /** Returns the largest square centred inside the frame. */
    square(): Frame;
    /** Returns the shorter half side, the radius a centred disc fills the frame with. */
    radius(): number;
    /** Splits the frame into n columns, left to right. */
    cols(n: number): Frame[];
    /** Splits the frame into n rows, top to bottom. */
    rows(n: number): Frame[];
    /** Splits the frame into rows by cols tiles, row by row and left to right, each inset by gap pixels. */
    panels(rows: number, cols: number, gap: number): Frame[];
}

/** Builds a frame from its corner and its size. */
export function frame(x: number, y: number, w: number, h: number): Frame;

/** The pen: the nine verbs of board.rs and image, drawn by the raster, the svg or the canvas renderer alike. */
export interface Pen {
    /** The width in pixels. */
    readonly width: number;
    /** The height in pixels. */
    readonly height: number;
    /** Returns the largest centred square left after a margin of that fraction of the short side. */
    frame(margin: number): Frame;
    /** Returns the whole pen inset by a margin of that fraction of the short side. */
    area(margin: number): Frame;
    /** Fills an axis-aligned rectangle. */
    rect(x: number, y: number, w: number, h: number, c: Color): void;
    /** Fills a rectangle with rounded corners of radius r, clamped to half the short side. */
    round_rect(x: number, y: number, w: number, h: number, r: number, c: Color): void;
    /** Fills a disc. */
    disc(cx: number, cy: number, r: number, c: Color): void;
    /** Strokes a circle of radius r, the stroke centred on it. */
    ring(cx: number, cy: number, r: number, thick: number, c: Color): void;
    /** Strokes a straight run between two points, with round caps. */
    segment(a: Point, b: Point, thick: number, c: Color): void;
    /** Strokes a chain of points as one stroke, with round caps and joints, blended once where it crosses itself. */
    polyline(pts: Point[], thick: number, c: Color): void;
    /** Fills a triangle. */
    triangle(a: Point, b: Point, c: Point, color: Color): void;
    /** Fills any simple polygon, its inside decided by the even-odd rule. */
    polygon(pts: Point[], c: Color): void;
    /** Strokes the arc about a center between two angles in radians, clockwise on the screen, with round caps. */
    arc(center: Point, r: number, angles: [number, number], thick: number, c: Color): void;
    /** Paints pixels nearest sampled into the box, each blended at its own color alpha times its own cover. */
    image(x: number, y: number, w: number, h: number, pixels: Pixels): void;
}

/** The pixel oracle: a pen that fills an rgba byte array, the same bytes in Bun and the browser. */
export interface Raster extends Pen {
    /** Returns the pixels drawn so far, shape height then width, rgba bytes row by row. */
    pixels(): { shape: [number, number]; colors: Uint8ClampedArray };
}

/** A PNG encoder for the svg renderer: takes pixels, returns the bytes, an ArrayBuffer or a data URL, maybe late. */
export type Png = (pixels: { shape: [number, number]; colors: Uint8Array }) => Uint8Array | ArrayBuffer | string | Promise<Uint8Array | ArrayBuffer | string>;

/** A pen that builds an SVG string; an image goes in as an embedded PNG. */
export interface Svg extends Pen {
    /** Returns the finished SVG; the png encoder is required when an image was drawn. */
    text(options?: { png?: Png }): Promise<string>;
}

/** Builds a raster pen of width by height flooded with the ground, clear by default. */
export function raster(width: number, height: number, ground?: Color): Raster;
/** Builds an svg pen of width by height under the ground, no rect when the ground is clear. */
export function svg(width: number, height: number, ground?: Color): Svg;
/** Builds a pen over a Canvas2D context for live drawing, clearing it and flooding the ground first. */
export function canvas(ctx: CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D, width: number, height: number, ground?: Color): Pen;

/** A color as a hex string, a byte array or serde's r, g, b and a. */
export type Swatch = string | ArrayLike<number> | { r: number; g: number; b: number; a?: number };
/** A palette: a swatch for each of the eighteen names of Ink. */
export type Palette = Record<string, Swatch> | object;

/** A color ramp: a line through its stops, evenly spaced from zero to one. */
export class Ramp {
    /** Builds a ramp from its stops; the ground answers when there are none, clear by default. */
    constructor(stops: Color[], ground?: Color);
    /** The stops. */
    stops: Color[];
    /** The color an empty ramp reads. */
    ground: Color;
    /** Reads the ramp at t, clamped to the unit interval. */
    at(t: number): Color;
}

/** The Ramp of one Ink: its ground is the ink's ground, and it carries ink.rs's recipes. */
export interface Ramps {
    /** Builds a ramp from its stops. */
    new (stops: Color[]): Ramp;
    /** The heat ramp: ground, blue, yellow, foreground. */
    heat(): Ramp;
    /** The fire ramp: ground, orange, yellow, foreground. */
    fire(): Ramp;
    /** The diverging ramp: blue through the ground to orange. */
    diverge(): Ramp;
    /** The two-tone ramp from one color straight to another. */
    tone(a: Color, b: Color): Ramp;
}

/** The palette of one theme, resolved: the surfaces, the hues, the six inks, mix, fade and Ramp. */
export interface Ink {
    /** The ground every figure is painted on. */
    readonly ground: Color;
    /** The raised panel, one step off the ground. */
    readonly panel: Color;
    /** The hairline that separates one thing from the next. */
    readonly line: Color;
    /** The foreground, the strongest tone on the ground. */
    readonly fg: Color;
    /** The dimmed foreground, for anything secondary. */
    readonly dim: Color;
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
    /** The six inks in their fixed order: blue, orange, yellow, green, pink, indigo. */
    readonly inks: Color[];
    /** Blends two colors channel by channel, t clamped to the unit interval, each channel rounded half up. */
    mix(a: Color, b: Color, t: number): Color;
    /** Returns the color with its alpha replaced by that fraction of opaque, clamped to the unit interval. */
    fade(c: Color, alpha: number): Color;
    /** The ramp class of this ink. */
    Ramp: Ramps;
}

/** Resolves a palette into an Ink; throws when a name is missing. */
export function ink(palette: Palette): Ink;
/** Blends two colors channel by channel, t clamped to the unit interval. */
export function mix(a: Color, b: Color, t: number): Color;
/** Returns the color with its alpha replaced by that fraction of opaque. */
export function fade(c: Color, alpha: number): Color;

/** A flat design: its shape, height then width, and one type byte a cell. */
export interface Cell {
    /** The height then the width. */
    shape: number[];
    /** The type bytes, row by row. */
    types: ArrayLike<number>;
}

/** A flat design from a door: its shape, height then width, and one type byte a cell in data. */
export interface Tensor {
    /** The height then the width. */
    shape: number[];
    /** The type bytes, row by row. */
    data: ArrayLike<number>;
}

/** The square lattice, the design painter and the Kronecker masks. */
export namespace grid {
    /** A lattice of cols by rows cells laid over a frame, each cell drawn inside its own gap. */
    export class Grid {
        /** Lays a lattice over the frame, at least one by one, keeping a gap of that fraction of a cell. */
        constructor(frame: Frame, cols: number, rows: number, gap: number);
        /** The frame the lattice covers. */
        frame: Frame;
        /** The number of columns. */
        cols: number;
        /** The number of rows. */
        rows: number;
        /** The share of a cell left empty between one cell and the next. */
        gap: number;
        /** Returns the drawn box of one cell; at gap zero the edges snap to whole pixels so neighbours meet without a seam. */
        cell(col: number, row: number): Box;
        /** Fills one cell. */
        fill(pen: Pen, col: number, row: number, color: Color): void;
        /** Fills every cell whose type byte the ink maps to a color; reads a cell's types or a tensor's data alike. */
        paint(pen: Pen, cells: Cell | Tensor, ink: (type: number) => Color | null | undefined): void;
        /** Fills every true cell of a mask. */
        carpet(pen: Pen, mask: boolean[][], color: Color): void;
    }
    /** The MrlyProd logo, the five by five seed the mark grows from. */
    export const LOGO: readonly string[];
    /** Grows a 0/1 string mask by Kronecker substitution, level one being the seed itself. */
    export function mask(rows: readonly string[], level: number): boolean[][];
    /** Fills every true cell of a mask laid over a frame. */
    export function carpet(pen: Pen, frame: Frame, mask: boolean[][], gap: number, color: Color): void;
}

/** The lattice of the square grid, also at the top level. */
export import Grid = grid.Grid;

/** The triangle meshes of the hexagon world. */
export namespace hex {
    /** Draws the triangle mesh of a hex slice into the frame, centred and equilateral, each triangle pulled back by gap pixels; throws when the cell is square. */
    export function draw(pen: Pen, frame: Frame, cell: { cell: Cell; start: number }, gap: number, ink: (type: number) => Color | null | undefined): void;
    /** Returns the number of unit triangles in a plain hexagon of side n, six n squared. */
    export function count(n: number): number;
    /** Returns the number of triangles in row of a plain hexagon of side n, rows counted from the top. */
    export function row_len(n: number, row: number): number;
    /** Reads the type byte at a column of a row of a hexagon slice of side n, the row centred in the slice's width. */
    export function at(slice: { cell: Cell }, side: number, row: number, col: number): number;
    /** Draws a plain hexagon of side n, each triangle asked of the ink by row, column and one for up or zero for down, pulled back by gap pixels. */
    export function hexagon(pen: Pen, frame: Frame, n: number, gap: number, ink: (row: number, col: number, up: number) => Color | null | undefined): void;
}

/** The isometric cube: faces, stamps and cages. */
export namespace iso {
    /** A point of cube space. */
    export interface Vec3 {
        /** The x component, running right and down. */
        x: number;
        /** The y component, running left and down. */
        y: number;
        /** The z component, running up. */
        z: number;
    }
    /** An outward face of a filled site: its normal and its four corners. */
    export interface Quad {
        /** The outward normal. */
        normal: Vec3;
        /** The four corners in winding order. */
        verts: Vec3[];
    }
    /** A face seen from the viewer: its projected corners, its tone and its depth. */
    export interface Face {
        /** The four corners projected, before any fit. */
        quad: Point[];
        /** Zero for the top, one for the left, two for the right. */
        tone: number;
        /** The mean of x, y and z over the corners, summed in f32 as Rust does; the larger is nearer. */
        depth: number;
    }
    /** The twelve edges of a cube as corner pairs, low to high, corner i having x as bit 0, y as bit 1 and z as bit 2. */
    export const EDGES: readonly (readonly [number, number])[];
    /** Projects a point of cube space to the isometric plane: x runs right and down, y left and down, z up. */
    export function project(x: number, y: number, z: number): Point;
    /** Drops the faces turned away from the viewer and returns the rest projected and sorted back to front. */
    export function faces(quads: Quad[]): Face[];
    /** Returns the map that fits the faces into the frame, centred and keeping the proportions. */
    export function fit(faces: Face[], frame: Frame): (p: Point) => Point;
    /** Draws a cube's exposed faces fitted to the frame and painted back to front; the shades are the top, the left and the right, and the edge, when given, is a hairline round every face. */
    export function draw(pen: Pen, frame: Frame, quads: Quad[], shade: readonly Color[], edge?: Color | null): void;
    /** Stamps a cube's faces back to front with its center at cx, cy and s pixels to the unit, each ringed in the edge at thick when an edge is given. */
    export function stamp(pen: Pen, quads: Quad[], cx: number, cy: number, s: number, shade: readonly Color[], edge: Color | null, thick: number): void;
    /** Strokes the twelve edges of a box of half the given half side about cx, cy at s pixels to the unit, in EDGES order. */
    export function cage(pen: Pen, cx: number, cy: number, s: number, half: number, thick: number, color: Color): void;
}

/** The plain marks: bars, dots, rings, staircases, curves and a bare axis. */
export namespace plot {
    /** Draws one bar per value from the foot of the frame, the tallest filling its height. */
    export function bars(pen: Pen, frame: Frame, values: ArrayLike<number>, gap: number, color: Color): void;
    /** Draws one disc at each point. */
    export function dots(pen: Pen, pts: Point[], r: number, color: Color): void;
    /** Strokes one circle per radius about the same center. */
    export function rings(pen: Pen, center: Point, radii: number[], thick: number, color: Color): void;
    /** Strokes the values as a staircase across the frame, one tread per value. */
    export function staircase(pen: Pen, frame: Frame, values: ArrayLike<number>, thick: number, color: Color): void;
    /** Strokes a curve through the paired data, both axes mapped from their own range into the frame. */
    export function curve(pen: Pen, frame: Frame, xs: ArrayLike<number>, ys: ArrayLike<number>, thick: number, color: Color): void;
    /** Strokes the bare hairline box of the frame, without a tick or a label. */
    export function axis(pen: Pen, frame: Frame, color: Color): void;
    /** Strokes a horizontal rule at y across the frame as forty eight dashes, each over half a step long. */
    export function dashed(pen: Pen, frame: Frame, y: number, thick: number, color: Color): void;
    /** Strokes the hairline foot of the frame alone. */
    export function baseline(pen: Pen, frame: Frame, color: Color): void;
}

/** The scalar fields painted through a ramp, and the patch for a pixel field with a cover of its own. */
export namespace field {
    /** Paints a scalar field into the frame, nearest sampled, normalised to its own range. */
    export function draw(pen: Pen, frame: Frame, width: number, height: number, values: ArrayLike<number>, ramp: Ramp): void;
    /** Paints a scalar field into the frame, nearest sampled, normalised to the given range. */
    export function draw_range(pen: Pen, frame: Frame, width: number, height: number, values: ArrayLike<number>, span: [number, number], ramp: Ramp): void;
    /** Paints a function over the unit square into the frame, sampled on a resolution by resolution grid. */
    export function sample(pen: Pen, frame: Frame, resolution: number, f: (u: number, v: number) => number, ramp: Ramp): void;
    /** Makes a patch of whole pixels at x, y of w by h; throws on anything but whole pixels. */
    export function patch(x: number, y: number, w: number, h: number): Patch;
}
