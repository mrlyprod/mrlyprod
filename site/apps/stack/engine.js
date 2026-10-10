import { pick, rgb } from '../../lib/scene.js';
import { named, resolve, total, word } from '../designs/engine.js';

export const KINDS = [['tourbillon', 'Tourbillon'], ['moire', 'Moire'], ['star', 'Star']];
export const COMBINES = [['sum', 'Sum'], ['and', 'And'], ['xor', 'Xor'], ['hive', 'Hive']];
export const RAMPS = [['tint', 'Tint'], ['ink', 'Ink'], ['bands', 'Bands']];
export const SETS = ['odd', 'primes', 'squarefree', 'prime powers'];
export const WEIGHTS = ['plain', 'mobius', 'harmonic'];
export const CARPET = '23';
export const SWEEP = 4;
export const BEAT = 350;
export const HOLD = 3000;
export const LIVE = 192;
export const FINE = 384;
export const LEAST = 96;
export const LAYERS = 21;
export const READ = 128;
export const DEEPEST = 5;
export const EYES = 6;
export const EYE = 1e-6;
const DIM = 2;
const MERGE = { sum: 'Sum', and: 'And', xor: 'Xor', hive: 'Sum' };
const BLEND = { sum: 'mean', and: 'meet', xor: 'parity', hive: 'mean' };
const FOLD = { Sum: (a, b) => a + b, And: (a, b) => a * b, Xor: (a, b) => (a + b) % 2 };
const SCHEDULE = 'degrees';
const MODE = 'cells';
const DIAGONAL = [1, 1, 1];
const MIDDLE = 0.5;

/* TIME */

export function odd(limit) {
  const top = Math.max(1, Math.floor(limit)) | 1;
  return Array.from({ length: (top + 1) / 2 }, (_, i) => 2 * i + 1);
}

export function angle(increment, t, still) {
  if (still) return increment;
  const turned = (increment + (SWEEP * t) / 1000) % 360;
  return turned < 0 ? turned + 360 : turned;
}

export function shown(n, t, still) {
  if (still || n <= 1) return n;
  const loop = n * BEAT + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(n, 1 + Math.floor(phase / BEAT));
}

export function live(layers) {
  const side = 2 * Math.round((LIVE * Math.sqrt(LAYERS / Math.max(1, layers))) / 2);
  return Math.min(LIVE, Math.max(LEAST, side));
}

/* COLOUR */

const hex = (c) => `#${c.map((v) => Math.round(Math.min(255, Math.max(0, v))).toString(16).padStart(2, '0')).join('')}`;

const contrast = ([r, g, b]) => ((r * 299 + g * 587 + b * 114) / 255000 > 0.5 ? [0, 0, 0] : [255, 255, 255]);

const along = (stops, t) => {
  const k = t * (stops.length - 1);
  const i = Math.min(stops.length - 2, Math.floor(k));
  const f = k - i;
  return stops[i].map((v, c) => v + (stops[i + 1][c] - v) * f);
};

export function stops(ramp, paper, accent) {
  const a = rgb(paper);
  const b = rgb(accent);
  return ramp === 'ink' ? [a, b, contrast(a)] : [a, b];
}

export function bins(ramp, levels, paper, accent) {
  const n = Math.max(2, Math.floor(levels));
  const list = stops(ramp, paper, accent);
  if (ramp === 'bands') return Array.from({ length: n }, (_, i) => hex(list[i % 2]));
  return Array.from({ length: n }, (_, i) => hex(along(list, i / (n - 1))));
}

export function colorizer(ramp, levels, { paper, accent }) {
  return { Bins: { background: hex(rgb(paper)), ramp: bins(ramp, levels, paper, accent) } };
}

/* STUDY */

export function design(math, value, rand) {
  const base = Number(value.base) === 2 ? 2 : 3;
  const code = String(resolve(value.code, total(math, DIM, base), rand));
  const level = Math.min(DEEPEST, Math.max(1, Math.round(Number(value.level) || 1)));
  return { base, code, level, name: word(named(math, DIM, base), code) };
}

const spin = (math, plan, size) => math.tourbillon.field(plan.limit, size, SCHEDULE, plan.increment, plan.set, plan.weights, MODE, BLEND[plan.combine], plan.seed);

export function spun(rand) {
  return { set: pick(rand, SETS), weights: pick(rand, WEIGHTS) };
}

export function study(math, value, rand) {
  const { kind, limit, combine, seed } = value;
  const numbers = odd(limit);
  const n = numbers.length;
  if (kind === 'moire') {
    const made = design(math, value, rand);
    const witness = math.moire.pairs.witness(limit);
    const spec = math.moire.Spec.new(made.code, made.base, DIM);
    return { kind, numbers, spec, level: made.level, merge: MERGE[combine], lattice: combine === 'hive' ? 'Hex' : 'Square', facts: { kind, ...made, layers: n, scales: numbers, limit, max: witness.max, at: witness.at, prime: witness.prime } };
  }
  if (kind === 'tourbillon') {
    const { set, weights } = spun(rand);
    const plan = { kind, limit, increment: value.increment, combine, seed, set, weights };
    const layers = math.tourbillon.layers(limit, SCHEDULE, value.increment, set, weights, seed);
    const [classes, pairs] = math.tourbillon.sharing(layers);
    const read = math.tourbillon.stats(spin(math, plan, READ), READ, limit, SCHEDULE, value.increment, set, weights, BLEND[combine], seed);
    const scales = layers.map((layer) => layer.scale);
    return { ...plan, numbers, layers: layers.length, facts: { kind, layers: layers.length, scales, limit, set, weights, period: math.tourbillon.period(value.increment) ?? null, classes, pairs, mean: read.mean, rms: read.rms, centre: read.centre } };
  }
  const star = new math.six.star.Star(CARPET);
  const decay = math.six.star.decay(star.excesses(n, 0), n);
  star.free();
  const branch = math.six.star.Branch.of(n);
  return { kind, numbers, merge: MERGE[combine], facts: { kind, layers: n, scales: numbers, limit, scaled: decay.scaled, logged: decay.logged, slope: decay.slope ?? null, branch: math.six.star.Branch.name(branch) } };
}

/* FIELD */

export function field(math, plan, at, size) {
  if (plan.kind === 'moire') {
    const made = math.moire.stack(plan.spec, plan.numbers.slice(0, at), plan.merge, plan.level, plan.lattice, size, []);
    return { field: made, data: made.data, size };
  }
  if (plan.kind === 'tourbillon') {
    const data = spin(math, { ...plan, increment: at }, size);
    return { field: math.moire.Field.from_data(data, size), data, size };
  }
  const spec = math.moire.Spec.new(CARPET, 2, 3);
  const frame = math.moire.frame(DIAGONAL, MIDDLE);
  const fold = FOLD[plan.merge];
  const data = new Float32Array(size * size).fill(plan.merge === 'And' ? 1 : 0);
  let inside = null;
  for (const n of plan.numbers.slice(0, at)) {
    const volume = math.moire.volume(spec, [n], plan.merge, 1, n);
    const [cut, mask] = volume.plane(frame, size);
    volume.free();
    inside ??= mask;
    for (let i = 0; i < data.length; i++) data[i] = fold(data[i], cut[i]);
  }
  for (let i = 0; i < data.length; i++) if (!inside?.[i]) data[i] = NaN;
  return { field: math.moire.Field.from_data(data, size), data, size };
}

export function paint(math, { field: made, data }, colorizer, levels, invert) {
  const image = math.moire.render(made, colorizer, Math.max(2, Math.floor(levels)), false, Number(invert) === 1, 1);
  const colors = image.colors;
  for (let i = 0; i < data.length; i++) if (Number.isNaN(data[i])) colors[i * 4 + 3] = 0;
  return image;
}

/* EYES */

export function eyeOf(eyes, increment) {
  const at = eyes.findIndex((eye) => Math.abs(eye.angle - increment) < EYE);
  return at < 0 ? '' : String(at);
}

const trim = (x) => String(Number(x.toFixed(6)));

export function label({ angle: turn, numer, denom }) {
  return denom === 1 ? `${trim(turn)} deg` : `${trim(turn)} deg, ${numer}/${denom}`;
}
