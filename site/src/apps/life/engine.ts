import * as life from "@mrlyprod/mrly/life";
import { fate, hash, population } from "../../lib/life/engine.ts";

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

export type Act =
  | { verb: "tick" }
  | { verb: "paint"; args: { cell: number; alive: number } }
  | { verb: "clear" }
  | { verb: "rule"; args: Law };

export const TICK: Act = { verb: "tick" };
export const CLEAR: Act = { verb: "clear" };
export const painting = (cell: number, alive: number): Act => ({ verb: "paint", args: { cell, alive } });
export const ruling = (law: Law): Act => ({ verb: "rule", args: law });

const ZERO = 48;
const encoder = new TextEncoder();
const decoder = new TextDecoder();

/* BITS */

export function bits(types: ArrayLike<number>): string {
  const out = new Uint8Array(types.length);
  for (let i = 0; i < types.length; i++) out[i] = types[i] ? ZERO + 1 : ZERO;
  return decoder.decode(out);
}

export function types(text: string): Uint8Array {
  const out = encoder.encode(text);
  for (let i = 0; i < out.length; i++) out[i] -= ZERO;
  return out;
}

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
