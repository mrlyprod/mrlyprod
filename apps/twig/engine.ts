import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type State = {
  levels: number;
  leaves: string;
  taps: number;
  par: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "levels" | "seed">;

export type Table = { seed?: number; round?: number };

export type Act = { verb: "spin"; args: { fork: number } };

export const LOW = 2;
export const HIGH = 5;

export const spin = (fork: number): Act => ({ verb: "spin", args: { fork } });

const TRIES = 64;

/* TREE */

export const forks = (levels: number) => 2 ** levels - 1;

export const depth = (fork: number) => Math.floor(Math.log2(fork + 1));

export function block(levels: number, fork: number): [number, number] {
  const d = depth(fork);
  const size = 2 ** (levels - d);
  const lo = (fork - (2 ** d - 1)) * size;
  return [lo, lo + size];
}

export const sorted = (levels: number) => Array.from({ length: 2 ** levels }, (_, k) => k.toString(36)).join("");

export function spun(state: Pick<State, "levels" | "leaves">, fork: number): string {
  const [lo, hi] = block(state.levels, fork);
  const all = [...state.leaves];
  return [...all.slice(0, lo), ...all.slice(lo, hi).reverse(), ...all.slice(hi)].join("");
}

export function neat(state: Pick<State, "levels" | "leaves">, fork: number): boolean {
  const [lo, hi] = block(state.levels, fork);
  for (let k = lo + 1; k < hi; k++)
    if (Number.parseInt(state.leaves[k], 36) < Number.parseInt(state.leaves[k - 1], 36)) return false;
  return true;
}

/* SOLVE */

export function solve(state: Pick<State, "levels" | "leaves">): number[] {
  const out: number[] = [];
  let at = { levels: state.levels, leaves: state.leaves };
  for (let i = 0; i < forks(at.levels); i++) {
    const [lo, hi] = block(at.levels, i);
    if (Number.parseInt(at.leaves[lo], 36) > Number.parseInt(at.leaves[hi - 1], 36)) {
      out.push(i);
      at = { ...at, leaves: spun(at, i) };
    }
  }
  return out;
}

/* RULES */

const judge = (state: State): State => (state.leaves === sorted(state.levels) ? { ...state, over: "done" } : state);

export function boot({ levels, seed }: Config): State {
  if (!Number.isInteger(levels) || levels < LOW || levels > HIGH) throw new Error(`twig: not ${LOW} to ${HIGH} levels`);
  let rolled = mix(seed);
  const roll = (n: number) => {
    rolled = xorshift(rolled);
    return rolled % n;
  };
  const n = forks(levels);
  const want = Math.floor(n / 2) + 1;
  let best: State | null = null;
  for (let t = 0; t < TRIES; t++) {
    let leaves = sorted(levels);
    for (let i = 0; i < n; i++) if (roll(2)) leaves = spun({ levels, leaves }, i);
    const state: State = { levels, leaves, taps: 0, par: 0, over: null, seed: rolled };
    const par = solve(state).length;
    if (!par) continue;
    if (!best || par > best.par) best = { ...state, par };
    if (par >= want) break;
  }
  if (!best) throw new Error("twig: no tree dealt");
  return best;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "spin") return state;
  const { fork } = act.args;
  if (!Number.isInteger(fork) || fork < 0 || fork >= forks(state.levels)) return state;
  return judge({ ...state, leaves: spun(state, fork), taps: state.taps + 1 });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return Array.from({ length: forks(state.levels) }, (_, k) => spin(k));
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

export function deal({ levels }: Pick<Config, "levels">, { seed, round = 0 }: Table): State {
  return boot({ levels, seed: dealt(seed, round) });
}
