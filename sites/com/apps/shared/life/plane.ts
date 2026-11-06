import type { Tensor } from "@mrlyprod/life";
import { veil } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Board, Opts, View } from "@mrlyprod/ui/kit/run.ts";
import { board } from "@mrlyprod/ui/kit/run.ts";
import type { Value } from "@mrlyprod/ui/kit/spec.ts";
import type { Door } from "./engine.ts";
import { types } from "./engine.ts";
import type { Recipe } from "./recipe.ts";
import { fixed, recipeOf, said } from "./recipe.ts";
import type { State } from "./run.ts";
import { boot, step as play, TICK } from "./run.ts";
import { runs } from "./scene.ts";

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

export type Given = Opts & { life: Door; mrly: Mrly; read: Read; onStat?: (stat: Stat) => void };

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
  state: () => unknown;
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

export function plane(canvas: HTMLCanvasElement, view: View, opts: Given): Stage {
  const ctx = canvas.getContext("2d")!;
  const { life, mrly, read } = opts;
  let value = read(opts);
  let state: State | null = null;
  let recipe: Recipe | null = null;
  let made = NONE;
  let name = "";
  let error = "";
  let frame: Uint8Array | null = null;
  let playing = Boolean(view.lock);
  let since = 0;
  let lay: Board | null = null;
  let skin: Skin | null = null;
  let dirty = true;
  let gone = false;
  const alone = () => recipe?.mode === "Island";
  const gen = () => state?.gen ?? 0;
  const speed = () => Number(value.speed) || 1;
  const look = () => {
    frame = state ? types(state.cells) : null;
  };
  const build = () => {
    state = null;
    recipe = null;
    made = NONE;
    name = "";
    error = "";
    try {
      recipe = recipeOf(life, mrly, value);
      name = life.recipe.name(recipe);
      state = boot(recipe);
      made = {
        mask: { shape: state.span, data: types(state.mask) },
        budget: state.budget,
        reach: state.reach,
        flips: state.birth.includes(0),
        birth: state.birth,
        survive: state.survive,
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
      pop: state?.pop ?? 0,
      fate: state?.stop ? said(state.stop) : "",
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
      background: state?.background ?? 0,
      flips: made.flips,
    });
  const advance = () => {
    if (!state || state.stop) return;
    state = play(state, TICK);
    look();
    if (state.stop) playing = false;
  };
  const budget = () =>
    state ? Math.max(1, Math.min(MOST, Math.floor(OPS / (state.cap ** 2 * Math.max(1, made.budget))))) : 1;
  const layout = () => board(view, state?.cap ?? 1, state?.cap ?? 1, FIT);
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.clearRect(0, 0, view.w, view.h);
    dirty = false;
    if (!state || !frame || !lay) return;
    const full = state.background === 1;
    const { rows, cols, origin } = state;
    const { x, y, w, h, cell } = lay;
    if (alone()) {
      ctx.fillStyle = full ? skin!.ink : skin!.dim;
      ctx.fillRect(x, y, w, h);
      ctx.strokeStyle = skin!.wall;
      ctx.lineWidth = view.dpr;
      ctx.strokeRect(x + 0.5, y + 0.5, w - 1, h - 1);
      const lo = -Math.floor((state.cap - state.side) / 2);
      ctx.fillStyle = full ? skin!.paper : skin!.ink;
      runs(ctx, frame, cols, rows, x + (origin[1] - lo) * cell, y + (origin[0] - lo) * cell, cell);
      return;
    }
    ctx.fillStyle = full ? skin!.ink : skin!.dim;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.fillStyle = full ? skin!.paper : skin!.ink;
    const span = cols * cell;
    for (let ty = y - Math.ceil(y / span) * span; ty < view.h; ty += span)
      for (let tx = x - Math.ceil(x / span) * span; tx < view.w; tx += span) runs(ctx, frame, cols, rows, tx, ty, cell);
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
    if (view.fixed && !playing && !state?.stop) toggle();
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
    if (!playing && state?.stop) build();
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
    while (state && !state.stop) advance();
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
    state = null;
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
    state: () => state,
  };
}
