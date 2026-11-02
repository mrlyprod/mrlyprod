import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type State = {
  rings: number;
  arms: number;
  turns: string;
  taps: number;
  par: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "rings" | "arms" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "turn"; args: { ring: number } };

export const FULL = 12;
export const FEW = 3;
export const MANY = 7;
export const THIN = 2;
export const THICK = 4;

export const turn = (ring: number): Act => ({ verb: "turn", args: { ring } });

const TRIES = 64;

/* RINGS */

export const period = (arms: number) => FULL / arms;

export const home = (state: Pick<State, "turns">) => [...state.turns].filter((one) => one === "0").length;

export function spun(state: Pick<State, "turns" | "arms">, ring: number): string {
  const q = period(state.arms);
  const digits = [...state.turns].map(Number);
  for (const k of [ring, ring + 1]) if (k < digits.length) digits[k] = (digits[k] + 1) % q;
  return digits.join("");
}

/* SOLVE */

export function solve(state: Pick<State, "turns" | "arms">): number[] {
  const out: number[] = [];
  let at = { ...state };
  for (let k = 0; k < at.turns.length; k++)
    while (at.turns[k] !== "0") {
      out.push(k);
      at = { ...at, turns: spun(at, k) };
    }
  return out;
}

/* RULES */

const judge = (state: State): State => (home(state) === state.rings ? { ...state, over: "done" } : state);

export function boot({ rings, arms, seed }: Config): State {
  if (!Number.isInteger(rings) || rings < FEW || rings > MANY) throw new Error(`swirl: not ${FEW} to ${MANY} rings`);
  if (!Number.isInteger(arms) || arms < THIN || arms > THICK || FULL % arms)
    throw new Error(`swirl: not ${THIN} to ${THICK} arms`);
  let rolled = mix(seed);
  const roll = (n: number) => {
    rolled = xorshift(rolled);
    return rolled % n;
  };
  const q = period(arms);
  const want = Math.floor((rings * (q - 1)) / 2) + 1;
  let best: State | null = null;
  for (let t = 0; t < TRIES; t++) {
    const turns = Array.from({ length: rings }, () => roll(q)).join("");
    const state: State = { rings, arms, turns, taps: 0, par: 0, over: null, seed: rolled };
    const par = solve(state).length;
    if (!par) continue;
    if (!best || par > best.par) best = { ...state, par };
    if (par >= want) break;
  }
  if (!best) throw new Error("swirl: no whorl dealt");
  return best;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "turn") return state;
  const { ring } = act.args;
  if (!Number.isInteger(ring) || ring < 0 || ring >= state.rings) return state;
  return judge({ ...state, turns: spun(state, ring), taps: state.taps + 1 });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return Array.from({ length: state.rings }, (_, k) => turn(k));
}

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  let n = mix(seed ?? 0);
  for (let i = 0; i < round; i++) n = xorshift(n);
  return n;
}

export function deal({ rings, arms }: Pick<Config, "rings" | "arms">, { seed, round = 0 }: Table): State {
  return boot({ rings, arms, seed: dealt(seed, round) });
}
