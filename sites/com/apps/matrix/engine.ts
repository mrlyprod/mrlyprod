import { HUES } from "@mrlyprod/ui/kit/color.ts";
import FONT from "@mrlyprod/ui/kit/font/font.gen.json" with { type: "json" };
import type { Entry } from "@mrlyprod/ui/kit/font/font.ts";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { xorshift } from "@mrlyprod/ui/kit/rand.ts";

export type State = {
  cols: number;
  rows: number;
  pool: number;
  hues: number;
  drops: number[];
  tints: number[];
  lit: number[];
  seed: number;
};

export type Config = Pick<State, "pool" | "hues" | "seed">;

export type Stage = { w: number; h: number; dpr: number; lock: boolean | number };

export type Act = { verb: "tick" } | { verb: "size"; args: { cols: number; rows: number; opening: number } };

export const TICK: Act = { verb: "tick" };
export const sizing = (cols: number, rows: number, opening: number): Act => ({
  verb: "size",
  args: { cols, rows, opening },
});

const WIDE = 4294967296;
const FALL = 0.975;
const SETS: Record<string, RegExp> = { l: /^[A-Z]$/, d: /^[0-9]$/, s: /^[^\sA-Za-z0-9]$/u };
const SIDE = 5;

const square = (char: string) =>
  (FONT as Record<string, Entry>)[char].rows.length === SIDE &&
  (FONT as Record<string, Entry>)[char].rows.every((row) => row.length === SIDE);

export function glyphs(set: string): string[] {
  const rules = [...set].map((one) => SETS[one]);
  return Object.keys(FONT).filter((char) => square(char) && rules.some((rule) => rule.test(char)));
}

export function fell(y: number, h: number, roll: number): boolean {
  return y > h && roll > FALL;
}

function roll(seed: number, count: number): [number, number] {
  const next = xorshift(seed);
  return [Math.floor((next / WIDE) * count), next];
}

const tint = (hues: number, seed: number): [number, number] => (hues ? roll(seed, hues) : [-1, seed]);

export function boot({ pool, hues, seed }: Config): State {
  return { cols: 0, rows: 0, pool, hues, drops: [], tints: [], lit: [], seed };
}

function size(state: State, cols: number, rows: number, opening: number): State {
  const drops = state.drops.slice();
  const tints = state.tints.slice();
  const open = Boolean(opening) && drops.length === 0;
  let seed = state.seed;
  for (let i = drops.length; i < cols; i++) {
    if (open) drops[i] = 1;
    else {
      const [k, next] = roll(seed, rows);
      drops[i] = 1 + k;
      seed = next;
    }
    [tints[i], seed] = tint(state.hues, seed);
  }
  return { ...state, cols, rows, drops, tints, seed };
}

function tick(state: State): State {
  const { cols, rows, pool, hues } = state;
  const drops = state.drops.slice();
  const tints = state.tints.slice();
  const lit: number[] = [];
  let seed = state.seed;
  for (let i = 0; i < cols; i++) {
    lit[i] = -1;
    if (drops[i] <= rows) [lit[i], seed] = roll(seed, pool);
    seed = xorshift(seed);
    if (fell(drops[i], rows, seed / WIDE)) {
      drops[i] = 0;
      [tints[i], seed] = tint(hues, seed);
    }
    drops[i]++;
  }
  return { ...state, drops, tints, lit, seed };
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  if (act.verb === "size") return size(state, act.args.cols, act.args.rows, act.args.opening);
  return state;
}

export const moves = (_state: State): Act[] => [TICK];

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

export const opening = ({ set, color }: { set: string; color: string }, rand: Rng): State =>
  boot({ pool: glyphs(set).length, hues: color === "rainbow" ? HUES.length : 0, seed: Math.floor(rand() * WIDE) });

export function fitted(state: State, cell: number, { w, h, dpr, lock }: Stage): State {
  const unit = Math.max(1, Math.round(cell * dpr));
  const cols = Math.max(1, Math.floor(w / unit));
  const rows = Math.max(1, Math.floor(h / unit));
  return step(state, sizing(cols, rows, lock ? 0 : 1));
}

export const deal = (value: { set: string; color: string; cell: number }, table: Stage & { rand: Rng }): State =>
  fitted(opening(value, table.rand), value.cell, table);
