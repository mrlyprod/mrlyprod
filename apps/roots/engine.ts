import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Inherit = "heir" | "commons" | "none";

export type Over = "done" | null;

export type State = {
  families: number;
  inherit: Inherit;
  length: number;
  year: number;
  age: number[];
  span: number[];
  skill: number[];
  coins: number[];
  tiles: number[];
  land: number[];
  pool: number;
  gen: number[];
  fifth: number[];
  ends: number[];
  faded: number[];
  bought: number[];
  births: number;
  empty: number;
  moved: number;
  pairs: number;
  work: number;
  rent: number;
  shared: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "families" | "inherit" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  top: number;
  mobility: number;
  pairs: number;
  made: number;
  rented: number;
  shared: number;
  nothing: number;
  commons: number;
  generations: number;
  richest: number;
  poorest: number;
};

export const RULES: Inherit[] = ["heir", "commons", "none"];
export const YEARS = 200;
export const ADULT = 18;
export const PRICE = 60;
export const LAND = 3;
export const MILLE = 1000;
export const NONE = -1;
export const COMMONS = -2;
export const FIFTHS = 5;
export const tick: Act = { verb: "tick", args: {} };

const SPAN = 42;
const SPREAD = 21;
const YIELD = 4;
const LIVING = 4;
const SPEND = 10;
const LUCK = 5;
const BASE = 3;
const TEACH = 3;
const LEARN = 6;
const TOP = 9;
const FAMILIES = 64;

/* REPORT */

export const wealth = (state: Pick<State, "coins" | "tiles">, family: number) =>
  state.coins[family] + state.tiles[family] * PRICE;

export const held = (state: Pick<State, "land" | "pool">) =>
  state.pool + state.land.filter((one) => one === COMMONS).length * PRICE;

const share = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);

const adults = (state: Pick<State, "age">) => state.age.flatMap((one, family) => (one >= ADULT ? [family] : []));

export function report(state: State): Report {
  const all = adults(state)
    .map((family) => wealth(state, family))
    .sort((a, b) => b - a);
  const total = all.reduce((a, b) => a + b, 0);
  const top = all.slice(0, Math.ceil(all.length / FIFTHS)).reduce((a, b) => a + b, 0);
  const income = state.work + state.rent + state.shared;
  const owned = state.coins.reduce((a, b) => a + b, 0) + state.tiles.reduce((a, b) => a + b, 0) * PRICE;
  return {
    top: share(top, total),
    mobility: share(state.moved, state.pairs),
    pairs: state.pairs,
    made: share(state.work, income),
    rented: share(state.rent, income),
    shared: share(state.shared, income),
    nothing: share(state.empty, state.births),
    commons: share(held(state), owned + held(state)),
    generations: Math.max(...state.gen),
    richest: all[0] ?? 0,
    poorest: all[all.length - 1] ?? 0,
  };
}

/* RULES */

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

function fade(world: State, family: number) {
  const coins = world.coins[family];
  const mine = wealth(world, family);
  const rank = Math.floor((world.ends.filter((one) => one > mine).length * FIFTHS) / (world.ends.length + 1));
  if (world.fifth[family] !== NONE) {
    world.pairs++;
    if (world.fifth[family] !== rank) world.moved++;
  }
  world.fifth[family] = world.ends.length * 2 >= world.families ? rank : NONE;
  world.ends = [...world.ends, mine].slice(-world.families);
  world.faded.push(family);
  for (let i = 0; i < world.land.length; i++) {
    if (world.land[i] !== family) continue;
    if (world.inherit === "commons") world.land[i] = COMMONS;
    else if (world.inherit === "none") world.land[i] = NONE;
  }
  if (world.inherit === "commons") world.pool += coins;
  if (world.inherit !== "heir") {
    world.coins[family] = 0;
    world.tiles[family] = 0;
  }
  world.births++;
  if (world.coins[family] + world.tiles[family] === 0) world.empty++;
  world.age[family] = 0;
  world.span[family] = SPAN + roll(world, SPREAD);
  world.skill[family] = Math.min(TOP, 1 + roll(world, BASE) + Math.floor(world.skill[family] / TEACH));
  world.gen[family]++;
}

function live(world: State) {
  const n = world.families;
  for (let family = 0; family < n; family++) {
    if (world.age[family] < ADULT) continue;
    const made = world.skill[family] + roll(world, LUCK);
    world.coins[family] += made;
    world.work += made;
  }
  for (const owner of world.land) {
    if (owner >= 0) {
      world.coins[owner] += YIELD;
      world.rent += YIELD;
    } else if (owner === COMMONS) world.pool += YIELD;
  }
  const grown = adults(world);
  if (grown.length && world.pool >= grown.length) {
    const each = Math.floor(world.pool / grown.length);
    for (const family of grown) world.coins[family] += each;
    world.pool -= each * grown.length;
    world.shared += each * grown.length;
  }
  for (const family of grown)
    world.coins[family] = Math.max(0, world.coins[family] - LIVING - Math.floor((world.coins[family] * SPEND) / 100));
  for (let tile = world.land.indexOf(NONE); tile !== NONE; tile = world.land.indexOf(NONE)) {
    let buyer = NONE;
    for (const family of grown)
      if (world.coins[family] >= PRICE && (buyer === NONE || world.coins[family] > world.coins[buyer])) buyer = family;
    if (buyer === NONE) break;
    world.land[tile] = buyer;
    world.tiles[buyer]++;
    world.coins[buyer] -= PRICE;
    world.bought.push(tile);
  }
}

function grow(world: State) {
  for (let family = 0; family < world.families; family++) {
    world.age[family]++;
    if (world.age[family] < ADULT + 12 && world.age[family] % LEARN === 0)
      world.skill[family] = Math.min(TOP, world.skill[family] + 1);
    if (world.age[family] >= world.span[family]) fade(world, family);
  }
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    age: state.age.slice(),
    span: state.span.slice(),
    skill: state.skill.slice(),
    coins: state.coins.slice(),
    tiles: state.tiles.slice(),
    land: state.land.slice(),
    gen: state.gen.slice(),
    fifth: state.fifth.slice(),
    faded: [],
    bought: [],
  };
}

export function boot({ families, inherit, length, seed }: Config): State {
  if (!RULES.includes(inherit)) throw new Error(`roots: no rule ${inherit}`);
  if (families < 2 || families > FAMILIES) throw new Error(`roots: families run from 2 to ${FAMILIES}`);
  const state: State = {
    families,
    inherit,
    length,
    year: 0,
    age: [],
    span: [],
    skill: [],
    coins: Array(families).fill(0),
    tiles: Array(families).fill(0),
    land: Array(families * LAND).fill(NONE),
    pool: 0,
    gen: Array(families).fill(1),
    fifth: Array(families).fill(NONE),
    ends: [],
    faded: [],
    bought: [],
    births: 0,
    empty: 0,
    moved: 0,
    pairs: 0,
    work: 0,
    rent: 0,
    shared: 0,
    over: null,
    seed: mix(seed),
  };
  for (let family = 0; family < families; family++) {
    state.span.push(SPAN + roll(state, SPREAD));
    state.age.push(roll(state, state.span[family]));
    state.skill.push(1 + roll(state, BASE) + Math.min(TOP - BASE, Math.floor(state.age[family] / LEARN)));
  }
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.year = state.year + 1;
  live(world);
  grow(world);
  world.over = world.year >= state.length ? "done" : null;
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

export const run = (state: State, years: number) => {
  let here = state;
  for (let i = 0; i < years && !here.over; i++) here = step(here, tick);
  return here;
};

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  if (!round) return seed ?? 0;
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal({ families, inherit }: Pick<Config, "families" | "inherit">, { seed, round = 0 }: Table): State {
  return boot({ families, inherit, length: YEARS, seed: dealt(seed, round) });
}
