import type { Cell, Tensor } from "@mrlyprod/mrly/math";
import type { Raw } from "../app/spec.ts";
import { tidy } from "../app/spec.ts";
import type { Run } from "../app/svg.ts";
import { runs } from "../app/svg.ts";
import type { Code, Mrly } from "../designs/engine.ts";
import { canon, DESIGN } from "../designs/engine.ts";
import type { Design } from "../designs/store.ts";

/* TYPES */

export type Memo<T> = (next: string, make: () => T) => T;

export type Seam = [number, number, number, number];

export type Layers = { lit: Run[]; faint: Run[]; drawn: Cell };

export type Source = { code: Code; base: number; number: number; level: number; x: number; y: number };

export type Counts = {
  code: Code;
  name: string;
  level: number;
  side: number;
  copies: number;
  cols: number;
  rows: number;
  fills: number;
  voids: number;
  exposed: number;
  one: number;
  corners: number;
  edges: number;
  euler: number;
  dimension: number;
};

export type Tagged = { name?: string; code: Code };

export type Layout = { px: number; x: number; y: number };

export type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

export type Paint = (pen: Pen, box: Layout, w: number, h: number) => void;

export const DIM = 2;
export const BUDGET = 262144;
const MIME: Partial<Record<string, string>> = { png: "image/png", webp: "image/webp" };

/* ROOM */

export const cap = (math: Mrly, number: number, copies = 1) =>
  math.bang.level_cap(number, DIM, Math.floor(BUDGET / copies));

/* MEMO */

export function memo<T>(): Memo<T> {
  let key: string | null = null;
  let held: T;
  return (next, make) => {
    if (next !== key) {
      held = make();
      key = next;
    }
    return held;
  };
}

/* DESIGN */

export const read = (value: Raw) => tidy(DESIGN, value) as Design;

export const design = (math: Mrly, base: number, code: Code | null | undefined) => canon(math, DIM, base, code);

export const named = (math: Mrly, code: Code, base: number) => math.name.of(code, DIM, base) ?? "";

export const tag = (facts: Tagged) => `${facts.name || "code"} ${facts.code}`;

export const tensor = (cell: Cell): Tensor => ({ shape: cell.shape, data: cell.types });

export const cell = (grid: Tensor): Cell => ({ shape: grid.shape, types: grid.data });

/* SHEET */

export function sheet(math: Mrly, tile: Cell, x: number, y: number): Cell {
  return x * y === 1
    ? tile
    : math.two.merge(
        Array.from({ length: x * y }, () => tile),
        x,
        y,
      );
}

export function joins([rows, cols]: readonly number[], side: number): Seam[] {
  const out: Seam[] = [];
  for (let col = side; col < cols; col += side) out.push([0, col, 0, rows]);
  for (let row = side; row < rows; row += side) out.push([1, row, 0, cols]);
  return out;
}

export function layers(math: Mrly, grid: Cell, invert: number): Layers {
  const other = math.cell.models.invert(grid);
  const lit = invert ? other : grid;
  return { lit: runs(lit), faint: runs(invert ? grid : other), drawn: lit };
}

/* COUNTS */

export function count(math: Mrly, { code, base, number, level, x, y }: Source, tile: Cell, block: Cell): Counts {
  const read = math.two.census(block);
  const [rows, cols] = block.shape;
  return {
    code,
    name: named(math, code, base),
    level,
    side: tile.shape[0],
    copies: x * y,
    cols,
    rows,
    fills: read.fills,
    voids: read.voids,
    exposed: Number(read.perimeter),
    one: Number(math.two.perimeter(tile)),
    corners: read.vertices,
    edges: read.edges,
    euler: read.euler,
    dimension: math.counts.dimension(code, number, DIM, base),
  };
}

/* LAYOUT */

export function lay(cols: number, rows: number, w: number, h: number, fit: number): Layout {
  const px = Math.min((w * fit) / cols, (h * fit) / rows);
  return { px, x: (w - px * cols) / 2, y: (h - px * rows) / 2 };
}

/* PAINT */

export function stamp(pen: Pen, list: readonly Run[], box: Layout, color: string) {
  pen.fillStyle = color;
  pen.beginPath();
  for (const [row, col, len] of list) {
    const left = Math.round(box.x + col * box.px);
    const top = Math.round(box.y + row * box.px);
    pen.rect(left, top, Math.round(box.x + (col + len) * box.px) - left, Math.round(box.y + (row + 1) * box.px) - top);
  }
  pen.fill();
}

export function stroke(pen: Pen, lines: readonly Seam[], box: Layout, color: string, thick: number) {
  pen.fillStyle = color;
  pen.beginPath();
  for (const [axis, at, from, len] of lines) {
    const line = Math.round((axis ? box.y : box.x) + at * box.px - thick / 2);
    const start = Math.round((axis ? box.x : box.y) + from * box.px);
    const span = Math.round((axis ? box.x : box.y) + (from + len) * box.px) - start;
    if (axis) pen.rect(start, line, span, thick);
    else pen.rect(line, start, thick, span);
  }
  pen.fill();
}

/* SHOT */

export function shot(cols: number, rows: number, size: number, paint: Paint, kind = "png"): Promise<Blob> {
  const long = Math.max(cols, rows);
  const w = Math.round((size * cols) / long);
  const h = Math.round((size * rows) / long);
  const page = new OffscreenCanvas(w, h);
  paint(page.getContext("2d")!, { px: size / long, x: 0, y: 0 }, w, h);
  return page.convertToBlob({ type: MIME[kind] ?? MIME.png });
}
