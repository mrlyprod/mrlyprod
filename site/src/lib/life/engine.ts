import type { Cell, Fate, Tensor } from "@mrlyprod/mrly/life";
import type { Scalar, Value } from "../app/spec.ts";
import type { Mrly } from "../designs/engine.ts";
import type { Rng } from "../rand.ts";

/* TYPES */

export type Door = typeof import("@mrlyprod/mrly/life");

export type Dim = 1 | 2;

export type Point = [number, number];

type Art = [string, string[], number?];

export type Pattern = { name: string; w: number; h: number; board: number; cells: Point[] };

export type Start = [string, string];

export type Side = { counts: number[]; name: string };

export type Rule = { birth: number[]; survive: number[]; name: string };

export type Shape = {
  dim: Dim;
  code: number;
  number: number;
  level: number;
  span: number;
  mask: Tensor;
  cells: number;
  lattice: number;
};

export type Ending = { fate: Fate; count: number; loop: number };

export const BOARD = 96;
export const LINE = 320;
export const ROWS = 1024;
export const KEEP = 16;
export const LIMIT = 512;
export const SPANS: Record<Dim, number> = { 1: 81, 2: 27 };

const CODES: Record<Dim, number> = { 1: 4, 2: 16 };
const FIRST: Record<Dim, number> = { 1: 1, 2: 7 };

const ART: Record<string, Art> = {
  glider: ["Glider", [".#.", "..#", "###"], 32],
  lwss: ["LWSS", [".#..#", "#....", "#...#", "####."], 32],
  pentomino: ["R-pentomino", [".##", "##.", ".#."]],
  acorn: ["Acorn", [".#.....", "...#...", "##..###"]],
  diehard: ["Diehard", ["......#.", "##......", ".#...###"]],
  gun: [
    "Gosper gun",
    [
      "........................#...........",
      "......................#.#...........",
      "............##......##............##",
      "...........#...#....##............##",
      "##........#.....#...##..............",
      "##........#...#.##....#.#...........",
      "..........#.....#.......#...........",
      "...........#...#....................",
      "............##......................",
    ],
  ],
  blinker: ["Blinker", ["###"], 16],
  toad: ["Toad", [".###", "###."], 16],
  beacon: ["Beacon", ["##..", "##..", "..##", "..##"], 16],
  pulsar: [
    "Pulsar",
    [
      "..###...###..",
      ".............",
      "#....#.#....#",
      "#....#.#....#",
      "#....#.#....#",
      "..###...###..",
      ".............",
      "..###...###..",
      "#....#.#....#",
      "#....#.#....#",
      "#....#.#....#",
      ".............",
      "..###...###..",
    ],
    32,
  ],
  block: ["Block", ["##", "##"], 16],
  beehive: ["Beehive", [".##.", "#..#", ".##."], 16],
};

export const PATTERNS: Record<string, Pattern> = Object.fromEntries(
  Object.entries(ART).map(([id, [name, rows, board = BOARD]]): [string, Pattern] => [
    id,
    {
      name,
      w: rows[0].length,
      h: rows.length,
      board,
      cells: rows.flatMap((row, y) => [...row].flatMap((ch, x): Point[] => (ch === "#" ? [[x, y]] : []))),
    },
  ]),
);

export const SEEDS: Start[] = [
  ["noise", "Soup"],
  ["one", "One cell"],
  ["design", "Design"],
];

export const STARTS: Start[] = [...SEEDS, ...Object.entries(PATTERNS).map(([id, { name }]): Start => [id, name])];

export const LINES: Scalar[] = ["noise", "one"];

/* COUNTS */

export function parse(text: Scalar | null | undefined, limit = 9999): number[] {
  const clean = String(text ?? "").trim();
  const out = new Set<number>();
  if (/^\d+$/.test(clean)) for (const ch of clean) out.add(+ch);
  else {
    for (const part of clean.match(/\d+(?:-\d+)?/g) ?? []) {
      const [a, b = a] = part.split("-").map(Number);
      for (let n = Math.min(a, b); n <= Math.min(limit, Math.max(a, b)); n++) out.add(n);
    }
  }
  return [...out].filter((n) => n <= limit).sort((a, b) => a - b);
}

export function spell(counts: readonly number[]): string {
  if (!counts.length) return "-";
  if (counts.every((n) => n < 10)) return counts.join("");
  const runs: [number, number][] = [];
  for (const n of counts) {
    const last = runs[runs.length - 1];
    if (last && last[1] === n - 1) last[1] = n;
    else runs.push([n, n]);
  }
  return runs.map(([a, b]) => (a === b ? `${a}` : `${a}-${b}`)).join(",");
}

export function sources(life: Door): string[] {
  return life.Source.all()
    .filter((seq) => !seq.is_random())
    .map((seq) => seq.name());
}

export function drawn(life: Door, text: Scalar | null | undefined, limit: number): Side | null {
  try {
    const seq = life.Source.parse(
      String(text ?? "")
        .trim()
        .toLowerCase(),
    );
    const name = seq.name();
    const counts = life.Counts.drawn(seq, false, false);
    const list = Array.from(counts.values(limit));
    counts.free();
    return { counts: list, name };
  } catch {
    return null;
  }
}

function side(life: Door, text: Scalar | null | undefined, limit: number): Side {
  const made = drawn(life, text, limit);
  if (made) return made;
  const counts = parse(text, limit);
  return { counts, name: spell(counts) };
}

export function ruleOf(life: Door, value: Value, cells: number): Rule {
  const birth = side(life, value.born, cells);
  const survive = side(life, value.stay, cells);
  return { birth: birth.counts, survive: survive.counts, name: `B${birth.name}/S${survive.name}` };
}

/* MASK */

export const dimOf = (value: Value) => (Number(value?.dim) === 1 ? 1 : 2);

export const codesOf = (dim: Dim) => Array.from({ length: CODES[dim] }, (_, i) => String(i));

export function codeOf(text: Scalar, dim: Dim = 2): number {
  const n = Number.parseInt(String(text), 10);
  return Number.isInteger(n) ? Math.min(CODES[dim] - 1, Math.max(0, n)) : FIRST[dim];
}

export function levelOf(number: number, level: Scalar, cap: number): number {
  let l = Math.max(1, Math.floor(Number(level) || 1));
  while (l > 1 && number ** l > cap) l--;
  return l;
}

export function fitLevel(number: number, cap: number): number {
  let l = 1;
  while (number ** (l + 1) <= cap) l++;
  return l;
}

export function maskOf(life: Door, value: Value): Shape {
  const dim = dimOf(value);
  const code = codeOf(value.code, dim);
  const number = Number(value.number) || 3;
  const level = levelOf(number, value.level, SPANS[dim]);
  const made = life.design_mask(dim, code, number, level);
  const mask = dim === 1 ? { shape: [1, made.shape[0]], data: made.data } : made;
  let cells = 0;
  for (const bit of mask.data) cells += bit;
  return { dim, code, number, level, span: mask.shape[1], mask, cells, lattice: life.lattice_index(made) };
}

export function maskName(math: Mrly, code: number, dim = 2): string {
  return math.name.of(code, dim, 2) ?? "";
}

export function lattice(index: number): string {
  return index === 0 ? "none" : index === 1 ? "one" : `${index} copies`;
}

/* SEEDS */

function place(types: Uint8Array, width: number, design: ArrayLike<number>, side: number, top: number, left: number) {
  for (let r = 0; r < side; r++)
    for (let c = 0; c < side; c++) types[(top + r) * width + left + c] = design[r * side + c];
}

export const startOf = (value: Value) =>
  Number(value?.dim) === 1 && !LINES.includes(value.from) ? "noise" : value?.from;

export const sideOf = (value: Value) => PATTERNS[startOf(value)]?.board ?? BOARD;

export function sow(value: Value, rand: Rng, math: Mrly, width: number, height = 1): Uint8Array {
  const types = new Uint8Array(width * height);
  const from = height === 1 && PATTERNS[value.from] ? "noise" : value.from;
  const pattern = PATTERNS[from];
  if (from === "one") types[(height >> 1) * width + (width >> 1)] = 1;
  else if (pattern) {
    const left = (width - pattern.w + 1) >> 1;
    const top = (height - pattern.h + 1) >> 1;
    for (const [x, y] of pattern.cells) types[(top + y) * width + left + x] = 1;
  } else if (from === "design") {
    const number = Number(value.number) || 3;
    const level = fitLevel(number, Math.min(width, height > 1 ? height : width));
    const side = number ** level;
    const design = math.two.create(codeOf(value.code), number, level, 0, 2).types;
    const left = (width - side) >> 1;
    if (height > 1) place(types, width, design, side, (height - side) >> 1, left);
    else types.set(design.subarray((side >> 1) * side, (side >> 1) * side + side), left);
  } else {
    const density = Number(value.density);
    for (let i = 0; i < types.length; i++) types[i] = rand() < density ? 1 : 0;
  }
  return types;
}

/* FATE */

export function population(types: ArrayLike<number>): number {
  let n = 0;
  for (let i = 0; i < types.length; i++) n += types[i];
  return n;
}

export function hash(types: ArrayLike<number>): number {
  let h = 2166136261;
  for (let i = 0; i < types.length; i++) h = Math.imul(h ^ types[i], 16777619);
  return h >>> 0;
}

export function fate(seen: number[], h: number, pop: number): string {
  const at = seen.lastIndexOf(h);
  const verdict = pop === 0 ? "dead" : at < 0 ? "" : at === seen.length - 1 ? "still" : `loop ${seen.length - at}`;
  seen.push(h);
  if (seen.length > KEEP) seen.shift();
  return verdict;
}

const ENDS: Record<Fate, string> = { Dead: "dies", Alive: "freezes", Loop: "loops", Timeout: "still moving" };

export function destiny(
  life: Door,
  cell: Cell,
  mask: Tensor,
  rule: Pick<Rule, "birth" | "survive">,
  wrap: Scalar | boolean,
  limit = LIMIT,
): Ending {
  const config = new life.Config(
    { shape: mask.shape, types: mask.data },
    life.Counts.list(rule.birth),
    life.Counts.list(rule.survive),
  );
  config.boundary = wrap ? "Wrap" : "Constant";
  config.max_generations = limit;
  const run = life.animate(cell, config);
  const read = { fate: run.fate, count: run.count, loop: run.loop_length };
  run.free();
  config.free();
  return read;
}

export function told({ fate: end, count, loop }: Ending): string {
  return `${ENDS[end] ?? end} after ${count}${loop ? `, period ${loop}` : ""}`;
}
