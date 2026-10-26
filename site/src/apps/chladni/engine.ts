import type { chladni } from "@mrlyprod/mrly/num";
import type { Scalar, Value } from "../../lib/app/spec.ts";
import { turn } from "../../lib/app/turn.ts";
import type { Paints } from "../../lib/color.ts";
import { rgb } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Rng } from "../../lib/rand.ts";
import type { Unit } from "../../lib/units/index.ts";

/* TYPES */

export type Num = Unit<"num">;

export type Units = { num?: Num | null; math?: Mrly | null };

export type Figure = Pick<chladni.Rung, "n" | "m" | "sign">;

export type Design = { code: string; number: number; level: number };

export type Asked = Figure & Design & { view: string; k: number };

export type Absent<T> = { [K in keyof T]?: undefined };

export type Made = { side: number; values: number[]; domains: number[]; bound: number[]; fields: Float64Array };

export type Fault = { error: string };

export type Solved = (Made & Absent<Fault>) | (Fault & Absent<Made>);

export type Plate = { side: number; data: Float64Array };

export type Shape = Plate & { smooth: boolean; mask: string; key: string; k?: number; made?: Made } & Absent<Fault>;

export type Missed = Fault & { key: string } & Absent<Omit<Shape, "key" | "error">>;

export type Raster = Shape | Missed;

export type Carpet = Design & { name: string };

export type Look = Pick<Paints, "paper" | "accent"> & { ink?: string };

export type Inks = { paper: number[]; accent: number[]; other: number[]; face: number; sand: number; ink: string };

export const SIDE = 128;
export const TOP = 12;
export const COUNT = 48;
const GRID = 128 * 128;
const LEVELS = 4;
export const RATE = 60;
export const REST = 180;
export const CHUNK = 64;
export const HOLD = 4000;
export const TAP = 1500;
const SPOT = 0.04;
export const POUR = 30;
export const ROOM = 3;
export const NUMBERS = [3, 5, 7, 9];
const CODES = Array.from({ length: 16 }, (_, i) => String(i));

const TRIES = 8;
const KEEP = 4;
const FACE = 0.08;
const held = new Map<string, boolean>();
const solved = new Map<string, Solved>();
const names = new Map<Scalar, string>();
let rungs: chladni.Rung[] | null = null;

/* DOORS */

export function ladder(num: Num): chladni.Rung[] {
  rungs ??= num.chladni.ladder(TOP).map(({ n, m, sign, pitch }) => ({ n, m, sign, pitch }));
  return rungs;
}

const figure = ({ n, m, sign }: Figure): Figure => ({ n: Math.min(n, m), m: Math.max(n, m), sign: n === m ? 1 : sign });

export function rungOf(num: Num, value: Figure): number {
  const want = figure(value);
  return ladder(num).findIndex((one) => one.n === want.n && one.m === want.m && one.sign === want.sign);
}

function holds(math: Mrly | null | undefined, code: Scalar, number: Scalar, level: number): boolean {
  const door = math?.nodal?.holds;
  if (!door) return true;
  const key = `${code} ${number} ${level}`;
  if (!held.has(key)) {
    let ok = false;
    try {
      ok = Boolean(door(BigInt(code), Number(number), level));
    } catch {
      ok = false;
    }
    held.set(key, ok);
  }
  return held.get(key)!;
}

const design = ({ code, number, level }: Value) => `${code} ${number} ${level}`;

function figures(math: Mrly | null | undefined, value: Design): Solved {
  const door = math?.nodal?.figures;
  if (!door) return { error: "the carpet waits for its door" };
  const key = design(value);
  if (!solved.has(key)) {
    let made: Solved;
    try {
      const out = door(BigInt(value.code), value.number, value.level, COUNT);
      made = {
        side: out.side,
        values: Array.from(out.values),
        domains: Array.from(out.domains),
        bound: Array.from(out.bound),
        fields: Float64Array.from(out.fields, (v) => v ?? NaN),
      };
    } catch (fault) {
      made = { error: String((fault as { message?: unknown } | null | undefined)?.message ?? fault) };
    }
    solved.set(key, made);
    if (solved.size > KEEP) solved.delete(solved.keys().next().value!);
  }
  return solved.get(key)!;
}

export const known = (value: Value) => solved.get(design(value))?.values?.length;

/* ROWS */

const range = (top: number) => Array.from({ length: Math.max(0, top) }, (_, i) => i + 1);

const capOf = (math: Mrly | null | undefined, number: Scalar) =>
  math ? math.bang.level_cap(Number(number), 2, GRID) : LEVELS;

const named = (math: Mrly | null | undefined, code: Scalar) => {
  if (!math) return `code ${code}`;
  if (!names.has(code)) names.set(code, math.name.of(code, 2, 2) ?? `code ${code}`);
  return names.get(code)!;
};

const holding = (math: Mrly | null | undefined, value: Value) =>
  range(capOf(math, value.number)).filter((level) => holds(math, value.code, value.number, level));

export function codes(value: Value, units?: Units): [string, string][] {
  const math = units?.math;
  const all = CODES.map((code): [string, string] => [code, named(math, code)]);
  return math ? all.filter(([code]) => holding(math, { ...value, code }).length) : all;
}

export const lowest = (value: Value, units?: Units) => (units?.math ? (holding(units.math, value)[0] ?? 1) : 1);

export const highest = (value: Value, units?: Units) =>
  units?.math ? (holding(units.math, value).at(-1) ?? 1) : LEVELS;

/* SHELVES */

const SHOWN = 25;

export function carpets(math: Mrly): Carpet[] {
  return NUMBERS.flatMap((number) =>
    CODES.flatMap((code) => {
      const levels = holding(math, { code, number });
      const level = levels.find((one) => number ** one >= SHOWN) ?? levels.at(-1);
      return level ? [{ number, code, level, name: named(math, code) }] : [];
    }),
  );
}

export const cell = (math: Mrly, one: Design) => math.two.create(one.code, one.number, one.level, 0, 2);

export function hits(one: Figure, query: string): boolean {
  return (query.match(/\d+/g) ?? []).every((word) => Number(word) === one.n || Number(word) === one.m);
}

/* RASTER */

const kOf = (value: Asked) => Math.max(1, Math.min(value.k, known(value) || value.k));

export const keyOf = (value: Asked) =>
  value.view === "carpet"
    ? `carpet ${design(value)} ${kOf(value)}`
    : `plate ${value.n} ${value.m} ${value.n === value.m ? 1 : value.sign}`;

export function raster(units: { num: Num; math?: Mrly | null }, value: Asked): Raster {
  if (value.view === "carpet") {
    const made = figures(units.math, value);
    const key = keyOf(value);
    if (made.error) return { error: made.error, key };
    const ok = made as Made;
    if (!ok.values.length) return { error: "the design has no figures", key };
    const k = kOf(value);
    const size = ok.side * ok.side;
    return {
      side: ok.side,
      data: ok.fields.subarray((k - 1) * size, k * size),
      smooth: false,
      mask: `carpet ${design(value)}`,
      key,
      k,
      made: ok,
    };
  }
  const { n, m } = value;
  return {
    side: SIDE,
    data: units.num.chladni.plate(n, m, n === m ? 1 : value.sign, SIDE),
    smooth: true,
    mask: "plate",
    key: keyOf(value),
  };
}

const on = (data: ArrayLike<number>, side: number, x: number, y: number) => {
  const at = Math.min(side - 1, Math.floor(y * side)) * side + Math.min(side - 1, Math.floor(x * side));
  return !Number.isNaN(data[at]);
};

/* SAND */

export function sprinkle(rand: Rng, data: ArrayLike<number>, side: number, count: number): Float32Array {
  const grains = new Float32Array(2 * count).fill(NaN);
  for (let i = 0; i < count; i++) {
    for (let k = 0; k < TRIES; k++) {
      const x = rand();
      const y = rand();
      if (!on(data, side, x, y)) continue;
      grains[2 * i] = x;
      grains[2 * i + 1] = y;
      break;
    }
  }
  return grains;
}

export function live(grains: ArrayLike<number>): number {
  let n = 0;
  for (let i = 0; i < grains.length; i += 2) if (!Number.isNaN(grains[i])) n++;
  return n;
}

export function scatter(
  rand: Rng,
  x: number,
  y: number,
  count: number,
  data: ArrayLike<number>,
  side: number,
): number[] {
  const out: number[] = [];
  for (let i = 0; i < TRIES * count && out.length < 2 * count; i++) {
    const dx = 2 * rand() - 1;
    const dy = 2 * rand() - 1;
    if (dx * dx + dy * dy > 1) continue;
    const px = x + dx * SPOT;
    const py = y + dy * SPOT;
    if (px < 0 || py < 0 || px >= 1 || py >= 1 || !on(data, side, px, py)) continue;
    out.push(px, py);
  }
  return out;
}

export function fill(grains: Float32Array, points: Float32Array | number[], cap: number): Float32Array {
  const take = Math.min(points.length / 2, Math.max(0, cap - live(grains)));
  let put = 0;
  for (let i = 0; i < grains.length && put < take; i += 2) {
    if (!Number.isNaN(grains[i])) continue;
    grains[i] = points[2 * put];
    grains[i + 1] = points[2 * put + 1];
    put++;
  }
  if (put === take) return grains;
  const grown = new Float32Array(grains.length + 2 * (take - put));
  grown.set(grains);
  grown.set(points.slice(2 * put, 2 * take), grains.length);
  return grown;
}

/* PAINT */

const pack: (...rgba: number[]) => number = (r, g, b, a = 255) =>
  new Uint32Array(Uint8ClampedArray.from([r, g, b, a]).buffer)[0];

const mix = (a: readonly number[], b: readonly number[], k: number) => a.map((v, i) => v + (b[i] - v) * k);

export function inks(look: Look): Inks {
  const paper = rgb(look.paper);
  const accent = rgb(look.accent);
  return {
    paper,
    accent,
    other: rgb(turn(look.accent, 180)),
    face: pack(...mix(paper, accent, FACE)),
    sand: pack(...accent),
    ink: look.ink ?? look.accent,
  };
}

function ramp(inks: Inks, f: number): number[] {
  if (Number.isNaN(f)) return [0, 0, 0, 0];
  return [...mix(inks.paper, f >= 0 ? inks.accent : inks.other, Math.min(1, Math.abs(f))), 255];
}

export function base(shape: Pick<Plate, "data">, inks: Inks): Uint32Array {
  return Uint32Array.from(shape.data, (v) => (!Number.isNaN(v) ? inks.face : 0));
}

export function stamp(words: Uint32Array, grains: ArrayLike<number>, side: number, colour: number): Uint32Array {
  for (let i = 0; i < grains.length; i += 2) {
    const x = grains[i];
    if (Number.isNaN(x)) continue;
    const px = Math.min(side - 1, Math.floor(x * side));
    const py = Math.min(side - 1, Math.floor(grains[i + 1] * side));
    words[py * side + px] = colour;
  }
  return words;
}

export function paint(shape: Plate, inks: Inks) {
  const bytes = new Uint8ClampedArray(shape.side * shape.side * 4);
  for (let i = 0; i < shape.data.length; i++) bytes.set(ramp(inks, shape.data[i]), 4 * i);
  return bytes;
}

export const swatch = (num: Num, one: Figure, inks: Inks, side: number) =>
  paint({ side, data: num.chladni.plate(one.n, one.m, one.n === one.m ? 1 : one.sign, side) }, inks);
