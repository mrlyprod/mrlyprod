import { blend, bright, hex, rgb, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Act, Over, State } from "./engine.ts";
import {
  around,
  BLOOMS,
  deal as dealing,
  depths,
  drop,
  EMPTY,
  FLOWER,
  front,
  frosty,
  future,
  LARGE,
  LOST,
  lost,
  MOST,
  parent,
  reachable,
  SEED,
  SMALL,
  STONE,
  saved,
  step,
  WAIT,
} from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { size: number; frost: number; flowers: number; play: string; tint: string };

export type Told = { turn: number; stones: number; alive: number; lost: number; of: number; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = {
  done: () => boolean;
  again: () => void;
  wait: () => void;
  press: () => void;
  move: (key: string) => void;
  hover: (u: number, v: number) => void;
  tap: (u: number, v: number) => void;
};

export type Held = Scene & Partial<Hands>;

export type Frost = ReturnType<typeof make>;

export const SPEC: Spec = [
  {
    key: "size",
    label: "Field",
    kind: "slider",
    def: 15,
    min: SMALL,
    max: LARGE,
    step: 1,
    unit: "cells",
    group: "Board",
  },
  { key: "frost", label: "Frost", kind: "slider", def: 2, min: 1, max: MOST, step: 1, group: "Board" },
  { key: "flowers", label: "Flowers", kind: "slider", def: 6, min: 1, max: BLOOMS, step: 1, group: "Board" },
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
  "Frost creeps over a field of flowers. Each turn it spreads into every square that touches exactly one frosted square, never one that touches two or more, so it grows as a branching crystal.",
  "Faint needles show where the frost goes next. Before each turn you may drop one stone at the tip of a needle, and frost never enters a stone. A flower the frost reaches is lost; a pulsing ring marks one with a single turn left to save.",
  "A stone can also do harm: a square that two branches would have reached at once stays clear forever, so blocking one of them can let the other through. The round ends once no flower can be reached without your help, and the frost then grows out to the end.",
  "Tap a needle to drop a stone at its tip, or Wait to let the frost grow. With the keys, the arrows move, X drops a stone in the ring, and Enter waits. Smart looks ahead one stone at a time. Reroll deals another field.",
];

const ARROWS = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

const STEPS: Record<string, [number, number]> = {
  ArrowUp: [0, -1],
  ArrowDown: [0, 1],
  ArrowLeft: [-1, 0],
  ArrowRight: [1, 0],
};

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  gestures: true,
  name: (value: Value) => `frost-${value.size}-${value.frost}-${value.flowers}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : "Wait"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: "x", label: "Stone", act: "stone" },
    { key: ARROWS, label: "Move", act: "move" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.wait?.()),
    stone: (scene) => scene.press?.(),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "garden",
    title: "Garden",
    pick: (rand: Rng) => ({ play: "smart", size: 15, frost: 2, flowers: 5 + Math.floor(rand() * 2) }),
  },
  {
    id: "winter",
    title: "Winter",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 25,
      frost: 3 + Math.floor(rand() * 2),
      flowers: 10 + Math.floor(rand() * 3),
    }),
  },
];

export const FIT = 0.86;
const BED = 0.45;
const DOT = 0.035;
const PEBBLE = 0.36;
const PETAL = 0.36;
const HEART = 0.13;
const ROOT = 0.2;
const TWIG = 0.055;
const THICK = 0.035;
const STAR = 0.3;
const BARB = 0.2;
const FAN = 0.75;
const SHEEN = 9;
const WARN = 2;
const NEEDLE = 0.8;
const TIP = 0.11;
const SHOW = 300;
const LATE = 1e9;
const CORNER = 0.22;
const DROP = 0.35;
const SINK = 0.06;
const TINGE = 0.06;
const SPOT = 0.66;
const LAMP = 0.1;
const BEAM = 0.26;
const TURN = 150;
const NIGHT = "#eef3f8";
const DAY = "#fbfdff";
const BLACK = "#000000";
const WHITE = "#ffffff";
const PACE = 520;
const SLOW = 900;
const AIM = 300;
const SIGHT = 700;
const GROW = 360;
const CREEP = 620;
const RUSH = 90;
const SWEEP = 160;
const PLOP = 260;
const BLOOM = 1400;
const BREATH = 2800;
const SHIVER = 900;
const HOLD = 3600;
const REST = 6400;
const FADE = 1200;
const LEAVE = 900;
const TAIL = 100;
const DEAL: (keyof Knobs)[] = ["size", "frost", "flowers"];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const spring = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 + 2.6 * (k - 1) ** 3 + 1.6 * (k - 1) ** 2);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (p: number, salt: number) => {
  let s = Math.imul(p + 1, 0x9e3779b1) ^ Math.imul(salt + 7, 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 15), 0x2c1b3c6d);
  return ((s ^ (s >>> 13)) >>> 0) / 4294967296;
};

/* COLOURS */

export function turned(color: string, degrees: number, light: number): string {
  const [r, g, b] = rgb(color).map((v) => v / 255);
  const hi = Math.max(r, g, b);
  const lo = Math.min(r, g, b);
  const d = hi - lo;
  let h = 0;
  if (d > 0) h = hi === r ? ((g - b) / d) % 6 : hi === g ? (b - r) / d + 2 : (r - g) / d + 4;
  const hue = (((h * 60 + degrees) % 360) + 360) % 360;
  const l = (hi + lo) / 2;
  const s = Math.min(0.82, Math.max(0.62, d === 0 ? 0 : d / (1 - Math.abs(2 * l - 1))));
  const c = (1 - Math.abs(2 * light - 1)) * s;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = light - c / 2;
  const [a, e, f] =
    hue < 60
      ? [c, x, 0]
      : hue < 120
        ? [x, c, 0]
        : hue < 180
          ? [0, c, x]
          : hue < 240
            ? [0, x, c]
            : hue < 300
              ? [x, 0, c]
              : [c, 0, x];
  return hex([(a + m) * 255, (e + m) * 255, (f + m) * 255]);
}

/* TREE */

export function branches(state: Pick<State, "size" | "board">): Int32Array {
  const { size, board } = state;
  const out = new Int32Array(board.length);
  for (let p = 0; p < board.length; p++) {
    if (!frosty(board[p])) continue;
    for (let q = p; q >= 0; q = parent(size, q, board[q])) out[q]++;
  }
  return out;
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let deep: Int32Array = new Int32Array(0);
  let weight: Int32Array = new Int32Array(0);
  let needles: number[] = [];
  let soon: number[] = [];
  let top = 0;
  let cursor: [number, number] = [0, 0];
  let hovered = false;
  let grew = { top: 0, at: view.t, gap: GROW };
  let plopped: { p: number; at: number } | null = null;
  let queued: Act | null = null;
  let queuedAt = -Infinity;
  let ended = Infinity;
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
  const grow = lock ? CREEP : GROW;
  const rest = lock ? REST : HOLD;
  const span = (ms: number) => (view.still ? 0 : ms);
  const tell = () => {
    if (gone) return;
    opts.onTell?.({
      turn: state.turn,
      stones: state.stones,
      alive: saved(state),
      lost: lost(state),
      of: state.flowers,
      over: state.over,
    });
  };
  const learn = () => {
    deep = depths(state);
    weight = branches(state);
    needles = state.over ? [] : front(state);
    const when = future(state.size, state.board);
    soon = state.over ? [] : [...state.board].flatMap((one, p) => (one === FLOWER && when[p] === WARN ? [p] : []));
  };
  const highest = () => deep.reduce((most, one) => Math.max(most, one), 0);
  const deal = () => {
    state = dealing(value, { seed, round });
    learn();
    top = highest();
    grew = { top, at: view.t - grow - SHOW, gap: grow };
    plopped = null;
    queued = null;
    ended = state.over ? view.t : Infinity;
    acted = view.t;
    busy = view.t;
    cursor = [Math.floor(state.size / 2), Math.floor(state.size / 2)];
    tell();
  };
  const layout = () => {
    const cell = (Math.min(view.w, view.h) * FIT) / state.size;
    const side = cell * state.size;
    return { left: (view.w - side) / 2, top: (view.h - side) / 2, cell };
  };
  const reached = (d: number) => {
    if (d <= grew.top || view.still) return 1;
    return ease((view.t - grew.at - (d - grew.top - 1) * grew.gap) / grow);
  };
  const bloomed = () => grew.at + Math.max(0, top - grew.top - 1) * grew.gap + span(grow);
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const threatened = () => soon.length > 0 || needles.some((p) => state.board[p] === FLOWER);
  const paint = () => {
    const { left, top: up, cell } = layout();
    const { accent, paper, ink } = view.look();
    const dark = !bright(paper);
    const n = state.size;
    const board = state.board;
    const snow = blend(dark ? NIGHT : DAY, accent, TINGE);
    const field = dark ? blend(paper, accent, 0.1) : snow;
    const ice = dark ? blend(accent, WHITE, 0.5) : blend(accent, BLACK, 0.12);
    const petal = turned(accent, TURN, dark ? 0.66 : 0.6);
    const heart = blend(petal, WHITE, dark ? 0.55 : 0.7);
    const rock = dark ? blend(paper, WHITE, 0.32) : blend(snow, ink, 0.5);
    const thin = (k: number) => Math.max(1, cell * k);
    const center = (p: number): [number, number] => [
      left + ((p % n) + 0.5) * cell,
      up + (Math.floor(p / n) + 0.5) * cell,
    ];
    const since = Math.min(LATE, Math.max(0, view.t - bloomed()));
    const done = state.over === "done" ? fade(since / BLOOM) : 0;
    const breath = 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const reach = Math.min(view.w, view.h) * SPOT;
    const lamp = ctx.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, reach);
    lamp.addColorStop(0, veil(accent, (dark ? LAMP : LAMP / 2) + BEAM * done * (0.75 + 0.25 * breath)));
    lamp.addColorStop(1, veil(accent, 0));
    ctx.fillStyle = lamp;
    ctx.fillRect(0, 0, view.w, view.h);
    const pad = cell * BED;
    ctx.save();
    ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.45 : 0.14})`;
    ctx.shadowBlur = cell * 1.2;
    ctx.shadowOffsetY = cell * 0.2;
    ctx.beginPath();
    ctx.roundRect(left - pad, up - pad, n * cell + 2 * pad, n * cell + 2 * pad, cell * 0.7);
    ctx.fillStyle = field;
    ctx.fill();
    ctx.restore();
    ctx.beginPath();
    for (let p = 0; p < n * n; p++) {
      if (board[p] !== EMPTY) continue;
      const [x, y] = center(p);
      ctx.moveTo(x + cell * DOT, y);
      ctx.arc(x, y, cell * DOT, 0, 2 * Math.PI);
    }
    ctx.fillStyle = veil(ice, dark ? 0.22 : 0.2);
    ctx.fill();
    const flower = (p: number, frozen: number, glow: number) => {
      const [x, y] = center(p);
      const r = cell * PETAL;
      const shiver =
        !state.over && needles.includes(p) && !view.still ? Math.sin((2 * Math.PI * view.t) / SHIVER) * 0.08 : 0;
      const spin = hash(p, 1) * Math.PI + shiver;
      ctx.save();
      ctx.translate(x, y);
      ctx.rotate(spin);
      if (glow > 0) {
        ctx.shadowColor = petal;
        ctx.shadowBlur = cell * (0.4 + 0.5 * breath) * glow;
      }
      ctx.beginPath();
      for (let i = 0; i < 5; i++) {
        const a = (i * 2 * Math.PI) / 5;
        ctx.moveTo(Math.cos(a) * r * 0.92, Math.sin(a) * r * 0.92);
        ctx.ellipse(Math.cos(a) * r * 0.5, Math.sin(a) * r * 0.5, r * 0.46, r * 0.3, a, 0, 2 * Math.PI);
      }
      ctx.fillStyle = frozen < 1 ? blend(petal, snow, frozen * 0.8) : veil(ice, dark ? 0.16 : 0.12);
      ctx.fill();
      ctx.shadowBlur = 0;
      if (frozen > 0) {
        ctx.strokeStyle = veil(ice, 0.6 * frozen);
        ctx.lineWidth = thin(0.025);
        ctx.stroke();
      }
      ctx.beginPath();
      ctx.arc(0, 0, cell * HEART, 0, 2 * Math.PI);
      ctx.fillStyle = frozen < 1 ? blend(heart, snow, frozen * 0.8) : veil(ice, 0.35);
      ctx.fill();
      ctx.restore();
    };
    for (let p = 0; p < n * n; p++) {
      if (board[p] !== STONE) continue;
      const [x, y] = center(p);
      const k = plopped && plopped.p === p && !view.still ? spring((view.t - plopped.at) / span(PLOP)) : 1;
      const rx = cell * PEBBLE * (0.9 + 0.2 * hash(p, 2)) * k;
      const ry = cell * PEBBLE * (0.78 + 0.14 * hash(p, 3)) * k;
      const tilt = hash(p, 4) * Math.PI;
      ctx.save();
      ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.55 : 0.25})`;
      ctx.shadowBlur = cell * DROP;
      ctx.shadowOffsetY = cell * SINK;
      const shine = ctx.createRadialGradient(x - rx * 0.35, y - ry * 0.45, 0, x, y, Math.max(rx, ry) * 1.1);
      shine.addColorStop(0, blend(rock, WHITE, dark ? 0.25 : 0.35));
      shine.addColorStop(1, rock);
      ctx.beginPath();
      ctx.ellipse(x, y, Math.max(0.1, rx), Math.max(0.1, ry), tilt, 0, 2 * Math.PI);
      ctx.fillStyle = shine;
      ctx.fill();
      ctx.restore();
    }
    for (let p = 0; p < n * n; p++) if (LOST.includes(board[p])) flower(p, reached(deep[p]), 0);
    const bins = new Map<number, number[]>();
    for (let p = 0; p < n * n; p++) {
      if (!frosty(board[p]) || board[p] === SEED || reached(deep[p]) <= 0) continue;
      const w = Math.min(ROOT, TWIG + THICK * Math.log2(weight[p]));
      const bin = Math.round(w * 100);
      if (!bins.has(bin)) bins.set(bin, []);
      bins.get(bin)!.push(p);
    }
    ctx.save();
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    if (dark) {
      ctx.shadowColor = accent;
      ctx.shadowBlur = cell * (0.25 + 0.35 * done * (0.6 + 0.4 * breath));
    }
    ctx.strokeStyle = ice;
    for (const [bin, list] of [...bins].sort((a, b) => b[0] - a[0])) {
      ctx.beginPath();
      for (const p of list) {
        const [x, y] = center(p);
        const [px, py] = center(parent(n, p, board[p]));
        const k = reached(deep[p]);
        ctx.moveTo(px, py);
        ctx.lineTo(px + (x - px) * k, py + (y - py) * k);
      }
      ctx.lineWidth = thin(bin / 100);
      ctx.stroke();
    }
    ctx.shadowBlur = 0;
    ctx.beginPath();
    for (let p = 0; p < n * n; p++) {
      if (!frosty(board[p]) || board[p] === SEED) continue;
      const k = reached(deep[p]);
      if (k <= 0) continue;
      const [x, y] = center(p);
      const [px, py] = center(parent(n, p, board[p]));
      const ux = (x - px) / cell;
      const uy = (y - py) / cell;
      const reach = cell * BARB * k;
      for (const at of [0.35, 0.75]) {
        if (at > k) continue;
        const bx = px + (x - px) * at;
        const by = py + (y - py) * at;
        for (const side of [-1, 1]) {
          ctx.moveTo(bx, by);
          ctx.lineTo(
            bx + (ux * Math.cos(FAN) - side * uy * Math.sin(FAN)) * reach,
            by + (uy * Math.cos(FAN) + side * ux * Math.sin(FAN)) * reach,
          );
        }
      }
    }
    ctx.strokeStyle = veil(ice, 0.55);
    ctx.lineWidth = thin(0.03);
    ctx.stroke();
    for (const [bin, list] of bins) {
      if (bin < SHEEN) continue;
      ctx.beginPath();
      for (const p of list) {
        const [x, y] = center(p);
        const [px, py] = center(parent(n, p, board[p]));
        const k = reached(deep[p]);
        ctx.moveTo(px, py);
        ctx.lineTo(px + (x - px) * k, py + (y - py) * k);
      }
      ctx.strokeStyle = veil(WHITE, dark ? 0.5 : 0.45);
      ctx.lineWidth = thin(bin / 300);
      ctx.stroke();
    }
    ctx.restore();
    ctx.save();
    if (dark) {
      ctx.shadowColor = accent;
      ctx.shadowBlur = cell * 0.4;
    }
    ctx.strokeStyle = ice;
    ctx.lineCap = "round";
    ctx.lineWidth = thin(0.06);
    ctx.beginPath();
    for (let p = 0; p < n * n; p++) {
      if (board[p] !== SEED) continue;
      const [x, y] = center(p);
      for (let i = 0; i < 6; i++) {
        const a = (i * Math.PI) / 3 + Math.PI / 6;
        const r = cell * STAR;
        ctx.moveTo(x, y);
        ctx.lineTo(x + Math.cos(a) * r, y + Math.sin(a) * r);
        const bx = x + Math.cos(a) * r * 0.55;
        const by = y + Math.sin(a) * r * 0.55;
        for (const side of [-1, 1]) {
          ctx.moveTo(bx, by);
          ctx.lineTo(bx + Math.cos(a + side * 0.8) * r * 0.32, by + Math.sin(a + side * 0.8) * r * 0.32);
        }
      }
    }
    ctx.stroke();
    ctx.restore();
    const shown = state.over ? 0 : view.still ? 1 : fade((view.t - bloomed()) / SHOW);
    if (shown > 0) {
      ctx.save();
      ctx.lineCap = "round";
      ctx.setLineDash([thin(0.05), thin(0.07)]);
      ctx.beginPath();
      for (const p of needles) {
        const from = around(n, p).find((q) => frosty(board[q]))!;
        const [x, y] = center(p);
        const [px, py] = center(from);
        ctx.moveTo(px, py);
        ctx.lineTo(px + (x - px) * NEEDLE, py + (y - py) * NEEDLE);
      }
      ctx.strokeStyle = veil(ice, 0.5 * shown);
      ctx.lineWidth = thin(0.04);
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.beginPath();
      for (const p of needles) {
        if (board[p] !== EMPTY) continue;
        const [x, y] = center(p);
        ctx.moveTo(x + cell * TIP, y);
        ctx.arc(x, y, cell * TIP, 0, 2 * Math.PI);
      }
      ctx.strokeStyle = veil(ice, 0.6 * shown);
      ctx.stroke();
      ctx.restore();
    }
    for (let p = 0; p < n * n; p++) if (board[p] === FLOWER) flower(p, 0, done);
    for (const p of soon) {
      const [x, y] = center(p);
      const beat = view.still ? 1 : 0.5 - 0.5 * Math.cos((2 * Math.PI * view.t) / SHIVER);
      ctx.beginPath();
      ctx.arc(x, y, cell * (0.44 + 0.06 * beat), 0, 2 * Math.PI);
      ctx.strokeStyle = veil(petal, 0.35 + 0.35 * beat);
      ctx.lineWidth = thin(0.04);
      ctx.stroke();
    }
    if (state.over) return;
    const aim = queued?.verb === "drop" ? queued.args : null;
    if (aim) {
      const [x, y] = center(aim.y * n + aim.x);
      ctx.beginPath();
      ctx.arc(x, y, cell * PEBBLE, 0, 2 * Math.PI);
      ctx.fillStyle = veil(rock, 0.35);
      ctx.fill();
      ctx.strokeStyle = accent;
      ctx.lineWidth = thin(0.045);
      ctx.stroke();
      return;
    }
    if (value.play !== "me") return;
    const p = cursor[1] * n + cursor[0];
    const [x, y] = center(p);
    if (hovered && reachable(n, board, p)) {
      ctx.beginPath();
      ctx.arc(x, y, cell * PEBBLE * 0.9, 0, 2 * Math.PI);
      ctx.fillStyle = veil(rock, 0.3);
      ctx.fill();
    }
    const inset = cell * 0.06;
    ctx.beginPath();
    ctx.roundRect(x - cell / 2 + inset, y - cell / 2 + inset, cell - 2 * inset, cell - 2 * inset, cell * CORNER);
    ctx.strokeStyle = accent;
    ctx.lineWidth = thin(0.045);
    ctx.stroke();
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return false;
    acted = view.t;
    queued = null;
    const before = top;
    learn();
    top = highest();
    grew = { top: before, at: view.t, gap: state.over ? (lock ? SWEEP : RUSH) : grow };
    if (act.verb === "drop") plopped = { p: act.args.y * state.size + act.args.x, at: view.t };
    busy = bloomed() + SHOW;
    if (state.over) {
      ended = bloomed();
      busy = ended + (value.play === "me" ? BLOOM : rest);
    }
    tell();
    paint();
    return true;
  };
  const advance = () => {
    if (paused || held < Infinity) return;
    if (state.over) {
      if (value.play === "me") return;
      if (view.t - ended < rest) return;
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
    if (view.t - acted < pace || view.t < bloomed()) return;
    const act = pick(state, value.play, view.rand);
    if (!act) return;
    if (act.verb !== "drop" || view.still) {
      apply(act);
      return;
    }
    queued = act;
    queuedAt = view.t;
    busy = view.t + span(sight);
    paint();
  };
  const draw = () => {
    if (view.t !== last) advance();
    last = view.t;
    if (view.t <= busy + TAIL || (threatened() && !view.still)) paint();
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
  const read = (u: number, v: number): [number, number] | null => {
    const { left, top: up, cell } = layout();
    const fx = (u * view.w - left) / cell;
    const fy = (v * view.h - up) / cell;
    if (fx < 0 || fy < 0 || fx >= state.size || fy >= state.size) return null;
    return [Math.floor(fx), Math.floor(fy)];
  };
  const hover = (u: number, v: number) => {
    if (state.over) return;
    const spot = read(u, v);
    const moved = spot !== null && (spot[0] !== cursor[0] || spot[1] !== cursor[1]);
    if (!moved && hovered === (spot !== null)) return;
    if (spot) cursor = spot;
    hovered = spot !== null;
    paint();
  };
  const tap = (u: number, v: number) => {
    if (state.over) return;
    const spot = read(u, v);
    if (spot) apply(drop(spot[0], spot[1]));
  };
  const press = () => {
    if (!state.over) apply(drop(cursor[0], cursor[1]));
  };
  const wait = () => {
    if (!state.over) apply(WAIT);
  };
  const move = (key: string) => {
    const way = STEPS[key];
    if (!way || state.over) return;
    const x = Math.min(state.size - 1, Math.max(0, cursor[0] + way[0]));
    const y = Math.min(state.size - 1, Math.max(0, cursor[1] + way[1]));
    if (x === cursor[0] && y === cursor[1]) return;
    cursor = [x, y];
    hovered = false;
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
    wait,
    press,
    move,
    hover,
    tap,
    ring: () => cursor,
    state: () => state,
  };
}
