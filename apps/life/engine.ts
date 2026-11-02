import * as life from "@mrlyprod/life";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Value } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import type { Door, Rule, Shape } from "../shared/life/engine.ts";
import {
  BOARD,
  bits,
  fate,
  hash,
  LINE,
  maskOf,
  population,
  ruleOf,
  sideOf,
  sow,
  types,
} from "../shared/life/engine.ts";
import { fixed, recipeOf } from "../shared/life/recipe.ts";
import * as run from "../shared/life/run.ts";

/* TYPES */

export type State = {
  width: number;
  height: number;
  wrap: number;
  birth: number[];
  survive: number[];
  span: number[];
  mask: string;
  cells: string;
  gen: number;
  pop: number;
  seen: number[];
  word: string;
};

export type Config = Pick<State, "width" | "height" | "wrap" | "birth" | "survive" | "span" | "mask" | "cells">;

export type Law = Pick<State, "wrap" | "birth" | "survive" | "span" | "mask">;

export type Size = Pick<State, "width" | "height">;

export type Table = { seed?: number; rand: Rng; lock?: boolean | number; units: { life: Door; math: Mrly } };

export type Act =
  | { verb: "tick" }
  | { verb: "paint"; args: { cell: number; alive: number } }
  | { verb: "clear" }
  | { verb: "rule"; args: Law };

export const TICK: Act = { verb: "tick" };
export const CLEAR: Act = { verb: "clear" };
export const painting = (cell: number, alive: number): Act => ({ verb: "paint", args: { cell, alive } });
export const ruling = (law: Law): Act => ({ verb: "rule", args: law });

export { bits, types };

/* FATE */

function judged(state: State, cells: ArrayLike<number>, seen: number[]): State {
  const pop = population(cells);
  const kept = seen.slice();
  const word = fate(kept, hash(cells), pop);
  return { ...state, cells: bits(cells), pop, seen: kept, word };
}

/* RULES */

export function boot(config: Config): State {
  return judged({ ...config, gen: 0, pop: 0, seen: [], word: "" }, types(config.cells), []);
}

function tick(state: State): State {
  const grid = { shape: [state.height, state.width], types: types(state.cells) };
  const mask = { shape: state.span, data: types(state.mask) };
  const next = life.next_grid(grid, state.birth, state.survive, mask, state.wrap ? "Wrap" : "Constant").types;
  return judged({ ...state, gen: state.gen + 1 }, next, state.seen);
}

function paint(state: State, cell: number, alive: number): State {
  if (cell < 0 || cell >= state.cells.length || Number(state.cells[cell]) === alive) return state;
  const cells = types(state.cells);
  cells[cell] = alive;
  return judged(state, cells, []);
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  if (act.verb === "paint") return paint(state, act.args.cell, act.args.alive ? 1 : 0);
  if (act.verb === "clear") return judged({ ...state, gen: 0 }, new Uint8Array(state.cells.length), []);
  if (act.verb === "rule") {
    const { wrap, birth, survive, span, mask } = act.args;
    return { ...state, wrap, birth, survive, span, mask };
  }
  return state;
}

export const moves = (state: State): Act[] => [
  TICK,
  CLEAR,
  ...Array.from(state.cells, (bit, cell) => painting(cell, bit === "1" ? 0 : 1)),
];

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

/* DEAL */

export function sized(dim: number, value: Value, lock?: boolean | number): Size {
  const width = dim === 1 ? LINE : lock ? BOARD : sideOf(value);
  return { width, height: dim === 1 ? 1 : width };
}

export const lawOf = (value: Value, rule: Rule, mask: Shape["mask"]): Law => ({
  wrap: value.wrap ? 1 : 0,
  birth: rule.birth,
  survive: rule.survive,
  span: [...mask.shape],
  mask: bits(mask.data),
});

export const sown = (value: Value, rand: Rng, math: Mrly, size: Size, law: Law): State =>
  boot({ ...size, ...law, cells: bits(sow(value, rand, math, size.width, size.height)) });

export function deal(value: Value, { seed, rand, lock, units: { life, math } }: Table): State | run.State {
  const knobs = { ...value, seed } as Value;
  if (!fixed(knobs)) return run.boot(recipeOf(life, math, knobs));
  const shape = maskOf(life, knobs);
  return sown(
    knobs,
    rand,
    math,
    sized(shape.dim, knobs, lock),
    lawOf(knobs, ruleOf(life, knobs, shape.cells), shape.mask),
  );
}

export const reducer = (value: Value) => (fixed(value) ? { boot, moves, step, read } : run);
