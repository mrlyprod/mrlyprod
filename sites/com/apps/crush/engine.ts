import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { below, sprite } from "../shared/games/sprites.ts";

export type Tile = Sprite & { ripe: boolean };

export type Falling = { x: number; y: number; tile: Tile };

export type Act = { verb: "tick" } | { verb: "shift"; args: { dx: number } } | { verb: "drop" } | { verb: "crush" };

export type State = {
  grid: (Tile | null)[];
  falling: Falling | null;
  crushed: number;
  over: boolean;
  seed: number;
};

export const COLS = 9;
export const ROWS = 9;

export const TICK: Act = { verb: "tick" };
export const DROP: Act = { verb: "drop" };
export const CRUSH: Act = { verb: "crush" };
export const shift = (dx: number): Act => ({ verb: "shift", args: { dx } });

const GROUP = 3;
const WIDE = 4294967296;

const free = (grid: State["grid"], x: number, y: number) =>
  x >= 0 && x < COLS && y >= 0 && y < ROWS && !grid[y * COLS + x];

const around = (at: number) => {
  const x = at % COLS;
  return [at - COLS, at + COLS, x ? at - 1 : -1, x < COLS - 1 ? at + 1 : -1].filter(
    (one) => one >= 0 && one < COLS * ROWS,
  );
};

function ripened(grid: State["grid"]): State["grid"] {
  const next = grid.slice();
  const seen = new Set<number>();
  for (const [start, tile] of grid.entries()) {
    if (!tile || seen.has(start)) continue;
    const group = [start];
    seen.add(start);
    for (const at of group)
      for (const near of around(at)) {
        if (seen.has(near) || grid[near]?.design !== tile.design) continue;
        seen.add(near);
        group.push(near);
      }
    if (group.length < GROUP) continue;
    for (const at of group) next[at] = { ...grid[at]!, ripe: true };
  }
  return next;
}

function fell(grid: State["grid"]): State["grid"] {
  const next: State["grid"] = grid.map(() => null);
  for (let x = 0; x < COLS; x++) {
    let write = ROWS - 1;
    for (let y = ROWS - 1; y >= 0; y--) {
      const tile = grid[y * COLS + x];
      if (tile) next[write-- * COLS + x] = tile;
    }
  }
  return next;
}

function spawned(state: State): State {
  const [made, a] = sprite(state.seed);
  const [x, seed] = below(a, COLS);
  return { ...state, falling: { x, y: 0, tile: { ...made, ripe: false } }, seed };
}

function locked(state: State, { x, y, tile }: Falling): State {
  const placed = state.grid.slice();
  placed[y * COLS + x] = tile;
  const grid = ripened(placed);
  if (grid.slice(0, COLS).some(Boolean)) return { ...state, grid, falling: null, over: true };
  return spawned({ ...state, grid });
}

function landing(state: State, { x, y }: Falling): number {
  let at = y;
  while (free(state.grid, x, at + 1)) at++;
  return at;
}

export function boot({ seed }: { seed: number }): State {
  const start: State = { grid: Array(COLS * ROWS).fill(null), falling: null, crushed: 0, over: false, seed: mix(seed) };
  return spawned(start);
}

export function moves(state: State): Act[] {
  const { falling } = state;
  if (state.over || !falling) return [];
  const sides = [-1, 1].filter((dx) => free(state.grid, falling.x + dx, falling.y)).map(shift);
  return [TICK, ...sides, DROP, CRUSH];
}

export function step(state: State, act: Act): State {
  const { falling } = state;
  if (state.over || !falling) return state;
  if (act.verb === "tick")
    return free(state.grid, falling.x, falling.y + 1)
      ? { ...state, falling: { ...falling, y: falling.y + 1 } }
      : locked(state, falling);
  if (act.verb === "shift") {
    const { dx } = act.args;
    if (Math.abs(dx) !== 1 || !free(state.grid, falling.x + dx, falling.y)) return state;
    return { ...state, falling: { ...falling, x: falling.x + dx } };
  }
  if (act.verb === "drop") return locked(state, { ...falling, y: landing(state, falling) });
  if (act.verb !== "crush") return state;
  const ripe = state.grid.filter((tile) => tile?.ripe).length;
  if (!ripe) return state;
  const left = state.grid.map((tile) => (tile?.ripe ? null : tile));
  return { ...state, grid: ripened(fell(left)), crushed: state.crushed + ripe };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
