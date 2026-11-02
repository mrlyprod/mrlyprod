import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Answers = "off" | "wiki" | "oracle";

export type Over = "done" | null;

export type State = {
  pupils: number;
  answers: Answers;
  length: number;
  day: number;
  parent: number[];
  depth: number[];
  by: number[];
  first: number[];
  know: number[];
  target: number[];
  doing: number[];
  tries: number[];
  days: number[];
  patience: number[];
  asked: number;
  taught: number;
  invented: number;
  solved: number;
  lessons: number[];
  asks: number[];
  solves: number[];
  posed: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "pupils" | "answers" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  skill: number;
  asked: number;
  taught: number;
  invented: number;
  open: number;
  told: number;
  understood: number;
  puzzles: number;
  deepest: number;
  reach: number;
  solved: number;
};

export const WORLDS: Answers[] = ["off", "wiki", "oracle"];
export const DAYS = 120;
export const NONE = -1;
export const UNKNOWN = 0;
export const TOLD = 1;
export const UNDERSTOOD = 2;
export const IDLE = 0;
export const LEARN = 1;
export const INVENT = 2;
export const REVIEW = 3;
export const CAP = 160;
export const BRANCH = 3;
export const TENTHS = 10;
export const tick: Act = { verb: "tick", args: {} };

const CURRICULUM = 9;
const START = 3;
const BASE = 2;
const HARD = 1;
const SKILLED = 5;
const LUCK = 2;
const TEACH = 4;
const WAIT = 3;
const PUPILS = 64;
const HALF = 2;

/* READ */

export const knows = (state: Pick<State, "know" | "pupils">, puzzle: number, pupil: number) =>
  state.know[puzzle * state.pupils + pupil];

export const cost = (depth: number) => BASE + HARD * depth;

export const understood = (state: State, pupil: number) => {
  let n = 0;
  for (let p = 0; p < state.parent.length; p++) if (knows(state, p, pupil) === UNDERSTOOD) n++;
  return n;
};

const best = (state: State, puzzle: number) => {
  let top = UNKNOWN;
  for (let b = 0; b < state.pupils; b++) top = Math.max(top, knows(state, puzzle, b));
  return top;
};

export const level = (state: State) => state.parent.map((_, puzzle) => best(state, puzzle));

export function report(state: State): Report {
  const tops = level(state);
  let total = 0;
  for (let b = 0; b < state.pupils; b++) total += understood(state, b);
  return {
    skill: Math.floor((total * TENTHS) / state.pupils),
    asked: state.asked,
    taught: state.taught,
    invented: state.invented,
    open: tops.filter((one) => one === UNKNOWN).length,
    told: tops.filter((one) => one === TOLD).length,
    understood: tops.filter((one) => one === UNDERSTOOD).length,
    puzzles: state.parent.length,
    deepest: Math.max(0, ...state.depth.filter((_, p) => tops[p] === UNDERSTOOD)),
    reach: Math.max(0, ...state.depth.filter((_, p) => tops[p] !== UNKNOWN)),
    solved: state.solved,
  };
}

/* RULES */

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

const children = (world: State, puzzle: number) => world.parent.filter((one) => one === puzzle).length;

function grow(world: State, parent: number, by: number) {
  const puzzle = world.parent.length;
  world.parent.push(parent);
  world.depth.push(world.depth[parent] + 1);
  world.by.push(by);
  world.first.push(NONE);
  for (let b = 0; b < world.pupils; b++) world.know.push(UNKNOWN);
  return puzzle;
}

function learn(world: State, puzzle: number, pupil: number) {
  world.know[puzzle * world.pupils + pupil] = UNDERSTOOD;
  world.solves.push(pupil, puzzle);
  if (world.first[puzzle] !== NONE) return;
  world.first[puzzle] = world.day;
  world.solved++;
}

function choose(world: State, pupil: number) {
  const n = world.parent.length;
  const fresh: number[] = [];
  const seeds: number[] = [];
  const told: number[] = [];
  for (let p = 0; p < n; p++) {
    const mine = knows(world, p, pupil);
    if (mine === UNKNOWN && knows(world, world.parent[p], pupil) !== UNKNOWN) fresh.push(p);
    if (mine === UNDERSTOOD && n < CAP && children(world, p) < BRANCH) seeds.push(p);
    if (mine === TOLD) told.push(p);
  }
  const [doing, from] = fresh.length
    ? [LEARN, fresh]
    : seeds.length
      ? [INVENT, seeds]
      : told.length
        ? [REVIEW, told]
        : [IDLE, [0]];
  world.doing[pupil] = doing;
  world.target[pupil] = from[roll(world, from.length)];
  world.tries[pupil] = 0;
  world.days[pupil] = 0;
}

function teacher(world: State, puzzle: number, pupil: number) {
  const from = roll(world, world.pupils);
  for (let k = 0; k < world.pupils; k++) {
    const b = (from + k) % world.pupils;
    if (b !== pupil && knows(world, puzzle, b) === UNDERSTOOD) return b;
  }
  return NONE;
}

function work(world: State, pupil: number, skill: number) {
  const doing = world.doing[pupil];
  const puzzle = world.target[pupil];
  if (doing === IDLE) return choose(world, pupil);
  if (doing === LEARN && knows(world, puzzle, pupil) !== UNKNOWN) return choose(world, pupil);
  if (doing === INVENT && (world.parent.length >= CAP || children(world, puzzle) >= BRANCH))
    return choose(world, pupil);
  const stuck = world.days[pupil] >= world.patience[pupil];
  world.days[pupil]++;
  const machine =
    doing === LEARN && (world.answers === "oracle" || (world.answers === "wiki" && world.first[puzzle] !== NONE));
  if (stuck && machine) {
    world.know[puzzle * world.pupils + pupil] = TOLD;
    world.asked++;
    world.asks.push(pupil);
    return choose(world, pupil);
  }
  world.tries[pupil] += 1 + Math.floor(skill / SKILLED) + roll(world, LUCK);
  if (stuck && doing !== INVENT) {
    const b = teacher(world, puzzle, pupil);
    if (b !== NONE) {
      world.tries[pupil] += TEACH;
      world.taught++;
      world.lessons.push(b, pupil);
    }
  }
  if (doing === INVENT) {
    if (world.tries[pupil] < cost(world.depth[puzzle] + 1)) return;
    const made = grow(world, puzzle, pupil);
    world.invented++;
    world.posed.push(made);
    world.doing[pupil] = LEARN;
    world.target[pupil] = made;
    world.tries[pupil] = 0;
    world.days[pupil] = 0;
    return;
  }
  if (world.tries[pupil] < cost(world.depth[puzzle]) / (doing === REVIEW ? HALF : 1)) return;
  learn(world, puzzle, pupil);
  choose(world, pupil);
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    parent: state.parent.slice(),
    depth: state.depth.slice(),
    by: state.by.slice(),
    first: state.first.slice(),
    know: state.know.slice(),
    target: state.target.slice(),
    doing: state.doing.slice(),
    tries: state.tries.slice(),
    days: state.days.slice(),
    patience: state.patience.slice(),
    lessons: [],
    asks: [],
    solves: [],
    posed: [],
  };
}

export function boot({ pupils, answers, length, seed }: Config): State {
  if (!WORLDS.includes(answers)) throw new Error(`school: no world ${answers}`);
  if (pupils < 2 || pupils > PUPILS) throw new Error(`school: pupils run from 2 to ${PUPILS}`);
  const state: State = {
    pupils,
    answers,
    length,
    day: 0,
    parent: [0],
    depth: [0],
    by: [NONE],
    first: [0],
    know: Array(pupils).fill(UNDERSTOOD),
    target: Array(pupils).fill(0),
    doing: Array(pupils).fill(IDLE),
    tries: Array(pupils).fill(0),
    days: Array(pupils).fill(0),
    patience: [],
    asked: 0,
    taught: 0,
    invented: 0,
    solved: 0,
    lessons: [],
    asks: [],
    solves: [],
    posed: [],
    over: null,
    seed: mix(seed),
  };
  while (state.parent.length <= CURRICULUM) {
    const open = state.parent.flatMap((_, p) => (state.depth[p] < START && children(state, p) < BRANCH ? [p] : []));
    grow(state, open[roll(state, open.length)], NONE);
  }
  for (let b = 0; b < pupils; b++) state.patience.push(roll(state, WAIT));
  for (let b = 0; b < pupils; b++) choose(state, b);
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.day = state.day + 1;
  const skills = Array.from({ length: world.pupils }, (_, b) => understood(state, b));
  const from = roll(world, world.pupils);
  for (let k = 0; k < world.pupils; k++) {
    const b = (from + k) % world.pupils;
    work(world, b, skills[b]);
  }
  world.over = world.day >= state.length ? "done" : null;
  return world;
}

export function moves(state: State): Act[] {
  return state.over ? [] : [tick];
}

export function read(state: State, path = ""): unknown {
  let here: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (here === null || typeof here !== "object") return undefined;
    here = (here as Record<string, unknown>)[key];
  }
  return here;
}

export const run = (state: State, days: number) => {
  let here = state;
  for (let i = 0; i < days && !here.over; i++) here = step(here, tick);
  return here;
};

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  if (!round) return seed ?? 0;
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal({ pupils, answers }: Pick<Config, "pupils" | "answers">, { seed, round = 0 }: Table): State {
  return boot({ pupils, answers, length: DAYS, seed: dealt(seed, round) });
}
