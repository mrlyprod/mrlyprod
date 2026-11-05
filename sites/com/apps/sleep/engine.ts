import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix, pick, xorshift } from "@mrlyprod/ui/kit/rand.ts";
import { DESIGNS, LEVELS, NUMBERS } from "../shared/designs/tiles.ts";

export type Knobs = {
  design: string;
  number: number | "";
  level: number | "";
  count: number;
  speed: number;
  size: number;
};

export type Stage = { w: number; h: number; dpr: number; lock: number };

export type Sleeper = {
  d: number;
  design: string;
  number: number;
  level: number;
  vx: number;
  vy: number;
  x: number;
  y: number;
  side: number;
};

export type State = Knobs & Stage & { rand: number; crowd: Sleeper[] };

export type Config = Knobs & Stage & { seed: number };

export type Table = Omit<Stage, "lock"> & { lock: boolean | number; seed?: number; rand: Rng };

export type Bounce = { p: number; v: number; hit: boolean };

export type Act =
  | { verb: "tick"; args: { ms: number } }
  | { verb: "stage"; args: Stage }
  | { verb: "fit" }
  | { verb: "set"; args: Knobs & { seed?: number } };

export const FIT: Act = { verb: "fit" };
export const TICK: Act = { verb: "tick", args: { ms: 1000 / 60 } };
export const ticking = (ms: number): Act => ({ verb: "tick", args: { ms } });
export const staging = (stage: Stage): Act => ({ verb: "stage", args: stage });
export const setting = (knobs: Knobs & { seed?: number }): Act => ({ verb: "set", args: knobs });

const DEEP = 0.4;

const GOLD = 0.618034;

const FINE = 25;

const MARKS = ["design", "number", "level"] satisfies (keyof Knobs)[];

const depth = (k: number) => DEEP + (1 - DEEP) * ((k * GOLD) % 1);

export function bounce(p: number, v: number, max: number): Bounce {
  if (p <= 0) return { p: Math.min(-p, max), v: Math.abs(v), hit: true };
  if (p >= max) return { p: Math.max(2 * max - p, 0), v: -Math.abs(v), hit: true };
  return { p, v, hit: false };
}

function crew(state: State) {
  const next: State = { ...state, crowd: state.crowd.map((one) => ({ ...one })) };
  const rand = () => {
    next.rand = xorshift(next.rand);
    return next.rand / 4294967296;
  };
  const roll = (one: Sleeper) => {
    one.design = next.design || pick(rand, DESIGNS);
    one.number = next.number || pick(rand, NUMBERS);
    const level = next.level || pick(rand, LEVELS);
    one.level = next.lock && one.number ** level > FINE ? 1 : level;
  };
  const fit = (one: Sleeper) => {
    const n = one.number ** one.level;
    const reach = Math.min(Math.sqrt(next.w * next.h) * next.size * one.d, next.w, next.h);
    one.side = Math.max(1, Math.floor(reach / n)) * n;
    one.x = Math.min(one.x, Math.max(0, next.w - one.side));
    one.y = Math.min(one.y, Math.max(0, next.h - one.side));
  };
  const spawn = (k: number) => {
    const one: Sleeper = { d: depth(k), design: "", number: 0, level: 0, vx: 0, vy: 0, x: 0, y: 0, side: 0 };
    roll(one);
    one.vx = (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand());
    one.vy = (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand());
    fit(one);
    one.x = rand() * Math.max(0, next.w - one.side);
    one.y = rand() * Math.max(0, next.h - one.side);
    return one;
  };
  const gather = () => {
    next.crowd.length = Math.min(next.crowd.length, next.count);
    while (next.crowd.length < next.count) next.crowd.push(spawn(next.crowd.length));
  };
  const move = (one: Sleeper, run: number) => {
    const hx = bounce(one.x + one.vx * run * one.d, one.vx, Math.max(0, next.w - one.side));
    const hy = bounce(one.y + one.vy * run * one.d, one.vy, Math.max(0, next.h - one.side));
    one.x = hx.p;
    one.vx = hx.v;
    one.y = hy.p;
    one.vy = hy.v;
    if (!hx.hit && !hy.hit) return;
    roll(one);
    fit(one);
  };
  return { next, roll, fit, gather, move };
}

export function boot({ seed, ...rest }: Config): State {
  const { next, gather } = crew({ ...rest, rand: mix(seed), crowd: [] });
  gather();
  return next;
}

export function step(state: State, act: Act): State {
  if (act.verb === "stage") return { ...state, ...act.args };
  const { next, roll, fit, gather, move } = crew(state);
  if (act.verb === "tick") {
    const run = (state.speed * state.dpr * act.args.ms) / 1000;
    if (run > 0) for (const one of next.crowd) move(one, run);
  }
  if (act.verb === "fit") next.crowd.forEach(fit);
  if (act.verb === "set") {
    const { seed, ...knobs } = act.args;
    Object.assign(next, knobs);
    if (seed !== undefined) {
      next.rand = mix(seed);
      next.crowd = [];
    } else if (MARKS.some((key) => knobs[key] !== state[key])) next.crowd.forEach(roll);
    next.crowd.forEach(fit);
    gather();
  }
  return next;
}

export const moves = (_: State): Act[] => [TICK, FIT];

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

export const deal = (
  { design, number, level, count, speed, size }: Knobs,
  { w, h, dpr, lock, seed, rand }: Table,
): State =>
  boot({
    design,
    number,
    level,
    count,
    speed,
    size,
    w,
    h,
    dpr,
    lock: lock ? 1 : 0,
    seed: seed ?? Math.floor(rand() * 4294967296),
  });
