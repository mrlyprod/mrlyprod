import type { gauss } from "@mrlyprod/mrly/num";
import type { Value } from "../../lib/app/spec.ts";
import { num as fix, runs, sheet } from "../../lib/app/svg.ts";
import type { Paints } from "../../lib/color.ts";
import { rgb, veil } from "../../lib/color.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Unit } from "../../lib/units/index.ts";

/* TYPES */

export type Num = Unit<"num">;

export type Word = "gaussian" | "eisenstein";

export type Pair = [number, number];

export type Setting = { ring: Word; limit: number; look?: string; click?: string };

export type Lens = { id: string; name: string; note: string; set: Setting };

export type Prime = { ring: Word; a: number; b: number; norm: number; fate: number };

export type Entry = { id: string; label: string; sub: string; set: Setting; on: boolean };

export type Survey = ReturnType<typeof survey>;

export type Plan = ReturnType<typeof cut>;

export type Facts = Plan["facts"];

export type Point = NonNullable<ReturnType<typeof hit>>;

export type Box = ReturnType<typeof lay>;

export type Ink = ReturnType<typeof inks>;

type Kept = { num: Num; word: Word; r: number; plan: Plan };

export const RINGS: [Word, string][] = [
  ["gaussian", "Gaussian, a + bi"],
  ["eisenstein", "Eisenstein, a + bω"],
];
export const LOOKS: [string, string][] = [
  ["fate", "Fate"],
  ["norm", "Norm"],
  ["plain", "Plain"],
];
export const LETTERS: Record<Word, string> = { gaussian: "i", eisenstein: "ω" };
export const FATES = ["composite", "split", "inert", "ramified", "unit", "zero"];
export const REACH: Record<Word, number> = { gaussian: 200, eisenstein: 231 };
export const SPAN: Pair = [20, 120];
export const PEEK = 20;
export const THUMB = 14;
export const BANDS = 12;
export const GROW = 8000;
export const PAD = 16;
export const UNIT = 10;
export const INSET = 0.1;
const CLASSES: Record<gauss.Class, number> = { Composite: 0, Split: 1, Inert: 2, Ramified: 3, Unit: 4, Zero: 5 };
const DIM = 0.6;
const FAINT = 0.16;

export const cap = (ring: Word) => REACH[ring];

export const coords = (a: number, b: number) => `${a},${b}`;

export function parse(click: unknown): Pair | null {
  const found = /^\s*(-?\d+)\s*,\s*(-?\d+)\s*$/.exec(String(click ?? ""));
  return found ? [Number(found[1]), Number(found[2])] : null;
}

export const WINDOWS: Lens[] = [
  {
    id: "gaussian",
    name: "Gaussian",
    note: "snowflake",
    set: { ring: "gaussian", limit: 40, look: "fate", click: "" },
  },
  {
    id: "eisenstein",
    name: "Eisenstein",
    note: "snowflake",
    set: { ring: "eisenstein", limit: 40, look: "fate", click: "" },
  },
  {
    id: "gaussian-norm",
    name: "Gaussian",
    note: "norms",
    set: { ring: "gaussian", limit: 40, look: "norm", click: "" },
  },
  {
    id: "eisenstein-norm",
    name: "Eisenstein",
    note: "norms",
    set: { ring: "eisenstein", limit: 40, look: "norm", click: "" },
  },
  {
    id: "gaussian-close",
    name: "Gaussian",
    note: "close-up",
    set: { ring: "gaussian", limit: 12, look: "fate", click: "" },
  },
  {
    id: "eisenstein-close",
    name: "Eisenstein",
    note: "close-up",
    set: { ring: "eisenstein", limit: 12, look: "fate", click: "" },
  },
];

export const POINTS: Prime[] = [
  { ring: "gaussian", a: 1, b: 1, norm: 2, fate: 3 },
  { ring: "gaussian", a: 3, b: 0, norm: 9, fate: 2 },
  { ring: "gaussian", a: 2, b: 1, norm: 5, fate: 1 },
  { ring: "gaussian", a: 2, b: 3, norm: 13, fate: 1 },
  { ring: "gaussian", a: 4, b: 1, norm: 17, fate: 1 },
  { ring: "eisenstein", a: 1, b: -1, norm: 3, fate: 3 },
  { ring: "eisenstein", a: 2, b: 0, norm: 4, fate: 2 },
  { ring: "eisenstein", a: 3, b: 1, norm: 7, fate: 1 },
  { ring: "eisenstein", a: 4, b: 1, norm: 13, fate: 1 },
];

const ENDS = ["", "splits", "stays prime", "ramifies"];

export function shelves(value: Value): [string, Entry[]][] {
  const windows = WINDOWS.map(({ id, name: label, note, set }) => ({
    id,
    label,
    sub: note,
    set,
    on: value.ring === set.ring && value.limit === set.limit && value.look === set.look,
  }));
  const points = POINTS.map(({ ring, a, b, norm, fate }) => {
    const click = coords(a, b);
    return {
      id: `${ring}-${click}`,
      label: name(ring, a, b),
      sub: `${fate === 2 ? Math.sqrt(norm) : norm} ${ENDS[fate]}`,
      set: { ring, limit: PEEK, click },
      on: value.ring === ring && value.click === click,
    };
  });
  return [
    ["Windows", windows],
    ["Primes", points],
  ];
}

export function shuffle(rand: Rng) {
  const [low, high] = SPAN;
  return { ring: pick(rand, RINGS)[0], limit: low + Math.floor(rand() * (high - low + 1)), look: pick(rand, LOOKS)[0] };
}

/* STUDY */

const wide: Partial<Record<Word, Survey>> = {};

let kept: Kept | null = null;

export function study(num: Num, word: Word, r: number) {
  if (kept && kept.num === num && kept.word === word && kept.r === r) return kept.plan;
  const plan = cut(num, whole(num, word, r), r);
  kept = { num, word, r, plan };
  return plan;
}

function whole(num: Num, word: Word, r: number) {
  const was = wide[word]?.num === num ? wide[word] : null;
  if (was && was.R >= r) return was;
  const R = Math.min(cap(word), Math.max(r, was ? 2 * was.R : r));
  wide[word] = survey(num, word, R);
  return wide[word]!;
}

function survey(num: Num, word: Word, R: number) {
  const g = num.gauss;
  const ring = g.Ring.named(word)!;
  const side = 2 * R + 1;
  const top = Number(g.Ring.top(ring, R));
  const units = g.Ring.units(ring);
  const window = new g.Window(ring, R);
  const count = window.census().points;
  const a = new Int32Array(count);
  const b = new Int32Array(count);
  const norm = new Uint32Array(count);
  const fate = new Uint8Array(count);
  const head = new Uint8Array(count);
  const shells = g.shells(ring, top);
  const classes = g.classes(ring, top);
  let n = 0;
  let k = 0;
  const put = (x: number, y: number, m: number, f: number, first: boolean) => {
    a[n] = x;
    b[n] = y;
    norm[n] = m;
    fate[n] = f;
    head[n] = first ? 1 : 0;
    n++;
  };
  put(0, 0, 0, CLASSES[window.class(0, 0)], true);
  for (let m = 1; m <= top; m++) {
    const some = shells[m] / units;
    for (let j = 0; j < some; j++, k++) {
      const [x, y] = classes[k];
      if (!window.holds(x, y)) continue;
      const f = CLASSES[window.class(x, y)];
      g.Ring.associates(ring, x, y).forEach(([u, v], i) => {
        put(Number(u), Number(v), m, f, i === 0);
      });
    }
  }
  window.free();
  if (n !== count || k !== classes.length)
    throw new Error(`gaussian: the shells lay ${n} points, the window holds ${count}`);
  return { num, word, ring, R, side, count, a, b, norm, fate, head, shells };
}

function cut(num: Num, full: Survey, r: number) {
  const g = num.gauss;
  const { word, ring, R, side, shells } = full;
  const window = new g.Window(ring, r);
  const census = window.census();
  const inside = new Int32Array(side * side);
  for (const [x, y] of window.points()) inside[(Number(y) + R) * side + (Number(x) + R)] = -1;
  window.free();
  const count = census.points;
  const top = Number(g.Ring.top(ring, r));
  const a = new Int32Array(count);
  const b = new Int32Array(count);
  const norm = new Uint32Array(count);
  const fate = new Uint8Array(count);
  const head = new Uint8Array(count);
  const held = new Uint32Array(top + 1);
  const ex = g.Ring.place(ring, 1, 0);
  const ey = g.Ring.place(ring, 0, 1);
  let x0 = Infinity;
  let y0 = Infinity;
  let x1 = -Infinity;
  let y1 = -Infinity;
  let n = 0;
  for (let p = 0; p < full.count; p++) {
    const at = (full.b[p] + R) * side + (full.a[p] + R);
    if (!inside[at]) continue;
    inside[at] = n + 1;
    a[n] = full.a[p];
    b[n] = full.b[p];
    norm[n] = full.norm[p];
    fate[n] = full.fate[p];
    head[n] = full.head[p];
    held[norm[n]]++;
    const x = a[n] * ex[0] + b[n] * ey[0];
    const y = a[n] * ex[1] + b[n] * ey[1];
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
    n++;
  }
  if (n !== count) throw new Error(`gaussian: the cut keeps ${n} points, the window holds ${count}`);
  let round = top;
  for (let m = 1; m <= top; m++) {
    if (held[m] !== shells[m]) {
      round = m - 1;
      break;
    }
  }
  const [busy, most] = g.peak(ring, round);
  const facts = { ...census, ring: word, reach: r, top, symmetry: g.Ring.symmetry(ring), busy, most };
  return {
    word,
    ring,
    R,
    side,
    r,
    top,
    count,
    a,
    b,
    norm,
    fate,
    head,
    inside,
    shells,
    ex,
    ey,
    frame: [x0 - 0.5, y0 - 0.5, x1 + 0.5, y1 + 0.5],
    facts,
  };
}

/* POINT */

export function hit(num: Num, plan: Plan, x: number, y: number) {
  const { R, side, inside, ring } = plan;
  if (x < -R || x > R || y < -R || y > R) return null;
  const index = inside[(y + R) * side + (x + R)] - 1;
  if (index < 0) return null;
  const G = num.gauss.Ring;
  const norm = plan.norm[index];
  const factors = num.factor.factorize_wide(norm).map(([p, e]): Pair => [Number(p), e]);
  const associates = G.associates(ring, x, y).map(([u, v]): Pair => [Number(u), Number(v)]);
  const conjugate = G.conjugate(ring, x, y).map(Number);
  return { a: x, b: y, index, norm, fate: plan.fate[index], factors, associates, conjugate, shell: plan.shells[norm] };
}

export function walk(plan: Plan, from: number, dir: number): Pair | null {
  const { count, fate, head } = plan;
  for (let k = 1; k <= count; k++) {
    const p = (((from + dir * k) % count) + count) % count;
    if (head[p] && fate[p] >= 1 && fate[p] <= 3) return [plan.a[p], plan.b[p]];
  }
  return null;
}

export function turned(point: Point, dir: number) {
  const list = point.associates;
  return point.norm ? list[dir > 0 ? 1 : list.length - 1] : null;
}

export function name(word: Word, a: number, b: number) {
  const letter = LETTERS[word];
  const size = Math.abs(b) === 1 ? "" : String(Math.abs(b));
  if (b === 0) return String(a);
  if (a === 0) return `${b < 0 ? "-" : ""}${size}${letter}`;
  return `${a} ${b < 0 ? "-" : "+"} ${size}${letter}`;
}

export const clicked = (word: Word, point: Point) => `${name(word, point.a, point.b)} at ${point.a}, ${point.b}`;

export const product = (factors: readonly Pair[]) =>
  factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : String(p))).join(" · ");

export function verdict(word: Word, point: Point) {
  const { fate, norm, factors } = point;
  const [ca, cb] = point.conjugate;
  if (fate === 5) return "the origin";
  if (fate === 4) return "a unit, norm 1";
  if (fate === 3) return `prime: ${norm} ramifies, a unit times a square`;
  if (fate === 2) return `prime: ${factors[0][0]} stays prime in the plane`;
  if (fate === 1) return `prime: ${norm} splits as (${name(word, point.a, point.b)})(${name(word, ca, cb)})`;
  return `composite, norm ${norm} = ${product(factors)}`;
}

/* TIME */

export const share = (t: number, span: number) => Math.min(1, Math.max(0, t / span));

export const reached = (part: number, count: number) => Math.round(part * part * (count - 1));

/* LAYOUT */

export function lay(frame: readonly number[], w: number, h: number, pad: number, cell = 0) {
  const [x0, y0, x1, y1] = frame;
  const k = cell > 0 ? cell : Math.max(0.01, Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1)));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}

/* STYLE */

export const SKIP = 0;
export const FAINTLY = 1;
export const SPLIT = 2;
export const INERT = 3;
export const RAMIFIED = 4;
export const UNITS = 5;
export const PLAIN = 6;
export const BAND = 7;

export function styleOf(look: string, faint: number, fate: number, norm: number, top: number) {
  if (fate === 5) return SKIP;
  if (fate === 0) return faint ? FAINTLY : SKIP;
  if (fate === 4) return UNITS;
  if (look === "norm") return BAND + Math.min(BANDS - 1, Math.floor(Math.sqrt(norm / (top || 1)) * BANDS));
  if (look === "plain") return PLAIN;
  return SPLIT + fate - 1;
}

const css = (list: readonly number[]) => `rgb(${list.map((v) => Math.round(v)).join(", ")})`;

export function inks({ paper, accent }: Pick<Paints, "paper" | "accent">) {
  const [r, g, b] = rgb(paper);
  const fg = css(0.299 * r + 0.587 * g + 0.114 * b > 128 ? [0, 0, 0] : [255, 255, 255]);
  return {
    paper,
    accent,
    fg,
    dim: veil(fg, DIM),
    faint: veil(fg, FAINT),
    split: palette.blue,
    inert: palette.orange,
    ramified: palette.pink,
    unit: palette.green,
    fire: [palette.red, palette.orange, palette.yellow, fg],
  };
}

export function ramp(stops: readonly string[], t: number) {
  const list = stops.map(rgb);
  const k = Math.min(1, Math.max(0, t)) * (list.length - 1);
  const i = Math.min(list.length - 2, Math.floor(k));
  const f = k - i;
  return css(list[i].map((v, c) => v + (list[i + 1][c] - v) * f));
}

export function tones(ink: Ink) {
  const list: (string | null)[] = [null, ink.faint, ink.split, ink.inert, ink.ramified, ink.unit, ink.accent];
  for (let k = 0; k < BANDS; k++) list.push(ramp(ink.fire, k / (BANDS - 1)));
  return list;
}

/* FILES */

export function picture(plan: Plan, style: ArrayLike<number>, tone: readonly (string | null)[]) {
  const { word, r, count, a, b, ex, ey, frame } = plan;
  const [x0, y0, x1, y1] = frame;
  const side = 2 * r + 1;
  const w = (x1 - x0) * UNIT;
  const h = (y1 - y0) * UNIT;
  const inset = INSET * UNIT;
  const groups = tone.map((color): string[] | null => (color ? [] : null));
  if (word === "gaussian") {
    const cells = tone.map((color) => (color ? new Uint8Array(side * side) : null));
    for (let p = 0; p < count; p++) if (cells[style[p]]) cells[style[p]]![(b[p] + r) * side + (a[p] + r)] = 1;
    cells.forEach((types, s) => {
      if (!types) return;
      const gap = s === FAINTLY ? 0 : inset;
      for (const [row, col, len] of runs({ shape: [side, side], types }))
        groups[s]!.push(
          `<rect x="${fix(col * UNIT + gap)}" y="${fix((side - 1 - row) * UNIT + gap)}" width="${fix(len * UNIT - 2 * gap)}" height="${fix(UNIT - 2 * gap)}"/>`,
        );
    });
  } else {
    for (let p = 0; p < count; p++) {
      const s = style[p];
      if (!groups[s]) continue;
      const x = a[p] * ex[0] + b[p] * ey[0] - x0;
      const y = y1 - (a[p] * ex[1] + b[p] * ey[1]);
      groups[s].push(
        `<circle cx="${fix(x * UNIT)}" cy="${fix(y * UNIT)}" r="${fix(UNIT / 2 - (s === FAINTLY ? 0 : inset))}"/>`,
      );
    }
  }
  const body = groups.map((marks, s) => (marks?.length ? `<g fill="${tone[s]}">${marks.join("")}</g>` : ""));
  return sheet(w, h, body.join(""));
}

export function table(plan: Plan) {
  const { count, a, b, norm, fate } = plan;
  const rows = ["a,b,norm,fate"];
  for (let p = 0; p < count; p++)
    if (fate[p] >= 1 && fate[p] <= 3) rows.push(`${a[p]},${b[p]},${norm[p]},${FATES[fate[p]]}`);
  return `${rows.join("\n")}\n`;
}
