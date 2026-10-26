import type { RefObject } from "react";
import type { Page } from "../../lib/app/page.tsx";
import type { Opts } from "../../lib/app/run.ts";
import type { Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { KINDS } from "../../lib/sequences/chart.ts";
import type { Key, Row, Slug } from "../../lib/sequences/engine.ts";
import { applies, MEASURES, print, TERMS } from "../../lib/sequences/engine.ts";
import type { Grown, Rect, Series, Slots, Tones, View } from "../../lib/sequences/paint.ts";
import {
  chart,
  clear,
  drawRibbon,
  FACE,
  fit,
  font,
  growth,
  legend,
  ribbon,
  shade,
  slotAt,
  tones,
} from "../../lib/sequences/paint.ts";
import type { Units } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Rule } from "./engine.ts";
import { ARG, decimal, OPS, PAIRED, TAKES } from "./engine.ts";

/* TYPES */

export type Knobs = {
  measure: Slug;
  axis: string;
  terms: number;
  chart: string;
  digits: number;
  mix: number;
  op: string;
  scode: string;
  smeasure: Slug;
  arg: number;
};

export type Panel = { row: Series & Grown; rule: Rule; note: string; mate?: Row | null };

export type Feed = { panels: Panel[]; version: number };

export type Fed = Opts & Partial<Feed> & { live?: RefObject<Feed> };

export type Hands = { mix: () => void; design: (by: number) => void; terms: (by: number) => void; cycle: () => void };

export type Stage = ReturnType<typeof make>;

export type Held = Stage & Hands;

const measures = (value: Value) =>
  MEASURES.filter((one) => applies(one, value.dim ?? 2, value.base ?? 2)).map((one): [string, string] => [
    one.slug,
    one.label,
  ]);

const mixed = (value: Value) => Number(value.mix) === 1;

const paired = (value: Value) => typeof value.op === "string" && PAIRED.includes(value.op);

const takes = (value: Value) => typeof value.op === "string" && TAKES.includes(value.op);

const HIDE = () => false;

export const SPEC: Spec = [
  { key: "measure", label: "Measure", kind: "pick", def: "surface", options: measures, group: "Sequence", when: HIDE },
  {
    key: "axis",
    label: "Axis",
    kind: "segment",
    def: "level",
    options: [
      ["level", "Level"],
      ["side", "Odd side"],
    ],
    group: "Sequence",
    when: HIDE,
  },
  { key: "terms", label: "Terms", kind: "slider", def: 12, min: TERMS.min, max: TERMS.max, step: 1, group: "Sequence" },
  { key: "chart", label: "Chart", kind: "pick", def: "pins", options: KINDS, group: "View" },
  {
    key: "digits",
    label: "Digits",
    kind: "segment",
    def: 2,
    options: [
      [2, "2"],
      [3, "3"],
      [10, "10"],
    ],
    group: "View",
    when: (value) => value.chart === "digits",
  },
  { key: "mix", label: "Mix a second", kind: "toggle", def: 0, group: "Mix" },
  { key: "op", label: "Operation", kind: "pick", def: "hadamard", options: OPS, group: "Mix", when: mixed },
  {
    key: "scode",
    label: "Second code",
    kind: "text",
    def: "3",
    group: "Mix",
    when: (value) => mixed(value) && paired(value),
  },
  {
    key: "smeasure",
    label: "Its measure",
    kind: "pick",
    def: "fills",
    options: measures,
    group: "Mix",
    when: (value) => mixed(value) && paired(value),
  },
  {
    key: "arg",
    label: "Argument",
    kind: "slider",
    def: 2,
    min: (value) => (value.op === "scale" ? ARG.min : value.op === "decimate" ? 2 : 1),
    max: ARG.max,
    step: 1,
    group: "Mix",
    when: (value) => mixed(value) && takes(value),
  },
];

export const filed = (value: Value) => {
  const dim = Number(value.dim);
  const row = print({
    dim,
    base: dim === 3 ? 2 : value.base,
    code: String(value.code ?? "").replace(/\W+/g, ""),
    measure: value.measure,
    axis: value.axis,
  } as Key);
  return ["plot", row, ...(mixed(value) ? [value.op] : []), value.chart].join("-");
};

export const units = wasm("math", "num");

export const PAGE: Page<Units<"math" | "num">, Held> = {
  defaults: { dim: 2, base: 2, code: "7" },
  design: {
    dim: "dim",
    base: "base",
    code: "code",
    spaces: [
      [2, 2],
      [2, 3],
      [3, 2],
    ],
  },
  units,
  spec: SPEC,
  name: filed,
  about: [
    "One sequence of the ledger drawn large. Pick a design, a measure and an axis, and the terms stand up as pins, a log plot, steps, partial sums, ratios, a difference triangle or a heatmap of their digits.",
    "Under the chart is the rule: the smallest linear recurrence every term satisfies, its characteristic polynomial and its largest real root, the growth. A rule needs about twice its order plus two terms before it can be trusted, so a short read may find none where a longer one would.",
    "The mix runs a term operation over this sequence and a second one from the same space and axis: add, subtract, multiply, convolve, or shift, thin, difference, sum and scale the first alone. Sequences with linear recurrences are closed under every one of these, so the mix has a rule of its own.",
    "Terms are exact. The rule and the mix work within 128 bits and say where they stop.",
  ],
  keys: [
    { key: "Enter", label: "Mix", act: "mix", button: true, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Design", act: "design" },
    { key: ["ArrowUp", "ArrowDown"], label: "Terms", act: "terms" },
    { key: "c", label: "Chart", act: "cycle", menu: "View" },
  ],
  actions: {
    mix: (scene) => scene.mix?.(),
    design: (scene, e) => scene.design?.(e.key === "ArrowLeft" ? -1 : 1),
    terms: (scene, e) => scene.terms?.(e.key === "ArrowDown" ? -1 : 1),
    cycle: (scene) => scene.cycle?.(),
  },
};

/* RULE */

export function lines(rule: Rule): [string, string] {
  if (rule.error) return [`no rule read: ${rule.error}`, ""];
  if (rule.order === null) return ["no linear rule fits these terms", `read from ${rule.read} terms`];
  const growth = Number.isFinite(rule.growth)
    ? `growth ${decimal(rule.growth)}, ${decimal(rule.exponent)} decades a term`
    : rule.from;
  return [rule.recurrence, `${rule.polynomial} · ${growth} · read from ${rule.read} terms`];
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Fed = {}) {
  const ctx = canvas.getContext("2d")!;
  let { chart: kind, digits: base } = tidy(SPEC, opts) as Knobs;
  const feed = () => opts.live?.current ?? opts;
  const grown = growth(view);
  let seen: number | null = null;
  let dirty = true;
  let moving = false;
  let picked: number[] = [];
  let hits: (Slots | null)[] = [];
  const panel = (one: Panel, box: Rect, i: number, t: Tones, most: number) => {
    const { px } = t;
    const row = one.row;
    const color = shade(t.accent, t.paper, i, 2);
    const top = legend(ctx, [row], [color], t, box.x, box.y, box.w, true);
    const laid = ribbon(ctx, row, t, box.w, most);
    const rule = lines(one.rule);
    const tail = laid.height + px * 4.4;
    const area = { x: box.x, y: box.y + top.height, w: box.w, h: Math.max(px * 6, box.h - top.height - tail) };
    const at = picked[i] ?? -1;
    const slots = row.terms.length
      ? chart(ctx, area, [row], [color], t, { kind, base, at, shown: () => grown.shown(row) })
      : null;
    if (!row.terms.length) {
      ctx.fillStyle = t.dim;
      ctx.fillText(one.note || "no terms read", area.x, area.y + px * 2);
    }
    const y = area.y + area.h + px * 0.6;
    drawRibbon(ctx, laid, box.x, y, t, at, color);
    ctx.font = font(t, 1.05, "500");
    ctx.fillStyle = t.accent;
    ctx.fillText(fit(ctx, rule[0], box.w), box.x, y + laid.height + px * 1.6);
    ctx.font = font(t, 0.9);
    ctx.fillStyle = t.dim;
    ctx.fillText(fit(ctx, rule[1], box.w), box.x, y + laid.height + px * 3.1);
    if (one.note && row.terms.length) ctx.fillText(fit(ctx, one.note, box.w), box.x, y + laid.height + px * 4.3);
    ctx.font = font(t);
    return slots;
  };
  const paint = (panels: Panel[]) => {
    const t = tones(view);
    clear(ctx, view, t);
    const { x: pad, top: high, bottom: low } = t.edge;
    const gap = Math.round(t.px * 2);
    if (!panels.length) {
      ctx.fillStyle = t.dim;
      ctx.textAlign = "center";
      ctx.fillText("reading", view.w / 2, view.h / 2);
      ctx.textAlign = "left";
      hits = [];
      return;
    }
    const inner = { x: pad, y: high, w: view.w - 2 * pad, h: view.h - high - low };
    const two = panels.length > 1;
    const across = two && inner.w > inner.h * 1.3;
    const boxes = panels.map((_, i) => {
      if (!two) return inner;
      if (across) return { x: inner.x + i * ((inner.w + gap) / 2), y: inner.y, w: (inner.w - gap) / 2, h: inner.h };
      return { x: inner.x, y: inner.y + i * ((inner.h + gap) / 2), w: inner.w, h: (inner.h - gap) / 2 };
    });
    hits = panels.map((one, i) => panel(one, boxes[i], i, t, two ? 2 : 3));
  };
  const draw = () => {
    const data = feed();
    const panels = Array.isArray(data.panels) ? data.panels : [];
    const version = data.version ?? 0;
    const rows = panels.map((one) => one.row);
    const busy = grown.busy(rows);
    if (!view.still && !dirty && version === seen && !busy && !moving) return;
    moving = busy;
    if (version !== seen) picked = picked.map((at, i) => (at < (rows[i]?.terms.length ?? 0) ? at : -1));
    seen = version;
    dirty = false;
    paint(panels);
  };
  const again = () => {
    dirty = true;
    draw();
  };
  const set = (value: Opts) => {
    ({ chart: kind, digits: base } = tidy(SPEC, value) as Knobs);
    dirty = true;
  };
  const tap = (x: number, y: number) => {
    for (let i = 0; i < hits.length; i++) {
      const j = slotAt(hits[i], x, y);
      if (j >= 0) {
        picked[i] = picked[i] === j ? -1 : j;
        again();
        return { panel: i, term: picked[i] };
      }
    }
    return null;
  };
  return { draw, size: again, theme: again, set, tap, font: FACE };
}
