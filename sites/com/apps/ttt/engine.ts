import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { below, sprites } from "../shared/games/sprites.ts";

export type Over = "won" | "lost" | "draw" | null;

export type Act = { verb: "place"; args: { at: number } };

export type State = {
  cells: (number | null)[];
  faces: Sprite[];
  over: Over;
  line: number[];
  seed: number;
};

export const SIDE = 3;

const ME = 0;
const RIVAL = 1;
const WIDE = 4294967296;

const WINS = [
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  [0, 4, 8],
  [2, 4, 6],
];

const free = (cells: State["cells"]) => cells.flatMap((one, at) => (one === null ? [at] : []));

const won = (cells: State["cells"]) =>
  WINS.find(([a, b, c]) => cells[a] !== null && cells[a] === cells[b] && cells[a] === cells[c]) ?? [];

function judged(state: State): State {
  const line = won(state.cells);
  if (line.length) return { ...state, line, over: state.cells[line[0]] === ME ? "won" : "lost" };
  return free(state.cells).length ? state : { ...state, over: "draw" };
}

export function boot({ seed }: { seed: number }): State {
  const [faces, next] = sprites(mix(seed), 2);
  return { cells: Array(SIDE * SIDE).fill(null), faces, over: null, line: [], seed: next };
}

export function moves(state: State): Act[] {
  return state.over ? [] : free(state.cells).map((at) => ({ verb: "place", args: { at } }));
}

export function step(state: State, act: Act): State {
  const { at } = act.args;
  if (state.over || state.cells[at] !== null) return state;
  const cells = state.cells.slice();
  cells[at] = ME;
  const mine = judged({ ...state, cells });
  if (mine.over) return mine;
  const open = free(cells);
  const [pick, seed] = below(state.seed, open.length);
  cells[open[pick]] = RIVAL;
  return judged({ ...state, cells, seed });
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
