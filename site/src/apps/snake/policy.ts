import type { Rng } from "../../lib/rand.ts";
import type { State } from "./engine.ts";
import { ahead, bearing, DIRS, flip } from "./engine.ts";

const STRAIGHT = 0.7;
const GOAL = 1;
const WALL = 2;

export const STARVED = -1;

/* HELPERS */

const heading = (state: State) => (state.queue.length ? state.queue[state.queue.length - 1] : state.dir);

function shuffled(rand: Rng): number[] {
  const order = DIRS.slice();
  for (let i = order.length - 1; i > 0; i--) {
    const j = Math.floor(rand() * (i + 1));
    [order[i], order[j]] = [order[j], order[i]];
  }
  return order;
}

function alive(state: State, dir: number): boolean {
  const { size, wrap, snake } = state;
  const to = ahead(size, wrap, snake[0], dir);
  if (to < 0) return false;
  const at = snake.indexOf(to);
  return at < 0 || at === snake.length - 1;
}

/* SEARCH */

function route(
  size: number,
  wrap: number,
  snake: readonly number[],
  order: readonly number[],
  goals: readonly number[],
  skip: number,
  walls: readonly number[] = [],
): number[] | null {
  const length = snake.length;
  const index = new Int32Array(size * size).fill(-1);
  snake.forEach((cell, i) => {
    index[cell] = i;
  });
  const mark = new Uint8Array(size * size);
  for (const cell of walls) mark[cell] = WALL;
  for (const cell of goals) mark[cell] = GOAL;
  const from = new Int32Array(size * size).fill(-2);
  const dist = new Int32Array(size * size);
  const head = snake[0];
  const queue = [head];
  from[head] = -1;
  for (let q = 0; q < queue.length; q++) {
    const at = queue[q];
    const moves = dist[at] + 1;
    for (const dir of order) {
      if (at === head && dir === skip) continue;
      const to = ahead(size, wrap, at, dir);
      if (to < 0 || from[to] !== -2 || mark[to] === WALL) continue;
      if (index[to] >= 0 && index[to] < length - moves) continue;
      from[to] = at;
      dist[to] = moves;
      if (mark[to] === GOAL) {
        const path = [to];
        while (from[path[0]] !== head) path.unshift(from[path[0]]);
        return path;
      }
      queue.push(to);
    }
  }
  return null;
}

function room(size: number, wrap: number, snake: readonly number[]): number {
  const used = new Uint8Array(size * size);
  for (const cell of snake) used[cell] = 1;
  const queue = [snake[0]];
  let count = 0;
  for (let q = 0; q < queue.length; q++) {
    for (const dir of DIRS) {
      const to = ahead(size, wrap, queue[q], dir);
      if (to < 0 || used[to]) continue;
      used[to] = 1;
      count++;
      queue.push(to);
    }
  }
  return count;
}

function grown(snake: readonly number[], way: readonly number[]): number[] {
  const seq = [...snake].reverse().concat(way);
  return seq.slice(seq.length - (snake.length + 1)).reverse();
}

/* POLICIES */

function smart(state: State, rand: Rng): number {
  const { size, wrap, snake, foods } = state;
  const order = shuffled(rand);
  const skip = flip(heading(state));
  const first = (way: readonly number[]) => bearing(size, snake[0], way[0])!;
  const safe = (way: readonly number[]) => {
    const at = way.findIndex((cell) => foods.includes(cell));
    if (at < 0) return true;
    const next = grown(snake, way.slice(0, at + 1));
    const rest = foods.filter((cell) => cell !== way[at]);
    return next.length === size * size || route(size, wrap, next, order, [next[next.length - 1]], -1, rest) !== null;
  };
  const way = route(size, wrap, snake, order, foods, skip);
  if (state.hunger >= size * size) return way ? first(way) : STARVED;
  if (way && safe(way)) return first(way);
  const tail = [snake[snake.length - 1]];
  const chase = route(size, wrap, snake, order, tail, skip);
  if (chase && safe(chase)) return first(chase);
  const detour = route(size, wrap, snake, order, tail, skip, foods);
  if (detour) return first(detour);
  let best = -1;
  let pick = heading(state);
  for (const dir of order) {
    if (dir === skip || !alive(state, dir)) continue;
    const to = ahead(size, wrap, snake[0], dir);
    const body = foods.includes(to) ? snake : snake.slice(0, -1);
    const count = room(size, wrap, [to, ...body]);
    if (count > best) {
      best = count;
      pick = dir;
    }
  }
  return pick;
}

function silly(state: State, rand: Rng): number {
  const course = heading(state);
  const open = DIRS.filter((dir) => dir !== flip(course) && alive(state, dir));
  if (!open.length) return course;
  const turns = open.filter((dir) => dir !== course);
  if (open.includes(course) && (!turns.length || rand() < STRAIGHT)) return course;
  return turns[Math.floor(rand() * turns.length)];
}

export function pick(state: State, mode: string, rand: Rng): number {
  return mode === "silly" ? silly(state, rand) : smart(state, rand);
}
