import { mix, xorshift } from "../../net/sdk/rand.ts";
import type { Mrly } from "../shared/designs/engine.ts";

/* TYPES */

export type Over = "mended" | null;

export type State = {
  number: number;
  level: number;
  motif: string;
  goal: string;
  turns: number[];
  moves: number;
  par: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "number" | "level" | "motif" | "goal" | "seed"> & { mess: number };

export type Table = { seed?: number; round?: number; units: { math: Mrly } };

export type Act = { verb: "turn"; args: { block: number; way: number } };

export type Laid = { side: number; cells: Uint8Array; boxes: Int32Array };

export const RIGHT = 1;
export const LEFT = -1;
export const WAYS = [RIGHT, LEFT];
export const turn = (block: number, way: number): Act => ({ verb: "turn", args: { block, way } });

export const DIM = 2;
export const BASE = 3;
export const HOME = "473";
export const NUMBERS = [3, 5, 7];
export const ROOM = 128;

const QUARTERS = [0, 1, 2, 3];
const EXTRA = 64;

/* DOOR */

export function cap(number: number): number {
  let level = 1;
  while (number ** (level + 1) <= ROOM) level++;
  return level;
}

export function coded(math: Mrly, code: unknown): string {
  const text = String(code ?? "").trim();
  const count = Number(math.bang.factory.total_codes(DIM, BASE));
  if (text === "" || !Number.isFinite(Number(text))) return HOME;
  return String(((Math.floor(Number(text)) % count) + count) % count);
}

export function grown(math: Mrly, number: number, level: number, code: string): string {
  const cell = math.two.create(code, number, level, 0, BASE);
  return Array.from(cell.types, (type) => (type ? "1" : "0")).join("");
}

/* TREE */

export const fills = (motif: string) => [...motif].flatMap((one, i) => (one === "1" ? [i] : []));

export function counts(state: Pick<State, "motif" | "level">): number[] {
  const f = fills(state.motif).length;
  return Array.from({ length: state.level }, (_, d) => f ** d);
}

export function starts(state: Pick<State, "motif" | "level">): number[] {
  const out = [0];
  for (const n of counts(state)) out.push(out[out.length - 1] + n);
  return out;
}

export const blocks = (state: Pick<State, "motif" | "level">) => starts(state)[state.level];

export function depth(state: Pick<State, "motif" | "level">, block: number): number {
  const at = starts(state);
  let d = 0;
  while (d + 1 < at.length - 1 && at[d + 1] <= block) d++;
  return d;
}

export function spin(n: number, slot: number, q: number): number {
  let r = Math.floor(slot / n);
  let c = slot % n;
  for (let i = 0; i < (q & 3); i++) [r, c] = [c, n - 1 - r];
  return r * n + c;
}

export function rotate(motif: string, n: number, q: number): string {
  const out = new Array<string>(n * n).fill("0");
  for (const slot of fills(motif)) out[spin(n, slot, q)] = "1";
  return out.join("");
}

export const symmetries = (motif: string, n: number) => QUARTERS.filter((q) => rotate(motif, n, q) === motif);

/* PICTURE */

export function lay(state: State): Laid {
  const { number: n, level, motif, turns } = state;
  const slots = fills(motif);
  const f = slots.length;
  const at = starts(state);
  const side = n ** level;
  const cells = new Uint8Array(side * side);
  const boxes = new Int32Array(at[level] * 3);
  const walk = (d: number, p: number, x: number, y: number, size: number, abs: number) => {
    const id = at[d] + p;
    boxes.set([x, y, size], id * 3);
    const a = (abs + turns[id]) & 3;
    const sub = size / n;
    for (let k = 0; k < f; k++) {
      const slot = spin(n, slots[k], a);
      const cx = x + (slot % n) * sub;
      const cy = y + Math.floor(slot / n) * sub;
      if (d === level - 1) cells[cy * side + cx] = 1;
      else walk(d + 1, p * f + k, cx, cy, sub, a);
    }
  };
  walk(0, 0, 0, 0, side, 0);
  return { side, cells, boxes };
}

export const picture = (state: State) => Array.from(lay(state).cells, (one) => (one ? "1" : "0")).join("");

export function under(state: State, laid: Laid, d: number, x: number, y: number): number {
  const at = starts(state);
  for (let id = at[d]; id < at[d + 1]; id++) {
    const [bx, by, size] = laid.boxes.subarray(id * 3, id * 3 + 3);
    if (x >= bx && x < bx + size && y >= by && y < by + size) return id;
  }
  return -1;
}

/* PAR */

const cost = (q: number) => Math.min(q & 3, 4 - (q & 3));

export function table(state: State): Int32Array {
  const { number: n, level, motif, turns } = state;
  const good = symmetries(motif, n);
  const f = fills(motif).length;
  const at = starts(state);
  const best = new Int32Array(at[level] * 4);
  for (let d = level - 1; d >= 0; d--) {
    for (let id = at[d]; id < at[d + 1]; id++) {
      const first = d + 1 < level ? at[d + 1] + (id - at[d]) * f : -1;
      for (let p = 0; p < 4; p++) {
        let low = Number.MAX_SAFE_INTEGER;
        for (const a of good) {
          let sum = cost(a - p - turns[id]);
          for (let k = 0; first >= 0 && k < f; k++) sum += best[(first + k) * 4 + a];
          low = Math.min(low, sum);
        }
        best[id * 4 + p] = low;
      }
    }
  }
  return best;
}

export function plan(state: State): number[] {
  const { number: n, level, motif, turns } = state;
  const good = symmetries(motif, n);
  const f = fills(motif).length;
  const at = starts(state);
  const best = table(state);
  const want = new Int32Array(at[level]);
  const held = new Int32Array(at[level]);
  for (let d = 0; d < level; d++) {
    for (let id = at[d]; id < at[d + 1]; id++) {
      const p = d ? held[id] : 0;
      const first = d + 1 < level ? at[d + 1] + (id - at[d]) * f : -1;
      let pick = good[0];
      let low = Number.MAX_SAFE_INTEGER;
      for (const a of good) {
        let sum = cost(a - p - turns[id]);
        for (let k = 0; first >= 0 && k < f; k++) sum += best[(first + k) * 4 + a];
        if (sum < low) {
          low = sum;
          pick = a;
        }
      }
      want[id] = (pick - p - turns[id]) & 3;
      for (let k = 0; first >= 0 && k < f; k++) held[first + k] = pick;
    }
  }
  return Array.from(want);
}

export const par = (state: State) => (blocks(state) ? table(state)[0] : 0);

/* RULES */

export function boot({ number, level, motif, goal, mess, seed }: Config): State {
  const side = number ** level;
  if (motif.length !== number * number) throw new Error(`mend: the motif is not ${number} by ${number}`);
  if (goal.length !== side * side) throw new Error(`mend: the goal is not ${side} by ${side}`);
  const shape = { motif, level };
  const each = counts(shape);
  const at = starts(shape);
  const turns = new Array<number>(at[level]).fill(0);
  let next = mix(seed);
  const jolt = () => {
    next = xorshift(next);
    const d = next % level;
    next = xorshift(next);
    const id = at[d] + (next % each[d]);
    next = xorshift(next);
    turns[id] = (turns[id] + 1 + (next % 3)) & 3;
  };
  const state = (): State => ({ number, level, motif, goal, turns, moves: 0, par: 0, over: null, seed: next });
  if (!each.every(Boolean)) return { ...state(), over: "mended" };
  for (let i = 0; i < mess; i++) jolt();
  const whole = symmetries(motif, number).length === QUARTERS.length;
  for (let i = 0; i < EXTRA && !whole && !par(state()); i++) jolt();
  const dealt = state();
  const fewest = par(dealt);
  return { ...dealt, par: fewest, over: picture(dealt) === goal ? "mended" : null };
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "turn") return state;
  const { block, way } = act.args;
  if (!Number.isInteger(block) || block < 0 || block >= state.turns.length || !WAYS.includes(way)) return state;
  const turns = state.turns.slice();
  turns[block] = (turns[block] + way + 4) & 3;
  const next = { ...state, turns, moves: state.moves + 1 };
  return picture(next) === state.goal ? { ...next, over: "mended" } : next;
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return state.turns.flatMap((_, block) => WAYS.map((way) => turn(block, way)));
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
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal(
  { code, number, level, mess }: Pick<Config, "number" | "level" | "mess"> & { code: unknown },
  { seed, round = 0, units }: Table,
): State {
  const motif = coded(units.math, code);
  return boot({
    number,
    level,
    motif: grown(units.math, number, 1, motif),
    goal: grown(units.math, number, level, motif),
    mess,
    seed: dealt(seed, round),
  });
}
