import { colorizer, paint } from "../../lib/moire/engine.js";

export const SCHEDULES = [
  ["unspun", "Unspun"],
  ["degrees", "Layer k at k increments"],
  ["golden", "Golden angle"],
  ["primes", "Layer k at the k-th prime"],
  ["random", "Random, seeded"],
  ["gaussian", "Gaussian angle of n"],
];
export const SETS = [
  ["odd", "Odd"],
  ["primes", "Primes"],
  ["squarefree", "Squarefree"],
  ["prime powers", "Prime powers"],
];
export const WEIGHTS = [
  ["plain", "Plain"],
  ["mobius", "Mobius"],
  ["harmonic", "Harmonic"],
];
export const MODES = [
  ["cells", "Cells"],
  ["edges", "Edges"],
  ["corners", "Corners"],
];
export const BLENDS = [
  ["mean", "Mean"],
  ["sum", "Sum"],
  ["union", "Union"],
  ["meet", "Meet"],
  ["parity", "Parity"],
  ["difference", "Difference"],
];
export const SIZES = [256, 512, 1024];
export const SWEEP = 6;
export const NUDGE = 0.05;
export const DRAG = 90;
export const SLOP = 0.01;
export const EYES = 12;
const EYE = 1e-6;
export const BUDGET = 28 * 384 * 384;
export const LEAN = BUDGET / 3;
export const HUSH = 1.5;
export const REEL = 50;
export const LEAST = 128;
export const STAMP = 1000;
export const READ = 250;

/* TURN */

export function angle(from, ms, rate = SWEEP) {
  const turned = (from + (rate * ms) / 1000) % 360;
  return turned < 0 ? turned + 360 : turned;
}

export const settle = (x) => Math.min(360, Math.max(0, Number(Number(x).toFixed(6))));

export const place = (x, places) => (Number.isFinite(x) ? x.toFixed(places) : "none");

export const trim = (x) => String(Number(Number(x).toFixed(6)));

export function live(layers, fine, budget = BUDGET) {
  const side = 8 * Math.round(Math.sqrt(budget / Math.max(1, layers)) / 8);
  return Math.min(fine, Math.max(LEAST, side));
}

/* EYES */

export function eyeOf(eyes, increment) {
  const at = eyes.findIndex((eye) => Math.abs(eye.angle - increment) < EYE);
  return at < 0 ? "" : String(at);
}

export const label = ({ angle: turn, numer, denom }) =>
  denom === 1 ? `${trim(turn)} deg` : `${trim(turn)} deg = ${numer}/${denom}`;

export const KINDS = [
  ["unit", "Turns"],
  ["all", "All turns"],
  ["sched", "Schedules"],
];

export function shown(eyes, kind, find) {
  const want = find.trim().toLowerCase();
  return eyes
    .map((eye, at) => [eye, at])
    .filter(([eye]) => (kind === "unit" ? eye.numer <= 90 : label(eye).includes(want)));
}

/* FIELD */

export function spin(math, value, at, size) {
  const data = math.tourbillon.field(
    value.limit,
    size,
    value.sched,
    at,
    value.set,
    value.weights,
    value.mode,
    value.blend,
    value.seed,
  );
  return { field: math.moire.Field.from_data(data, size), data, size, at };
}

export const reading = (math, value, held) =>
  math.tourbillon.stats(
    held.data,
    held.size,
    value.limit,
    value.sched,
    held.at,
    value.set,
    value.weights,
    value.blend,
    value.seed,
  );

export function shade(value, stats) {
  const signed = Boolean(stats?.weighted) && value.weights === "mobius";
  const ramp = value.ramp === "auto" ? (signed ? "diverge" : "fire") : value.ramp;
  return { ramp, levels: Math.max(2, Number(value.levels) || (stats?.layers ?? 1) + 1), signed };
}

export function swatch(math, value, at, size, look) {
  const held = spin(math, value, at, size);
  const how = shade(value, reading(math, value, held));
  const out = paint(math, held, colorizer(how.ramp, how.levels, look), how.levels, value.invert, how.signed);
  held.field.free();
  return out;
}
