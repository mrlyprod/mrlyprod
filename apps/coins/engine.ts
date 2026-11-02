import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type State = {
  currencies: number;
  fee: number;
  traders: number;
  length: number;
  day: number;
  tx: number[];
  ty: number[];
  region: number[];
  capital: number[];
  bx: number[];
  by: number[];
  till: number[];
  rate: number[];
  purse: number[];
  home: number[];
  x: number[];
  y: number[];
  at: number[];
  goal: number[];
  buy: number[];
  booth: number[];
  wait: number[];
  coin: number[];
  wallet: number[];
  traded: number;
  failed: number;
  waited: number;
  lost: number;
  money: number;
  changed: number[];
  sold: number[];
  gave: number[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "currencies" | "fee" | "traders" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  traded: number;
  failed: number;
  stuck: number;
  lost: number;
  pace: number;
  spread: number;
  booths: number;
};

export const MAX = 5;
export const MAP = 1000;
export const SIDE = 4;
export const TOWNS = SIDE * SIDE;
export const DAYS = 120;
export const PRICE = 20;
export const PAR = 1000;
export const MILLE = 1000;
export const NONE = -1;
export const SPEED = 70;
export const PATIENCE = 12;
export const tick: Act = { verb: "tick", args: {} };

const START = 100;
const BOOTH = 40;
const DRIFT = 15;
const LOW = 600;
const HIGH = 1600;
const ROOM = 45;
const JITTER = 160;
const FIFTHS = 5;
const MOST = 128;
const CROSS = 64;
const APART = 110;

/* MAP */

const far = (ax: number, ay: number, bx: number, by: number) => (ax - bx) ** 2 + (ay - by) ** 2;

export const pair = (a: number, b: number) => (a < b ? a * MAX + b : b * MAX + a);

export const pairs = (currencies: number) => {
  const out: number[] = [];
  for (let a = 0; a < currencies; a++) for (let b = a + 1; b < currencies; b++) out.push(pair(a, b));
  return out;
};

export function nearest(state: Pick<State, "tx" | "ty">, x: number, y: number) {
  let best = 0;
  for (let town = 1; town < state.tx.length; town++)
    if (far(x, y, state.tx[town], state.ty[town]) < far(x, y, state.tx[best], state.ty[best])) best = town;
  return best;
}

function border(state: State, a: number, b: number): [number, number] {
  const near: [number, number, number][] = [];
  for (let u = 0; u < TOWNS; u++)
    for (let w = 0; w < TOWNS; w++)
      if (state.region[u] === a && state.region[w] === b)
        near.push([far(state.tx[u], state.ty[u], state.tx[w], state.ty[w]), u, w]);
  near.sort((p, q) => p[0] - q[0] || p[1] - q[1] || p[2] - q[2]);
  const cross = ([, u, w]: [number, number, number]): [number, number] => {
    const at = (k: number): [number, number] => [
      Math.floor(state.tx[u] + ((state.tx[w] - state.tx[u]) * k) / CROSS),
      Math.floor(state.ty[u] + ((state.ty[w] - state.ty[u]) * k) / CROSS),
    ];
    let k = 1;
    while (k < CROSS && state.region[nearest(state, ...at(k))] !== b) k++;
    return at(k - 0.5);
  };
  const placed = pairs(state.currencies).filter((p) => state.bx[p] !== NONE);
  const spots = near.map(cross);
  const room = ([x, y]: [number, number]) =>
    Math.min(APART * APART, ...placed.map((p) => far(x, y, state.bx[p], state.by[p])));
  return spots.reduce((best, one) => (room(one) > room(best) ? one : best));
}

/* REPORT */

export const value = (state: Pick<State, "wallet" | "rate">, trader: number) => {
  let sum = 0;
  for (let c = 0; c < MAX; c++) sum += state.wallet[trader * MAX + c] * state.rate[c];
  return sum;
};

const share = (n: number, of: number) => (of ? Math.floor((n * MILLE) / of) : 0);

export function report(state: State): Report {
  const all = state.home.map((_, trader) => value(state, trader)).sort((a, b) => b - a);
  const total = all.reduce((a, b) => a + b, 0);
  const top = all.slice(0, Math.ceil(all.length / FIFTHS)).reduce((a, b) => a + b, 0);
  return {
    traded: state.traded,
    failed: state.failed,
    stuck: state.wait.filter((one) => one > 0).length,
    lost: share(state.lost, state.money),
    pace: share(state.traders * state.day, state.traded),
    spread: share(top, total),
    booths: pairs(state.currencies).length,
  };
}

/* RULES */

const roll = (world: State, n: number) => {
  world.seed = xorshift(world.seed);
  return world.seed % n;
};

export const cost = (state: Pick<State, "rate" | "fee">, from: number, to: number) => {
  const base = Math.ceil((PRICE * state.rate[to]) / state.rate[from]);
  return { base, need: Math.ceil((PRICE * state.rate[to] * (100 + state.fee)) / (state.rate[from] * 100)) };
};

function drift(world: State) {
  if (world.currencies < 2) return;
  for (let c = 0; c < world.currencies; c++)
    world.rate[c] = Math.max(LOW, Math.min(HIGH, world.rate[c] + roll(world, DRIFT * 2 + 1) - DRIFT));
}

function wages(world: State) {
  for (let town = 0; town < TOWNS; town++) {
    if (!world.purse[town]) continue;
    const mine = world.home.flatMap((one, trader) => (one === town ? [trader] : []));
    if (!mine.length) continue;
    const each = Math.floor(world.purse[town] / mine.length);
    const c = world.region[town];
    for (const trader of mine) world.wallet[trader * MAX + c] += each;
    world.purse[town] -= each * mine.length;
  }
}

function plan(world: State, trader: number) {
  const from = world.at[trader];
  let goal = roll(world, TOWNS - 1);
  if (from !== NONE && goal >= from) goal++;
  const want = world.region[goal];
  const purse = trader * MAX;
  if (world.wallet[purse + want] >= PRICE) {
    world.goal[trader] = goal;
    world.buy[trader] = 1;
    world.booth[trader] = NONE;
    world.coin[trader] = want;
    return;
  }
  let best = NONE;
  for (let c = 0; c < world.currencies; c++) {
    if (c === want || world.wallet[purse + c] < cost(world, c, want).need) continue;
    if (best === NONE || world.wallet[purse + c] * world.rate[c] > world.wallet[purse + best] * world.rate[best])
      best = c;
  }
  if (best === NONE) return;
  world.goal[trader] = goal;
  world.buy[trader] = 1;
  world.booth[trader] = pair(best, want);
  world.coin[trader] = best;
}

function walk(world: State, trader: number, x: number, y: number) {
  const dx = x - world.x[trader];
  const dy = y - world.y[trader];
  const d = Math.sqrt(dx * dx + dy * dy);
  if (d <= SPEED) {
    world.x[trader] = x;
    world.y[trader] = y;
    return true;
  }
  world.x[trader] += Math.trunc((dx * SPEED) / d);
  world.y[trader] += Math.trunc((dy * SPEED) / d);
  return false;
}

function change(world: State, trader: number) {
  const p = world.booth[trader];
  const want = world.region[world.goal[trader]];
  const from = world.coin[trader];
  const { base, need } = cost(world, from, want);
  const purse = trader * MAX;
  const side = (c: number) => p * 2 + (c === Math.floor(p / MAX) ? 0 : 1);
  if (world.till[side(want)] >= PRICE && world.wallet[purse + from] >= need) {
    world.till[side(want)] -= PRICE;
    world.till[side(from)] += base;
    world.wallet[purse + from] -= need;
    world.wallet[purse + want] += PRICE;
    world.lost += (need - base) * world.rate[from];
    world.booth[trader] = NONE;
    world.coin[trader] = want;
    world.wait[trader] = 0;
    world.changed.push(p);
    return;
  }
  world.wait[trader]++;
  world.waited++;
  if (world.wait[trader] <= PATIENCE) return;
  world.wait[trader] = 0;
  world.failed++;
  world.gave.push(trader);
  world.goal[trader] = world.home[trader];
  world.buy[trader] = 0;
  world.booth[trader] = NONE;
}

function arrive(world: State, trader: number) {
  const goal = world.goal[trader];
  world.at[trader] = goal;
  world.goal[trader] = NONE;
  if (!world.buy[trader]) return;
  const c = world.region[goal];
  world.wallet[trader * MAX + c] -= PRICE;
  world.purse[goal] += PRICE;
  world.traded++;
  world.sold.push(goal);
}

function trade(world: State) {
  for (let trader = 0; trader < world.traders; trader++) {
    if (world.goal[trader] === NONE) {
      plan(world, trader);
      continue;
    }
    world.at[trader] = NONE;
    const p = world.booth[trader];
    if (p !== NONE) {
      if (walk(world, trader, world.bx[p], world.by[p])) change(world, trader);
      continue;
    }
    const goal = world.goal[trader];
    if (walk(world, trader, world.tx[goal], world.ty[goal])) arrive(world, trader);
  }
}

/* REDUCER */

function copy(state: State): State {
  return {
    ...state,
    till: state.till.slice(),
    rate: state.rate.slice(),
    purse: state.purse.slice(),
    x: state.x.slice(),
    y: state.y.slice(),
    at: state.at.slice(),
    goal: state.goal.slice(),
    buy: state.buy.slice(),
    booth: state.booth.slice(),
    wait: state.wait.slice(),
    coin: state.coin.slice(),
    wallet: state.wallet.slice(),
    changed: [],
    sold: [],
    gave: [],
  };
}

export function boot({ currencies, fee, traders, length, seed }: Config): State {
  if (currencies < 1 || currencies > MAX) throw new Error(`coins: currencies run from 1 to ${MAX}`);
  if (traders < 2 || traders > MOST) throw new Error(`coins: traders run from 2 to ${MOST}`);
  if (fee < 0 || fee > 100) throw new Error("coins: the fee runs from 0 to 100");
  const cell = MAP / SIDE;
  const state: State = {
    currencies,
    fee,
    traders,
    length,
    day: 0,
    tx: [],
    ty: [],
    region: [],
    capital: [],
    bx: Array(MAX * MAX).fill(NONE),
    by: Array(MAX * MAX).fill(NONE),
    till: Array(MAX * MAX * 2).fill(0),
    rate: Array(MAX).fill(PAR),
    purse: Array(TOWNS).fill(0),
    home: [],
    x: [],
    y: [],
    at: [],
    goal: Array(traders).fill(NONE),
    buy: Array(traders).fill(0),
    booth: Array(traders).fill(NONE),
    wait: Array(traders).fill(0),
    coin: [],
    wallet: Array(traders * MAX).fill(0),
    traded: 0,
    failed: 0,
    waited: 0,
    lost: 0,
    money: traders * START * PAR,
    changed: [],
    sold: [],
    gave: [],
    over: null,
    seed: mix(seed),
  };
  for (let town = 0; town < TOWNS; town++) {
    state.tx.push((town % SIDE) * cell + ROOM + roll(state, JITTER));
    state.ty.push(Math.floor(town / SIDE) * cell + ROOM + roll(state, JITTER));
  }
  state.capital.push(roll(state, TOWNS));
  while (state.capital.length < MAX) {
    let best = NONE;
    let reach = -1;
    for (let town = 0; town < TOWNS; town++) {
      const d = Math.min(...state.capital.map((c) => far(state.tx[town], state.ty[town], state.tx[c], state.ty[c])));
      if (d > reach) {
        reach = d;
        best = town;
      }
    }
    state.capital.push(best);
  }
  for (let town = 0; town < TOWNS; town++) {
    let best = 0;
    for (let c = 1; c < currencies; c++) {
      const [a, b] = [state.capital[c], state.capital[best]];
      if (
        far(state.tx[town], state.ty[town], state.tx[a], state.ty[a]) <
        far(state.tx[town], state.ty[town], state.tx[b], state.ty[b])
      )
        best = c;
    }
    state.region.push(best);
  }
  for (const p of pairs(currencies)) {
    [state.bx[p], state.by[p]] = border(state, Math.floor(p / MAX), p % MAX);
    state.till[p * 2] = BOOTH;
    state.till[p * 2 + 1] = BOOTH;
  }
  const order = Array.from({ length: TOWNS }, (_, town) => town);
  for (let i = TOWNS - 1; i > 0; i--) {
    const j = roll(state, i + 1);
    [order[i], order[j]] = [order[j], order[i]];
  }
  for (let trader = 0; trader < traders; trader++) {
    const home = order[trader % TOWNS];
    state.home.push(home);
    state.x.push(state.tx[home]);
    state.y.push(state.ty[home]);
    state.at.push(home);
    state.coin.push(state.region[home]);
    state.wallet[trader * MAX + state.region[home]] = START;
  }
  return state;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const world = copy(state);
  world.day = state.day + 1;
  drift(world);
  wages(world);
  trade(world);
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

export function deal(
  { currencies, fee, traders }: Pick<Config, "currencies" | "fee" | "traders">,
  { seed, round = 0 }: Table,
): State {
  return boot({ currencies, fee, traders, length: DAYS, seed: dealt(seed, round) });
}
