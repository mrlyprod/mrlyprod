import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch, Raw, Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { veil } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { DESIGN } from "../../lib/designs/engine.ts";
import type { Design } from "../../lib/designs/store.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick, rng } from "../../lib/rand.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Chaos, Doors, Fit, Law, Meter, Mode, Num, Pair, Source, State } from "./engine.ts";
import {
  boot,
  boxOf,
  CHECKS,
  CUT,
  cell,
  csv,
  DIM,
  DRAW,
  deal,
  distance,
  FRAME,
  feed,
  fit,
  fromCloud,
  games,
  HEXES,
  hit,
  LEVELS,
  law,
  lay,
  MOSAICS,
  MOST,
  meter,
  modeOf,
  modes,
  origin,
  PAIR,
  pair,
  replay,
  restricted,
  SPEEDS,
  schedule,
  sides,
  spot,
  step,
  study,
  toCloud,
  VIEWS,
  WEIGHTS,
  WORDS,
  weighted,
} from "./engine.ts";

/* TYPES */

export type Seen = Pick<View, "w" | "h" | "dpr" | "t" | "rand" | "look" | "still" | "lock">;

export type Facts = {
  n: number;
  k: number;
  counted: number;
  distance: number;
  painted: number;
  total: number;
  outside: number;
  agree: number;
  checked: number;
  lap: number;
  laps: number;
  perfect: boolean;
  mode: Mode;
  law: string;
  stuck: boolean;
  lost: boolean;
};

export type Given = Opts & { math?: Mrly | null; num?: Num | null; onFacts?: (facts: Facts) => void };

export type Hands = { deeper: (by: number) => void; faster: (by: number) => void; flip: () => void };

export type Stage = ReturnType<typeof make>;

export type Handled = Stage & Hands;

export type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

type Saver = { id: string; title: string; pick: (rand: Rng) => Patch };

const design = (value: Value) => value.view === "design";

const typed = (value: Value) => value.view === "word" && value.word === "typed";

export const SPEC: Spec = [
  { key: "view", label: "Source", kind: "segment", def: "design", options: VIEWS, group: "Source" },
  {
    key: "side",
    label: "Side",
    kind: "pick",
    def: 0,
    options: sides,
    group: "Source",
    when: (value) => design(value) || value.view === "word",
  },
  {
    key: "word",
    label: "Word",
    kind: "pick",
    def: "thue",
    options: WORDS,
    group: "Source",
    when: (value) => value.view === "word",
  },
  { key: "letters", label: "Letters", kind: "text", def: PAIR.join(" "), group: "Source", when: typed },
  {
    key: "mosaic",
    label: "Mosaic",
    kind: "pick",
    def: MOSAICS[0].key,
    options: MOSAICS.map((one) => [one.key, one.name]),
    group: "Source",
    when: (value) => value.view === "mosaic",
  },
  {
    key: "hex",
    label: "Base",
    kind: "pick",
    def: HEXES[0][0],
    options: HEXES,
    group: "Source",
    when: (value) => value.view === "hex",
  },
  { key: "cut", label: "Cut x + y", kind: "text", def: CUT, group: "Source", when: (value) => value.view === "slice" },
  {
    key: "game",
    label: "Rule",
    kind: "pick",
    def: "repeat",
    options: (_value, units) => games(units?.math).map((one) => [one.key, one.name]),
    group: "Source",
    when: restricted,
  },
  {
    key: "rule",
    label: "Code",
    kind: "text",
    def: "",
    group: "Source",
    when: (value) => restricted(value) && value.game === "typed",
  },
  {
    key: "mode",
    label: "Draws",
    kind: "segment",
    def: "random",
    options: modes,
    group: "Draws",
    when: (value) => value.view !== "rule",
  },
  {
    key: "weights",
    label: "Weights",
    kind: "segment",
    def: "uniform",
    options: WEIGHTS,
    group: "Draws",
    when: (value) => modeOf(value) === "random",
  },
  { key: "speed", label: "Points a frame", kind: "pick", def: 100, options: SPEEDS, group: "Draws" },
  {
    key: "level",
    label: "Level k",
    kind: "slider",
    def: 2,
    min: 1,
    max: (value, units) =>
      units?.math && units?.num ? study(units as Doors, { ...value, ...tidy(DESIGN, value) } as Chaos).cap : LEVELS,
    step: 1,
    keep: true,
    group: "Meter",
  },
  { key: "grid", label: "Grid", kind: "toggle", def: 0, group: "Meter" },
];

export const tidied = (value: Raw, units?: Doors): Chaos =>
  ({ ...value, ...(tidy(DESIGN, value) as Design), ...tidy(SPEC, value, units) }) as unknown as Chaos;

export const units = wasm("math", "num");

export function file(value: Value) {
  const view = String(value.view);
  const own =
    {
      design: [value.base, value.code, value.side || value.base],
      word: [value.word, value.side || 2],
      mosaic: [value.mosaic],
      hex: [value.hex],
      slice: [value.base, value.code, String(value.cut).replace("/", "-")],
      rule: [value.game],
    }[view] ?? [];
  return ["chaos", view, ...own, modeOf(value), value.level].join("-");
}

export const SAVERS: Saver[] = [
  {
    id: "cloud",
    title: "Design cloud",
    pick: (rand) => ({
      view: "design",
      base: 2,
      code: pick(rand, ["7", "11", "13", "14", "6", "9", "10", "5"]),
      side: pick(rand, [0, 0, 3]),
      mode: "random",
      weights: "uniform",
      speed: 1000,
      level: 3,
      grid: 0,
    }),
  },
  {
    id: "hex",
    title: "Hex cloud",
    pick: (rand) => ({ view: "hex", hex: pick(rand, HEXES)[0], mode: "random", speed: 1000, grid: 0 }),
  },
];

export const PAGE = {
  defaults: { base: 2, code: "7" },
  spec: SPEC,
  units,
  name: file,
  design: { dim: DIM, base: "base", code: "code", when: (value) => design(value) || value.view === "slice" },
  about: [
    "The chaos game draws a design with a pencil: start anywhere, pick a digit of the design at random, move to that digit's corner by the base, mark the point, repeat. Every point sits in the cell of its last draws, since each draw becomes the new leading digit and the older ones move one place finer. The game is a shift register whose cells are the design's cells, and the cloud is the design.",
    "The meter watches that law. After k draws the level-k cell of the point is exactly uniform over the design's cells whatever the start, and with weighted draws it follows the product law exactly. The readout is the distance of the painted histogram from that law, the count of cells painted, and the check that each point's cell is the word of its last k draws. The grid toggle shows the level-k cells filling. Tap the picture to start again from the tap.",
    "Random draws paint every cell in about m^k log m^k draws. A de Bruijn schedule paints every level-k cell exactly once per lap of m^k draws, and no schedule is shorter. A window rule such as never the same corner twice restricts the draws, and the game then draws the set of the reversed rule from the memory dial, of dimension log rho over log b, and its cells are a design's cells only when the rule is a product.",
    "A magic word is a schedule of digit sets, one per step. A mosaic or a slice is a game on a graph of states, one edge per corner, and a hexagonal base turns each copy by a unit. The mathematics is in [the beneath notes](/research/beneath/); the designs come from [Designs](/designs/), the words from [Magic](/research/magic/) and the bases from [Radix](/radix/).",
  ],
  keys: [
    { key: "ArrowUp", label: "Level", act: "up", menu: "View" },
    { key: "ArrowDown", label: "Level", act: "down", menu: "View" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Speed", act: "speed" },
    { key: "g", label: "Grid", act: "grid", menu: "View" },
  ],
  actions: {
    up: (scene) => scene.deeper?.(1),
    down: (scene) => scene.deeper?.(-1),
    speed: (scene, e) => scene.faster?.(e.key === "ArrowLeft" ? -1 : 1),
    grid: (scene) => scene.flip?.(),
  },
} satisfies Page<Loaded<"math" | "num">, Handled>;

const R = 1024;
const DOT = 2;
const ALPHA = 0.3;
const FAINT = 0.12;
const STRONG = 0.42;
const PAD = 12;
const LIT = 48;
const LINES = 32;
const TELL = 200;
const BURST = 30000;
const THUMB = 1500;
const WIDE = 4294967296;

/* PAINT */

export function stamp(pen: Pen, f: Fit, x: number, y: number, dot = DOT) {
  const [cx, cy] = toCloud(f, x, y);
  pen.fillRect(cx - dot / 2, cy - dot / 2, dot, dot);
}

export function recolour(pen: Pen, w: number, h: number, colour: string) {
  pen.globalCompositeOperation = "source-in";
  pen.fillStyle = colour;
  pen.fillRect(0, 0, w, h);
  pen.globalCompositeOperation = "source-over";
}

export function cellsOf(lawed: Law): Uint8Array {
  const out = new Uint8Array(lawed.cells);
  for (const at of lawed.support) out[Math.floor(at / lawed.tags)] = 1;
  return out;
}

export function hexCentre(source: Source, k: number, idx: number): Pair {
  const hex = source.hex!;
  const m = source.letters.length;
  const [x0, y0, x1, y1] = hex.box;
  let zx = (x0 + x1) / 2;
  let zy = (y0 + y1) / 2;
  for (let j = 0; j < k; j++) {
    const letter = Math.floor(idx / m ** j) % m;
    const [dx, dy] = hex.digits[letter];
    const [ux, uy] = hex.units[letter];
    const [bx, by] = hex.beta;
    const ax = ux * zx - uy * zy + dx;
    const ay = ux * zy + uy * zx + dy;
    const den = bx * bx + by * by;
    zx = (ax * bx + ay * by) / den;
    zy = (ay * bx - ax * by) / den;
  }
  return [zx, zy];
}

export function overlay(pen: Pen, source: Source, lawed: Law, lit: Uint8Array, f: Fit, accent: string) {
  const k = lawed.k;
  const have = cellsOf(lawed);
  if (source.hex) {
    const side = 2 * source.hex.reach * source.hex.norm ** (-k / 2) * f.s;
    pen.clearRect(0, 0, R, R);
    for (const alpha of [FAINT, STRONG]) {
      pen.fillStyle = veil(accent, alpha);
      for (let idx = 0; idx < lawed.cells; idx++) {
        if (!have[idx] || (alpha === STRONG) !== Boolean(lit[idx])) continue;
        const [cx, cy] = toCloud(f, ...hexCentre(source, k, idx));
        pen.fillRect(cx - side / 2, cy - side / 2, side, side);
      }
    }
    return;
  }
  const side = source.base ** k;
  const image = pen.createImageData(side, side);
  const [r, g, b] = accent.startsWith("#")
    ? [1, 3, 5].map((at) => Number.parseInt(accent.slice(at, at + 2), 16))
    : [0, 0, 0];
  for (let idx = 0; idx < lawed.cells; idx++) {
    if (!have[idx]) continue;
    const cx = idx % side;
    const cy = Math.floor(idx / side);
    const at = ((side - 1 - cy) * side + cx) * 4;
    image.data[at] = r;
    image.data[at + 1] = g;
    image.data[at + 2] = b;
    image.data[at + 3] = Math.round(255 * (lit[idx] ? STRONG : FAINT));
  }
  pen.putImageData(image, 0, 0);
}

export function light(pen: Pen, source: Source, k: number, idx: number, f: Fit, accent: string) {
  pen.fillStyle = veil(accent, STRONG);
  if (source.hex) {
    const side = 2 * source.hex.reach * source.hex.norm ** (-k / 2) * f.s;
    const [cx, cy] = toCloud(f, ...hexCentre(source, k, idx));
    pen.fillRect(cx - side / 2, cy - side / 2, side, side);
    return;
  }
  const side = source.base ** k;
  pen.clearRect(idx % side, side - 1 - Math.floor(idx / side), 1, 1);
  pen.fillRect(idx % side, side - 1 - Math.floor(idx / side), 1, 1);
}

export function sketch(ctx: CanvasRenderingContext2D, w: number, h: number, dpr: number, units: Doors, patch: Raw) {
  const value = tidied(patch, units);
  const source = study(units, value);
  const tape = feed(Math.floor(rng(1)() * WIDE));
  const start = origin(source, tape.rand);
  let state = boot(deal(source, 1, false, null, start, tape.seed));
  const box = lay(w, h, 4 * dpr);
  const f = fit(boxOf(source), box.size);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, w, h);
  const css = typeof getComputedStyle === "function" ? getComputedStyle(ctx.canvas) : null;
  ctx.fillStyle = css?.getPropertyValue("--fg").trim() || "#888";
  for (let i = 0; i < THUMB; i++) {
    const next = step(state, DRAW);
    if (next.n === state.n) break;
    state = next;
    const [cx, cy] = toCloud(f, ...spot(state));
    ctx.fillRect(box.x + cx - dpr / 2, box.y + cy - dpr / 2, dpr, dpr);
  }
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const { math, num, onFacts } = opts;
  if (!math || !num)
    throw new Error("chaos: the scene wants @mrlyprod/mrly/math and @mrlyprod/mrly/num as opts.math and opts.num");
  const units = { math, num };
  const ctx = canvas.getContext("2d")!;
  const read = (next: Opts) => tidied(next, units);
  let value = read(opts);
  let seed = opts.seed;
  let source = study(units, value);
  let k = Math.min(value.level, source.cap);
  let mode: Mode = modeOf(value);
  let heavy = weighted(value);
  let sched: Uint8Array | null = null;
  let lawed: Law = law(source, k, heavy);
  let met: Meter = meter(lawed, 0);
  let start: Pair = [0, 0];
  let walked: State | null = null;
  let lit = new Uint8Array(0);
  let log = new Uint8Array(MOST);
  let f = fit(boxOf(source), R);
  let box = lay(view.w, view.h, PAD * view.dpr);
  let cloud: OffscreenCanvas | null = null;
  let sheet: OffscreenCanvas | null = null;
  let playing = true;
  let anchor = view.t;
  let drawn = 0;
  let told = -Infinity;
  let agree = 0;
  let checked = 0;
  let burst = true;
  let gone = false;
  let dirty = true;
  const recent = new Float32Array(2 * LIT);
  let hot = 0;
  let held = 0;
  const look = () => view.look();
  const pen = () => {
    cloud ??= new OffscreenCanvas(R, R);
    return cloud.getContext("2d")!;
  };
  const page = () => {
    const side = source.hex ? R : source.base ** k;
    if (!sheet || sheet.width !== side) sheet = new OffscreenCanvas(side, side);
    return sheet.getContext("2d")!;
  };
  const facts = (): Facts => ({
    n: walked?.n ?? 0,
    k,
    counted: met.counted,
    distance: distance(met),
    painted: met.painted,
    total: lawed.total,
    outside: met.outside,
    agree,
    checked,
    lap: met.lap,
    laps: met.laps,
    perfect: met.perfect,
    mode: sched ? "bruijn" : mode,
    law: sched
      ? "uniform"
      : source.kind === "word"
        ? "the phase law, uniform over each phase's cells"
        : source.auto.states === 1
          ? heavy
            ? "the product law 1, 2, 3"
            : "uniform"
          : "the stationary law",
    stuck: Boolean(walked?.stuck),
    lost: lawed.lost,
  });
  const tell = () => {
    if (gone) return;
    told = view.t;
    onFacts?.(facts());
  };
  const overlaid = () => {
    if (!value.grid) return;
    overlay(page(), source, lawed, lit, source.hex ? f : { s: 1, ox: 0, oy: 0 }, look().accent);
  };
  const clear = () => {
    if (!cloud) return;
    const p = pen();
    p.globalCompositeOperation = "source-over";
    p.clearRect(0, 0, R, R);
  };
  const inkPen = () => {
    const p = pen();
    p.fillStyle = veil(look().ink, ALPHA);
    return p;
  };
  const restart = (from?: Pair) => {
    k = Math.min(value.level, source.cap);
    mode = modeOf(value);
    heavy = weighted(value);
    sched = mode === "bruijn" ? schedule(math, source, k) : null;
    lawed = law(source, k, heavy);
    met = meter(lawed, sched?.length ?? 0);
    lit = new Uint8Array(lawed.cells);
    const tape = feed(walked ? walked.seed : Math.floor(view.rand() * WIDE));
    start = from ?? origin(source, tape.rand);
    walked = boot(deal(source, k, heavy, sched, start, tape.seed));
    f = fit(boxOf(source), R);
    log = new Uint8Array(MOST);
    drawn = 0;
    anchor = view.t;
    agree = 0;
    checked = 0;
    hot = 0;
    held = 0;
    burst = true;
    clear();
    overlaid();
    dirty = true;
    tell();
  };
  const relevel = () => {
    const was = walked!;
    if (was.n > MOST) return restart();
    k = Math.min(value.level, source.cap);
    sched = mode === "bruijn" ? schedule(math, source, k) : null;
    lawed = law(source, k, heavy);
    met = meter(lawed, sched?.length ?? 0);
    lit = new Uint8Array(lawed.cells);
    walked = replay(boot(deal(source, k, heavy, sched, start, was.seed)), met, source.auto.tag, log, was.n);
    for (const at of lawed.support) if (met.hist[at]) lit[Math.floor(at / lawed.tags)] = 1;
    agree = 0;
    checked = 0;
    overlaid();
    dirty = true;
    tell();
  };
  const advance = (count: number) => {
    if (!walked || count <= 0) return;
    const p = inkPen();
    const accent = look().accent;
    const sheetPen = value.grid ? page() : null;
    for (let i = 0; i < count; i++) {
      const next = step(walked, DRAW);
      const moved = next.n !== walked.n;
      walked = next;
      if (!moved) break;
      const n = walked.n;
      const [x, y] = spot(walked);
      if (n <= MOST) log[n - 1] = walked.moved;
      stamp(p, f, x, y);
      const [cx, cy] = toCloud(f, x, y);
      recent[2 * hot] = cx;
      recent[2 * hot + 1] = cy;
      hot = (hot + 1) % LIT;
      held = Math.min(LIT, held + 1);
      if (n < k) continue;
      const at = cell(walked);
      hit(met, pair(lawed, at, source.auto.tag[walked.node]));
      if (!lit[at]) {
        lit[at] = 1;
        if (sheetPen) light(sheetPen, source, k, at, f, accent);
      }
      if (!source.hex && checked < CHECKS) {
        checked++;
        if (Number(math.game.cell([x, y], source.base, k)) === at) agree++;
      }
    }
    dirty = true;
  };
  const paint = () => {
    const { accent, ink } = look();
    const dpr = view.dpr;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (cloud) ctx.drawImage(cloud, 0, 0, R, R, box.x, box.y, box.size, box.size);
    if (value.grid && sheet) {
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(sheet, 0, 0, sheet.width, sheet.height, box.x, box.y, box.size, box.size);
      ctx.imageSmoothingEnabled = true;
      const side = source.hex ? 0 : source.base ** k;
      if (side && side <= LINES) {
        ctx.strokeStyle = veil(ink, 0.25);
        ctx.lineWidth = dpr;
        ctx.beginPath();
        for (let i = 0; i <= side; i++) {
          const at = (i / side) * box.size;
          ctx.moveTo(box.x + at, box.y);
          ctx.lineTo(box.x + at, box.y + box.size);
          ctx.moveTo(box.x, box.y + at);
          ctx.lineTo(box.x + box.size, box.y + at);
        }
        ctx.stroke();
      }
    }
    const scale = box.size / R;
    ctx.fillStyle = accent;
    for (let i = 0; i < held; i++) {
      const j = (hot - 1 - i + LIT) % LIT;
      const r = (i === 0 ? 4 : 2.2) * dpr;
      ctx.beginPath();
      ctx.arc(box.x + recent[2 * j] * scale, box.y + recent[2 * j + 1] * scale, r, 0, 2 * Math.PI);
      ctx.fill();
    }
  };
  const draw = () => {
    if (view.still) {
      if (burst) {
        burst = false;
        advance(BURST);
        tell();
      }
    } else if (playing) {
      const target = Math.floor(((view.t - anchor) * value.speed) / FRAME);
      if (target > drawn) {
        advance(target - drawn);
        drawn = target;
      }
    }
    if (dirty || (playing && !view.still)) paint();
    dirty = false;
    if (view.t - told >= TELL) tell();
  };
  const size = () => {
    box = lay(view.w, view.h, PAD * view.dpr);
    dirty = true;
  };
  const theme = () => {
    if (cloud) recolour(pen(), R, R, veil(look().ink, 1));
    overlaid();
    dirty = true;
  };
  const toggle = () => {
    playing = !playing;
    if (playing) anchor = view.t - (drawn * FRAME) / value.speed;
    dirty = true;
  };
  const set = (next: Opts) => {
    const was = value;
    value = read(next);
    const fresh = study(units, value);
    const reseeded = next.seed !== seed;
    seed = next.seed;
    if (reseeded) walked = null;
    if (fresh !== source) {
      source = fresh;
      sheet = null;
      return restart();
    }
    if (modeOf(value) !== mode || weighted(value) !== heavy || reseeded) return restart();
    if (Math.min(value.level, source.cap) !== k) {
      sheet = null;
      relevel();
    }
    if (value.grid !== was.grid) {
      sheet = null;
      overlaid();
      dirty = true;
    }
    if (value.speed !== was.speed) anchor = view.t - (drawn * FRAME) / value.speed;
    dirty = true;
  };
  const tap = (x: number, y: number) => {
    const scale = R / box.size;
    const [ux, uy] = fromCloud(f, (x - box.x) * scale, (y - box.y) * scale);
    if (source.hex) return restart([ux, uy]);
    if (ux < 0 || ux >= 1 || uy < 0 || uy >= 1) return;
    restart([ux, uy]);
  };
  const stop = () => {
    gone = true;
  };
  restart();
  return {
    draw,
    size,
    theme,
    set,
    toggle,
    playing: () => playing,
    tap,
    stop,
    facts,
    sheet: () => csv(met),
    source: () => source,
  };
}
