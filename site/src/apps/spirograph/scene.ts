import type { Readout } from "../../lib/app/hooks.ts";
import type { Mathy, Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Row, Scalar, Units, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Stroke } from "../../lib/app/svg.ts";
import { lines } from "../../lib/app/svg.ts";
import type { Paints } from "../../lib/color.ts";
import { mix, paints, rgb, veil } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Covered, Facts, Fault, Fit, Plan, Roll, Shape, Wheel } from "./engine.ts";
import {
  BANDS,
  BASES,
  cap,
  cover,
  EASE,
  fit,
  INKS,
  index,
  KIND,
  KINDS,
  LEAST,
  legal,
  MODES,
  message,
  NUMBERS,
  PAD,
  played,
  resolve,
  roll,
  SIX,
  share,
  span,
  study,
} from "./engine.ts";

/* TYPES */

export type Spirograph = Roll & Wheel & { at: number; ink: string; fill: number; code: string };

export type Gated = Row & { need?: (value: Value) => boolean };

export type Lean = Fault | Omit<Shape, "mask">;

export type Stat = {
  facts: Facts;
  fault: string | null;
  cover: Lean | null;
  ms: number;
  part: number;
  playing: boolean;
};

export type Seen = Pick<View, "rand" | "still" | "lock" | "w" | "h" | "dpr" | "t" | "wake"> & {
  look: () => Pick<Paints, "paper" | "accent">;
};

export type Given = Opts & { math?: Mrly | null; tell?: (stat: Stat) => void };

export type Played = ReturnType<typeof make>;

type Saver = { id: string; title: string; pick: (rand: Rng) => Partial<Spirograph> };

type Now = {
  plan: Plan | null;
  fault: string | null;
  shape: Covered | null;
  ms: number;
  playing: boolean;
  pct: number;
};

const HIDE = () => false;
const POLYGONS: Scalar[] = ["polyin", "polyout"];
const caps = (units?: Units) => units?.math?.spirograph;

export const SPEC: Gated[] = [
  { key: "kind", label: "Track", kind: "pick", def: "in", options: KINDS, group: "Track", when: HIDE },
  {
    key: "sides",
    label: "Sides",
    kind: "slider",
    def: 4,
    min: 3,
    max: (_value, units) => caps(units)?.SIDES()[1],
    step: 1,
    group: "Track",
    when: HIDE,
    need: (value) => POLYGONS.includes(value.kind),
  },
  {
    key: "ring",
    label: "Ring R",
    kind: "slider",
    def: 7,
    min: 1,
    max: (_value, units) => caps(units)?.RADIUS_CAP(),
    step: 1,
    group: "Track",
    when: HIDE,
  },
  {
    key: "wheel",
    label: "Wheel r",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value, units) =>
      legal(
        value.kind as string,
        value.ring as number,
        caps(units)?.RADIUS_CAP() ?? Infinity,
        value.sides as number,
      )[1],
    step: 1,
    group: "Track",
    when: HIDE,
  },
  {
    key: "laps",
    label: "Laps",
    kind: "slider",
    def: 2,
    min: 1,
    max: (_value, units) => caps(units)?.LAPS_CAP(),
    step: 1,
    group: "Track",
    when: HIDE,
    need: (value) => value.kind !== "in" && value.kind !== "out",
  },
  {
    key: "at",
    label: "Drawn",
    kind: "slider",
    def: 100,
    min: 0,
    max: 100,
    step: 0.1,
    unit: "%",
    group: "Draw",
    when: HIDE,
  },
  { key: "ink", label: "Ink", kind: "pick", def: "kind", options: INKS, group: "Draw", when: HIDE },
  { key: "fill", label: "Fill between the walls", kind: "toggle", def: 0, group: "Draw", when: HIDE },
  { key: "base", label: "Base", kind: "segment", def: 3, options: BASES, group: "Wheel", when: HIDE },
  { key: "code", label: "Code", kind: "text", def: "", group: "Wheel", when: HIDE },
  {
    key: "number",
    label: "Side",
    kind: "pick",
    def: 3,
    options: NUMBERS.map((n) => [n, String(n)]),
    group: "Wheel",
    when: HIDE,
  },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 1,
    min: 1,
    max: (value, units) => units?.math && cap(units.math, value.number as number),
    step: 1,
    group: "Wheel",
    when: HIDE,
  },
  { key: "mode", label: "Pencils", kind: "pick", def: "fill", options: MODES, group: "Wheel", when: HIDE },
  {
    key: "reach",
    label: "Reach",
    kind: "slider",
    def: 0.9,
    min: 0.2,
    max: 1.3,
    step: 0.01,
    unit: "r",
    group: "Wheel",
    when: HIDE,
  },
  {
    key: "jitter",
    label: "Jitter",
    kind: "slider",
    def: 0,
    min: 0,
    max: 1,
    step: 0.01,
    unit: "cells",
    group: "Wheel",
    when: HIDE,
  },
];

export const units = wasm("math");

export const file = (value: Value, loaded?: Mathy) =>
  `spirograph-${loaded?.math ? resolve(loaded.math, value as Spirograph) : String(value.code ?? "").trim() || "carpet"}-${value.kind}-${value.ring}-${value.wheel}`.replace(
    /[^\w.+-]+/g,
    "-",
  );

export const PAGE = {
  spec: SPEC,
  units,
  design: { dim: 2, base: "base", code: "code" },
  name: file,
  record: true,
  gestures: true,
  about: [
    "A design is the wheel and its cells are the holes. A pencil sits in every cell, the wheel rolls without slipping inside or outside a circle, along a line or around a polygon, and every pencil draws its own curve. Play draws it once; tap the picture to play or stop, drag across it to scrub. The pencils are the design's own cells, so the picture is a rotation average of the design, as on [Radial](/radial/) and [Spin](/spin/); [Tourbillon](/tourbillon/) turns the layers of a stack instead.",
    "On a circle write R/r = a/b in lowest terms. Two pencils draw the same curve when turning the wheel by a whole number of b-ths of a turn carries one seat onto the other, and the square grid allows only a half turn (b even) or a quarter turn (b a multiple of 4).",
    "When every seat sits off the centre and inside the window min(1, (a - b)/b), no curve loops: two curves cross 2ab times and one curve crosses itself a(b - 1) times, so k distinct curves make 2ab C(k,2) + k a(b - 1) nodes.",
    "Fill treats every curve as a wall and shades the shape between the outer and inner walls, and covered is its share of the disc around it. It is a flood on a raster, so the wall share is printed beside it. Everything is computed in Rust.",
  ],
  keys: [
    {
      key: "Enter",
      label: (_value, handle) => (handle?.playing?.() ? "Stop" : "Play"),
      act: "toggle",
      button: true,
      menu: "Edit",
    },
  ],
  actions: { toggle: (scene) => scene.toggle() },
} satisfies Page<Loaded<"math">, Played>;

/* SAVERS */

const SEATED: [number, number][] = [
  [3, 2],
  [5, 1],
  [7, 1],
];
const PENCILS = ["fill", "both", "corners"];
const REACHES = [0.7, 0.8, 0.9, 1, 1.1];
const ROSETTES: [number, number][] = [
  [7, 3],
  [5, 2],
  [8, 3],
  [7, 4],
  [9, 4],
  [7, 2],
  [5, 3],
];
const BLOOMS: [number, number][] = [
  [1, 1],
  [2, 1],
  [3, 1],
  [4, 1],
  [3, 2],
];
const FACETS = [3, 4, 5, 6, 7, 8];

const seated = (rand: Rng) => {
  const [number, level] = pick(rand, SEATED);
  return { number, level, mode: pick(rand, PENCILS), reach: pick(rand, REACHES), base: pick(rand, [3, 3, 2]) };
};

const circle = (kind: string, list: [number, number][]) => (rand: Rng) => {
  const [ring, wheel] = pick(rand, list);
  return { kind, ring, wheel, ...seated(rand) };
};

const faceted = (rand: Rng) => {
  const sides = pick(rand, FACETS);
  const inside = rand() < 0.5;
  return {
    kind: inside ? "polyin" : "polyout",
    sides,
    ring: pick(rand, inside ? [7, 8] : [3, 4]),
    wheel: 2,
    laps: 2,
    ...seated(rand),
  };
};

export const SAVERS: Saver[] = [
  { id: "rosettes", title: "Rosettes", pick: circle("in", ROSETTES) },
  { id: "blooms", title: "Blooms", pick: circle("out", BLOOMS) },
  { id: "polygon", title: "Polygon rolls", pick: faceted },
];

const STUDY: (keyof Spirograph)[] = [
  "kind",
  "sides",
  "ring",
  "wheel",
  "laps",
  "base",
  "code",
  "number",
  "level",
  "mode",
  "reach",
  "jitter",
  "seed",
];

const TAU = Math.PI * 2;
const FADE = 0.55;
const LINE = 1.25;
const DOT = 2.5;
const SPOT = 0.35;
const INSET = 0.08;
const DASH = [4, 6];
const TRACK = 0.3;
const RING = 0.45;
const CELLS = 0.2;
const FILL = 0.18;
const SHAPE = 3;

/* READOUTS */

const fixed = (x: number, places: number) => (Number.isFinite(x) ? x.toFixed(places) : "-");

const pct = (x: number, places = 0) => `${fixed(x * 100, places)}%`;

const plural = (n: number, word: string) => `${n} ${word}${n === 1 ? "" : "s"}`;

const turned = (x: number) => (Number.isInteger(x) ? String(x) : x.toFixed(2));

export const parked = (part: number) => Math.round(Math.min(1, Math.max(0, part)) * 1000) / 10;

export const scrub = (start: number, dx: number, width: number) => parked(start / 100 + dx / (width || 1));

export function law(f: Facts): string {
  if (f.round)
    return `R/r = ${f.ratio[0]}/${f.ratio[1]}, closes after ${plural(f.orbits, "orbit")}, ${f.fold}-fold, ${f.distinct} distinct of ${f.pencils}`;
  if (f.kind === "line")
    return `${plural(f.distinct, "shape")} of ${f.pencils}: on a line the curves of one radius are shifts of one shape`;
  return `${plural(f.pencils, "curve")} over ${plural(f.orbits, "lap")}: no coincidence law on a polygon`;
}

export function crossings(f: Facts): string | null {
  if (!f.round || !f.pencils) return null;
  if (f.nodes === null)
    return "not the law: a seat sits at the centre, or at or past the window min(1, (a - b)/b) inside or 1 outside";
  return `N = 2ab C(k,2) + k a(b - 1) = ${f.nodes}`;
}

const covered = (c: Lean) =>
  c.fault ??
  `wall ${pct(c.wall, 2)}, hole ${pct(c.hole, 2)}, winding ${fixed(c.winding, 4)} of an exact ${fixed(c.areas, 4)}, disc ${fixed(c.disc.radius, 3)} out, ${fixed(c.disc.hole, 3)} in, raster ${c.side}`;

export function status(stat: Omit<Stat, "playing"> | null): Readout[] {
  if (!stat) return [];
  const { facts: f, fault, cover: c, ms, part } = stat;
  const nodes = crossings(f);
  return [
    ...(fault ? [["fault", fault] satisfies Readout] : []),
    ["wheel", `${f.name ?? "code"} ${f.code} base ${f.base} level ${f.level}`],
    ["pencils", f.pencils],
    ["curves", f.distinct],
    ["nodes", f.nodes ?? "-"],
    ["turns", turned(f.turns)],
    ...(c && !c.fault ? [["covered", pct((c as Omit<Shape, "mask">).covered)] satisfies Readout] : []),
    ["drawn", pct(part)],
    ["draw", `${Math.round(ms)} ms`],
    ["law", law(f)],
    ...(nodes ? [["crossings", nodes] satisfies Readout] : []),
    [
      "seats",
      `${f.fills} on fills, ${f.voids} on voids, ${f.corners} on corners, reach ${fixed(f.reach, 2)} r, cell ${fixed(f.side, 4)} r`,
    ],
    ["path", `length ${fixed(f.total, 2)}, ${f.samples} samples a pencil`],
    ...(c ? [["cover", covered(c)] satisfies Readout] : []),
  ];
}

/* PAINT */

const mask = (shape: Pick<Shape, "side" | "mask">, color: string) => {
  const { side } = shape;
  const page = new OffscreenCanvas(side, side);
  const ink = page.getContext("2d")!;
  ink.fillStyle = color;
  for (let row = 0; row < side; row++) {
    for (let col = 0; col < side; col++) {
      if (shape.mask[row * side + col] !== SHAPE) continue;
      let run = 1;
      while (col + run < side && shape.mask[row * side + col + run] === SHAPE) run++;
      ink.fillRect(col, row, run, 1);
      col += run;
    }
  }
  return page;
};

const lean = (shape: Covered | null) =>
  shape && (Object.fromEntries<unknown>(Object.entries(shape).filter(([key]) => key !== "mask")) as Lean);

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("spirograph: the scene wants @mrlyprod/mrly/math as opts.math");
  const tell = typeof opts.tell === "function" ? opts.tell : null;
  const lock = Boolean(view.lock);
  const rolled = lock && !opts.code;
  const ctx = canvas.getContext("2d")!;
  const read = (raw: Opts) =>
    ({ ...tidy(SPEC, raw), ...(lock && !raw.ink ? { ink: "one" } : {}), seed: Number(raw.seed) >>> 0 }) as Spirograph;
  const clock = () => globalThis.performance?.now?.() ?? 0;
  let value = read(opts);
  let plan: Plan | null = null;
  let fault: string | null = null;
  let shape: Covered | null = null;
  let sheet: OffscreenCanvas | null = null;
  let ms = 0;
  let length = LEAST;
  let part = lock ? 0 : value.at / 100;
  let playing = false;
  let from = 0;
  let origin = 0;
  let stale = false;
  let dirty = true;
  let shown = -1;
  let drawn = -1;
  let layer: OffscreenCanvas | null = null;
  let pen: OffscreenCanvasRenderingContext2D | null = null;
  let stamp: OffscreenCanvas | null = null;
  let box: Fit | null = null;
  let accent = "#000";
  let shades: string[] = [];
  let of: number[] = [];
  let groups: number[][] = [];
  let last: Now | null = null;
  const report = () => {
    if (!tell) return;
    const now: Now = { plan, fault, shape, ms, playing, pct: Math.round(part * 100) };
    if (last && (Object.keys(now) as (keyof Now)[]).every((key) => now[key] === last![key])) return;
    last = now;
    tell({ facts: plan!.facts, fault, cover: lean(shape), ms, part, playing });
  };
  const learn = (next: Spirograph) => {
    const began = clock();
    try {
      plan = study(math, next, rolled ? roll(math, next.base, view.rand) : resolve(math, next));
      fault = null;
    } catch (error) {
      fault = message(error);
      ms = clock() - began;
      if (plan) return false;
      const first = read({ seed: next.seed, ink: next.ink });
      plan = study(math, first, resolve(math, first));
    }
    length = span(plan.beats) * (lock ? EASE : 1);
    shape = next.fill ? cover(math, plan) : null;
    ms = clock() - began;
    return true;
  };
  const paint = () => {
    const look = view.look();
    accent = look.accent;
    if (value.ink === "one") {
      const a = rgb(look.accent);
      const p = rgb(look.paper);
      shades = Array.from({ length: BANDS }, (_, b) => mix(a, p, FADE * (1 - b / (BANDS - 1))));
      of = plan!.bands;
    } else {
      shades = (value.ink === "wheel" ? SIX : KIND).map((hue) => paints(canvas, hue).accent);
      of = value.ink === "wheel" ? plan!.pencils.map((_, k) => k % SIX.length) : plan!.kinds;
    }
    groups = shades.map(() => []);
    of.forEach((c, k) => {
      groups[c].push(k);
    });
    sheet = shape && !shape.fault ? mask(shape as Shape, veil(accent, FILL)) : null;
    layer = null;
    dirty = true;
  };
  const spot = (k: number, i: number): [number, number] => {
    const j = (k * plan!.samples + i) * 2;
    return [box!.ox + plan!.trace[j] * box!.k, box!.oy - plan!.trace[j + 1] * box!.k];
  };
  const rail = (target: OffscreenCanvasRenderingContext2D, dpr: number) => {
    const { outline, closed } = plan!.track;
    if (!outline.length) return;
    target.beginPath();
    for (const [i, [x, y]] of outline.entries()) {
      if (i) target.lineTo(box!.ox + x * box!.k, box!.oy - y * box!.k);
      else target.moveTo(box!.ox + x * box!.k, box!.oy - y * box!.k);
    }
    if (closed) target.closePath();
    target.strokeStyle = veil(accent, TRACK);
    target.lineWidth = dpr;
    target.setLineDash(DASH.map((d) => d * dpr));
    target.stroke();
    target.setLineDash([]);
  };
  const press = () => {
    const { cell, side, track } = plan!;
    const [height, width] = cell.shape;
    const unit = side * track.wheel * box!.k;
    const edge = Math.ceil(Math.max(width, height) * unit) + 2;
    stamp = new OffscreenCanvas(edge, edge);
    const ink = stamp.getContext("2d")!;
    ink.fillStyle = veil(accent, CELLS);
    const gap = unit * INSET;
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        if (cell.types[row * width + col])
          ink.fillRect(
            edge / 2 + (col - width / 2) * unit + gap,
            edge / 2 + (row - height / 2) * unit + gap,
            unit - 2 * gap,
            unit - 2 * gap,
          );
      }
    }
  };
  const reset = () => {
    box = fit(plan!.frame, view.w, view.h, PAD * view.dpr);
    press();
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext("2d")!;
    pen.lineJoin = "round";
    pen.lineCap = "round";
    rail(pen, view.dpr);
    pen.lineWidth = LINE * view.dpr;
    drawn = -1;
  };
  const extend = (to: number) => {
    if (to <= drawn) return;
    const start = Math.max(0, drawn);
    groups.forEach((group, c) => {
      if (!group.length) return;
      pen!.beginPath();
      for (const k of group) {
        const [x, y] = spot(k, start);
        pen!.moveTo(x, y);
        for (let i = start + 1; i <= to; i++) {
          const [u, v] = spot(k, i);
          pen!.lineTo(u, v);
        }
      }
      pen!.strokeStyle = shades[c];
      pen!.stroke();
    });
    drawn = to;
  };
  const fill = () => {
    const { x, y, radius } = (shape as Shape).disc;
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(
      sheet!,
      box!.ox + (x - radius) * box!.k,
      box!.oy - (y + radius) * box!.k,
      2 * radius * box!.k,
      2 * radius * box!.k,
    );
    ctx.imageSmoothingEnabled = true;
  };
  const wheel = (i: number) => {
    const { track, samples, side } = plan!;
    const s = (track.total * i) / (samples - 1);
    const [cx, cy] = math.spirograph.pose(track, s);
    const phi = math.spirograph.turn(track, s);
    const dpr = view.dpr;
    const x = box!.ox + cx * box!.k;
    const y = box!.oy - cy * box!.k;
    ctx.lineWidth = dpr;
    ctx.strokeStyle = veil(accent, RING);
    ctx.beginPath();
    ctx.arc(x, y, track.wheel * box!.k, 0, TAU);
    ctx.stroke();
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(-phi);
    ctx.drawImage(stamp!, -stamp!.width / 2, -stamp!.height / 2);
    ctx.restore();
    const dot = Math.max(0.75 * dpr, Math.min(DOT * dpr, SPOT * side * track.wheel * box!.k));
    groups.forEach((group, c) => {
      if (!group.length) return;
      ctx.fillStyle = shades[c];
      ctx.beginPath();
      for (const k of group) {
        const [px, py] = spot(k, i);
        ctx.moveTo(px + dot, py);
        ctx.arc(px, py, dot, 0, TAU);
      }
      ctx.fill();
    });
    ctx.fillStyle = accent;
    ctx.beginPath();
    ctx.arc(x, y, 0.8 * DOT * dpr, 0, TAU);
    ctx.fill();
  };
  const draw = () => {
    if (stale) {
      stale = false;
      if (learn(value)) {
        paint();
        if (playing) {
          from = part;
          origin = view.t;
        }
      }
      report();
    }
    if (lock) part = share(view.t, length, view.still);
    else if (playing) {
      part = played(from, view.t - origin, length);
      if (part >= 1) playing = false;
    }
    const i = index(part, plan!.samples);
    if (dirty || i !== shown) {
      if (!layer || i < drawn) reset();
      extend(i);
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.clearRect(0, 0, view.w, view.h);
      if (sheet) fill();
      ctx.drawImage(layer!, 0, 0);
      wheel(i);
      shown = i;
      dirty = false;
    }
    report();
  };
  const start = () => {
    from = part >= 1 ? 0 : part;
    origin = view.t;
    playing = true;
    dirty = true;
    if (view.still) view.wake?.();
    report();
  };
  const halt = () => {
    playing = false;
    part = parked(part) / 100;
    dirty = true;
    report();
  };
  const toggle = () => (playing ? halt() : start());
  const take = () => {
    if (!playing) start();
  };
  const set = (raw: Opts) => {
    const next = read(raw);
    const was = value;
    value = next;
    if (STUDY.some((key) => next[key] !== was[key])) stale = true;
    else if (next.ink !== was.ink || next.fill !== was.fill) {
      if (next.fill !== was.fill) shape = next.fill ? cover(math, plan!) : null;
      paint();
    }
    if (next.at !== was.at) {
      playing = false;
      part = next.at / 100;
    }
    dirty = true;
    report();
  };
  const theme = () => paint();
  const size = () => {
    layer = null;
    dirty = true;
  };
  const picture = () => {
    const { track, pencils, trace, samples } = plan!;
    const css = fit(plan!.frame, view.w / view.dpr, view.h / view.dpr, PAD);
    const to = (x: number, y: number): [number, number] => [css.ox + x * css.k, css.oy - y * css.k];
    const upto = Math.max(1, index(part, samples));
    const strokes: Stroke[] = [
      {
        points: track.outline.flatMap(([x, y]) => to(x, y)),
        color: veil(accent, TRACK),
        width: 1,
        dash: DASH,
        closed: track.closed,
      },
    ];
    pencils.forEach((_, k) => {
      const points = new Float64Array((upto + 1) * 2);
      for (let i = 0; i <= upto; i++)
        [points[i * 2], points[i * 2 + 1]] = to(trace[(k * samples + i) * 2], trace[(k * samples + i) * 2 + 1]);
      strokes.push({ points, color: shades[of[k]], width: LINE });
    });
    return lines({ width: view.w / view.dpr, height: view.h / view.dpr, strokes });
  };
  learn(value);
  paint();
  report();
  return { draw, size, theme, set, toggle, take, playing: () => playing, svg: picture };
}
