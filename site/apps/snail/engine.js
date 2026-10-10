import { num as fmt, path, runs, sheet } from '../../lib/svg.js';
import { named, resolve, total, word } from '../designs/engine.js';

export const DIM = 2;
export const NUMBERS = [2, 3, 5, 7];
export const GROWTHS = [['Every', 'Every'], ['Prime', 'Primes']];
export const CELLS = [[0, 'Fit'], [0.5, '1/2 px'], [1, '1 px'], [2, '2 px'], [4, '4 px']];
export const TOP = 2000;
export const TILE = 60;
export const LEAST = 400;
export const MOST = 3000;
export const HOLD = 3000;
export const FADE = 1000;
export const LEAD = 600;
export const PAD = 24;
export const SIZE = 2048;
export const FAINT = 0.6;
export const EDGE = 0.45;
export const SOFT = 2;
export const FILL = 0.08;
export const TRAIL = 0.5;

/* STUDY */

export function study(math, num, value, rand) {
  const { base, number, top, growth } = value;
  const code = String(resolve(value.code, total(math, DIM, base), rand));
  const shell = num.spiral.snail(number, top, growth);
  const peak = num.spiral.level_of(top, number);
  const art = Array.from({ length: peak + 1 }, (_, level) => (level ? math.two.create(code, number, level, 0, base) : null));
  const lines = art.map((cell) => (cell ? runs(cell) : null));
  const times = schedule(number, top);
  const counts = Array.from(shell.levels);
  const facts = {
    code,
    name: word(named(math, DIM, base), code),
    tiles: shell.tiles.length,
    primes: shell.primes,
    grown: shell.tiles.length - counts[0],
    levels: counts.map((count, level) => ({ level, count, side: number ** level })),
    side: number ** counts.findLastIndex((count) => count > 0),
    area: Number(shell.area),
    width: shell.high[0] - shell.low[0],
    height: shell.high[1] - shell.low[1],
  };
  return { code, shell, art, lines, times, facts };
}

export const tag = ({ name, code }) => `${name || 'code'} ${code}`;

/* TIME */

export function schedule(number, top) {
  const levels = [];
  let start = 0;
  for (let lo = 1; lo <= top; lo *= number) {
    const count = Math.min(top, lo * number - 1) - lo + 1;
    const span = Math.min(MOST, Math.max(LEAST, count * TILE));
    levels.push({ lo, count, start, span });
    start += span;
  }
  return { levels, length: start, total: top };
}

export function index(times, t) {
  if (t <= 0) return 0;
  if (t >= times.length) return times.total;
  for (const { lo, count, start, span } of times.levels) {
    if (t < start + span) return lo - 1 + ((t - start) / span) * count;
  }
  return times.total;
}

export function phase(t, length, still) {
  if (still) return length;
  const loop = length + HOLD + FADE;
  return ((t % loop) + loop) % loop;
}

export function fade(now, length) {
  const f = Math.min(1, Math.max(0, (now - length - HOLD) / FADE));
  return (1 + Math.cos(Math.PI * f)) / 2;
}

/* CAMERA */

export function head(tiles, b) {
  const whole = Math.floor(b);
  const centre = (i) => [tiles[i].x + tiles[i].side / 2, tiles[i].y + tiles[i].side / 2];
  if (whole <= 0) return centre(0);
  if (whole >= tiles.length) return centre(tiles.length - 1);
  const [ax, ay] = centre(whole - 1);
  const [bx, by] = centre(whole);
  const f = b - whole;
  return [ax + (bx - ax) * f, ay + (by - ay) * f];
}

export function cameras(shell, w, h, cell, dpr, pad = PAD) {
  const { tiles } = shell;
  const n = tiles.length;
  const cams = new Float64Array(3 * (n + 1));
  const sums = new Float64Array(3 * (n + 1));
  const boxes = new Float64Array(4 * (n + 1));
  const room = [Math.max(1, w - 2 * pad * dpr), Math.max(1, h - 2 * pad * dpr)];
  const keep = [Math.max(1, w / 2 - (pad * dpr) / 2), Math.max(1, h / 2 - (pad * dpr) / 2)];
  let lx = Infinity;
  let ly = Infinity;
  let hx = -Infinity;
  let hy = -Infinity;
  for (let i = 0; i < n; i++) {
    const { x, y, side } = tiles[i];
    lx = Math.min(lx, x);
    ly = Math.min(ly, y);
    hx = Math.max(hx, x + side);
    hy = Math.max(hy, y + side);
    const at = 3 * (i + 1);
    cams[at] = (lx + hx) / 2;
    cams[at + 1] = (ly + hy) / 2;
    cams[at + 2] = Math.log(Math.min(room[0] / (hx - lx), room[1] / (hy - ly)));
    for (let j = 0; j < 3; j++) sums[at + j] = sums[at - 3 + j] + cams[at + j];
    boxes.set([lx, ly, hx, hy], 4 * (i + 1));
  }
  const fit = { tiles, cams, sums, boxes, keep, count: n };
  return cell > 0 ? { ...fit, scale: Math.log(cell * dpr) } : fit;
}

const at = ({ cams, sums }, x, j) => {
  const i = Math.floor(x);
  const f = x - i;
  return sums[3 * i + j] + (f > 0 ? f * cams[3 * (i + 1) + j] : 0);
};

const one = ({ cams, count }, x) => {
  const i = Math.min(count, Math.max(1, Math.ceil(x)));
  return [cams[3 * i], cams[3 * i + 1], cams[3 * i + 2]];
};

export function mean(times, lens, ta, tb) {
  if (tb - ta < 1e-6 || ta >= times.length) return one(lens, index(times, tb));
  const acc = [0, 0, 0];
  for (const { start, span, count } of times.levels) {
    const t0 = Math.max(ta, start);
    const t1 = Math.min(tb, start + span);
    if (t1 <= t0) continue;
    const i0 = index(times, t0);
    const i1 = index(times, t1);
    const ms = span / count;
    for (let j = 0; j < 3; j++) acc[j] += ms * (at(lens, i1, j) - at(lens, i0, j));
  }
  if (tb > times.length) {
    const rest = tb - Math.max(ta, times.length);
    const last = one(lens, times.total);
    for (let j = 0; j < 3; j++) acc[j] += rest * last[j];
  }
  return acc.map((v) => v / (tb - ta));
}

export function camera(times, lens, now, still = false) {
  const b = index(times, now);
  if ('scale' in lens && !still) {
    const [x, y] = head(lens.tiles, b);
    return [x, y, lens.scale];
  }
  const [cx, cy, ls] = mean(times, lens, now, now + LEAD);
  const i = Math.min(lens.count, Math.max(1, Math.ceil(b)));
  const [lx, ly, hx, hy] = lens.boxes.subarray(4 * i, 4 * i + 4);
  const reach = Math.min(lens.keep[0] / Math.max(cx - lx, hx - cx), lens.keep[1] / Math.max(cy - ly, hy - cy));
  return [cx, cy, Math.min(ls, Math.log(reach))];
}

/* SVG */

export function picture({ shell, lines }, accent, trail = true) {
  const { tiles, low, high } = shell;
  const width = high[0] - low[0];
  const height = high[1] - low[1];
  const k = SIZE / Math.max(width, height);
  const hair = fmt(1 / k, 4);
  const defs = lines.map((cell, level) => (cell ? `<g id="l${level}">${cell.map(([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`).join('')}</g>` : '')).join('');
  const marks = [];
  const frames = [];
  const points = [];
  for (const tile of tiles) {
    const x = tile.x - low[0];
    const y = high[1] - tile.y - tile.side;
    const faint = tile.prime ? '' : ` opacity="${FAINT}"`;
    points.push(x + tile.side / 2, y + tile.side / 2);
    if (!tile.level) {
      marks.push(`<rect x="${x}" y="${y}" width="1" height="1"${faint}/>`);
      continue;
    }
    marks.push(`<use href="#l${tile.level}" x="${x}" y="${y}"${faint}/>`);
    frames.push(`<rect x="${x}" y="${y}" width="${tile.side}" height="${tile.side}"${faint}/>`);
  }
  const body = [
    `<defs>${defs}</defs>`,
    `<g fill="${accent}" fill-opacity="${FILL}">${frames.join('')}</g>`,
    `<g fill="${accent}" shape-rendering="crispEdges">${marks.join('')}</g>`,
    `<g fill="none" stroke="${accent}" stroke-opacity="${EDGE}" stroke-width="${hair}">${frames.join('')}</g>`,
    trail ? `<path d="${path(points)}" fill="none" stroke="${accent}" stroke-opacity="${TRAIL}" stroke-width="${fmt(1.5 / k, 4)}" stroke-linejoin="round" stroke-linecap="round"/>` : '',
  ];
  return sheet(width * k, height * k, body.join(''), `0 0 ${width} ${height}`);
}
