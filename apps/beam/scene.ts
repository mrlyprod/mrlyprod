import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Act, Hop, Over, State } from "./engine.ts";
import {
  deal as dealing,
  FEW,
  LARGE,
  lit,
  MANY,
  mirror,
  RISE,
  SKY,
  SMALL,
  STAR,
  STEPS,
  step,
  tilt,
  trace,
} from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { size: number; mirrors: number; stars: number; play: string; tint: string };

export type Told = { taps: number; par: number; lit: number; of: number; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = {
  done: () => boolean;
  again: () => void;
  hint: () => void;
  press: () => void;
  move: (key: string) => void;
  hover: (u: number, v: number) => void;
  tap: (u: number, v: number) => void;
};

export type Held = Scene & Partial<Hands>;

export type Beam = ReturnType<typeof make>;

export const SPEC: Spec = [
  {
    key: "size",
    label: "Room",
    kind: "slider",
    def: 9,
    min: SMALL,
    max: LARGE,
    step: 1,
    unit: "cells",
    group: "Room",
  },
  { key: "mirrors", label: "Mirrors", kind: "slider", def: 16, min: FEW, max: MANY, step: 1, group: "Room" },
  { key: "stars", label: "Stars", kind: "slider", def: 5, min: 1, max: SKY, step: 1, group: "Room" },
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
  "A lamp on the wall shines one beam into a dark room of mirrors. Each mirror leans one way or the other and turns the beam a quarter turn. Tap a mirror to tilt it the other way.",
  "Light every star at once. The beam must pass over all of them on its one path before it leaves the room; a star it misses stays dark.",
  "A mirror the beam never meets does nothing, so the room is a tree of choices: every mirror on the path opens a new branch. Par counts the fewest tilts that light the room, and some mirrors must be met twice, once from each side.",
  "Tap a mirror, or move with the arrows and tilt with X. Enter asks for a hint. Smart finds the fewest tilts and plays them in the beam's order. Reroll deals another room.",
];

const ARROWS = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

const KEYED: Record<string, [number, number]> = {
  ArrowUp: [0, -1],
  ArrowDown: [0, 1],
  ArrowLeft: [-1, 0],
  ArrowRight: [1, 0],
};

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  gestures: true,
  name: (value: Value) => `beam-${value.size}-${value.mirrors}-${value.stars}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : "Hint"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: "x", label: "Tilt", act: "tilt" },
    { key: ARROWS, label: "Move", act: "move" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.hint?.()),
    tilt: (scene) => scene.press?.(),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "lantern",
    title: "Lantern",
    pick: (rand: Rng) => ({ play: "smart", size: 9, mirrors: 14 + Math.floor(rand() * 5), stars: 4 }),
  },
  {
    id: "hall",
    title: "Hall",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 14,
      mirrors: 32 + Math.floor(rand() * 9),
      stars: 6 + Math.floor(rand() * 3),
    }),
  },
];

export const FIT = 0.84;
export const BED = 0.7;
const START = 0.95;
const DOT = 0.03;
const GLASS = 0.4;
const PANE = 0.09;
const PIVOT = 0.06;
const SPARK = 0.3;
const PIN = 0.34;
const ORB = 0.24;
const HALO = 0.8;
const CORNER = 0.22;
const LIGHT = 0.12;
const LAMP = 0.1;
const SPOT = 0.7;
const BLACK = "#000000";
const WHITE = "#ffffff";
const SPEED = 30;
const SLOWER = 14;
const SWING = 220;
const PACE = 700;
const SLOW = 1300;
const AIM = 320;
const SIGHT = 800;
const FLASH = 500;
const BLOOM = 1400;
const BREATH = 2600;
const TWINKLE = 1700;
const RUN = 2200;
const HOLD = 3600;
const REST = 6400;
const FADE = 1200;
const LEAVE = 900;
const TAIL = 100;
const DEAL: (keyof Knobs)[] = ["size", "mirrors", "stars"];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const spring = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 + 2.6 * (k - 1) ** 3 + 1.6 * (k - 1) ** 2);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (p: number, salt: number) => {
  let s = Math.imul(p + 1, 0x9e3779b1) ^ Math.imul(salt + 7, 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 15), 0x2c1b3c6d);
  return ((s ^ (s >>> 13)) >>> 0) / 4294967296;
};

/* PATH */

export type Ray = { points: [number, number][]; arcs: number[]; length: number; hops: Hop[] };

export function ray(state: Pick<State, "size" | "board" | "lamp" | "way">): Ray {
  const { size, lamp, way } = state;
  const hops = trace(state);
  const center = (p: number): [number, number] => [(p % size) + 0.5, Math.floor(p / size) + 0.5];
  const [lx, ly] = center(lamp);
  const points: [number, number][] = [[lx - STEPS[way][0] * START, ly - STEPS[way][1] * START]];
  const arcs = [0];
  let from = way;
  hops.forEach((hop, i) => {
    if (hop.way !== from) {
      points.push(center(hop.p));
      arcs.push(START + i);
    }
    from = hop.way;
  });
  const last = hops[hops.length - 1];
  const [ex, ey] = center(last.p);
  const out = 0.5 + BED;
  points.push([ex + STEPS[last.way][0] * out, ey + STEPS[last.way][1] * out]);
  const length = START + hops.length - 1 + out;
  arcs.push(length);
  return { points, arcs, length, hops };
}

export function shared(a: Hop[], b: Hop[]): number {
  let k = 0;
  while (k < a.length && k < b.length && a[k].p === b[k].p && a[k].way === b[k].way) k++;
  return k >= a.length && k >= b.length ? Infinity : START + k;
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let path: Ray;
  let old: Ray | null = null;
  let from = 0;
  let until = 0;
  let shone = { at: view.t, from: 0, speed: SPEED };
  let swung: { p: number; at: number } | null = null;
  let cursor: [number, number] = [0, 0];
  let hovered = false;
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
  const speed = lock ? SLOWER : SPEED;
  const rest = lock ? REST : HOLD;
  const span = (ms: number) => (view.still ? 0 : ms);
  const after = (at: number, ms: number) => (view.still ? 1 : fade((view.t - at) / ms));
  const tell = () => {
    if (gone) return;
    opts.onTell?.({ taps: state.taps, par: state.par, lit: lit(state).size, of: state.stars, over: state.over });
  };
  const reach = () =>
    view.still ? path.length : Math.min(path.length, shone.from + ((view.t - shone.at) * shone.speed) / 1000);
  const arrived = () => shone.at + (span(1000) * Math.max(0, path.length - shone.from)) / shone.speed;
  const deal = () => {
    state = dealing(value, { seed, round });
    path = ray(state);
    old = null;
    shone = { at: view.t, from: 0, speed };
    swung = null;
    queued = null;
    ended = state.over ? view.t : Infinity;
    acted = view.t;
    busy = arrived();
    cursor = [Math.floor(state.size / 2), Math.floor(state.size / 2)];
    tell();
  };
  const layout = () => {
    const cell = Math.max(0.01, (Math.min(view.w, view.h) * FIT) / (state.size + 2 * BED));
    const side = cell * state.size;
    return { left: (view.w - side) / 2, top: (view.h - side) / 2, cell };
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const firsts = () => {
    const out = new Map<number, number>();
    path.hops.forEach((hop, i) => {
      if (state.board[hop.p] === STAR && !out.has(hop.p)) out.set(hop.p, START + i);
    });
    return out;
  };
  const paint = () => {
    const { left, top: up, cell } = layout();
    const { accent, paper, ink } = view.look();
    const dark = !bright(paper);
    const n = state.size;
    const board = state.board;
    const thin = (k: number) => Math.max(0.5, cell * k);
    const px = ([x, y]: [number, number]): [number, number] => [left + x * cell, up + y * cell];
    const center = (p: number): [number, number] => px([(p % n) + 0.5, Math.floor(p / n) + 0.5]);
    const ground = dark ? blend(paper, accent, 0.05) : blend(paper, accent, 0.04);
    const floor = dark ? blend(blend(paper, BLACK, 0.4), accent, 0.07) : blend(WHITE, accent, 0.07);
    const glow = dark ? blend(accent, WHITE, 0.25) : blend(accent, BLACK, 0.08);
    const core = dark ? blend(accent, WHITE, 0.82) : blend(accent, WHITE, 0.55);
    const gold = dark ? blend(accent, WHITE, 0.35) : blend(accent, BLACK, 0.04);
    const silver = dark ? blend(paper, WHITE, 0.62) : blend(ink, WHITE, 0.45);
    const back = dark ? blend(paper, WHITE, 0.2) : blend(ink, WHITE, 0.72);
    const reached = reach();
    const done = state.over === "done" ? after(ended, BLOOM) : 0;
    const since = Math.max(0, view.t - ended);
    const breath = done > 0 ? 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH) : 0;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.fillStyle = ground;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const big = Math.max(1, Math.min(view.w, view.h) * SPOT);
    const lamp = ctx.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, big);
    lamp.addColorStop(0, veil(accent, (dark ? LAMP : LAMP / 2) + 0.12 * done * (0.7 + 0.3 * breath)));
    lamp.addColorStop(1, veil(accent, 0));
    ctx.fillStyle = lamp;
    ctx.fillRect(0, 0, view.w, view.h);
    const pad = cell * BED;
    ctx.save();
    ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.16})`;
    ctx.shadowBlur = cell * 1.2;
    ctx.shadowOffsetY = cell * 0.2;
    ctx.beginPath();
    ctx.roundRect(left - pad, up - pad, n * cell + 2 * pad, n * cell + 2 * pad, cell * 0.6);
    ctx.fillStyle = floor;
    ctx.fill();
    ctx.shadowColor = "transparent";
    ctx.strokeStyle = veil(dark ? WHITE : ink, dark ? 0.06 : 0.07);
    ctx.lineWidth = thin(0.03);
    ctx.stroke();
    ctx.restore();
    ctx.beginPath();
    for (let y = 1; y < n; y++)
      for (let x = 1; x < n; x++) {
        const [a, b] = px([x, y]);
        ctx.moveTo(a + cell * DOT, b);
        ctx.arc(a, b, cell * DOT, 0, 2 * Math.PI);
      }
    ctx.fillStyle = veil(ink, dark ? 0.14 : 0.12);
    ctx.fill();
    const line = (list: [number, number][], arcs: number[], a: number, b: number) => {
      ctx.beginPath();
      let moved = false;
      for (let i = 0; i + 1 < list.length; i++) {
        const s0 = arcs[i];
        const s1 = arcs[i + 1];
        if (s1 <= a || s0 >= b || s1 <= s0) continue;
        const k0 = Math.max(0, (a - s0) / (s1 - s0));
        const k1 = Math.min(1, (b - s0) / (s1 - s0));
        const [x0, y0] = list[i];
        const [x1, y1] = list[i + 1];
        const p0 = px([x0 + (x1 - x0) * k0, y0 + (y1 - y0) * k0]);
        const p1 = px([x0 + (x1 - x0) * k1, y0 + (y1 - y0) * k1]);
        if (!moved) ctx.moveTo(...p0);
        ctx.lineTo(...p1);
        moved = true;
      }
      return moved;
    };
    const shine = (one: Ray, a: number, b: number, alpha: number) => {
      if (alpha <= 0 || !line(one.points, one.arcs, a, b)) return;
      ctx.save();
      ctx.lineCap = "round";
      ctx.lineJoin = "round";
      ctx.globalAlpha *= alpha;
      ctx.strokeStyle = veil(glow, dark ? 0.16 + 0.08 * done : 0.12 + 0.06 * done);
      ctx.lineWidth = thin(0.42 + 0.12 * done * breath);
      ctx.stroke();
      if (dark) {
        ctx.shadowColor = glow;
        ctx.shadowBlur = cell * (0.4 + 0.3 * done);
      }
      ctx.strokeStyle = glow;
      ctx.lineWidth = thin(0.13);
      ctx.stroke();
      ctx.shadowBlur = 0;
      ctx.strokeStyle = core;
      ctx.lineWidth = thin(0.045);
      ctx.stroke();
      ctx.restore();
    };
    if (old) {
      const k = 1 - after(shone.at, FLASH);
      shine(old, from, until, k * 0.8);
    }
    shine(path, 0, reached, 1);
    const tail = path.points[path.points.length - 1];
    if (reached >= path.length) {
      const way = path.hops[path.hops.length - 1].way;
      const [x0, y0] = px(tail);
      const far = Math.max(view.w, view.h);
      const x1 = x0 + STEPS[way][0] * far;
      const y1 = y0 + STEPS[way][1] * far;
      const out = ctx.createLinearGradient(x0, y0, x1, y1);
      out.addColorStop(0, veil(glow, dark ? 0.55 : 0.45));
      out.addColorStop(0.35, veil(glow, 0));
      ctx.save();
      ctx.lineCap = "round";
      ctx.strokeStyle = out;
      ctx.lineWidth = thin(0.1);
      ctx.beginPath();
      ctx.moveTo(x0, y0);
      ctx.lineTo(x1, y1);
      ctx.stroke();
      ctx.restore();
    }
    if (done > 0 && !view.still) {
      const s = ((since % RUN) / RUN) * (path.length + 2) - 1;
      ctx.save();
      ctx.globalAlpha *= done;
      if (line(path.points, path.arcs, Math.max(0, s - 0.9), Math.max(0, s))) {
        ctx.lineCap = "round";
        ctx.strokeStyle = veil(WHITE, dark ? 0.85 : 0.75);
        ctx.lineWidth = thin(0.09);
        ctx.stroke();
      }
      ctx.restore();
    }
    const hit = new Set(path.hops.filter((hop) => mirror(board[hop.p])).map((hop) => hop.p));
    const met = new Map<number, number>();
    path.hops.forEach((hop, i) => {
      if (hit.has(hop.p) && !met.has(hop.p)) met.set(hop.p, START + i);
    });
    for (let p = 0; p < n * n; p++) {
      if (!mirror(board[p])) continue;
      const [x, y] = center(p);
      const target = board[p] === RISE ? -Math.PI / 4 : Math.PI / 4;
      const k = swung && swung.p === p ? after(swung.at, SWING) : 1;
      const angle = target - (1 - k) * (Math.PI / 2);
      const on = met.has(p) && reached >= met.get(p)!;
      if (on) {
        const flare = ctx.createRadialGradient(x, y, 0, x, y, cell * 0.55);
        flare.addColorStop(0, veil(glow, dark ? 0.5 : 0.3));
        flare.addColorStop(1, veil(glow, 0));
        ctx.fillStyle = flare;
        ctx.fillRect(x - cell * 0.6, y - cell * 0.6, cell * 1.2, cell * 1.2);
      }
      const half = cell * GLASS;
      const ux = Math.cos(angle) * half;
      const uy = Math.sin(angle) * half;
      const nx = -Math.sin(angle) * cell * PANE * 0.5;
      const ny = Math.cos(angle) * cell * PANE * 0.5;
      ctx.save();
      ctx.lineCap = "round";
      ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.2})`;
      ctx.shadowBlur = cell * 0.18;
      ctx.shadowOffsetY = cell * 0.05;
      ctx.beginPath();
      ctx.moveTo(x - ux + nx, y - uy + ny);
      ctx.lineTo(x + ux + nx, y + uy + ny);
      ctx.strokeStyle = back;
      ctx.lineWidth = thin(PANE);
      ctx.stroke();
      ctx.shadowBlur = 0;
      ctx.shadowOffsetY = 0;
      const sheen = ctx.createLinearGradient(x - ux, y - uy, x + ux, y + uy);
      sheen.addColorStop(0, silver);
      sheen.addColorStop(0.45, on ? core : blend(silver, WHITE, 0.6));
      sheen.addColorStop(1, silver);
      ctx.beginPath();
      ctx.moveTo(x - ux - nx, y - uy - ny);
      ctx.lineTo(x + ux - nx, y + uy - ny);
      ctx.strokeStyle = sheen;
      ctx.lineWidth = thin(PANE * 0.8);
      ctx.stroke();
      ctx.beginPath();
      ctx.arc(x, y, cell * PIVOT, 0, 2 * Math.PI);
      ctx.fillStyle = on ? core : back;
      ctx.fill();
      ctx.restore();
    }
    const stars = firsts();
    for (let p = 0; p < n * n; p++) {
      if (board[p] !== STAR) continue;
      const [x, y] = center(p);
      const arc = stars.get(p);
      const on = arc !== undefined && reached >= arc;
      const lit = on ? shone.at + (span(1000) * Math.max(0, arc - shone.from)) / shone.speed : Infinity;
      const pop = on && !view.still ? spring((view.t - lit) / (SWING * 2)) : 1;
      const twinkle = on ? 0.5 + 0.5 * Math.sin((2 * Math.PI * (view.t + hash(p, 1) * TWINKLE)) / TWINKLE) : 0;
      const r = cell * PIN * (on ? 0.85 + 0.25 * pop + 0.06 * twinkle * done : 0.8);
      const inner = r * 0.3;
      const spin = (on ? 0 : Math.PI / 4) + hash(p, 2) * 0.2;
      ctx.save();
      ctx.translate(x, y);
      ctx.rotate(spin);
      if (on) {
        const halo = ctx.createRadialGradient(0, 0, 0, 0, 0, Math.max(0.1, r * (1.4 + 0.5 * done * breath)));
        halo.addColorStop(0, veil(gold, dark ? 0.5 : 0.38));
        halo.addColorStop(1, veil(gold, 0));
        ctx.fillStyle = halo;
        ctx.beginPath();
        ctx.arc(0, 0, Math.max(0.1, r * (1.4 + 0.5 * done * breath)), 0, 2 * Math.PI);
        ctx.fill();
      }
      ctx.beginPath();
      for (let i = 0; i < 8; i++) {
        const a = (i * Math.PI) / 4 - Math.PI / 2;
        const d = i % 2 ? inner : r;
        if (i) ctx.lineTo(Math.cos(a) * d, Math.sin(a) * d);
        else ctx.moveTo(Math.cos(a) * d, Math.sin(a) * d);
      }
      ctx.closePath();
      if (on) {
        if (dark) {
          ctx.shadowColor = gold;
          ctx.shadowBlur = cell * SPARK;
        }
        ctx.fillStyle = gold;
        ctx.fill();
        ctx.shadowBlur = 0;
        ctx.beginPath();
        ctx.arc(0, 0, Math.max(0.1, inner * 0.7), 0, 2 * Math.PI);
        ctx.fillStyle = blend(gold, WHITE, 0.75);
        ctx.fill();
      } else {
        ctx.fillStyle = veil(ink, dark ? 0.06 : 0.04);
        ctx.fill();
        ctx.strokeStyle = veil(ink, dark ? 0.4 : 0.35);
        ctx.lineWidth = thin(0.03);
        ctx.stroke();
      }
      ctx.restore();
    }
    const [ox, oy] = px(path.points[0]);
    const halo = ctx.createRadialGradient(ox, oy, 0, ox, oy, cell * HALO);
    halo.addColorStop(0, veil(glow, dark ? 0.55 : 0.35));
    halo.addColorStop(1, veil(glow, 0));
    ctx.fillStyle = halo;
    ctx.beginPath();
    ctx.arc(ox, oy, cell * HALO, 0, 2 * Math.PI);
    ctx.fill();
    const bulb = ctx.createRadialGradient(ox - cell * 0.06, oy - cell * 0.08, 0, ox, oy, cell * ORB);
    bulb.addColorStop(0, WHITE);
    bulb.addColorStop(0.5, core);
    bulb.addColorStop(1, glow);
    ctx.beginPath();
    ctx.arc(ox, oy, cell * ORB, 0, 2 * Math.PI);
    ctx.fillStyle = bulb;
    ctx.fill();
    ctx.strokeStyle = veil(dark ? WHITE : ink, 0.25);
    ctx.lineWidth = thin(0.03);
    ctx.stroke();
    if (state.over) return;
    const aim = queued?.args;
    if (aim) {
      const [x, y] = center(aim.y * n + aim.x);
      ctx.beginPath();
      ctx.arc(x, y, cell * 0.46, 0, 2 * Math.PI);
      ctx.strokeStyle = accent;
      ctx.lineWidth = thin(0.045);
      ctx.stroke();
      return;
    }
    if (value.play !== "me") return;
    const p = cursor[1] * n + cursor[0];
    const [x, y] = center(p);
    if (hovered && mirror(board[p])) {
      ctx.beginPath();
      ctx.arc(x, y, cell * 0.46, 0, 2 * Math.PI);
      ctx.fillStyle = veil(accent, LIGHT);
      ctx.fill();
    }
    const inset = cell * 0.06;
    ctx.beginPath();
    ctx.roundRect(
      x - cell / 2 + inset,
      y - cell / 2 + inset,
      Math.max(0, cell - 2 * inset),
      Math.max(0, cell - 2 * inset),
      cell * CORNER,
    );
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
    const before = path;
    const shown = reach();
    path = ray(state);
    const cut = Math.min(shared(before.hops, path.hops), shown);
    old = cut < shown ? before : null;
    from = cut;
    until = shown;
    shone = { at: view.t, from: Math.min(cut, path.length), speed };
    swung = { p: act.args.y * state.size + act.args.x, at: view.t };
    busy = Math.max(arrived(), view.t + span(Math.max(SWING, FLASH)));
    if (state.over) {
      ended = arrived();
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
        busy = Math.max(busy, view.t + FADE);
      }
      return;
    }
    if (value.play === "me") return;
    if (queued) {
      if (view.t - queuedAt >= span(sight)) apply(queued);
      return;
    }
    if (view.t - acted < pace || view.t < arrived()) return;
    const act = pick(state, value.play, view.rand);
    if (!act) return;
    if (view.still) {
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
    if (view.t <= busy + TAIL || (state.over && !view.still)) paint();
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
    if (spot) apply(tilt(spot[0], spot[1]));
  };
  const press = () => {
    if (!state.over) apply(tilt(cursor[0], cursor[1]));
  };
  const hint = () => {
    if (state.over) return;
    const act = pick(state, "smart", view.rand);
    if (!act) return;
    cursor = [act.args.x, act.args.y];
    apply(act);
  };
  const move = (key: string) => {
    const way = KEYED[key];
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
    hint,
    press,
    move,
    hover,
    tap,
    ring: () => cursor,
    state: () => state,
  };
}
