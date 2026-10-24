import { tidy } from "../../lib/app/spec.js";
import { num as fix, runs, sheet } from "../../lib/app/svg.js";
import { canon, DESIGN, title, total } from "../../lib/designs/engine.js";

export const DIM = 2;
export const VIEWS = [
  ["word", "Word"],
  ["plane", "Lifts"],
  ["design", "Design"],
];
export const READS = [
  ["power", "Sign power"],
  ["filter", "Filter"],
];
export const PICTURES = ["word", "plane", "power", "filter"];
export const DESIGNED = ["power", "filter"];
export const GRIDS = ["plane", "power", "filter"];
export const KINDS = ["Parity", "And", "Xor", "Sum"];
export const FORMULA = { Parity: "t(i) xor t(j)", And: "t(i and j)", Xor: "t(i xor j)", Sum: "t(i + j)" };
export const LIFTS = KINDS.map((kind) => [kind, `${kind}, ${FORMULA[kind]}`]);
export const COLOURS = [
  ["letters", "Letters"],
  ["runs", "Runs"],
];
export const FOLDS = [
  ["sign", "Plus-minus, the exclusive or"],
  ["design", "The design, the and"],
];
export const NUMBERS = [2, 3, 5];
export const CELLS = [
  [0, "Fit"],
  [1, "1 px"],
  [2, "2 px"],
  [4, "4 px"],
  [8, "8 px"],
  [16, "16 px"],
];
export const ROUNDS = 10;
export const SIDE = 512;
export const FALLBACK = "9";
export const SCHEDULE = ["3", "7"];
export const RATES = 64;
export const PAD = 16;
export const GAP = 0.015;
export const UNIT = 10;
export const BUDGET = 65536;
export const ROW = 4;
const TRIES = 32;

export const PAINTS = { sign: ["plus", "minus"], diff: ["unlit", "diff"], runs: ["unlit", "one", "two"] };

export function cap(number) {
  let level = 1;
  while (number ** (level + 1) <= SIDE) level++;
  return level;
}

export const LEVELS = cap(2);

export const based = (value) => tidy(DESIGN, value).base;

export function sides(value) {
  const base = based(value);
  return [[0, String(base)], ...NUMBERS.filter((n) => n > base).map((n) => [n, String(n)])];
}

export const sideOf = (value) => value.number || based(value);

export const pictureOf = (value) => (value.view === "design" ? value.read : value.view);

export const place = (kind) => (DESIGNED.includes(kind) ? { view: "design", read: kind } : { view: kind });

/* CHOICE */

export function flat(types, side) {
  let rows = true;
  let cols = true;
  for (let r = 0; r < side && (rows || cols); r++) {
    for (let c = 0; c < side; c++) {
      if (types[r * side + c] !== types[c]) rows = false;
      if (types[r * side + c] !== types[r * side]) cols = false;
    }
  }
  return rows || cols;
}

export function roll(math, number, base, rand) {
  const count = total(math, DIM, base);
  for (let i = 0; i < TRIES; i++) {
    const code = 1 + Math.floor(rand() * (count - 2));
    if (!flat(math.two.create(code, number, 1, 0, base).types, number)) return String(code);
  }
  return FALLBACK;
}

export const coded = (math, value) => canon(math, DIM, value.base, value.code);

const named = (math, code, base = 2) => math.name.of(code, DIM, base) ?? "";

const tiles = new Map();

export function rendered(num, math, tile) {
  if (!tiles.has(math))
    tiles.set(
      math,
      Array.from({ length: total(math, DIM, 2) }, (_, code) =>
        Array.from(num.morse.signs(math.two.create(String(code), 2, 1, 0, 2).types)).join(""),
      ),
    );
  const at = tiles.get(math).indexOf(Array.from(tile).join(""));
  return at < 0 ? "" : String(at);
}

const schedules = new Map();

export function schedule(math) {
  if (schedules.has(math)) return schedules.get(math);
  const word = math.bang.word;
  const bangs = SCHEDULE.map((code) => new math.name.Bang(code, DIM, 2));
  const pair = bangs.map((bang) => math.bang.MagicLayer.new(bang, 2));
  for (const bang of bangs) bang.free();
  const rates = (kind) => word.rates(word.spell(kind, pair, RATES));
  const thue = rates("ThueMorse");
  const [rate, exponent] = thue.at(-1);
  const [control] = rates("Periodic").at(-1);
  const read = {
    zero: named(math, SCHEDULE[0]) || SCHEDULE[0],
    one: named(math, SCHEDULE[1]) || SCHEDULE[1],
    exponent,
    rate,
    control,
    prefix: thue.length,
  };
  schedules.set(math, read);
  return read;
}

/* STUDY */

let kept = null;

const keyed = (value, code) =>
  ({
    word: [value.round, value.colour],
    plane: [value.level, value.lift],
    power: [code, value.number, value.base, value.power],
    filter: [code, value.number, value.base, value.depth, value.fold],
  })[pictureOf(value)];

const read = (units, given) => {
  const value = { ...given, number: sideOf(given) };
  const kind = pictureOf(value);
  return { value, kind, code: DESIGNED.includes(kind) ? coded(units.math, value) : "" };
};

export function survey(units, given) {
  const { value, kind, code } = read(units, given);
  return SURVEYS[kind](units, value, code);
}

export function study(units, given) {
  const { value, kind, code } = read(units, given);
  const key = JSON.stringify([kind, keyed(value, code)]);
  if (kept && kept.num === units.num && kept.math === units.math && kept.key === key) return kept.plan;
  const plan = SURVEYS[kind](units, value, code);
  kept = { num: units.num, math: units.math, key, plan };
  return plan;
}

const count = (grid, bit) => {
  let n = 0;
  for (let i = 0; i < grid.length; i++) if (grid[i] === bit) n++;
  return n;
};

const tally = (runs) => {
  const read = { runs: runs.length, longest: 0, singles: 0, doubles: 0 };
  for (const run of runs) {
    read.longest = Math.max(read.longest, run);
    if (run === 1) read.singles++;
    if (run === 2) read.doubles++;
  }
  return read;
};

export const print = (tile, side) =>
  Array.from({ length: side }, (_, r) =>
    Array.from(tile.slice(r * side, (r + 1) * side), (bit) => (bit ? "-" : "+")).join(""),
  ).join("/");

const filled = (types) => Uint8Array.from(types, (bit) => (bit ? 1 : 0));

export function spread(lengths, length) {
  const out = new Uint8Array(length);
  let at = 0;
  for (const run of lengths) {
    out.fill(run, at, at + run);
    at += run;
  }
  return out;
}

const SURVEYS = {
  word: ({ num, math }, { round, colour }) => {
    const m = num.morse;
    const stages = Array.from({ length: round }, (_, k) => m.stage(k + 1));
    const stage = stages.at(-1);
    const byRuns = colour === "runs";
    const rows = byRuns ? stages.map((letters) => spread(m.runs(letters), letters.length)) : stages;
    const boundary = m.boundary(stage);
    const read = tally(m.runs(stage));
    const facts = {
      view: "word",
      round,
      colour,
      letters: stage.length,
      plus: count(stage, 0),
      minus: count(stage, 1),
      agree: m.faults(stage, m.digits(stage.length)) === 0,
      ...read,
      ones: count(stage, 1),
      cube: read.longest <= 2,
      doubling: m.faults(boundary, m.doubling(boundary.length)) === 0,
      ...schedule(math),
    };
    return { view: "word", side: 0, types: null, paint: byRuns ? "runs" : "sign", rows, facts };
  },
  plane: ({ num, math }, { level, lift }) => {
    const m = num.morse;
    const side = 2 ** level;
    const grid = m.lift(lift, side);
    const read = m.fold(grid, side, 2);
    const code = read.folds ? rendered(num, math, read.tile) : "";
    const twin = lift !== KINDS[0] && m.faults(grid, m.lift(KINDS[0], side)) === 0 ? KINDS[0] : "";
    const facts = {
      view: "plane",
      level,
      side,
      cells: side * side,
      kind: lift,
      formula: m.Lift.formula(lift),
      folds: read.folds,
      faults: read.faults,
      first: read.first ?? null,
      tile: print(read.tile, 2),
      twin,
      code,
      name: code ? named(math, code) || code : "",
      plus: count(grid, 0),
      minus: count(grid, 1),
    };
    return { view: "plane", side, types: grid, paint: "sign", rows: [], facts };
  },
  power: ({ num, math }, { power: level, number, base }, code) => {
    const m = num.morse;
    const tile = math.two.create(code, number, 1, 0, base);
    const design = math.two.create(code, number, level, 0, base);
    const side = design.shape[0];
    const cells = side * side;
    const sign = m.signs(tile.types);
    const power = m.power(sign, number, level);
    const types = filled(design.types);
    const agree = m.faults(types, power);
    const exact = number === 2 && m.faults(power, m.lift(KINDS[0], side)) === 0;
    const facts = {
      view: "power",
      level,
      code,
      name: named(math, code, base),
      title: title(math, DIM, base, code),
      number,
      base,
      side,
      cells,
      filled: count(types, 1),
      plus: count(power, 0),
      agree,
      differ: cells - agree,
      tile: print(sign, number),
      exact,
    };
    return { view: "power", side, types: power, paint: "sign", rows: [], facts };
  },
  filter: ({ num, math }, { depth: level, number, base, fold }, code) => {
    const m = num.morse;
    const signed = fold === "sign";
    const tile = math.two.create(code, number, 1, 0, base);
    const sign = m.signs(tile.types);
    const grown = (k) => (signed ? m.power(sign, number, k) : filled(math.two.create(code, number, k, 0, base).types));
    const low = grown(level);
    const wide = m.upsample(low, number ** level, number);
    const high = grown(level + 1);
    const diff = m.difference(wide, high);
    const side = number ** (level + 1);
    const cells = side * side;
    const differ = count(diff, 1);
    const tiled = m.repeat(sign, number, side);
    const closed = m.faults(diff, signed ? tiled : m.mask(wide, tiled)) === 0;
    const morse = number === 2 ? m.faults(diff, m.lift(KINDS[0], side)) : null;
    const form = signed ? "the tile repeated" : "the blow-up, masked by the tile complement";
    const facts = {
      view: "filter",
      level,
      fold,
      code,
      name: named(math, code, base),
      title: title(math, DIM, base, code),
      number,
      base,
      side,
      cells,
      differ,
      closed,
      form,
      morse,
      exact: morse === 0,
      half: morse !== null && morse * 2 === cells,
      share: morse === null ? null : morse / cells,
      kin: number === 2 && m.faults(sign, m.lift(KINDS[0], 2)) === 0,
      tile: print(sign, number),
    };
    return { view: "filter", side, types: diff, paint: signed ? "sign" : "diff", rows: [], facts };
  },
};

/* LAYOUT */

export function lay(w, h, side, cell, pad) {
  const most = Math.max(1, Math.floor(Math.min(w, h) - 2 * pad));
  const size = cell > 0 ? cell * side : most >= side ? Math.floor(most / side) * side : most;
  return { x: Math.round((w - size) / 2), y: Math.round((h - size) / 2), size };
}

export function tower(w, h, rows, dpr, pad = PAD * dpr) {
  const gap = rows.length > 1 ? Math.max(1, Math.round(h * GAP)) : 0;
  const tall = (h - 2 * pad - (rows.length - 1) * gap) / rows.length;
  return rows.map((letters, k) => {
    const step = (w - 2 * pad) / letters.length;
    const top = Math.round(pad + k * (tall + gap));
    return {
      x: pad,
      y: top,
      h: Math.max(1, Math.round(pad + k * (tall + gap) + tall) - top),
      step,
      gap: step >= 3 * dpr ? Math.max(1, Math.round(dpr)) : 0,
    };
  });
}

/* FILES */

const layer = (types, rows, cols, dx, dy, w, h, paint, inks) =>
  PAINTS[paint]
    .map((ink, v) => {
      const mask = Uint8Array.from(types, (bit) => (bit === v ? 1 : 0));
      const body = runs({ shape: [rows, cols], types: mask })
        .map(
          ([row, col, len]) =>
            `<rect x="${fix(dx + col * w)}" y="${fix(dy + row * h)}" width="${fix(len * w)}" height="${fix(h)}"/>`,
        )
        .join("");
      return body ? `<g fill="${inks[ink]}" shape-rendering="crispEdges">${body}</g>` : "";
    })
    .join("");

export const fits = (plan) => plan.rows.reduce((sum, row) => sum + row.length, 0) + plan.side * plan.side <= BUDGET;

export function picture(plan, inks) {
  const { side, types, rows, paint } = plan;
  if (rows.length) {
    const width = UNIT * rows.at(-1).length;
    const tall = UNIT * ROW;
    const body = rows
      .map((letters, k) =>
        layer(letters, 1, letters.length, 0, k * (tall + UNIT), width / letters.length, tall, paint, inks),
      )
      .join("");
    return sheet(width, rows.length * tall + (rows.length - 1) * UNIT, body);
  }
  const size = side * UNIT;
  return sheet(size, size, layer(types, side, side, 0, 0, UNIT, UNIT, paint, inks));
}
