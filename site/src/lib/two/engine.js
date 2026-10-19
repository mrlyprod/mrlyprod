import { tidy } from '../app/spec.js';
import { DESIGN, canon } from '../designs/engine.js';
import { runs } from '../app/svg.js';

export const DIM = 2;
export const BUDGET = 262144;
const MIME = { png: 'image/png', webp: 'image/webp' };

/* ROOM */

export const cap = (math, number, copies = 1) => math.bang.level_cap(number, DIM, Math.floor(BUDGET / copies));

/* MEMO */

export function memo() {
  let key = null;
  let held;
  return (next, make) => {
    if (next !== key) {
      held = make();
      key = next;
    }
    return held;
  };
}

/* DESIGN */

export const read = (value) => tidy(DESIGN, value);

export const design = (math, base, code) => canon(math, DIM, base, code);

export const named = (math, code, base) => math.name.of(code, DIM, base) ?? '';

export const tag = (facts) => `${facts.name || 'code'} ${facts.code}`;

export const tensor = (cell) => ({ shape: cell.shape, data: cell.types });

export const cell = (grid) => ({ shape: grid.shape, types: grid.data });

/* SHEET */

export function sheet(math, tile, x, y) {
  return x * y === 1 ? tile : math.two.merge(Array.from({ length: x * y }, () => tile), x, y);
}

export function joins([rows, cols], side) {
  const out = [];
  for (let col = side; col < cols; col += side) out.push([0, col, 0, rows]);
  for (let row = side; row < rows; row += side) out.push([1, row, 0, cols]);
  return out;
}

export function layers(math, grid, invert) {
  const other = math.cell.models.invert(grid);
  const lit = invert ? other : grid;
  return { lit: runs(lit), faint: runs(invert ? grid : other), drawn: lit };
}

/* COUNTS */

export function count(math, { code, base, number, level, x, y }, tile, block) {
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

export function lay(cols, rows, w, h, fit) {
  const px = Math.min((w * fit) / cols, (h * fit) / rows);
  return { px, x: (w - px * cols) / 2, y: (h - px * rows) / 2 };
}

/* PAINT */

export function stamp(pen, list, box, color) {
  pen.fillStyle = color;
  pen.beginPath();
  for (const [row, col, len] of list) {
    const left = Math.round(box.x + col * box.px);
    const top = Math.round(box.y + row * box.px);
    pen.rect(left, top, Math.round(box.x + (col + len) * box.px) - left, Math.round(box.y + (row + 1) * box.px) - top);
  }
  pen.fill();
}

export function stroke(pen, lines, box, color, thick) {
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

export function shot(cols, rows, size, paint, kind = 'png') {
  const long = Math.max(cols, rows);
  const w = Math.round((size * cols) / long);
  const h = Math.round((size * rows) / long);
  const page = new OffscreenCanvas(w, h);
  paint(page.getContext('2d'), { px: size / long, x: 0, y: 0 }, w, h);
  return page.convertToBlob({ type: MIME[kind] ?? MIME.png });
}
