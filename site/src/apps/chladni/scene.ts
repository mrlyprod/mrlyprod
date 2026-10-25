import type { Page } from "../../lib/app/page.tsx";
import type { Box, Opts, View } from "../../lib/app/run.ts";
import type { Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { TINTS } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { rng } from "../../lib/rand.ts";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Absent, Asked, Fault, Figure, Inks, Num, Raster, Shape, Units } from "./engine.ts";
import {
  base,
  CHUNK,
  COUNT,
  codes,
  fill,
  HOLD,
  highest,
  inks,
  keyOf,
  known,
  ladder,
  live,
  lowest,
  NUMBERS,
  POUR,
  paint,
  RATE,
  REST,
  ROOM,
  raster,
  rungOf,
  scatter,
  sprinkle,
  stamp,
  TAP,
  TOP,
} from "./engine.ts";

/* TYPES */

export type Knobs = Asked & { look: string; grains: number; tint: string; seed: number };

export type Woven = {
  view: "carpet";
  k: number;
  count: number;
  lambda: number;
  domains: number;
  bound: number;
  grains: number;
};

export type Flat = { view: "plate"; n: number; m: number; sign: number; pitch: number | null; grains: number };

export type Stat =
  | (Fault & Absent<Woven> & Absent<Flat>)
  | (Woven & Absent<Fault> & Absent<Omit<Flat, keyof Woven>>)
  | (Flat & Absent<Fault> & Absent<Omit<Woven, keyof Flat>>);

export type Step = Figure | { k: number };

export type Given = Opts & {
  num?: Num | null;
  math?: Mrly | null;
  onTell?: (stat: Stat) => void;
  onWalk?: (patch: Step) => void;
};

export type Chladni = ReturnType<typeof make>;

export type Fit = { x: number; y: number; side: number };

type Sheet = {
  side: number;
  canvas: OffscreenCanvas;
  pen: OffscreenCanvasRenderingContext2D;
  image: ImageData;
  words: Uint32Array;
};

type Walk = { list: Step[]; from: number; t: number; j: number };

type Spot = { x: number; y: number };

const VIEWS: [string, string][] = [
  ["plate", "Plate"],
  ["carpet", "Carpet"],
];
const LOOKS: [string, string][] = [
  ["sand", "Sand"],
  ["field", "Field"],
  ["lines", "Lines"],
];
const GRAINS: [number, string][] = [
  [10000, "10k"],
  [20000, "20k"],
  [40000, "40k"],
  [80000, "80k"],
];

const HIDE = () => false;

const plate = (value: Value) => value.view !== "carpet";
const carpet = (value: Value) => value.view === "carpet";
const sanded = (value: Value) => value.look === "sand";

export const SPEC: Spec<Units> = [
  { key: "view", label: "View", kind: "segment", def: "plate", options: VIEWS, group: "Figure" },
  { key: "n", label: "n", kind: "slider", def: 3, min: 0, max: TOP, step: 1, when: plate, group: "Figure" },
  { key: "m", label: "m", kind: "slider", def: 5, min: 0, max: TOP, step: 1, when: plate, group: "Figure" },
  {
    key: "sign",
    label: "Sign",
    kind: "segment",
    def: -1,
    options: [
      [1, "+"],
      [-1, "-"],
    ],
    when: (value) => plate(value) && value.n !== value.m,
    group: "Figure",
  },
  {
    key: "number",
    label: "Number",
    kind: "pick",
    def: 3,
    options: NUMBERS.map((n) => [n, String(n)]),
    when: carpet,
    group: "Figure",
  },
  { key: "code", label: "Design", kind: "pick", def: "7", options: codes, when: HIDE, group: "Figure" },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: lowest,
    max: highest,
    step: 1,
    when: carpet,
    group: "Figure",
  },
  {
    key: "k",
    label: "k",
    kind: "slider",
    def: 8,
    min: 1,
    max: (value) => known(value) ?? COUNT,
    step: 1,
    keep: true,
    when: carpet,
    group: "Figure",
  },
  { key: "look", label: "Look", kind: "segment", def: "sand", options: LOOKS, group: "Look" },
  { key: "grains", label: "Grains", kind: "segment", def: 40000, options: GRAINS, when: sanded, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A square plate rings in a figure: where it moves, sand bounces off, and where it stays still, the sand piles up. The still lines are the zero set of cos(n pi x) cos(m pi y) plus or minus cos(m pi x) cos(n pi y); n, m and the sign pick the figure, and Play walks the figures in order of pitch, the sand settling on each.",
  "Carpet cuts the plate as a design: its cells are the plate and its holes are gone. The figures are the lowest modes of the graph of its cells, k in order of pitch; sand on a hole falls through, and a new figure pours in fresh sand to make up the loss. The status counts the nodal domains of figure k against the bound Courant gives it.",
  "Sand shakes the grains, Field paints the signed amplitude and Lines traces the zero set alone; at a saddle Lines may join two domains. A tap or a drag sprinkles sand, and Reroll sprinkles afresh. The counts are in [the automata note](/research/automata/).",
];

const signed = (value: Value) => (value.n === value.m ? "" : Number(value.sign) > 0 ? "p" : "m");

export const named = (value: Value) =>
  carpet(value)
    ? `chladni-${value.code}-${value.number}-${value.level}-k${value.k}-${value.seed}`
    : `chladni-${value.n}-${value.m}${signed(value)}-${value.seed}`;

export const units = wasm("num", "math");

export const PAGE = {
  design: { dim: 2, base: 2, code: "code", when: carpet },
  spec: SPEC,
  units,
  record: true,
  gestures: true,
  about: ABOUT,
  name: named,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Pick<Chladni, "walking"> | null) => (scene?.walking?.() ? "Stop" : "Play"),
      act: "walk",
      button: true,
      menu: "Edit",
    },
  ],
  actions: { walk: (scene) => scene.walk?.() },
} satisfies Page<Loaded<"num" | "math">, Chladni>;

const FIT = 0.94;
const LINE = 1.5;
const TOLD = 100;

const sorts = (value: Knobs) =>
  [value.seed, value.view, value.grains, carpet(value) ? [value.code, value.number, value.level].join(" ") : ""].join(
    "|",
  );

export function fit(view: Box, side?: number): Fit {
  const most = Math.max(1, Math.floor(Math.min(view.w, view.h) * FIT));
  const edge = side && most >= side ? Math.floor(most / side) * side : most;
  return { x: Math.floor((view.w - edge) / 2), y: Math.floor((view.h - edge) / 2), side: edge };
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const { num, math } = opts;
  if (!num) throw new Error("chladni: the scene wants @mrlyprod/mrly/num as opts.num");
  const units = { num, math };
  const ctx = canvas.getContext("2d")!;
  const read = (next: Opts) => ({ ...tidy(SPEC, next, units), seed: Number(next.seed) >>> 0 }) as Knobs;
  let value = read(opts);
  let shape: Raster | null = null;
  let grains: Float32Array = new Float32Array(0);
  let done = 0;
  let hand = 0;
  let clock = { t: 0, step: 0 };
  let walk: Walk | null = null;
  let ink: Inks | null = null;
  let face: Sheet | null = null;
  let field: Sheet | null = null;
  let trace: Float64Array | null = null;
  let last: Spot | null = null;
  let dirty = true;
  let told = -Infinity;
  let said = "";
  let gone = false;
  let pressed = false;
  let woke = false;
  const pages: Record<string, Sheet> = {};
  const page = (kind: string, side: number) => {
    if (pages[kind]?.side !== side) {
      const one = new OffscreenCanvas(side, side);
      const image = new ImageData(side, side);
      pages[kind] = { side, canvas: one, pen: one.getContext("2d")!, image, words: new Uint32Array(image.data.buffer) };
    }
    return pages[kind];
  };
  const calm = () => view.still || (woke && !walk);
  const dress = () => {
    const next = inks(view.look());
    if (ink && JSON.stringify(next) === JSON.stringify(ink)) return;
    ink = next;
    face = null;
    field = null;
    dirty = true;
  };
  const sync = () => {
    if (shape?.key === keyOf(value)) return;
    const want = raster(units, value);
    if (shape?.mask !== want.mask) face = null;
    shape = want;
    field = null;
    trace = null;
    dirty = true;
  };
  const steps = (to: number) => {
    if (shape!.error) return;
    while (done < to) {
      const n = Math.min(CHUNK, to - done);
      grains = num.chladni.shake(shape!.data!, shape!.side!, grains, done, n, value.seed);
      done += n;
      dirty = true;
    }
  };
  const anchor = (t = view.t) => {
    clock = { t, step: done };
  };
  const touched = (t = view.t) => {
    anchor(t);
    hand = done;
  };
  const due = (t: number) => Math.min(hand + REST, clock.step + Math.floor(((t - clock.t) * RATE) / 1000));
  const run = (t: number) => {
    if (!sanded(value) || shape!.error) return;
    if (!calm()) steps(due(t));
    else if (!pressed) steps(hand + REST);
  };
  const epoch = (rested: boolean) => {
    grains = shape!.error ? new Float32Array(0) : sprinkle(rng(value.seed), shape!.data!, shape!.side!, value.grains);
    done = 0;
    hand = 0;
    anchor();
    dirty = true;
    if (rested) steps(REST);
  };
  const order = (): Step[] =>
    carpet(value)
      ? Array.from({ length: Math.max(0, (shape!.made?.values.length ?? 0) - 1) }, (_, i) => ({ k: i + 2 }))
      : ladder(num).map(({ n, m, sign }) => ({ n, m, sign }));
  const place = () => (carpet(value) ? (shape!.k ?? 2) - 2 : rungOf(num, value));
  const top = () => {
    const want = value.grains - live(grains);
    if (shape!.error || want <= 0) return;
    grains = fill(grains, sprinkle(view.rand, shape!.data!, shape!.side!, want), value.grains);
  };
  const turned = (t?: number) => {
    sync();
    top();
    touched(t);
  };
  const show = (next: Step, t: number) => {
    value = { ...value, ...next };
    turned(t);
    opts.onWalk?.(next);
  };
  const stride = () => {
    if (!walk) return;
    const j = Math.floor((view.t - walk.t) / HOLD);
    if (j <= walk.j) return;
    const at = walk.t + j * HOLD;
    run(at);
    walk.j = j;
    show(walk.list[(walk.from + j) % walk.list.length], at);
  };
  const rebase = (next: number) => {
    const list = order();
    if (!list.length) {
      walk = null;
      return;
    }
    walk = { list, from: (place() + next + list.length) % list.length, t: view.t, j: next ? -1 : 0 };
  };
  const stat = (): Stat => {
    if (shape!.error) return { error: shape!.error };
    const left = live(grains);
    if (carpet(value)) {
      const i = shape!.k! - 1;
      return {
        view: "carpet",
        k: shape!.k!,
        count: shape!.made!.values.length,
        lambda: shape!.made!.values[i],
        domains: shape!.made!.domains[i],
        bound: shape!.made!.bound[i],
        grains: left,
      };
    }
    const rung = ladder(num)[rungOf(num, value)];
    return {
      view: "plate",
      n: value.n,
      m: value.m,
      sign: value.n === value.m ? 0 : value.sign,
      pitch: rung?.pitch ?? null,
      grains: left,
    };
  };
  const tell = (force: boolean) => {
    if (gone || (!force && view.t - told < TOLD)) return;
    const now = stat();
    const key = JSON.stringify(now);
    told = view.t;
    if (key === said) return;
    said = key;
    opts.onTell?.(now);
  };
  const ground = (box: Fit) => {
    const one = page("mask", shape!.side!);
    if (face !== one) {
      face = one;
      one.words.set(base(shape as Shape, ink!));
      one.pen.putImageData(one.image, 0, 0);
    }
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(one.canvas, box.x, box.y, box.side, box.side);
  };
  const sand = (box: Fit) => {
    const side = Math.max(1, Math.round(box.side / view.dpr));
    const one = page("sand", side);
    one.words.fill(0);
    stamp(one.words, grains, side, ink!.sand);
    one.pen.putImageData(one.image, 0, 0);
    ctx.drawImage(one.canvas, box.x, box.y, box.side, box.side);
  };
  const lines = (box: Fit) => {
    trace ??= num.chladni.lines(shape!.data!, shape!.side!);
    ctx.strokeStyle = ink!.ink;
    ctx.lineWidth = LINE * view.dpr;
    ctx.lineCap = "round";
    ctx.beginPath();
    for (let i = 0; i < trace.length; i += 4) {
      ctx.moveTo(box.x + trace[i] * box.side, box.y + trace[i + 1] * box.side);
      ctx.lineTo(box.x + trace[i + 2] * box.side, box.y + trace[i + 3] * box.side);
    }
    ctx.stroke();
  };
  const amplitude = (box: Fit) => {
    const one = page("field", shape!.side!);
    if (field !== one) {
      field = one;
      one.image.data.set(paint(shape as Shape, ink!));
      one.pen.putImageData(one.image, 0, 0);
    }
    ctx.imageSmoothingEnabled = shape!.smooth!;
    ctx.drawImage(one.canvas, box.x, box.y, box.side, box.side);
  };
  const frame = () => {
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (shape!.error) return;
    const box = fit(view, shape!.smooth ? 0 : shape!.side);
    if (value.look === "field") amplitude(box);
    else ground(box);
    if (sanded(value)) sand(box);
    if (value.look === "lines") lines(box);
  };
  const draw = () => {
    if (view.fixed && !walk) march();
    stride();
    sync();
    run(view.t);
    if (!dirty) return;
    frame();
    tell(done === hand + REST);
  };
  const march = () => {
    if (walk) {
      walk = null;
      return false;
    }
    rebase(1);
    if (!walk) return false;
    woke ||= view.still;
    view.wake?.();
    stride();
    return true;
  };
  const set = (next: Opts) => {
    const was = value;
    value = read(next);
    if (sorts(value) !== sorts(was)) {
      sync();
      epoch(false);
      if (walk) rebase(0);
    } else if (keyOf(value) !== keyOf(was)) {
      turned();
      if (walk) rebase(0);
    }
    if (sanded(value) && !sanded(was)) anchor();
    dress();
    dirty = true;
    tell(true);
  };
  const touch = (u: number, v: number, start: boolean) => {
    if (!sanded(value) || shape!.error || (!start && !last)) return null;
    if (start) pressed = true;
    const box = fit(view, shape!.smooth ? 0 : shape!.side);
    const x = (u * view.w - box.x) / box.side;
    const y = (v * view.h - box.y) / box.side;
    const css = box.side / view.dpr;
    const count = start ? TAP : Math.floor(Math.hypot(x - last!.x, y - last!.y) * css * POUR);
    if (!start && !count) return null;
    last = { x, y };
    const points = scatter(view.rand, x, y, count, shape!.data!, shape!.side!);
    if (!points.length) return null;
    grains = fill(grains, points, ROOM * value.grains);
    touched();
    dirty = true;
    draw();
    tell(true);
    return null;
  };
  const lift = () => {
    if (!pressed) return null;
    pressed = false;
    last = null;
    draw();
    tell(true);
    return null;
  };
  const size = () => {
    dirty = true;
  };
  const theme = () => dress();
  const stop = () => {
    gone = true;
  };
  dress();
  sync();
  epoch(true);
  return { draw, set, size, theme, stop, touch, lift, stat, walk: march, walking: () => Boolean(walk) };
}
