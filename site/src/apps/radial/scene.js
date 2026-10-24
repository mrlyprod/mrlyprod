import { defaults, tidy } from "../../lib/app/spec.js";
import {
  BLENDS,
  COPIES,
  degrees,
  edge,
  key,
  layout,
  OUT,
  paint,
  petals,
  rosette,
  stops,
  study,
} from "../../lib/spin/engine.js";
import { LOOK, PAD, place, SOURCE, sheet, tag } from "../../lib/spin/scene.js";
import { wasm } from "../../lib/units/index.js";

const HIDE = () => false;

export const STACK = [
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

export const SPEC = [...STACK, ...LOOK];

export const STACKS = [
  ["Pair", { copies: 2, full: 1 }],
  ["Three", { copies: 3, full: 1 }],
  ["Six", { copies: 6, full: 1 }],
  ["Twelve", { copies: 12, full: 1 }],
  ["Afterglow", { copies: 2, full: 0, step: 45 }],
  ["Pentagram", { copies: 5, full: 0, step: 144 }],
  ["Golden angle", { copies: 34, full: 0, step: 137.5 }],
];

const STEP = { ArrowUp: 0.5, ArrowDown: -0.5 };

const ABOUT = [
  "Copy a design, turn the copy about the centre, lay it on the first, and repeat. The blend merges the copies: their mean, sum, union, meet, parity, or what the first keeps that no other has.",
  "A full turn shared over q copies is what a screen shows when a spinning design advances p/q of a turn a frame. The stack keeps only the circular harmonics whose order is a multiple of q, so a design of rotation order g shows lcm(q, g) petals.",
  "A step that is no full turn leaves a partial orbit. The readouts give the share of order zero, the infinite spin, and the leading orders of the source; its rotation order is the gcd of the orders it carries.",
  "Two copies at 45 degrees is the picture an afterglow paints at 900 rpm on a 60 Hz screen. Which harmonics a turned stack keeps is in [the spin topic](/research/spin/) of Research.",
];

export const name = (value, units) => `radial-${tag(value, units?.math)}-${value.copies}`;

export const units = wasm("math");

export const PAGE = {
  defaults: defaults(SOURCE),
  spec: SPEC,
  units,
  design: { dim: "dim", base: "base", code: "code", when: (value) => value.source !== "moire" },
  about: ABOUT,
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
  actions: {
    whole: (scene) => scene.whole?.(),
    copies: (scene, e) => scene.copies?.(e.key === "ArrowLeft" ? -1 : 1),
    step: (scene, e) => scene.step?.(STEP[e.key]),
  },
};

const LOOKS = ["ramp", "levels", "invert", "tint", "copies", "full", "step", "blend", "samples"];

export const read = (value) => ({ ...tidy(SOURCE, value), ...tidy(SPEC, value) });

/* SCENE */

export const facts = (math, plan, value) => ({
  kind: plan.kind,
  code: plan.code,
  name: plan.name,
  side: plan.side,
  copies: value.copies,
  step: degrees(value),
  order: plan.order,
  petals: petals(math, plan, value),
  share: plan.share[0],
  leading: plan.leading,
});

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  const ctx = canvas.getContext("2d");
  if (!math) return { draw: () => ctx.clearRect(0, 0, view.w, view.h) };
  let given = opts;
  let value = read(given);
  let plan = null;
  let layer = null;
  let disc = null;
  let line = edge(view.look());
  let fresh = true;
  const marked = () => key(value);
  const learn = () => {
    plan = study(math, value);
  };
  const tell = () => given.onFacts?.(facts(math, plan, value));
  const coat = () => {
    const look = view.look();
    line = edge(look);
    const rose = rosette(math, plan, value);
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
    place(ctx, layer, disc, line, view.dpr);
  };
  const set = (next) => {
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
