import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Act, Line, Over, State } from "./engine.ts";
import {
  broken,
  DEEP,
  deal as dealing,
  decode,
  fits,
  flags,
  flapped,
  flip,
  fold,
  HIGH,
  LARGE,
  LOW,
  needed,
  punch,
  SMALL,
  sheet,
  stack,
  step,
  X,
  Y,
} from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { size: number; folds: number; play: string; tint: string };

export type Told = { moves: number; par: number; over: Over; holes: number; of: number };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Crease = { axis: number; line: number; from: number };

export type Hands = {
  done: () => boolean;
  again: () => void;
  press: () => void;
  bend: (key: string) => void;
  move: (key: string) => void;
  hover: (u: number, v: number) => void;
  tap: (u: number, v: number) => void;
};

export type Held = Scene & Partial<Hands>;

export type Fold = ReturnType<typeof make>;

export const SPEC: Spec = [
  {
    key: "size",
    label: "Sheet",
    kind: "slider",
    def: 8,
    min: SMALL,
    max: LARGE,
    step: 1,
    unit: "cells",
    group: "Board",
  },
  { key: "folds", label: "Folds", kind: "slider", def: 3, min: 1, max: DEEP, step: 1, group: "Board" },
  {
    key: "play",
    label: "Player",
    kind: "segment",
    def: "me",
    options: [
      ["me", "Me"],
      ["smart", "Smart"],
    ],
    group: "Play",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A square of paper and a lace to cut from it: the rings show where the holes go. Fold the paper, punch through every layer at once, and open it to see the lace.",
  "A punch goes through the whole stack under it, so it may only land where every layer wants a hole. Fold over a ring whose hole is still to come onto plain paper and that hole is lost for good: the sheet is stuck.",
  "Par is the fewest folds and punches that make the lace without ever unfolding, found by a search over every way to fold. Smart plays it in exactly par.",
  "Tap or click a crease between cells to fold the smaller side over it, and a ring to punch it. With the keys, the arrows move, W, A, S and D fold at that edge of the cell in the ring, and Enter punches. Reroll deals another lace.",
];

const ARROWS = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

const STEPS: Record<string, [number, number]> = {
  ArrowUp: [0, -1],
  ArrowDown: [0, 1],
  ArrowLeft: [-1, 0],
  ArrowRight: [1, 0],
};

const BENDS: Record<string, [number, number, number]> = {
  a: [X, 0, LOW],
  d: [X, 1, HIGH],
  w: [Y, 0, LOW],
  s: [Y, 1, HIGH],
};

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  gestures: true,
  name: (value: Value) => `fold-${value.size}-${value.folds}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : "Punch"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: ["a", "d", "w", "s"], label: "Fold", act: "bend" },
    { key: ARROWS, label: "Move", act: "move" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.press?.()),
    bend: (scene, e: Fired) => scene.bend?.(String(e.key ?? "").toLowerCase()),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "lace",
    title: "Lace",
    pick: (rand: Rng) => ({ play: "smart", size: 8, folds: 3 + Math.floor(rand() * 2) }),
  },
  {
    id: "doily",
    title: "Doily",
    pick: (rand: Rng) => ({ play: "smart", size: 9, folds: 5 + Math.floor(rand() * 2) }),
  },
];

export const FIT = 0.88;
const ZOOM = 1.7;
const HOLE = 0.3;
const PENCIL = 0.035;
const RIM = 0.03;
const SEAM = 0.025;
const CREASE = 0.022;
const EDGE = 0.2;
const DASH = 0.14;
const CORNER = 0.2;
const DROP = 0.45;
const SINK = 0.08;
const TILT = 0.3;
const SHADOW = 0.3;
const STACK = 0.05;
const DEEPEST = 0.22;
const SPOT = 0.62;
const LAMP = 0.1;
const BEAM = 0.3;
const BREATH = 2800;
const CHADS = 5;
const TINGE = 0.08;
const NIGHT = "#f3efe6";
const DAY = "#fffdf8";
const BLACK = "#000000";
const WHITE = "#ffffff";
const PACE = 380;
const AIM = 260;
const BEND = 560;
const POP = 420;
const OPEN = 600;
const WAIT = 500;
const STUCK = 1600;
const HOLD = 3200;
const BLOOM = 1400;
const TAIL = 100;
const SLOW = 820;
const SIGHT = 600;
const LONG = 1100;
const BURST = 700;
const UNFOLD = 1100;
const PAUSE = 1100;
const REST = 6400;
const FADE = 1200;
const LEAVE = 900;
const DEAL: (keyof Knobs)[] = ["size", "folds"];

const ease = (k: number) => (k >= 1 ? 1 : 1 - (1 - k) ** 3);

const swing = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : k < 0.5 ? 4 * k ** 3 : 1 - (-2 * k + 2) ** 3 / 2);

const spring = (k: number) => (k >= 1 ? 1 : 1 + 2.6 * (k - 1) ** 3 + 1.6 * (k - 1) ** 2);

const fade = (k: number) => ease(Math.max(0, k));

/* CREASES */

export type Seam = { axis: number; at: number; along: number; valley: boolean };

export function seams(state: Pick<State, "size" | "folds">): Seam[] {
  const n = state.size;
  const out: Seam[] = [];
  const turned = [new Uint8Array(n), new Uint8Array(n)];
  for (let i = 0; i < state.folds.length; i++) {
    const was = sheet(state, i);
    const now = sheet(state, i + 1);
    const { axis, line, from } = decode(state.folds[i]);
    const [before, after] = axis === X ? [was.xs, now.xs] : [was.ys, now.ys];
    for (let b = 1; b < n; b++) {
      if (before.at[b - 1] === before.at[b] || after.at[b - 1] !== after.at[b]) continue;
      for (let j = 0; j < n; j++)
        out.push({ axis, at: b, along: j, valley: ((turned[axis][b - 1] + turned[1 - axis][j]) & 1) === 0 });
    }
    for (let c = 0; c < n; c++) if (flapped(line, from, before.at[c])) turned[axis][c] ^= 1;
  }
  return out;
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let cursor: [number, number] = [0, 0];
  let aim: Crease | null = null;
  let bent: { code: number; at: number } | null = null;
  let popped: { x: number; y: number; at: number; layers: number } | null = null;
  let queued: Act | null = null;
  let queuedAt = -Infinity;
  let opened = Infinity;
  let acted = view.t;
  let last = view.t;
  let round = 0;
  let paused = false;
  let busy = -Infinity;
  let gone = false;
  let rise = -Infinity;
  let held = Infinity;
  let down = Infinity;
  const landing: (() => void)[] = [];
  const lock = Boolean(view.lock);
  const pace = lock ? SLOW : PACE;
  const sight = lock ? SIGHT : AIM;
  const bend = lock ? LONG : BEND;
  const pop = lock ? BURST : POP;
  const open = lock ? UNFOLD : OPEN;
  const wait = lock ? PAUSE : WAIT;
  const rest = lock ? REST : HOLD;
  const span = (ms: number) => (view.still ? 0 : ms);
  const tell = () => {
    if (gone) return;
    opts.onTell?.({
      moves: state.moves,
      par: state.par,
      over: state.over,
      holes: [...state.holes].filter((one) => one === "1").length,
      of: [...state.target].filter((one) => one === "1").length,
    });
  };
  const middle = () => {
    const { xs, ys } = sheet(state);
    cursor = [Math.floor((xs.lo + xs.hi - 1) / 2), Math.floor((ys.lo + ys.hi - 1) / 2)];
  };
  const deal = () => {
    state = dealing(value, { seed, round });
    bent = null;
    popped = null;
    aim = null;
    queued = null;
    opened = Infinity;
    acted = view.t;
    busy = view.t;
    middle();
    tell();
  };
  const layout = (count = state.folds.length, next = count, k = 0) => {
    const a = sheet(state, count);
    const b = sheet(state, next);
    const mid = (u: number, v: number) => u + (v - u) * k;
    const w = mid(a.xs.hi - a.xs.lo, b.xs.hi - b.xs.lo);
    const h = mid(a.ys.hi - a.ys.lo, b.ys.hi - b.ys.lo);
    const base = (Math.min(view.w, view.h) * FIT) / state.size;
    const cell = Math.max(base, Math.min(base * ZOOM, (view.w * FIT) / w, (view.h * FIT) / h));
    const cx = mid(a.xs.lo + a.xs.hi, b.xs.lo + b.xs.hi) / 2;
    const cy = mid(a.ys.lo + a.ys.hi, b.ys.lo + b.ys.hi) / 2;
    return { x: view.w / 2 - cx * cell, y: view.h / 2 - cy * cell, cell };
  };
  const flat = () => opened + state.folds.length * span(open);
  const shown = (): { count: number; turn: { code: number; k: number } | null; flat: number } => {
    const n = state.folds.length;
    if (view.t >= opened) {
      const far = Math.max(0, (view.t - opened) / span(open));
      const whole = Math.floor(far);
      if (!(far < n)) return { count: 0, turn: null, flat: 1 };
      const at = n - 1 - whole;
      return { count: at, turn: { code: state.folds[at], k: 1 - swing(far - whole) }, flat: 0 };
    }
    if (bent && view.t - bent.at < span(bend))
      return { count: n - 1, turn: { code: bent.code, k: swing((view.t - bent.at) / span(bend)) }, flat: 0 };
    return { count: n, turn: null, flat: 0 };
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - flat() - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const paint = () => {
    const show = shown();
    const { x: left, y: top, cell } = show.turn ? layout(show.count, show.count + 1, show.turn.k) : layout(show.count);
    const { accent, paper } = view.look();
    const dark = !bright(paper);
    const tone = blend(dark ? NIGHT : DAY, accent, TINGE);
    const deep = blend(accent, BLACK, 0.35);
    const glow = dark ? blend(accent, WHITE, 0.35) : accent;
    const thin = (k: number) => Math.max(1, cell * k);
    const n = state.size;
    const now = sheet(state, show.count);
    const layers = new Int32Array(n * n);
    const closed = new Uint8Array(n * n);
    for (let y = 0; y < n; y++)
      for (let x = 0; x < n; x++) {
        const p = now.ys.at[y] * n + now.xs.at[x];
        layers[p]++;
        if (state.holes[y * n + x] !== "1") closed[p] = 1;
      }
    const turn = show.turn ? decode(show.turn.code) : null;
    const lifted = (p: number) =>
      turn !== null && flapped(turn.line, turn.from, turn.axis === X ? p % n : Math.floor(p / n));
    const marks = flags(state, now);
    const opening = show.flat ? fade((view.t - flat()) / BLOOM) : 0;
    const live = state.over === "done" ? opening : 0;
    const breath = 0.5 - 0.5 * Math.cos((2 * Math.PI * Math.max(0, view.t - flat())) / BREATH);
    const pressed = popped && !view.still ? Math.min(1, (view.t - popped.at) / pop) : 1;
    const radius = (p: number) =>
      cell * HOLE * (popped && p === popped.y * n + popped.x ? Math.max(0, spring(pressed)) : 1);
    const corner = (p: number) => [left + (p % n) * cell, top + Math.floor(p / n) * cell];
    const holes = (list: number[]) => {
      for (const p of list) {
        if (closed[p]) continue;
        const [x, y] = corner(p);
        const r = radius(p);
        ctx.moveTo(x + cell / 2 + r, y + cell / 2);
        ctx.arc(x + cell / 2, y + cell / 2, r, 0, 2 * Math.PI);
      }
    };
    const cells = (list: number[]) => {
      for (const p of list) {
        const [x, y] = corner(p);
        ctx.rect(x, y, cell, cell);
      }
      holes(list);
    };
    const group = (keep: (p: number) => boolean, lift: number, dim: number) => {
      const kept: number[] = [];
      for (let p = 0; p < n * n; p++) if (layers[p] && keep(p)) kept.push(p);
      if (!kept.length) return;
      ctx.save();
      ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.18})`;
      ctx.shadowBlur = cell * DROP * (1 + 2 * lift);
      ctx.shadowOffsetY = cell * SINK * (1 + 3 * lift);
      ctx.beginPath();
      cells(kept);
      ctx.fillStyle = tone;
      ctx.fill("evenodd");
      ctx.restore();
      for (const count of [...new Set(kept.map((p) => layers[p]))].filter((one) => one > 1)) {
        ctx.beginPath();
        cells(kept.filter((p) => layers[p] === count));
        ctx.fillStyle = blend(tone, deep, Math.min(DEEPEST, STACK * (count - 1)));
        ctx.fill("evenodd");
      }
      ctx.beginPath();
      holes(kept);
      ctx.save();
      if (live > 0) {
        ctx.shadowColor = accent;
        ctx.shadowBlur = cell * (0.3 + 0.5 * breath) * live;
      }
      ctx.fillStyle = glow;
      ctx.fill();
      ctx.restore();
      ctx.strokeStyle = veil(deep, 0.35);
      ctx.lineWidth = thin(RIM);
      ctx.stroke();
      ctx.beginPath();
      for (const p of kept) {
        const [x, y] = corner(p);
        const right = p % n < n - 1 ? p + 1 : -1;
        const below = p + n < n * n ? p + n : -1;
        if (right >= 0 && layers[right] && keep(right) && layers[right] !== layers[p]) {
          ctx.moveTo(x + cell, y);
          ctx.lineTo(x + cell, y + cell);
        }
        if (below >= 0 && layers[below] && keep(below) && layers[below] !== layers[p]) {
          ctx.moveTo(x, y + cell);
          ctx.lineTo(x + cell, y + cell);
        }
      }
      ctx.strokeStyle = veil(deep, 0.3);
      ctx.lineWidth = thin(SEAM);
      ctx.stroke();
      ctx.save();
      ctx.setLineDash([cell * 0.07, cell * 0.05]);
      ctx.beginPath();
      for (const p of kept) {
        if (!needed(marks[p])) continue;
        const [x, y] = corner(p);
        ctx.moveTo(x + cell / 2 + cell * HOLE, y + cell / 2);
        ctx.arc(x + cell / 2, y + cell / 2, cell * HOLE, 0, 2 * Math.PI);
      }
      ctx.strokeStyle = veil(deep, 0.55);
      ctx.lineWidth = thin(PENCIL);
      ctx.stroke();
      ctx.restore();
      if (dim > 0.005) {
        ctx.beginPath();
        cells(kept);
        ctx.fillStyle = veil(BLACK, dim);
        ctx.fill("evenodd");
      }
    };
    const stroke = (path: () => void, color: string, width: number, dash: number[] = []) => {
      ctx.save();
      ctx.setLineDash(dash);
      ctx.beginPath();
      path();
      ctx.strokeStyle = color;
      ctx.lineWidth = width;
      ctx.stroke();
      ctx.restore();
    };
    const line = (axis: number, k: number, from: number, to: number) => {
      if (axis === X) {
        ctx.moveTo(left + k * cell, top + from * cell);
        ctx.lineTo(left + k * cell, top + to * cell);
      } else {
        ctx.moveTo(left + from * cell, top + k * cell);
        ctx.lineTo(left + to * cell, top + k * cell);
      }
    };
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    const reach = Math.min(view.w, view.h) * SPOT;
    const lamp = ctx.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, reach);
    lamp.addColorStop(0, veil(accent, (dark ? LAMP : LAMP / 2) + BEAM * live * (0.75 + 0.25 * breath)));
    lamp.addColorStop(1, veil(accent, 0));
    ctx.fillStyle = lamp;
    ctx.fillRect(0, 0, view.w, view.h);
    group((p) => !lifted(p), 0, 0);
    if (!show.turn && !state.over)
      stroke(
        () => {
          for (const axis of [X, Y]) {
            const own = axis === X ? now.xs : now.ys;
            const across = axis === X ? now.ys : now.xs;
            for (let k = own.lo + 1; k < own.hi; k++)
              if (fits(own, k, LOW) || fits(own, k, HIGH)) line(axis, k, across.lo, across.hi);
          }
        },
        veil(deep, 0.16),
        thin(CREASE),
        [cell * DASH, cell * DASH * 0.7],
      );
    if (show.flat && opening > 0) {
      const all = seams(state);
      for (const valley of [true, false])
        stroke(
          () => {
            for (const one of all) if (one.valley === valley) line(one.axis, one.at, one.along, one.along + 1);
          },
          veil(deep, 0.34 * opening),
          thin(CREASE),
          valley ? [cell * 0.12, cell * 0.08] : [cell * 0.16, cell * 0.06, cell * 0.03, cell * 0.06],
        );
    }
    if (turn && show.turn) {
      const lift = Math.sin(Math.PI * show.turn.k);
      const c = Math.cos(Math.PI * show.turn.k);
      const own = turn.axis === X ? now.xs : now.ys;
      const across = turn.axis === X ? now.ys : now.xs;
      const flap = turn.from === LOW ? turn.line - own.lo : own.hi - turn.line;
      const at = (turn.axis === X ? left : top) + turn.line * cell;
      const way = turn.from === LOW ? 1 : -1;
      const reach = flap * cell * (0.25 + 0.6 * lift);
      const lo = (turn.axis === X ? top : left) + across.lo * cell;
      const hi = (turn.axis === X ? top : left) + across.hi * cell;
      const fall =
        turn.axis === X
          ? ctx.createLinearGradient(at, 0, at + way * reach, 0)
          : ctx.createLinearGradient(0, at, 0, at + way * reach);
      fall.addColorStop(0, veil(BLACK, SHADOW * lift));
      fall.addColorStop(1, veil(BLACK, 0));
      ctx.fillStyle = fall;
      if (turn.axis === X) ctx.fillRect(Math.min(at, at + way * reach), lo, reach, hi - lo);
      else ctx.fillRect(lo, Math.min(at, at + way * reach), hi - lo, reach);
      ctx.save();
      if (turn.axis === X) {
        ctx.translate(at, 0);
        ctx.scale(c, 1);
        ctx.translate(-at, 0);
      } else {
        ctx.translate(0, at);
        ctx.scale(1, c);
        ctx.translate(0, -at);
      }
      group(lifted, lift, TILT * (1 - Math.abs(c)));
      ctx.restore();
    }
    if (popped && pressed < 1) {
      const x = left + (popped.x + 0.5) * cell;
      const y = top + (popped.y + 0.5) * cell;
      const e = ease(pressed);
      stroke(
        () => ctx.arc(x, y, cell * HOLE * (1 + 1.4 * e), 0, 2 * Math.PI),
        veil(accent, 1 - e),
        thin(0.07 * (1 - e)),
      );
      const many = Math.min(CHADS, popped.layers);
      for (let i = 0; i < many; i++) {
        const angle = -Math.PI / 2 + (i - (many - 1) / 2) * 0.75;
        const far = cell * (0.35 + 0.9 * e);
        const cx = x + Math.cos(angle) * far;
        const cy = y + Math.sin(angle) * far + cell * 1.4 * e * e;
        ctx.beginPath();
        ctx.arc(cx, cy, cell * HOLE * 0.8 * (1 - 0.4 * e), 0, 2 * Math.PI);
        ctx.fillStyle = veil(tone, 1 - e);
        ctx.fill();
        ctx.strokeStyle = veil(deep, 0.3 * (1 - e));
        ctx.lineWidth = thin(RIM);
        ctx.stroke();
      }
    }
    if (show.turn) return;
    if (state.over === "stuck" && !show.flat)
      stroke(
        () => {
          for (let p = 0; p < n * n; p++) {
            if (!broken(marks[p])) continue;
            const [x, y] = corner(p);
            ctx.moveTo(x + cell * 0.3, y + cell * 0.3);
            ctx.lineTo(x + cell * 0.7, y + cell * 0.7);
            ctx.moveTo(x + cell * 0.7, y + cell * 0.3);
            ctx.lineTo(x + cell * 0.3, y + cell * 0.7);
          }
        },
        deep,
        thin(0.06),
      );
    if (state.over) return;
    if (aim) {
      const own = aim.axis === X ? now.xs : now.ys;
      const across = aim.axis === X ? now.ys : now.xs;
      const from = aim.from === LOW ? own.lo : aim.line;
      const to = aim.from === LOW ? aim.line : own.hi;
      ctx.fillStyle = veil(accent, 0.14);
      if (aim.axis === X)
        ctx.fillRect(left + from * cell, top + across.lo * cell, (to - from) * cell, (across.hi - across.lo) * cell);
      else ctx.fillRect(left + across.lo * cell, top + from * cell, (across.hi - across.lo) * cell, (to - from) * cell);
      stroke(() => line(aim!.axis, aim!.line, across.lo, across.hi), accent, thin(0.05), [
        cell * DASH,
        cell * DASH * 0.6,
      ]);
      return;
    }
    if (value.play !== "me") return;
    const pad = cell * 0.06;
    stroke(
      () =>
        ctx.roundRect(
          left + cursor[0] * cell + pad,
          top + cursor[1] * cell + pad,
          cell - 2 * pad,
          cell - 2 * pad,
          cell * CORNER,
        ),
      accent,
      thin(0.045),
    );
  };
  const finish = () => {
    opened = view.t + span(state.over === "stuck" ? STUCK : wait);
    busy = flat() + (value.play === "me" ? BLOOM : rest);
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return false;
    acted = view.t;
    aim = null;
    queued = null;
    if (act.verb === "fold") {
      const { axis, line, from } = act.args;
      bent = view.still ? null : { code: state.folds[state.folds.length - 1], at: view.t };
      const at = axis === X ? 0 : 1;
      if (flapped(line, from, cursor[at])) cursor[at] = flip(line, cursor[at]);
      busy = view.t + span(bend);
    } else {
      popped = { ...act.args, at: view.t, layers: stack(was, sheet(was), act.args.x, act.args.y).length };
      cursor = [act.args.x, act.args.y];
      busy = view.t + span(pop);
    }
    if (state.over) finish();
    tell();
    paint();
    return true;
  };
  const advance = () => {
    if (paused || held < Infinity) return;
    if (state.over) {
      if (value.play === "me") return;
      if (view.t - flat() < rest) return;
      round++;
      deal();
      if (lock) {
        rise = view.t;
        busy = view.t + FADE;
      }
      return;
    }
    if (value.play === "me") return;
    if (queued) {
      if (view.t - queuedAt >= span(sight)) apply(queued);
      return;
    }
    if (view.t - acted < pace) return;
    const act = pick(state, value.play, view.rand);
    if (!act) return;
    if (act.verb !== "fold" || view.still) {
      apply(act);
      return;
    }
    queued = act;
    queuedAt = view.t;
    aim = { ...act.args };
    busy = view.t + span(sight);
    paint();
  };
  const draw = () => {
    if (view.t !== last) advance();
    last = view.t;
    if (view.t <= busy + TAIL) paint();
    if (view.t >= down + LEAVE) land();
  };
  const land = () => {
    for (const done of landing.splice(0)) done();
  };
  const hold = () => {
    if (!gone && !view.still && held === Infinity) held = view.t;
  };
  const windDown = (onDone?: () => void) => {
    if (onDone) landing.push(onDone);
    if (gone || view.still) return land();
    hold();
    if (down === Infinity) down = view.t;
    busy = Math.max(busy, down + LEAVE);
  };
  const lines = (): [Line, Line] => {
    const { xs, ys } = sheet(state);
    return [xs, ys];
  };
  const side = (line: Line, k: number, near: number): number | null => {
    const first = near < k ? LOW : HIGH;
    if (fits(line, k, first)) return first;
    return fits(line, k, -first) ? -first : null;
  };
  const read = (u: number, v: number): { crease: Crease | null; spot: [number, number] | null } => {
    const { x: left, y: top, cell } = layout();
    const fx = (u * view.w - left) / cell;
    const fy = (v * view.h - top) / cell;
    const [xs, ys] = lines();
    const inside = fx >= xs.lo && fx < xs.hi && fy >= ys.lo && fy < ys.hi;
    if (!inside) return { crease: null, spot: null };
    const kx = Math.round(fx);
    const ky = Math.round(fy);
    const dx = Math.abs(fx - kx);
    const dy = Math.abs(fy - ky);
    const sx = dx < EDGE ? side(xs, kx, fx) : null;
    const sy = dy < EDGE ? side(ys, ky, fy) : null;
    const crease =
      sx !== null && (sy === null || dx <= dy)
        ? { axis: X, line: kx, from: sx }
        : sy !== null
          ? { axis: Y, line: ky, from: sy }
          : null;
    return { crease, spot: [Math.floor(fx), Math.floor(fy)] };
  };
  const same = (a: Crease | null, b: Crease | null) =>
    a === b || (!!a && !!b && a.axis === b.axis && a.line === b.line && a.from === b.from);
  const hover = (u: number, v: number) => {
    if (state.over) return;
    const { crease, spot } = read(u, v);
    const moved = spot !== null && (spot[0] !== cursor[0] || spot[1] !== cursor[1]);
    if (same(crease, aim) && !moved) return;
    aim = crease;
    if (spot) cursor = spot;
    paint();
  };
  const tap = (u: number, v: number) => {
    if (state.over) return;
    const { crease, spot } = read(u, v);
    if (crease) apply(fold(crease.axis, crease.line, crease.from));
    else if (spot) apply(punch(spot[0], spot[1]));
  };
  const press = () => {
    if (!state.over) apply(punch(cursor[0], cursor[1]));
  };
  const bendAt = (key: string) => {
    const way = BENDS[key];
    if (!way || state.over) return;
    const [axis, shift, near] = way;
    const k = cursor[axis] + shift;
    const from = side(lines()[axis], k, near === LOW ? k - 1 : k);
    if (from !== null) apply(fold(axis, k, from));
  };
  const move = (key: string) => {
    const way = STEPS[key];
    if (!way || state.over) return;
    const [xs, ys] = lines();
    const x = Math.min(xs.hi - 1, Math.max(xs.lo, cursor[0] + way[0]));
    const y = Math.min(ys.hi - 1, Math.max(ys.lo, cursor[1] + way[1]));
    if (x === cursor[0] && y === cursor[1]) return;
    cursor = [x, y];
    aim = null;
    paint();
  };
  const set = (next: Given) => {
    const was = value;
    value = tidy(SPEC, next) as Knobs;
    const fresh = next.seed !== seed || DEAL.some((key) => was[key] !== value[key]);
    seed = next.seed;
    if (fresh) {
      round = 0;
      deal();
    }
    paint();
  };
  const stop = () => {
    gone = true;
    land();
  };
  deal();
  paint();
  return {
    draw,
    size: paint,
    theme: paint,
    set,
    stop,
    toggle: () => {
      paused = !paused;
    },
    playing: () => !paused,
    hold,
    windDown,
    done: () => state.over !== null,
    press,
    bend: bendAt,
    move,
    hover,
    tap,
    ring: () => cursor,
    state: () => state,
  };
}
