export const BOARD = 96;
export const LINE = 320;
export const ROWS = 1024;
export const KEEP = 16;
export const LIMIT = 512;
export const SPANS = { 1: 81, 2: 27 };

const CODES = { 1: 4, 2: 16 };
const FIRST = { 1: 1, 2: 7 };

const ART = {
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

export const PATTERNS = Object.fromEntries(
  Object.entries(ART).map(([id, [name, rows, board = BOARD]]) => [
    id,
    {
      name,
      w: rows[0].length,
      h: rows.length,
      board,
      cells: rows.flatMap((row, y) => [...row].flatMap((ch, x) => (ch === "#" ? [[x, y]] : []))),
    },
  ]),
);

export const SEEDS = [
  ["noise", "Soup"],
  ["one", "One cell"],
  ["design", "Design"],
];

export const STARTS = [...SEEDS, ...Object.entries(PATTERNS).map(([id, { name }]) => [id, name])];

export const LINES = ["noise", "one"];

/* COUNTS */

export function parse(text, limit = 9999) {
  const clean = String(text ?? "").trim();
  const out = new Set();
  if (/^\d+$/.test(clean)) for (const ch of clean) out.add(+ch);
  else {
    for (const part of clean.match(/\d+(?:-\d+)?/g) ?? []) {
      const [a, b = a] = part.split("-").map(Number);
      for (let n = Math.min(a, b); n <= Math.min(limit, Math.max(a, b)); n++) out.add(n);
    }
  }
  return [...out].filter((n) => n <= limit).sort((a, b) => a - b);
}

export function spell(counts) {
  if (!counts.length) return "-";
  if (counts.every((n) => n < 10)) return counts.join("");
  const runs = [];
  for (const n of counts) {
    const last = runs[runs.length - 1];
    if (last && last[1] === n - 1) last[1] = n;
    else runs.push([n, n]);
  }
  return runs.map(([a, b]) => (a === b ? `${a}` : `${a}-${b}`)).join(",");
}

export function sources(life) {
  return life.Source.all()
    .filter((seq) => !seq.is_random())
    .map((seq) => seq.name());
}

export function drawn(life, text, limit) {
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

function side(life, text, limit) {
  const made = drawn(life, text, limit);
  if (made) return made;
  const counts = parse(text, limit);
  return { counts, name: spell(counts) };
}

export function ruleOf(life, value, cells) {
  const birth = side(life, value.born, cells);
  const survive = side(life, value.stay, cells);
  return { birth: birth.counts, survive: survive.counts, name: `B${birth.name}/S${survive.name}` };
}

/* MASK */

export const dimOf = (value) => (Number(value?.dim) === 1 ? 1 : 2);

export const codesOf = (dim) => Array.from({ length: CODES[dim] }, (_, i) => String(i));

export function codeOf(text, dim = 2) {
  const n = Number.parseInt(String(text), 10);
  return Number.isInteger(n) ? Math.min(CODES[dim] - 1, Math.max(0, n)) : FIRST[dim];
}

export function levelOf(number, level, cap) {
  let l = Math.max(1, Math.floor(Number(level) || 1));
  while (l > 1 && number ** l > cap) l--;
  return l;
}

export function fitLevel(number, cap) {
  let l = 1;
  while (number ** (l + 1) <= cap) l++;
  return l;
}

export function maskOf(life, value) {
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

export function maskName(math, code, dim = 2) {
  return math.name.of(code, dim, 2) ?? "";
}

export function lattice(index) {
  return index === 0 ? "none" : index === 1 ? "one" : `${index} copies`;
}

/* SEEDS */

function place(types, width, design, side, top, left) {
  for (let r = 0; r < side; r++)
    for (let c = 0; c < side; c++) types[(top + r) * width + left + c] = design[r * side + c];
}

export const startOf = (value) => (Number(value?.dim) === 1 && !LINES.includes(value.from) ? "noise" : value?.from);

export const sideOf = (value) => PATTERNS[startOf(value)]?.board ?? BOARD;

export function sow(value, rand, math, width, height = 1) {
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

export function population(types) {
  let n = 0;
  for (let i = 0; i < types.length; i++) n += types[i];
  return n;
}

export function hash(types) {
  let h = 2166136261;
  for (let i = 0; i < types.length; i++) h = Math.imul(h ^ types[i], 16777619);
  return h >>> 0;
}

export function fate(seen, h, pop) {
  const at = seen.lastIndexOf(h);
  const verdict = pop === 0 ? "dead" : at < 0 ? "" : at === seen.length - 1 ? "still" : `loop ${seen.length - at}`;
  seen.push(h);
  if (seen.length > KEEP) seen.shift();
  return verdict;
}

const ENDS = { Dead: "dies", Alive: "freezes", Loop: "loops", Timeout: "still moving" };

export function destiny(life, cell, mask, rule, wrap, limit = LIMIT) {
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

export function told({ fate: end, count, loop }) {
  return `${ENDS[end] ?? end} after ${count}${loop ? `, period ${loop}` : ""}`;
}
