import { named, resolve, total, word } from '../designs/engine.js';
import { runs } from '../../lib/svg.js';

export const DIM = 2;
export const BUDGET = 262144;
export const DEN = 32;
export const SHAPES = [['', 'None'], ['ball', 'Circle'], ['box', 'Square'], ['diamond', 'Diamond'], ['triangle', 'Triangle'], ['octagon', 'Octagon']];
export const REACH = { ball: 23, box: 16, diamond: 32, octagon: 22 };
const CUT = 1;
const IN = 2;

/* ROOM */

export function cap(number, copies) {
  let level = 1;
  while (number ** (2 * (level + 1)) * copies <= BUDGET) level++;
  return level;
}

/* SHAPE */

export function outline(math, name, radius) {
  const r = new math.shape.Frac(radius, DEN);
  const shape = math.shape.named(name, DIM, r);
  r.free();
  return shape;
}

export function square(sheet) {
  const [rows, cols] = sheet.shape;
  const m = Math.min(rows, cols);
  const top = (rows - m) >> 1;
  const left = (cols - m) >> 1;
  const data = new Uint8Array(m * m);
  for (let r = 0; r < m; r++) data.set(sheet.data.subarray((top + r) * cols + left, (top + r) * cols + left + m), r * m);
  return { shape: [m, m], data, top, left };
}

export function paste(window, data, rows, cols) {
  const m = window.shape[0];
  const out = new Uint8Array(rows * cols);
  for (let r = 0; r < m; r++) out.set(data.subarray(r * m, (r + 1) * m), (window.top + r) * cols + window.left);
  return out;
}

/* STUDY */

export function study(math, value, rand) {
  const { base, number, x, y, crop, radius, touch, invert } = value;
  const code = String(resolve(value.code, total(math, DIM, base), rand));
  const top = cap(number, x * y);
  const level = Math.min(value.level, top);
  const tile = math.two.create(code, number, level, 0, base);
  const block = math.two.merge(Array.from({ length: x * y }, () => tile), x, y);
  const [rows, cols] = block.shape;
  const sheet = { shape: block.shape, data: block.types };
  const shape = crop ? outline(math, crop, radius) : null;
  const window = shape ? square(sheet) : null;
  const kept = shape ? math.shape.crop({ shape: window.shape, data: window.data }, shape, Boolean(touch)) : sheet;
  const tally = shape ? math.shape.census(shape, { shape: window.shape, data: window.data }).cells : null;
  const cells = tally ? tally[IN] + (touch ? tally[CUT] : 0) : rows * cols;
  const region = shape && invert ? math.shape.regions(shape, window.shape).data : null;
  const inside = (i) => !region || region[i] === IN || (touch && region[i] === CUT);
  const types = invert ? kept.data.map((on, i) => (on || !inside(i) ? 0 : 1)) : kept.data;
  const drawn = { shape: block.shape, types: window ? paste(window, types, rows, cols) : types };
  const read = math.two.census({ shape: kept.shape, types: kept.data });
  const facts = {
    code,
    name: word(named(math, DIM, base), code),
    level,
    capped: value.level > top,
    side: tile.shape[0],
    cols,
    rows,
    cells,
    fills: read.fills,
    voids: cells - read.fills,
    perimeter: Number(read.perimeter),
    euler: read.euler,
    dimension: math.counts.dimension(code, number, DIM, base),
  };
  return { drawn, runs: runs(drawn), cols, rows, facts };
}

/* LABEL */

export const tag = (facts) => `${facts.name || 'code'} ${facts.code}`;

/* LAYOUT */

export function lay(cols, rows, w, h, fit) {
  const px = Math.min((w * fit) / cols, (h * fit) / rows);
  return { px, x: (w - px * cols) / 2, y: (h - px * rows) / 2 };
}
