import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Mode = "vote" | "ruler" | "lottery";

export type How = "open" | "granted" | "refused" | "dropped";

export type Wish = {
  id: number;
  by: number;
  hour: number;
  at: number;
  judge: number;
  votes: string;
  end: number;
  how: How;
};

export type Over = "done" | null;

export type State = {
  bots: number;
  decide: Mode;
  eager: number;
  patience: number;
  length: number;
  tick: number;
  hours: number[];
  ruler: number;
  wishes: Wish[];
  over: Over;
  seed: number;
};

export type Config = Pick<State, "bots" | "decide" | "eager" | "patience" | "length" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tick"; args: Record<string, never> };

export type Report = { posted: number; granted: number; wait: number; reach: number; dropped: number };

export const MODES: Mode[] = ["vote", "ruler", "lottery"];
export const HOURS = 12;
export const NEAR = 2;
export const DRIFT = 3;
export const WEEK = 168;
export const MILLE = 1000;
export const tick: Act = { verb: "tick", args: {} };

const NONE = "0";
const YES = "1";
const NO = "2";
const HUNDRED = 100;

/* TASTE */

export function apart(a: number, b: number): number {
  const d = (((a - b) % HOURS) + HOURS) % HOURS;
  return Math.min(d, HOURS - d);
}

export const likes = (state: Pick<State, "hours">, bot: number, wish: Pick<Wish, "by" | "hour">) =>
  bot === wish.by || apart(state.hours[bot], wish.hour) <= NEAR;

export const majority = (bots: number) => Math.floor(bots / 2) + 1;

/* REPORT */

export function median(list: readonly number[]): number {
  if (!list.length) return 0;
  const sorted = [...list].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 ? sorted[mid] * MILLE : (sorted[mid - 1] + sorted[mid]) * (MILLE / 2);
}

export function report(state: State): Report {
  const posted = state.wishes.length;
  const won = state.wishes.filter((one) => one.how === "granted");
  const decided = state.wishes.filter((one) => one.how === "granted" || one.how === "refused");
  return {
    posted,
    granted: posted ? Math.floor((won.length * MILLE) / posted) : 0,
    wait: median(decided.map((one) => one.end - one.at)),
    reach: new Set(won.map((one) => one.by)).size,
    dropped: state.wishes.filter((one) => one.how === "dropped").length,
  };
}

/* RULES */

function wishing(state: State, seed: number): { wishes: Wish[]; seed: number } {
  const wishes = state.wishes.slice();
  const busy = new Set(wishes.filter((one) => one.how === "open").map((one) => one.by));
  let next = seed;
  for (let by = 0; by < state.bots; by++) {
    if (busy.has(by)) continue;
    next = xorshift(next);
    if (next % HUNDRED >= state.eager) continue;
    next = xorshift(next);
    const hour = (state.hours[by] + (next % (2 * DRIFT + 1)) - DRIFT + HOURS) % HOURS;
    next = xorshift(next);
    const judge = (by + 1 + (next % (state.bots - 1))) % state.bots;
    const votes = Array.from({ length: state.bots }, (_, i) => (i === by ? YES : NONE)).join("");
    wishes.push({ id: wishes.length, by, hour, at: state.tick, judge, votes, end: -1, how: "open" });
  }
  return { wishes, seed: next };
}

function close(wish: Wish, granted: boolean, at: number): Wish {
  return { ...wish, end: at, how: granted ? "granted" : "refused" };
}

function voting(state: State): Wish[] {
  const wishes = state.wishes.slice();
  const need = majority(state.bots);
  const against = state.bots - need + 1;
  for (let bot = 0; bot < state.bots; bot++) {
    const i = wishes.findIndex((one) => one.how === "open" && one.votes[bot] === NONE);
    if (i < 0) continue;
    const wish = wishes[i];
    const votes = wish.votes.slice(0, bot) + (likes(state, bot, wish) ? YES : NO) + wish.votes.slice(bot + 1);
    const yes = votes.split(YES).length - 1;
    const no = votes.split(NO).length - 1;
    const cast = { ...wish, votes };
    wishes[i] = yes >= need ? close(cast, true, state.tick) : no >= against ? close(cast, false, state.tick) : cast;
  }
  return wishes;
}

function judging(state: State, judge: (wish: Wish) => number): Wish[] {
  const wishes = state.wishes.slice();
  const acted = new Set<number>();
  for (let i = 0; i < wishes.length; i++) {
    const wish = wishes[i];
    const who = judge(wish);
    if (wish.how !== "open" || acted.has(who)) continue;
    acted.add(who);
    wishes[i] = close(wish, likes(state, who, wish), state.tick);
  }
  return wishes;
}

function deciding(state: State): Wish[] {
  if (state.decide === "vote") return voting(state);
  if (state.decide === "ruler") return judging(state, () => state.ruler);
  return judging(state, (wish) => wish.judge);
}

function dropping(state: State, wishes: Wish[]): Wish[] {
  return wishes.map((one) =>
    one.how === "open" && state.tick - one.at >= state.patience ? { ...one, end: state.tick, how: "dropped" } : one,
  );
}

export function boot({ bots, decide, eager, patience, length, seed }: Config): State {
  if (bots < 3) throw new Error("decide: a village needs at least 3 bots");
  if (!MODES.includes(decide)) throw new Error(`decide: no rule ${decide}`);
  let next = mix(seed);
  const hours: number[] = [];
  for (let i = 0; i < bots; i++) {
    next = xorshift(next);
    hours.push(next % HOURS);
  }
  next = xorshift(next);
  const ruler = next % bots;
  const born: State = {
    bots,
    decide,
    eager,
    patience,
    length,
    tick: 0,
    hours,
    ruler,
    wishes: [],
    over: null,
    seed: next,
  };
  return { ...born, ...wishing(born, next) };
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tick") return state;
  const now = state.tick + 1;
  const decided = deciding({ ...state, tick: now });
  const kept = dropping({ ...state, tick: now }, decided);
  const over: Over = now >= state.length ? "done" : null;
  const posted = over ? { wishes: kept, seed: state.seed } : wishing({ ...state, tick: now, wishes: kept }, state.seed);
  return { ...state, tick: now, wishes: posted.wishes, seed: posted.seed, over };
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
  { bots, decide, eager, patience }: Pick<Config, "bots" | "decide" | "eager" | "patience">,
  { seed, round = 0 }: Table,
): State {
  return boot({ bots, decide, eager, patience, length: WEEK, seed: dealt(seed, round) });
}
