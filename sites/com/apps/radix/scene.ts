import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Had } from "@mrlyprod/ui/kit/hooks.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Box, Design, Facts, Num, Pair, Plan, Preset, Shaped, Word } from "./engine.ts";
import {
  bases,
  cap,
  corners,
  count,
  DRAWN,
  dial,
  every,
  flat,
  GAP,
  glance,
  KEEP,
  KOCH,
  LETTERS,
  lattice,
  lay,
  PAD,
  PRESETS,
  pair,
  picture,
  preset,
  RINGS,
  reach,
  rolled,
  SMALL,
  spell,
  study,
  THIN,
  twisted,
  wears,
} from "./engine.ts";

export type Knobs = Design & { level: number; line: number; cell: number; tint: string };

export type Sketch = Design & { level?: number; line?: number };

export type Given = Opts & { mrly?: Doors | null; onFacts?: (facts: Facts) => void };

export type Seen = Pick<View, "w" | "h" | "dpr"> & { look: () => Pick<Paints, "paper" | "accent"> };

export type Moves = { flat: () => void; every: () => void };

export type Doors = Mrly & Had<"gaussian">;

export type Listed = [Preset, Shaped | null];

export type Radix = ReturnType<typeof make>;

export const units = wasm(...CRATES, "gaussian");

let shelved: { num: Num; list: Listed[] } | null = null;

export function presets(mrly: Doors): Listed[] {
  const num = mrly.gaussian;
  if (shelved?.num !== num) shelved = { num, list: PRESETS.map(([name]): Listed => [name, preset(num, mrly, name)]) };
  return shelved.list;
}

export function next(name: string, by: number) {
  const at = PRESETS.findIndex(([one]) => one === name);
  if (at < 0) return PRESETS[by > 0 ? 0 : PRESETS.length - 1][0];
  return PRESETS[(at + by + PRESETS.length) % PRESETS.length][0];
}

const dialed = (value: Value, units?: Doors) => dial(units!.gaussian, units!, value as unknown as Design);

export type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

const OWN = () => false;

export const SPEC: Spec = [
  { key: "ring", label: "Ring", kind: "pick", def: KOCH.ring, options: RINGS, group: "Design", when: OWN },
  {
    key: "base",
    label: "Base",
    kind: "pick",
    def: KOCH.base,
    options: (value) => bases(value.ring as Word),
    group: "Design",
    when: OWN,
  },
  { key: "d", label: "Digits", kind: "text", def: KOCH.d, group: "Design", when: OWN },
  { key: "t", label: "Twists", kind: "text", def: KOCH.t, group: "Design", when: OWN },
  {
    key: "level",
    label: (value) => `Level 1 to ${cap(value)}`,
    kind: "slider",
    def: KOCH.level,
    min: 1,
    max: (value) => cap(value),
    step: 1,
    group: "Design",
    when: OWN,
  },
  { key: "line", label: "Drawn as", kind: "segment", def: KOCH.line, options: DRAWN, group: "Design", when: OWN },
  {
    key: "cell",
    label: "Cell",
    kind: "slider",
    def: 0,
    min: 0,
    max: 32,
    step: 1,
    unit: "px",
    zero: "Fit",
    group: "Look",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const file = (value: Value) =>
  [
    "radix",
    value.ring,
    spell(LETTERS[value.ring as Word] ?? "i", ...(pair(value.base) as Pair)).replaceAll(" ", ""),
    value.level,
    value.line ? "line" : "cells",
  ].join("-");

export const PAGE = {
  spec: SPEC,
  name: file,
  keys: [
    { key: "Enter", label: "Drop the twists", act: "flat", button: true, menu: "Edit", when: twisted },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Preset", act: "preset" },
    { key: "a", label: "Every residue", act: "every", menu: "Edit" },
  ],
  units,
  acts: {
    flat: (value, _key, units) => flat(dialed(value, units)),
    level: (value, key) => ({ level: Number(value.level) + (key === "ArrowDown" ? -1 : 1) }),
    preset: (value, key, units) => {
      const was = presets(units!).find(([, one]) => wears(value as unknown as Design, one))?.[0] ?? "";
      return preset(units!.gaussian, units!, next(was, key === "ArrowLeft" ? -1 : 1)) ?? {};
    },
    every: (value, _key, units) => every(dialed(value, units)),
  },
  reroll: (value, seed, units) => rolled(dialed(value, units), rng(seed)),
} satisfies Page<Loaded<Crate | "gaussian">, Radix>;

const CHUNK = 8192;
const DESIGN: (keyof Knobs)[] = ["ring", "base", "d", "t"];
const LOOK: (keyof Knobs)[] = ["level", "line", "cell", "tint"];

export function splat(pen: Pen, plan: Plan, box: Box, at: number, line: boolean, accent: string, dpr: number) {
  const { sides, norm, angle, sign } = plan;
  const pts = plan.levels[at - 1];
  const n = pts.length / 2;
  const { spacing, radius } = lattice(norm, sides, at);
  const [m11, m12, m21, m22] = box.quarter ? [0, -box.k, box.k, 0] : [box.k, 0, 0, box.k];
  const { cx, cy, ox, oy } = box;
  const x = (i: number) => ox + m11 * (pts[2 * i] - cx) + m12 * (pts[2 * i + 1] - cy);
  const y = (i: number) => oy + m21 * (pts[2 * i] - cx) + m22 * (pts[2 * i + 1] - cy);
  if (line) {
    pen.strokeStyle = accent;
    pen.lineWidth = Math.min(2 * dpr, Math.max(0.6 * dpr, spacing * box.k * THIN));
    pen.lineJoin = "round";
    pen.lineCap = "round";
    pen.beginPath();
    pen.moveTo(x(0), y(0));
    for (let i = 1; i < n; i++) pen.lineTo(x(i), y(i));
    pen.stroke();
    return;
  }
  pen.fillStyle = accent;
  pen.beginPath();
  let open = 0;
  const flush = () => {
    pen.fill();
    pen.beginPath();
    open = 0;
  };
  if (2 * radius * box.k < SMALL * dpr) {
    const s = Math.max(spacing * reach(sides, angle, at) * box.k, dpr);
    for (let i = 0; i < n; i++) {
      pen.rect(x(i) - s / 2, y(i) - s / 2, s, s);
      if (++open >= CHUNK) flush();
    }
  } else {
    const ring = corners(sides, angle, at, radius * box.k * (1 - GAP));
    for (let j = 0; j < sides; j++) {
      const vx = ring[2 * j];
      const vy = ring[2 * j + 1] * sign;
      ring[2 * j] = box.quarter ? -vy : vx;
      ring[2 * j + 1] = box.quarter ? vx : vy;
    }
    for (let i = 0; i < n; i++) {
      const px = x(i);
      const py = y(i);
      pen.moveTo(px + ring[0], py + ring[1]);
      for (let j = 1; j < sides; j++) pen.lineTo(px + ring[2 * j], py + ring[2 * j + 1]);
      pen.closePath();
      if (++open >= CHUNK) flush();
    }
  }
  pen.fill();
}

const EDGE = 0.08;

export function sketch(
  pen: CanvasRenderingContext2D,
  units: Doors,
  patch: Sketch,
  side: number,
  dpr: number,
  accent: string,
) {
  const line = Boolean(patch.line);
  const plan = study(units.gaussian, units, patch, glance(count(patch.d), patch.level));
  const box = lay(line ? plan.bounds : plan.stage, side, side, side * EDGE);
  pen.setTransform(1, 0, 0, 1, 0, 0);
  pen.clearRect(0, 0, side, side);
  splat(pen, plan, box, plan.cap, line, accent, dpr);
  plan.radix.free();
}

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { mrly, onFacts } = opts;
  if (!mrly) throw new Error("radix: the scene wants the design crates and @mrlyprod/gaussian as opts.mrly");
  const num = mrly.gaussian;
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let plan = study(num, mrly, value);
  const held = new Map<number, OffscreenCanvas>();
  const spare: OffscreenCanvas[] = [];
  let box: Box | null = null;
  let accent = "#000";
  let last = "";
  let gone = false;
  const level = () => Math.min(value.level, plan.cap);
  const tell = () => {
    if (!gone) onFacts?.(plan.facts(level()));
  };
  const paint = () => {
    accent = view.look().accent;
  };
  const fit = () => {
    const pinned = value.cell > 0 ? (value.cell * view.dpr) / lattice(plan.norm, plan.sides, level()).spacing : 0;
    box = lay(value.line ? plan.bounds : plan.stage, view.w, view.h, PAD * view.dpr, pinned);
  };
  const page = () => spare.pop() ?? new OffscreenCanvas(view.w, view.h);
  const lays = (sheet: OffscreenCanvas, at: number) => {
    const pen = sheet.getContext("2d")!;
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.globalAlpha = 1;
    pen.clearRect(0, 0, view.w, view.h);
    splat(pen, plan, box!, at, Boolean(value.line), accent, view.dpr);
  };
  const want = (at: number) => {
    if (held.has(at)) {
      const kept = held.get(at)!;
      held.delete(at);
      held.set(at, kept);
      return kept;
    }
    while (held.size >= KEEP) {
      const [old, sheet] = held.entries().next().value!;
      held.delete(old);
      spare.push(sheet);
    }
    const sheet = page();
    lays(sheet, at);
    held.set(at, sheet);
    return sheet;
  };
  const release = () => {
    spare.push(...held.values());
    held.clear();
    last = "";
  };
  const draw = () => {
    const at = level();
    if (String(at) === last) return;
    last = String(at);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(want(at), 0, 0);
  };
  const set = (next: Opts) => {
    const was = value;
    value = tidy(SPEC, next) as Knobs;
    const remade = DESIGN.some((key) => value[key] !== was[key]);
    if (!remade && LOOK.every((key) => value[key] === was[key])) return;
    if (remade) {
      plan.radix.free();
      plan = study(num, mrly, value);
    }
    if (value.tint !== was.tint) paint();
    const scaled =
      remade || value.cell !== was.cell || value.line !== was.line || (value.cell > 0 && value.level !== was.level);
    if (scaled) fit();
    if (scaled || value.tint !== was.tint) release();
    last = "";
    if (remade || value.level !== was.level) tell();
  };
  const size = () => {
    fit();
    held.clear();
    spare.length = 0;
    last = "";
  };
  const theme = () => {
    paint();
    release();
  };
  const stop = () => {
    gone = true;
  };
  paint();
  fit();
  tell();
  return { draw, set, size, theme, stop, still: true, svg: () => picture(plan, accent, level(), value.line) };
}
