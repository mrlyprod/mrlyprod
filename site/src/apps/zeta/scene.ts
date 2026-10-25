import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Paints } from "../../lib/color.ts";
import { blend, TINTS, veil } from "../../lib/color.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Units as Loaded } from "../../lib/units/index.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Facts, Inks, Look, Num, Reading, Rect, Setting } from "./engine.ts";
import {
  along,
  BEAD,
  BEAT,
  bands,
  CURVE,
  DASH,
  DOT,
  FADE,
  FILL,
  FLASH,
  FOLDING,
  fold,
  GHOST,
  HEAD,
  HOLD,
  hop,
  index,
  LINE,
  LOOKS,
  ladder,
  PAD,
  PER,
  PIP,
  passed,
  phase,
  picture,
  plane,
  REACH,
  RING,
  reading,
  STAIR,
  STEP,
  snap,
  study,
  TRAIL,
  table,
  UPTO,
  VIEWS,
  ZEROS,
} from "./engine.ts";

/* TYPES */

export type Knobs = Value & {
  view: Setting["view"];
  t: number;
  speed: number;
  zeros: number;
  x: number;
  look: Look;
  trail: number;
  tint: string;
};

export type Told = { t: number; walk: Reading; stairs: Facts };

export type Seen = Pick<View, "w" | "h" | "dpr" | "t" | "still" | "lock" | "fixed" | "wake"> & {
  look: () => Pick<Paints, "accent"> & Partial<Pick<Paints, "ink">>;
};

export type Given = Opts & { num?: Num | null; onTell?: (said: Told) => void };

export type Zeta = ReturnType<typeof make>;

const walks = (value: Value) => value.view !== "stairs";

const climbs = (value: Value) => !walks(value);

export const SPEC: Spec = [
  { key: "view", label: "View", kind: "segment", def: "walk", options: VIEWS, group: "Zeta" },
  {
    key: "t",
    label: "t",
    kind: "slider",
    def: 30,
    min: 0,
    max: REACH,
    step: STEP,
    show: (v) => v.toFixed(2),
    group: "Zeta",
    when: walks,
  },
  {
    key: "speed",
    label: "Speed",
    kind: "slider",
    def: 4,
    min: 1,
    max: 20,
    step: 1,
    unit: "t/s",
    group: "Zeta",
    when: walks,
  },
  { key: "zeros", label: "Zeros", kind: "slider", def: 10, min: 0, max: ZEROS, step: 1, group: "Zeta", when: climbs },
  { key: "x", label: "x", kind: "slider", def: 100, min: UPTO[0], max: UPTO[1], step: 1, group: "Zeta", when: climbs },
  { key: "look", label: "Path", kind: "segment", def: "line", options: LOOKS, group: "Look", when: walks },
  {
    key: "trail",
    label: "Trail",
    kind: "slider",
    def: 4,
    min: 1,
    max: 40,
    step: 1,
    unit: "t",
    group: "Look",
    when: walks,
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "Walk: put s = 1/2 + it and let t grow. Zeta draws a curve that runs straight through the origin, the yellow ring, once for every zero on the critical line.",
  "Stairs: psi(x), the yellow steps, adds ln p at every prime power up to x. The explicit formula with no zeros is the pink dashed guess; each zero you add folds the formula closer to the steps.",
  "Drag the walk to scrub t, drag the stairs to change the zeros, tap to play or fold, and press the arrow keys to hop from zero to zero.",
  "Every number comes from Rust: Euler-Maclaurin low on the line, Riemann-Siegel above it, and the zeros by bisection to a billionth.",
];

export const PAGE = {
  spec: SPEC,
  record: true,
  gestures: true,
  about: ABOUT,
  name: (value) => (walks(value) ? `zeta-walk-${value.t}` : `zeta-stairs-${value.x}-${value.zeros}`),
  keys: [
    {
      key: "Enter",
      label: (value: Value, scene: Pick<Zeta, "word"> | null) => scene?.word?.() ?? (walks(value) ? "Play" : "Fold"),
      act: "go",
      button: true,
      menu: "Edit",
    },
    { key: ["ArrowLeft", "ArrowRight"], label: "Zero to zero", act: "hop", when: walks, menu: "View" },
  ],
  actions: { go: (scene) => scene.go?.(), hop: (scene, e) => scene.hop?.(e.key === "ArrowLeft" ? -1 : 1) },
} satisfies Page<Loaded<"num">, Zeta>;

const between = (rand: Rng, lo: number, hi: number) => lo + Math.floor(rand() * (hi - lo + 1));

export const SAVERS = [
  {
    id: "walk",
    title: "Critical line",
    pick: (rand: Rng) => ({
      view: "walk",
      t: between(rand, 10, 120),
      speed: 2,
      look: pick(rand, LOOKS)[0],
      trail: between(rand, 6, 16),
    }),
  },
  {
    id: "waves",
    title: "Prime waves",
    pick: (rand: Rng) => ({ view: "stairs", zeros: between(rand, 12, 40), x: between(rand, 30, 150) }),
  },
];

export const units = wasm("num");

const TAU = Math.PI * 2;
const TELL = 125;
const SENT = 16;
const BURST = 0.08;
const FLARE = 3;
const SMALL = 8;
const ZERO = 3;
const KNOT = 2.5;
export const SLOW = 3;
export const DRIFT = 0.04;
export const DAWN = 0.5;
const STRIDE = 1;
export const SPREAD = 200;
export const CYCLE = 8;
const TONE = 0.5;

/* INKS */

export function inked(canvas: Element | null, accent: string, ink: string | null = null): Inks {
  const css = typeof getComputedStyle === "function" && canvas ? getComputedStyle(canvas) : null;
  const read = (name: string, fallback: string) => css?.getPropertyValue(name).trim() || fallback;
  const guess = read("--pink", palette.pink);
  if (ink) return { accent, zero: accent, head: blend(accent, ink, TONE), guess };
  return { accent, zero: read("--yellow", palette.yellow), head: read("--orange", palette.orange), guess };
}

/* WAVE */

export function hum(gammas: ArrayLike<number>, k: number, u: number) {
  let sum = 0;
  for (let j = 0; j < gammas.length && j < k; j++) {
    const g = gammas[j];
    sum += (Math.min(1, k - j) * (0.5 * Math.cos(g * u) + g * Math.sin(g * u))) / (0.25 + g * g);
  }
  return -2 * sum;
}

/* THUMB */

function line(
  ctx: CanvasRenderingContext2D,
  pts: number[],
  color: string,
  width: number,
  dash: number[] | null = null,
) {
  if (pts.length < 4) return;
  ctx.strokeStyle = color;
  ctx.lineWidth = width;
  if (dash) ctx.setLineDash(dash);
  ctx.beginPath();
  ctx.moveTo(pts[0], pts[1]);
  for (let j = 2; j < pts.length; j += 2) ctx.lineTo(pts[j], pts[j + 1]);
  ctx.stroke();
  if (dash) ctx.setLineDash([]);
}

export function sketch(
  num: Num,
  set: Setting,
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  dpr: number,
  inks: Inks,
) {
  const rect = { x: 0, y: 0, w, h };
  const pad = SMALL * dpr;
  ctx.lineJoin = "round";
  ctx.lineCap = "round";
  if (set.view === "stairs") {
    const steps = fold(num, set.x);
    const box = ladder(rect, steps.x, steps.top(set.zeros), pad);
    ctx.save();
    ctx.beginPath();
    ctx.rect(box.left, box.roof, box.right - box.left, box.floor - box.roof);
    ctx.clip();
    line(
      ctx,
      along(box, steps.grid, steps.curve(0)),
      inks.guess,
      dpr,
      DASH.map((d) => d * dpr),
    );
    line(ctx, along(box, steps.grid, steps.curve(set.zeros)), inks.accent, CURVE * dpr);
    line(ctx, along(box, steps.ladder.xs, steps.ladder.ys), inks.zero, STAIR * dpr);
    ctx.restore();
    return;
  }
  const plan = study(num);
  const i = index(set.t);
  const box = plane(rect, plan.peak[i], pad);
  const path: number[] = [];
  for (let j = 0; j <= i; j++) path.push(box.ox + plan.xs[j] * box.k, box.oy - plan.ys[j] * box.k);
  line(ctx, path, inks.accent, TRAIL * dpr);
  ctx.strokeStyle = inks.zero;
  ctx.lineWidth = dpr;
  ctx.beginPath();
  ctx.arc(box.ox, box.oy, ZERO * dpr, 0, TAU);
  ctx.stroke();
  ctx.fillStyle = inks.head;
  ctx.beginPath();
  ctx.arc(path[path.length - 2], path[path.length - 1], KNOT * dpr, 0, TAU);
  ctx.fill();
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const num = opts.num;
  if (!num) throw new Error("zeta: the scene wants @mrlyprod/mrly/num as opts.num");
  const ctx = canvas.getContext("2d")!;
  const plan = study(num);
  let value = tidy(SPEC, opts) as Knobs;
  let head = value.t;
  let walking = Boolean(view.lock);
  let clock: number | null = null;
  let rest: number | null = null;
  let folding: number | null = null;
  let rings: number[] = [];
  let inks: Inks;
  let shown = value.zeros;
  let toldAt = -Infinity;
  let sent: number[] = [];
  let gone = false;
  const auto = () => view.lock || view.fixed;
  const pace = view.lock ? SLOW : 1;
  const paint = () => {
    const { accent, ink } = view.look();
    inks = inked(canvas, accent, view.lock ? ink : null);
  };
  const busy = () => (walks(value) ? walking : folding !== null);
  const word = () => (!walks(value) ? "Fold" : walking ? "Pause" : head >= REACH ? "Replay" : "Play");
  const tell = (force = false) => {
    if (gone || !opts.onTell) return;
    if (!force && toldAt > -Infinity && (!busy() || view.t - toldAt < TELL)) return;
    toldAt = view.t;
    const t = snap(head);
    if (!sent.includes(t)) sent = [...sent, t].slice(-SENT);
    const steps = fold(num, value.x);
    opts.onTell({ t, walk: reading(num, plan, head), stairs: steps.facts(folding === null ? value.zeros : shown) });
  };
  const advance = () => {
    const now = view.t;
    const dt = clock === null ? 0 : Math.max(0, now - clock);
    clock = now;
    rings = rings.filter((at) => now - at <= FLASH);
    if (view.still || !walks(value)) return;
    const loop = auto();
    if (rest !== null) {
      if (!loop) rest = null;
      else if (now - rest >= HOLD) {
        rest = null;
        head = 0;
        walking = true;
      }
      return;
    }
    if (loop && !walking) {
      if (head >= REACH) head = 0;
      walking = true;
    }
    if (!walking) return;
    const before = passed(plan.zeros, head);
    head = Math.min(REACH, head + (value.speed * dt) / 1000);
    for (let k = passed(plan.zeros, head); k > before; k--) rings.push(now);
    if (head < REACH) return;
    walking = false;
    if (loop) rest = now;
    tell(true);
  };
  const folded = () => {
    const k = value.zeros;
    if (!walks(value) && auto() && folding === null) folding = view.t;
    if (folding === null) return { k, fade: 1 };
    const length = (k + 1) * Math.min(BEAT, FOLDING / (k + 1));
    if (auto()) {
      const { at, veil: fade } = phase((view.t - folding) / pace, length, view.still);
      return { k: Math.min(k, Math.floor(at * (k + 1))), fade, at };
    }
    const at = (view.t - folding) / length;
    if (at < 1) return { k: Math.min(k, Math.floor(at * (k + 1))), fade: 1 };
    folding = null;
    tell(true);
    return { k, fade: 1 };
  };
  const walker = (r: Rect, fade: number) => {
    const dpr = view.dpr;
    const i = index(head);
    const box = plane(r, plan.peak[i], PAD * dpr);
    const X = (j: number) => box.ox + plan.xs[j] * box.k;
    const Y = (j: number) => box.oy - plan.ys[j] * box.k;
    const every = Math.max(1, Math.round(PER * DOT));
    const lined = value.look !== "dots";
    const dotted = value.look !== "line";
    const run = (a: number, b: number, color: string, width: number) => {
      if (b <= a) return;
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.beginPath();
      ctx.moveTo(X(a), Y(a));
      for (let j = a + 1; j <= b; j++) ctx.lineTo(X(j), Y(j));
      ctx.stroke();
    };
    const beads = (a: number, b: number, color: string, radius: number) => {
      ctx.fillStyle = color;
      ctx.beginPath();
      for (let j = Math.ceil(a / every) * every; j <= b; j += every) {
        ctx.moveTo(X(j) + radius, Y(j));
        ctx.arc(X(j), Y(j), radius, 0, TAU);
      }
      ctx.fill();
    };
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    if (lined) run(0, i, veil(inks.accent, GHOST * fade), LINE * dpr);
    else beads(0, i, veil(inks.accent, GHOST * fade), BEAD * dpr);
    ctx.lineCap = "butt";
    for (const { from: a, to: b, first, alpha } of bands(i, Math.round(value.trail * PER))) {
      if (lined) run(a, b, veil(inks.accent, alpha * fade), TRAIL * dpr);
      if (dotted) beads(first, b, veil(inks.accent, alpha * fade), PIP * dpr);
    }
    ctx.strokeStyle = veil(inks.zero, fade);
    ctx.lineWidth = 2 * dpr;
    ctx.beginPath();
    ctx.arc(box.ox, box.oy, RING * dpr, 0, TAU);
    ctx.stroke();
    const reach = BURST * Math.min(r.w, r.h);
    for (const at of view.still ? [] : rings) {
      const k = (view.t - at) / FLASH;
      ctx.strokeStyle = veil(inks.zero, 1 - k);
      ctx.lineWidth = dpr + FLARE * dpr * (1 - k);
      ctx.beginPath();
      ctx.arc(box.ox, box.oy, RING * dpr + reach * (1 - (1 - k) ** 2), 0, TAU);
      ctx.stroke();
    }
    ctx.fillStyle = veil(inks.head, fade);
    ctx.beginPath();
    ctx.arc(X(i), Y(i), HEAD * dpr, 0, TAU);
    ctx.fill();
  };
  const stairs = (r: Rect) => {
    const dpr = view.dpr;
    const steps = fold(num, value.x);
    const { k, fade } = folded();
    shown = k;
    const box = ladder(r, steps.x, steps.top(value.zeros), PAD * dpr);
    const edge = along(box, steps.ladder.xs, steps.ladder.ys);
    ctx.save();
    ctx.beginPath();
    ctx.rect(box.left, box.roof, box.right - box.left, box.floor - box.roof);
    ctx.clip();
    ctx.lineJoin = "round";
    ctx.beginPath();
    ctx.moveTo(edge[0], edge[1]);
    for (let j = 2; j < edge.length; j += 2) ctx.lineTo(edge[j], edge[j + 1]);
    ctx.lineTo(box.left + (steps.x - 1) * box.kx, box.floor);
    ctx.lineTo(box.left, box.floor);
    ctx.closePath();
    ctx.fillStyle = veil(inks.zero, FILL);
    ctx.fill();
    line(
      ctx,
      along(box, steps.grid, steps.curve(0)),
      inks.guess,
      dpr,
      DASH.map((d) => d * dpr),
    );
    line(ctx, along(box, steps.grid, steps.curve(k)), veil(inks.accent, fade), CURVE * dpr);
    line(ctx, edge, inks.zero, STAIR * dpr);
    ctx.restore();
  };
  const waves = (r: Rect) => {
    const dpr = view.dpr;
    const { at = 1, fade } = folded();
    const span = Math.min(Math.log(value.x) - DAWN, r.w / (dpr * SPREAD));
    const from = DAWN + (view.still ? 0 : (DRIFT * view.t) / 1000);
    const n = Math.max(2, Math.ceil(r.w / (STRIDE * dpr)));
    const k = Math.min(at * value.zeros, passed(plan.zeros, (TAU * n) / (CYCLE * span)));
    const mid = r.y + r.h / 2;
    const ky = Math.max(0, r.h / 2 - PAD * dpr);
    ctx.strokeStyle = veil(inks.accent, fade);
    ctx.lineWidth = STAIR * dpr;
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    ctx.beginPath();
    for (let i = 0; i <= n; i++) {
      const s = i / n;
      const y = mid - hum(plan.zeros, k, from + s * span) * ky;
      if (i) ctx.lineTo(r.x + s * r.w, y);
      else ctx.moveTo(r.x, y);
    }
    ctx.stroke();
  };
  const draw = () => {
    advance();
    const fade = rest === null ? 1 : Math.max(0, Math.min(1, (HOLD - (view.t - rest)) / FADE));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    const stage = { x: 0, y: 0, w: view.w, h: view.h };
    if (walks(value)) walker(stage, fade);
    else if (view.lock) waves(stage);
    else stairs(stage);
    tell();
  };
  const wake = () => {
    if (view.still) view.wake?.();
    else draw();
  };
  const refold = () => {
    folding = view.t;
    wake();
    tell(true);
  };
  const go = () => {
    if (!walks(value)) return refold();
    if (walking) walking = false;
    else {
      if (head >= REACH) head = 0;
      walking = true;
      rest = null;
      clock = view.t;
    }
    wake();
    tell(true);
  };
  const toggle = () => {
    if (walks(value) || folding === null) return go();
    folding = null;
    draw();
    tell(true);
  };
  const jump = (dir: number) => {
    if (!walks(value)) return;
    head = hop(plan.zeros, head, dir);
    walking = false;
    rest = null;
    if (plan.zeros.includes(head)) rings.push(view.t);
    draw();
    tell(true);
  };
  const set = (next: Opts) => {
    const was = value;
    value = tidy(SPEC, { ...opts, ...next }) as Knobs;
    const echo = sent.indexOf(value.t);
    const moved = echo < 0 && value.t !== was.t;
    if (echo >= 0) sent = sent.slice(echo + 1);
    if (moved) {
      head = value.t;
      walking = auto() && walking;
      rest = null;
      rings = [];
    }
    if (value.zeros !== was.zeros || value.x !== was.x || value.view !== was.view) folding = null;
    if (value.tint !== was.tint) paint();
    if (moved || SPEC.some(({ key }) => key !== "t" && value[key] !== was[key])) tell(true);
  };
  const stop = () => {
    gone = true;
  };
  const svg = () => {
    const dpr = view.dpr;
    return picture(
      {
        plan,
        steps: fold(num, value.x),
        view: value.view,
        head,
        k: shown,
        zeros: value.zeros,
        kept: Math.round(value.trail * PER),
        look: value.look,
        w: view.w / dpr,
        h: view.h / dpr,
      },
      inks,
    );
  };
  const csv = () => table(plan.zeros);
  paint();
  return { draw, theme: paint, set, go, toggle, playing: busy, word, hop: jump, stop, svg, csv };
}
