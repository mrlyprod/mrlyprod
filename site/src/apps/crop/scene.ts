import type * as mrly from "@mrlyprod/mrly/math";
import type { Mathy, Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Raw, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { num, runs, sheet } from "../../lib/app/svg.ts";
import type { Paints } from "../../lib/color.ts";
import { paints, TINTS, veil } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { DESIGN } from "../../lib/designs/engine.ts";
import type { Design } from "../../lib/designs/store.ts";
import type { Memo, Paint, Pen } from "../../lib/two/engine.ts";
import { cell, DIM, design, lay, memo, named, shot, stamp } from "../../lib/two/engine.ts";
import type { Titled } from "../../lib/two/parts.tsx";
import { second } from "../../lib/two/scene.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Census, Circle, Kept, Look, Masked, Outline, Plane } from "./engine.ts";
import {
  CUT,
  census,
  circle,
  counted,
  exposed,
  grid,
  IN,
  keep,
  layers,
  OUT,
  outline,
  POLICIES,
  RDEN,
  RMAX,
  room,
  SHAPES,
  shape,
  top,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  number: number;
  level: number;
  shape: string;
  radius: number;
  mode: string;
  policy: string;
  edge: number;
  tint: string;
  circle: number;
  centre: string;
  count: number;
  r: number;
};

export type Framed = { shape: string; radius: number; mode: string; policy: string; edge: number };

export type Stage = { x: number; y: number; w: number; h: number };

type Staged = {
  code: string;
  types: mrly.Tensor;
  before: number;
  shape: Masked;
  outline: Outline;
  census: Census;
  regions: mrly.Tensor["data"];
  kept: Kept;
  look: Look;
  circle: Circle;
};

export type Stages = { [K in keyof Staged]?: Memo<Staged[K]> };

export type Plan = ReturnType<typeof plan>;

export type Facts = ReturnType<Plan["facts"]>;

export type Looks = { look: () => Pick<Paints, "paper" | "accent"> };

export type Ink = ReturnType<typeof inks>;

export type Seen = Pick<View, "w" | "h" | "dpr"> & Looks;

export type Given = Opts & { math?: Mrly | null };

export type Hands = { flip: () => void; grow: (by: number) => void; deeper: (by: number) => void };

export type Crop = ReturnType<typeof make>;

export type Handled = Crop & Hands;

const never = () => false;

const counting = (value: Value) => Number(value.circle) === 1;

const most = (value: Value, units: Mathy) =>
  room(units.math!, Number(value.number), Number(value.count), String(value.centre)).most;

export const SPEC: Titled<Mathy>[] = [
  { key: "number", label: "Number", kind: "slider", def: 3, min: 2, max: 7, step: 1, group: "Design", when: never },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value, units) => units && top(units.math!, Number(value.number)),
    step: 1,
    keep: true,
    group: "Design",
    when: never,
  },
  { key: "shape", label: "Shape", kind: "pick", def: "ball", options: SHAPES, group: "Shape", when: never },
  {
    key: "radius",
    label: "Radius",
    kind: "slider",
    def: 60,
    min: 0,
    max: RMAX,
    step: 1,
    unit: `/${RDEN}`,
    group: "Shape",
  },
  {
    key: "mode",
    label: "Mode",
    kind: "segment",
    def: "crop",
    options: [
      ["crop", "Crop"],
      ["anti", "Anti"],
    ],
    group: "Shape",
  },
  { key: "policy", label: "Policy", kind: "pick", def: "touching", options: POLICIES, group: "Shape" },
  { key: "edge", label: "Exact edge", kind: "toggle", def: 0, group: "Shape" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
  { key: "circle", label: "Count", kind: "toggle", def: 0, group: "Circle" },
  {
    key: "centre",
    label: "Centre",
    kind: "segment",
    def: "corner",
    options: [
      ["corner", "Corner"],
      ["centre", "Centre"],
    ],
    group: "Circle",
    when: counting,
  },
  {
    key: "count",
    label: "Count level",
    kind: "slider",
    def: 6,
    min: 1,
    max: (value, units) => units && counted(units.math!, Number(value.number)),
    step: 1,
    keep: true,
    group: "Circle",
    when: counting,
  },
  {
    key: "r",
    label: "Radius r",
    kind: "slider",
    def: 27,
    min: 1,
    max: (value, units) => units && Math.max(1, most(value, units)),
    unit: (value, units) => units && `/${most(value, units)}`,
    step: 1,
    keep: true,
    group: "Circle",
    when: counting,
  },
];

export const name = (value: Value) =>
  `crop-${String(value.code ?? "").trim() || value.seed}-${value.shape}-${value.radius}${value.mode === "anti" ? "-anti" : ""}`;

export const units = wasm("math");

const framed = (shape: string, radius: number, mode = "crop", policy = "touching", edge = 0): Framed => ({
  shape,
  radius,
  mode,
  policy,
  edge,
});

export const WINDOWS: [string, Framed][] = [
  ["Porthole", framed("ball", 36)],
  ["Moon", framed("ball", 84)],
  ["Lozenge", framed("diamond", 72)],
  ["Arrowhead", framed("triangle", 66)],
  ["Stop", framed("octagon", 66)],
  ["Frame", framed("box", 54, "anti")],
  ["Eclipse", framed("ball", 66, "anti")],
  ["Exact", framed("ball", 66, "crop", "touching", 1)],
  ["Fine rim", framed("ball", 66, "crop", "refined2")],
  ["Inside", framed("ball", 66, "crop", "inside")],
];

export const PAGE = {
  defaults: { base: 2, code: "7" },
  design: { dim: DIM, base: "base", code: "code" },
  spec: SPEC,
  units,
  gestures: true,
  about: [
    "A shape of rational radius sits on the unit square and keeps only the cells of the design it reaches: strictly inside, touching, or rebuilt on a finer lattice at the rim. Anti keeps the cells outside it.",
    "The census splits every cell into in, cut and out before anything is drawn. Drag the picture to set the radius: it is the distance from the centre.",
    "Count reads a deeper grid in whole cells and leads the readouts with it: N(r) the filled cells a ball of radius r holds, inside(r) those wholly in it, C(r) those its circle crosses. Info lists every number.",
    "The exact classification, the census of the three regions and the circle theorem are in [the crop note](/research/crop/). [Two](/two/) repeats the whole design into a sheet, and [Three](/three/) crops the cube.",
  ],
  name,
  keys: [
    { key: "Enter", label: "Anti", act: "mode", button: true, menu: "Edit", when: (value) => value.mode !== "anti" },
    { key: "Enter", label: "Crop", act: "mode", button: true, menu: "Edit", when: (value) => value.mode === "anti" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Radius", act: "radius" },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", menu: "View" },
  ],
  actions: {
    mode: (scene) => scene.flip?.(),
    radius: (scene, e) => scene.grow?.(e.key === "ArrowLeft" ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === "ArrowDown" ? -1 : 1),
  },
} satisfies Page<Loaded<"math">, Handled>;

const BOX = 0.06;
const FAINT = 0.2;
const RULE = 1.5;
const FIT = 0.9;
const UNIT = 10;

/* PLAN */

export function plan(math: Mrly, next: Raw, stages: Stages = {}) {
  const at = <K extends keyof Staged>(name: K): Memo<Staged[K]> => {
    const held: { [P in K]?: Memo<Staged[P]> } = stages;
    held[name] ??= memo();
    return held[name];
  };
  const value = { ...(tidy(DESIGN, next) as Design), ...(tidy(SPEC, next, { math }) as Knobs) };
  const { base, number, level, radius, policy, centre, count: deep, r } = value;
  const anti = value.mode === "anti";
  const edge = Boolean(value.edge);
  const code = at("code")(`${base}:${value.code}:${next.seed}`, () => design(math, base, value.code));
  const one = [code, base, number, level].join();
  const types = at("types")(one, () => grid(math, code, number, level, base));
  const before = at("before")(one, () => exposed(math, types));
  const form = [value.shape, radius, anti].join();
  const made = at("shape")(form, () => shape(math, value.shape, radius, anti));
  const edges = at("outline")(form, () => outline(anti ? (made as { Anti: Plane }).Anti : (made as Plane)));
  const both = `${one}|${form}`;
  const tally = at("census")(both, () => census(math, types, made));
  const regions = at("regions")(both, () => math.shape.regions(made, types.shape).data);
  const rule = edge ? "touching" : policy;
  const kept = at("kept")(`${both}|${rule}`, () => keep(math, types, made, rule, number));
  const look = at("look")(`${both}|${rule}|${edge}`, () => layers(types, regions, kept, edge));
  const table = counting(value)
    ? at("circle")([code, base, number, deep, centre].join(), () => circle(math, code, number, deep, base, centre))
    : null;
  const read = { ...value, code, anti, edge, rule };
  const facts = () => ({
    code,
    name: named(math, code, base),
    side: types.shape[0],
    level,
    note: edge ? "exact edge" : kept.over ? `${policy} over budget, touching` : policy,
    in: tally.filled[IN],
    cut: tally.filled[CUT],
    out: tally.filled[OUT],
    before,
    after: tally.after,
    circle: table && {
      side: table.side,
      r,
      seen: table.seen[r],
      inside: table.inside[r],
      cut: table.cut[r],
      d: table.d,
      mass: table.mass,
    },
  });
  return { value: read, types, made, edges, tally, kept, look, table, facts };
}

/* INK */

function inks(canvas: Element | null | undefined, view: Looks, tint: string) {
  const { paper, accent } = view.look();
  const rule = paints(canvas, "pink").accent;
  return {
    paper,
    accent,
    cut: second(canvas, tint, accent),
    rule: rule === accent ? paints(canvas, "red").accent : rule,
  };
}

/* CROP */

function trace(pen: Pen, edges: Outline, box: Stage) {
  pen.beginPath();
  if (edges.circle) {
    const [cx, cy, r] = edges.circle;
    pen.arc(box.x + cx * box.w, box.y + cy * box.h, Math.max(0, r * box.w), 0, Math.PI * 2);
    return;
  }
  edges.points.forEach(([x, y], i) => {
    const px = box.x + x * box.w;
    const py = box.y + y * box.h;
    if (i) pen.lineTo(px, py);
    else pen.moveTo(px, py);
  });
  pen.closePath();
}

function crop(pen: Pen, now: Plan, box: Stage, ink: Ink, dpr: number) {
  const { look, edges, value } = now;
  const coarse = { px: box.w / look.coarse, x: box.x, y: box.y };
  const fine = { px: box.w / look.side, x: box.x, y: box.y };
  pen.fillStyle = veil(ink.accent, BOX);
  pen.fillRect(box.x, box.y, box.w, box.h);
  stamp(pen, look.faint, coarse, veil(ink.accent, FAINT));
  if (value.edge) {
    pen.save();
    if (value.anti) {
      trace(pen, edges, box);
      pen.rect(box.x - box.w, box.y - box.h, 3 * box.w, 3 * box.h);
      pen.clip("evenodd");
    } else {
      trace(pen, edges, box);
      pen.clip();
    }
    stamp(pen, look.lit, fine, ink.accent);
    pen.restore();
  } else {
    stamp(pen, look.lit, fine, ink.accent);
    stamp(pen, look.cut, coarse, ink.cut);
  }
  trace(pen, edges, box);
  pen.strokeStyle = ink.rule;
  pen.lineWidth = RULE * dpr;
  pen.stroke();
}

export function stage(w: number, h: number): Stage {
  const { px, x, y } = lay(1, 1, w, h, FIT);
  return { x, y, w: px, h: px };
}

/* SKETCH */

const GLANCE = 64;

export function sketch(
  pen: CanvasRenderingContext2D,
  math: Mrly,
  next: Value,
  side: number,
  dpr: number,
  look: Pick<Paints, "paper" | "accent"> = paints(pen.canvas, String(next.tint)),
) {
  let level = 1;
  while (Number(next.number) ** (level + 1) <= GLANCE) level++;
  const now = plan(math, { ...next, level, circle: 0, policy: next.policy === "inside" ? "inside" : "touching" });
  pen.setTransform(1, 0, 0, 1, 0, 0);
  pen.clearRect(0, 0, side, side);
  crop(pen, now, stage(side, side), inks(pen.canvas, { look: () => look }, String(next.tint)), dpr);
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("crop: the scene wants @mrlyprod/mrly/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const stages: Stages = {};
  let now = plan(math, opts, stages);
  let dirty = true;
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    const ink = inks(canvas, view, now.value.tint);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.fillStyle = ink.paper;
    ctx.fillRect(0, 0, view.w, view.h);
    crop(ctx, now, stage(view.w, view.h), ink, view.dpr);
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const set = (next: Raw) => {
    now = plan(math, next, stages);
    touch();
  };
  const drag = (px: number, py: number) => {
    const { x, y, w } = stage(view.w, view.h);
    return { radius: Math.max(0, Math.min(RMAX, Math.round((Math.hypot(px - x - w / 2, py - y - w / 2) / w) * RDEN))) };
  };
  const paint: Paint = (pen, _box, w, h) => {
    const ink = inks(canvas, view, now.value.tint);
    pen.fillStyle = ink.paper;
    pen.fillRect(0, 0, w, h);
    crop(pen, now, { x: 0, y: 0, w, h }, ink, w / 512);
  };
  const svg = () => {
    const { accent } = view.look();
    const side = now.look.side;
    const span = side * UNIT;
    const box = { x: 0, y: 0, w: span, h: span };
    const lit = runs(cell(now.kept.grid))
      .map(
        ([row, col, len]) =>
          `<rect x="${num(col * UNIT)}" y="${num(row * UNIT)}" width="${num(len * UNIT)}" height="${num(UNIT)}"/>`,
      )
      .join("");
    const path = outlined(now.edges, box);
    const clip = now.value.edge
      ? `<defs><clipPath id="crop"><path d="${now.value.anti ? `M0 0H${num(span)}V${num(span)}H0Z` : ""}${path}" clip-rule="evenodd"/></clipPath></defs>`
      : "";
    const group = `<g fill="${accent}" shape-rendering="crispEdges"${now.value.edge ? ' clip-path="url(#crop)"' : ""}>${lit}</g>`;
    return sheet(span, span, `${clip}${group}`);
  };
  return {
    still: true,
    draw,
    set,
    size: touch,
    theme: touch,
    svg,
    shot: (size: number, kind?: string) => shot(1, 1, size, paint, kind),
    facts: () => now.facts(),
    drag,
  };
}

function outlined(edges: Outline, box: Stage) {
  if (edges.circle) {
    const [cx, cy, r] = edges.circle.map((v) => v * box.w);
    return `M${num(cx - r)} ${num(cy)}a${num(r)} ${num(r)} 0 1 0 ${num(2 * r)} 0a${num(r)} ${num(r)} 0 1 0 ${num(-2 * r)} 0Z`;
  }
  return `${edges.points.map(([x, y], i) => `${i ? "L" : "M"}${num(x * box.w)} ${num(y * box.h)}`).join("")}Z`;
}
