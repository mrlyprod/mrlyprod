import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, Scene, View } from "@mrlyprod/ui/kit/run.ts";
import type { Raw, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { defaults, tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Disc, Plan, Src, Stack } from "../shared/spin/engine.ts";
import {
  BLENDS,
  COPIES,
  degrees,
  drawn,
  edge,
  key,
  layout,
  OUT,
  paint,
  petals,
  rosette,
  stops,
  study,
} from "../shared/spin/engine.ts";
import type { Look } from "../shared/spin/scene.ts";
import { LOOK, PAD, place, SOURCE, sheet, tag } from "../shared/spin/scene.ts";

export type Radial = Src & Look & Stack;

export type Orbit = { copies: number; full: number; step?: number };

export type Facts = Pick<Plan, "kind" | "code" | "name" | "side" | "order" | "leading"> & {
  copies: number;
  step: number;
  petals: number | string;
  share: number;
};

export type Given = Opts & { mrly?: Mrly | null; onFacts?: (facts: Facts) => void };

export type Rolled = Partial<Src> & Pick<Stack, "copies" | "blend"> & { full: number };

const HIDE = () => false;

export const STACK: Spec = [
  { key: "copies", label: "Copies", kind: "slider", def: 6, min: 1, max: COPIES, step: 1 },
  { key: "full", label: "Full turn", kind: "toggle", def: 1 },
  {
    key: "step",
    label: "Step",
    kind: "slider",
    def: 60,
    min: 0,
    max: 360,
    step: 0.5,
    unit: "deg",
    when: (value) => Number(value.full) !== 1,
  },
  { key: "blend", label: "Blend", kind: "pick", def: "mean", options: BLENDS, when: HIDE },
  {
    key: "samples",
    label: "Samples",
    kind: "segment",
    def: 2,
    options: [
      [1, "1"],
      [2, "2"],
      [3, "3"],
    ],
  },
];

export const SPEC: Spec = [...STACK, ...LOOK];

export const STACKS: [string, Orbit][] = [
  ["Pair", { copies: 2, full: 1 }],
  ["Three", { copies: 3, full: 1 }],
  ["Six", { copies: 6, full: 1 }],
  ["Twelve", { copies: 12, full: 1 }],
  ["Afterglow", { copies: 2, full: 0, step: 45 }],
  ["Pentagram", { copies: 5, full: 0, step: 144 }],
  ["Golden angle", { copies: 34, full: 0, step: 137.5 }],
];

const STEP: Record<string, number> = { ArrowUp: 0.5, ArrowDown: -0.5 };

const BLEND = BLENDS.map(([one]) => one).filter((one) => one !== "meet");

export const wrap = (copies: number, by: number) => ((copies - 1 + by + COPIES) % COPIES) + 1;

export function rolled(mrly: Mrly, value: Raw, rand: Rng): Rolled {
  return {
    ...drawn(mrly, read(value), rand),
    copies: 1 + Math.floor(rand() * COPIES),
    blend: BLEND[Math.floor(rand() * BLEND.length)],
    full: 1,
  };
}

export const name = (value: Value, units?: Mrly) => `radial-${tag(value, units)}-${value.copies}`;

export const units = wasm(...CRATES);

export const PAGE = {
  defaults: defaults(SOURCE),
  spec: SPEC,
  units,
  design: { dim: "dim", base: "base", code: "code", when: (value) => value.source !== "moire" },
  name,
  keys: [
    {
      key: "Enter",
      label: "Full turn",
      act: "whole",
      button: true,
      menu: "Edit",
      when: (value) => Number(value.full) !== 1,
    },
    { key: ["ArrowLeft", "ArrowRight"], label: "Copies", act: "copies", menu: "Edit" },
    { key: Object.keys(STEP), label: "Step", act: "step" },
  ],
  acts: {
    whole: () => ({ full: 1 }),
    copies: (value, key) => ({
      copies: Math.min(COPIES, Math.max(1, Number(value.copies) + (key === "ArrowLeft" ? -1 : 1))),
    }),
    step: (value, key) => ({
      full: 0,
      step: Math.min(
        360,
        Math.max(0, (Number(value.full) === 1 ? 360 / Number(value.copies) : Number(value.step)) + STEP[key]),
      ),
    }),
  },
  reroll: (value, seed, units) => rolled(units!, value, rng(seed)),
} satisfies Page<Loaded<Crate>, Scene>;

const LOOKS: (keyof Radial)[] = ["ramp", "levels", "invert", "tint", "copies", "full", "step", "blend", "samples"];

export const read = (value: Raw) => ({ ...tidy(SOURCE, value), ...tidy(SPEC, value) }) as Radial;

export const facts = (mrly: Mrly, plan: Plan, value: Radial): Facts => ({
  kind: plan.kind,
  code: plan.code,
  name: plan.name,
  side: plan.side,
  copies: value.copies,
  step: degrees(value),
  order: plan.order,
  petals: petals(mrly, plan, value),
  share: plan.share[0],
  leading: plan.leading,
});

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Scene {
  const mrly = opts.mrly;
  const ctx = canvas.getContext("2d")!;
  if (!mrly) return { draw: () => ctx.clearRect(0, 0, view.w, view.h) };
  let given = opts;
  let value = read(given);
  let plan: Plan | null = null;
  let layer: OffscreenCanvas | null = null;
  let disc: Disc | null = null;
  let line = edge(view.look());
  let fresh = true;
  const marked = () => key(value);
  const learn = () => {
    plan = study(mrly, value);
  };
  const tell = () => given.onFacts?.(facts(mrly, plan!, value));
  const coat = () => {
    const look = view.look();
    line = edge(look);
    const rose = rosette(mrly, plan!, value);
    layer = sheet(paint(rose.values, rose.top, stops(value.ramp, look), value.levels, value.invert), OUT);
  };
  const draw = () => {
    if (fresh) {
      coat();
      fresh = false;
    }
    disc ??= layout(view.w, view.h, PAD * view.dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    place(ctx, layer!, disc, line, view.dpr);
  };
  const set = (next: Opts) => {
    const was = value;
    const at = marked();
    given = { ...given, ...next };
    value = read(given);
    if (marked() !== at) {
      learn();
      fresh = true;
    }
    if (LOOKS.some((name) => was[name] !== value[name])) fresh = true;
    tell();
  };
  const theme = () => {
    fresh = true;
  };
  const size = () => {
    disc = null;
  };
  learn();
  tell();
  return { draw, size, theme, set, still: true };
}
