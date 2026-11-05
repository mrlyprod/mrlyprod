import type { bang, memory } from "@mrlyprod/math";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { canon, DESIGN } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Design } from "@mrlyprod/ui/kit/designs/store.ts";
import type { Had } from "@mrlyprod/ui/kit/hooks.ts";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { xorshift } from "@mrlyprod/ui/kit/rand.ts";
import type { Raw, Scalar, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Unit } from "@mrlyprod/ui/kit/units/index.ts";

export type Num = Unit<"num">;

export type Doors = Had<"math" | "num">;

export type Kind = "design" | "word" | "mosaic" | "hex" | "slice" | "rule";

export type Mode = "random" | "restricted" | "bruijn";

export type Pair = [number, number];

export type Table = { rand: Rng; units: Doors };

export type Box = [number, number, number, number];

export type Knobs = {
  view: Kind;
  side: number;
  word: string;
  letters: string;
  mosaic: string;
  hex: string;
  cut: string;
  game: string;
  rule: string;
  mode: Mode;
  weights: string;
  level: number;
  speed: number;
  grid: number;
};

export type Chaos = Knobs & Design;

export type Letter = { x: number; y: number; c: number };

export type Move = { letter: number; next: number };

export type Auto = { states: number; start: number; moves: Move[][]; tag: number[]; tags: number };

export type Hex = { ring: string; beta: Pair; norm: number; digits: Pair[]; units: Pair[]; box: Box; reach: number };

export type Fact = readonly [string, Scalar];

export type Source = {
  kind: Kind;
  base: number;
  letters: Letter[];
  hex: Hex | null;
  auto: Auto;
  cap: number;
  rho: number;
  dimension: number;
  title: string;
  facts: Fact[];
  line: number | null;
  error: string;
};

export type Law = {
  k: number;
  cells: number;
  tags: number;
  probs: Float64Array;
  support: Int32Array;
  total: number;
  lost: boolean;
};

export type Meter = {
  law: Law;
  hist: Uint32Array;
  counted: number;
  painted: number;
  outside: number;
  lap: number;
  laps: number;
  inLap: number;
  doubled: boolean;
  perfect: boolean;
  lapHits: Uint32Array | null;
  lapOutside: number;
};

export type Plane = { beta: string[]; digits: string[][]; units: string[][] };

export type State = {
  base: number;
  k: number;
  letters: number[][];
  moves: Move[][];
  hex: Plane | null;
  weighted: number;
  sched: string | null;
  pos: number;
  node: number;
  px: number;
  py: number;
  zx: string | null;
  zy: string | null;
  cx: number;
  cy: number;
  n: number;
  letter: number;
  moved: number;
  stuck: number;
  seed: number;
};

export type Config = Pick<
  State,
  "base" | "k" | "letters" | "moves" | "node" | "hex" | "weighted" | "sched" | "seed"
> & {
  start: string[];
};

export type Act = { verb: "draw" } | { verb: "take"; args: { j: number } };

export type Fit = { s: number; ox: number; oy: number };

export type Mosaic = { key: string; name: string; base: number; masks: [number, number][][] };

export type Game = { key: string; name: string; width: number; code: string };

type Kept = { math: Mrly; num: Num; source: Source };

export const DIM = 2;
export const LEVELS = 6;
export const BUDGET = 2 ** 20;
export const FRAME = 1000 / 60;
export const MOST = 2 ** 22;
export const CHECKS = 65536;
export const CUT = "2/3";
export const PAIR: [string, string] = ["7", "9"];
export const SPELL = 16;
export const NUMBERS = [2, 3, 5];
export const VIEWS: [Kind, string][] = [
  ["design", "Design"],
  ["word", "Word"],
  ["mosaic", "Mosaic"],
  ["hex", "Hex"],
  ["slice", "Slice"],
  ["rule", "Rule"],
];
export const MODES: [Mode, string][] = [
  ["random", "Random"],
  ["restricted", "Restricted"],
  ["bruijn", "De Bruijn"],
];
export const WEIGHTS: [string, string][] = [
  ["uniform", "Uniform"],
  ["weighted", "1, 2, 3, ..."],
];
export const SPEEDS: [number, string][] = [
  [1, "1"],
  [10, "10"],
  [100, "100"],
  [1000, "1000"],
];
export const WORDS: [string, string][] = [
  ["thue", "Thue-Morse"],
  ["periodic", "Periodic"],
  ["typed", "Typed"],
];
export const HEXES: [string, string][] = [
  ["gasket", "Gasket"],
  ["koch", "Koch curve"],
  ["terdragon", "Terdragon"],
  ["flowsnake", "Flowsnake"],
  ["twindragon", "Twindragon"],
];
export const MOSAICS: Mosaic[] = [
  {
    key: "golden",
    name: "Golden",
    base: 2,
    masks: [
      [
        [0, 0],
        [1, 1],
        [2, 0],
      ],
      [
        [0, 1],
        [3, 0],
      ],
    ],
  },
  {
    key: "twist",
    name: "Twist",
    base: 2,
    masks: [
      [
        [0, 0],
        [1, 1],
        [2, 1],
      ],
      [
        [1, 0],
        [2, 0],
        [3, 1],
      ],
    ],
  },
  {
    key: "ladder",
    name: "Ladder",
    base: 2,
    masks: [
      [
        [0, 0],
        [3, 0],
        [1, 1],
      ],
      [
        [0, 1],
        [3, 1],
        [2, 0],
      ],
    ],
  },
  {
    key: "relay",
    name: "Carpet relay",
    base: 3,
    masks: [
      [
        [0, 0],
        [2, 0],
        [6, 0],
        [8, 0],
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
      ],
      [
        [4, 0],
        [0, 1],
        [2, 1],
        [6, 1],
        [8, 1],
      ],
    ],
  },
];
const GAME_KEYS = ["repeat", "anticlockwise", "opposite", "neighbour", "far", "shift", "cycle", "pair"];
const TYPED: Game = { key: "typed", name: "Typed code", width: 2, code: "" };
let listed: Game[] | null = null;
export function games(math?: Mrly | null): Game[] {
  if (!math) return listed ?? [...GAME_KEYS.map((key) => ({ key, name: key, width: 2, code: "" })), TYPED];
  listed ??= [
    ...math.game.named_games().map((one, i) => ({
      key: GAME_KEYS[i] ?? one.name,
      name: one.name.charAt(0).toUpperCase() + one.name.slice(1),
      width: one.width,
      code: String(one.code),
    })),
    TYPED,
  ];
  return listed;
}
const SLOTS = "ABCDEFGH";
const GLANCE = 4096;
const TAIL = 1.5;
const SWEEPS = 20000;
const CLOSE = 1e-13;
const FINE: Record<number, number> = { 2: 52, 3: 32, 5: 21 };
const WIDE = 4294967296;
const SPARE = 0x9e3779b9;
const ZERO = 48;
const CHUNK = 8192;
export const DRAW: Act = { verb: "draw" };
export const taking = (j: number): Act => ({ verb: "take", args: { j } });

export const based = (value: Raw) => (tidy(DESIGN, value) as Design).base;

export function sides(value: Value): [number, string][] {
  const base = value.view === "word" ? 2 : based(value);
  return [[0, String(base)], ...NUMBERS.filter((n) => n > base).map((n): [number, string] => [n, String(n)])];
}

export const sideOf = (value: Value) =>
  Number(value.side) || (value.view === "word" ? 2 : value.view === "rule" ? 2 : based(value));

export const mosaicOf = (value: Value) => MOSAICS.find((one) => one.key === value.mosaic) ?? MOSAICS[0];

export const gameOf = (math: Mrly, value: Value) => games(math).find((one) => one.key === value.game) ?? games(math)[0];

export function binary(value: Value): boolean {
  const view = value.view as Kind;
  if (view === "rule") return true;
  if (view === "hex") return false;
  if (view === "mosaic") return mosaicOf(value).base === 2;
  if (view === "word") return sideOf(value) === 2;
  return based(value) === 2 && sideOf(value) === 2;
}

export const restricted = (value: Value) => value.view === "rule" || (value.mode === "restricted" && binary(value));

export function modes(value: Value): [Mode, string][] {
  const view = value.view as Kind;
  if (view === "rule") return MODES;
  return MODES.filter(
    ([mode]) => mode === "random" || (mode === "restricted" ? binary(value) : view === "design" || view === "hex"),
  );
}

export function modeOf(value: Value): Mode {
  if (value.view === "rule") return "restricted";
  const mode = value.mode as Mode;
  return modes(value).some(([one]) => one === mode) ? mode : "random";
}

export const weighted = (value: Value) => modeOf(value) === "random" && value.weights === "weighted";

export function cut(text: unknown): Pair {
  const hit = /^\s*(\d{1,6})\s*(?:\/\s*(\d{1,6}))?\s*$/.exec(String(text ?? ""));
  if (!hit) return cut(CUT);
  const p = Number(hit[1]);
  const q = Number(hit[2] ?? 1);
  return q > 0 && p / q > 0 && p / q < DIM ? [p, q] : cut(CUT);
}

export function spelt(text: unknown): number[] {
  const codes = String(text ?? "")
    .split(/\D+/)
    .filter(Boolean)
    .map(Number)
    .filter((code) => code >= 1 && code < 2 ** (2 ** DIM));
  return codes.length ? codes : PAIR.map(Number);
}

export function ruleCode(math: Mrly, value: Value): { width: number; code: bigint } {
  const game = gameOf(math, value);
  if (game.key !== "typed") return { width: game.width, code: BigInt(game.code) };
  const text = String(value.rule ?? "").trim();
  if (!/^\d{1,20}$/.test(text)) return { width: 2, code: BigInt(games(math)[0].code) };
  const code = BigInt(text);
  if (code < 2n ** 16n) return { width: 2, code };
  if (code < 2n ** 64n) return { width: 3, code };
  return { width: 2, code: BigInt(games(math)[0].code) };
}

export const corner = (base: number, c: number): Letter => ({ x: c % base, y: Math.floor(c / base), c });

export function digits(math: Mrly, code: string | number, side: number, base: number): Letter[] {
  const { types } = math.two.create(code, side, 1, 0, base);
  const out: Letter[] = [];
  for (let y = 0; y < side; y++)
    for (let x = 0; x < side; x++) if (types[y * side + x]) out.push({ x, y, c: x + side * y });
  return out;
}

export const single = (count: number): Auto => ({
  states: 1,
  start: 0,
  moves: [Array.from({ length: count }, (_, letter) => ({ letter, next: 0 }))],
  tag: [0],
  tags: 1,
});

export function graph(
  edges: [number, number, number][],
  states: number,
  letters: Letter[],
  start: number,
): { auto: Auto; dead: number[] } {
  const moves = Array.from({ length: states }, (_, v) =>
    edges
      .filter(([, , to]) => to === v)
      .map(([from, c]): Move => ({ letter: letters.findIndex((one) => one.c === c), next: from })),
  );
  const alive = moves.map((list) => list.length > 0);
  for (let again = true; again; ) {
    again = false;
    for (let s = 0; s < states; s++) {
      if (!alive[s] || moves[s].some((move) => alive[move.next])) continue;
      alive[s] = false;
      again = true;
    }
  }
  const kept = moves.map((list, s) => (alive[s] ? list.filter((move) => alive[move.next]) : []));
  const dead = alive.flatMap((on, s) => (on ? [] : [s]));
  const most = kept.reduce((best, list, s) => (list.length > kept[best].length ? s : best), 0);
  const first = alive[start] ? start : most;
  return {
    auto: { states, start: first, moves: kept, tag: Array.from({ length: states }, (_, s) => s), tags: states },
    dead,
  };
}

export function restrict(math: Mrly, auto: Auto, letters: Letter[], rule: memory.Rule): Auto {
  const memory = rule.width - 1;
  const tail = (h: number[]) => (memory ? h.slice(-memory) : []);
  const seen = new Map<string, number>();
  const list: [number, number[]][] = [];
  const id = (s: number, h: number[]) => {
    const key = `${s}:${h.join(",")}`;
    if (!seen.has(key)) {
      seen.set(key, list.length);
      list.push([s, h]);
    }
    return seen.get(key)!;
  };
  id(auto.start, []);
  const moves: Move[][] = [];
  const tag: number[] = [];
  for (let i = 0; i < list.length; i++) {
    const [s, h] = list[i];
    const allowed = new Set(math.game.successors(rule, h));
    tag[i] = auto.tag[s];
    moves[i] = auto.moves[s]
      .filter((move) => allowed.has(letters[move.letter].c))
      .map((move) => ({ letter: move.letter, next: id(move.next, tail([...h, letters[move.letter].c])) }));
  }
  return { states: list.length, start: 0, moves, tag, tags: auto.tags };
}

const place = (num: Num, ring: string, [a, c]: [bigint, bigint]): Pair => {
  const [x, y] = num.gauss.Ring.place(ring as "Gaussian" | "Eisenstein", a, c);
  return [x, y];
};

export const fixed = (x: number, places = 6) => Number(x.toFixed(places));

function named(math: Mrly, code: string, base: number) {
  const name = math.name.of(code, DIM, base);
  return name ? `${name} ${code}` : `code ${code}`;
}

function design(units: Doors, value: Chaos): Source {
  const { math } = units;
  const base = value.base;
  const code = canon(math, DIM, base, value.code);
  const side = sideOf(value);
  const letters = digits(math, code, side, base);
  const dimension = letters.length ? math.counts.dimension(code, side, DIM, base) : 0;
  return {
    kind: "design",
    base: side,
    letters,
    hex: null,
    auto: single(letters.length),
    cap: 0,
    rho: letters.length,
    dimension,
    title: `${named(math, code, base)}, side ${side}`,
    facts: [
      ["design", named(math, code, base)],
      ["digits", `${letters.length} of ${side * side} at side ${side}`],
      ["dimension", `${fixed(dimension)}, log ${letters.length} over log ${side}`],
    ],
    line: null,
    error: letters.length ? "" : "the design has no digit, so the game stands still",
  };
}

export function layers(math: Mrly, value: Value): { layers: bang.MagicLayer[]; codes: number[] } {
  const side = sideOf(value);
  const layer = (code: number) => {
    const one = new math.name.Bang(code, DIM, 2);
    const made = math.bang.MagicLayer.new(one, side);
    one.free();
    return made;
  };
  const kind = value.word;
  if (kind === "typed") {
    const codes = spelt(value.letters);
    return { layers: codes.map(layer), codes };
  }
  const pair = PAIR.map((code) => layer(Number(code))) as [bang.MagicLayer, bang.MagicLayer];
  const spelled =
    kind === "periodic" ? math.bang.word.spell("Periodic", pair, 2) : math.bang.word.spell("ThueMorse", pair, SPELL);
  return { layers: spelled, codes: spelled.map((one) => Number(one.design.code)) };
}

function word(units: Doors, value: Chaos): Source {
  const { math } = units;
  const side = sideOf(value);
  const { layers: made, codes } = layers(math, value);
  const period = math.bang.word.period(made);
  const sets = new Map<number, Letter[]>();
  for (const code of new Set(codes)) sets.set(code, digits(math, code, side, 2));
  const letters: Letter[] = [];
  const at = new Map<number, number>();
  for (const list of sets.values())
    for (const one of list)
      if (!at.has(one.c)) {
        at.set(one.c, letters.length);
        letters.push(one);
      }
  letters.sort((a, b) => a.c - b.c);
  for (const [i, one] of letters.entries()) at.set(one.c, i);
  const moves = Array.from({ length: period }, (_, r) =>
    sets.get(codes[r])!.map((one): Move => ({ letter: at.get(one.c)!, next: (r + 1) % period })),
  );
  const sizes = codes.slice(0, period).map((code) => sets.get(code)!.length);
  const dimension = math.bang.word.dimension(made);
  const empty = sizes.some((size) => size === 0);
  return {
    kind: "word",
    base: side,
    letters,
    hex: null,
    auto: { states: period, start: 0, moves, tag: Array.from({ length: period }, (_, r) => r), tags: period },
    cap: 0,
    rho: Math.exp(sizes.reduce((sum, size) => sum + Math.log(Math.max(size, 1)), 0) / period),
    dimension,
    title: `${WORDS.find(([key]) => key === value.word)?.[1] ?? "word"} ${codes.slice(0, period).join(" ")}, side ${side}`,
    facts: [
      ["word", codes.slice(0, period).join(" ")],
      ["period", period],
      ["letter sizes", sizes.join(", ")],
      ["dimension", fixed(dimension)],
    ],
    line: null,
    error: empty ? "a letter has no digit, so the game stands still at its phase" : "",
  };
}

function mosaic(units: Doors, value: Chaos): Source {
  const { math } = units;
  const preset = mosaicOf(value);
  const made = math.game.mosaic_game({ base: preset.base, dimension: DIM, masks: preset.masks });
  const letters = [...new Set(preset.masks.flat().map(([c]) => c))]
    .sort((a, b) => a - b)
    .map((c) => corner(preset.base, c));
  const { auto, dead } = graph(made.edges, preset.masks.length, letters, 0);
  const counts = preset.masks.map((_, slot) => Array.from(math.game.counts(made.matrix, slot, LEVELS), Number));
  return {
    kind: "mosaic",
    base: preset.base,
    letters,
    hex: null,
    auto,
    cap: 0,
    rho: made.rho,
    dimension: made.dimension,
    title: `mosaic ${preset.name.toLowerCase()}`,
    facts: [
      ["mosaic", `${preset.name}, base ${preset.base}, ${preset.masks.length} slots`],
      [
        "masks",
        preset.masks
          .map((mask, s) => `${SLOTS[s]}: ${mask.map(([c, to]) => `${c} to ${SLOTS[to]}`).join(", ")}`)
          .join("; "),
      ],
      ["matrix", made.matrix.map((row) => `[${row.join(", ")}]`).join(" ")],
      ["rho", fixed(made.rho)],
      ["dimension", fixed(made.dimension)],
      ["cells by level", counts.map((list, s) => `${SLOTS[s]}: ${list.join(", ")}`).join("; ")],
    ],
    line: null,
    error: dead.length ? `slot ${dead.map((s) => SLOTS[s]).join(", ")} has no way on, so the game leaves it out` : "",
  };
}

function slice(units: Doors, value: Chaos): Source {
  const { math } = units;
  const base = value.base;
  const code = canon(math, DIM, base, value.code);
  const corners = math.bang.code_to_corners(code, DIM, base).map(([y, x]) => x + base * y);
  const [p, q] = cut(value.cut);
  const made = math.game.slice_game(base, corners, p, q);
  const letters = corners.map((c) => corner(base, c));
  const { auto, dead } = made.states.length
    ? graph(made.edges, made.states.length, letters, made.start)
    : { auto: single(0), dead: [] };
  const heights = made.states.map(([a, b]) => (b === 1 ? String(a) : `${a}/${b}`));
  const counts = made.states.length ? Array.from(math.game.counts(made.matrix, made.start, LEVELS), Number) : [];
  const dimension = Number.isFinite(made.dimension) ? made.dimension : 0;
  return {
    kind: "slice",
    base,
    letters,
    hex: null,
    auto,
    cap: 0,
    rho: made.rho,
    dimension,
    title: `${named(math, code, base)} cut at ${p}/${q}`,
    facts: [
      ["design", named(math, code, base)],
      ["cut", `x + y = ${p}/${q}`],
      ["heights", heights.join(", ") || "none"],
      ["matrix", made.matrix.map((row) => `[${row.join(", ")}]`).join(" ") || "empty"],
      ["rho", fixed(made.rho)],
      ["dimension", fixed(dimension)],
      ["cells on the line by level", counts.join(", ") || "none"],
      ["dead ends", dead.map((s) => heights[s]).join(", ") || "none"],
    ],
    line: made.states.length ? made.states[auto.start][0] / made.states[auto.start][1] : null,
    error: !made.states.length
      ? "no cell meets this line, so the game stands still"
      : auto.moves[auto.start].length === 0
        ? "no height carries on, so the game stands still"
        : auto.start !== made.start
          ? `the height ${heights[made.start]} is a dead end, so the game starts at ${heights[auto.start]}`
          : "",
  };
}

function hexagonal(units: Doors, value: Chaos): Source {
  const { math, num } = units;
  const key = (HEXES.find(([one]) => one === value.hex) ?? HEXES[0])[0] as
    | "gasket"
    | "koch"
    | "terdragon"
    | "flowsnake"
    | "twindragon";
  const made = math.radix[key]();
  const ring = made.ring();
  const base = made.base();
  const value_ = base.value();
  const beta = place(num, ring, value_);
  const norm = Number(base.norm());
  const digitsOf = made.digits().map((z) => place(num, ring, z));
  const units_ = made.twists().map((z) => place(num, ring, z));
  const size = made.size();
  const dimension = made.dimension();
  const code = made.code();
  let level = 1;
  while (level < LEVELS * 2 && size ** (level + 1) <= GLANCE) level++;
  const pts = made.plane(level);
  const box: Box = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [x, y] of pts) {
    box[0] = Math.min(box[0], x);
    box[1] = Math.min(box[1], y);
    box[2] = Math.max(box[2], x);
    box[3] = Math.max(box[3], y);
  }
  const shrink = norm ** (-level / 2);
  const pad = (TAIL * Math.max(box[2] - box[0], box[3] - box[1]) * shrink) / (1 - shrink);
  box[0] -= pad;
  box[1] -= pad;
  box[2] += pad;
  box[3] += pad;
  base.free();
  made.free();
  const letters = digitsOf.map((_, i) => ({ x: i, y: 0, c: i }));
  const word = ring === "Eisenstein" ? "w" : "i";
  const spell = ([a, c]: [bigint, bigint]) =>
    c === 0n ? String(a) : a === 0n ? `${c === 1n ? "" : c}${word}` : `${a} + ${c === 1n ? "" : c}${word}`;
  const turned = units_.filter(([x, y]) => Math.abs(x - 1) > 1e-9 || Math.abs(y) > 1e-9).length;
  return {
    kind: "hex",
    base: Math.sqrt(norm),
    letters,
    hex: {
      ring,
      beta,
      norm,
      digits: digitsOf,
      units: units_,
      box,
      reach: Math.max(box[2] - box[0], box[3] - box[1]) / 2,
    },
    auto: single(size),
    cap: 0,
    rho: size,
    dimension,
    title: HEXES.find(([one]) => one === key)![1],
    facts: [
      ["radix design", `${HEXES.find(([one]) => one === key)![1]}, Z[${word}], base ${spell(value_)}, code ${code}`],
      ["digits", `${size}, norm ${norm}, ${turned ? `${turned} turned` : "no twist"}`],
      ["dimension", `${fixed(dimension)}, log ${size} over log sqrt ${norm}, the similarity dimension`],
    ],
    line: null,
    error: "",
  };
}

function ruled(units: Doors, value: Chaos): Source {
  const { math } = units;
  const { width, code } = ruleCode(math, value);
  const rule = new math.memory.Rule(DIM, width, code);
  const letters = Array.from({ length: 2 ** DIM }, (_, c) => corner(2, c));
  const auto = restrict(math, single(letters.length), letters, rule);
  const rho = math.memory.perron(rule);
  const exponent = math.memory.exponent(rule);
  const dimension = Number.isFinite(exponent) ? exponent : 0;
  const kappa = math.memory.kappa(rule);
  const windows = math.memory.allowed_windows(rule);
  const back = math.game.reverse(rule);
  const dial = Array.from({ length: LEVELS }, (_, i) => math.memory.cells(back, i + 1).length);
  back.free();
  rule.free();
  const game = gameOf(math, value);
  return {
    kind: "rule",
    base: 2,
    letters,
    hex: null,
    auto,
    cap: 0,
    rho,
    dimension,
    title: game.key === "typed" ? `rule ${code}` : game.name.toLowerCase(),
    facts: [
      ["rule", `${game.key === "typed" ? "typed code" : game.name.toLowerCase()} ${code}, width ${width}`],
      ["windows", `${windows} of ${2 ** (DIM * width)}`],
      ["rho", fixed(rho)],
      ["dimension", `${fixed(dimension)}, log2 rho`],
      ["kappa", fixed(kappa)],
      ["dial cells by level", dial.join(", ")],
    ],
    line: null,
    error: auto.moves[0].length ? "" : "no window is allowed, so the game stands still",
  };
}

const BUILD: Record<Kind, (units: Doors, value: Chaos) => Source> = {
  design,
  word,
  mosaic,
  hex: hexagonal,
  slice,
  rule: ruled,
};

export function cellsAt(source: Source, k: number) {
  return source.hex ? source.letters.length ** k : (source.base * source.base) ** k;
}

export function cap(source: Source): number {
  let k = 1;
  while (k < LEVELS && source.auto.states * cellsAt(source, k + 1) <= BUDGET) k++;
  return k;
}

const keyed = (value: Chaos): unknown[] => {
  const bound = restricted(value) ? [value.game, value.rule] : [];
  return {
    design: [value.base, value.code, value.side, ...bound],
    word: [value.word, value.letters, value.side, ...bound],
    mosaic: [value.mosaic, ...bound],
    hex: [value.hex],
    slice: [value.base, value.code, value.cut, ...bound],
    rule: [value.game, value.rule],
  }[value.view];
};

export function build(units: Doors, value: Chaos): Source {
  const kind = VIEWS.some(([one]) => one === value.view) ? value.view : "design";
  let source = BUILD[kind](units, value);
  if (kind !== "rule" && restricted(value)) {
    const { width, code } = ruleCode(units.math, value);
    const rule = new units.math.memory.Rule(DIM, width, code);
    const auto = restrict(units.math, source.auto, source.letters, rule);
    rule.free();
    const matrix = Array.from({ length: auto.states }, (_, s) =>
      Array.from({ length: auto.states }, (_, t) => auto.moves[s].filter((move) => move.next === t).length),
    );
    const rho = auto.states ? units.math.game.perron(matrix) : 0;
    const game = gameOf(units.math, value);
    source = {
      ...source,
      auto,
      rho,
      dimension: rho > 0 ? Math.log(rho) / Math.log(source.base) : 0,
      title: `${source.title} under ${game.key === "typed" ? `rule ${code}` : game.name.toLowerCase()}`,
      facts: [
        ...source.facts,
        ["rule", `${game.key === "typed" ? "typed code" : game.name.toLowerCase()} ${code}, width ${width}`],
        ["restricted rho", fixed(rho)],
        ["restricted dimension", fixed(rho > 0 ? Math.log(rho) / Math.log(source.base) : 0)],
      ],
      error: source.error || (auto.moves[0].length ? "" : "the rule allows no first draw, so the game stands still"),
    };
  }
  return { ...source, cap: cap(source) };
}

const kept = new Map<string, Kept>();

const KEEP = 16;

export function study(units: Doors, value: Chaos): Source {
  const key = JSON.stringify([value.view, keyed(value)]);
  const had = kept.get(key);
  if (had && had.math === units.math && had.num === units.num) {
    kept.delete(key);
    kept.set(key, had);
    return had.source;
  }
  const source = build(units, value);
  kept.set(key, { math: units.math, num: units.num, source });
  while (kept.size > KEEP) kept.delete(kept.keys().next().value!);
  return source;
}

export function odds(count: number, weighted: boolean): Float64Array {
  const out = new Float64Array(count);
  let sum = 0;
  for (let j = 0; j < count; j++) sum += weighted ? j + 1 : 1;
  for (let j = 0; j < count; j++) out[j] = (weighted ? j + 1 : 1) / sum;
  return out;
}

function reaches(auto: Auto, from: number): Uint8Array {
  const seen = new Uint8Array(auto.states);
  const queue = [from];
  seen[from] = 1;
  while (queue.length) {
    const s = queue.pop()!;
    for (const move of auto.moves[s])
      if (!seen[move.next]) {
        seen[move.next] = 1;
        queue.push(move.next);
      }
  }
  return seen;
}

export function stationary(auto: Auto, probs: Float64Array[]): Float64Array {
  const S = auto.states;
  let d = new Float64Array(S);
  d[auto.start] = 1;
  for (let sweep = 0; sweep < SWEEPS; sweep++) {
    const next = new Float64Array(S);
    for (let s = 0; s < S; s++) {
      if (!d[s]) continue;
      next[s] += d[s] / 2;
      const moves = auto.moves[s];
      for (let j = 0; j < moves.length; j++) next[moves[j].next] += (d[s] * probs[s][j]) / 2;
    }
    let diff = 0;
    for (let s = 0; s < S; s++) diff += Math.abs(next[s] - d[s]);
    d = next;
    if (diff < CLOSE) break;
  }
  const forward = Array.from({ length: S }, (_, s) => reaches(auto, s));
  for (let s = 0; s < S; s++) if (!forward[s].every((seen, t) => !seen || forward[t][s])) d[s] = 0;
  const mass = d.reduce((sum, p) => sum + p, 0);
  if (mass > 0) for (let s = 0; s < S; s++) d[s] /= mass;
  return d;
}

export function law(source: Source, k: number, weighted: boolean): Law {
  const { auto, base, hex, letters } = source;
  const S = auto.states;
  const T = auto.tags;
  const radix = hex ? letters.length : base * base;
  const cells = radix ** k;
  const probs = auto.moves.map((moves) => odds(moves.length, weighted));
  const pi = stationary(auto, probs);
  let cur = Float64Array.from(pi);
  for (let t = 0; t < k; t++) {
    const span = radix ** t;
    const unit = hex ? 1 : base ** t;
    const next = new Float64Array(S * span * radix);
    for (let c = 0; c < span; c++) {
      const cx = c % unit;
      const cy = Math.floor(c / unit);
      for (let s = 0; s < S; s++) {
        const p = cur[c * S + s];
        if (!p) continue;
        const moves = auto.moves[s];
        const odd = probs[s];
        for (let j = 0; j < moves.length; j++) {
          const { letter, next: to } = moves[j];
          const at = hex
            ? c + letter * span
            : cx + letters[letter].x * unit + (cy + letters[letter].y * unit) * unit * base;
          next[at * S + to] += p * odd[j];
        }
      }
    }
    cur = next;
  }
  const out = new Float64Array(cells * T);
  for (let c = 0; c < cells; c++) for (let s = 0; s < S; s++) out[c * T + auto.tag[s]] += cur[c * S + s];
  const found: number[] = [];
  let mass = 0;
  for (let i = 0; i < out.length; i++)
    if (out[i] > 0) {
      found.push(i);
      mass += out[i];
    }
  return { k, cells, tags: T, probs: out, support: Int32Array.from(found), total: found.length, lost: mass < 0.5 };
}

export function schedule(math: Mrly, source: Source, k: number): Uint8Array | null {
  if (source.auto.states !== 1 || !source.letters.length) return null;
  return math.game.de_bruijn(source.letters.length, k);
}

export function configOf(
  source: Source,
  k: number,
  weighted: boolean,
  sched: Uint8Array | null,
  start: Pair,
  seed: number,
): Config {
  const hex = source.hex;
  const put = ([x, y]: Pair) => [String(x), String(y)];
  let tape: string | null = null;
  if (sched) {
    tape = "";
    for (let i = 0; i < sched.length; i += CHUNK)
      tape += String.fromCharCode(...Array.from(sched.subarray(i, i + CHUNK), (j) => j + ZERO));
  }
  return {
    base: hex ? 0 : source.base,
    k,
    letters: source.letters.map(({ x, y }) => [x, y]),
    moves: source.auto.moves,
    node: source.auto.start,
    hex: hex ? { beta: put(hex.beta), digits: hex.digits.map(put), units: hex.units.map(put) } : null,
    weighted: weighted ? 1 : 0,
    sched: tape,
    start: put(start),
    seed,
  };
}

export function feed(seed: number) {
  let s = seed >>> 0 || SPARE;
  return {
    rand: () => {
      const r = s / WIDE;
      s = xorshift(s);
      return r;
    },
    get seed() {
      return s;
    },
  };
}

const fine = (base: number, less = 0) => base ** ((FINE[base] ?? 20) - less);

export function boot({ start, ...config }: Config): State {
  const { base, k } = config;
  const hex = config.hex !== null;
  const clamp = (v: string, side: number) => Math.min(side - 1, Math.max(0, Math.floor(Number(v) * side)));
  const side = base ** k;
  return {
    ...config,
    seed: config.seed >>> 0 || SPARE,
    pos: 0,
    px: hex ? 0 : clamp(start[0], fine(base)),
    py: hex ? 0 : clamp(start[1], fine(base)),
    zx: hex ? start[0] : null,
    zy: hex ? start[1] : null,
    cx: hex ? 0 : clamp(start[0], side),
    cy: hex ? 0 : clamp(start[1], side),
    n: 0,
    letter: -1,
    moved: -1,
    stuck: 0,
  };
}

const parsed = new WeakMap<object, number[][]>();

function plane(hex: Plane): number[][] {
  let out = parsed.get(hex);
  if (!out) {
    out = [hex.beta, ...hex.digits, ...hex.units].map(([x, y]) => [Number(x), Number(y)]);
    parsed.set(hex, out);
  }
  return out;
}

function take(state: State, j: number): State {
  const move = state.moves[state.node][j];
  if (!move) return state.stuck ? state : { ...state, stuck: 1 };
  const { base, k, letters, hex, sched } = state;
  const letter = move.letter;
  const next: State = {
    ...state,
    pos: sched ? (state.pos + 1) % sched.length : state.pos,
    node: move.next,
    letter,
    moved: j,
    n: state.n + 1,
  };
  if (hex) {
    const at = plane(hex);
    const m = letters.length;
    const [bx, by] = at[0];
    const [dx, dy] = at[1 + letter];
    const [ux, uy] = at[1 + m + letter];
    const zx = Number(state.zx);
    const zy = Number(state.zy);
    const ax = ux * zx - uy * zy + dx;
    const ay = ux * zy + uy * zx + dy;
    const den = bx * bx + by * by;
    next.zx = String((ax * bx + ay * by) / den);
    next.zy = String((ay * bx - ax * by) / den);
    next.cx = letter * m ** (k - 1) + Math.floor(state.cx / m);
    return next;
  }
  const [lx, ly] = letters[letter];
  const top = fine(base, 1);
  next.px = lx * top + Math.floor(state.px / base);
  next.py = ly * top + Math.floor(state.py / base);
  const edge = base ** (k - 1);
  next.cx = lx * edge + Math.floor(state.cx / base);
  next.cy = ly * edge + Math.floor(state.cy / base);
  return next;
}

function draw(state: State): State {
  const moves = state.moves[state.node];
  if (!moves.length) return state.stuck ? state : { ...state, stuck: 1 };
  if (state.sched) return take(state, state.sched.charCodeAt(state.pos) - ZERO);
  const r = state.seed / WIDE;
  const seeded = { ...state, seed: xorshift(state.seed) };
  if (!state.weighted) return take(seeded, Math.floor(r * moves.length));
  const sum = (moves.length * (moves.length + 1)) / 2;
  let j = 0;
  let acc = 1 / sum;
  while (j < moves.length - 1 && acc <= r) {
    j++;
    acc += (j + 1) / sum;
  }
  return take(seeded, j);
}

export function step(state: State, act: Act): State {
  if (act.verb === "draw") return draw(state);
  if (act.verb === "take") return take(state, act.args.j);
  return state;
}

export function moves(state: State): Act[] {
  const list = state.moves[state.node];
  return list.length ? [DRAW, ...list.map((_, j) => taking(j))] : [];
}

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

export const spot = (state: State): Pair =>
  state.hex ? [Number(state.zx), Number(state.zy)] : [state.px / fine(state.base), state.py / fine(state.base)];

export const cell = (state: State) => (state.hex ? state.cx : state.cx + state.cy * state.base ** state.k);

export const pair = (law: Law, cell: number, tag: number) => cell * law.tags + tag;

export function opening(math: Mrly, source: Source, value: Chaos, seed: number, from?: Pair) {
  const k = Math.min(value.level, source.cap);
  const heavy = weighted(value);
  const sched = modeOf(value) === "bruijn" ? schedule(math, source, k) : null;
  const tape = feed(seed);
  const start = from ?? origin(source, tape.rand);
  return { k, heavy, sched, start, state: boot(configOf(source, k, heavy, sched, start, tape.seed)) };
}

export const deal = (value: Chaos, { rand, units }: Table): State =>
  opening(units.math, study(units, value), value, Math.floor(rand() * WIDE)).state;

export function meter(law: Law, lap: number): Meter {
  return {
    law,
    hist: new Uint32Array(law.probs.length),
    counted: 0,
    painted: 0,
    outside: 0,
    lap,
    laps: 0,
    inLap: 0,
    doubled: false,
    perfect: lap > 0,
    lapHits: lap > 0 ? new Uint32Array(law.probs.length) : null,
    lapOutside: 0,
  };
}

export function hit(meter: Meter, at: number): void {
  const { law, hist, lapHits } = meter;
  const inLaw = law.probs[at] > 0;
  if (!hist[at]++ && inLaw) meter.painted++;
  if (!inLaw) meter.outside++;
  meter.counted++;
  if (!lapHits) return;
  if (lapHits[at]++) meter.doubled = true;
  if (++meter.inLap < meter.lap) return;
  meter.laps++;
  meter.perfect = meter.perfect && !meter.doubled && meter.lap === law.total && meter.outside === meter.lapOutside;
  lapHits.fill(0);
  meter.inLap = 0;
  meter.doubled = false;
  meter.lapOutside = meter.outside;
}

export function distance(meter: Meter): number {
  const { law, hist, counted, outside } = meter;
  if (!counted) return 1;
  let sum = outside / counted;
  for (const at of law.support) sum += Math.abs(hist[at] / counted - law.probs[at]);
  return sum / 2;
}

export function replay(state: State, made: Meter, tag: number[], log: Uint8Array, count: number): State {
  let at = state;
  for (let n = 1; n <= count; n++) {
    const next = take(at, log[n - 1]);
    if (next.n === at.n) return next;
    at = next;
    if (n >= made.law.k) hit(made, pair(made.law, cell(at), tag[at.node]));
  }
  return at;
}

export function csv(meter: Meter): string {
  const { law, hist } = meter;
  const rows = ["cell,tag,count,law"];
  for (let at = 0; at < hist.length; at++)
    if (hist[at] || law.probs[at] > 0)
      rows.push(`${Math.floor(at / law.tags)},${at % law.tags},${hist[at]},${law.probs[at].toPrecision(9)}`);
  return `${rows.join("\n")}\n`;
}

export const unit: Box = [0, 0, 1, 1];

export const boxOf = (source: Source): Box => source.hex?.box ?? unit;

export function fit(box: Box, R: number): Fit {
  const bw = box[2] - box[0] || 1;
  const bh = box[3] - box[1] || 1;
  const s = R / Math.max(bw, bh);
  return { s, ox: (R - bw * s) / 2 - box[0] * s, oy: R - (R - bh * s) / 2 + box[1] * s };
}

export const toCloud = (f: Fit, x: number, y: number): Pair => [f.ox + x * f.s, f.oy - y * f.s];

export const fromCloud = (f: Fit, cx: number, cy: number): Pair => [(cx - f.ox) / f.s, (f.oy - cy) / f.s];

export function lay(w: number, h: number, pad: number) {
  const size = Math.max(1, Math.min(w, h) - 2 * pad);
  return { x: (w - size) / 2, y: (h - size) / 2, size };
}

export function origin(source: Source, rand: Rng): Pair {
  if (source.hex) {
    const [x0, y0, x1, y1] = source.hex.box;
    return [x0 + rand() * (x1 - x0), y0 + rand() * (y1 - y0)];
  }
  const height = source.line;
  if (height !== null) {
    const lo = Math.max(0, height - 1);
    const hi = Math.min(1, height);
    if (lo < hi) {
      const t = lo + rand() * (hi - lo);
      return [t, Math.min(height - t, 1 - 1e-12)];
    }
  }
  return [rand(), rand()];
}
