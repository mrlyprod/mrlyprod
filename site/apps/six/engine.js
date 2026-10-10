import { num, sheet } from '../../lib/svg.js';
import { named, resolve, title, word } from '../designs/engine.js';

export const DIM = 3;
export const NUMBERS = [3, 5, 7, 9];
export const PROJECTIONS = [['iso', 'Iso'], ['pro', 'Ortho'], ['cut', 'Cut']];
export const ORIENTS = [['pointy', 'Pointy'], ['flat', 'Flat']];
export const ORDER = ['VOID', 'FILL', 'UP', 'LEFT', 'RIGHT'];
export const BUDGET = 32000;
export const RINGS = 4;
export const COPIES = [1, 7, 19, 37, 61];
export const RATIO = Math.sqrt(3) / 2;
export const SCALE = 8;

/* SIZE */

export function cap(number, copies) {
  let level = 1;
  while (6 * (number ** (level + 1)) ** 2 * copies <= BUDGET) level++;
  return level;
}

export function copies(math, rings) {
  if (!rings) return 1;
  let n = 0;
  for (const bit of math.six.radial_mask(rings + 1, 'Horizontal').data) if (bit) n++;
  return n;
}

export function parity(one, rings, side) {
  return (one.start + (rings % 2 ? side : 0)) % 2;
}

/* STUDY */

export function study(math, value, rand) {
  const six = math.six;
  const total = Number(math.bang.factory.total_codes(DIM, value.base));
  const code = String(resolve(value.code, total, rand));
  const n = copies(math, value.radius);
  const level = Math.min(value.level, cap(value.number, n));
  const side = value.number ** level;
  const grown = math.three.create(code, value.number, level, value.base);
  const cube = value.invert ? math.cell.models.anti(grown) : grown;
  const one = six[value.projection](cube);
  const sheet = value.radius ? six.new_(six.tessellate(one, six.radial_mask(value.radius + 1, one.orientation)), one.projection, one.orientation, parity(one, value.radius, side)) : one;
  const seen = value.projection === 'iso' ? six.skin(one) : one;
  const tally = six.census(seen, false);
  const facts = {
    code,
    name: word(named(math, DIM, value.base), code),
    title: title(math, DIM, value.base, code),
    level,
    side,
    copies: n,
    triangles: tally.triangles,
    fills: tally.fills,
    voids: tally.voids,
    pieces: six.components(seen),
    holes: six.holes(seen),
    euler: Number(six.euler(seen, false)),
  };
  return { one, sheet, facts };
}

/* MESH */

export function kinds(math) {
  const six = math.six;
  return { VOID: six.VOID(), FILL: six.FILL(), GRID: six.GRID(), UP: six.UP(), LEFT: six.LEFT(), RIGHT: six.RIGHT() };
}

export function turned(orientation, orient) {
  return (orient === 'flat') !== (orientation === 'Horizontal');
}

export function bounds(groups) {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const { points } of groups) {
    for (let i = 0; i < points.length; i += 2) {
      x0 = Math.min(x0, points[i]);
      x1 = Math.max(x1, points[i]);
      y0 = Math.min(y0, points[i + 1]);
      y1 = Math.max(y1, points[i + 1]);
    }
  }
  return Number.isFinite(x0) ? [x0, y0, x1, y1] : [0, 0, 1, 1];
}

export function mesh(math, sheet, turn) {
  const codes = kinds(math);
  const names = Object.fromEntries(Object.entries(codes).map(([name, type]) => [type, name]));
  const custom = Object.fromEntries(Object.entries(codes).map(([name, type]) => [String(type), [[type, 0, 0, name === 'GRID' ? 0 : 255]]]));
  const across = sheet.orientation === 'Horizontal';
  const runs = new Map();
  for (const [corners, rgba] of math.six.triangles(math.six.paint(sheet, custom))) {
    const name = names[rgba[0]];
    if (!runs.has(name)) runs.set(name, []);
    const pts = runs.get(name);
    for (const [x, y] of corners) {
      const u = across ? Number(x) : Number(x) * RATIO;
      const v = across ? Number(y) * RATIO : Number(y);
      if (turn) pts.push(0 - v, u);
      else pts.push(u, v);
    }
  }
  const groups = ORDER.filter((name) => runs.has(name)).map((name) => ({ name, points: Float64Array.from(runs.get(name)) }));
  return { groups, box: bounds(groups), across };
}

export function fit(box, w, h, pad) {
  const [x0, y0, x1, y1] = box;
  const k = Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 - ((y0 + y1) / 2) * k };
}

/* SVG */

export function wrap(text, across, turn) {
  const found = /^\s*<svg width="([\d.]+)" height="([\d.]+)"[^>]*>([\s\S]*)<\/svg>\s*$/.exec(text);
  if (!found) return text;
  const [, w, h, body] = found;
  const sx = across ? 1 : RATIO;
  const sy = across ? RATIO : 1;
  const W = w * sx;
  const H = h * sy;
  const [width, height] = turn ? [H, W] : [W, H];
  const transform = `${turn ? `translate(${num(H, 3)} 0) rotate(90) ` : ''}scale(${num(sx, 3)} ${num(sy, 3)})`;
  return sheet(width, height, `<g transform="${transform}">${body.trim()}</g>`);
}
