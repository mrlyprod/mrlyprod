import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Box, Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Scalar, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Touched } from "@mrlyprod/ui/kit/touch.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Combine, Held, Look, Plan, Ramp, Table, Trial } from "../shared/moire/engine.ts";
import {
  BEAT,
  COMBINES,
  CROPS,
  climb,
  colorizer,
  crop,
  css,
  DEEPEST,
  drawn,
  field,
  GRAIN,
  heat,
  law,
  matrix,
  odd,
  paint,
  palette,
  RAMPS,
  SIZES,
  study,
  TOP,
} from "../shared/moire/engine.ts";
import { fit, shell } from "../shared/moire/sheet.ts";

export type Moire = {
  view: string;
  limit: number;
  level: number;
  combine: Combine;
  size: number;
  scale: number;
  mate: number;
  ramp: Ramp;
  levels: number;
  invert: number;
  tint: string;
  crop: string;
  radius: number;
  anti: number;
  base: Scalar;
  code: Scalar;
  seed: number;
};

export type Fault = { error: string };

export type Matrix = {
  view: "matrix";
  row: number;
  mate: number;
  r: number | null;
  prime: boolean;
  max: number;
  at: number;
  k: number;
};

export type Field = {
  view: "field";
  name: string;
  code: string;
  base: number;
  level: number;
  layers: number;
  scales: number[];
  size: number;
};

export type Facts = Fault | Matrix | Field;

export type Given = Opts & {
  mrly?: Mrly | null;
  base?: Scalar;
  code?: Scalar;
  onFacts?: (facts: Facts) => void;
  onLimit?: (limit: number) => void;
};

export type Grid = { x: number; y: number; cell: number; n: number };

export type Player = Touched & {
  draw: () => void;
  set?: (next: Opts) => void;
  toggle?: () => boolean;
  theme?: () => void;
  size?: () => void;
  stop?: () => void;
  playing?: () => boolean;
};

const VIEWS: [string, string][] = [
  ["field", "Field"],
  ["matrix", "R\u2011matrix"],
];

const stacks = (value: Value) => value.view !== "matrix";

const reads = (value: Value) => !stacks(value);

const cropped = (value: Value) => stacks(value) && Boolean(value.crop);

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "field", options: VIEWS, group: "Stack" },
  { key: "limit", label: "Scales up to", kind: "slider", def: TOP, min: 1, max: TOP, step: 2, group: "Stack" },
  { key: "level", label: "Level", kind: "slider", def: 1, min: 1, max: DEEPEST, step: 1, when: stacks, group: "Stack" },
  { key: "combine", label: "Combine", kind: "pick", def: "sum", options: COMBINES, when: stacks, group: "Stack" },
  {
    key: "size",
    label: "Size",
    kind: "segment",
    def: 512,
    options: SIZES.map((n) => [n, String(n)]),
    when: stacks,
    group: "Stack",
  },
  {
    key: "scale",
    label: "Carpet m",
    kind: "slider",
    def: 9,
    min: 3,
    max: (value) => Math.max(3, odd(Number(value.limit)).at(-1)!),
    step: 2,
    keep: true,
    when: reads,
    group: "R-matrix",
  },
  {
    key: "mate",
    label: "Against n",
    kind: "slider",
    def: 3,
    min: 3,
    max: (value) => Math.max(3, Number(value.scale) - 2),
    step: 2,
    keep: true,
    when: (value) => reads(value) && Number(value.scale) > 3,
    group: "R-matrix",
  },
  { key: "ramp", label: "Ramp", kind: "pick", def: "tint", options: RAMPS, when: stacks, group: "Look" },
  { key: "levels", label: "Levels", kind: "slider", def: 16, min: 2, max: 64, step: 1, when: stacks, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, when: stacks, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
  { key: "crop", label: "Crop", kind: "pick", def: "", options: CROPS, when: stacks, group: "Crop" },
  {
    key: "radius",
    label: "Radius",
    kind: "slider",
    def: 16,
    min: 1,
    max: GRAIN,
    step: 1,
    unit: `of ${GRAIN}`,
    when: cropped,
    group: "Crop",
  },
  { key: "anti", label: "Anti", kind: "toggle", def: 0, when: cropped, group: "Crop" },
];

export const DESIGN = { base: 2, code: "9" };

const named = (value: Value) =>
  value.view === "matrix" ? `moire-matrix-${value.limit}` : `moire-${value.code}-${value.limit}`;

export const units = wasm(...CRATES);

export const PAGE = {
  defaults: DESIGN,
  design: { dim: 2, base: "base", code: "code" },
  spec: SPEC,
  units,
  record: true,
  gestures: true,
  name: named,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.playing?.() ? "Stop" : "Play"),
      act: "play",
      button: true,
      menu: "Edit",
    },
  ],
  actions: { play: (scene) => scene.toggle?.() },
  reroll: (_value, seed, units) => drawn(units!, seed),
} satisfies Page<Loaded<Crate>, Player>;

const read = (opts: Given) =>
  ({ ...tidy(SPEC, opts), base: opts.base, code: opts.code, seed: Number(opts.seed) >>> 0 }) as Moire;

const GAP = 0.06;

export function cells(view: Box, n: number): Grid {
  const box = fit(view.w, view.h);
  const cell = Math.max(1, Math.floor(box.side / Math.max(1, n)));
  const span = cell * n;
  return { x: box.x + Math.floor((box.side - span) / 2), y: box.y + Math.floor((box.side - span) / 2), cell, n };
}

export function cellAt(grid: Grid, x: number, y: number): [number, number] | null {
  const i = Math.floor((y - grid.y) / grid.cell);
  const j = Math.floor((x - grid.x) / grid.cell);
  return i >= 0 && j >= 0 && i < grid.n && j < grid.n ? [i, j] : null;
}

function board(ctx: CanvasRenderingContext2D, view: View, look: Look, table: Table, trial: Trial) {
  const { scales, grid: rows } = table;
  const n = scales.length;
  if (!n) return null;
  const ink = palette(look);
  const grid = cells(view, n);
  const { x, y, cell } = grid;
  const gap = Math.max(1, Math.round(cell * GAP));
  rows.forEach((row, i) => {
    row.forEach((r, j) => {
      ctx.fillStyle = css(heat(ink, r));
      ctx.fillRect(x + j * cell, y + i * cell, cell - gap, cell - gap);
    });
  });
  ctx.lineWidth = Math.max(1, view.dpr);
  ctx.strokeStyle = css(ink.dim);
  ctx.beginPath();
  ctx.moveTo(x, y);
  ctx.lineTo(x + n * cell, y + n * cell);
  ctx.stroke();
  const line = Math.max(2, Math.round(view.dpr * 1.5));
  ctx.lineWidth = line;
  const i = scales.indexOf(trial.row);
  const j = scales.indexOf(trial.mate);
  if (i >= 0) {
    ctx.strokeStyle = css(ink.yellow);
    ctx.strokeRect(x - line, y + i * cell - line, n * cell + line, cell + line);
  }
  if (j >= 0) {
    ctx.strokeStyle = css(ink.pink);
    ctx.strokeRect(x + j * cell - line, y - line, cell + line, n * cell + line);
  }
  return grid;
}

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Player {
  const mrly = opts.mrly;
  if (!mrly) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  let value = read(opts);
  let plan: Plan | null = null;
  let held: Held | null = null;
  let trial: Trial | null = null;
  let grid: Grid | null = null;
  let error = "";
  let tone = 0;
  let playing = false;
  let from = value.limit;
  let origin = 0;
  let dirty = true;
  let told = "";
  let failed = "";
  let gone = false;
  const done = { plan: "", field: "", paint: "", law: "" };
  const planKey = () => [value.base, value.code, value.level, value.limit, value.combine].join(" ");
  const fieldKey = () => [planKey(), value.size, value.crop, value.radius, value.anti].join(" ");
  const paintKey = () => [fieldKey(), value.ramp, value.levels, value.invert, value.tint, tone].join(" ");
  const lawKey = () => [value.limit, value.scale, value.mate].join(" ");
  const facts = (): Facts => {
    if (error) return { error };
    if (reads(value)) {
      const w = trial!.witness;
      return {
        view: "matrix",
        row: trial!.row,
        mate: trial!.mate,
        r: trial!.r,
        prime: w?.prime ?? false,
        max: w?.max ?? 0,
        at: w?.at ?? 0,
        k: trial!.k,
      };
    }
    return {
      view: "field",
      name: plan!.name,
      code: plan!.code,
      base: plan!.base,
      level: plan!.level,
      layers: plan!.numbers.length,
      scales: plan!.numbers,
      size: held!.size,
    };
  };
  const tell = () => {
    if (gone) return;
    const now = facts();
    const key = JSON.stringify(now);
    if (key === told) return;
    told = key;
    opts.onFacts?.(now);
  };
  const stack = () => {
    if (done.plan !== planKey()) {
      plan = study(mrly, value);
      done.plan = planKey();
    }
    if (done.field !== fieldKey()) {
      held?.field.free();
      held = null;
      held = field(mrly, plan!, value.size, crop(mrly, value.crop, value.radius, value.anti, value.size));
      done.field = fieldKey();
    }
    if (done.paint !== paintKey()) {
      pad.put(
        paint(mrly, held!, colorizer(value.ramp, value.levels, view.look()), value.levels, value.invert),
        held!.size,
      );
      done.paint = paintKey();
      dirty = true;
    }
  };
  const weigh = () => {
    if (done.law !== lawKey()) {
      trial = law(mrly, value.limit, value.scale, value.mate);
      done.law = lawKey();
      dirty = true;
    }
  };
  const sync = () => {
    const want = reads(value) ? lawKey() : paintKey();
    if (want === failed) return;
    try {
      if (reads(value)) weigh();
      else stack();
      error = "";
      failed = "";
    } catch (fault) {
      error = String((fault as { message?: unknown } | null | undefined)?.message ?? fault);
      failed = want;
      dirty = true;
    }
    tell();
  };
  const frame = () => {
    pad.clear();
    grid = null;
    if (!error && reads(value)) grid = board(pad.ctx, view, view.look(), matrix(mrly, value.limit), trial!);
    else if (!error && held) pad.blit(held.size, fit(view.w, view.h), false);
    dirty = false;
  };
  const draw = () => {
    if (view.fixed && !playing) toggle();
    if (playing) {
      const k = climb(from, 1 + Math.floor((view.t - origin) / BEAT));
      if (k !== value.limit) {
        value = { ...value, limit: k };
        opts.onLimit?.(k);
      }
    }
    sync();
    if (dirty) frame();
  };
  const toggle = () => {
    playing = !playing;
    if (playing) {
      from = value.limit;
      origin = view.t;
      view.wake?.();
    }
    return playing;
  };
  const set = (next: Opts) => {
    const was = value.limit;
    value = read(next);
    if (playing && value.limit !== was) {
      from = value.limit;
      origin = view.t;
    }
    dirty = true;
  };
  const touch = (u: number, v: number, start: boolean) => {
    if (stacks(value)) {
      if (start) toggle();
      return null;
    }
    const at = grid && cellAt(grid, u * view.w, v * view.h);
    if (!at) return null;
    const { scales } = matrix(mrly, value.limit);
    const [i, j] = at;
    return i === j ? { scale: scales[i] } : { scale: scales[Math.max(i, j)], mate: scales[Math.min(i, j)] };
  };
  const theme = () => {
    tone += 1;
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const stop = () => {
    gone = true;
    held?.field.free();
    held = null;
  };
  return { draw, set, toggle, touch, theme, size, stop, playing: () => playing };
}
