import type { readings } from "@mrlyprod/num";
import type { Rgb } from "../../net/sdk/color.ts";
import { blend, rgb, veil } from "../../net/sdk/color.ts";
import type { Had } from "../../net/sdk/hooks.ts";
import type { Patch, Value } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Unit } from "../shared/units/index.ts";

/* TYPES */

export type Num = Unit<"num">;

export type Doors = Had<"num">;

export type Design = readings.Design;

export type Reading = readings.Reading;

export type Head = readings.Head;

export type Battery = readings.Battery;

export type Arc = readings.Arc;

export type Kind = "plaid" | "battery" | "arcs";

export type Test = "shift" | "dilation" | "kernel" | "square";

export type Knobs = {
  view: Kind;
  power: number;
  width: number;
  reading: Reading;
  rudin: number;
  base: number;
  digits: string;
};

export type Picked = { rudin: number; base: number; digits: string };

export type Featured = { label: string; sub: string; pick: Picked };

export type Look = { paper: string; accent: string; ink: string };

export type Inks = Look & { plus: string; minus: string; dim: string; hair: string; model: string };

export type Bar = { test: Test; label: string; scale: number; offset: number; r: number; z: number; head: Head | null };

export type Model = { arcs: Arc[]; model: number[] };

export type Told = {
  view: Kind;
  title: string;
  name: string;
  reading: Reading;
  formula: string;
  power: number;
  count: number;
  bias: number;
  z: number;
  heads: Head[];
};

export type Plaid = Told & { view: "plaid"; width: number; rows: number; plus: number };

export type Tested = Told & {
  view: "battery";
  largest: Record<Test, Bar | null>;
  lit: Bar[];
  verdict: string;
};

export type Arcs = Told & {
  view: "arcs";
  exponents: number[];
  observed: number[];
  model: number[] | null;
  arcs: Arc[] | null;
  predicted: number | null;
  residual: number | null;
  pending: boolean;
  why: string;
};

export type Facts = Plaid | Tested | Arcs;

export type FactsOf<K extends Kind> = Extract<Facts, { view: K }>;

export type Plan =
  | { view: "plaid"; stream: Int8Array; width: number; rows: number; facts: Plaid }
  | { view: "battery"; bars: Bar[]; facts: Tested }
  | { view: "arcs"; facts: Arcs };

export type PlanOf<K extends Kind> = Extract<Plan, { view: K }>;

export type Box = { x: number; y: number; w: number; h: number };

export type Scale = { hi: number; at: (z: number) => number };

type Kept<T> = { num: Num; made: T };

export const VIEWS: [Kind, string][] = [
  ["plaid", "Plaid"],
  ["battery", "Battery"],
  ["arcs", "Arcs"],
];
export const READINGS: [Reading, string][] = [
  ["N", "n"],
  ["Square", "n²"],
  ["SquarePlusN", "n² + n"],
  ["Cube", "n³"],
  ["Primes", "primes"],
];
export const BASES = [2, 3, 4, 5, 6];
export const POWERS = { min: 10, max: 20 };
export const FIRST = 8;
export const CUT = 0.25;
export const BAND: [number, number] = [2, 3];
export const SHIFTS = 64;
export const SCALES = { min: 2, max: 9 };
export const KERNELS = [2, 3];
export const FOLD = 9;
export const PAD = 16;
export const FACE = "Noto Sans Mono";
export const FLOOR = 0.1;
export const THUMB = 32;
export const NICKNAMES: Record<string, string> = {
  "2:1": "Thue-Morse",
  "4:12": "Thue-Morse again",
  "3:2": "the ternary parity",
  "rudin-shapiro": "Rudin-Shapiro",
};
export const FEATURED: Featured[] = [
  { label: "Thue-Morse", sub: "2:1, the parity of the binary ones", pick: { rudin: 0, base: 2, digits: "1" } },
  { label: "Rudin-Shapiro", sub: "the parity of the 11 blocks, white", pick: { rudin: 1, base: 2, digits: "1" } },
  { label: "Ternary parity", sub: "3:2, the ternary twos", pick: { rudin: 0, base: 3, digits: "2" } },
  { label: "4:13", sub: "the strongest bias along the squares", pick: { rudin: 0, base: 4, digits: "13" } },
  { label: "6:135", sub: "the least exponent, eta 0.1755", pick: { rudin: 0, base: 6, digits: "135" } },
  { label: "6:5", sub: "a mirror pair, this side lights", pick: { rudin: 0, base: 6, digits: "5" } },
  { label: "6:12345", sub: "the mirror, one spectrum, no bias", pick: { rudin: 0, base: 6, digits: "12345" } },
  { label: "5:14", sub: "a twin whose Gauss sum survives", pick: { rudin: 0, base: 5, digits: "14" } },
  { label: "5:34", sub: "its twin, the Gauss sum vanishes", pick: { rudin: 0, base: 5, digits: "34" } },
];
const KEEP = 4;
const BATTERIES = 6;
const LIT = 3;
const EDGE = 4;

/* DESIGN */

export const digitsOf = (base: number) => Array.from({ length: Math.max(0, base - 1) }, (_, i) => String(i + 1));

export const baseOf = (value: Value) => (Number(value.rudin) === 1 ? 2 : Number(value.base));

export function designOf(value: Value): Design {
  if (Number(value.rudin) === 1) return { base: 2, mask: 0, rudin: true };
  const base = Number(value.base);
  let mask = 0;
  for (const c of String(value.digits ?? "")) {
    const d = Number(c);
    if (d >= 1 && d < base) mask |= 1 << (d - 1);
  }
  return { base, mask, rudin: false };
}

export const place = (pick: Picked): Patch => ({ ...pick });

export const keyOf = (design: Design) => (design.rudin ? "rudin" : `${design.base}:${design.mask}`);

export const holds = (value: Value, pick: Picked) =>
  Number(value.rudin) === 1 ? pick.rudin === 1 : pick.rudin === 0 && keyOf(designOf(value)) === keyOf(designOf(pick));

export function title(num: Num, design: Design): string {
  const name = num.readings.name(design);
  const nick = NICKNAMES[name];
  if (name === "rudin-shapiro") return nick!;
  return nick ? `${name}, ${nick}` : name;
}

export const slug = (num: Num, design: Design) => num.readings.name(design).replace(":", "-");

export const formula = (num: Num, reading: Reading) => num.readings.Reading.formula(reading);

/* WIDTH */

export const cap = (base: number, count: number) => Math.max(1, Math.floor(Math.log(count) / Math.log(base) + 1e-9));

export const fit = (base: number, count: number) =>
  Math.min(cap(base, count), Math.max(1, Math.round(Math.log(Math.sqrt(count)) / Math.log(base))));

export function widthOf(value: Value): number {
  const count = 2 ** Number(value.power);
  const base = baseOf(value);
  const k = Number(value.width) > 0 ? Math.min(Number(value.width), cap(base, count)) : fit(base, count);
  return base ** k;
}

/* STREAMS */

const streams = new Map<string, Kept<Int8Array>>();

export function signs(num: Num, design: Design, reading: Reading, count: number): Int8Array {
  const key = `${keyOf(design)}|${reading}`;
  const had = streams.get(key);
  if (had && had.num === num && had.made.length >= count) return had.made.subarray(0, count);
  const made = num.readings.stream(design, reading, count);
  streams.delete(key);
  streams.set(key, { num, made });
  while (streams.size > KEEP) streams.delete(streams.keys().next().value!);
  return made;
}

export function sum(stream: ArrayLike<number>, count = stream.length): number {
  let total = 0;
  for (let i = 0; i < count; i++) total += stream[i];
  return total;
}

export const zOf = (r: number, count: number) => Math.abs(r) * Math.sqrt(count);

/* BATTERY */

const batteries = new Map<string, Kept<Battery>>();

export function verdict(num: Num, design: Design, reading: Reading, count: number): Battery {
  const key = `${keyOf(design)}|${reading}|${count}`;
  const had = batteries.get(key);
  if (had && had.num === num) return had.made;
  const made = num.readings.battery(signs(num, design, reading, FOLD * count));
  batteries.delete(key);
  batteries.set(key, { num, made });
  while (batteries.size > BATTERIES) batteries.delete(batteries.keys().next().value!);
  return made;
}

const headOf = (heads: Head[], kernel: boolean, scale: number, offset: number) =>
  heads.find((one) => one.kernel === kernel && one.scale === scale && one.offset === offset) ?? null;

export function bars(read: Battery, heads: Head[]): Bar[] {
  const n = read.count;
  const out: Bar[] = read.shift.map((r, i) => ({
    test: "shift",
    label: `h = ${i + 1}`,
    scale: 1,
    offset: i + 1,
    r,
    z: zOf(r, n),
    head: null,
  }));
  for (const one of read.dilation)
    out.push({
      test: "dilation",
      label: `(${one.scale}, ${one.offset})`,
      scale: one.scale,
      offset: one.offset,
      r: one.correlation,
      z: zOf(one.correlation, n),
      head: headOf(heads, false, one.scale, one.offset),
    });
  read.kernel.forEach((r, i) => {
    out.push({
      test: "kernel",
      label: `b = ${KERNELS[i]}`,
      scale: KERNELS[i],
      offset: 0,
      r,
      z: zOf(r, n),
      head: headOf(heads, true, KERNELS[i], 0),
    });
  });
  out.push({
    test: "square",
    label: "square",
    scale: 6,
    offset: 0,
    r: read.square,
    z: zOf(read.square, n),
    head: null,
  });
  return out;
}

export const largest = (list: Bar[], test: Test) =>
  list.filter((one) => one.test === test).reduce<Bar | null>((top, one) => (!top || one.z > top.z ? one : top), null);

export const lit = (list: Bar[]) => list.filter((one) => one.z > LIT).sort((a, b) => b.z - a.z);

export function verdictOf(list: Bar[]): string {
  const top = lit(list)[0];
  if (!top) return `passes: every z inside ${LIT}`;
  const where = `the ${top.test} test at ${top.label}, z ${top.z.toFixed(2)}`;
  return top.z > EDGE ? `fails: ${where} lights` : `at the edge: ${where}, inside the lab's ${EDGE}`;
}

export function scaleOf(list: Bar[]): Scale {
  const top = list.reduce((most, one) => Math.max(most, one.z), 0);
  const hi = Math.max(10, top * 1.25);
  const span = Math.log(hi / FLOOR);
  return { hi, at: (z) => (z <= FLOOR ? 0 : Math.min(1, Math.log(z / FLOOR) / span)) };
}

/* ARCS */

const models = new Map<string, Kept<Model>>();

export const EXPONENTS = Array.from({ length: POWERS.max - FIRST + 1 }, (_, i) => FIRST + i);

export function modelled(num: Num, design: Design): Model | null {
  const had = models.get(keyOf(design));
  return had && had.num === num ? had.made : null;
}

export function warm(num: Num, design: Design): Model {
  const had = modelled(num, design);
  if (had) return had;
  const arcs = num.readings.arcs(design, CUT);
  const model = arcs.length ? Array.from(num.readings.model(design, CUT, EXPONENTS)) : EXPONENTS.map(() => 1);
  const made = { arcs, model };
  models.set(keyOf(design), { num, made });
  return made;
}

export const why = (reading: Reading, model: Model | null) =>
  reading !== "Square"
    ? "the major arcs predict the squares; along this reading the bias stands alone"
    : model && !model.arcs.length
      ? `no arc below ${CUT}: the model is the constant 1`
      : "";

/* STUDY */

let kept: { num: Num; key: string; plan: Plan } | null = null;

const told = (num: Num, value: Knobs, design: Design, stream: Int8Array, count: number): Told => ({
  view: value.view,
  title: title(num, design),
  name: num.readings.name(design),
  reading: value.reading,
  formula: formula(num, value.reading),
  power: value.power,
  count,
  bias: sum(stream, count),
  z: 0,
  heads: num.readings.near_heads(design, value.reading),
});

const signed = (one: Told) => ({ ...one, z: one.bias / Math.sqrt(one.count) });

function pairs(heads: Head[]) {
  return heads.map((one) => `${one.kernel ? `kernel ${one.scale}` : `(${one.scale}, ${one.offset})`}`);
}

export function said(heads: Head[]): string {
  if (!heads.length) return "none: a free reading";
  const exact = heads.filter((one) => one.exact);
  const twisted = heads.filter((one) => !one.exact);
  const parts = [
    exact.length
      ? `${pairs(exact).join(", ")} exact, reading ${exact.map((one) => (one.sign > 0 ? "+1" : "-1")).join(", ")}`
      : "",
    twisted.length ? `${pairs(twisted).join(", ")} with a carry` : "",
  ].filter(Boolean);
  return parts.join("; ");
}

export function survey(num: Num, value: Knobs): Plan {
  const design = designOf(value);
  const count = 2 ** value.power;
  if (value.view === "plaid") {
    const stream = signs(num, design, value.reading, count);
    const width = widthOf(value);
    const rows = Math.ceil(count / width);
    const base = signed(told(num, value, design, stream, count));
    const plus = (count + base.bias) / 2;
    return { view: "plaid", stream, width, rows, facts: { ...base, view: "plaid", width, rows, plus } };
  }
  if (value.view === "battery") {
    const read = verdict(num, design, value.reading, count);
    const stream = signs(num, design, value.reading, count);
    const base = signed(told(num, value, design, stream, count));
    const list = bars(read, base.heads);
    const lights = lit(list);
    return {
      view: "battery",
      bars: list,
      facts: {
        ...base,
        view: "battery",
        largest: {
          shift: largest(list, "shift"),
          dilation: largest(list, "dilation"),
          kernel: largest(list, "kernel"),
          square: largest(list, "square"),
        },
        lit: lights,
        verdict: verdictOf(list),
      },
    };
  }
  const stream = signs(num, design, value.reading, count);
  const base = signed(told(num, value, design, stream, count));
  const exponents = EXPONENTS.filter((e) => e <= value.power);
  const observed = Array.from(num.readings.bias(stream, exponents), Number);
  const squares = value.reading === "Square";
  const made = squares ? modelled(num, design) : null;
  const model = made ? made.model.slice(0, exponents.length) : null;
  const predicted = model ? model[exponents.length - 1] : null;
  return {
    view: "arcs",
    facts: {
      ...base,
      view: "arcs",
      exponents,
      observed,
      model: made ? made.model : null,
      arcs: made ? made.arcs : null,
      predicted,
      residual: predicted === null ? null : base.bias - predicted,
      pending: squares && !made,
      why: why(value.reading, made),
    },
  };
}

export function study(num: Num, value: Knobs): Plan {
  const design = designOf(value);
  const key = JSON.stringify([
    value.view,
    keyOf(design),
    value.reading,
    value.power,
    value.view === "plaid" ? widthOf(value) : 0,
    value.view === "arcs" && value.reading === "Square" ? Boolean(modelled(num, design)) : false,
  ]);
  if (kept && kept.num === num && kept.key === key) return kept.plan;
  const plan = survey(num, value);
  kept = { num, key, plan };
  return plan;
}

/* STEPS */

export const stepped = (value: Value, by: number) => {
  const span = POWERS.max - POWERS.min + 1;
  return { power: POWERS.min + ((((Number(value.power) - POWERS.min + by) % span) + span) % span) };
};

export const turned = (value: Value, by: number) => {
  const at = VIEWS.findIndex(([one]) => one === value.view);
  return { view: VIEWS[(at + by + VIEWS.length) % VIEWS.length][0] };
};

/* FILES */

const cell = (v: unknown) => String(v);

export function batteryCsv(num: Num, value: Knobs): string {
  const design = designOf(value);
  const count = 2 ** value.power;
  const list = bars(verdict(num, design, value.reading, count), num.readings.near_heads(design, value.reading));
  const head = ["design", "reading", "N", "test", "pair", "r", "z", "near-head", "exact"];
  const rows = list.map((one) =>
    [
      num.readings.name(design),
      formula(num, value.reading),
      count,
      one.test,
      one.label,
      one.r,
      one.z,
      one.head ? "yes" : "no",
      one.head ? (one.head.exact ? "yes" : "no") : "",
    ].map(cell),
  );
  return [head, ...rows].map((row) => row.join(",")).join("\n");
}

export function biasCsv(num: Num, value: Knobs): string {
  const design = designOf(value);
  const stream = signs(num, design, value.reading, 2 ** value.power);
  const exponents = EXPONENTS.filter((e) => e <= value.power);
  const observed = Array.from(num.readings.bias(stream, exponents), Number);
  const made = value.reading === "Square" ? warm(num, design) : null;
  const head = ["design", "reading", "e", "N", "S", "z", "model", "z_model", "residual"];
  const rows = exponents.map((e, i) => {
    const n = 2 ** e;
    const model = made ? made.model[i] : null;
    return [
      num.readings.name(design),
      formula(num, value.reading),
      e,
      n,
      observed[i],
      observed[i] / Math.sqrt(n),
      model ?? "",
      model === null ? "" : model / Math.sqrt(n),
      model === null ? "" : observed[i] - model,
    ].map(cell);
  });
  return [head, ...rows].map((row) => row.join(",")).join("\n");
}

/* INKS */

export function inks(look: Look): Inks {
  return {
    ...look,
    plus: palette.orange,
    minus: palette.blue,
    dim: blend(look.ink, look.paper, 0.5),
    hair: veil(look.ink, 0.14),
    model: palette.orange,
  };
}

export const font = (dpr: number, k = 1) => `${Math.round(12 * dpr * k)}px "${FACE}", ui-monospace, monospace`;

/* RASTER */

export function stamp(stream: ArrayLike<number>, width: number, rows: number, plus: Rgb, minus: Rgb): OffscreenCanvas {
  const page = new OffscreenCanvas(width, rows);
  const pen = page.getContext("2d")!;
  const image = pen.createImageData(width, rows);
  const data = image.data;
  const n = Math.min(stream.length, width * rows);
  for (let i = 0; i < n; i++) {
    const colour = stream[i] > 0 ? plus : minus;
    data[i * 4] = colour[0];
    data[i * 4 + 1] = colour[1];
    data[i * 4 + 2] = colour[2];
    data[i * 4 + 3] = 255;
  }
  pen.putImageData(image, 0, 0);
  return page;
}

export function lay(w: number, h: number, cols: number, rows: number, pad: number): Box & { cell: number } {
  const room = { w: Math.max(1, w - 2 * pad), h: Math.max(1, h - 2 * pad) };
  const raw = Math.min(room.w / cols, room.h / rows);
  const cell = raw >= 1 ? Math.floor(raw) : raw;
  const box = { w: cell * cols, h: cell * rows };
  return { x: Math.round((w - box.w) / 2), y: Math.round((h - box.h) / 2), w: box.w, h: box.h, cell };
}

export function drawPlaid(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  plan: PlanOf<"plaid">,
  source: OffscreenCanvas,
) {
  const box = lay(w, h, plan.width, plan.rows, PAD * dpr);
  ctx.imageSmoothingEnabled = box.cell < 1;
  ctx.drawImage(source, 0, 0, plan.width, plan.rows, box.x, box.y, box.w, box.h);
  return box;
}

/* BARS */

const text = (
  ctx: CanvasRenderingContext2D,
  colour: string,
  what: string,
  x: number,
  y: number,
  align: CanvasTextAlign = "left",
) => {
  ctx.fillStyle = colour;
  ctx.textAlign = align;
  ctx.fillText(what, x, y);
};

function band(ctx: CanvasRenderingContext2D, box: Box, scale: Scale, ink: Inks) {
  const y2 = box.y + box.h - scale.at(BAND[0]) * box.h;
  const y3 = box.y + box.h - scale.at(BAND[1]) * box.h;
  ctx.fillStyle = ink.hair;
  ctx.fillRect(box.x, y3, box.w, y2 - y3);
  ctx.strokeStyle = ink.hair;
  ctx.lineWidth = 1;
  for (const y of [y2, y3]) {
    ctx.beginPath();
    ctx.moveTo(box.x, y);
    ctx.lineTo(box.x + box.w, y);
    ctx.stroke();
  }
}

function bar(
  ctx: CanvasRenderingContext2D,
  one: Bar,
  x: number,
  width: number,
  box: Box,
  scale: Scale,
  ink: Inks,
  dpr: number,
) {
  const tall = scale.at(one.z) * box.h;
  const y = box.y + box.h - tall;
  ctx.fillStyle = one.z > LIT ? ink.accent : ink.dim;
  ctx.fillRect(x, y, width, tall);
  if (!one.head) return;
  ctx.strokeStyle = ink.ink;
  ctx.lineWidth = Math.max(1, dpr);
  ctx.setLineDash(one.head.exact ? [] : [3 * dpr, 3 * dpr]);
  ctx.strokeRect(x - dpr, box.y, width + 2 * dpr, box.h);
  ctx.setLineDash([]);
}

export function drawBattery(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  plan: PlanOf<"battery">,
  ink: Inks,
) {
  const pad = PAD * dpr;
  const px = Math.round(12 * dpr);
  const line = px * 1.5;
  const gap = px * 2;
  const scale = scaleOf(plan.bars);
  ctx.font = font(dpr);
  ctx.textBaseline = "alphabetic";
  const inner = { x: pad, y: pad, w: w - 2 * pad, h: h - 2 * pad };
  const upper = { x: inner.x, y: inner.y + line, w: inner.w - px * 3, h: Math.round(inner.h * 0.42) - line - gap };
  text(ctx, ink.ink, `shift, h = 1 to ${SHIFTS}`, upper.x, inner.y + px);
  band(ctx, upper, scale, ink);
  const shifts = plan.bars.filter((one) => one.test === "shift");
  const slot = upper.w / SHIFTS;
  shifts.forEach((one, i) => {
    bar(ctx, one, upper.x + i * slot + slot * 0.15, Math.max(1, slot * 0.7), upper, scale, ink, dpr);
  });
  for (const z of [1, BAND[0], BAND[1], 10, 100, 1000]) {
    if (z > scale.hi) continue;
    text(ctx, ink.dim, `z ${z}`, upper.x + upper.w + px * 0.5, upper.y + upper.h - scale.at(z) * upper.h + px * 0.35);
  }
  for (const hh of [1, 16, 32, 48, 64])
    text(ctx, ink.dim, String(hh), upper.x + (hh - 0.5) * slot, upper.y + upper.h + px, "center");
  const top = upper.y + upper.h + gap;
  const lower = { x: inner.x, y: top + line, w: inner.w, h: inner.y + inner.h - top - line };
  const left = { x: lower.x + px * 3, y: lower.y + px, w: Math.round(lower.w * 0.72) - px * 3, h: lower.h - px * 2 };
  const right = { x: left.x + left.w + px * 4, y: left.y, w: lower.x + lower.w - left.x - left.w - px * 4, h: left.h };
  text(ctx, ink.ink, "dilation, y(n) against y(Bn + r)", lower.x, top + px);
  const cols = SCALES.max;
  const rowsN = SCALES.max - SCALES.min + 1;
  const cw = left.w / cols;
  const rh = left.h / rowsN;
  for (let r = 0; r < cols; r++) text(ctx, ink.dim, `r ${r}`, left.x + (r + 0.5) * cw, left.y - px * 0.3, "center");
  for (let b = SCALES.min; b <= SCALES.max; b++) {
    const row = b - SCALES.min;
    const cellBox = { x: left.x, y: left.y + row * rh + rh * 0.12, w: cw * b, h: rh * 0.76 };
    text(ctx, ink.dim, `B ${b}`, left.x - px * 0.5, cellBox.y + cellBox.h, "right");
    band(ctx, cellBox, scale, ink);
    for (const one of plan.bars) {
      if (one.test !== "dilation" || one.scale !== b) continue;
      bar(ctx, one, left.x + one.offset * cw + cw * 0.2, Math.max(1, cw * 0.6), cellBox, scale, ink, dpr);
    }
  }
  const rest = plan.bars.filter((one) => one.test === "kernel" || one.test === "square");
  const rbox = { x: right.x, y: right.y + px, w: right.w, h: right.h - px * 2.5 };
  text(ctx, ink.ink, "kernel and square", right.x, top + px);
  band(ctx, rbox, scale, ink);
  const rs = rbox.w / rest.length;
  rest.forEach((one, i) => {
    bar(ctx, one, rbox.x + i * rs + rs * 0.25, Math.max(1, rs * 0.5), rbox, scale, ink, dpr);
    text(
      ctx,
      ink.dim,
      one.test === "square" ? "sq" : one.label,
      rbox.x + (i + 0.5) * rs,
      rbox.y + rbox.h + px,
      "center",
    );
  });
}

/* LINES */

function polyline(
  ctx: CanvasRenderingContext2D,
  points: [number, number][],
  colour: string,
  width: number,
  dash: number[],
) {
  if (!points.length) return;
  ctx.strokeStyle = colour;
  ctx.lineWidth = width;
  ctx.setLineDash(dash);
  ctx.beginPath();
  points.forEach(([x, y], i) => {
    if (i) ctx.lineTo(x, y);
    else ctx.moveTo(x, y);
  });
  ctx.stroke();
  ctx.setLineDash([]);
}

export function drawArcs(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  plan: PlanOf<"arcs">,
  ink: Inks,
) {
  const pad = PAD * dpr;
  const px = Math.round(12 * dpr);
  const f = plan.facts;
  const zs = f.observed.map((s, i) => s / Math.sqrt(2 ** f.exponents[i]));
  const zm = f.model ? f.model.map((s, i) => s / Math.sqrt(2 ** EXPONENTS[i])) : [];
  const reach = Math.max(4, 1.15 * Math.max(...zs.map(Math.abs), ...zm.map(Math.abs)));
  const box = { x: pad + px * 3, y: pad + px * 2.5, w: w - 2 * pad - px * 3, h: h - 2 * pad - px * 4 };
  const xAt = (e: number) => box.x + ((e - FIRST) / (POWERS.max - FIRST)) * box.w;
  const yAt = (z: number) => box.y + box.h / 2 - (z / reach) * (box.h / 2);
  ctx.font = font(dpr);
  ctx.textBaseline = "alphabetic";
  ctx.fillStyle = ink.hair;
  ctx.fillRect(box.x, yAt(BAND[0]), box.w, yAt(-BAND[0]) - yAt(BAND[0]));
  polyline(
    ctx,
    [
      [box.x, yAt(0)],
      [box.x + box.w, yAt(0)],
    ],
    ink.dim,
    1,
    [],
  );
  for (const e of EXPONENTS) {
    if (e % 2) continue;
    polyline(
      ctx,
      [
        [xAt(e), box.y],
        [xAt(e), box.y + box.h],
      ],
      ink.hair,
      1,
      [],
    );
    text(ctx, ink.dim, `2^${e}`, xAt(e), box.y + box.h + px * 1.2, "center");
  }
  const step = reach > 20 ? 10 : reach > 8 ? 4 : 2;
  for (let z = -Math.floor(reach / step) * step; z <= reach; z += step)
    text(ctx, ink.dim, String(z), box.x - px * 0.5, yAt(z) + px * 0.35, "right");
  if (f.model)
    polyline(
      ctx,
      EXPONENTS.map((e, i) => [xAt(e), yAt(zm[i])]),
      ink.model,
      Math.max(1, 1.5 * dpr),
      [4 * dpr, 3 * dpr],
    );
  polyline(
    ctx,
    f.exponents.map((e, i) => [xAt(e), yAt(zs[i])]),
    ink.accent,
    Math.max(1, 2 * dpr),
    [],
  );
  ctx.fillStyle = ink.accent;
  f.exponents.forEach((e, i) => {
    ctx.beginPath();
    ctx.arc(xAt(e), yAt(zs[i]), 2.5 * dpr, 0, Math.PI * 2);
    ctx.fill();
  });
  text(ctx, ink.accent, `S(N)/sqrt(N) along ${f.formula}`, box.x, pad + px);
  if (f.model) text(ctx, ink.model, "major arcs below 1/4", box.x + box.w, pad + px, "right");
  else text(ctx, ink.dim, f.pending ? "summing the arcs" : f.why, box.x + box.w, pad + px, "right");
}

export function sketch(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  plan: Plan,
  ink: Inks,
  source: OffscreenCanvas | null,
) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  if (plan.view === "plaid") {
    if (source) drawPlaid(ctx, w, h, dpr, plan, source);
    return;
  }
  if (plan.view === "battery") return drawBattery(ctx, w, h, dpr, plan, ink);
  drawArcs(ctx, w, h, dpr, plan, ink);
}

export function thumb(
  num: Num,
  pick: Picked,
  reading: Reading,
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  ink: Inks,
) {
  const design = designOf(pick);
  const base = design.rudin ? 2 : design.base;
  const width = base ** Math.max(1, Math.round(Math.log(THUMB) / Math.log(base)));
  const count = width * width;
  const stream = num.readings.stream(design, reading, count);
  const source = stamp(stream, width, width, rgb(ink.plus), rgb(ink.minus));
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  const box = lay(w, h, width, width, 0);
  ctx.imageSmoothingEnabled = box.cell < 1;
  ctx.drawImage(source, 0, 0, width, width, box.x, box.y, box.w, box.h);
}
