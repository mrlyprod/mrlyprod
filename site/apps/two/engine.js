import { settle } from '../designs/engine.js';
import { runs } from '../../lib/svg.js';

export const DIM = 2;
export const BUDGET = 262144;
export const DEN = 32;
export const SHAPES = [['', 'None'], ['ball', 'Circle'], ['box', 'Square'], ['diamond', 'Diamond'], ['triangle', 'Triangle'], ['octagon', 'Octagon']];
const CUT = 1;
const IN = 2;

/* ROOM */

export const cap = (math, number, copies) => math.bang.level_cap(number, DIM, Math.floor(BUDGET / copies));

export const reach = (value, units) => (units && value.crop ? units.math.shape.reach(value.crop, DIM, DEN) ?? DEN : DEN);

/* SHAPE */

export function outline(math, name, radius) {
  const r = new math.shape.Frac(radius, DEN);
  const shape = math.shape.named(name, DIM, r);
  r.free();
  return shape;
}

/* STUDY */

export function study(math, value, rand) {
  const { base, number, x, y, crop, radius, touch, invert } = value;
  const code = settle(math, DIM, base, value.code, rand);
  const top = cap(math, number, x * y);
  const level = Math.min(value.level, top);
  const tile = math.two.create(code, number, level, 0, base);
  const block = math.two.merge(Array.from({ length: x * y }, () => tile), x, y);
  const [rows, cols] = block.shape;
  const sheet = { shape: block.shape, data: block.types };
  const shape = crop ? math.shape.fit(outline(math, crop, radius), block.shape) : null;
  const kept = shape ? math.shape.crop(sheet, shape, Boolean(touch)) : sheet;
  const tally = shape ? math.shape.census(shape, sheet).cells : null;
  const cells = tally ? tally[IN] + (touch ? tally[CUT] : 0) : rows * cols;
  const region = shape && invert ? math.shape.regions(shape, block.shape).data : null;
  const inside = (i) => !region || region[i] === IN || (touch && region[i] === CUT);
  const types = invert ? kept.data.map((on, i) => (on || !inside(i) ? 0 : 1)) : kept.data;
  const drawn = { shape: block.shape, types };
  const read = math.two.census({ shape: kept.shape, types: kept.data });
  const facts = {
    code,
    name: math.name.of(code, DIM, base) ?? '',
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
