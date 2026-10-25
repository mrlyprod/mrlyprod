import type { Had } from "../../lib/app/hooks.ts";
import type { Mathy, Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Raw, Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Run } from "../../lib/app/svg.ts";
import { runs } from "../../lib/app/svg.ts";
import type { Paints } from "../../lib/color.ts";
import { HUES, mix, rgb, TINTS } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Name } from "../../lib/theme/theme.ts";
import { dark, light } from "../../lib/theme/theme.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Curves, Drawn, Fit, Inks, Look, Plan, Shades, Studied, Told } from "./engine.ts";
import { arc, BASES, cap, DIM, FADE, fit, LOOKS, PAD, picture, resolve, study, thick } from "./engine.ts";

/* TYPES */

export type Knobs = {
  base: number;
  code: string;
  level: number;
  cells: number;
  look: Look;
  width: number;
  tint: string;
  strand: string;
};

export type Given = Opts & { math?: Mrly | null; onTell?: (facts: Told) => void };

export type Seen = Pick<View, "w" | "h" | "dpr" | "t" | "still" | "wake"> & {
  look: () => Pick<Paints, "paper" | "accent">;
};

export type Hands = { step: (by: number) => void; deeper: (by: number) => void };

export type Arcs = ReturnType<typeof make>;

export type Handled = Arcs & Hands;

export type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

type Matrix = [number, number, number, number, number, number];

const HIDE = () => false;

export const SPEC: Spec<Had<"math">> = [
  { key: "base", label: "Base", kind: "segment", def: 2, options: BASES, when: HIDE },
  { key: "code", label: "Code", kind: "text", def: "", when: HIDE },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 5,
    min: 1,
    max: (value, units) => units && cap(units.math, value.base),
    step: 1,
    group: "Arcs",
  },
  { key: "cells", label: "Shade cells", kind: "toggle", def: 1, group: "Arcs" },
  { key: "look", label: "Show", kind: "segment", def: "all", options: LOOKS, group: "Arcs" },
  { key: "width", label: "Width", kind: "slider", def: 0, min: 0, max: 0.5, step: 0.02, zero: "Auto", group: "Arcs" },
  { key: "tint", label: "Loops", kind: "pick", def: "orange", options: TINTS, group: "Colour" },
  { key: "strand", label: "Strands", kind: "pick", def: "blue", options: TINTS, group: "Colour" },
];

export const ABOUT = [
  "Every cell carries two quarter arcs, each joining the midpoints of two neighbouring edges. A filled cell takes the arcs round its lower-left and upper-right corners, an empty cell the other two.",
  "The arcs join into curves. Strands run from border to border, and there are always twice the side of them. Loops close on themselves, and the design decides how many.",
  "A law shows where one is proved: the carpet at base 3 and all sixteen codes at base 2. The library holds the designs whose law counts some loops. Every count comes from the crate, by union-find over the edge midpoints, and row 0 is at the bottom.",
  "Trace, or a tap, lights the loops one by one. Reroll rolls a design that loops at this level. [Designs](/designs/) lists the designs, and [Two](/two/) repeats one into a sheet.",
];

const units = wasm("math");

export const file = (value: Value, unit?: Mathy) =>
  `arcs-${unit?.math ? resolve(unit.math, value.base as number, value.code) : String(value.code ?? "").trim() || "carpet"}-base${value.base}-level${value.level}`.replace(
    /[^\w.+-]+/g,
    "-",
  );

export const PAGE = {
  spec: SPEC,
  units,
  name: file,
  about: ABOUT,
  design: { dim: DIM, base: "base", code: "code", bases: BASES.map(([base]) => base) },
  keys: [
    { key: "Enter", label: "Trace", act: "trace", button: true, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Design", act: "design" },
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
  ],
  actions: {
    trace: (scene) => scene.trace?.(),
    design: (scene, e) => scene.step?.(e.key === "ArrowLeft" ? -1 : 1),
    up: (scene) => scene.deeper?.(1),
    down: (scene) => scene.deeper?.(-1),
  },
} satisfies Page<Loaded<"math">, Handled>;

const CHUNK = 8192;

const EDGE = 0.08;

const FLIP = (h: number): Matrix => [1, 0, 0, -1, 0, h];

const bright = (color: string) => rgb(color).reduce((a, b) => a + b, 0) > 382;

export function shades(canvas: Element | null, look: Pick<Paints, "paper" | "accent">, strand: string): Shades {
  const css = typeof getComputedStyle === "function" && canvas?.nodeType ? getComputedStyle(canvas) : null;
  const read = (name: string, fallback: string) => css?.getPropertyValue(name).trim() || fallback;
  const other = HUES.includes(strand) ? read(`--${strand}`, palette[strand as Name]) : read("--accent", light.accent);
  return {
    loop: look.accent,
    strand: other,
    dim: mix(rgb(look.accent), rgb(look.paper), FADE),
    fill: read("--line", bright(look.paper) ? light.line : dark.line),
  };
}

/* STROKES */

export function quarter(target: Pen, plan: Pick<Curves, "side" | "cells">, box: Fit, a: number, d: number, f: number) {
  const g = arc(plan, a);
  const r = box.px / 2;
  const cx = box.x + g.cx * box.px;
  const cy = box.y + g.cy * box.px;
  if (d > 0) {
    target.moveTo(cx, box.y + (g.y + 0.5) * box.px);
    target.arc(cx, cy, r, g.av, g.av + g.sweep * f, g.sweep < 0);
  } else {
    target.moveTo(box.x + (g.x + 0.5) * box.px, cy);
    target.arc(cx, cy, r, g.ah, g.ah - g.sweep * f, g.sweep > 0);
  }
}

export function strokes(target: Pen, plan: Curves, box: Fit, loops: number) {
  const { order, dir, bounds, closed } = plan;
  let open = 0;
  target.beginPath();
  for (let k = 0; k < closed.length; k++) {
    if (closed[k] !== loops) continue;
    for (let j = bounds[k]; j < bounds[k + 1]; j++) {
      quarter(target, plan, box, order[j], dir[j], 1);
      if (++open >= CHUNK) {
        target.stroke();
        target.beginPath();
        open = 0;
      }
    }
  }
  if (open) target.stroke();
}

export function shade(target: Pen, spans: readonly Run[], box: Fit) {
  target.beginPath();
  for (const [row, col, len] of spans) target.rect(box.x + col * box.px, box.y + row * box.px, len * box.px, box.px);
  target.fill();
}

export function sketch(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  plan: Curves,
  value: Drawn,
  inks: Inks,
) {
  const box = fit(plan.side, w, h, w * EDGE);
  ctx.setTransform(...FLIP(h));
  ctx.lineCap = "round";
  ctx.lineWidth = thick(value.width, box.px, dpr);
  if (value.cells) {
    ctx.fillStyle = inks.fill;
    shade(ctx, runs({ shape: [plan.side, plan.side], types: plan.filled }), box);
  }
  if (value.look !== "loops") {
    ctx.strokeStyle = inks.strand;
    strokes(ctx, plan, box, 0);
  }
  if (value.look !== "strands") {
    ctx.strokeStyle = inks.loop;
    strokes(ctx, plan, box, 1);
  }
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { math, onTell } = opts;
  if (!math) throw new Error("arcs: the scene wants @mrlyprod/mrly/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const clean = (next: Raw) => tidy(SPEC, next, { math }) as Knobs;
  let value = clean(opts);
  let plan: Studied | null = null;
  let spans: Run[] = [];
  let inks: Shades | null = null;
  let box: Fit | null = null;
  let layer: OffscreenCanvas | null = null;
  let pen: OffscreenCanvasRenderingContext2D | null = null;
  let stamped = 0;
  let tracing: { from: number } | null = null;
  let dirty = true;
  const plot = () => {
    plan = study(math, value);
    spans = plan.side ? runs({ shape: [plan.side, plan.side], types: plan.filled }) : [];
    tracing = null;
    onTell?.(plan.facts);
  };
  const paint = () => {
    inks = shades(canvas, view.look(), value.strand);
  };
  const width = () => thick(value.width, box!.px, view.dpr);
  const sweep = (target: Pen, a: number, d: number, f: number) => quarter(target, plan! as Plan, box!, a, d, f);
  const trace = (target: Pen, k: number, f: number) => {
    const { order, dir, bounds } = plan! as Plan;
    const from = bounds[k];
    const n = bounds[k + 1] - from;
    const whole = f >= 1 ? n : Math.floor(f * n);
    const part = f >= 1 ? 0 : f * n - whole;
    target.beginPath();
    for (let j = 0; j < whole; j++) sweep(target, order[from + j], dir[from + j], 1);
    if (part > 0) sweep(target, order[from + whole], dir[from + whole], part);
    target.stroke();
  };
  const lay = (target: Pen, loops: number, color: string) => {
    target.strokeStyle = color;
    target.lineWidth = width();
    strokes(target, plan! as Plan, box!, loops);
  };
  const blank = (): [OffscreenCanvas, OffscreenCanvasRenderingContext2D] => {
    const sheet = new OffscreenCanvas(view.w, view.h);
    const ink = sheet.getContext("2d")!;
    ink.setTransform(...FLIP(view.h));
    ink.lineCap = "round";
    ink.lineJoin = "round";
    return [sheet, ink];
  };
  const reset = () => {
    box = fit((plan! as Plan).side, view.w, view.h, PAD * view.dpr);
    const [back, ink] = blank();
    if (value.cells && spans.length) {
      ink.fillStyle = inks!.fill;
      shade(ink, spans, box);
    }
    if (value.look !== "loops") lay(ink, 0, inks!.strand);
    if (value.look !== "strands") lay(ink, 1, tracing ? inks!.dim : inks!.loop);
    if (!tracing) {
      layer = back;
      return;
    }
    [layer, pen] = blank();
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.drawImage(back, 0, 0);
    pen.setTransform(...FLIP(view.h));
    stamped = 0;
  };
  const extend = (now: number) => {
    const { clock, lit } = plan! as Plan;
    if (stamped >= clock.byEnd.length) return;
    pen!.strokeStyle = inks!.loop;
    pen!.lineWidth = width();
    while (stamped < clock.byEnd.length && clock.end[clock.byEnd[stamped]] <= now)
      trace(pen!, lit[clock.byEnd[stamped++]], 1);
  };
  const draw = () => {
    if (!dirty && !tracing) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (!plan!.side) return;
    if (!layer) reset();
    const now = tracing ? view.t - tracing.from : Infinity;
    if (tracing && now >= plan!.clock.span) {
      tracing = null;
      reset();
    } else if (tracing) extend(now);
    ctx.drawImage(layer!, 0, 0);
    if (!tracing) return;
    const { clock, lit } = plan!;
    ctx.setTransform(...FLIP(view.h));
    ctx.strokeStyle = inks!.loop;
    ctx.lineWidth = width();
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    for (const k of clock.byStart) {
      if (clock.start[k] > now) break;
      if (clock.end[k] > now) trace(ctx, lit[k], (now - clock.start[k]) / clock.dur[k]);
    }
  };
  const touch = () => {
    layer = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    touch();
  };
  const start = () => {
    if (!plan!.side || value.look === "strands" || !plan!.lit.length) return false;
    if (view.still) view.wake?.();
    tracing = { from: view.t };
    touch();
    draw();
    return true;
  };
  const set = (next: Opts) => {
    const was = value;
    value = clean(next);
    const same = (keys: (keyof Knobs)[]) => keys.every((key) => value[key] === was[key]);
    if (!same(["base", "code", "level"])) plot();
    else if (same(["look", "cells", "width", "tint", "strand"])) return;
    if (value.look === "strands") tracing = null;
    if (!same(["tint", "strand"])) paint();
    touch();
  };
  const svg = () => (plan!.side ? picture(plan!, value, inks!) : null);
  paint();
  plot();
  return {
    still: true,
    draw,
    size: touch,
    theme,
    set,
    trace: start,
    svg,
    get facts() {
      return plan!.facts;
    },
  };
}
