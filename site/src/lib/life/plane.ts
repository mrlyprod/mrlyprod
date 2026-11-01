import type { Cell, Tensor } from "@mrlyprod/mrly/life";
import type { Board, Opts, View } from "../app/run.ts";
import { board } from "../app/run.ts";
import type { Value } from "../app/spec.ts";
import { veil } from "../color.ts";
import type { Mrly } from "../designs/engine.ts";
import type { Door } from "./engine.ts";
import type { Recipe, Run, Stop } from "./recipe.ts";
import { fixed, recipeOf, said } from "./recipe.ts";
import { runs } from "./scene.ts";

/* TYPES */

export type Stat = {
  gen: number;
  pop: number;
  fate: string;
  name: string;
  counts: number[][];
  mask: Tensor;
  cells: number;
  lattice: string;
  label: string;
  error: string;
  recipe?: string;
  sides?: [string, string];
  budget?: number;
  reach?: number;
  background?: number;
  flips?: boolean;
};

export type Read = (next: Opts) => Value;

export type Given = Opts & { life: Door; math: Mrly; read: Read; onStat?: (stat: Stat) => void };

export type Stage = {
  draw: () => void;
  size: () => void;
  theme: () => void;
  set: (next: Opts) => boolean;
  toggle: () => void;
  playing: () => boolean;
  step: () => void;
  doom: () => void;
  reset: () => void;
  clear: () => void;
  touch: (u: number, v: number, start: boolean) => undefined;
  stop: () => void;
};

type Skin = { ink: string; dim: string; paper: string; wall: string };

type Made = { mask: Tensor; budget: number; reach: number; flips: boolean; birth: number[]; survive: number[] };

export const KNOBS = [
  "mode",
  "shape",
  "side",
  "depth",
  "tiling",
  "cap",
  "x",
  "mask",
  "code",
  "number",
  "level",
  "born",
  "stay",
  "seed",
];

const FIT = 0.96;

const GROUND = 0.07;

const WALL = 0.35;

const OPS = 4e6;

const MOST = 32;

const EPS = 1e-6;

const NONE: Made = {
  mask: { shape: [0, 0], data: new Uint8Array(0) },
  budget: 0,
  reach: 0,
  flips: false,
  birth: [],
  survive: [],
};

const message = (thrown: unknown) => String((thrown as { message?: unknown } | null | undefined)?.message ?? thrown);

/* SCENE */

export function plane(canvas: HTMLCanvasElement, view: View, opts: Given): Stage {
  const ctx = canvas.getContext("2d")!;
  const { life, math, read } = opts;
  let value = read(opts);
  let run: Run | null = null;
  let recipe: Recipe | null = null;
  let made = NONE;
  let name = "";
  let error = "";
  let frame: Cell | null = null;
  let origin: [number, number] = [0, 0];
  let ended: Stop | undefined;
  let playing = Boolean(view.lock);
  let since = 0;
  let lay: Board | null = null;
  let skin: Skin | null = null;
  let dirty = true;
  let gone = false;
  const alone = () => recipe?.mode === "Island";
  const gen = () => run?.generation() ?? 0;
  const speed = () => Number(value.speed) || 1;
  const look = () => {
    if (!run) {
      frame = null;
      ended = undefined;
      return;
    }
    frame = run.frame();
    const [r, c] = run.origin();
    origin = [Number(r), Number(c)];
    ended = run.stop();
  };
  const build = () => {
    run?.free();
    run = null;
    recipe = null;
    made = NONE;
    name = "";
    error = "";
    try {
      recipe = recipeOf(life, math, value);
      name = life.recipe.name(recipe);
      run = new life.recipe.Run(recipe);
      made = {
        mask: run.mask(),
        budget: run.budget(),
        reach: run.reach(),
        flips: run.flips(),
        birth: Array.from(run.birth()),
        survive: Array.from(run.survive()),
      };
    } catch (thrown) {
      error = message(thrown);
    }
    look();
    since = view.t;
    dirty = true;
  };
  const dress = () => {
    const { accent, paper } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND), paper, wall: veil(accent, WALL) };
  };
  const tell = () =>
    !gone &&
    opts.onStat?.({
      gen: gen(),
      pop: run?.population() ?? 0,
      fate: ended ? said(ended) : "",
      name: recipe ? `B${recipe.birth}/S${recipe.survive}` : "",
      counts: [made.birth, made.survive],
      mask: made.mask,
      cells: made.budget,
      lattice: "",
      label: recipe ? `${recipe.mask}, budget ${made.budget}, reach ${made.reach}` : "",
      error,
      recipe: name,
      sides: recipe ? [recipe.birth, recipe.survive] : ["", ""],
      budget: made.budget,
      reach: made.reach,
      background: run?.background() ?? 0,
      flips: made.flips,
    });
  const advance = () => {
    if (!run || ended) return;
    ended = run.step();
    look();
    if (ended) playing = false;
  };
  const budget = () =>
    run ? Math.max(1, Math.min(MOST, Math.floor(OPS / (run.cap() ** 2 * Math.max(1, made.budget))))) : 1;
  const layout = () => board(view, run?.cap() ?? 1, run?.cap() ?? 1, FIT);
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.clearRect(0, 0, view.w, view.h);
    dirty = false;
    if (!run || !frame || !lay) return;
    const full = run.background() === 1;
    const [rows, cols] = frame.shape;
    const { x, y, w, h, cell } = lay;
    if (alone()) {
      ctx.fillStyle = full ? skin!.ink : skin!.dim;
      ctx.fillRect(x, y, w, h);
      ctx.strokeStyle = skin!.wall;
      ctx.lineWidth = view.dpr;
      ctx.strokeRect(x + 0.5, y + 0.5, w - 1, h - 1);
      const lo = -Math.floor((run.cap() - run.side()) / 2);
      ctx.fillStyle = full ? skin!.paper : skin!.ink;
      runs(ctx, frame.types, cols, rows, x + (origin[1] - lo) * cell, y + (origin[0] - lo) * cell, cell);
      return;
    }
    ctx.fillStyle = full ? skin!.ink : skin!.dim;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.fillStyle = full ? skin!.paper : skin!.ink;
    const span = cols * cell;
    for (let ty = y - Math.ceil(y / span) * span; ty < view.h; ty += span)
      for (let tx = x - Math.ceil(x / span) * span; tx < view.w; tx += span)
        runs(ctx, frame.types, cols, rows, tx, ty, cell);
  };
  const whole = () => {
    lay = layout();
    paint();
  };
  const theme = () => {
    dress();
    whole();
  };
  const draw = () => {
    if (view.fixed && !playing && !ended) toggle();
    if (playing) {
      const need = Math.floor(((view.t - since) * speed()) / 1000 + EPS) - gen();
      if (need > 0) {
        const n = Math.min(need, budget());
        for (let i = 0; i < n && playing; i++) advance();
        if (need > n) since = view.t - (gen() * 1000) / speed();
        dirty = true;
        tell();
      }
    }
    if (dirty) paint();
  };
  const toggle = () => {
    if (!playing && ended) build();
    playing = !playing;
    if (playing) {
      since = view.t - (gen() * 1000) / speed();
      view.wake?.();
    }
    tell();
    if (dirty) paint();
  };
  const step = () => {
    advance();
    since -= 1000 / speed();
    tell();
    paint();
  };
  const doom = () => {
    while (run && !ended) advance();
    playing = false;
    tell();
    paint();
  };
  const reset = () => {
    playing = false;
    build();
    tell();
    whole();
  };
  const set = (next: Opts) => {
    const was = value;
    value = read(next);
    if (fixed(value)) return false;
    const moved = (...keys: string[]) => keys.some((key) => String(value[key]) !== String(was[key]));
    if (moved(...KNOBS)) build();
    if (moved("speed")) since = view.t - (gen() * 1000) / speed();
    tell();
    whole();
    return true;
  };
  const stop = () => {
    gone = true;
    run?.free();
    run = null;
  };
  dress();
  build();
  whole();
  tell();
  return {
    draw,
    size: whole,
    theme,
    set,
    toggle,
    playing: () => playing,
    step,
    doom,
    reset,
    clear: () => {},
    touch: () => undefined,
    stop,
  };
}
