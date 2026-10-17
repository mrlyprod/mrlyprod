import { rng } from '../../lib/rand.js';
import { num, sheet as paper } from '../../lib/app/svg.js';
import { roll, settle, title } from '../../lib/designs/engine.js';

export const DIM = 3;
export const NUMBERS = [3, 5, 7, 9];
export const VIEWS = [['cut', 'Middle slice'], ['pro', 'Three sides'], ['iso', 'Iso skin']];
export const SHEETS = [['rect', 'Rectangle'], ['disc', 'Disc']];
export const PRESETS = [[5, 5], [3, 9]];
export const ORDER = ['VOID', 'FILL', 'UP', 'LEFT', 'RIGHT'];
export const BUDGET = 32000;
export const REPEATS = 12;
export const RINGS = 4;
export const RATIO = Math.sqrt(3) / 2;
export const SCALE = 8;

/* SIZE */

export const copies = (math, value) => (value.sheet === 'disc' ? math.six.copies(value.radius) : value.x * value.y);

export const cap = (math, number, n) => math.bang.level_cap(number, 2, Math.floor(BUDGET / (6 * n)));

export const croppable = (value) => value.sheet !== 'disc' && value.x >= 2 && value.y >= 2;

export const cropped = (value) => croppable(value) && Number(value.overhang) !== 1;

export const reroll = (math, base, seed) => roll(math, DIM, base, rng(seed));

/* STUDY */

const keyed = (...parts) => JSON.stringify(parts);

export function studio(math) {
  const six = math.six;
  const held = new Map();
  const layer = (name, key, build) => {
    const was = held.get(name);
    if (was?.key === key) return was.out;
    const out = build();
    held.set(name, { key, out });
    return out;
  };
  return (value) => {
    const n = copies(math, value);
    const top = cap(math, value.number, n);
    const level = Math.min(value.level, top);
    const blank = String(value.code ?? '').trim() === '';
    const code = layer('code', keyed(value.base, value.code, blank ? value.seed : 0), () => settle(math, DIM, value.base, value.code, rng(value.seed)));
    const grown = keyed(code, value.number, level, value.base, value.invert);
    const cube = layer('cube', grown, () => {
      const made = math.three.create(code, value.number, level, value.base);
      return value.invert ? math.cell.models.anti(made) : made;
    });
    const seen = keyed(grown, value.view);
    const one = layer('one', seen, () => six[value.view](cube));
    const crop = cropped(value);
    const disc = value.sheet === 'disc';
    const key = keyed(seen, disc ? ['disc', value.radius] : [value.x, value.y, crop]);
    const sheet = layer('sheet', key, () => (disc ? six.disc(one, value.radius) : six.tile_cell(one, value.x, value.y, crop)));
    const drawn = layer('drawn', key, () => six.framed(sheet));
    const facts = layer('facts', keyed(key, top), () => {
      const skin = six.skin(sheet);
      const tally = six.census(skin, false);
      return {
        code,
        name: math.name.of(code, DIM, value.base) ?? '',
        title: title(math, DIM, value.base, code),
        level,
        top,
        side: value.number ** level,
        copies: n,
        tile: [six.width(one), six.height(one)],
        sheet: [six.width(sheet), six.height(sheet)],
        triangles: tally.triangles,
        fills: tally.fills,
        voids: tally.voids,
        exposed: six.fills_only(skin).boundary_edges,
        alone: six.fills_only(six.skin(one)).boundary_edges,
        pieces: six.components(skin),
        holes: six.holes(skin),
        corners: tally.vertices,
        edges: tally.edges,
        euler: tally.euler,
      };
    });
    return { key, cube, one, sheet, drawn, facts };
  };
}

/* MESH */

export function kinds(math) {
  const six = math.six;
  return { VOID: six.VOID(), FILL: six.FILL(), GRID: six.GRID(), UP: six.UP(), LEFT: six.LEFT(), RIGHT: six.RIGHT() };
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
  return paper(width, height, `<g transform="${transform}">${body.trim()}</g>`);
}
