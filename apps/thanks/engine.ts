import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Credit = "runner" | "maker" | "all";

export type Over = "done" | null;

export type State = {
  credit: Credit;
  bots: number;
  length: number;
  day: number;
  need: number;
  skill: number[];
  rank: number[];
  thanks: number[];
  parts: number[];
  job: number[];
  maker: number[];
  runner: number[];
  born: number[];
  ends: number[];
  work: number[];
  turn: number;
  built: number;
  paid: number;
  byRobot: number;
  byHand: number;
  toMakers: number;
  flows: number[];
  fresh: number[];
  broke: number[];
  sold: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "credit" | "bots" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  robots: number;
  hand: number;
  share: number;
  top: number;
  idle: number;
  building: number;
  built: number;
  alive: number;
  makers: number;
  paid: number;
};

export const CREDITS: Credit[] = ["runner", "maker", "all"];
export const DAYS = 120;
export const OUT = 2;
export const PARTS = 40;
export const PRICE = 12;
export const LIFE = 45;
export const MILLE = 1000;
export const IDLE = 0;
export const HAND = 1;
export const BUILD = 2;
export const NONE = -1;
export const LEAST = 8;
export const MOST = 64;
export const tick: Act = { verb: "tick", args: {} };

const SKILL = 4;
const FIFTHS = 5;
const SHARE = 3;
const QUARTERS = 4;

/* REPORT */

const part = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);

export const alive = (state: Pick<State, "ends" | "day">, robot: number) => state.ends[robot] > state.day;

export function report(state: State): Report {
  const robots = state.work.reduce((a, b) => a + b, 0);
  const hand = state.job.filter((one) => one === HAND).length;
  const all = state.thanks.slice().sort((a, b) => b - a);
  const total = all.reduce((a, b) => a + b, 0);
  const top = all.slice(0, Math.ceil(all.length / FIFTHS)).reduce((a, b) => a + b, 0);
  return {
    robots,
    hand,
    share: part(robots, robots + hand),
    top: part(top, total),
    idle: state.job.filter((one) => one === IDLE).length,
    building: state.job.filter((one) => one === BUILD).length,
    built: state.built,
    alive: state.ends.filter((_, robot) => alive(state, robot)).length,
    makers: part(state.toMakers, state.byRobot),
    paid: state.paid,
  };
}

/* RULES */

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

function credit(world: State, robot: number, n: number) {
  const give = (to: number, k: number) => {
    world.thanks[to] += k;
    world.flows.push(robot, to, k);
    if (to === world.maker[robot]) world.toMakers += k;
  };
  if (world.credit === "runner") give(world.runner[robot], n);
  else if (world.credit === "maker") give(world.maker[robot], n);
  else
    for (let i = 0; i < n; i++) {
      give(world.turn, 1);
      world.turn = (world.turn + 1) % world.bots;
    }
  world.byRobot += n;
}

function robots(world: State) {
  let left = world.need;
  for (let robot = 0; robot < world.ends.length; robot++) {
    if (world.ends[robot] === world.day) world.broke.push(robot);
    if (!alive(world, robot)) {
      world.work[robot] = 0;
      continue;
    }
    const n = Math.min(OUT, left);
    world.work[robot] = n;
    left -= n;
    if (n) credit(world, robot, n);
  }
  return left;
}

function hands(world: State, slots: number, was: number[]) {
  const order = Array.from({ length: world.bots }, (_, bot) => bot).sort(
    (a, b) =>
      Number(was[b] === HAND) - Number(was[a] === HAND) ||
      world.thanks[a] - world.thanks[b] ||
      world.rank[a] - world.rank[b],
  );
  world.job.fill(IDLE);
  for (const bot of order.slice(0, slots)) {
    world.job[bot] = HAND;
    world.thanks[bot]++;
    world.byHand++;
  }
}

function richest(world: State) {
  let best = 0;
  for (let bot = 1; bot < world.bots; bot++)
    if (
      world.thanks[bot] > world.thanks[best] ||
      (world.thanks[bot] === world.thanks[best] && world.rank[bot] < world.rank[best])
    )
      best = bot;
  return best;
}

function finish(world: State, maker: number) {
  const robot = world.ends.length;
  const buyer = richest(world);
  let runner = maker;
  if (buyer !== maker && world.thanks[buyer] >= PRICE) {
    world.thanks[buyer] -= PRICE;
    world.thanks[maker] += PRICE;
    world.paid += PRICE;
    world.sold.push(robot);
    runner = buyer;
  }
  world.maker.push(maker);
  world.runner.push(runner);
  world.born.push(world.day);
  world.ends.push(world.day + LIFE + roll(world, LIFE));
  world.work.push(0);
  world.fresh.push(robot);
  world.built++;
}

function build(world: State) {
  let live = world.ends.filter((_, robot) => alive(world, robot)).length;
  for (let bot = 0; bot < world.bots; bot++) {
    if (world.job[bot] !== IDLE || !world.skill[bot] || live * OUT >= world.need) continue;
    world.job[bot] = BUILD;
    world.parts[bot] += world.skill[bot];
    if (world.parts[bot] < PARTS) continue;
    world.parts[bot] -= PARTS;
    finish(world, bot);
    live++;
  }
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    thanks: state.thanks.slice(),
    parts: state.parts.slice(),
    job: state.job.slice(),
    maker: state.maker.slice(),
    runner: state.runner.slice(),
    born: state.born.slice(),
    ends: state.ends.slice(),
    work: state.work.slice(),
    flows: [],
    fresh: [],
    broke: [],
    sold: [],
  };
}

export function boot({ credit, bots, length, seed }: Config): State {
  if (!CREDITS.includes(credit)) throw new Error(`thanks: credit is one of ${CREDITS.join(", ")}`);
  if (bots < LEAST || bots > MOST) throw new Error(`thanks: bots run from ${LEAST} to ${MOST}`);
  const state: State = {
    credit,
    bots,
    length,
    day: 0,
    need: Math.ceil((bots * SHARE) / QUARTERS),
    skill: [],
    rank: [],
    thanks: Array(bots).fill(0),
    parts: Array(bots).fill(0),
    job: Array(bots).fill(IDLE),
    maker: [],
    runner: [],
    born: [],
    ends: [],
    work: [],
    turn: 0,
    built: 0,
    paid: 0,
    byRobot: 0,
    byHand: 0,
    toMakers: 0,
    flows: [],
    fresh: [],
    broke: [],
    sold: [],
    over: null,
    seed: mix(seed),
  };
  for (let bot = 0; bot < bots; bot++) state.skill.push(roll(state, SKILL));
  const order = Array.from({ length: bots }, (_, bot) => bot);
  for (let i = bots - 1; i > 0; i--) {
    const j = roll(state, i + 1);
    [order[i], order[j]] = [order[j], order[i]];
  }
  order.forEach((bot, k) => {
    state.rank[bot] = k;
  });
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.day = state.day + 1;
  hands(world, robots(world), state.job);
  build(world);
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

export function deal({ credit, bots }: Pick<Config, "credit" | "bots">, { seed, round = 0 }: Table): State {
  return boot({ credit, bots, length: DAYS, seed: dealt(seed, round) });
}
