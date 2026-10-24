import { board } from "../../lib/app/run.js";
import { tidy } from "../../lib/app/spec.js";
import { rects } from "../../lib/app/svg.js";
import { paints, TINTS } from "../../lib/color.js";
import {
  canon,
  cap,
  census,
  DESIGN,
  dimension,
  distinct,
  facts,
  grow,
  SIDE,
  sequence,
  total,
} from "../../lib/designs/engine.js";
import { mesh } from "../../lib/designs/mesh.js";
import { flat, hue, iso } from "../../lib/designs/thumb.js";
import { wasm } from "../../lib/units/index.js";
import { space } from "./stage.js";

const cube = (value) => Number(value.dim) === 3;

const HIDE = () => false;

export const SPEC = [
  ...DESIGN.map((row) => ({ ...row, when: HIDE })),
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value, units) => units && cap(units.math, value.dim),
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

export const units = wasm("math");

export const named = (value) => {
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
  about: [
    "A code is a bitmask over the corners of a square or a cube. Each level substitutes that rule into itself: code 7 in the plane grows the carpet, code 23 in the cube the sponge.",
    "Turning and mirroring fold the codes into classes, and one design per class is all there is. The gallery shows one per class, and Info counts them for each dimension; the counts are Burnside averages over the hyperoctahedral group, and walking the orbits agrees with them.",
    "Tap a design in the gallery, or tap the picture for the next one. Grow and the up and down arrows change the level, Reroll draws a class from the seed, and a cube turns when you drag it. The colour follows the degree of the normal form.",
    "The two moves are written up in the [core note](/research/core/).",
  ],
  keys: [
    { key: "Enter", label: "Grow", act: "grow", button: true, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Design", act: "design" },
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
  ],
  actions: {
    grow: (scene) => scene.grow?.(),
    design: (scene, e) => scene.step?.(e.key === "ArrowLeft" ? -1 : 1),
    up: (scene) => scene.deeper?.(1),
    down: (scene) => scene.deeper?.(-1),
  },
};

const FIT = 0.86;
const UNIT = 10;

/* PLAN */

const shape = (value, code) => `${value.dim}:${value.base}:${code}:${value.level}`;

export function plan(math, value, was) {
  const { dim, base, level } = value;
  const code = canon(math, dim, base, value.code);
  const key = shape(value, code);
  if (was?.key === key) return { ...was, value };
  const cell = grow(math, dim, base, code, level);
  const { upto, terms } = sequence(math, base);
  const info = {
    dim,
    base,
    code,
    level,
    side: cell.shape[0],
    name: math.name.of(code, dim, base) ?? "",
    ...facts(math, dim, base, code),
    ...census(math, dim, cell),
    grid: Number(math.counts.grid(SIDE, dim, level)),
    dimension: dimension(math, dim, base, code),
    ratio: math.counts.ratio(code, SIDE, dim, level, base),
    codes: total(math, dim, base),
    classes: distinct(math, dim, base),
    upto,
    terms,
  };
  return { key, value, cell, info };
}

/* STAGES */

function plane(canvas, view, cell, colour) {
  const ctx = canvas.getContext("2d");
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
  const swap = (next) => {
    cell = next;
    dirty = true;
  };
  const svg = () => rects(cell, { fill: colour(), ground: view.look().paper, unit: UNIT });
  return { draw, size: touch, theme: touch, swap, svg };
}

function solid(canvas, view, three, cell, math, colour, spin) {
  const blank = { draw: () => {}, theme: () => {}, swap: () => {} };
  if (!three) return blank;
  try {
    return space(three, canvas, view, mesh(math.three.quads(cell)), colour, spin);
  } catch {
    return blank;
  }
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, three, tell } = opts;
  let now = plan(math, tidy(SPEC, opts, { math }));
  const dim = now.info.dim;
  let ink = "";
  const tone = () => {
    ink = paints(canvas, hue(now.value.tint, now.info.degree)).accent;
  };
  tone();
  const colour = () => ink;
  const spin = () => dim === 3 && now.value.spin === 1;
  const stage =
    dim === 3 ? solid(canvas, view, three, now.cell, math, colour, spin) : plane(canvas, view, now.cell, colour);
  const draw = () => {
    if (spin() && view.still) view.wake?.();
    stage.draw();
  };
  const theme = () => {
    tone();
    stage.theme();
  };
  const set = (next) => {
    const value = tidy(SPEC, next, { math });
    if (value.dim !== dim) return false;
    const was = now;
    now = plan(math, value, was);
    if (now.cell !== was.cell) stage.swap(dim === 3 ? mesh(math.three.quads(now.cell)) : now.cell);
    theme();
    tell?.(now.info);
  };
  const svg = () =>
    dim === 3 ? stage.pose && iso(math, now.cell, ...stage.pose(), view.look().paper, colour()) : stage.svg();
  const obj = () => math.three.to_obj(now.cell);
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
