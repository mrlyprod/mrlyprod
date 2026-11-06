import type { Cell } from "@mrlyprod/math";
import { paints, TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Base, Census, Crate, Facts, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import {
  CRATES,
  canon,
  cap,
  census,
  DESIGN,
  dimension,
  distinct,
  drawn,
  facts,
  gallery,
  grow,
  next,
  SIDE,
  sequence,
  total,
} from "@mrlyprod/ui/kit/designs/engine.ts";
import { mesh } from "@mrlyprod/ui/kit/designs/mesh.ts";
import { flat, hue, iso } from "@mrlyprod/ui/kit/designs/thumb.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, Scene, View } from "@mrlyprod/ui/kit/run.ts";
import { board } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Raw, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import { rects } from "@mrlyprod/ui/kit/svg.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Three } from "../shared/three/three.ts";
import type { Seen } from "./stage.ts";
import { space } from "./stage.ts";

export type Knobs = { dim: number; base: Base; code: string; level: number; tint: string; spin: number };

export type Info = Facts &
  Census & {
    dim: number;
    base: number;
    code: string;
    level: number;
    side: number;
    name: string;
    grid: number;
    dimension: number;
    ratio: number;
    codes: number;
    classes: number;
    upto: number;
    terms: string[];
  };

export type Plan = { key: string; value: Knobs; cell: Cell; info: Info };

export type Given = Opts & { mrly: Mrly; three?: Three | null; tell?: (info: Info) => void };

export type Stage = {
  draw: () => void;
  theme: () => void;
  size?: () => void;
  svg?: () => string;
  turn?: (dx: number, dy: number) => void;
  pose?: () => [number, number];
  stop?: () => void;
};

type Room = ReturnType<typeof space>;

type Staged = { dim: 2; stage: ReturnType<typeof plane> } | { dim: 3; stage: ReturnType<typeof solid> };

export type Designs = ReturnType<typeof make>;

const cube = (value: Value) => Number(value.dim) === 3;

const HIDE = () => false;

export const SPEC: Spec<Mrly> = [
  ...DESIGN.map((row) => ({ ...row, when: HIDE })),
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value, units) => units && cap(units, value.dim as number),
    step: 1,
    group: "Design",
  },
  {
    key: "tint",
    label: "Colour",
    kind: "pick",
    def: "degree",
    options: [["degree", "By degree"], ["site", "Site"], ...TINTS.slice(1)],
    group: "Look",
  },
  { key: "spin", label: "Spin", kind: "toggle", def: 0, group: "Look", when: cube },
];

export const units = wasm(...CRATES);

export const named = (value: Raw) => {
  const code = String(value.code ?? "").replace(/\W+/g, "");
  return [
    "designs",
    `${value.dim}d`,
    ...(Number(value.base) === 3 ? ["base3"] : []),
    ...(code ? [code] : []),
    `level${value.level}`,
  ].join("-");
};

export const PAGE = {
  spec: SPEC,
  units,
  name: named,
  gestures: true,
  design: { dim: "dim", base: "base", code: "code" },
  keys: [
    { key: "Enter", label: "Grow", act: "grow", button: true, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Design", act: "design" },
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
  ],
  acts: {
    grow: (value, _key, units) => ({
      level: Number(value.level) >= cap(units!, Number(value.dim)) ? 1 : Number(value.level) + 1,
    }),
    design: (value, key, units) => step(units!, value, key === "ArrowLeft" ? -1 : 1),
    up: (value) => ({ level: Number(value.level) + 1 }),
    down: (value) => ({ level: Number(value.level) - 1 }),
  },
  reroll: (value, seed, units) => {
    const [mrly, dim, base] = [units!, Number(value.dim), Number(value.base)];
    return {
      code: drawn(mrly, dim, base, gallery(mrly, dim, base, "classes"), canon(mrly, dim, base, value.code), seed),
    };
  },
} satisfies Page<Loaded<Crate>, Scene>;

export const steps = (mrly: Mrly, dim: number, base: number) =>
  gallery(mrly, dim, base, "classes") ?? gallery(mrly, dim, base, "named");

export function step(mrly: Mrly, value: Value, by: number): Patch {
  const [dim, base] = [Number(value.dim), Number(value.base)];
  const code = canon(mrly, dim, base, value.code);
  const list = steps(mrly, dim, base) ?? [];
  return { code: next(list, list.includes(code) ? code : facts(mrly, dim, base, code).rep, by) };
}

const FIT = 0.86;
const UNIT = 10;

const shape = (value: Knobs, code: string) => `${value.dim}:${value.base}:${code}:${value.level}`;

export function plan(mrly: Mrly, value: Knobs, was?: Plan): Plan {
  const { dim, base, level } = value;
  const code = canon(mrly, dim, base, value.code);
  const key = shape(value, code);
  if (was?.key === key) return { ...was, value };
  const cell = grow(mrly, dim, base, code, level);
  const { upto, terms } = sequence(mrly, base);
  const info = {
    dim,
    base,
    code,
    level,
    side: cell.shape[0],
    name: mrly.math.name.of(code, dim, base) ?? "",
    ...facts(mrly, dim, base, code),
    ...census(mrly, dim, cell),
    grid: Number(mrly.sequences.counts.grid(SIDE, dim, level)),
    dimension: dimension(mrly, dim, base, code),
    ratio: mrly.sequences.counts.ratio(code, SIDE, dim, level, base),
    codes: total(mrly, dim, base),
    classes: distinct(mrly, dim, base),
    upto,
    terms,
  };
  return { key, value, cell, info };
}

function plane(canvas: HTMLCanvasElement, view: View, cell: Cell, colour: () => string) {
  const ctx = canvas.getContext("2d")!;
  let dirty = true;
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    const n = cell.shape[0];
    const { x, y, w } = board(view, n, n, FIT);
    flat(ctx, cell, x, y, w, colour());
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const swap = (next: Cell) => {
    cell = next;
    dirty = true;
  };
  const svg = () => rects(cell, { fill: colour(), ground: view.look().paper, unit: UNIT });
  return { draw, size: touch, theme: touch, swap, svg };
}

function solid(
  canvas: HTMLCanvasElement,
  view: Seen,
  three: Three | null | undefined,
  cell: Cell,
  mrly: Mrly,
  colour: () => string,
  spin: () => boolean,
) {
  const blank: Pick<Room, "draw" | "theme" | "swap"> & Partial<Room> = {
    draw: () => {},
    theme: () => {},
    swap: () => {},
  };
  if (!three) return blank;
  try {
    return space(three, canvas, view, mesh(mrly.three.quads(cell)), colour, spin);
  } catch {
    return blank;
  }
}

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {} as Given) {
  const { mrly, three, tell } = opts;
  let now = plan(mrly, tidy(SPEC, opts, mrly) as Knobs);
  const dim = now.info.dim;
  let ink = "";
  const tone = () => {
    ink = paints(canvas, hue(now.value.tint, now.info.degree)).accent;
  };
  tone();
  const colour = () => ink;
  const spin = () => dim === 3 && now.value.spin === 1;
  const staged: Staged =
    dim === 3
      ? { dim: 3, stage: solid(canvas, view, three, now.cell, mrly, colour, spin) }
      : { dim: 2, stage: plane(canvas, view, now.cell, colour) };
  const stage: Stage = staged.stage;
  const swap = () => {
    if (staged.dim === 3) staged.stage.swap(mesh(mrly.three.quads(now.cell)));
    else staged.stage.swap(now.cell);
  };
  const draw = () => {
    if (spin() && view.still) view.wake?.();
    stage.draw();
  };
  const theme = () => {
    tone();
    stage.theme();
  };
  const set = (next: Opts) => {
    const value = tidy(SPEC, next, mrly) as Knobs;
    if (value.dim !== dim) return false;
    const was = now;
    now = plan(mrly, value, was);
    if (now.cell !== was.cell) swap();
    theme();
    tell?.(now.info);
  };
  const svg = () =>
    staged.dim === 3
      ? staged.stage.pose && iso(mrly, now.cell, ...staged.stage.pose(), view.look().paper, colour())
      : staged.stage.svg();
  const obj = () => mrly.three.to_obj(now.cell);
  tell?.(now.info);
  return {
    draw,
    size: stage.size,
    turn: stage.turn,
    stop: stage.stop,
    pose: stage.pose,
    theme,
    set,
    svg,
    still: true,
    ...(dim === 3 ? { obj } : {}),
  };
}
