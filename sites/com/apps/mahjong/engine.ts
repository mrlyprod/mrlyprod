import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import { HUES } from "@mrlyprod/ui/kit/theme/hues.ts";
import { below, shuffled } from "../shared/games/sprites.ts";

export type Suit = "carpet" | "plate" | "loop" | "rose" | "cube" | "star" | "dragon";

export type Grown = {
  suit: "carpet" | "loop" | "cube" | "star";
  code: string;
  base: number;
  number: number;
  level: number;
};

export type Plate = { suit: "plate"; n: number; m: number; sign: number; fine?: number };

export type Rose = { suit: "rose"; ring: number; wheel: number; lace: number; code: string; number: number };

export type Dragon = { suit: "dragon"; name: string; level: number };

export type Bare = Grown | Plate | Rose | Dragon;

export type Face = Bare & { hue: string };

export type Spot = { x: number; y: number; z: number };

export type Tile = Spot & { face: number; gone: boolean };

export type Past = { a: number; b: number; score: number; streak: number };

export type Config = { seed: number; ratio?: number; level?: number; score?: number };

export type State = {
  level: number;
  seed: number;
  ratio: number;
  tiles: Tile[];
  deck: Face[];
  picked: number;
  hint: number[];
  past: Past[];
  score: number;
  streak: number;
  hints: number;
  undos: number;
  shuffles: number;
  cleared: boolean;
  stuck: boolean;
  over: boolean;
};

export type Act =
  | { verb: "pick"; args: { at: number } }
  | { verb: "shuffle" }
  | { verb: "undo" }
  | { verb: "hint" }
  | { verb: "next" };

type Links = { above: number[][]; left: number[][]; right: number[][] };

type Rule = (i: number, j: number) => boolean;

export const TALL = 1.16;

export const SUITS: Suit[] = ["carpet", "plate", "loop", "rose", "cube", "star", "dragon"];

const WIDE = 4294967296;
const STEP = 2;
const TRIES = 40;
const TOP = 9;
const POINTS = 10;
const CLEAR = 100;
const BURN = 4;
const MOST = 8;
const HINTS = 3;
const UNDOS = 3;
const SHUFFLES = 2;
const LARGEST = 144;
const MEAGRE = 4;
const MOST_RATIO = 3;

const RULES: Record<string, Rule> = {
  solid: () => true,
  carpet: (i, j) => !(i % 3 === 1 && j % 3 === 1),
  net: (i, j) => i % 2 === 0 || j % 2 === 0,
  void: (i, j) => (i + j) % 2 === 0,
  htree: (_i, j) => j % 2 === 0,
  vtree: (i) => i % 2 === 0,
  star: (i, j) => i === 0 || j === 0 || i === j,
};

const PLANS: { area: number; layers: string[] }[] = [
  { area: 20, layers: ["solid", "htree"] },
  { area: 30, layers: ["carpet", "solid"] },
  { area: 38, layers: ["solid", "vtree", "solid"] },
  { area: 44, layers: ["carpet", "solid", "net"] },
  { area: 52, layers: ["solid", "carpet", "htree"] },
  { area: 60, layers: ["net", "solid", "carpet", "star"] },
  { area: 66, layers: ["carpet", "solid", "void", "solid"] },
  { area: 72, layers: ["solid", "net", "solid", "vtree"] },
];

const CARPETS: [string, number][] = [
  ["495", 3],
  ["186", 3],
  ["341", 3],
  ["325", 3],
  ["170", 3],
  ["7", 2],
  ["14", 2],
  ["9", 2],
  ["6", 2],
  ["1", 2],
];

const LOOPS: [string, number, number][] = [
  ["495", 3, 2],
  ["186", 3, 2],
  ["341", 3, 2],
  ["170", 3, 2],
  ["325", 3, 2],
  ["7", 2, 3],
  ["14", 2, 3],
  ["6", 2, 4],
  ["9", 2, 4],
];

const PLATES: [number, number, number][] = [
  [1, 2, 1],
  [1, 3, 1],
  [2, 3, 1],
  [1, 4, -1],
  [3, 4, 1],
  [3, 5, 1],
  [2, 5, -1],
  [1, 5, 1],
  [3, 7, 1],
  [4, 7, -1],
  [5, 6, 1],
  [2, 7, 1],
  [2, 9, 1],
  [5, 7, -1],
  [4, 9, 1],
  [5, 8, -1],
];

const ROSES: [number, number, number, string, number][] = [
  [3, 1, 4, "495", 3],
  [5, 2, 3, "495", 3],
  [7, 3, 2, "186", 3],
  [5, 3, 2, "341", 3],
  [8, 3, 2, "170", 3],
  [7, 2, 2, "495", 5],
  [9, 4, 2, "341", 3],
  [11, 3, 1, "186", 3],
  [7, 4, 2, "495", 3],
  [9, 2, 2, "170", 3],
];

const CUBES = ["23", "31", "43", "77", "83", "86", "89", "101", "105", "107", "111", "113", "126", "150", "151", "232"];

const STARS = ["23", "31", "43", "77", "86", "105", "107", "111", "126", "150", "151"];

const DRAGONS: [string, number][] = [
  ["twindragon", 12],
  ["terdragon", 7],
  ["flowsnake", 4],
  ["koch", 5],
  ["gasket", 6],
  ["twindragon", 10],
  ["terdragon", 6],
  ["flowsnake", 3],
  ["koch", 4],
  ["gasket", 4],
];

const planAt = (level: number) => PLANS[Math.min(level - 1, PLANS.length - 1)];

function hewn(rich: number): Pick<Grown, "number" | "level">[] {
  return [
    { number: 3, level: rich >= 8 ? 4 : rich >= 3 ? 3 : 2 },
    { number: 5, level: rich >= 3 ? 2 : 1 },
    ...(rich >= 6 ? [{ number: 7, level: 2 }] : []),
  ];
}

const POOLS: Record<Suit, (rich: number) => Bare[]> = {
  carpet: (rich) =>
    CARPETS.flatMap(([code, base]) => hewn(rich).map((one): Bare => ({ suit: "carpet", code, base, ...one }))),
  plate: (rich) =>
    PLATES.slice(0, Math.min(PLATES.length, 8 + rich)).map(([n, m, sign]): Bare => ({ suit: "plate", n, m, sign })),
  loop: () => LOOPS.map(([code, base, level]): Bare => ({ suit: "loop", code, base, number: 3, level })),
  rose: () =>
    ROSES.map(([ring, wheel, lace, code, number]): Bare => ({ suit: "rose", ring, wheel, lace, code, number })),
  cube: (rich) => CUBES.map((code): Bare => ({ suit: "cube", code, base: 2, number: 3, level: rich >= 9 ? 3 : 2 })),
  star: (rich) => STARS.map((code): Bare => ({ suit: "star", code, base: 2, number: 3, level: rich >= 9 ? 3 : 2 })),
  dragon: () => DRAGONS.map(([name, level]): Bare => ({ suit: "dragon", name, level })),
};

export const unlocked = (level: number) => Math.min(level, SUITS.length);

export const newest = (level: number): Suit => SUITS[(level - 1) % SUITS.length];

function spread(level: number, count: number): Suit[] {
  const open = SUITS.slice(0, unlocked(level));
  const lead = newest(level);
  const leading = open.length === 1 ? count : Math.ceil(count / 2);
  const rest = open.filter((one) => one !== lead);
  return Array.from({ length: count }, (_, i) => (i < leading ? lead : rest[i % rest.length]));
}

export function deckFor(level: number, count: number, seed: number): [Face[], number] {
  let at = seed;
  const pools = new Map<Suit, Bare[]>();
  const draw = (suit: Suit) => {
    if (!pools.get(suit)?.length) {
      const [fresh, next] = shuffled(at, POOLS[suit](level));
      pools.set(suit, fresh);
      at = next;
    }
    return pools.get(suit)!.pop()!;
  };
  const [hues, next] = shuffled(at, HUES);
  at = next;
  const taken = new Set<string>();
  const deck = spread(level, count).map((suit, i): Face => {
    const bare = draw(suit);
    let turn = i;
    while (taken.has(JSON.stringify({ ...bare, hue: hues[turn % hues.length] }))) turn++;
    const face = { ...bare, hue: hues[turn % hues.length] } as Face;
    taken.add(JSON.stringify(face));
    return face;
  });
  return [deck, at];
}

export function spots(level: number, ratio: number): Spot[] {
  const plan = planAt(level);
  const cols = Math.max(MEAGRE, Math.round(Math.sqrt(plan.area * ratio * TALL)));
  const rows = Math.max(MEAGRE, Math.round(plan.area / cols));
  const wide = cols >= rows;
  const out: Spot[] = [];
  const has = new Set<string>();
  const key = (x: number, y: number, z: number) => `${x},${y},${z}`;
  const under = (x: number, y: number, z: number) =>
    z === 0 ||
    [
      [x, y],
      [x + 1, y],
      [x, y + 1],
      [x + 1, y + 1],
    ].every(([px, py]) => has.has(key(px, py, z - 1)));
  for (const [z, name] of plan.layers.entries()) {
    const rule = RULES[name];
    const across = cols - 2 * z;
    const down = rows - 2 * z;
    if (across < 1 || down < 1) break;
    const placed: Spot[] = [];
    for (let row = 0; row < down; row++) {
      for (let col = 0; col < across; col++) {
        const i = Math.floor(Math.abs(col - (across - 1) / 2));
        const j = Math.floor(Math.abs(row - (down - 1) / 2));
        const x = (col + z) * STEP;
        const y = (row + z) * STEP;
        if (rule(wide ? i : j, wide ? j : i) && under(x, y, z)) placed.push({ x, y, z });
      }
    }
    for (const { x, y } of placed)
      for (const [px, py] of [
        [x, y],
        [x + 1, y],
        [x, y + 1],
        [x + 1, y + 1],
      ])
        has.add(key(px, py, z));
    out.push(...placed);
  }
  if (out.length % 2) {
    const lift = Math.max(...out.map((one) => one.z));
    const cx = (cols - 1) * STEP * 0.5;
    const cy = (rows - 1) * STEP * 0.5;
    const near = out
      .map((one, at) => ({ at, far: one.z === lift ? Math.hypot(one.x - cx, one.y - cy) : Infinity }))
      .sort((a, b) => a.far - b.far)[0].at;
    out.splice(near, 1);
  }
  return out.slice(0, LARGEST);
}

export function linksOf(tiles: readonly Spot[]): Links {
  const above = tiles.map((): number[] => []);
  const left = tiles.map((): number[] => []);
  const right = tiles.map((): number[] => []);
  for (const [a, one] of tiles.entries()) {
    for (const [b, other] of tiles.entries()) {
      if (a === b) continue;
      const crossY = Math.abs(one.y - other.y) < STEP;
      if (other.z === one.z + 1 && crossY && Math.abs(one.x - other.x) < STEP) above[a].push(b);
      if (other.z === one.z && crossY) {
        if (other.x + STEP === one.x) left[a].push(b);
        if (one.x + STEP === other.x) right[a].push(b);
      }
    }
  }
  return { above, left, right };
}

const freeIn = (links: Links, gone: (at: number) => boolean, at: number) => {
  if (gone(at) || links.above[at].some((b) => !gone(b))) return false;
  return links.left[at].every(gone) || links.right[at].every(gone);
};

export function freeTiles(state: Pick<State, "tiles">): boolean[] {
  const { tiles } = state;
  const links = linksOf(tiles);
  const gone = (at: number) => tiles[at].gone;
  return tiles.map((_, at) => freeIn(links, gone, at));
}

export function pairsOf(state: Pick<State, "tiles">): [number, number][] {
  const free = freeTiles(state);
  const seen = new Map<number, number[]>();
  const out: [number, number][] = [];
  for (const [at, one] of state.tiles.entries()) {
    if (!free[at]) continue;
    const list = seen.get(one.face) ?? [];
    for (const other of list) out.push([other, at]);
    seen.set(one.face, [...list, at]);
  }
  return out;
}

function laid(places: readonly Spot[], faces: readonly number[], seed: number): { tiles: Tile[]; seed: number } | null {
  const links = linksOf(places);
  const gone = new Array<boolean>(places.length).fill(false);
  const out = places.map((spot): Tile => ({ x: spot.x, y: spot.y, z: spot.z, face: -1, gone: false }));
  let at = seed;
  let left = places.length;
  const isGone = (i: number) => gone[i];
  for (const face of faces) {
    const open = places.flatMap((_, i) => (freeIn(links, isGone, i) ? [i] : []));
    if (open.length < 2) return null;
    const [first, a] = below(at, open.length);
    const one = open.splice(first, 1)[0];
    const [second, b] = below(a, open.length);
    const two = open[second];
    at = b;
    out[one].face = face;
    out[two].face = face;
    gone[one] = true;
    gone[two] = true;
    left -= 2;
  }
  return left === 0 ? { tiles: out, seed: at } : null;
}

function placed(
  places: readonly Spot[],
  faces: readonly number[],
  seed: number,
): { tiles: Tile[]; seed: number } | null {
  let at = seed;
  for (let i = 0; i < TRIES; i++) {
    const [order, next] = shuffled(at, faces);
    at = next;
    const done = laid(places, order, at);
    if (done) return done;
    at = mix((at + i + 1) >>> 0);
  }
  return null;
}

export function flatten(count: number, ratio: number): Spot[] {
  const across = Math.max(2, Math.ceil(Math.sqrt(count * ratio * TALL)));
  return Array.from({ length: count }, (_, i) => ({
    x: (i % across) * STEP,
    y: Math.floor(i / across) * STEP,
    z: 0,
  }));
}

function pairsFor(count: number, level: number): number[] {
  const pairs = count / 2;
  const fours = Math.min(Math.floor(pairs / 2), (level - 1) * 3);
  const single = pairs - 2 * fours;
  const faces: number[] = [];
  for (let i = 0; i < fours; i++) faces.push(i, i);
  for (let i = 0; i < single; i++) faces.push(fours + i);
  return faces;
}

function judged(state: State): State {
  const left = state.tiles.some((one) => !one.gone);
  if (!left) return { ...state, cleared: true, stuck: false, over: false };
  const stuck = pairsOf(state).length === 0;
  const over = stuck && state.shuffles === 0 && (state.undos === 0 || state.past.length === 0);
  return { ...state, cleared: false, stuck, over };
}

function board(level: number, seed: number, ratio: number): { tiles: Tile[]; deck: Face[]; seed: number } {
  let places = spots(level, ratio);
  const faces = pairsFor(places.length, level);
  const [deck, after] = deckFor(level, Math.max(...faces) + 1, seed);
  let made = placed(places, faces, after);
  if (!made) {
    places = flatten(places.length, ratio);
    made = placed(places, faces, after);
  }
  return { tiles: made!.tiles, deck, seed: made!.seed };
}

export function boot({ seed, ratio = 1.6, level = 1, score = 0 }: Config): State {
  const made = board(level, mix(seed), ratio);
  return judged({
    level,
    ratio,
    ...made,
    picked: -1,
    hint: [],
    past: [],
    score,
    streak: 0,
    hints: HINTS,
    undos: UNDOS,
    shuffles: SHUFFLES,
    cleared: false,
    stuck: false,
    over: false,
  });
}

export const multiplier = (streak: number) => Math.min(MOST, 1 + Math.floor(streak / BURN));

function match(state: State, a: number, b: number): State {
  const tiles = state.tiles.slice();
  tiles[a] = { ...tiles[a], gone: true };
  tiles[b] = { ...tiles[b], gone: true };
  const streak = state.streak + 1;
  const earned = POINTS * multiplier(streak);
  const left = tiles.some((one) => !one.gone);
  return judged({
    ...state,
    tiles,
    picked: -1,
    hint: [],
    past: [...state.past, { a, b, score: state.score, streak: state.streak }],
    streak,
    score: state.score + earned + (left ? 0 : CLEAR * state.level),
    hints: left ? state.hints : Math.min(TOP, state.hints + 1),
    undos: left ? state.undos : Math.min(TOP, state.undos + 1),
    shuffles: left || state.level % 2 ? state.shuffles : Math.min(TOP, state.shuffles + 1),
  });
}

function pick(state: State, at: number): State {
  if (state.cleared || !freeTiles(state)[at]) return state;
  if (state.picked === at) return { ...state, picked: -1 };
  if (state.picked < 0) return { ...state, picked: at };
  if (state.tiles[state.picked].face === state.tiles[at].face) return match(state, state.picked, at);
  return { ...state, picked: at, streak: 0 };
}

function reshuffle(state: State): State {
  if (state.shuffles < 1 || state.cleared) return state;
  const alive = state.tiles.flatMap((one, at) => (one.gone ? [] : [at]));
  const counts = new Map<number, number>();
  for (const at of alive) counts.set(state.tiles[at].face, (counts.get(state.tiles[at].face) ?? 0) + 1);
  const faces: number[] = [];
  for (const [face, count] of counts) for (let i = 0; i < count / 2; i++) faces.push(face);
  const held = alive.map((at) => state.tiles[at]);
  const made = placed(held, faces, state.seed) ?? placed(flatten(held.length, state.ratio), faces, state.seed);
  const tiles = state.tiles.slice();
  for (const [k, at] of alive.entries()) tiles[at] = made!.tiles[k];
  return judged({
    ...state,
    tiles,
    seed: made!.seed,
    shuffles: state.shuffles - 1,
    picked: -1,
    hint: [],
    past: [],
    streak: 0,
  });
}

function undo(state: State): State {
  const last = state.past.at(-1);
  if (!last || state.undos < 1 || state.cleared) return state;
  const tiles = state.tiles.slice();
  tiles[last.a] = { ...tiles[last.a], gone: false };
  tiles[last.b] = { ...tiles[last.b], gone: false };
  return judged({
    ...state,
    tiles,
    picked: -1,
    hint: [],
    past: state.past.slice(0, -1),
    score: last.score,
    streak: last.streak,
    undos: state.undos - 1,
  });
}

function hint(state: State): State {
  if (state.hints < 1 || state.cleared) return state;
  const [first] = pairsOf(state);
  if (!first) return state;
  return { ...state, hint: first, hints: state.hints - 1, streak: 0 };
}

function next(state: State): State {
  if (!state.cleared) return state;
  const level = state.level + 1;
  const made = board(level, mix((state.seed + level) >>> 0), state.ratio);
  return judged({ ...state, level, ...made, picked: -1, hint: [], past: [], streak: 0, cleared: false });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const free = freeTiles(state);
  return [
    ...state.tiles.flatMap((_, at): Act[] => (free[at] ? [{ verb: "pick", args: { at } }] : [])),
    { verb: "shuffle" },
    { verb: "undo" },
    { verb: "hint" },
    { verb: "next" },
  ];
}

export function step(state: State, act: Act): State {
  if (state.over) return state;
  if (act.verb === "pick") return pick(state, act.args.at);
  if (act.verb === "shuffle") return reshuffle(state);
  if (act.verb === "undo") return undo(state);
  if (act.verb === "hint") return hint(state);
  if (act.verb === "next") return next(state);
  return state;
}

export const deal = (
  _value: unknown,
  { rand, w = 1, h = 1, level, score }: { rand: Rng; w?: number; h?: number; level?: number; score?: number },
): State =>
  boot({ seed: Math.floor(rand() * WIDE), ratio: Math.min(MOST_RATIO, Math.max(1 / MOST_RATIO, w / h)), level, score });
