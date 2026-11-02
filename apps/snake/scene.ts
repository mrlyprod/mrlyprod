import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Handle, Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, View } from "../../net/sdk/run.ts";
import { board } from "../../net/sdk/run.ts";
import type { Row, Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Tile } from "../shared/designs/tiles.ts";
import { DESIGNS, ink, NUMBERS, tile } from "../shared/designs/tiles.ts";
import type { Over, State } from "./engine.ts";
import { bearing, DOWN, deal, LEFT, RIGHT, STARVE, step, TICK, toward, UP } from "./engine.ts";
import { pick as choose, STARVED } from "./policy.ts";

/* TYPES */

export type Part = "head" | "body" | "food";

export type Knobs = Record<Part, string> & {
  size: number;
  wrap: number;
  apples: number;
  tick: number;
  play: string;
  tint: string;
};

export type Told = { onScore: number; onOver: Over };

export type Hooks = { [K in keyof Told]?: (arg: Told[K]) => void };

export type Given = Opts & Hooks & { player?: boolean };

export type Snake = ReturnType<typeof make>;

export const PARTS: Part[] = ["head", "body", "food"];

const HIDE = () => false;

export const named = (word: string) => word[0].toUpperCase() + word.slice(1);

export const PIECES: [string, string][] = [
  ["", "Random"],
  ...DESIGNS.map((design): [string, string] => [design, named(design)]),
];

export const SPEC: Spec = [
  { key: "size", label: "Size", kind: "slider", def: 16, min: 8, max: 32, step: 2, group: "Board" },
  { key: "wrap", label: "Wrap", kind: "toggle", def: 1, group: "Board" },
  { key: "apples", label: "Apples", kind: "slider", def: 1, min: 1, max: 10, step: 1, group: "Board" },
  { key: "tick", label: "Tick", kind: "slider", def: 150, min: 50, max: 500, step: 25, unit: "ms", group: "Pace" },
  {
    key: "play",
    label: "Player",
    kind: "segment",
    def: "me",
    options: [
      ["me", "Me"],
      ["smart", "Smart"],
      ["silly", "Silly"],
    ],
    group: "Play",
  },
  ...PARTS.map(
    (part): Row => ({
      key: part,
      label: named(part),
      kind: "pick",
      def: "",
      options: PIECES,
      group: "Pieces",
      when: HIDE,
    }),
  ),
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ARROWS: Record<string, number> = { ArrowUp: UP, ArrowDown: DOWN, ArrowLeft: LEFT, ArrowRight: RIGHT };

const WASD: Record<string, number> = { w: UP, a: LEFT, s: DOWN, d: RIGHT };

const steer = (scene: Handle<Snake>, e: Fired) =>
  scene.turn({ ...ARROWS, ...WASD }[e.key!.length === 1 ? e.key!.toLowerCase() : e.key!]);

export const PAGE = {
  spec: SPEC,
  gestures: true,
  keys: [
    { key: Object.keys(ARROWS), label: "Turn", act: "arrows" },
    { key: "r", label: "New game", act: "reroll" },
    { key: "?", label: "Keys", act: "keys" },
    { key: Object.keys(WASD), label: "Turn", act: "wasd" },
  ],
  actions: { arrows: steer, wasd: steer },
} satisfies Page<Ready & Units, Snake>;

export const SAVERS = [
  {
    id: "solo",
    title: "One apple",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 12 + 2 * Math.floor(rand() * 5),
      wrap: pick(rand, [0, 1]),
      apples: 1,
      tick: 200,
    }),
  },
  {
    id: "feast",
    title: "Many apples",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 20 + 2 * Math.floor(rand() * 4),
      wrap: pick(rand, [0, 1]),
      apples: 5 + Math.floor(rand() * 5),
      tick: 175,
    }),
  },
];

const TAU = Math.PI * 2;
const HOLD = 1000;
const FIT = 0.86;
const DIM = 0.35;
const PREVIEW = 160;
const SLIDE = 0.8;
const POP = 520;
const GULP = 260;
const BORN = 380;
const TRAIL = 1100;
const FALL = 420;
const SPOT = 0.75;
const LAMP = 0.16;
const BODY = 0.66;
const HEAD = 0.42;
const APPLE = 0.36;
const SPARKS = 7;
const WHITE = "#ffffff";
const BLACK = "#000000";
const DX = [0, 1, 0, -1];
const DY = [-1, 0, 1, 0];

type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

type Mark = { cell: number; t: number };

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const snap = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 + 2.4 * (k - 1) ** 3 + 1.4 * (k - 1) ** 2);

const sheet = (w: number, h: number, was: OffscreenCanvas | null) =>
  was && was.width === w && was.height === h ? was : new OffscreenCanvas(w, h);

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const screen = canvas.getContext("2d")!;
  const rand = view.rand;
  const value = tidy(SPEC, opts) as Knobs;
  const play = value.play;
  const auto = play !== "me" || !opts.player;
  const pace = value.tick;
  const offscreen = typeof OffscreenCanvas === "function";
  let state: State;
  let marks: Record<string, Tile>;
  let ended = 0;
  const start = view.t;
  let at = view.t;
  let moved = view.t;
  let gone = false;
  let left = -1;
  let trail: Mark[] = [];
  let pops: Mark[] = [];
  let born: Mark[] = [];
  let floor: OffscreenCanvas | null = null;
  let floored = "";
  let scales: OffscreenCanvas | null = null;
  let scaled: State | null = null;
  let keyed = "";
  const tell = <K extends keyof Told>(name: K, arg: Told[K]) => !gone && (opts as Hooks)[name]?.(arg);
  const begin = () => {
    state = deal(value, view);
    marks = Object.fromEntries(
      PARTS.map((part) => {
        const design = pick(rand, DESIGNS);
        return [part, tile(value[part] || design, pick(rand, NUMBERS))];
      }),
    );
    ended = 0;
    left = -1;
    trail = [];
    pops = [];
    born = state.foods.map((cell) => ({ cell, t: at }));
    moved = at;
    tell("onScore", state.score);
    tell("onOver", state.over);
  };
  const advance = () => {
    if (state.over) {
      if (auto && at - ended >= HOLD) begin();
      return;
    }
    const was = state;
    if (auto && !state.queue.length) {
      const dir = choose(state, play, rand);
      state = step(state, dir === STARVED ? STARVE : toward(dir));
    }
    state = step(state, TICK);
    if (state.snake !== was.snake) {
      moved = at;
      const tail = was.snake[was.snake.length - 1];
      left = state.snake.length === was.snake.length ? tail : -1;
      if (left >= 0) trail = [...trail.filter((one) => at - one.t < TRAIL), { cell: left, t: at }];
      const fresh = state.foods.filter((cell) => !was.foods.includes(cell));
      if (fresh.length) born = [...born.filter((one) => at - one.t < BORN), ...fresh.map((cell) => ({ cell, t: at }))];
    }
    if (state.score !== was.score) {
      tell("onScore", state.score);
      pops = [...pops.filter((one) => at - one.t < POP), { cell: state.snake[0], t: at }];
    }
    if (!state.over) return;
    ended = at;
    tell("onOver", state.over);
  };
  const paint = () => {
    const n = state.size;
    const { x: x0, y: y0, w: side, cell: c } = board(view, n, n, FIT);
    const look = view.look();
    const { paper, accent } = look;
    const dark = !bright(paper);
    const fore = look.ink ?? (dark ? WHITE : BLACK);
    const tone = dark ? accent : blend(accent, BLACK, 0.08);
    const red = dark ? palette.red : blend(palette.red, BLACK, 0.06);
    const t = view.t;
    const quiet = view.still && t === start;
    const center = (cell: number): [number, number] => [
      x0 + ((cell % n) + 0.5) * c,
      y0 + (Math.floor(cell / n) + 0.5) * c,
    ];
    const span = (pen: Pen, a: number, b: number, f0: number, f1: number, dx = 0, dy = 0) => {
      const d = bearing(n, a, b) ?? 0;
      const [ax, ay] = center(a);
      const [bx, by] = center(b);
      const ux = DX[d] * c;
      const uy = DY[d] * c;
      pen.moveTo(ax + ux * f0 + dx, ay + uy * f0 + dy);
      pen.lineTo(ax + ux * f1 + dx, ay + uy * f1 + dy);
      if (Math.abs(bx - ax) + Math.abs(by - ay) < c * 1.5) return;
      pen.moveTo(bx - ux * (1 - f0) + dx, by - uy * (1 - f0) + dy);
      pen.lineTo(bx - ux * (1 - f1) + dx, by - uy * (1 - f1) + dy);
    };
    const ground = (pen: Pen) => {
      const base = blend(paper, accent, dark ? 0.06 : 0.05);
      pen.fillStyle = base;
      pen.fillRect(0, 0, view.w, view.h);
      const big = Math.max(1, Math.max(view.w, view.h) * SPOT);
      const lamp = pen.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, big);
      lamp.addColorStop(0, veil(accent, dark ? LAMP : LAMP / 2));
      lamp.addColorStop(1, veil(accent, 0));
      pen.fillStyle = lamp;
      pen.fillRect(0, 0, view.w, view.h);
      const pad = Math.max(0, Math.min(c * 0.7 + side * 0.02, Math.min(x0, y0) * 0.75));
      pen.save();
      pen.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.16})`;
      pen.shadowBlur = Math.max(1, pad * 1.2);
      pen.shadowOffsetY = pad * 0.2;
      pen.beginPath();
      pen.roundRect(x0 - pad, y0 - pad, side + 2 * pad, side + 2 * pad, Math.max(0, pad * 0.7));
      pen.fillStyle = dark ? blend(blend(paper, BLACK, 0.4), accent, 0.08) : blend(WHITE, accent, 0.05);
      pen.fill();
      pen.shadowColor = "transparent";
      pen.strokeStyle = veil(dark ? WHITE : fore, dark ? 0.07 : 0.08);
      pen.lineWidth = Math.max(0.5, c * 0.06);
      pen.stroke();
      pen.restore();
      pen.beginPath();
      for (let y = 0; y < n; y++) for (let x = (y & 1) ^ 1; x < n; x += 2) pen.rect(x0 + x * c, y0 + y * c, c, c);
      pen.fillStyle = veil(fore, dark ? 0.04 : 0.035);
      pen.fill();
      pen.save();
      pen.lineWidth = Math.max(1, c * 0.06);
      pen.strokeStyle = veil(fore, state.wrap ? 0.12 : 0.28);
      if (state.wrap) pen.setLineDash([Math.max(1, c * 0.3), Math.max(1, c * 0.3)]);
      pen.strokeRect(x0, y0, side, side);
      pen.restore();
    };
    const scaly = (pen: Pen) => {
      pen.fillStyle = veil(blend(tone, BLACK, 0.4), dark ? 0.5 : 0.4);
      const s = Math.round(c * 0.4);
      for (const cell of state.snake.slice(1)) {
        const [x, y] = center(cell);
        ink(pen as CanvasRenderingContext2D, marks.body, Math.round(x - s / 2), Math.round(y - s / 2), s);
      }
    };
    const key = `${view.w}|${view.h}|${paper}|${accent}|${fore}|${n}|${state.wrap}`;
    screen.setTransform(1, 0, 0, 1, 0, 0);
    screen.globalAlpha = 1;
    if (offscreen) {
      const w = Math.max(1, Math.ceil(view.w));
      const h = Math.max(1, Math.ceil(view.h));
      const next = sheet(w, h, floor);
      if (next !== floor || floored !== key) {
        floor = next;
        const pen = floor.getContext("2d")!;
        pen.setTransform(1, 0, 0, 1, 0, 0);
        pen.clearRect(0, 0, w, h);
        ground(pen);
        floored = key;
      }
      screen.drawImage(floor, 0, 0);
    } else ground(screen);
    const ctx = screen;
    const k = quiet ? 1 : Math.min(1, (t - moved) / (pace * SLIDE));
    const e = ease(k);
    const fall = state.over && state.over !== "won" ? ease((t - ended) / FALL) : 0;
    const alpha = 1 - (1 - DIM) * fall;
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    for (const one of trail) {
      const f = (t - one.t) / TRAIL;
      if (f >= 1 || f < 0) continue;
      const [x, y] = center(one.cell);
      ctx.fillStyle = veil(tone, (dark ? 0.26 : 0.2) * (1 - f) * alpha);
      ctx.beginPath();
      ctx.arc(x, y, Math.max(0, c * (0.08 + 0.2 * (1 - f))), 0, TAU);
      ctx.fill();
    }
    ctx.globalAlpha = alpha;
    for (const cell of state.foods) {
      const b = born.find((one) => one.cell === cell);
      const grow = b && !quiet ? snap((t - b.t) / BORN) : 1;
      const [x, y] = center(cell);
      const bob = quiet ? 0 : Math.sin(t / 380 + cell) * c * 0.03;
      const r = Math.max(0, c * APPLE * grow);
      ctx.fillStyle = `rgba(0, 0, 0, ${dark ? 0.4 : 0.16})`;
      ctx.beginPath();
      ctx.ellipse(x + c * 0.04, y + c * 0.3, r * 0.85, r * 0.3, 0, 0, TAU);
      ctx.fill();
      const ay = y + bob;
      ctx.fillStyle = red;
      ctx.beginPath();
      ctx.arc(x, ay, r, 0, TAU);
      ctx.fill();
      if (c >= 10) {
        ctx.save();
        ctx.clip();
        ctx.fillStyle = veil(blend(red, BLACK, 0.45), 0.35);
        const s = Math.round(r * 1.3);
        ink(ctx, marks.food, Math.round(x - s / 2), Math.round(ay - s / 2), s);
        ctx.restore();
        ctx.strokeStyle = blend(palette.brown, BLACK, 0.2);
        ctx.lineWidth = Math.max(1, c * 0.06);
        ctx.beginPath();
        ctx.moveTo(x, ay - r * 0.8);
        ctx.lineTo(x + r * 0.15, ay - r * 1.2);
        ctx.stroke();
        ctx.fillStyle = palette.green;
        ctx.beginPath();
        ctx.ellipse(x + r * 0.42, ay - r * 1.05, r * 0.32, r * 0.15, -0.5, 0, TAU);
        ctx.fill();
      }
      ctx.fillStyle = veil(WHITE, dark ? 0.4 : 0.5);
      ctx.beginPath();
      ctx.arc(x - r * 0.35, ay - r * 0.35, r * 0.28, 0, TAU);
      ctx.fill();
    }
    const body = state.snake;
    const head = body[0];
    const neck = body[1] ?? head;
    const last = body.length - 1;
    const wide = Math.max(1, c * BODY);
    const lift = c * 0.06;
    const drop = c * 0.12;
    const tube = (pen: Pen, dx: number, dy: number) => {
      pen.beginPath();
      for (let i = 1; i < last; i++) span(pen, body[i], body[i + 1], 0, 1, dx, dy);
      if (last >= 1) span(pen, neck, head, 0, Math.max(0.02, e), dx, dy);
      if (left >= 0 && k < 1) span(pen, left, body[last], e, 1, dx, dy);
    };
    ctx.save();
    if (state.wrap) {
      ctx.beginPath();
      ctx.rect(x0, y0, side, side);
      ctx.clip();
    }
    ctx.lineWidth = wide;
    ctx.strokeStyle = `rgba(0, 0, 0, ${dark ? 0.42 : 0.16})`;
    tube(ctx, lift, drop);
    ctx.stroke();
    ctx.strokeStyle = tone;
    tube(ctx, 0, 0);
    ctx.stroke();
    if (c >= 8) {
      if (offscreen) {
        const w = Math.max(1, Math.ceil(view.w));
        const h = Math.max(1, Math.ceil(view.h));
        const next = sheet(w, h, scales);
        if (next !== scales || scaled !== state || keyed !== key) {
          scales = next;
          const pen = scales.getContext("2d")!;
          pen.setTransform(1, 0, 0, 1, 0, 0);
          pen.clearRect(0, 0, w, h);
          scaly(pen);
          scaled = state;
          keyed = key;
        }
        ctx.drawImage(scales, 0, 0);
      } else scaly(ctx);
    }
    ctx.lineWidth = Math.max(1, wide * 0.26);
    ctx.strokeStyle = veil(WHITE, dark ? 0.3 : 0.38);
    tube(ctx, -wide * 0.14, -wide * 0.17);
    ctx.stroke();
    ctx.restore();
    const d = bearing(n, neck, head) ?? state.dir;
    const ux = DX[d];
    const uy = DY[d];
    const [nx, ny] = center(neck);
    const [hx0, hy0] = center(head);
    const jump = Math.abs(hx0 - nx) + Math.abs(hy0 - ny) > c * 1.5;
    const hx = last >= 1 && !jump ? nx + ux * c * e : hx0;
    const hy = last >= 1 && !jump ? ny + uy * c * e : hy0;
    const gulp = pops.length && !quiet ? Math.max(0, 1 - (t - pops[pops.length - 1].t) / GULP) : 0;
    const hr = Math.max(0, c * HEAD * (1 + 0.2 * Math.sin(Math.PI * gulp)));
    ctx.fillStyle = `rgba(0, 0, 0, ${dark ? 0.42 : 0.16})`;
    ctx.beginPath();
    ctx.arc(hx + lift, hy + drop, hr, 0, TAU);
    ctx.fill();
    ctx.fillStyle = blend(tone, WHITE, dark ? 0.12 : 0.08);
    ctx.beginPath();
    ctx.arc(hx, hy, hr, 0, TAU);
    ctx.fill();
    if (c >= 10) {
      ctx.save();
      ctx.clip();
      ctx.fillStyle = veil(blend(tone, BLACK, 0.4), 0.4);
      const s = Math.round(hr * 1.2);
      ink(ctx, marks.head, Math.round(hx - s / 2 - ux * hr * 0.3), Math.round(hy - s / 2 - uy * hr * 0.3), s);
      ctx.restore();
    }
    ctx.fillStyle = veil(WHITE, dark ? 0.35 : 0.45);
    ctx.beginPath();
    ctx.arc(hx - hr * 0.35, hy - hr * 0.4, hr * 0.3, 0, TAU);
    ctx.fill();
    if (c >= 6) {
      const shut = state.over === "wall" || state.over === "bite";
      for (const side of [-1, 1]) {
        const ex = hx + ux * hr * 0.3 - uy * side * hr * 0.45;
        const ey = hy + uy * hr * 0.3 + ux * side * hr * 0.45;
        const er = hr * 0.27;
        ctx.fillStyle = WHITE;
        ctx.beginPath();
        ctx.arc(ex, ey, er, 0, TAU);
        ctx.fill();
        ctx.fillStyle = "#101014";
        ctx.beginPath();
        if (shut) ctx.rect(ex - er * 0.8, ey - er * 0.2, er * 1.6, er * 0.4);
        else ctx.arc(ex + ux * er * 0.35, ey + uy * er * 0.35, er * 0.55, 0, TAU);
        ctx.fill();
      }
    }
    ctx.globalAlpha = 1;
    if (!quiet)
      for (const one of pops) {
        const f = (t - one.t) / POP;
        if (f >= 1 || f < 0) continue;
        const [x, y] = center(one.cell);
        const g = ease(f);
        ctx.strokeStyle = veil(red, 0.8 * (1 - f));
        ctx.lineWidth = Math.max(0.5, c * 0.12 * (1 - f));
        ctx.beginPath();
        ctx.arc(x, y, c * (0.4 + 1.1 * g), 0, TAU);
        ctx.stroke();
        ctx.fillStyle = veil(blend(red, WHITE, 0.3), 1 - f);
        ctx.beginPath();
        for (let i = 0; i < SPARKS; i++) {
          const a = (i / SPARKS) * TAU + one.cell;
          const px = x + Math.cos(a) * c * (0.3 + 1.3 * g);
          const py = y + Math.sin(a) * c * (0.3 + 1.3 * g);
          const pr = Math.max(0, c * 0.1 * (1 - f));
          ctx.moveTo(px + pr, py);
          ctx.arc(px, py, pr, 0, TAU);
        }
        ctx.fill();
      }
    if (state.over && !quiet) {
      const f = (t - ended) / (FALL * 2);
      if (f >= 0 && f < 1) {
        const hit = state.over === "won" ? tone : red;
        ctx.strokeStyle = veil(hit, 0.9 * (1 - f));
        ctx.lineWidth = Math.max(0.5, c * 0.16 * (1 - f));
        ctx.beginPath();
        if (state.over === "won") ctx.rect(x0 - c * f, y0 - c * f, side + 2 * c * f, side + 2 * c * f);
        else ctx.arc(hx0, hy0, c * (0.5 + 2 * ease(f)), 0, TAU);
        ctx.stroke();
      }
    }
  };
  const draw = () => {
    if (view.t - at > pace * 8) at = view.t - pace;
    while (view.t - at >= pace) {
      at += pace;
      advance();
    }
    paint();
  };
  const restart = () => {
    at = view.t;
    begin();
    paint();
  };
  const stop = () => {
    gone = true;
  };
  const move = (dir: number) => {
    state = step(state, toward(dir));
    if (!auto) view.wake?.();
  };
  begin();
  if (view.lock && view.still) for (let i = 0; i < PREVIEW && !state!.over; i++) advance();
  paint();
  return { draw, pace, size: paint, theme: paint, turn: move, restart, stop, state: () => state };
}
