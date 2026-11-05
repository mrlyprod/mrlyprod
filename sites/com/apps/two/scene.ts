import type { Cell } from "@mrlyprod/math";
import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { TINTS, veil } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { DESIGN } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Design } from "@mrlyprod/ui/kit/designs/store.ts";
import type { Mathy, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Raw, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import { rects } from "@mrlyprod/ui/kit/svg.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Counts, Layers, Memo, Paint, Pen, Seam } from "../shared/two/engine.ts";
import {
  cap,
  count,
  DIM,
  design,
  joins,
  lay,
  layers,
  memo,
  reroll,
  sheet,
  shot,
  stamp,
  stroke,
} from "../shared/two/engine.ts";
import type { Titled } from "../shared/two/parts.tsx";
import { second } from "../shared/two/scene.ts";

export type Sheet = [string, number, number];

export type Knobs = {
  number: number;
  level: number;
  x: number;
  y: number;
  seams: number;
  invert: number;
  tint: string;
};

export type Read = Design & Knobs;

export type Plan = {
  value: Read;
  tile: Cell;
  block: Cell;
  look: Layers;
  lines: Seam[];
  facts: () => Counts;
};

type Staged = { code: string; tile: Cell; block: Cell; look: Layers; one: Cell; facts: Counts };

export type Stages = { [K in keyof Staged]?: Memo<Staged[K]> };

export type Seen = Pick<View, "w" | "h"> & { look: () => Pick<Paints, "paper" | "accent"> };

export type Given = Opts & { math?: Mrly | null };

export type Two = ReturnType<typeof make>;

export const SHEETS: Sheet[] = [
  ["Single", 1, 1],
  ["Square", 3, 3],
  ["Block", 5, 5],
  ["Wall", 8, 8],
  ["Full", 12, 12],
  ["Pair", 2, 1],
  ["Wide", 9, 3],
  ["Tall", 3, 9],
  ["Strip", 12, 1],
  ["Column", 1, 12],
];

const never = () => false;

export const SPEC: Titled<Mathy>[] = [
  {
    key: "number",
    label: "Side",
    kind: "segment",
    def: 3,
    options: [
      [3, "3"],
      [5, "5"],
      [7, "7"],
    ],
    group: "Design",
    when: never,
  },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 2,
    min: 1,
    max: (value, units) => units && cap(units.math!, Number(value.number), Number(value.x) * Number(value.y)),
    step: 1,
    keep: true,
    group: "Design",
    when: never,
  },
  { key: "x", label: "Across", kind: "slider", def: 5, min: 1, max: 12, step: 1, group: "Sheet", when: never },
  { key: "y", label: "Down", kind: "slider", def: 5, min: 1, max: 12, step: 1, group: "Sheet", when: never },
  { key: "seams", label: "Seams", kind: "toggle", def: 0, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const name = (value: Value) => `two-${String(value.code ?? "").trim() || value.seed}-${value.x}x${value.y}`;

export const units = wasm("math");

export const PAGE = {
  defaults: { base: 3, code: "495" },
  design: { dim: DIM, base: "base", code: "code" },
  spec: SPEC,
  units,
  name,
  keys: [
    { key: "Enter", label: "Seams", act: "seams", button: true, menu: "View", when: (value) => !value.seams },
    {
      key: "Enter",
      label: "Hide seams",
      act: "seams",
      button: true,
      menu: "View",
      when: (value) => Boolean(value.seams),
    },
    { key: ["ArrowUp", "ArrowDown"], label: "Level", act: "level", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Across", act: "across" },
  ],
  acts: {
    seams: (value) => ({ seams: value.seams ? 0 : 1 }),
    level: (value, key, units) => ({
      level: Math.max(
        1,
        Math.min(
          cap(units!.math, Number(value.number), Number(value.x) * Number(value.y)),
          Number(value.level) + (key === "ArrowDown" ? -1 : 1),
        ),
      ),
    }),
    across: (value, key) => ({ x: Math.max(1, Math.min(12, Number(value.x) + (key === "ArrowLeft" ? -1 : 1))) }),
  },
  reroll,
} satisfies Page<Loaded<"math">, Two>;

const FIT = 0.9;

const UNIT = 10;

const FAINT = 0.16;

const SEAM = 0.3;

export function plan(math: Mrly, next: Raw, stages: Stages = {}): Plan {
  const at = <K extends keyof Staged>(name: K): Memo<Staged[K]> => {
    const held: { [P in K]?: Memo<Staged[P]> } = stages;
    held[name] ??= memo();
    return held[name];
  };
  const value = { ...(tidy(DESIGN, next) as Design), ...(tidy(SPEC, next, { math }) as Knobs) };
  const code = at("code")(`${value.base}:${value.code}:${next.seed}`, () => design(math, value.base, value.code));
  const kept = [code, value.base, value.number, value.level].join();
  const tile = at("tile")(kept, () => math.two.create(code, value.number, value.level, 0, value.base));
  const whole = `${kept}|${value.x}|${value.y}`;
  const block = at("block")(whole, () => sheet(math, tile, value.x, value.y));
  const lit = `${whole}|${value.invert}`;
  const look = at("look")(lit, () => layers(math, block, value.invert));
  const one = at("one")(`${kept}|${value.invert}`, () => (value.invert ? math.cell.models.invert(tile) : tile));
  const lines = value.seams ? joins(block.shape, tile.shape[0]) : [];
  const read = { ...value, code };
  const facts = () => at("facts")(lit, () => count(math, read, one, look.drawn));
  return { value: read, tile, block, look, lines, facts };
}

const GLANCE = 0.92;

export function sketch(pen: Pen, math: Mrly, next: Raw, x: number, y: number, side: number, accent: string) {
  const value = { ...(tidy(DESIGN, next) as Design), ...(tidy(SPEC, next, { math }) as Knobs) };
  const tile = math.two.create(design(math, value.base, value.code), value.number, 1, 0, value.base);
  const block = sheet(math, tile, x, y);
  const look = layers(math, block, value.invert);
  const [rows, cols] = block.shape;
  const box = lay(cols, rows, side, side, GLANCE);
  pen.setTransform(1, 0, 0, 1, 0, 0);
  pen.clearRect(0, 0, side, side);
  stamp(pen, look.faint, box, veil(accent, FAINT));
  stamp(pen, look.lit, box, accent);
}

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("two: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const stages: Stages = {};
  let now = plan(math, opts, stages);
  let dirty = true;
  const paint: Paint = (pen, box, w, h) => {
    const { paper, accent } = view.look();
    pen.fillStyle = paper;
    pen.fillRect(0, 0, w, h);
    stamp(pen, now.look.faint, box, veil(accent, FAINT));
    stamp(pen, now.look.lit, box, accent);
    if (now.lines.length)
      stroke(pen, now.lines, box, second(canvas, now.value.tint, accent), Math.max(1, Math.round(box.px * SEAM)));
  };
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    const [rows, cols] = now.block.shape;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    paint(ctx, lay(cols, rows, view.w, view.h, FIT), view.w, view.h);
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const set = (next: Raw) => {
    now = plan(math, next, stages);
    touch();
  };
  const svg = () => rects(now.look.drawn, { fill: view.look().accent, unit: UNIT });
  const picture = (size: number, kind = "png") => {
    const [rows, cols] = now.block.shape;
    return shot(cols, rows, size, paint, kind);
  };
  return { still: true, draw, set, size: touch, theme: touch, svg, shot: picture, facts: () => now.facts() };
}
