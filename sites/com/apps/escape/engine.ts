import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { below, sprites } from "../shared/games/sprites.ts";

export type Ending = "door" | "caught" | null;

export type Act = { verb: "tick" } | { verb: "go"; args: { dir: number } };

export type State = {
  layout: number;
  runner: number;
  hunters: number[];
  dots: number[];
  want: number;
  moved: number;
  hold: number;
  ending: Ending;
  level: number;
  faces: { runner: Sprite; hunters: Sprite[]; number: number };
  over: boolean;
  seed: number;
};

export const SIDE = 11;
export const TICK_MS = 150;
export const WALL = 0;
export const DOOR = 4;
export const TICK: Act = { verb: "tick" };

const START = 1;
const FLOOR = 2;
const SPAWN = 3;
const RATIO = 2;
const HOLD = 7;
const HUNTERS = 4;
const NUMBERS = [3, 5, 7, 9];
const WIDTH = 4294967296;

const STEPS = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];

const GRIDS = [
  [
    "00000400000",
    "03222222230",
    "02020002020",
    "02222222220",
    "02020202020",
    "42022122024",
    "02020202020",
    "02222222220",
    "02020002020",
    "03222222230",
    "00000400000",
  ],
  [
    "00000400000",
    "03222222230",
    "02002020020",
    "02022222020",
    "02220202220",
    "42022122024",
    "02220202220",
    "02022222020",
    "02002020020",
    "03222222230",
    "00000400000",
  ],
  [
    "00000400000",
    "03222222230",
    "02020202020",
    "02222222220",
    "02020202020",
    "42222122224",
    "02020202020",
    "02222222220",
    "02020202020",
    "03222222230",
    "00000400000",
  ],
];

export const LAYOUTS = GRIDS.map((rows) => rows.flatMap((row) => [...row].map(Number)));

const found = (cells: readonly number[], kind: number) => cells.flatMap((one, at) => (one === kind ? [at] : []));

function ahead(at: number, dir: number): number {
  const x = (at % SIDE) + STEPS[dir][0];
  const y = Math.floor(at / SIDE) + STEPS[dir][1];
  return x < 0 || x >= SIDE || y < 0 || y >= SIDE ? -1 : y * SIDE + x;
}

function spawned(layout: number, level: number, given: number): State {
  const cells = LAYOUTS[layout];
  const [faces, a] = sprites(given, HUNTERS + 1);
  const [number, seed] = below(a, NUMBERS.length);
  return {
    layout,
    runner: found(cells, START)[0],
    hunters: found(cells, SPAWN),
    dots: found(cells, FLOOR),
    want: -1,
    moved: 0,
    hold: 0,
    ending: null,
    level,
    faces: { runner: faces[0], hunters: faces.slice(1), number: NUMBERS[number] },
    over: false,
    seed,
  };
}

function distances(cells: readonly number[], from: number): number[] {
  const far = Array<number>(cells.length).fill(Infinity);
  const queue = [from];
  far[from] = 0;
  for (let head = 0; head < queue.length; head++) {
    const at = queue[head];
    for (let dir = 0; dir < STEPS.length; dir++) {
      const to = ahead(at, dir);
      if (to < 0 || cells[to] === WALL || cells[to] === DOOR || far[to] !== Infinity) continue;
      far[to] = far[at] + 1;
      queue.push(to);
    }
  }
  return far;
}

function chased(state: State, cells: readonly number[], runner: number) {
  const far = distances(cells, runner);
  let seed = state.seed;
  const hunters = state.hunters.map((at) => {
    if (far[at] === Infinity) return at;
    const near = STEPS.map((_, dir) => ahead(at, dir)).filter((to) => to >= 0);
    const least = Math.min(...near.map((to) => far[to]));
    const best = near.filter((to) => far[to] === least);
    const [pick, next] = below(seed, best.length);
    seed = next;
    return best[pick];
  });
  return { hunters, seed };
}

function nextLevel(state: State): State {
  const others = LAYOUTS.map((_, at) => at).filter((at) => at !== state.layout);
  const [pick, seed] = below(state.seed, others.length);
  return spawned(others[pick], state.level + 1, seed);
}

function tick(state: State): State {
  if (state.over) return state;
  if (state.hold > 0) {
    const hold = state.hold - 1;
    if (hold > 0) return { ...state, hold };
    return state.ending === "caught" ? { ...state, hold, over: true } : nextLevel(state);
  }
  if (state.want < 0 && state.moved === 0) return state;
  const cells = LAYOUTS[state.layout];
  const to = state.want < 0 ? -1 : ahead(state.runner, state.want);
  const free = to >= 0 && cells[to] !== WALL && (cells[to] !== DOOR || state.dots.length === 0);
  const runner = free ? to : state.runner;
  const dots = state.dots.filter((at) => at !== runner);
  const moved = state.moved + 1;
  const walked = { ...state, runner, dots, want: -1, moved };
  if (cells[runner] === DOOR) return { ...walked, hold: HOLD, ending: "door" };
  if (state.hunters.includes(runner)) return { ...walked, hold: HOLD, ending: "caught" };
  if (moved % RATIO) return walked;
  const { hunters, seed } = chased(walked, cells, runner);
  const ending = hunters.includes(runner) ? "caught" : null;
  return { ...walked, hunters, seed, ending, hold: ending ? HOLD : 0 };
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  return state.over ? state : { ...state, want: act.args.dir };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => {
  const [layout, seed] = below(mix(Math.floor(rand() * WIDTH)), LAYOUTS.length);
  return spawned(layout, 1, seed);
};
