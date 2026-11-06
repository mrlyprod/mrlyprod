import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Ready } from "@mrlyprod/ui/kit/hooks.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Option, Spec, Units, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Unit } from "@mrlyprod/ui/kit/units/index.ts";
import type { Tensor } from "@mrlyprod/wolfram";
import type { Start } from "../shared/life/engine.ts";
import { population, sow } from "../shared/life/engine.ts";
import { PANE, reach, runs, SOUP } from "../shared/life/scene.ts";

export type Knobs = { rule: number; from: string; density: number; wrap: number; steps: number; seed?: number };

export type Stat = { rows: number; cols: number; live: number; how: string };

export type Door = Unit<"wolfram">;

export type Given = Opts & { wolfram: Door; mrly: Mrly; onStat?: (stat: Stat) => void };

export type Card = ReturnType<typeof card>;

export type Wolfram = ReturnType<typeof make>;

const RULES = 256;

const turn = (value: Value, by: number) => ({ rule: (Number(value.rule) + by + RULES) % RULES });

type Sheet = { rows: number; cols: number; data: Tensor["data"] };

type Skin = { ink: string; dim: string };

type Lay = { x: number; y: number; w: number; h: number; cell: number; lo: number; cols: number };

const STARTS: Start[] = [
  ["one", "One cell"],
  ["noise", "Soup"],
  ["design", "Carpet row"],
];

export const SPEC: Spec = [
  { key: "rule", label: "Rule", kind: "slider", def: 110, min: 0, max: 255, step: 1, group: "Rule", when: PANE },
  { key: "from", label: "Start", kind: "pick", def: "one", options: STARTS, group: "Run" },
  {
    key: "density",
    label: "Density",
    kind: "slider",
    def: 0.35,
    min: 0.05,
    max: 0.95,
    step: 0.01,
    group: "Run",
    when: SOUP,
  },
  { key: "wrap", label: "Wrap", kind: "toggle", def: 0, group: "Run" },
  {
    key: "steps",
    label: "Generations",
    kind: "segment",
    def: 128,
    options: [64, 128, 256, 512].map((n): Option => [n, String(n)]),
    group: "Run",
  },
];

export const FAMOUS: [number, string][] = [
  [30, "chaos"],
  [90, "Sierpinski"],
  [110, "universal"],
  [184, "traffic"],
  [54, "complex"],
  [150, "XOR"],
  [45, "random"],
  [22, "nested"],
  [60, "left XOR"],
  [102, "right XOR"],
  [126, "fractal"],
  [105, "XNOR"],
];

export const PAGE = {
  spec: SPEC,
  design: { dim: 3, base: 2, code: "rule" },
  gestures: true,
  name: (value) =>
    [
      "wolfram",
      value.rule,
      ...(value.from === "noise" ? ["soup", value.seed] : value.from === "design" ? ["carpet"] : []),
      ...(value.wrap ? ["ring"] : []),
    ].join("-"),
  keys: [
    { key: ["ArrowRight", "Enter", "."], label: "Next rule", act: "next", button: true, menu: "View" },
    { key: ["ArrowLeft", ","], label: "Previous rule", act: "prev", menu: "View" },
  ],
  acts: { next: (value) => turn(value, 1), prev: (value) => turn(value, -1) },
  reroll: (_value, seed) => ({ rule: Math.floor(rng(seed)() * RULES) }),
} satisfies Page<Ready & Units, Wolfram>;

const FIT = 0.96;
const GROUND = 0.07;
const THUMB = 24;
const MARGIN = 6;

export function card(wolfram: Door, rule: number) {
  const mates = Array.from(wolfram.cube_orbit(rule), (one) => ({ rule: one, onto: wolfram.surjective(one) }));
  const told = wolfram.outer_totalistic(rule);
  return {
    name: wolfram.rule_name(rule),
    corners: Uint8Array.from(wolfram.corner_bits(rule)),
    popcount: wolfram.popcount(rule),
    lambda: wolfram.lambda(rule),
    degree: wolfram.rule_degree(rule),
    genus: wolfram.genus(rule),
    affine: wolfram.affine(rule),
    onto: wolfram.surjective(rule),
    reversible: wolfram.reversible(rule),
    bs: told ? `B${Array.from(told[0]).join("")}/S${Array.from(told[1]).join("")}` : "",
    mates,
    klass: Array.from(wolfram.wolfram_class(rule)),
    npn: wolfram.npn_class(rule)[0],
    gasket: wolfram.gasket(rule) ?? "",
  };
}

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {} as Given) {
  const ctx = canvas.getContext("2d")!;
  const { wolfram, mrly } = opts;
  let value: Knobs = { ...(tidy(SPEC, opts) as Knobs), seed: opts.seed };
  let edited: Uint8Array | null = null;
  let grid: Sheet | null = null;
  let how = "";
  let skin: Skin | null = null;
  let lay: Lay | null = null;
  let span: [number, number] | null = null;
  let dirty = true;
  let gone = false;
  let brush = 1;
  const build = () => {
    const { rule, steps, from } = value;
    const wrap = Boolean(value.wrap);
    const width = 2 * steps + 1;
    const ring = wrap ? ` on a ring of ${width}` : "";
    let made: Tensor;
    if (edited) {
      made = wolfram.history(edited, rule, steps, wrap);
      how = `a seed row edited by hand${ring}`;
    } else if (from === "one" && !wrap) {
      made = wolfram.single_seed(rule, steps);
      how = `one live cell, padded by ${steps} and cropped to ${width}`;
    } else {
      made = wolfram.history(sow({ ...value, code: "7", number: 3 }, rng(value.seed), mrly, width), rule, steps, wrap);
      how = `${from === "noise" ? `a soup of ${width} cells at density ${value.density}` : from === "design" ? "the middle row of the carpet" : "one live cell"}${ring}`;
    }
    grid = { rows: made.shape[0], cols: made.shape[1], data: made.data };
  };
  const tell = () => !gone && opts.onStat?.({ rows: grid!.rows, cols: grid!.cols, live: population(grid!.data), how });
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const lit = (): [number, number] => {
    const box = reach(grid!.data, grid!.cols, grid!.rows);
    return box ? [Math.max(0, box.x0 - MARGIN), Math.min(grid!.cols - 1, box.x1 + MARGIN)] : [0, grid!.cols - 1];
  };
  const layout = (): Lay => {
    const [lo, hi] = span!;
    const cols = hi - lo + 1;
    const cell = Math.min((view.w * FIT) / cols, (view.h * FIT) / grid!.rows);
    const w = cell * cols;
    const h = cell * grid!.rows;
    return { x: (view.w - w) / 2, y: (view.h - h) / 2, w, h, cell, lo, cols };
  };
  const paint = () => {
    lay = layout();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = skin!.dim;
    ctx.fillRect(lay.x, lay.y, lay.w, lay.h);
    ctx.fillStyle = skin!.ink;
    ctx.save();
    ctx.beginPath();
    ctx.rect(lay.x, lay.y, lay.w, lay.h);
    ctx.clip();
    runs(ctx, grid!.data, grid!.cols, grid!.rows, lay.x - lay.lo * lay.cell, lay.y, lay.cell);
    ctx.restore();
    dirty = false;
  };
  const draw = () => {
    if (dirty) paint();
  };
  const theme = () => {
    dress();
    paint();
  };
  const redo = (keep?: boolean) => {
    build();
    if (!keep || !span) span = lit();
    tell();
    paint();
  };
  const touch = (u: number, v: number, start: boolean): undefined => {
    if (!lay) return;
    const col = Math.floor((u * view.w - lay.x) / lay.cell) + lay.lo;
    if (col < lay.lo || col >= lay.lo + lay.cols) return;
    if (Math.abs(v * view.h - lay.y - lay.cell / 2) > Math.max(lay.cell, THUMB * view.dpr)) return;
    edited ??= Uint8Array.from(grid!.data.subarray(0, grid!.cols));
    if (start) brush = edited[col] ? 0 : 1;
    if (edited[col] === brush) return;
    edited[col] = brush;
    redo(true);
  };
  const set = (next: Opts) => {
    const was = value;
    value = { ...(tidy(SPEC, next) as Knobs), seed: next.seed };
    if (
      (["steps", "from", "density", "seed"] satisfies (keyof Knobs)[]).some(
        (key) => String(value[key]) !== String(was[key]),
      )
    )
      edited = null;
    redo();
  };
  const stop = () => {
    gone = true;
  };
  dress();
  build();
  span = lit();
  tell();
  return { still: true, draw, size: paint, theme, set, touch, stop };
}
