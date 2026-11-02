import { blend, bright, hex, rgb, TINTS, veil } from "../../net/sdk/color.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick as choose } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import { board } from "../../net/sdk/run.ts";
import type { Spec, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import type { Units as Loaded } from "../shared/units/index.ts";
import { wasm } from "../shared/units/index.ts";
import type { Act, Laid, Over, State } from "./engine.ts";
import {
  BASE,
  cap,
  DIM,
  deal as dealing,
  depth,
  LEFT,
  lay,
  NUMBERS,
  RIGHT,
  starts,
  step,
  turn,
  under,
} from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { number: number; level: number; mess: number; play: string; code: string; tint: string };

export type Told = { moves: number; par: number; over: Over; level: number; of: number };

export type Given = Opts & { math?: Mrly | null; onTell?: (told: Told) => void };

export type Hands = {
  level: () => number;
  done: () => boolean;
  again: () => void;
  deeper: () => void;
  twist: (way: number) => void;
  move: (key: string) => void;
  hover: (u: number, v: number) => void;
  tap: (u: number, v: number, way: number) => void;
};

export type Held = Scene & Partial<Hands>;

export type Mend = ReturnType<typeof make>;

type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

const HIDE = () => false;

export const SPEC: Spec = [
  { key: "number", label: "Side", kind: "pick", def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: "Board" },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 3,
    min: 1,
    max: (value) => cap(Number(value.number) || NUMBERS[0]),
    step: 1,
    group: "Board",
  },
  { key: "mess", label: "Scramble", kind: "slider", def: 10, min: 1, max: 40, step: 1, unit: "turns", group: "Board" },
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
  { key: "code", label: "Code", kind: "text", def: "", group: "Design", when: HIDE },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const units = wasm("math");

const ABOUT = [
  "A design is a code: the cells of a small square it fills. Stamped into every filled cell of itself, level by level, it grows into a fractal whose blocks at every scale are the same picture.",
  "Here the blocks have been turned. Turn them back. A block carries everything inside it, so the whole comes first, then the big blocks, then the small ones. The faint picture is the fractal to match.",
  "Par is the fewest quarter turns that mend the deal, worked out level by level. Smart plays it in exactly par.",
  "Tap or click a block to turn it right, right click or shift click to turn it left. Enter picks the level of the blocks you turn, the arrows move, and the period and comma turn. Reroll deals again, and the design chip picks another fractal. [Designs](/designs/) grows any code.",
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
  units,
  about: ABOUT,
  gestures: true,
  design: { dim: DIM, base: BASE, code: "code", blank: "Triangle" },
  name: (value: Value) => `mend-${value.code || "triangle"}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : `Level ${scene?.level?.() ?? 3}`),
      act: "focus",
      button: true,
      menu: "Edit",
    },
    { key: ".", label: "Turn right", act: "right" },
    { key: ",", label: "Turn left", act: "left" },
    { key: ARROWS, label: "Move", act: "move" },
    { key: "r", label: "New deal", act: "reroll" },
  ],
  actions: {
    focus: (scene) => (scene.done?.() ? scene.again?.() : scene.deeper?.()),
    right: (scene) => scene.twist?.(RIGHT),
    left: (scene) => scene.twist?.(LEFT),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Loaded<"math">, Held>;

const CODES = ["473", "95", "123", "179", "409", "79"];

export const SAVERS = [
  {
    id: "mending",
    title: "Mending",
    pick: (rand: Rng) => ({
      play: "smart",
      number: 3,
      level: 3,
      mess: 10 + Math.floor(rand() * 5),
      code: choose(rand, CODES),
    }),
  },
  {
    id: "deep",
    title: "Deep mend",
    pick: (rand: Rng) => ({
      play: "smart",
      number: 3,
      level: 4,
      mess: 16 + Math.floor(rand() * 7),
      code: choose(rand, CODES),
    }),
  },
];

const FIT = 0.86;
const RING = 0.12;
const LINE = 2;
const PAD = 0.12;
const CORNER = 0.18;
const PACE = 420;
const SPIN = 260;
const HOLD = 2400;
const BLOOM = 1600;
const TAIL = 100;
const SLOW = 1100;
const TURN = 720;
const REST = 5200;
const FADE = 1200;
const LEAVE = 900;
const SETTLE = 760;
const GAP = 0.08;
const TILE = 7;
const BED = 0.9;
const SPOT = 0.75;
const LAMP = 0.16;
const BREATH = 3600;
const SHEEN = 1500;
const WHITE = "#ffffff";
const BLACK = "#000000";
const DEAL: (keyof Knobs)[] = ["number", "level", "mess", "code"];

const ease = (k: number) => (k >= 1 ? 1 : 1 - (1 - k) ** 3);

const snap = (k: number) => (k >= 1 ? 1 : 1 + 2.4 * (k - 1) ** 3 + 1.4 * (k - 1) ** 2);

const opposite = (color: string, dark: boolean) => {
  const c = rgb(color);
  const sum = Math.max(...c) + Math.min(...c);
  const flip = hex(c.map((v) => sum - v));
  const [r, g, b] = rgb(flip);
  const lum = Math.max(1, 0.3 * r + 0.59 * g + 0.11 * b);
  return dark
    ? blend(flip, WHITE, Math.max(0, (110 - lum) / Math.max(1, 255 - lum)))
    : blend(flip, BLACK, Math.max(0, (lum - 150) / lum));
};

const fade = (k: number) => ease(Math.max(0, k));

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("mend: the scene wants @mrlyprod/math as opts.math");
  const screen = canvas.getContext("2d")!;
  let ctx: Pen = screen;
  let sheet: OffscreenCanvas | null = null;
  let drawn: State | null = null;
  let keyed = "";
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let laid: Laid;
  let goal: Uint8Array;
  let focus = 0;
  let cursor = -1;
  let spun: { block: number; way: number; at: number } | null = null;
  let settled: { block: number; at: number } | null = null;
  let hinted: State | null = null;
  let hint: Act | null = null;
  let acted = view.t;
  let ended = view.t;
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
  const twirl = lock ? TURN : SPIN;
  const rest = lock ? REST : HOLD;
  const tell = () => {
    if (gone) return;
    opts.onTell?.({
      moves: state.moves,
      par: state.par,
      over: state.over,
      level: state.level - focus,
      of: state.level,
    });
  };
  const deal = () => {
    state = dealing(value, { seed, round, units: { math } });
    laid = lay(state);
    goal = Uint8Array.from(state.goal, (one) => (one === "1" ? 1 : 0));
    focus = 0;
    cursor = state.turns.length ? 0 : -1;
    spun = null;
    settled = null;
    acted = view.t;
    ended = view.t;
    busy = view.t;
    tell();
  };
  const layout = () => board(view, laid.side, laid.side, FIT);
  const box = (block: number) => laid.boxes.subarray(block * 3, block * 3 + 3);
  const tiles = (
    cells: Uint8Array,
    left: number,
    top: number,
    cell: number,
    keep: (x: number, y: number) => boolean,
    rule: (i: number) => boolean,
    from = 0,
    tall = 1,
  ) => {
    const side = laid.side;
    const gap = cell >= TILE ? cell * GAP : 0;
    ctx.beginPath();
    for (let y = 0; y < side; y++) {
      let x = 0;
      while (x < side) {
        const i = y * side + x;
        if (!cells[i] || !rule(i) || !keep(x, y)) {
          x++;
          continue;
        }
        const start = x;
        if (gap > 0) x++;
        else while (x < side && cells[y * side + x] && rule(y * side + x) && keep(x, y)) x++;
        const w = (x - start) * cell - 2 * gap;
        const h = Math.max(0, cell - 2 * gap);
        ctx.rect(left + start * cell + gap, top + y * cell + gap + h * from, w, h * tall);
      }
    }
    ctx.fill();
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const after = (from: number, span: number) => Math.max(0, Math.min(1, (view.t - from) / span));
  const right = (block: number) => {
    const [bx, by, size] = box(block);
    for (let y = by; y < by + size; y++)
      for (let x = bx; x < bx + size; x++) if (laid.cells[y * laid.side + x] !== goal[y * laid.side + x]) return false;
    return true;
  };
  const paint = () => {
    const { x: left, y: top, w: span, cell } = layout();
    const { accent, paper, ink } = view.look();
    const dark = !bright(paper);
    const thin = (k: number) => Math.max(0.5, cell * k);
    const ground = blend(paper, accent, dark ? 0.06 : 0.05);
    const floor = dark ? blend(blend(paper, BLACK, 0.4), accent, 0.08) : blend(WHITE, accent, 0.06);
    const well = dark ? blend(blend(paper, BLACK, 0.55), accent, 0.1) : blend(WHITE, accent, 0.11);
    const set = dark ? blend(accent, WHITE, 0.08) : accent;
    const stray = opposite(accent, dark);
    const glow = dark ? blend(accent, WHITE, 0.3) : blend(accent, BLACK, 0.05);
    const ghost = veil(accent, dark ? 0.2 : 0.16);
    const k = spun ? snap(after(spun.at, twirl)) : 1;
    if (spun && view.t - spun.at >= twirl) spun = null;
    const turning = spun ? box(spun.block) : null;
    const inside = (x: number, y: number) =>
      turning !== null &&
      x >= turning[0] &&
      x < turning[0] + turning[2] &&
      y >= turning[1] &&
      y < turning[1] + turning[2];
    const outside = (x: number, y: number) => !inside(x, y);
    const done = state.over ? fade(after(ended, BLOOM)) : 0;
    const since = Math.max(0, view.t - ended);
    const breath = done > 0 && !view.still ? 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH) : 0;
    const all = () => true;
    const good = (i: number) => goal[i] === 1;
    const bad = (i: number) => goal[i] === 0;
    const pad = Math.max(0, Math.min(cell * BED + span * 0.02, Math.min(left, top) * 0.75));
    const body = (keep: (x: number, y: number) => boolean, glowing: boolean) => {
      if (glowing) {
        ctx.save();
        ctx.shadowColor = glow;
        ctx.shadowBlur = Math.max(1, cell * 1.8);
        ctx.fillStyle = veil(glow, 0.9);
        tiles(laid.cells, left, top, cell, keep, good);
        ctx.restore();
      }
      ctx.fillStyle = set;
      tiles(laid.cells, left, top, cell, keep, good);
      ctx.fillStyle = stray;
      tiles(laid.cells, left, top, cell, keep, bad);
      if (cell < TILE) return;
      ctx.fillStyle = veil(WHITE, dark ? 0.26 : 0.32);
      tiles(laid.cells, left, top, cell, keep, all, 0, 0.16);
      ctx.fillStyle = veil(BLACK, dark ? 0.3 : 0.16);
      tiles(laid.cells, left, top, cell, keep, all, 0.84, 0.16);
    };
    const still = () => {
      const big = Math.max(1, Math.max(view.w, view.h) * SPOT);
      const lamp = ctx.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, big);
      lamp.addColorStop(0, veil(accent, (dark ? LAMP : LAMP / 2) + (state.over ? 0.1 : 0)));
      lamp.addColorStop(1, veil(accent, 0));
      ctx.fillStyle = lamp;
      ctx.fillRect(0, 0, view.w, view.h);
      ctx.save();
      ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.16})`;
      ctx.shadowBlur = Math.max(1, pad * 1.2);
      ctx.shadowOffsetY = pad * 0.2;
      ctx.beginPath();
      ctx.roundRect(left - pad, top - pad, span + 2 * pad, span + 2 * pad, Math.max(0, pad * 0.7));
      ctx.fillStyle = floor;
      ctx.fill();
      ctx.shadowColor = "transparent";
      ctx.strokeStyle = veil(dark ? WHITE : ink, dark ? 0.07 : 0.08);
      ctx.lineWidth = thin(0.04);
      ctx.stroke();
      ctx.restore();
      ctx.fillStyle = well;
      ctx.fillRect(left, top, span, span);
      if (!state.over) {
        ctx.fillStyle = ghost;
        tiles(goal, left, top, cell, all, (i) => !laid.cells[i]);
      }
      body(outside, state.over !== null);
    };
    screen.setTransform(1, 0, 0, 1, 0, 0);
    screen.globalAlpha = 1;
    screen.fillStyle = ground;
    screen.fillRect(0, 0, view.w, view.h);
    const alpha = lock ? shade() : 1;
    const key = `${view.w}|${view.h}|${paper}|${accent}|${ink}|${spun ? spun.block : -1}`;
    if (typeof OffscreenCanvas === "function") {
      const w = Math.max(1, Math.ceil(view.w));
      const h = Math.max(1, Math.ceil(view.h));
      if (!sheet || sheet.width !== w || sheet.height !== h) {
        sheet = new OffscreenCanvas(w, h);
        drawn = null;
      }
      if (drawn !== state || keyed !== key) {
        const pen = sheet.getContext("2d")!;
        pen.setTransform(1, 0, 0, 1, 0, 0);
        pen.globalAlpha = 1;
        pen.clearRect(0, 0, w, h);
        ctx = pen;
        still();
        ctx = screen;
        drawn = state;
        keyed = key;
      }
      screen.globalAlpha = alpha;
      screen.drawImage(sheet, 0, 0);
    } else {
      screen.globalAlpha = alpha;
      still();
    }
    ctx = screen;
    if (done > 0 && breath > 0) {
      ctx.fillStyle = veil(glow, (dark ? 0.08 : 0.06) * breath * done);
      ctx.fillRect(left, top, span, span);
    }
    if (turning && spun) {
      const half = (turning[2] * cell) / 2;
      const mx = left + turning[0] * cell + half;
      const my = top + turning[1] * cell + half;
      const lift = 1 + 0.05 * Math.sin(Math.PI * Math.min(1, k));
      ctx.save();
      ctx.translate(mx, my);
      ctx.rotate((-(1 - k) * spun.way * Math.PI) / 2);
      ctx.scale(lift, lift);
      ctx.translate(-mx, -my);
      ctx.fillStyle = `rgba(0, 0, 0, ${dark ? 0.45 : 0.18})`;
      tiles(laid.cells, left, top + cell * 0.3, cell, inside, all);
      body(inside, false);
      ctx.restore();
    }
    if (settled && !view.still) {
      const s = after(settled.at, SETTLE);
      if (view.t >= settled.at + SETTLE) settled = null;
      else if (view.t >= settled.at) {
        const [bx, by, size] = box(settled.block);
        const grow = cell * (0.2 + 1.4 * ease(s));
        const x = left + bx * cell - grow;
        const y = top + by * cell - grow;
        const w = size * cell + 2 * grow;
        ctx.save();
        ctx.globalAlpha *= 1 - s;
        ctx.strokeStyle = veil(glow, 0.3);
        ctx.lineWidth = Math.max(2, cell * 0.8 * (1 - s) + 3 * view.dpr);
        ctx.beginPath();
        ctx.roundRect(x, y, w, w, Math.max(0, Math.min(w * CORNER, cell * 2)));
        ctx.stroke();
        ctx.strokeStyle = glow;
        ctx.lineWidth = Math.max(1, cell * 0.22 * (1 - s) + view.dpr);
        ctx.beginPath();
        ctx.roundRect(x, y, w, w, Math.max(0, Math.min(w * CORNER, cell * 2)));
        ctx.stroke();
        ctx.fillStyle = veil(glow, 0.18);
        ctx.fill();
        ctx.restore();
      }
    }
    if (done > 0 && !view.still && since < SHEEN + BLOOM) {
      const s = since / (SHEEN + BLOOM);
      const at = left - span + s * 3 * span;
      const sheen = ctx.createLinearGradient(at - span * 0.3, top, at + span * 0.3, top + span);
      sheen.addColorStop(0, veil(WHITE, 0));
      sheen.addColorStop(0.5, veil(WHITE, dark ? 0.55 : 0.45));
      sheen.addColorStop(1, veil(WHITE, 0));
      ctx.fillStyle = sheen;
      tiles(laid.cells, left, top, cell, all, all);
    }
    if (state.over) return;
    if (hinted !== state) {
      hinted = state;
      hint = pick(state, "smart", view.rand);
    }
    const ring = (block: number) => {
      const [bx, by, size] = box(block);
      const p = Math.max(1, Math.round(cell * Math.min(size, 3) * PAD));
      const x = left + bx * cell - p;
      const y = top + by * cell - p;
      const s = size * cell + 2 * p;
      ctx.beginPath();
      ctx.roundRect(x, y, s, s, Math.max(0, Math.min(s * CORNER, cell * 2)));
      return [x + s / 2, y + s / 2, s] as const;
    };
    if (hint && !spun) {
      const [mx, my, s] = ring(hint.args.block);
      const way = hint.args.way;
      const tone = dark ? blend(stray, WHITE, 0.2) : blend(stray, BLACK, 0.1);
      ctx.save();
      ctx.setLineDash([thin(0.5), thin(0.35)]);
      ctx.lineWidth = Math.max(1, view.dpr * 1.5);
      ctx.strokeStyle = veil(tone, 0.85);
      ctx.stroke();
      ctx.setLineDash([]);
      const r = Math.max(2, Math.min(s * 0.22, cell * 2.4));
      const a0 = -Math.PI / 2 - way * 0.2;
      const a1 = a0 + way * Math.PI * 1.1;
      ctx.lineWidth = Math.max(1.5, r * 0.16);
      ctx.lineCap = "round";
      ctx.beginPath();
      ctx.arc(mx, my, r, a0, a1, way < 0);
      ctx.stroke();
      const hx = mx + r * Math.cos(a1);
      const hy = my + r * Math.sin(a1);
      const tx = -Math.sin(a1) * way;
      const ty = Math.cos(a1) * way;
      const h = r * 0.42;
      ctx.beginPath();
      ctx.moveTo(hx + tx * h, hy + ty * h);
      ctx.lineTo(hx - Math.cos(a1) * h * 0.6, hy - Math.sin(a1) * h * 0.6);
      ctx.lineTo(hx + Math.cos(a1) * h * 0.6, hy + Math.sin(a1) * h * 0.6);
      ctx.closePath();
      ctx.fillStyle = tone;
      ctx.fill();
      ctx.restore();
    }
    if (cursor < 0 || lock) return;
    ring(cursor);
    ctx.fillStyle = veil(accent, RING);
    ctx.fill();
    ctx.lineWidth = Math.max(1, Math.round(LINE * view.dpr));
    ctx.strokeStyle = glow;
    ctx.stroke();
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return;
    laid = lay(state);
    acted = view.t;
    cursor = act.args.block;
    focus = depth(state, cursor);
    spun = view.still ? null : { block: cursor, way: act.args.way, at: view.t };
    busy = view.t + twirl;
    if (!view.still && right(cursor)) {
      settled = { block: cursor, at: view.t + twirl * 0.6 };
      busy = view.t + twirl + SETTLE;
    }
    if (state.over) {
      ended = view.t;
      busy = view.t + (lock ? rest : BLOOM + SHEEN);
    }
    tell();
    paint();
  };
  const advance = () => {
    if (value.play === "me" || paused || held < Infinity) return;
    if (state.over) {
      if (view.t - ended < rest) return;
      round++;
      deal();
      if (lock) {
        rise = view.t;
        busy = view.t + FADE;
      }
      return;
    }
    if (view.t - acted < pace) return;
    const act = pick(state, value.play, view.rand);
    if (act) apply(act);
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
  const cellAt = (u: number, v: number): [number, number] => {
    const { x: left, y: top, cell } = layout();
    return [Math.floor((u * view.w - left) / cell), Math.floor((v * view.h - top) / cell)];
  };
  const aim = (block: number) => {
    if (block === cursor) return;
    cursor = block;
    paint();
  };
  const hover = (u: number, v: number) => {
    if (state.over) return;
    const [x, y] = cellAt(u, v);
    const block = under(state, laid, focus, x, y);
    if (block >= 0) aim(block);
  };
  const tap = (u: number, v: number, way: number) => {
    if (state.over) return;
    const [x, y] = cellAt(u, v);
    const block = under(state, laid, focus, x, y);
    if (block < 0) return;
    aim(block);
    apply(turn(block, way));
  };
  const twist = (way: number) => {
    if (cursor >= 0) apply(turn(cursor, way));
  };
  const deeper = () => {
    if (state.over || !state.turns.length) return;
    const at = starts(state);
    const next = (focus + 1) % state.level;
    const [bx, by, size] = cursor >= 0 ? box(cursor) : [0, 0, laid.side];
    const mid = Math.floor(size / 2);
    let block = next ? under(state, laid, next, bx + mid, by + mid) : 0;
    for (let id = at[next]; block < 0 && id < at[next + 1]; id++) {
      const [x, y] = box(id);
      if (x >= bx && x < bx + size && y >= by && y < by + size) block = id;
    }
    focus = next;
    cursor = block < 0 ? at[next] : block;
    tell();
    paint();
  };
  const move = (key: string) => {
    const way = STEPS[key];
    if (!way || state.over || cursor < 0) return;
    const at = starts(state);
    const [bx, by] = box(cursor);
    let best = -1;
    let low = Infinity;
    for (let id = at[focus]; id < at[focus + 1]; id++) {
      const [x, y] = box(id);
      const along = (x - bx) * way[0] + (y - by) * way[1];
      if (along <= 0) continue;
      const score = along + 2 * Math.abs((x - bx) * way[1] + (y - by) * way[0]);
      if (score < low) {
        low = score;
        best = id;
      }
    }
    if (best >= 0) aim(best);
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
    level: () => state.level - focus,
    done: () => state.over !== null,
    deeper,
    twist,
    move,
    hover,
    tap,
    ring: () => cursor,
    state: () => state,
  };
}
