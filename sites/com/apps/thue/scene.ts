import { rgb } from "@mrlyprod/ui/kit/color.ts";
import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES, DESIGN, next } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Design } from "@mrlyprod/ui/kit/designs/store.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Raw, Row, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { maxOf, minOf, tidy } from "@mrlyprod/ui/kit/spec.ts";
import { palette } from "@mrlyprod/ui/kit/theme/palette.gen.js";
import { dark, light } from "@mrlyprod/ui/kit/theme/theme.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Bar, Grid, Inks, Kind, Knobs, Plan, Thue } from "./engine.ts";
import {
  based,
  CELLS,
  COLOURS,
  cap,
  coded,
  DESIGNED,
  DIM,
  FOLDS,
  fits,
  GRIDS,
  KINDS,
  LEVELS,
  LIFTS,
  lay,
  PAD,
  PAINTS,
  PICTURES,
  picture,
  pictureOf,
  place,
  READS,
  ROUNDS,
  roll,
  sideOf,
  sides,
  study,
  tower,
  VIEWS,
} from "./engine.ts";

export type Seen = Pick<View, "w" | "h" | "dpr">;

export type Given = Opts & { mrly?: Mrly | null };

export type Hands = {
  toggle: () => void;
  playing: () => boolean;
};

export type Stage = ReturnType<typeof make>;

export type Handled = Stage & Hands;

const never = () => false;

const word = (value: Raw) => value.view === "word";

const reads = (kind: string) => (value: Value) => pictureOf(value) === kind;

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "plane", options: VIEWS, group: "View" },
  { key: "read", label: "Read", kind: "segment", def: "power", options: READS, group: "View", when: never },
  {
    key: "round",
    label: "Rounds",
    kind: "slider",
    def: 6,
    min: 1,
    max: ROUNDS,
    step: 1,
    show: (v) => `${v} · ${2 ** v} letters`,
    group: "View",
    when: word,
  },
  { key: "colour", label: "Colour", kind: "segment", def: "letters", options: COLOURS, group: "View", when: never },
  { key: "lift", label: "Lift", kind: "pick", def: "Parity", options: LIFTS, group: "View", when: never },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 6,
    min: 1,
    max: LEVELS,
    step: 1,
    show: (v) => `${v} · side ${2 ** v}`,
    group: "View",
    when: reads("plane"),
  },
  { key: "fold", label: "Fold", kind: "pick", def: "sign", options: FOLDS, group: "View", when: reads("filter") },
  {
    key: "number",
    label: "Tile side",
    kind: "pick",
    def: 0,
    options: sides,
    group: "View",
    when: (value) => DESIGNED.includes(pictureOf(value)),
  },
  {
    key: "power",
    label: "Level",
    kind: "slider",
    def: 5,
    min: 1,
    max: (value) => cap(sideOf(value)),
    step: 1,
    show: (v, value) => `${v} · side ${sideOf(value) ** v}`,
    group: "View",
    when: reads("power"),
  },
  {
    key: "depth",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value) => cap(sideOf(value)) - 1,
    step: 1,
    show: (v, value) => `${v} · side ${sideOf(value) ** (v + 1)}`,
    group: "View",
    when: reads("filter"),
  },
  {
    key: "cell",
    label: "Cell",
    kind: "pick",
    def: 0,
    options: CELLS,
    group: "View",
    when: (value) => GRIDS.includes(pictureOf(value)),
  },
];

export const tidied = (value: Raw): Thue => ({
  ...value,
  ...(tidy(DESIGN, value) as Design),
  ...(tidy(SPEC, value) as Knobs),
});

export const units = wasm(...CRATES);

export function file(value: Raw, given?: Mrly) {
  const one = tidied(value);
  const kind = pictureOf(one);
  const code = DESIGNED.includes(kind) ? coded(given!, one) : "";
  const parts = {
    word: ["word", one.round, ...(one.colour === "runs" ? ["runs"] : [])],
    plane: [one.lift.toLowerCase(), one.level],
    power: [code, "power", one.power],
    filter: [code, "filter", one.depth],
  }[kind];
  return ["thue", ...parts].join("-");
}

const row = (key: string): Row => SPEC.find((one) => one.key === key)!;

export const KNOBS: Record<Kind, string> = { word: "round", plane: "level", power: "power", filter: "depth" };

const within = (n: number, low: number, high: number) => Math.max(low, Math.min(high, n));

export function shift(value: Value, by: number, wrap = false) {
  const key = KNOBS[pictureOf(value)];
  const spec = row(key);
  const low = minOf(spec, value)!;
  const top = maxOf(spec, value)!;
  const to = (value[key] as number) + by;
  return { [key]: wrap ? (to > top ? low : to < low ? top : to) : within(to, low, top) };
}

export const tapped = (value: Value) =>
  value.view === "plane" ? { lift: next(KINDS, value.lift, 1) } : shift(value, 1, true);

export const turned = (value: Value, by: number) => place(next(PICTURES, pictureOf(value), by));

const among = <T>(list: readonly T[], rand: Rng) => list[Math.floor(rand() * list.length)];

const span = (top: number) => Array.from({ length: top }, (_, i) => i + 1);

export function rerolled(mrly: Mrly, value: Value, rand: Rng) {
  const kind = pictureOf(value);
  if (kind === "plane")
    return {
      lift: among(
        KINDS.filter((one) => one !== value.lift),
        rand,
      ),
      level: among(span(LEVELS), rand),
    };
  if (kind === "word")
    return {
      round: among(
        span(ROUNDS).filter((one) => one !== value.round),
        rand,
      ),
      colour: among(COLOURS, rand)[0],
    };
  return { code: roll(mrly, sideOf(value), based(value), rand) };
}

export const PAGE = {
  defaults: { base: 2, code: "9" },
  spec: SPEC,
  units,
  name: file,
  design: { dim: DIM, base: "base", code: "code", when: (value) => value.view === "design" },
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.playing?.() ? "Stop" : "Play"),
      act: "play",
      button: true,
      menu: "Edit",
      when: word,
    },
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "View", act: "view" },
  ],
  actions: {
    play: (scene) => scene.toggle?.(),
  },
  acts: {
    up: (value) => shift(value, 1),
    down: (value) => shift(value, -1),
    view: (value, key) => turned(value, key === "ArrowLeft" ? -1 : 1),
  },
  reroll: (value, seed, units) => rerolled(units!, value, rng(seed)),
} satisfies Page<Loaded<Crate>, Handled>;

export function inks(canvas: Element | null | undefined): Inks {
  const css = typeof getComputedStyle === "function" && canvas ? getComputedStyle(canvas) : null;
  const read = (name: string, fallback: string) => css?.getPropertyValue(name).trim() || fallback;
  const base = typeof matchMedia === "function" && matchMedia("(prefers-color-scheme: dark)").matches ? dark : light;
  return {
    plus: palette.orange,
    minus: palette.blue,
    one: palette.yellow,
    two: palette.pink,
    diff: palette.pink,
    unlit: read("--panel", base.panel),
  };
}

const EDGE = 0.06;

export function bars(
  ctx: CanvasRenderingContext2D,
  rows: readonly ArrayLike<number>[],
  box: readonly Bar[],
  colours: readonly string[],
) {
  rows.forEach((row, k) => {
    const at = box[k];
    colours.forEach((colour, v) => {
      ctx.fillStyle = colour;
      for (let i = 0; i < row.length; i++) {
        if (row[i] !== v) continue;
        const x = Math.round(at.x + i * at.step);
        ctx.fillRect(x, at.y, Math.max(1, Math.round(at.x + (i + 1) * at.step) - x - at.gap), at.h);
      }
    });
  });
}

export function sketch(ctx: CanvasRenderingContext2D, w: number, h: number, dpr: number, plan: Plan, ink: Inks) {
  const pad = Math.round(w * EDGE);
  if (!plan.types)
    return bars(
      ctx,
      plan.rows,
      tower(w, h, plan.rows, dpr, pad),
      PAINTS[plan.paint].map((key) => ink[key]),
    );
  const box = lay(w, h, plan.side, 0, pad);
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(
    stamp(
      plan.types,
      plan.side,
      plan.side,
      PAINTS[plan.paint].map((key) => rgb(ink[key])),
    ),
    0,
    0,
    plan.side,
    plan.side,
    box.x,
    box.y,
    box.size,
    box.size,
  );
}

export function stamp(types: ArrayLike<number>, w: number, h: number, colours: readonly (readonly number[])[]) {
  const page = new OffscreenCanvas(w, h);
  const pen = page.getContext("2d")!;
  const image = pen.createImageData(w, h);
  const data = image.data;
  for (let i = 0; i < w * h; i++) {
    const colour = colours[types[i]];
    if (!colour) continue;
    data[i * 4] = colour[0];
    data[i * 4 + 1] = colour[1];
    data[i * 4 + 2] = colour[2];
    data[i * 4 + 3] = 255;
  }
  pen.putImageData(image, 0, 0);
  return page;
}

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { mrly } = opts;
  if (!mrly) throw new Error("thue: the scene wants the design crates as opts.mrly");
  const ctx = canvas.getContext("2d")!;
  let value = tidied(opts);
  let plan = study(mrly, value);
  let ink: Inks | null = null;
  let source: OffscreenCanvas | null = null;
  let grid: Grid | null = null;
  let stack: Bar[] | null = null;
  let dirty = true;
  const paint = () => {
    ink = inks(canvas);
    source = plan.types
      ? stamp(
          plan.types,
          plan.side,
          plan.side,
          PAINTS[plan.paint].map((key) => rgb(ink![key])),
        )
      : null;
  };
  const fit = () => {
    const { w, h, dpr } = view;
    if (plan.rows.length) {
      stack = tower(w, h, plan.rows, dpr);
      grid = null;
    } else {
      grid = lay(w, h, plan.side, value.cell * dpr, PAD * dpr);
      stack = null;
    }
  };
  const letters = (box: Bar[]) =>
    bars(
      ctx,
      plan.rows,
      box,
      PAINTS[plan.paint].map((key) => ink![key]),
    );
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    if (!grid && !stack) fit();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.imageSmoothingEnabled = false;
    if (source && grid) ctx.drawImage(source, 0, 0, plan.side, plan.side, grid.x, grid.y, grid.size, grid.size);
    else if (stack) letters(stack);
  };
  const size = () => {
    grid = null;
    stack = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    dirty = true;
  };
  const set = (next: Opts) => {
    if (word(next) !== word(value)) return false;
    value = tidied(next);
    const was = plan;
    plan = study(mrly, value);
    if (plan !== was) paint();
    grid = null;
    stack = null;
    dirty = true;
  };
  const svg = () => (fits(plan) ? picture(plan, ink!) : null);
  paint();
  return { draw, size, theme, set, svg, still: !word(value), facts: () => plan.facts };
}
