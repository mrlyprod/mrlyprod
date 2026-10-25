import type { tourbillon } from "@mrlyprod/mrly/math";
import type { Scalar } from "../../lib/app/spec.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Held, Look, Painted, Ramp } from "../../lib/moire/engine.ts";
import { colorizer, paint } from "../../lib/moire/engine.ts";

/* TYPES */

export type Stack = {
  limit: number;
  sched: string;
  set: string;
  weights: string;
  mode: string;
  blend: string;
  seed: number;
};

export type Tones = { weights: string; ramp: Ramp | "auto"; levels: number };

export type Swatched = Stack & Tones & { invert: Scalar };

export type Spun = Held & { at: number };

export type Shade = { ramp: Ramp; levels: number; signed: boolean };

export const SCHEDULES: [string, string][] = [
  ["unspun", "Unspun"],
  ["degrees", "Layer k at k increments"],
  ["golden", "Golden angle"],
  ["primes", "Layer k at the k-th prime"],
  ["random", "Random, seeded"],
  ["gaussian", "Gaussian angle of n"],
];
export const SETS: [string, string][] = [
  ["odd", "Odd"],
  ["primes", "Primes"],
  ["squarefree", "Squarefree"],
  ["prime powers", "Prime powers"],
];
export const WEIGHTS: [string, string][] = [
  ["plain", "Plain"],
  ["mobius", "Mobius"],
  ["harmonic", "Harmonic"],
];
export const MODES: [string, string][] = [
  ["cells", "Cells"],
  ["edges", "Edges"],
  ["corners", "Corners"],
];
export const BLENDS: [string, string][] = [
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

export function angle(from: number, ms: number, rate = SWEEP): number {
  const turned = (from + (rate * ms) / 1000) % 360;
  return turned < 0 ? turned + 360 : turned;
}

export const settle = (x: number) => Math.min(360, Math.max(0, Number(Number(x).toFixed(6))));

export const place = (x: number, places: number) => (Number.isFinite(x) ? x.toFixed(places) : "none");

export const trim = (x: Scalar) => String(Number(Number(x).toFixed(6)));

export function live(layers: number, fine: number, budget = BUDGET): number {
  const side = 8 * Math.round(Math.sqrt(budget / Math.max(1, layers)) / 8);
  return Math.min(fine, Math.max(LEAST, side));
}

/* EYES */

export function eyeOf(eyes: readonly tourbillon.Eye[], increment: number): string {
  const at = eyes.findIndex((eye) => Math.abs(eye.angle - increment) < EYE);
  return at < 0 ? "" : String(at);
}

export const label = ({ angle: turn, numer, denom }: tourbillon.Eye) =>
  denom === 1 ? `${trim(turn)} deg` : `${trim(turn)} deg = ${numer}/${denom}`;

export const KINDS: [string, string][] = [
  ["unit", "Turns"],
  ["all", "All turns"],
  ["sched", "Schedules"],
];

export function shown(eyes: readonly tourbillon.Eye[], kind: Scalar, find: string): [tourbillon.Eye, number][] {
  const want = find.trim().toLowerCase();
  return eyes
    .map((eye, at): [tourbillon.Eye, number] => [eye, at])
    .filter(([eye]) => (kind === "unit" ? eye.numer <= 90 : label(eye).includes(want)));
}

/* FIELD */

export function spin(math: Mrly, value: Stack, at: number, size: number): Spun {
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

export const reading = (math: Mrly, value: Stack, held: Spun) =>
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

export function shade(value: Tones, stats: tourbillon.Stats | null): Shade {
  const signed = Boolean(stats?.weighted) && value.weights === "mobius";
  const ramp = value.ramp === "auto" ? (signed ? "diverge" : "fire") : value.ramp;
  return { ramp, levels: Math.max(2, Number(value.levels) || (stats?.layers ?? 1) + 1), signed };
}

export function swatch(math: Mrly, value: Swatched, at: number, size: number, look: Look): Painted {
  const held = spin(math, value, at, size);
  const how = shade(value, reading(math, value, held));
  const out = paint(math, held, colorizer(how.ramp, how.levels, look), how.levels, value.invert, how.signed);
  held.field.free();
  return out;
}
