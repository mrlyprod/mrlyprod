import * as life from "@mrlyprod/life";
import { bits, population, types } from "./engine.ts";
import type { Mode, Recipe, Stop } from "./recipe.ts";

export type State = {
  mode: Mode;
  x: number;
  cap: number;
  side: number;
  reach: number;
  budget: number;
  birth: number[];
  survive: number[];
  span: number[];
  mask: string;
  rows: number;
  cols: number;
  cells: string;
  origin: number[];
  background: number;
  gen: number;
  pop: number;
  seen: Record<string, number[]>;
  stop: Stop | null;
};

export type Config = Recipe;

export type Act = { verb: "tick" };

export const TICK: Act = { verb: "tick" };

function times(ah: number, al: number, bh: number, bl: number): [number, number] {
  const a1 = al >>> 16;
  const a0 = al & 0xffff;
  const b1 = bl >>> 16;
  const b0 = bl & 0xffff;
  const p01 = a0 * b1;
  const p10 = a1 * b0;
  const mid = ((a0 * b0) >>> 16) + (p01 & 0xffff) + (p10 & 0xffff);
  const carry = a1 * b1 + (p01 >>> 16) + (p10 >>> 16) + (mid >>> 16);
  return [(carry + Math.imul(al, bh) + Math.imul(ah, bl)) >>> 0, Math.imul(al, bl) >>> 0];
}

const BYTES = Array.from({ length: 256 }, (_, i) => i.toString(16).padStart(2, "0"));

const word = (n: number) => BYTES[n >>> 24] + BYTES[(n >>> 16) & 255] + BYTES[(n >>> 8) & 255] + BYTES[n & 255];

const hex = (hi: number, lo: number) => word(hi) + word(lo);

function hashed(rows: number, cols: number, cells: Uint8Array, background: number): string {
  let [ah, al] = [0xcbf29ce4, 0x84222325];
  let [bh, bl] = [0x9e3779b9, 0x7f4a7c15];
  const quads =
    cells.byteOffset % 4 || cells.length < 64
      ? null
      : new Uint32Array(cells.buffer, cells.byteOffset, cells.length >> 2);
  const nibble = (q: number) => Math.imul(quads![q] & 0x01010101, 0x01020408) >>> 24;
  for (let at = 0; at < cells.length; at += 64) {
    let lo = 0;
    let hi = 0;
    if (quads && at + 64 <= cells.length) {
      const q = at >> 2;
      for (let i = 0; i < 8; i++) lo |= nibble(q + i) << (4 * i);
      for (let i = 0; i < 8; i++) hi |= nibble(q + 8 + i) << (4 * i);
    } else {
      const end = Math.min(64, cells.length - at);
      for (let i = 0; i < 32 && i < end; i++) lo |= (cells[at + i] & 1) << i;
      for (let i = 32; i < end; i++) hi |= (cells[at + i] & 1) << (i - 32);
    }
    [ah, al] = times((ah ^ hi) >>> 0, (al ^ lo) >>> 0, 0x100, 0x1b3);
    const rh = ((bh << 29) | (bl >>> 3)) >>> 0;
    const rl = ((bl << 29) | (bh >>> 3)) >>> 0;
    [bh, bl] = times((rh ^ hi) >>> 0, (rl ^ lo) >>> 0, 0xbf58476d, 0x1ce4e5b9);
  }
  [ah, al] = times(ah, (al ^ cols) >>> 0, 0x100, 0x1b3);
  [bh, bl] = times(bh, (bl ^ rows) >>> 0, 0x94d049bb, 0x133111eb);
  return hex(ah, al) + hex(bh, (bl ^ background) >>> 0);
}

type Frame = { rows: number; cols: number; cells: Uint8Array; origin: number[] };

function walls(state: State) {
  const lo = -Math.floor((state.cap - state.side) / 2);
  return { lo, hi: lo + state.cap - 1, reach: state.reach };
}

function hit(state: State, { rows, cols, origin }: Frame): boolean {
  if (!rows) return false;
  const { lo, hi, reach } = walls(state);
  const [r0, c0] = origin;
  return r0 - lo < reach || c0 - lo < reach || hi - (r0 + rows - 1) < reach || hi - (c0 + cols - 1) < reach;
}

function wallBy(state: State, { rows, cols, origin }: Frame, dr: number, dc: number): number {
  const { lo, hi, reach } = walls(state);
  const [r0, c0] = origin;
  let k = Number.POSITIVE_INFINITY;
  const room = (space: number, speed: number) => {
    if (speed > 0) k = Math.min(k, Math.floor(space / speed) + 1);
  };
  room(hi - reach - (r0 + rows - 1), dr);
  room(r0 - lo - reach, -dr);
  room(hi - reach - (c0 + cols - 1), dc);
  room(c0 - lo - reach, -dc);
  return k;
}

const at = (kind: string, generation: number, period: number): Stop => ({
  kind,
  generation,
  period,
  dr: 0,
  dc: 0,
  wall_by: 0,
});

export function boot(config: Config): State {
  const run = new life.Run(config);
  try {
    const frame = run.frame();
    const [rows, cols] = frame.shape;
    const origin = Array.from(run.origin(), Number);
    const mask = run.mask();
    const stop = run.stop();
    return {
      mode: config.mode,
      x: config.x,
      cap: run.cap(),
      side: run.side(),
      reach: run.reach(),
      budget: run.budget(),
      birth: Array.from(run.birth()),
      survive: Array.from(run.survive()),
      span: Array.from(mask.shape),
      mask: bits(mask.data),
      rows,
      cols,
      cells: bits(frame.types),
      origin,
      background: run.background(),
      gen: 0,
      pop: population(frame.types),
      seen: { [hashed(rows, cols, Uint8Array.from(frame.types), 0)]: [0, ...origin] },
      stop: stop ? at(stop.kind, stop.generation, stop.period) : null,
    };
  } finally {
    run.free();
  }
}

function fate(state: State, frame: Frame, pop: number, background: number, gen: number): [Stop | null, State["seen"]] {
  const island = state.mode === "Island";
  if (!pop) return [at("dead", gen, 1), state.seen];
  if (island && hit(state, frame)) return [at("wall", gen, 0), state.seen];
  const id = hashed(frame.rows, frame.cols, frame.cells, background);
  const seen = state.seen[id];
  if (!seen) return [null, { ...state.seen, [id]: [gen, ...frame.origin] }];
  const period = gen - seen[0];
  const dr = frame.origin[0] - seen[1];
  const dc = frame.origin[1] - seen[2];
  if (!island || (!dr && !dc)) return [at(period === 1 ? "still" : "loop", gen, period), state.seen];
  const by = gen + wallBy(state, frame, dr, dc) * period;
  const wall_by = by <= state.x ? by : 0;
  return [{ kind: "mover", generation: gen, period, dr, dc, wall_by }, state.seen];
}

function settled(state: State, frame: Frame, pop: number, background: number): State {
  const gen = state.gen + 1;
  const [stop, seen] = fate(state, frame, pop, background, gen);
  const { rows, cols, origin } = frame;
  return { ...state, rows, cols, cells: bits(frame.cells), origin, background, gen, pop, seen, stop };
}

function tick(state: State): State {
  if (state.stop) return state;
  if (state.gen >= state.x) return { ...state, stop: at("limit", state.gen, 0) };
  const frame = { shape: [state.rows, state.cols], types: types(state.cells) };
  const mask = { shape: state.span, data: types(state.mask) };
  const [next, shift, background] = life.evolve(frame, mask, state.birth, state.survive, state.mode, state.background);
  const [rows, cols] = next.shape;
  const cells = Uint8Array.from(next.types);
  const origin = [state.origin[0] + Number(shift[0]), state.origin[1] + Number(shift[1])];
  return settled(state, { rows, cols, cells, origin }, population(cells), background);
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  return state;
}

export const moves = (state: State): Act[] => (state.stop ? [] : [TICK]);

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}
