import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type Choose = "first" | "near" | "most";

export type State = {
  choose: Choose;
  bots: number;
  length: number;
  hour: number;
  x: number[];
  y: number[];
  gx: number[];
  gy: number[];
  can: number[];
  like: number[];
  wish: number[];
  task: number[];
  kind: number[];
  by: number[];
  born: number[];
  hearts: number[];
  taker: number[];
  end: number[];
  granted: number;
  gone: number;
  waited: number;
  posted: number[];
  given: number[];
  dropped: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "choose" | "bots" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  granted: number;
  waiting: number;
  oldest: number;
  nobody: number;
  gone: number;
  wait: number;
};

export const CHOICES: Choose[] = ["first", "near", "most"];
export const MAP = 1000;
export const KINDS = 6;
export const MOON = 5;
export const HOURS = 240;
export const NONE = -1;
export const OPEN = 0;
export const GRANTED = 1;
export const LET = 2;
export const SPEED = 45;
export const PATIENCE = 60;
export const MILLE = 1000;
export const tick: Act = { verb: "tick", args: {} };

const STROLL = 14;
const POST = 9;
const HEART = 5;
const SKY = 18;
const EDGE = 70;
const FEW = 4;
const MOST = 64;

/* RULES */

const far = (ax: number, ay: number, bx: number, by: number) => (ax - bx) ** 2 + (ay - by) ** 2;

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

const spot = (world: State) => EDGE + roll(world, MAP - EDGE * 2);

export const able = (state: Pick<State, "can">, kind: number) => state.can.includes(kind);

function walk(world: State, bot: number, x: number, y: number, speed: number) {
  const dx = x - world.x[bot];
  const dy = y - world.y[bot];
  const d = Math.sqrt(dx * dx + dy * dy);
  if (d <= speed) {
    world.x[bot] = x;
    world.y[bot] = y;
    return true;
  }
  world.x[bot] += Math.trunc((dx * speed) / d);
  world.y[bot] += Math.trunc((dy * speed) / d);
  return false;
}

function drop(world: State) {
  for (let w = 0; w < world.kind.length; w++) {
    if (world.end[w] !== OPEN || world.taker[w] !== NONE || world.hour - world.born[w] < PATIENCE) continue;
    world.end[w] = LET;
    world.wish[world.by[w]] = NONE;
    world.gone++;
    world.dropped.push(w);
  }
}

function post(world: State) {
  for (let bot = 0; bot < world.bots; bot++) {
    if (world.wish[bot] !== NONE || world.task[bot] !== NONE || roll(world, POST)) continue;
    const r = roll(world, SKY);
    const w = world.kind.length;
    world.kind.push(r ? r % MOON : MOON);
    world.by.push(bot);
    world.born.push(world.hour);
    world.hearts.push(0);
    world.taker.push(NONE);
    world.end.push(OPEN);
    world.wish[bot] = w;
    world.posted.push(w);
  }
}

function heart(world: State) {
  for (let bot = 0; bot < world.bots; bot++) {
    if (roll(world, HEART)) continue;
    const mine: number[] = [];
    for (let w = 0; w < world.kind.length; w++)
      if (world.end[w] === OPEN && world.by[w] !== bot && world.kind[w] === world.like[bot]) mine.push(w);
    if (mine.length) world.hearts[mine[roll(world, mine.length)]]++;
  }
}

export function better(state: State, bot: number, a: number, b: number) {
  if (b === NONE) return true;
  if (state.choose === "near") {
    const da = far(state.x[bot], state.y[bot], state.x[state.by[a]], state.y[state.by[a]]);
    const db = far(state.x[bot], state.y[bot], state.x[state.by[b]], state.y[state.by[b]]);
    if (da !== db) return da < db;
  }
  if (state.choose === "most" && state.hearts[a] !== state.hearts[b]) return state.hearts[a] > state.hearts[b];
  return state.born[a] < state.born[b] || (state.born[a] === state.born[b] && a < b);
}

function assign(world: State) {
  for (let k = 0; k < world.bots; k++) {
    const bot = (k + world.hour) % world.bots;
    if (world.task[bot] !== NONE || world.wish[bot] !== NONE) continue;
    let best = NONE;
    for (let w = 0; w < world.kind.length; w++)
      if (
        world.end[w] === OPEN &&
        world.taker[w] === NONE &&
        world.kind[w] === world.can[bot] &&
        world.by[w] !== bot &&
        better(world, bot, w, best)
      )
        best = w;
    if (best === NONE) continue;
    world.taker[best] = bot;
    world.task[bot] = best;
  }
}

function move(world: State) {
  for (let bot = 0; bot < world.bots; bot++) {
    const w = world.task[bot];
    if (w !== NONE) {
      const to = world.by[w];
      if (!walk(world, bot, world.x[to], world.y[to], SPEED)) continue;
      world.end[w] = GRANTED;
      world.wish[to] = NONE;
      world.task[bot] = NONE;
      world.granted++;
      world.waited += world.hour - world.born[w];
      world.given.push(w);
      continue;
    }
    if (world.wish[bot] !== NONE) continue;
    if (walk(world, bot, world.gx[bot], world.gy[bot], STROLL)) {
      world.gx[bot] = spot(world);
      world.gy[bot] = spot(world);
    }
  }
}

/* REPORT */

export function report(state: State): Report {
  let waiting = 0;
  let oldest = 0;
  let nobody = 0;
  for (let w = 0; w < state.kind.length; w++) {
    if (state.end[w] !== OPEN) continue;
    waiting++;
    oldest = Math.max(oldest, state.hour - state.born[w]);
    if (!able(state, state.kind[w])) nobody++;
  }
  const wait = state.granted ? Math.floor((state.waited * MILLE) / state.granted) : 0;
  return { granted: state.granted, waiting, oldest, nobody, gone: state.gone, wait };
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    x: state.x.slice(),
    y: state.y.slice(),
    gx: state.gx.slice(),
    gy: state.gy.slice(),
    wish: state.wish.slice(),
    task: state.task.slice(),
    kind: state.kind.slice(),
    by: state.by.slice(),
    born: state.born.slice(),
    hearts: state.hearts.slice(),
    taker: state.taker.slice(),
    end: state.end.slice(),
    posted: [],
    given: [],
    dropped: [],
  };
}

export function boot({ choose, bots, length, seed }: Config): State {
  if (!CHOICES.includes(choose)) throw new Error(`wish: choose is one of ${CHOICES.join(", ")}`);
  if (bots < FEW || bots > MOST) throw new Error(`wish: bots run from ${FEW} to ${MOST}`);
  const state: State = {
    choose,
    bots,
    length,
    hour: 0,
    x: [],
    y: [],
    gx: [],
    gy: [],
    can: [],
    like: [],
    wish: Array(bots).fill(NONE),
    task: Array(bots).fill(NONE),
    kind: [],
    by: [],
    born: [],
    hearts: [],
    taker: [],
    end: [],
    granted: 0,
    gone: 0,
    waited: 0,
    posted: [],
    given: [],
    dropped: [],
    over: null,
    seed: mix(seed),
  };
  for (let bot = 0; bot < bots; bot++) {
    state.x.push(spot(state));
    state.y.push(spot(state));
    state.gx.push(spot(state));
    state.gy.push(spot(state));
    state.can.push(roll(state, MOON));
    state.like.push(Math.min(roll(state, MOON), roll(state, MOON)));
  }
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.hour = state.hour + 1;
  drop(world);
  post(world);
  heart(world);
  assign(world);
  move(world);
  world.over = world.hour >= state.length ? "done" : null;
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

export const run = (state: State, hours: number) => {
  let here = state;
  for (let i = 0; i < hours && !here.over; i++) here = step(here, tick);
  return here;
};

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  if (!round) return seed ?? 0;
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal({ choose, bots }: Pick<Config, "choose" | "bots">, { seed, round = 0 }: Table): State {
  return boot({ choose, bots, length: HOURS, seed: dealt(seed, round) });
}
