import type { Color, Frame, Pen } from "@mrlyprod/draw";
import { frame as box, hex, canvas as pen, svg as sheet } from "@mrlyprod/draw";
import type { Cell6d } from "@mrlyprod/math";
import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { rgb, TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Code, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { DESIGN, roll } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Had, Readout } from "@mrlyprod/ui/kit/hooks.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Raw, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import { DIM, design, memo, named, tag as title } from "../shared/three/engine.ts";

export type Codes = { fill: number; void: number };

export type Paint = Codes & { accent: Color };

export type Study = ReturnType<typeof study>;

export type Told = { rows: Readout[] };

export type Seen = Pick<View, "w" | "h" | "dpr"> & { look: () => Pick<Paints, "accent"> };

export type Given = Opts & { math?: Mrly | null; tell?: (info: Told) => void };

const side = (value: Value, by: number): Patch => ({
  k: Math.min(TOP, Math.max(1, Number(value.tile) ? Number(value.k) : Number(value.k) + by)),
  tile: 0,
});

export type Slices = ReturnType<typeof make>;

export const TOP = 16;
export const TILES: [number, string][] = [
  [0, "Off"],
  [3, "Base 3"],
  [5, "Base 5"],
];
const REACH = 100;
const NET = "232";
const CARPET = "23";
const PAD = 0.04;
const FAINT = 18;

export const codesOf = (math: Mrly): Codes => ({ fill: math.six.FILL(), void: math.six.VOID() });

export const lid = (math: Mrly, tile: number) => (tile ? math.bang.level_cap(tile, 1, REACH) : 1);

const tiled = (value: Value) => Number(value.tile) !== 0;

export const SPEC: Spec<Had<"math">> = [
  {
    key: "k",
    label: "Side k",
    kind: "slider",
    def: 2,
    min: 1,
    max: TOP,
    step: 1,
    group: "Section",
    when: (value) => !tiled(value),
  },
  { key: "tile", label: "Fractal tile", kind: "segment", def: 0, options: TILES, group: "Section" },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 1,
    min: 1,
    max: (value, units) => units && lid(units.math, Number(value.tile)),
    step: 1,
    group: "Section",
    when: tiled,
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const units = wasm("math");

export const name = (value: Value) =>
  `slices-${String(value.code ?? "").trim() || CARPET}-${tiled(value) ? `t${value.tile}-l${value.level}` : `k${value.k}`}`;

export const PAGE = {
  defaults: { base: 2, code: "23" },
  design: { dim: 3, base: "base", code: "code" },
  spec: SPEC,
  units,
  gestures: true,
  name,
  keys: [
    { key: "ArrowLeft", label: "Smaller side", act: "smaller", menu: "View" },
    { key: "ArrowRight", label: "Larger side", act: "larger", menu: "View" },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", when: tiled },
  ],
  acts: {
    smaller: (value) => side(value, -1),
    larger: (value) => side(value, 1),
    level: (value, key) => ({ level: Number(value.level) + (key === "ArrowDown" ? -1 : 1) }),
  },
  reroll: (value, seed, units) => ({ code: roll(units!.math, DIM, Number(tidy(DESIGN, value).base), rng(seed)) }),
} satisfies Page<Loaded<"math">, Slices>;

const cuts = memo(6);
const splits = memo(8);
const glances = memo(20);

function cut(math: Mrly, code: Code, base: number, number: number, level: number) {
  return cuts(`${code} ${base} ${number} ${level}`, () => {
    const cell = math.three.create(code, number, level, base);
    const mesh = math.six.cut(cell);
    return {
      mesh,
      side: cell.shape[0],
      tally: math.six.census(mesh, false),
      pieces: math.six.components(mesh),
      holes: math.six.holes(mesh),
      giant: math.six.giant(mesh),
    };
  });
}

export function partition(math: Mrly, number: number) {
  return splits(number, () => {
    const carpet = cut(math, CARPET, 2, number, 1).tally;
    const net = cut(math, NET, 2, number, 1).tally;
    return {
      carpet: carpet.fills,
      net: net.fills,
      together: carpet.fills + net.fills,
      hexagon: carpet.triangles,
      exact: carpet.fills + net.fills === carpet.triangles,
    };
  });
}

export function study(math: Mrly, value: Raw) {
  const { base, code } = design(math, { ...value, base: Number(value.base) });
  const tile = Number(value.tile);
  const k = Math.min(TOP, Math.max(1, Math.round(Number(value.k))));
  const number = tile || 2 * k - 1;
  const level = tile ? Math.min(Number(value.level), lid(math, tile)) : 1;
  const one = cut(math, code, base, number, level);
  const { tally } = one;
  const facts: Readout[] = [
    ["name", title(named(math, code, base), code, base)],
    ["side", one.side],
    ["triangles", tally.triangles],
    ["closed form", String(math.counts.six.solid_slice_triangles(one.side))],
    ["filled", tally.fills],
    ["empty", tally.voids],
    ["pieces", one.pieces],
    ["holes", one.holes],
    ["largest piece", one.giant],
    ["boundary", tally.boundary_edges],
    ["edges", tally.edges],
    ["interior", tally.interior_edges],
    ["vertices", tally.vertices],
    ["euler", String(tally.euler)],
  ];
  if (level === 1) {
    const split = partition(math, number);
    facts.push(
      ["carpet", split.carpet],
      ["net", split.net],
      ["together", split.together],
      ["hexagon", split.hexagon],
      ["partition", split.exact ? "exact" : "broken"],
    );
  }
  return { mesh: one.mesh, side: one.side, rows: facts };
}

export function glance(math: Mrly, value: Raw, k: number) {
  const { base, code } = design(math, { ...value, base: Number(value.base) });
  return glances(`${code} ${base} ${k}`, () => math.six.cut(math.three.create(code, 2 * k - 1, 1, base)));
}

export const layout = (w: number, h: number) => box(0, 0, w, h).inset(Math.min(w, h) * PAD);

export const scrub = (x: number, w: number) => Math.min(TOP, Math.max(1, Math.floor((x / Math.max(1, w)) * TOP) + 1));

export function picture(page: Pen, frame: Frame, plan: { mesh: Cell6d }, paint: Paint) {
  const fill = paint.accent;
  const [r, g, b] = fill;
  const faint: Color = [r, g, b, FAINT];
  const codes = { [paint.fill]: fill, [paint.void]: faint };
  hex.draw(page, frame, plan.mesh, 0, (type) => codes[type] ?? null);
}

export function thumb(ctx: CanvasRenderingContext2D, side: number, mesh: Cell6d, codes: Codes, accent: string) {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  picture(pen(ctx, side, side), layout(side, side), { mesh }, { ...codes, accent: [...rgb(accent), 255] });
}

const settle = (opts: Given & { math: Mrly }) => ({
  ...tidy(SPEC, opts, { math: opts.math }),
  seed: opts.seed,
  base: opts.base,
  code: opts.code,
});

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { math, tell } = opts;
  if (!math) throw new Error("slices: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const codes = codesOf(math);
  let value = settle({ ...opts, math });
  let plan = study(math, value);
  let dirty = true;
  const paint = (): Paint => ({ ...codes, accent: [...rgb(view.look().accent), 255] });
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    picture(pen(ctx, view.w, view.h), layout(view.w, view.h), plan, paint());
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  tell?.({ rows: plan.rows });
  return {
    still: true,
    draw,
    size: touch,
    theme: touch,
    set: (next: Raw) => {
      value = settle({ ...next, math });
      plan = study(math, value);
      tell?.({ rows: plan.rows });
      touch();
      return true;
    },
    seek: (x: number) => scrub(x, view.w),
    svg: () => {
      const w = view.w / view.dpr;
      const h = view.h / view.dpr;
      const page = sheet(w, h);
      picture(page, layout(w, h), plan, paint());
      return page.text();
    },
  };
}
