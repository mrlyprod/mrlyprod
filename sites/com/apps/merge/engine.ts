import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import { below } from "../shared/games/sprites.ts";

export type Act = { verb: "slide"; args: { dir: number } };

export type State = {
  cells: number[];
  score: number;
  over: boolean;
  fresh: number;
  seed: number;
};

export const SIZE = 4;

const UP = 0;
const RIGHT = 1;
const DOWN = 2;
const LEFT = 3;
const DIRS = [UP, RIGHT, DOWN, LEFT];
const SPAN = Array.from({ length: SIZE }, (_, i) => i);
const FAR = SIZE - 1;
const FOURS = 10;
const WIDE = 4294967296;

const SPOTS: Record<number, (line: number, i: number) => number> = {
  [UP]: (line, i) => i * SIZE + line,
  [RIGHT]: (line, i) => line * SIZE + FAR - i,
  [DOWN]: (line, i) => (FAR - i) * SIZE + line,
  [LEFT]: (line, i) => line * SIZE + i,
};

const LINES = DIRS.map((dir) => SPAN.map((line) => SPAN.map((i) => SPOTS[dir](line, i))));

function slid(cells: readonly number[], dir: number): { cells: number[]; gained: number } {
  const next = cells.map(() => 0);
  let gained = 0;
  for (const line of LINES[dir]) {
    const held = line.map((at) => cells[at]).filter(Boolean);
    let put = 0;
    for (let i = 0; i < held.length; i++) {
      const joined = held[i] === held[i + 1];
      next[line[put++]] = joined ? held[i] * 2 : held[i];
      if (joined) gained += held[i++] * 2;
    }
  }
  return { cells: next, gained };
}

const changes = (cells: readonly number[], dir: number) => slid(cells, dir).cells.some((one, at) => one !== cells[at]);

function spawned(state: State): State {
  const empty = state.cells.flatMap((one, at) => (one ? [] : [at]));
  const [spot, a] = below(state.seed, empty.length);
  const [roll, seed] = below(a, FOURS);
  const cells = state.cells.slice();
  const fresh = empty[spot];
  cells[fresh] = roll ? 2 : 4;
  return { ...state, cells, fresh, seed };
}

export function boot({ seed }: { seed: number }): State {
  const start: State = { cells: Array(SIZE * SIZE).fill(0), score: 0, over: false, fresh: -1, seed: mix(seed) };
  return spawned(spawned(start));
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return DIRS.filter((dir) => changes(state.cells, dir)).map((dir) => ({ verb: "slide", args: { dir } }));
}

export function step(state: State, act: Act): State {
  const { dir } = act.args;
  if (state.over || !DIRS.includes(dir) || !changes(state.cells, dir)) return state;
  const { cells, gained } = slid(state.cells, dir);
  const next = spawned({ ...state, cells, score: state.score + gained });
  return moves(next).length ? next : { ...next, over: true };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
