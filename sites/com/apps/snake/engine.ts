import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix, xorshift } from "@mrlyprod/ui/kit/rand.ts";

export type Over = "wall" | "bite" | "won" | "starved" | null;

export type State = {
  size: number;
  wrap: number;
  snake: number[];
  dir: number;
  queue: number[];
  foods: number[];
  score: number;
  hunger: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "size" | "wrap" | "seed"> & { apples: number };

export type Act = { verb: "turn"; args: { dir: number } } | { verb: "tick" } | { verb: "starve" };

export const TICK: Act = { verb: "tick" };
export const STARVE: Act = { verb: "starve" };
export const toward = (dir: number): Act => ({ verb: "turn", args: { dir } });

export const UP = 0;
export const RIGHT = 1;
export const DOWN = 2;
export const LEFT = 3;

const DX = [0, 1, 0, -1];
const DY = [-1, 0, 1, 0];
export const DIRS = [UP, RIGHT, DOWN, LEFT];
const LIMIT = 3;
const WIDE = 4294967296;

export const flip = (dir: number) => (dir + 2) & 3;

export function ahead(size: number, wrap: number, cell: number, dir: number): number {
  let x = (cell % size) + DX[dir];
  let y = Math.floor(cell / size) + DY[dir];
  if (wrap) {
    x = (x + size) % size;
    y = (y + size) % size;
  } else if (x < 0 || x >= size || y < 0 || y >= size) {
    return -1;
  }
  return y * size + x;
}

export function bearing(size: number, from: number, to: number): number | undefined {
  return DIRS.find((dir) => ahead(size, 1, from, dir) === to);
}

function drop(size: number, taken: readonly number[], seed: number): [number, number] {
  const used = new Uint8Array(size * size);
  for (const cell of taken) used[cell] = 1;
  const free: number[] = [];
  for (let cell = 0; cell < used.length; cell++) if (!used[cell]) free.push(cell);
  if (!free.length) return [-1, seed];
  const next = xorshift(seed);
  return [free[next % free.length], next];
}

export function boot({ size, apples, wrap, seed }: Config): State {
  let next = xorshift(mix(seed));
  const dir = next & 3;
  const head = (size >> 1) * size + (size >> 1);
  const snake = [head, ahead(size, wrap, head, flip(dir))];
  const foods: number[] = [];
  for (let i = 0; i < apples; i++) {
    const [spot, after] = drop(size, [...snake, ...foods], next);
    if (spot < 0) break;
    foods.push(spot);
    next = after;
  }
  return { size, wrap, snake, dir, queue: [], foods, score: 0, hunger: 0, over: null, seed: next };
}

function turn(state: State, dir: number): State {
  if (state.over || !DIRS.includes(dir) || state.queue.length >= LIMIT) return state;
  const last = state.queue.length ? state.queue[state.queue.length - 1] : state.dir;
  if (dir === last || dir === flip(last)) return state;
  return { ...state, queue: [...state.queue, dir] };
}

function tick(state: State): State {
  if (state.over) return state;
  const { size, wrap } = state;
  const queue = state.queue.slice();
  const dir = queue.length ? queue.shift()! : state.dir;
  const head = ahead(size, wrap, state.snake[0], dir);
  if (head < 0) return { ...state, dir, queue, over: "wall" };
  const eat = state.foods.indexOf(head);
  const body = eat >= 0 ? state.snake : state.snake.slice(0, -1);
  if (body.includes(head)) return { ...state, dir, queue, over: "bite" };
  const snake = [head, ...body];
  if (eat < 0) return { ...state, snake, dir, queue, hunger: state.hunger + 1 };
  const foods = state.foods.slice();
  const [spot, seed] = drop(size, [...snake, ...foods.filter((_, i) => i !== eat)], state.seed);
  if (spot < 0) foods.splice(eat, 1);
  else foods[eat] = spot;
  const over = snake.length === size * size ? "won" : null;
  return { ...state, snake, dir, queue, foods, score: state.score + 1, hunger: 0, over, seed };
}

export function step(state: State, act: Act): State {
  if (act.verb === "turn") return turn(state, act.args.dir);
  if (act.verb === "tick") return tick(state);
  if (act.verb === "starve" && !state.over) return { ...state, over: "starved" };
  return state;
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const turns = DIRS.filter((dir) => turn(state, dir) !== state).map(toward);
  return [...turns, TICK, STARVE];
}

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

const SQUARE = [0, 0, 0, 0];
const ENDS = [
  [0, 0, 1, 1],
  [1, 0, 0, 1],
  [1, 1, 0, 0],
  [0, 1, 1, 0],
];
const CORNERS: Record<number, number[]> = {
  [(1 << UP) | (1 << RIGHT)]: [0, 0, 0, 1],
  [(1 << UP) | (1 << LEFT)]: [0, 0, 1, 0],
  [(1 << DOWN) | (1 << RIGHT)]: [1, 0, 0, 0],
  [(1 << DOWN) | (1 << LEFT)]: [0, 1, 0, 0],
};

export function rounds(state: State): number[][] {
  const { size, snake } = state;
  const last = snake.length - 1;
  return snake.map((cell, i) => {
    if (last < 1) return SQUARE;
    if (i === 0) return ENDS[bearing(size, cell, snake[1])!] ?? SQUARE;
    if (i === last) return ENDS[bearing(size, cell, snake[i - 1])!] ?? SQUARE;
    const mask = (1 << bearing(size, cell, snake[i - 1])!) | (1 << bearing(size, cell, snake[i + 1])!);
    return CORNERS[mask] ?? SQUARE;
  });
}

export const deal = ({ size, apples, wrap }: Omit<Config, "seed">, { rand }: { rand: Rng }): State =>
  boot({ size, apples, wrap, seed: Math.floor(rand() * WIDE) });
