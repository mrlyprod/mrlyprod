import type { Paints } from "@mrlyprod/ui/kit/color.ts";
import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Patch, Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import type { Facts, Geo, Marking, Num, Pair, Plan, Quadratic, Setting, Tones } from "./engine.ts";
import {
  DIM,
  EDGE,
  fitted,
  HEX,
  HIT,
  ink,
  LATTICES,
  LEADS,
  LIT,
  landed,
  lay,
  MARKS,
  MISS,
  palette,
  picture,
  quadratic,
  REACH,
  read,
  SIDES,
  SOIL,
  share,
  shuffle,
  span,
  step,
  study,
  table,
  toned,
} from "./engine.ts";

export type Knobs = Quadratic & Marking & { tint: string };

export type Off = { off: true; x?: undefined; y?: undefined };

export type Spot = {
  n: number;
  x: number;
  y: number;
  ring: number;
  prime: boolean;
  factors: [number, number][];
  off?: undefined;
};

export type Picked = Off | Spot;

export type Told = Facts & { pick: Picked | null };

export type Seen = Pick<View, "w" | "h" | "dpr" | "t" | "still"> & { look: () => Pick<Paints, "paper" | "accent"> };

export type Given = Opts & { num?: Num | null; onFacts?: (facts: Told) => void };

export type Ulam = ReturnType<typeof make>;

export const units = wasm("num");

const line = (value: Value, by: number, num: Num): Patch => ({
  c: Number(value.a) > 0 ? step(num, Number(value.c), by, REACH) : Number(value.c),
});

type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

export const SPEC: Spec = [
  { key: "lattice", label: "Lattice", kind: "segment", def: "square", options: LATTICES, group: "Sheet" },
  { key: "side", label: "Side", kind: "slider", def: 201, min: SIDES[0], max: SIDES[1], step: 2, group: "Sheet" },
  { key: "a", label: "a", kind: "segment", def: 4, options: LEADS, group: "Sheet" },
  { key: "b", label: "b", kind: "number", def: -2, min: -REACH, max: REACH, step: 1, group: "Sheet" },
  { key: "c", label: "c", kind: "number", def: 41, min: -REACH, max: REACH, step: 1, group: "Sheet" },
  { key: "mark", label: "Mark", kind: "pick", def: "prime", options: MARKS, group: "Mark" },
  { key: "faint", label: "Faint composites", kind: "toggle", def: 1, group: "Mark" },
  { key: "tint", label: "Tint", kind: "pick", def: "yellow", options: TINTS, group: "Mark" },
];

const lined = (value: Value) => (value.a as number) > 0;

const term = (k: number, word: string) => (k ? `${k < 0 ? "-" : "+"}${Math.abs(k)}${word}` : "");

export const named = (value: Value) =>
  [
    "ulam",
    value.lattice,
    value.side,
    ...(lined(value) ? [`${value.a}k2${term(value.b as number, "k")}${term(value.c as number, "")}`] : []),
    ...(value.mark === "prime" ? [] : [value.mark]),
  ].join("-");

export const PAGE = {
  spec: SPEC,
  units,
  name: named,
  keys: [
    { key: "Enter", label: "Next c", act: "next", button: true, when: lined, menu: "Edit" },
    { key: ["ArrowLeft", "ArrowRight"], label: "Step c", act: "line", when: lined },
  ],
  acts: {
    next: (value, _key, units) => line(value, 1, units!.num),
    line: (value, key, units) => line(value, key === "ArrowLeft" ? -1 : 1, units!.num),
  },
  reroll: (_value, seed, units) => shuffle(units!.num, rng(seed)),
} satisfies Page<Loaded<"num">, Ulam>;

const GAP = 8;
const INSET = 0.07;
const RING = 0.8;
const HALO = 6;
const CHUNK = 4096;
const TURN = Math.PI * 2;
const THUMB = 53;

function outline(p: Pen, geo: Geo, dpr: number, x: number, y: number) {
  const px = geo.px;
  if (geo.lattice === "hex") {
    const r = geo.s * (px >= GAP * dpr ? 1 - INSET : 1);
    p.moveTo(x + HEX[0][0] * r, y + HEX[0][1] * r);
    for (let i = 1; i < HEX.length; i++) p.lineTo(x + HEX[i][0] * r, y + HEX[i][1] * r);
    p.closePath();
    return;
  }
  const g = px >= GAP * dpr ? Math.max(1, Math.round(px * INSET)) : 0;
  const left = Math.round(x - px / 2) + g;
  const top = Math.round(y - px / 2) + g;
  p.rect(left, top, Math.round(x + px / 2) - g - left, Math.round(y + px / 2) - g - top);
}

function stamp(p: Pen, geo: Geo, dpr: number, color: string, cells: readonly Pair[]) {
  p.fillStyle = color;
  let open = 0;
  for (const [x, y] of cells) {
    if (!open) p.beginPath();
    outline(p, geo, dpr, ...geo.at(x, y));
    if (++open >= CHUNK) {
      p.fill();
      open = 0;
    }
  }
  if (open) p.fill();
}

const where = (plan: Plan, ns: number[]): Pair[] => ns.map((n) => [plan.xs[n], plan.ys[n]]);

export function sketch(
  num: Num,
  one: Setting,
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  look: Pick<Paints, "paper" | "accent">,
) {
  const plan = study(num, tidy(SPEC, { ...one, side: THUMB }) as Knobs, w, h);
  const tones = palette(look.accent, look.paper);
  for (const t of [SOIL, DIM, LIT] as const) {
    const ns: number[] = [];
    for (let n = 1; n <= plan.top; n++) if (plan.tone[n] === t) ns.push(n);
    stamp(ctx, plan.geo, dpr, tones[t], where(plan, ns));
  }
  const { line } = plan;
  if (line)
    for (const hit of [true, false])
      stamp(
        ctx,
        plan.geo,
        dpr,
        tones[hit ? HIT : MISS],
        line.cells.filter((_, i) => line.hit[i] === hit),
      );
}

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const num = opts.num;
  if (!num) throw new Error("ulam: the scene wants @mrlyprod/num as opts.num");
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  const room = () => EDGE * view.dpr;
  let plan = study(num, value, view.w, view.h, room());
  const length = span(plan.rings);
  let whole = false;
  let layer: OffscreenCanvas | null = null;
  let pen: OffscreenCanvasRenderingContext2D | null = null;
  let drawn = 0;
  let shown = -1;
  let dirty = true;
  let picked: Picked | null = null;
  let known: Told | null = null;
  let paper: string;
  let tones: Tones;
  const paint = () => {
    const look = view.look();
    paper = look.paper;
    tones = palette(look.accent, paper);
  };
  const facts = () => (known = { ...read(num, plan, value), pick: picked });
  const tell = () => opts.onFacts?.(facts());
  const reset = () => {
    layer = new OffscreenCanvas(view.w, view.h);
    pen = layer.getContext("2d")!;
    drawn = 0;
  };
  const extend = (to: number) => {
    const { tone } = plan;
    const seen: number[] = [];
    for (let n = drawn + 1; n <= to; n++) if (tone[n]) seen.push(n);
    drawn = Math.max(drawn, to);
    for (const t of [SOIL, DIM, LIT] as const)
      stamp(
        pen!,
        plan.geo,
        view.dpr,
        tones[t],
        where(
          plan,
          seen.filter((n) => tone[n] === t),
        ),
      );
  };
  const overlay = (to: number) => {
    const { line } = plan;
    if (!line) return;
    const down = line.values.flatMap((n, i) => (n <= to ? [i] : []));
    for (const hit of [true, false])
      stamp(
        ctx,
        plan.geo,
        view.dpr,
        tones[hit ? HIT : MISS],
        down.filter((i) => line.hit[i] === hit).map((i) => line.cells[i]),
      );
  };
  const ring = () => {
    if (!picked || picked.off) return;
    const { geo } = plan;
    const [x, y] = geo.at(picked.x, picked.y);
    ctx.strokeStyle = ink(paper);
    ctx.lineWidth = 2 * view.dpr;
    ctx.beginPath();
    ctx.arc(x, y, Math.max(geo.px * RING, HALO * view.dpr), 0, TURN);
    ctx.stroke();
  };
  const draw = () => {
    const n = whole ? plan.top : landed(num, plan.name, plan.rings, share(view.t, length, view.still));
    if (layer && n === shown && !dirty) return;
    if (!layer) reset();
    extend(n);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.drawImage(layer!, 0, 0);
    overlay(n);
    ring();
    if (n >= plan.top) whole = true;
    shown = n;
    dirty = false;
  };
  const fresh = () => {
    layer = null;
    dirty = true;
  };
  const theme = () => {
    paint();
    fresh();
  };
  const size = () => {
    plan.geo = lay(num, plan.lattice, fitted(plan.lattice, plan.rings, view.w, view.h, room()), view.w, view.h);
    fresh();
  };
  const set = (next: Opts) => {
    const was = value;
    value = tidy(SPEC, next) as Knobs;
    const moved = (...keys: (keyof Knobs)[]) => keys.some((key) => value[key] !== was[key]);
    whole = true;
    if (moved("lattice", "side")) {
      plan = study(num, value, view.w, view.h, room());
      picked = null;
      fresh();
    } else {
      if (moved("mark", "faint")) {
        plan = { ...plan, ...toned(num, value, plan.top) };
        fresh();
      }
      if (moved("a", "b", "c")) plan = { ...plan, ...quadratic(num, value) };
    }
    if (moved("tint")) theme();
    dirty = true;
    tell();
  };
  const pick = (u: number, v: number) => {
    const { geo, name } = plan;
    const [x, y] = geo.cell(u * view.w, v * view.h);
    const at = Number(num.spiral.Lattice.ring_of(name, x, y));
    if (at > plan.rings) picked = { off: true };
    else if (picked && picked.x === x && picked.y === y) picked = null;
    else {
      const n = Number(num.spiral.Lattice.n(name, x, y));
      const pile = num.prime.pile(n);
      picked = { n, x, y, ring: at, prime: pile.prime, factors: pile.factors };
    }
    whole = true;
    dirty = true;
    draw();
    tell();
    return picked;
  };
  const svg = () => picture(plan, tones);
  const csv = () => (plan.line ? table(plan.line) : "");
  paint();
  facts();
  return {
    draw,
    size,
    theme,
    set,
    pick,
    svg,
    csv,
    get facts() {
      return known;
    },
  };
}
