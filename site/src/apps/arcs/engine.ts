import type { Scalar } from "../../lib/app/spec.ts";
import { num as fix, runs, sheet } from "../../lib/app/svg.ts";
import type { Code, Mrly } from "../../lib/designs/engine.ts";
import { codes, total } from "../../lib/designs/engine.ts";
import type { Rng } from "../../lib/rand.ts";

/* TYPES */

type Absent<T> = { [K in keyof T]?: undefined };

export type Lawful = { base: number; code: string; name: string; formula: string };

export type Arc = { x: number; y: number; cx: number; cy: number; av: number; ah: number; sweep: number };

export type Curves = {
  side: number;
  cells: Uint8Array;
  filled: Uint8Array;
  order: Int32Array;
  dir: Int8Array;
  bounds: Int32Array;
  closed: Uint8Array;
  loops: number;
  strands: number;
};

export type Clock = {
  span: number;
  dur: Float64Array;
  start: Float64Array;
  end: Float64Array;
  byStart: Int32Array;
  byEnd: Int32Array;
};

export type Law = { formula: string; loops: number };

export type Facts = {
  code: string;
  name: string;
  base: number;
  level: number;
  side: number;
  cells: number;
  filled: number;
  loops: number;
  strands: number;
  law: Law | null;
  fault?: undefined;
};

export type Fault = Absent<Omit<Facts, "code" | "base" | "level" | "fault">> & {
  code: string;
  base: number;
  level: number;
  fault: string;
};

export type Told = Facts | Fault;

export type Plan = Curves & { code: string; lit: Int32Array; clock: Clock; facts: Facts };

export type Broken = { code: string; facts: Fault; side?: undefined };

export type Studied = Plan | Broken;

export type Asked = { base: Scalar; code?: Scalar | null; level: number };

export type Fit = { px: number; x: number; y: number };

export type Look = "all" | "loops" | "strands";

export type Drawn = { look: Look; width: number; cells: number };

export type Shades = { loop: string; strand: string; dim: string; fill: string };

export type Inks = Pick<Shades, "fill" | "strand" | "loop">;

export const DIM = 2;
export const BUDGET = 65536;
export const BASES: [number, string][] = [
  [2, "2"],
  [3, "3"],
  [4, "4"],
  [5, "5"],
];
export const LOOKS: [Look, string][] = [
  ["all", "All"],
  ["loops", "Loops"],
  ["strands", "Strands"],
];
export const CANON = "carpet";
export const UNIT = 10;
export const PAD = 16;
export const AUTO = 0.16;
export const THIN = 0.6;
export const THICK = 3;
export const BEAT = 400;
export const PACE = 40;
export const LEAST = 2000;
export const MOST = 12000;
export const FADE = 0.55;
export const TRIES = 64;
export const KEEP = 8;
const HALF = Math.PI / 2;

/* ROOM */

export const cap = (math: Mrly, base: Scalar) => math.bang.level_cap(Number(base), DIM, BUDGET);

/* DESIGN */

export const names = (math: Mrly, base: number) => math.name.words(DIM, base);

export function resolve(math: Mrly, base: number, text: Scalar | null | undefined): string {
  const raw = String(text ?? "").trim();
  const words = names(math, base);
  if (raw === "") return (words.find(([, word]) => word === CANON) ?? words[0])[0];
  return words.find(([, word]) => word === raw.toLowerCase())?.[0] ?? raw;
}

const loops = (math: Mrly, code: Code, base: number, level: number) =>
  math.arcs.draw(String(code), base, level).loops > 0;

export function looping(math: Mrly, base: number, level: number, rand: Rng = Math.random): string {
  const count = total(math, DIM, base);
  let code = 1;
  for (let i = 0; i < TRIES; i++) {
    code = 1 + Math.floor(rand() * (count - 1));
    if (loops(math, code, base, level)) return String(code);
  }
  for (let k = 1; k < count - 1; k++) {
    const one = ((code - 1 + k) % (count - 1)) + 1;
    if (loops(math, one, base, level)) return String(one);
  }
  return String(code);
}

const lawful = new Map<Mrly, Lawful[]>();

export function laws(math: Mrly): Lawful[] {
  if (!lawful.has(math)) {
    const found = BASES.flatMap(([base]) =>
      (codes(math, DIM, base) ?? names(math, base).map(([code]) => code)).flatMap((code) => {
        const law = math.arcs.law(code, base, 1);
        return law && law.formula !== "0"
          ? [{ base, code, name: math.name.of(code, DIM, base) ?? "", formula: law.formula }]
          : [];
      }),
    );
    lawful.set(math, found);
  }
  return lawful.get(math)!;
}

/* ARCS */

export function corner(filled: number, which: number, x: number, y: number): [number, number] {
  if (which) return filled ? [x + 1, y + 1] : [x, y + 1];
  return filled ? [x, y] : [x + 1, y];
}

export function arc(plan: Pick<Curves, "side" | "cells">, a: number): Arc {
  const { side, cells } = plan;
  const i = a >> 1;
  const x = i % side;
  const y = (i - x) / side;
  const [cx, cy] = corner(cells[i] & 1, a & 1, x, y);
  const right = cx !== x;
  const down = cy !== y;
  return { x, y, cx, cy, av: down ? -HALF : HALF, ah: right ? Math.PI : 0, sweep: right !== down ? HALF : -HALF };
}

export function curves(math: Mrly, code: Code, base: number, level: number): Curves {
  const drawn = math.arcs.draw(code, base, level);
  const list = math.arcs.curves(code, base, level);
  const order = new Int32Array(list.reduce((sum, one) => sum + one.arcs.length, 0));
  const dir = new Int8Array(order.length);
  const bounds = new Int32Array(list.length + 1);
  const closed = new Uint8Array(list.length);
  list.forEach((one, k) => {
    order.set(one.arcs, bounds[k]);
    dir.set(one.dir, bounds[k]);
    bounds[k + 1] = bounds[k] + one.arcs.length;
    closed[k] = one.closed ? 1 : 0;
  });
  const cells = Uint8Array.from(drawn.cells);
  return {
    side: drawn.side,
    cells,
    filled: cells.map((byte) => byte & 1),
    order,
    dir,
    bounds,
    closed,
    loops: drawn.loops,
    strands: drawn.strands,
  };
}

/* STUDY */

const kept = new Map<string, { math: Mrly; plan: Plan }>();

function plan(math: Mrly, code: string, base: number, level: number): Plan {
  const drawn = curves(math, code, base, level);
  const law = math.arcs.law(code, base, level);
  const lit: number[] = [];
  for (let k = 0; k < drawn.closed.length; k++) if (drawn.closed[k]) lit.push(k);
  const clock = schedule(lit.map((k) => drawn.bounds[k + 1] - drawn.bounds[k]));
  let filled = 0;
  for (const on of drawn.filled) filled += on;
  const facts: Facts = {
    code,
    name: math.name.of(code, DIM, base) ?? "",
    base,
    level,
    side: drawn.side,
    cells: drawn.side * drawn.side,
    filled,
    loops: drawn.loops,
    strands: drawn.strands,
    law: law ? { formula: law.formula, loops: Number(law.loops) } : null,
  };
  return { ...drawn, code, lit: Int32Array.from(lit), clock, facts };
}

export function study(math: Mrly, value: Asked): Studied {
  const base = Number(value.base);
  const level = Math.max(1, Math.min(value.level, cap(math, base)));
  const code = resolve(math, base, value.code);
  const key = `${base}:${code}:${level}`;
  const hit = kept.get(key);
  if (hit && hit.math === math) {
    kept.delete(key);
    kept.set(key, hit);
    return hit.plan;
  }
  let made: Plan;
  try {
    made = plan(math, code, base, level);
  } catch (error) {
    return {
      code,
      facts: {
        code,
        base,
        level,
        fault: String((error as { message?: unknown } | null | undefined)?.message ?? error),
      },
    };
  }
  kept.set(key, { math, plan: made });
  while (kept.size > KEEP) kept.delete(kept.keys().next().value!);
  return made;
}

/* TIME */

export function schedule(lengths: readonly number[]): Clock {
  const n = lengths.length;
  const dur = Float64Array.from(lengths, (len) => BEAT + (len / PACE) * 1000);
  const sum = dur.reduce((a, b) => a + b, 0);
  const span = n ? Math.min(MOST, Math.max(LEAST, sum)) : 0;
  const start = new Float64Array(n);
  const end = new Float64Array(n);
  let before = 0;
  for (let k = 0; k < n; k++) {
    const rest = sum - dur[k];
    dur[k] = Math.min(dur[k], span);
    start[k] = rest > 0 ? Math.max(0, (before * (span - dur[k])) / rest) : 0;
    end[k] = Math.min(span, start[k] + dur[k]);
    before += dur[k];
  }
  const by = (times: Float64Array) => Int32Array.from(times.keys()).sort((a, b) => times[a] - times[b] || a - b);
  return { span, dur, start, end, byStart: by(start), byEnd: by(end) };
}

/* LAYOUT */

export function fit(side: number, w: number, h: number, pad: number): Fit {
  const px = Math.max(1, Math.min(w, h) - 2 * pad) / side;
  return { px, x: (w - px * side) / 2, y: (h - px * side) / 2 };
}

export function thick(width: number, px: number, dpr: number): number {
  if (width > 0) return Math.max(dpr, width * px);
  return Math.max(THIN * dpr, Math.min(THICK * dpr, AUTO * px));
}

/* SVG */

export const bend = (r: number, x: number, y: number, clockwise: boolean) =>
  `A${fix(r)} ${fix(r)} 0 0 ${clockwise ? 1 : 0} ${fix(x)} ${fix(y)}`;

export function trail(plan: Curves, k: number, unit: number): string {
  const { order, dir, bounds } = plan;
  const r = unit / 2;
  let d = "";
  for (let j = bounds[k]; j < bounds[k + 1]; j++) {
    const g = arc(plan, order[j]);
    const v = [g.cx * unit, (g.y + 0.5) * unit];
    const h = [(g.x + 0.5) * unit, g.cy * unit];
    const [from, to] = dir[j] > 0 ? [v, h] : [h, v];
    if (j === bounds[k]) d += `M${fix(from[0])} ${fix(from[1])}`;
    d += bend(r, to[0], to[1], dir[j] > 0 ? g.sweep > 0 : g.sweep < 0);
  }
  return plan.closed[k] ? `${d}Z` : d;
}

export function picture(plan: Curves, value: Drawn, shades: Inks): string {
  const { side, closed, filled } = plan;
  const w = side * UNIT;
  const body: string[] = [];
  if (value.cells) {
    const boxes = runs({ shape: [side, side], types: filled }).map(
      ([row, col, len]) =>
        `<rect x="${fix(col * UNIT)}" y="${fix(row * UNIT)}" width="${fix(len * UNIT)}" height="${fix(UNIT)}"/>`,
    );
    if (boxes.length) body.push(`<g fill="${shades.fill}">${boxes.join("")}</g>`);
  }
  const strands: string[] = [];
  const loops: string[] = [];
  for (let k = 0; k < closed.length; k++) (closed[k] ? loops : strands).push(trail(plan, k, UNIT));
  const stroke = (d: string, color: string) =>
    `<path d="${d}" fill="none" stroke="${color}" stroke-width="${fix((value.width || AUTO) * UNIT)}" stroke-linecap="round" stroke-linejoin="round"/>`;
  if (value.look !== "loops" && strands.length) body.push(stroke(strands.join(""), shades.strand));
  if (value.look !== "strands" && loops.length) body.push(stroke(loops.join(""), shades.loop));
  return sheet(w, w, `<g transform="matrix(1 0 0 -1 0 ${fix(w)})">${body.join("")}</g>`);
}
