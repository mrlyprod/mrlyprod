import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Economy = "money" | "gift" | "pool";

export type How = "make" | "use" | "give" | "buy" | "share" | "draw" | "spill" | "miss";

export type Flow = { how: How; from: number; to: number; kind: number; coin: number };

export type Over = "done" | null;

export type State = {
  bots: number;
  economy: Economy;
  plenty: number;
  generous: number;
  purse: number;
  length: number;
  tick: number;
  crafts: number[];
  stock: number[];
  coins: number[];
  store: number[];
  needs: number[];
  fed: number[];
  missed: number[];
  made: number;
  posted: number;
  spilled: number;
  flows: Flow[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "bots" | "economy" | "plenty" | "generous" | "purse" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = {
  fed: number;
  idle: number;
  top: number;
  fair: number;
  made: number;
  used: number;
  spilled: number;
  posted: number;
  hungry: number;
};

export const ECONOMIES: Economy[] = ["money", "gift", "pool"];
export const KINDS = 4;
export const CAP = 3;
export const NEAR = 2;
export const NEED = 10;
export const PATIENCE = 6;
export const WEEK = 168;
export const MILLE = 1000;
export const NONE = -1;
export const STORE = -1;
export const tick: Act = { verb: "tick", args: {} };

const HUNDRED = 100;
const FIFTH = 5;

/* NEIGHBOURS */

export function apart(bots: number, a: number, b: number): number {
  const d = (((a - b) % bots) + bots) % bots;
  return Math.min(d, bots - d);
}

export const fifth = (bots: number) => Math.max(1, Math.round(bots / FIFTH));

export const room = (state: Pick<State, "bots">) => CAP * state.bots;

/* REPORT */

export function report(state: State): Report {
  const used = state.fed.reduce((a, b) => a + b, 0);
  const best = [...state.fed].sort((a, b) => b - a).slice(0, fifth(state.bots));
  const top = best.reduce((a, b) => a + b, 0);
  return {
    fed: state.posted ? Math.floor((used * MILLE) / state.posted) : 0,
    idle: state.made ? Math.floor((state.spilled * MILLE) / state.made) : 0,
    top: used ? Math.floor((top * MILLE) / used) : 0,
    fair: Math.floor((fifth(state.bots) * MILLE) / state.bots),
    made: state.made,
    used,
    spilled: state.spilled,
    posted: state.posted,
    hungry: state.fed.filter((one, bot) => one < state.missed[bot]).length,
  };
}

/* RULES */

const roll = (work: State) => {
  work.seed = xorshift(work.seed);
  return work.seed % HUNDRED;
};

const flow = (how: How, from: number, to: number, kind: number, coin = 0): Flow => ({ how, from, to, kind, coin });

function making(work: State) {
  for (let bot = 0; bot < work.bots; bot++) {
    if (roll(work) >= work.plenty) continue;
    const kind = work.crafts[bot];
    work.made++;
    if (work.economy === "pool" && roll(work) < work.generous) {
      if (work.store[kind] < room(work)) {
        work.store[kind]++;
        work.flows.push(flow("share", bot, STORE, kind));
      } else {
        work.spilled++;
        work.flows.push(flow("spill", STORE, STORE, kind));
      }
      continue;
    }
    if (work.stock[bot] < CAP) {
      work.stock[bot]++;
      work.flows.push(flow("make", bot, bot, kind));
    } else {
      work.spilled++;
      work.flows.push(flow("spill", bot, bot, kind));
    }
  }
}

function sellers(work: State, bot: number, kind: number, near: number): number[] {
  const turn = (j: number) => (j - work.tick + work.bots * work.length) % work.bots;
  const out: number[] = [];
  for (let j = 0; j < work.bots; j++)
    if (j !== bot && work.crafts[j] === kind && work.stock[j] > 0 && apart(work.bots, bot, j) <= near) out.push(j);
  return out.sort((a, b) => apart(work.bots, bot, a) - apart(work.bots, bot, b) || turn(a) - turn(b));
}

function hand(work: State, how: How, from: number, to: number, kind: number, coin: number) {
  work.stock[from]--;
  work.fed[to]++;
  if (coin) {
    work.coins[to]--;
    work.coins[from]++;
  }
  work.flows.push(flow(how, from, to, kind, coin));
}

function serve(work: State, bot: number, kind: number): boolean {
  if (work.crafts[bot] === kind && work.stock[bot] > 0) {
    work.stock[bot]--;
    work.fed[bot]++;
    work.flows.push(flow("use", bot, bot, kind));
    return true;
  }
  if (work.economy === "pool") {
    if (!work.store[kind]) return false;
    work.store[kind]--;
    work.fed[bot]++;
    work.flows.push(flow("draw", STORE, bot, kind));
    return true;
  }
  if (work.economy === "money") {
    const [from] = sellers(work, bot, kind, work.bots);
    if (from === undefined || !work.coins[bot]) return false;
    hand(work, "buy", from, bot, kind, 1);
    return true;
  }
  for (const from of sellers(work, bot, kind, NEAR)) {
    if (roll(work) >= work.generous) continue;
    hand(work, "give", from, bot, kind, work.coins[bot] ? 1 : 0);
    return true;
  }
  return false;
}

function meeting(work: State) {
  const turn = (bot: number) => (bot - work.tick + work.bots * work.length) % work.bots;
  const open: [number, number][] = [];
  for (let i = 0; i < work.needs.length; i++) if (work.needs[i] !== NONE) open.push([i, work.needs[i]]);
  open.sort(
    ([a, since], [b, when]) => since - when || turn(Math.floor(a / KINDS)) - turn(Math.floor(b / KINDS)) || a - b,
  );
  for (const [i] of open) if (serve(work, Math.floor(i / KINDS), i % KINDS)) work.needs[i] = NONE;
}

function ageing(work: State) {
  for (let i = 0; i < work.needs.length; i++) {
    if (work.needs[i] === NONE || work.tick - work.needs[i] < PATIENCE) continue;
    const bot = Math.floor(i / KINDS);
    work.needs[i] = NONE;
    work.missed[bot]++;
    work.flows.push(flow("miss", bot, bot, i % KINDS));
  }
}

function wishing(work: State) {
  for (let i = 0; i < work.needs.length; i++) {
    if (work.needs[i] !== NONE || roll(work) >= NEED) continue;
    work.needs[i] = work.tick;
    work.posted++;
  }
}

function copy(state: State): State {
  return {
    ...state,
    crafts: state.crafts,
    stock: state.stock.slice(),
    coins: state.coins.slice(),
    store: state.store.slice(),
    needs: state.needs.slice(),
    fed: state.fed.slice(),
    missed: state.missed.slice(),
    flows: [],
  };
}

export function boot({ bots, economy, plenty, generous, purse, length, seed }: Config): State {
  if (bots < KINDS) throw new Error(`give: a village needs at least ${KINDS} bots`);
  if (!ECONOMIES.includes(economy)) throw new Error(`give: no economy ${economy}`);
  let next = mix(seed);
  const crafts = Array.from({ length: bots }, (_, i) => i % KINDS);
  for (let i = KINDS; i < bots; i++) {
    next = xorshift(next);
    crafts[i] = next % KINDS;
  }
  for (let i = bots - 1; i > 0; i--) {
    next = xorshift(next);
    const j = next % (i + 1);
    [crafts[i], crafts[j]] = [crafts[j], crafts[i]];
  }
  const work: State = {
    bots,
    economy,
    plenty,
    generous,
    purse,
    length,
    tick: 0,
    crafts,
    stock: Array(bots).fill(1),
    coins: Array(bots).fill(purse),
    store: Array(KINDS).fill(economy === "pool" ? 1 : 0),
    needs: Array(bots * KINDS).fill(NONE),
    fed: Array(bots).fill(0),
    missed: Array(bots).fill(0),
    made: 0,
    posted: 0,
    spilled: 0,
    flows: [],
    over: null,
    seed: next,
  };
  wishing(work);
  return work;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const work = copy(state);
  work.tick = state.tick + 1;
  making(work);
  meeting(work);
  ageing(work);
  work.over = work.tick >= state.length ? "done" : null;
  if (!work.over) wishing(work);
  return work;
}

export function moves(state: State): Act[] {
  return state.over ? [] : [tick];
}

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

export const run = (state: State, ticks: number) => {
  let at = state;
  for (let i = 0; i < ticks && !at.over; i++) at = step(at, tick);
  return at;
};

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  if (!round) return seed ?? 0;
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal(
  { bots, economy, plenty, generous, purse }: Pick<Config, "bots" | "economy" | "plenty" | "generous" | "purse">,
  { seed, round = 0 }: Table,
): State {
  return boot({ bots, economy, plenty, generous, purse, length: WEEK, seed: dealt(seed, round) });
}
