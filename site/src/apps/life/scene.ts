import type { Cell, Tensor } from "@mrlyprod/mrly/life";
import type { Ready } from "../../lib/app/hooks.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Board, Opts, View } from "../../lib/app/run.ts";
import { board } from "../../lib/app/run.ts";
import type { Option, Row, Scalar, Units, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { veil } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Door, Rule, Shape } from "../../lib/life/engine.ts";
import {
  BOARD,
  destiny,
  dimOf,
  fate,
  hash,
  LIMIT,
  LINE,
  LINES,
  lattice,
  maskName,
  maskOf,
  population,
  ROWS,
  ruleOf,
  SPANS,
  STARTS,
  sideOf,
  sow,
  told,
} from "../../lib/life/engine.ts";
import { PANE, runs, SOUP } from "../../lib/life/scene.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick, rng } from "../../lib/rand.ts";
import { wasm } from "../../lib/units/index.ts";

/* TYPES */

export type Knobs = {
  dim: number;
  code: string;
  number: number;
  level: number;
  born: string;
  stay: string;
  wrap: number;
  from: string;
  density: number;
  speed: number;
  seed?: number;
};

export type Preset = [string, Value];

export type Stat = {
  gen: number;
  pop: number;
  fate: string;
  name: string;
  counts: number[][];
  mask: Tensor;
  cells: number;
  lattice: string;
  label: string;
  error: string;
};

export type Given = Opts & { life: Door; math: Mrly; onStat?: (stat: Stat) => void };

export type Life = ReturnType<typeof make>;

type Rows = [Row & { options: readonly Option[] }, ...Row[]];

type Mould = Shape & { named?: string; label?: string };

type Lay = Board & { rows?: number };

type Skin = { ink: string; dim: string };

const NUMBERS = [3, 5, 7, 9].map((n): Option => [n, String(n)]);

const LEVELS = [1, 2, 3];

const START = "glider";

export const SPEC: Rows = [
  {
    key: "dim",
    label: "Dim",
    kind: "segment",
    def: 2,
    options: [
      [1, "1D"],
      [2, "2D"],
    ],
    group: "Mask",
    when: PANE,
  },
  { key: "code", label: "Code", kind: "text", def: "7", group: "Mask", when: PANE },
  { key: "number", label: "Side", kind: "pick", def: 3, options: NUMBERS, group: "Mask", when: PANE },
  {
    key: "level",
    label: "Level",
    kind: "pick",
    def: 1,
    options: (value) =>
      LEVELS.filter((n) => (value.number as number) ** n <= SPANS[dimOf(value)]).map((n): Option => [n, String(n)]),
    group: "Mask",
    when: PANE,
  },
  { key: "born", label: "Born", kind: "text", def: "3", group: "Rule", when: PANE },
  { key: "stay", label: "Stay", kind: "text", def: "23", group: "Rule", when: PANE },
  { key: "wrap", label: "Wrap", kind: "toggle", def: 1, group: "Rule", when: PANE },
  {
    key: "from",
    label: "Start",
    kind: "pick",
    def: START,
    options: (value) => (dimOf(value) === 1 ? STARTS.filter(([one]) => LINES.includes(one)) : STARTS),
    group: "Board",
    when: PANE,
  },
  {
    key: "density",
    label: "Density",
    kind: "slider",
    def: 0.3,
    min: 0.05,
    max: 0.95,
    step: 0.01,
    group: "Board",
    when: PANE,
  },
  {
    key: "speed",
    label: "Speed",
    kind: "slider",
    def: 25,
    min: 1,
    max: 60,
    step: 1,
    unit: "/s",
    group: "Board",
    when: PANE,
  },
];

export const RULES: Preset[] = [
  ["Conway", { born: "3", stay: "23" }],
  ["HighLife", { born: "36", stay: "23" }],
  ["Day and Night", { born: "3678", stay: "34678" }],
  ["Seeds", { born: "2", stay: "-" }],
  ["Life without Death", { born: "3", stay: "012345678" }],
  ["Maze", { born: "3", stay: "12345" }],
  ["Replicator", { born: "1357", stay: "1357" }],
  ["2x2", { born: "36", stay: "125" }],
  ["Diamoeba", { born: "35678", stay: "5678" }],
];

export const PRESETS: Preset[] = [
  ["Conway", { dim: 2, code: "7", number: 3, level: 1, born: "3", stay: "23" }],
  ["Cantor-Life", { dim: 1, code: "1", number: 3, level: 3, born: "3", stay: "23" }],
  ["Menger row", { dim: 1, code: "3", number: 3, level: 2, born: "3", stay: "23" }],
  ["Nine-cell XOR", { dim: 2, code: "15", number: 3, level: 1, born: "1357", stay: "02468" }],
  ["Rule 150", { dim: 1, code: "1", number: 3, level: 1, born: "1", stay: "02" }],
];

export const wears = (value: Value, preset: Value) =>
  Object.entries(preset).every(([key, v]) => String(value[key]) === String(v));

const slug = (text: Scalar) =>
  String(text)
    .toLowerCase()
    .replace(/[^a-z0-9.]+/g, "-")
    .replace(/^-+|-+$/g, "");

export const worn = (presets: readonly Preset[], value: Value) =>
  presets.find(([, one]) => wears(value, one))?.[0] ?? "";

export const PAGE = {
  spec: SPEC,
  design: { dim: "dim", base: 2, code: "code", dims: [1, 2] },
  gestures: true,
  record: true,
  about: [
    "Conway's Life is one rule on the eight cells around a cell: a dead cell with three live neighbours is born, a live cell with two or three stays, and every other cell dies.",
    "Here the neighbourhood is a design, picked with the design chip. The carpet at side 3, level 1 is the Moore ring; any other code, side and level is another life. Born and Stay count live mask cells, as digits or as a named sequence.",
    "The Library holds famous patterns to seed the board, whole worlds of mask and rule, and named rules. Reroll sows a random soup.",
    "Paint cells by hand, then press Play. Run to fate replays the board for up to 512 generations and says whether it dies, freezes, loops, or is still moving.",
    "In 1D the board is one line of 320 cells, drawn top to bottom with the newest row last. Wrap joins the edges. A side-3 line at level 1 runs elementary rules, as on [Wolfram](/wolfram/); the whole family is in [the automata note](/research/automata/).",
  ],
  name: (value) =>
    [
      "life",
      slug(
        worn(PRESETS, value) ||
          `${dimOf(value)}d ${value.code} ${value.number} ${value.level} b${value.born} s${value.stay}`,
      ),
      SOUP(value) ? value.seed : slug(value.from),
    ].join("-"),
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Pick<Life, "playing"> | null) => (scene?.playing?.() ? "Pause" : "Play"),
      act: "play",
      button: true,
      menu: "Edit",
    },
    { key: "n", label: "Step", act: "step", menu: "Edit" },
    { key: "f", label: "Run to fate", act: "fate", menu: "Edit" },
    { key: ["0", "Home"], label: "Reset", act: "reset" },
    { key: "c", label: "Clear", act: "clear" },
  ],
  actions: {
    play: (scene) => scene.toggle(),
    step: (scene) => scene.step(),
    fate: (scene) => scene.doom(),
    reset: (scene) => scene.reset(),
    clear: (scene) => scene.clear(),
  },
} satisfies Page<Ready & Units, Life>;

export const SAVERS = [
  {
    id: "soup",
    title: "Soup",
    pick: (rand: Rng) => ({
      dim: 2,
      code: "7",
      number: 3,
      level: 1,
      ...pick(rand, RULES.slice(0, 2))[1],
      wrap: 1,
      from: "noise",
      density: Math.round((0.26 + rand() * 0.12) * 100) / 100,
      speed: 12,
    }),
  },
  {
    id: "crystal",
    title: "Crystal",
    pick: () => ({
      dim: 2,
      code: "15",
      number: 3,
      level: 1,
      born: "1357",
      stay: "02468",
      wrap: 0,
      from: "one",
      speed: 10,
    }),
  },
  {
    id: "tapestry",
    title: "Tapestry",
    pick: (rand: Rng) => ({
      dim: 1,
      code: "1",
      number: 3,
      level: 1,
      born: "1",
      stay: rand() < 0.5 ? "02" : "1",
      wrap: 1,
      from: "one",
      speed: 16,
    }),
  },
];

export const units = wasm("life", "math");

export const HOLD = 3000;

const FIT = 0.96;

const GROUND = 0.07;

const OLD = 0.55;

const OPS = 4e6;

const STILL = 24;

const THUMB = 24;

const EPS = 1e-6;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {} as Given) {
  const ctx = canvas.getContext("2d")!;
  const { life, math } = opts;
  const read = (next: Opts): Knobs => {
    const one = { ...(tidy(SPEC, next) as Knobs), seed: next.seed };
    return view.lock && one.from === START ? { ...one, from: "noise" } : one;
  };
  let value = read(opts);
  let shape: Mould | null = null;
  let rule: Rule | null = null;
  let name = "";
  let width = BOARD;
  let height = BOARD;
  let ring: Uint8Array | null = null;
  let types: Cell["types"] | null = null;
  let age: Uint8Array | null = null;
  let gen = 0;
  let seen: number[] = [];
  let word = "";
  let verdict = "";
  let error = "";
  let pop = 0;
  let playing = Boolean(view.lock);
  let origin = 0;
  let lay: Lay | null = null;
  let skin: Skin | null = null;
  let shown = -1;
  let dirty = true;
  let gone = false;
  let brush = 1;
  let hands = false;
  let since = -1;
  const linear = () => shape!.dim === 1;
  const mould = () => {
    shape = maskOf(life, value);
    shape.named = maskName(math, shape.code, shape.dim);
    shape.label = `${shape.named || `code ${shape.code}`}, side ${shape.number}, level ${shape.level}`;
  };
  const measure = () => {
    width = linear() ? LINE : view.lock ? BOARD : sideOf(value);
    height = linear() ? 1 : width;
  };
  const judge = () => {
    rule = ruleOf(life, value, shape!.cells);
    name = shape!.named ? `${rule.name} on ${shape!.named}` : rule.name;
  };
  const grid = (): Cell => ({ shape: [height, width], types: types! });
  const begin = (next: Uint8Array) => {
    types = next;
    age = Uint8Array.from(types);
    ring = linear() ? new Uint8Array(ROWS * LINE) : null;
    ring?.set(types, 0);
    gen = 0;
    seen = [];
    pop = population(types);
    word = fate(seen, hash(types), pop);
    verdict = "";
    origin = view.t;
    since = -1;
    shown = -1;
  };
  const sowed = (rand: Rng = rng(value.seed)) => begin(sow(value, rand, math, width, height));
  const dress = () => {
    const { accent } = view.look();
    skin = { ink: accent, dim: veil(accent, GROUND) };
  };
  const tell = () =>
    !gone &&
    opts.onStat?.({
      gen,
      pop,
      fate: verdict || word,
      name,
      counts: [rule!.birth, rule!.survive],
      mask: shape!.mask,
      cells: shape!.cells,
      lattice: lattice(shape!.lattice),
      label: shape!.label!,
      error,
    });
  const advance = () => {
    types = life.next_grid(grid(), rule!.birth, rule!.survive, shape!.mask, value.wrap ? "Wrap" : "Constant").types;
    if (ring) ring.set(types, ((gen + 1) % ROWS) * LINE);
    else for (let i = 0; i < types.length; i++) age![i] = types[i] ? Math.min(255, age![i] + 1) : 0;
    gen++;
    pop = population(types);
    word = fate(seen, hash(types), pop);
  };
  const settle = () => {
    if (hands || !word) {
      since = -1;
      return;
    }
    if (since < 0) since = view.t;
    else if (view.t - since >= HOLD) {
      begin(sow(value, view.rand, math, width, height));
      tell();
      whole();
    }
  };
  const budget = () => Math.max(1, Math.min(32, Math.floor(OPS / (types!.length * Math.max(1, shape!.cells)))));
  const layout = (): Lay => {
    if (!linear()) return board(view, width, height, FIT);
    const cell = Math.max(1, Math.floor((view.w * FIT) / LINE));
    const rows = Math.max(1, Math.min(ROWS, Math.floor((view.h * FIT) / cell)));
    return {
      x: Math.floor((view.w - LINE * cell) / 2),
      y: Math.floor((view.h - rows * cell) / 2),
      w: LINE * cell,
      h: rows * cell,
      cell,
      rows,
    };
  };
  const ground = (x: number, y: number, w: number, h: number) => {
    ctx.fillStyle = skin!.dim;
    ctx.fillRect(x, y, w, h);
    ctx.fillStyle = skin!.ink;
  };
  const flatPaint = () => {
    const { x, y, w, h, cell } = lay!;
    ctx.clearRect(0, 0, view.w, view.h);
    ground(x, y, w, h);
    for (const [lit, alpha] of [
      [(a) => a === 1, 1],
      [(a) => a > 1, OLD],
    ] satisfies [(a: number) => boolean, number][]) {
      ctx.globalAlpha = alpha;
      ctx.beginPath();
      for (let i = 0; i < types!.length; i++)
        if (lit(age![i])) ctx.rect(x + (i % width) * cell, y + Math.floor(i / width) * cell, cell, cell);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  };
  const first = () => Math.max(0, gen - lay!.rows! + 1);
  const band = (g: number) => lay!.y + (g - first()) * lay!.cell;
  const row = (g: number) =>
    runs(ctx, ring!.subarray((g % ROWS) * LINE, (g % ROWS) * LINE + LINE), LINE, 1, lay!.x, band(g), lay!.cell);
  const sheet = () => {
    ctx.clearRect(0, 0, view.w, view.h);
    ground(lay!.x, lay!.y, lay!.w, lay!.h);
    for (let g = first(); g <= gen; g++) row(g);
  };
  const more = () => {
    const { x, y, w, h, cell, rows } = lay as Required<Lay>;
    const k = gen - shown;
    if (gen < rows) {
      for (let g = shown + 1; g <= gen; g++) row(g);
      return;
    }
    if (k >= rows || Math.max(0, shown - rows + 1) !== first() - k) {
      sheet();
      return;
    }
    const keep = (rows - k) * cell;
    ctx.drawImage(canvas, x, y + k * cell, w, keep, x, y, w, keep);
    ctx.clearRect(x, y + keep, w, h - keep);
    ground(x, y + keep, w, h - keep);
    for (let g = shown + 1; g <= gen; g++) row(g);
  };
  const paint = (full: boolean) => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    if (!linear()) flatPaint();
    else if (full || shown < 0 || gen < shown) sheet();
    else if (gen === shown) {
      ctx.clearRect(lay!.x, band(gen), lay!.w, lay!.cell);
      ground(lay!.x, band(gen), lay!.w, lay!.cell);
      row(gen);
    } else {
      ctx.fillStyle = skin!.ink;
      more();
    }
    shown = gen;
    dirty = false;
  };
  const whole = () => {
    lay = layout();
    paint(true);
  };
  const theme = () => {
    dress();
    whole();
  };
  const draw = () => {
    if (view.fixed && !playing) toggle();
    if (playing) {
      const speed = value.speed;
      const need = Math.floor(((view.t - origin) * speed) / 1000 + EPS) - gen;
      if (need > 0) {
        const n = Math.min(need, budget());
        for (let i = 0; i < n; i++) advance();
        if (need > n) origin = view.t - (gen * 1000) / speed;
        dirty = true;
        tell();
      }
    }
    if (view.lock) settle();
    if (dirty) paint(false);
  };
  const toggle = () => {
    playing = !playing;
    if (playing) {
      origin = view.t - (gen * 1000) / value.speed;
      view.wake?.();
    }
    tell();
  };
  const step = () => {
    advance();
    origin -= 1000 / value.speed;
    tell();
    paint(false);
  };
  const doom = () => {
    try {
      verdict = told(destiny(life, grid(), shape!.mask, rule!, value.wrap, LIMIT));
      error = "";
    } catch (thrown) {
      error = String((thrown as { message?: unknown } | null | undefined)?.message ?? thrown);
    }
    tell();
  };
  const reset = () => {
    playing = false;
    sowed();
    tell();
    whole();
  };
  const clear = () => {
    hands = true;
    types = new Uint8Array(types!.length);
    age = new Uint8Array(types.length);
    ring?.fill(0);
    gen = 0;
    seen = [];
    pop = 0;
    word = fate(seen, hash(types), 0);
    verdict = "";
    origin = view.t;
    tell();
    whole();
  };
  const touch = (u: number, v: number, start: boolean): undefined => {
    const { x, y, cell } = lay!;
    const col = Math.floor((u * view.w - x) / cell);
    const line = linear() ? 0 : Math.floor((v * view.h - y) / cell);
    if (col < 0 || col >= width || line < 0 || line >= height) return;
    if (linear() && Math.abs(v * view.h - band(gen) - cell / 2) > Math.max(cell, THUMB * view.dpr)) return;
    const i = line * width + col;
    hands = true;
    if (start) brush = types![i] ? 0 : 1;
    if (types![i] === brush) return;
    types![i] = brush;
    age![i] = brush;
    if (ring) ring[(gen % ROWS) * LINE + i] = brush;
    seen = [];
    pop = population(types!);
    word = fate(seen, hash(types!), pop);
    verdict = "";
    tell();
    paint(!linear());
  };
  const set = (next: Opts) => {
    const was = value;
    value = read(next);
    const moved = (...keys: (keyof Knobs)[]) => keys.some((key) => String(value[key]) !== String(was[key]));
    if (moved("dim", "code", "number", "level")) mould();
    if (moved("dim", "from")) measure();
    if (moved("dim", "code", "number", "level", "born", "stay")) judge();
    if (moved("dim", "from", "density", "seed") || (value.from === "design" && moved("code", "number"))) sowed();
    else if (moved("dim", "code", "number", "level", "born", "stay", "wrap")) verdict = "";
    if (moved("speed")) origin = view.t - (gen * 1000) / value.speed;
    tell();
    whole();
  };
  const stop = () => {
    gone = true;
  };
  dress();
  mould();
  measure();
  judge();
  sowed(view.rand);
  if (view.still && view.lock) {
    lay = layout();
    const n = linear() ? lay.rows! - 1 : Math.min(STILL, budget());
    for (let i = 0; i < n; i++) advance();
  }
  whole();
  tell();
  return { draw, size: whole, theme, set, toggle, playing: () => playing, step, doom, reset, clear, touch, stop };
}
