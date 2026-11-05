import type { Color } from "@mrlyprod/math";
import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { rgb, TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { canon, DESIGN } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Design } from "@mrlyprod/ui/kit/designs/store.ts";
import type { Had } from "@mrlyprod/ui/kit/hooks.ts";
import type { Mathy, Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Opts, Scene, View } from "@mrlyprod/ui/kit/run.ts";
import type { Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Built, Facts, Knobs, Look, Shade, Study } from "./engine.ts";
import {
  cap,
  copies,
  croppable,
  DIM,
  fit,
  kinds,
  mesh,
  NUMBERS,
  REPEATS,
  RINGS,
  reroll,
  SCALE,
  SHEETS,
  studio,
  VIEWS,
  wrap,
} from "./engine.ts";

export type Given = Opts & { math?: Mrly | null; onFacts?: (facts: Facts) => void };

export type Tones = Record<Shade, Color> & Partial<Record<string, Color>>;

const ORDER = VIEWS.map(([key]) => key);

export const turn = (value: Value, by: number) => ({
  view: ORDER[(ORDER.indexOf(value.view as Look) + by + ORDER.length) % ORDER.length],
});

export type Sheet = ReturnType<typeof make>;

const rect = (value: Value) => value.sheet !== "disc";

export const SPEC: Spec<Had<"math">> = [
  { key: "view", label: "View", kind: "segment", def: "cut", options: VIEWS },
  { key: "sheet", label: "Shape", kind: "segment", def: "rect", options: SHEETS, group: "Sheet" },
  { key: "x", label: "Across", kind: "slider", def: 5, min: 1, max: REPEATS, step: 1, group: "Sheet", when: rect },
  { key: "y", label: "Down", kind: "slider", def: 5, min: 1, max: REPEATS, step: 1, group: "Sheet", when: rect },
  {
    key: "radius",
    label: "Rings",
    kind: "slider",
    def: 2,
    min: 0,
    max: RINGS,
    step: 1,
    group: "Sheet",
    when: (value) => !rect(value),
  },
  { key: "overhang", label: "Keep the overhang", kind: "toggle", def: 1, group: "Sheet", when: croppable },
  { key: "number", label: "Side", kind: "pick", def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: "Tile" },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 2,
    min: 1,
    max: (value, units) => units && cap(units.math, value.number as number, copies(units.math, value as Knobs)),
    step: 1,
    group: "Tile",
    keep: true,
  },
  { key: "turn", label: "Quarter turn", kind: "toggle", def: 0, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const units = wasm("math");

export function name(value: Value, units?: Mathy) {
  const { base, code } = tidy(DESIGN, { ...value, dim: DIM }) as Design;
  return `six-${units?.math ? canon(units.math, DIM, base, code) : code}-${value.view}`;
}

export const PAGE = {
  defaults: { base: 2, code: "23" },
  spec: SPEC,
  units,
  name,
  design: { dim: DIM, base: "base", code: "code" },
  keys: [
    {
      key: "Enter",
      label: "Crop",
      act: "crop",
      button: true,
      menu: "Edit",
      when: (value) => croppable(value) && value.overhang === 1,
    },
    {
      key: "Enter",
      label: "Uncrop",
      act: "crop",
      button: true,
      menu: "Edit",
      when: (value) => croppable(value) && value.overhang !== 1,
    },
    { key: ["ArrowLeft", "ArrowRight"], label: "View", act: "view" },
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
  ],
  acts: {
    crop: (value) => ({ overhang: value.overhang === 1 ? 0 : 1 }),
    view: (value, key) => turn(value, key === "ArrowLeft" ? -1 : 1),
    up: (value) => ({ level: Number(value.level) + 1 }),
    down: (value) => ({ level: Number(value.level) - 1 }),
  },
  reroll: (value, seed, units) => ({ code: reroll(units!.math, Number(tidy(DESIGN, value).base), seed) }),
} satisfies Page<Loaded<"math">, Scene>;

const PAD = 24;
const EDGE = 0.06;
const VOID = 0.1;
const LEFT = 0.72;
const RIGHT = 0.5;

export function tones(look: Pick<Paints, "accent" | "paper">): Tones {
  const a = rgb(look.accent);
  const p = rgb(look.paper);
  const toward = (k: number): Color => {
    const at = (i: number) => Math.round(p[i] + (a[i] - p[i]) * k);
    return [at(0), at(1), at(2), 255];
  };
  return {
    FILL: [...a, 255],
    VOID: [...a, Math.round(255 * VOID)],
    UP: [...a, 255],
    LEFT: toward(LEFT),
    RIGHT: toward(RIGHT),
  };
}

const css = ([r, g, b, a]: readonly number[]) => `rgba(${r}, ${g}, ${b}, ${Math.round((a / 255) * 1000) / 1000})`;

export function fill(ctx: CanvasRenderingContext2D, built: Built, look: Tones, w: number, h: number, pad: number) {
  const { k, ox, oy } = fit(built.box, w, h, pad);
  for (const { name, points } of built.groups) {
    ctx.fillStyle = css(look[name]);
    ctx.beginPath();
    for (let i = 0; i < points.length; i += 6) {
      ctx.moveTo(ox + points[i] * k, oy + points[i + 1] * k);
      ctx.lineTo(ox + points[i + 2] * k, oy + points[i + 3] * k);
      ctx.lineTo(ox + points[i + 4] * k, oy + points[i + 5] * k);
      ctx.closePath();
    }
    ctx.fill();
  }
}

export const sketch = (
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  built: Built,
  look: Pick<Paints, "accent" | "paper">,
) => fill(ctx, built, tones(look), w, h, w * EDGE);

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("six: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const tell = opts.onFacts;
  const study = studio(math);
  const codes = kinds(math);
  let now: Study | null = null;
  let built: Built | null = null;
  let made = "";
  let turn = false;
  const take = (next: Opts) => {
    const value = {
      ...(tidy(SPEC, next, { math }) as Knobs),
      ...(tidy(DESIGN, { ...next, dim: DIM }) as Design),
      seed: next.seed,
    };
    now = study(value);
    turn = value.turn === 1;
    const key = `${now.key}${turn}`;
    if (key !== made) {
      built = mesh(math, now.drawn, turn);
      made = key;
    }
    tell?.(now.facts);
  };
  take(opts);
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    fill(ctx, built!, tones(view.look()), view.w, view.h, PAD * view.dpr);
  };
  const custom = () => {
    const look = tones(view.look());
    return Object.fromEntries(
      Object.entries(codes).map(([name, type]): [string, Color[]] => [String(type), [look[name] ?? [0, 0, 0, 0]]]),
    );
  };
  const svg = () =>
    wrap(
      math.six.svg(math.six.paint(now!.drawn, custom()), SCALE, undefined, 0, undefined),
      now!.drawn.orientation === "Horizontal",
      turn,
    );
  const tile = () =>
    wrap(math.six.rect_svg(math.six.paint(now!.one, custom()), SCALE), now!.one.orientation === "Horizontal", turn);
  return { draw: paint, still: true, set: take, svg, tile };
}
