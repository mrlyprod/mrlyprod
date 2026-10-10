export const KINDS = [['in', 'Inside a circle'], ['out', 'Outside a circle'], ['line', 'A line'], ['polyin', 'Inside a polygon'], ['polyout', 'Outside a polygon']];
export const MODES = [['fill', 'Filled cells'], ['void', 'Empty cells'], ['both', 'Both'], ['corners', 'Corners']];
export const NUMBERS = [3, 5, 7];
export const BASE = 3;
export const CODES = 512;
export const BANDS = 8;
export const PAD = 24;
export const PACE = 0.5;
export const HOLD = 3000;
export const RISE = 1000;
export const LEAST = 3000;
export const MOST = 60000;
export const LIVE = 480000;
export const DENSITY = 240;
export const COVER = { most: 64, side: 256, samples: 720 };
const FEW = 3;
const INSET = 1e-9;
const TAU = Math.PI * 2;

const attempt = (fn) => {
  try {
    return fn();
  } catch {
    return null;
  }
};

/* DESIGN */

export function bits(code) {
  let n = 0;
  for (let c = code; c; c >>>= 1) n += c & 1;
  return n;
}

export function roll(rand) {
  let code;
  do code = 1 + Math.floor(rand() * (CODES - 2));
  while (bits(code) < FEW);
  return code;
}

export function design(math, value, rand) {
  const typed = String(value.code ?? '').trim();
  const rolled = String(roll(rand));
  return typed && attempt(() => math.two.create(typed, value.number, 1, 0, BASE)) ? typed : rolled;
}

export function seat(math, value, code) {
  const s = math.spirograph;
  for (let level = value.level; ; level--) {
    const cell = math.two.create(code, value.number, level, 0, BASE);
    const [height, width] = cell.shape;
    const sit = () => s.pencils(cell.types, width, height, value.mode, value.reach, value.jitter, value.seed >>> 0);
    const pencils = level > 1 ? attempt(sit) : sit();
    if (pencils) return { cell, level, width, height, pencils };
  }
}

/* TRACK */

export function legal(kind, ring, wheel, sides) {
  if (kind === 'in') {
    const r = Math.max(ring, 2);
    return [r, Math.min(wheel, r - 1)];
  }
  if (kind !== 'polyin') return [ring, wheel];
  const cos = Math.cos(Math.PI / sides);
  const r = Math.max(ring, Math.ceil((1 + INSET) / cos));
  return [r, Math.min(wheel, Math.floor(r * cos - INSET))];
}

/* STUDY */

export function bands(pencils) {
  let lo = Infinity;
  let hi = -Infinity;
  const radii = pencils.map((p) => Math.hypot(p.x, p.y));
  for (const r of radii) {
    lo = Math.min(lo, r);
    hi = Math.max(hi, r);
  }
  return radii.map((r) => (hi > lo ? Math.min(BANDS - 1, Math.floor(((r - lo) / (hi - lo)) * BANDS)) : BANDS - 1));
}

export function study(math, value, rand) {
  const s = math.spirograph;
  const code = design(math, value, rand);
  const { cell, level, width, height, pencils } = seat(math, value, code);
  const [ring, wheel] = legal(value.kind, value.ring, value.wheel, value.sides);
  const track = s.track(value.kind, ring, wheel, value.sides, value.laps);
  const exact = value.jitter === 0;
  const n = pencils.length;
  const turns = Math.abs(s.turn(track, track.total)) / TAU;
  const beats = Math.max(turns, track.orbits);
  const samples = n ? Math.max(2, Math.min(Math.floor(s.POINT_CAP() / n), Math.floor(LIVE / n), Math.ceil(DENSITY * beats) + 1)) : 2;
  const trace = n ? s.trace(track, pencils, samples) : new Float32Array(0);
  const frame = Array.from(s.frame(track, pencils));
  const seats = s.seats(pencils);
  const distinct = n ? s.distinct(track, pencils, exact) : 0;
  const nodes = n ? s.nodes(track, pencils, exact) : undefined;
  const round = value.kind === 'in' || value.kind === 'out';
  const shape = round && n && distinct <= COVER.most ? attempt(() => ({ ...s.cover(track, pencils, exact, COVER.samples * track.orbits, COVER.side), disc: s.disc(track, pencils) })) : null;
  const facts = { code, level, width, height, pencils: n, fills: seats.fills, voids: seats.voids, corners: seats.corners, distinct, nodes: nodes === undefined ? null : Number(nodes), cover: shape ? shape.covered : null, ratio: [...track.ratio], orbits: track.orbits, turns, round };
  return { code, level, cell, track, pencils, samples, trace, frame, side: s.cell(width, height, value.reach), turns, beats, shape, bands: bands(pencils), facts };
}

/* TIME */

export function span(beats) {
  return Math.min(MOST, Math.max(LEAST, (beats / PACE) * 1000));
}

export function share(t, length, still) {
  if (still) return 1;
  const loop = length + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, phase / length);
}

export function shade(t, length, still) {
  if (still) return 1;
  const loop = length + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, Math.max(0, (phase - length) / RISE));
}

export function index(part, samples) {
  return Math.round(part * (samples - 1));
}

/* FIT */

export function fit(frame, w, h, pad) {
  const [x0, y0, x1, y1] = frame;
  const gap = Math.min(pad, Math.min(w, h) / 4);
  const k = Math.min((w - 2 * gap) / (x1 - x0 || 1), (h - 2 * gap) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}
