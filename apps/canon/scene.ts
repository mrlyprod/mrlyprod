import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick as choose } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import { board } from "../../net/sdk/run.ts";
import type { Option, Spec, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Mrly } from "../shared/designs/engine.ts";
import type { Units as Loaded } from "../shared/units/index.ts";
import { wasm } from "../shared/units/index.ts";
import type { Act, Over, State } from "./engine.ts";
import {
  BASE,
  cap,
  claim,
  DIM,
  deal as dealing,
  echoes,
  fills,
  NUMBERS,
  ONE,
  spot,
  starts,
  step,
  TWO,
  tally,
} from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { number: number; level: number; one: string; two: string; code: string; tint: string };

export type Told = { points: number[]; turn: number; over: Over; moves: number };

export type Given = Opts & { math?: Mrly | null; onTell?: (told: Told) => void };

export type Hands = {
  done: () => boolean;
  again: () => void;
  press: () => void;
  move: (key: string) => void;
  hover: (u: number, v: number) => void;
  tap: (u: number, v: number) => void;
};

export type Held = Scene & Partial<Hands>;

export type Canon = ReturnType<typeof make>;

type Pen = CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D;

const HIDE = () => false;

const PLAYERS: Option[] = [
  ["me", "Me"],
  ["smart", "Smart"],
  ["greedy", "Greedy"],
  ["random", "Random"],
];

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
  { key: "one", label: "Squares", kind: "segment", def: "me", options: PLAYERS, group: "Players" },
  { key: "two", label: "Dots", kind: "segment", def: "smart", options: PLAYERS, group: "Players" },
  { key: "code", label: "Code", kind: "text", def: "", group: "Design", when: HIDE },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const units = wasm("math");

const ABOUT = [
  "A design is a code: the cells of a small square it fills. Stamped into every filled cell of itself, level by level, it grows into a fractal, and every cell has a path: the filled cell it sits in at each scale, from the whole down.",
  "Two players take turns to claim a free cell. A claim echoes: the same path read from another scale is claimed too, so one tap lights a constellation across the board, like the voices of a canon entering one after another.",
  "Every block at every scale goes to whoever holds more of its parts; a tie goes to whoever holds the part that sits where the block itself sits. A block is worth its cells, so every scale weighs the same. When the board is full, the higher score wins.",
  "Squares and dots can each be you, Smart, Greedy or Random; a coin picks who opens. Tap or click a cell to claim it, or move with the arrows and claim with Enter. Reroll tosses the coin again, and the design chip picks another fractal. [Designs](/designs/) grows any code.",
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
  name: (value: Value) => `canon-${value.code || "triangle"}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New game" : "Claim"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: ARROWS, label: "Move", act: "move" },
    { key: "r", label: "New game", act: "reroll" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.press?.()),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Loaded<"math">, Held>;

const CODES = ["473", "95", "123", "179", "409"];

const FIVES = ["179", "409", "79"];

const pair = (rand: Rng, a: string, b: string) => (rand() < 0.5 ? { one: a, two: b } : { one: b, two: a });

export const SAVERS = [
  {
    id: "duet",
    title: "Duet",
    still: 8000,
    pick: (rand: Rng) => ({ number: 3, level: 3, ...pair(rand, "smart", "smart"), code: choose(rand, CODES) }),
  },
  {
    id: "fugue",
    title: "Fugue",
    still: 8000,
    pick: (rand: Rng) => ({ number: 3, level: 4, ...pair(rand, "smart", "greedy"), code: choose(rand, FIVES) }),
  },
];

const FIT = 0.88;
const GAP = 0.14;
const ROUND = 0.26;
const FREE = 0.1;
const OWN = 0.09;
const GROUND = 0.04;
const WEB = 0.14;
const CROWD = 216;
const THREAD = 0.85;
const HINT = 0.5;
const WIDTH = 0.08;
const LINE = 1.5;
const PACE = 520;
const SPIN = 300;
const HOLD = 3200;
const BLOOM = 1400;
const GLOW = 0.22;
const TAIL = 100;
const SLOW = 820;
const TURN = 640;
const REST = 6400;
const FADE = 1400;
const LEAVE = 900;
const VOICE = 90;
const RIPPLE = 760;
const BED = 0.9;
const SPOT = 0.75;
const LAMP = 0.16;
const BREATH = 3600;
const BEAD = 2400;
const WHITE = "#ffffff";
const BLACK = "#000000";
const DEAL: (keyof Knobs)[] = ["number", "level", "code"];

const ease = (k: number) => (k >= 1 ? 1 : 1 - (1 - k) ** 3);

const snap = (k: number) => (k >= 1 ? 1 : 1 + 2.4 * (k - 1) ** 3 + 1.4 * (k - 1) ** 2);

const fade = (k: number) => ease(Math.max(0, k));

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("canon: the scene wants @mrlyprod/math as opts.math");
  const screen = canvas.getContext("2d")!;
  let ctx: Pen = screen;
  let sheet: OffscreenCanvas | null = null;
  let drawn: State | null = null;
  let keyed = "";
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let side = 1;
  let spots = new Int32Array(0);
  let index = new Int32Array(0);
  let cursor = -1;
  let shown = false;
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
  const player = (who: number) => (who === ONE ? value.one : value.two);
  const human = (who: number) => player(who) === "me";
  const tell = () => {
    if (gone) return;
    opts.onTell?.({ points: state.points, turn: state.turn, over: state.over, moves: state.moves });
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return;
    acted = view.t;
    busy = view.t + Math.max(twirl, RIPPLE) + state.level * (lock ? VOICE * 1.8 : VOICE);
    if (state.over) {
      ended = view.t;
      busy = view.t + (lock ? rest : BLOOM);
    }
    tell();
  };
  const settle = () => {
    if (!view.still || paused || ![ONE, TWO].some(human)) return;
    while (!state.over && !human(state.turn)) {
      const act = pick(state, player(state.turn), view.rand);
      if (!act) return;
      apply(act);
    }
  };
  const sketch = () => {
    if (!lock || !view.still || [ONE, TWO].some(human)) return;
    const free = () => state.board.split("0").length - 1;
    while (!state.over && 2 * free() > state.board.length) {
      const act = pick(state, player(state.turn), view.rand);
      if (!act) return;
      apply(act);
    }
  };
  const deal = () => {
    state = dealing(value, { seed, round, units: { math } });
    side = value.number ** value.level;
    spots = new Int32Array(state.board.length * 2);
    index = new Int32Array(side * side).fill(-1);
    for (let cell = 0; cell < state.board.length; cell++) {
      const [x, y] = spot(state, cell);
      spots.set([x, y], cell * 2);
      index[y * side + x] = cell;
    }
    cursor = state.board.length ? 0 : -1;
    shown = false;
    acted = view.t;
    ended = view.t;
    busy = view.t;
    tell();
    settle();
    sketch();
  };
  const layout = () => board(view, side, side, FIT);
  const center = (cell: number, left: number, top: number, c: number): [number, number] => [
    left + (spots[cell * 2] + 0.5) * c,
    top + (spots[cell * 2 + 1] + 0.5) * c,
  ];
  const mark = (who: number, x: number, y: number, size: number) => {
    if (who === TWO) {
      ctx.moveTo(x + size / 2, y);
      ctx.arc(x, y, size / 2, 0, 2 * Math.PI);
    } else ctx.roundRect(x - size / 2, y - size / 2, size, size, size * ROUND);
  };
  const thread = (cells: number[], left: number, top: number, c: number, closed = true) => {
    if (cells.length < 2) return;
    ctx.beginPath();
    cells.forEach((cell, i) => {
      const [x, y] = center(cell, left, top, c);
      if (i) ctx.lineTo(x, y);
      else ctx.moveTo(x, y);
    });
    if (cells.length > 2 && closed) ctx.closePath();
    ctx.stroke();
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const after = (from: number, span: number) => Math.max(0, Math.min(1, (view.t - from) / span));
  const paint = () => {
    const { x: left, y: top, w: span, cell: c } = layout();
    const { paper, accent, ink } = view.look();
    const dark = !bright(paper);
    const one = dark ? palette.orange : blend(palette.orange, BLACK, 0.12);
    const two = dark ? blend(palette.pink, WHITE, 0.08) : blend(palette.pink, BLACK, 0.04);
    const tone = (who: number) => (who === ONE ? one : two);
    const shine = (who: number) => blend(tone(who), WHITE, dark ? 0.45 : 0.3);
    const f = fills(state.motif).length;
    const level = state.level;
    const count = state.board.length;
    const tile = (from: number, many: number) => {
      for (let cell = from; cell < from + many; cell++)
        ctx.rect(left + spots[cell * 2] * c, top + spots[cell * 2 + 1] * c, c, c);
    };
    const ground = blend(paper, accent, dark ? 0.06 : 0.05);
    const won = state.over === "one" ? ONE : state.over === "two" ? TWO : 0;
    const done = won ? fade(after(ended, BLOOM)) : 0;
    const since = Math.max(0, view.t - ended);
    const breath = done > 0 && !view.still ? 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH) : 0;
    const pad = Math.max(0, Math.min(c * BED + span * 0.02, Math.min(left, top) * 0.75));
    const lastly = state.last >= 0 ? echoes(state, state.last) : [];
    const mover = state.turn === ONE ? TWO : ONE;
    const fine = c < 10;
    const size = c * (1 - 2 * (fine ? GAP / 3 : GAP));
    const pieces = (who: number, cells: number[], scaled: (cell: number) => number, dx = 0, dy = 0) => {
      ctx.beginPath();
      for (const cell of cells) {
        if (state.board.charCodeAt(cell) - 48 !== who) continue;
        const [x, y] = center(cell, left, top, c);
        mark(who, x + dx, y + dy, Math.max(0, size * scaled(cell)));
      }
    };
    const gloss = (who: number, cells: number[], scaled: (cell: number) => number) => {
      if (fine) return;
      ctx.beginPath();
      for (const cell of cells) {
        if (state.board.charCodeAt(cell) - 48 !== who) continue;
        const [x, y] = center(cell, left, top, c);
        const s = Math.max(0, size * scaled(cell));
        mark(who, x - s * 0.12, y - s * 0.14, s * 0.42);
      }
      ctx.fillStyle = veil(WHITE, dark ? 0.32 : 0.38);
      ctx.fill();
    };
    const still = () => {
      const at = starts(f, level);
      const { owners, points } = tally(state);
      const floor = dark ? blend(blend(paper, BLACK, 0.4), accent, 0.08) : blend(WHITE, accent, 0.05);
      const big = Math.max(1, Math.max(view.w, view.h) * SPOT);
      const lit = won ? tone(won) : accent;
      const lamp = ctx.createRadialGradient(view.w / 2, view.h / 2, 0, view.w / 2, view.h / 2, big);
      lamp.addColorStop(0, veil(lit, (dark ? LAMP : LAMP / 2) + (won ? 0.08 : 0)));
      lamp.addColorStop(1, veil(lit, 0));
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
      ctx.lineWidth = Math.max(0.5, c * 0.06);
      ctx.stroke();
      ctx.restore();
      ctx.fillStyle = veil(ink, dark ? GROUND * 1.4 : GROUND);
      ctx.beginPath();
      tile(0, count);
      ctx.fill();
      for (let d = 0; d < level; d++) {
        const many = f ** (level - d);
        for (const who of [ONE, TWO]) {
          ctx.beginPath();
          for (let p = 0; p < at[d + 1] - at[d]; p++) if (owners[at[d] + p] === who) tile(p * many, many);
          ctx.fillStyle = veil(tone(who), (dark ? OWN * 1.4 : OWN) * (fine ? 1.6 : 1));
          ctx.fill();
        }
      }
      ctx.lineWidth = Math.max(1, c * WIDTH);
      ctx.lineJoin = "round";
      ctx.lineCap = "round";
      const thin = Math.min(1, CROWD / Math.max(1, count));
      for (const who of [ONE, TWO]) {
        ctx.strokeStyle = veil(tone(who), WEB * thin * (dark ? 1.2 : 0.7));
        for (let cell = 0; cell < count; cell++) {
          if (state.board.charCodeAt(cell) - 48 !== who) continue;
          const echo = echoes(state, cell);
          if (Math.min(...echo) === cell && !echo.includes(state.last)) thread(echo, left, top, c);
        }
      }
      ctx.beginPath();
      for (let cell = 0; cell < count; cell++) {
        if (state.board[cell] !== "0") continue;
        const [x, y] = center(cell, left, top, c);
        mark(ONE, x, y, size * (fine ? 0.9 : 0.72));
      }
      ctx.fillStyle = veil(blend(ink, accent, 0.25), dark ? FREE * 1.4 : FREE * 1.2);
      ctx.fill();
      const whole = () => 1;
      const calm = Array.from({ length: count }, (_, cell) => cell).filter((cell) => !lastly.includes(cell));
      for (const who of [ONE, TWO]) {
        ctx.save();
        ctx.shadowColor = `rgba(0, 0, 0, ${dark ? 0.55 : 0.22})`;
        ctx.shadowBlur = Math.max(0, c * 0.25);
        ctx.shadowOffsetY = c * 0.06;
        if (won === who) {
          ctx.shadowColor = shine(who);
          ctx.shadowBlur = Math.max(1, c * 0.9);
          ctx.shadowOffsetY = 0;
        }
        pieces(who, calm, whole);
        ctx.fillStyle = tone(who);
        ctx.fill();
        ctx.restore();
        gloss(who, calm, whole);
      }
      const bar = Math.max(2, Math.min(pad * 0.3, span * 0.014));
      const by = top + span + pad * 0.5 - bar / 2;
      const all = (level + 1) * count;
      if (by + bar > view.h || all <= 0) return;
      const a = (points[0] / all) * span;
      const b = (points[1] / all) * span;
      ctx.beginPath();
      ctx.roundRect(left, by, span, bar, bar / 2);
      ctx.fillStyle = veil(ink, dark ? 0.12 : 0.08);
      ctx.fill();
      if (a > 0) {
        ctx.beginPath();
        ctx.roundRect(left, by, a, bar, Math.min(bar, a) / 2);
        ctx.fillStyle = one;
        ctx.fill();
      }
      if (b > 0) {
        ctx.beginPath();
        ctx.roundRect(left + span - b, by, b, bar, Math.min(bar, b) / 2);
        ctx.fillStyle = two;
        ctx.fill();
      }
      if (points[0] + points[1] > 0) {
        ctx.fillStyle = veil(ink, 0.5);
        ctx.fillRect(left + span / 2 - 0.5, by - bar * 0.5, 1, bar * 2);
      }
    };
    screen.setTransform(1, 0, 0, 1, 0, 0);
    screen.globalAlpha = 1;
    screen.fillStyle = ground;
    screen.fillRect(0, 0, view.w, view.h);
    const alpha = lock ? shade() : 1;
    const key = `${view.w}|${view.h}|${paper}|${accent}|${ink}`;
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
    if (done > 0) {
      ctx.fillStyle = veil(tone(won), GLOW * (1 - 0.6 * done) + 0.06 * breath);
      ctx.beginPath();
      tile(0, count);
      ctx.fill();
    }
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    const voice = lock ? VOICE * 1.8 : VOICE;
    const enter = (i: number) => (view.still ? 1 : after(acted + i * voice, twirl));
    const grown = lastly.map((_, i) => snap(enter(i)));
    if (lastly.length > 1) {
      const reach = lastly.filter((_, i) => view.still || view.t >= acted + i * voice).length;
      const links = lastly.slice(0, Math.max(1, reach));
      const closed = links.length === lastly.length;
      ctx.strokeStyle = veil(tone(mover), dark ? 0.22 : 0.16);
      ctx.lineWidth = Math.max(2, c * WIDTH * 5);
      thread(links, left, top, c, closed);
      ctx.strokeStyle = veil(tone(mover), THREAD);
      ctx.lineWidth = Math.max(1, c * WIDTH * 1.6);
      thread(links, left, top, c, closed);
    }
    const aimed = shown && cursor >= 0 && !state.over && human(state.turn);
    const hinted = aimed && state.board[cursor] === "0" ? echoes(state, cursor) : [];
    if (hinted.length) {
      ctx.save();
      ctx.lineWidth = Math.max(1, c * WIDTH);
      ctx.setLineDash([Math.max(1, c * 0.25), Math.max(1, c * 0.25)]);
      ctx.strokeStyle = veil(tone(state.turn), HINT);
      thread(hinted, left, top, c);
      ctx.restore();
    }
    const scaled = (cell: number) => {
      const i = lastly.indexOf(cell);
      return i < 0 ? 1 : grown[i];
    };
    for (const who of [ONE, TWO]) {
      if (!lastly.some((cell) => state.board.charCodeAt(cell) - 48 === who)) continue;
      pieces(who, lastly, scaled, 0, c * 0.08);
      ctx.fillStyle = `rgba(0, 0, 0, ${dark ? 0.45 : 0.18})`;
      ctx.fill();
      pieces(who, lastly, scaled);
      ctx.fillStyle = tone(who);
      ctx.fill();
      gloss(who, lastly, scaled);
    }
    if (!view.still)
      lastly.forEach((cell, i) => {
        const k = after(acted + i * voice, RIPPLE);
        if (k <= 0 || k >= 1) return;
        const [x, y] = center(cell, left, top, c);
        ctx.save();
        ctx.globalAlpha *= (1 - k) * 0.9;
        ctx.strokeStyle = shine(mover);
        ctx.lineWidth = Math.max(1, c * 0.14 * (1 - k));
        ctx.beginPath();
        mark(mover, x, y, c * (0.9 + 2.2 * ease(k)));
        ctx.stroke();
        ctx.restore();
      });
    if (lock && !view.still && lastly.length > 1 && !state.over) {
      const loop = lastly.map((cell) => center(cell, left, top, c));
      const s = ((view.t - acted) / BEAD) % 1;
      const lengths = loop.map((p, i) => {
        const q = loop[(i + 1) % loop.length];
        return Math.hypot(q[0] - p[0], q[1] - p[1]);
      });
      const total = lengths.reduce((a, b) => a + b, 0);
      let along = s * total;
      let i = 0;
      while (i < lengths.length - 1 && along > lengths[i]) along -= lengths[i++];
      const k = lengths[i] > 0 ? along / lengths[i] : 0;
      const [ax, ay] = loop[i];
      const [bx, by] = loop[(i + 1) % loop.length];
      const bead = Math.max(0.5, c * 0.14);
      ctx.fillStyle = veil(shine(mover), 0.25);
      ctx.beginPath();
      ctx.arc(ax + (bx - ax) * k, ay + (by - ay) * k, bead * 3, 0, 2 * Math.PI);
      ctx.fill();
      ctx.fillStyle = shine(mover);
      ctx.beginPath();
      ctx.arc(ax + (bx - ax) * k, ay + (by - ay) * k, bead, 0, 2 * Math.PI);
      ctx.fill();
    }
    if (!aimed) return;
    ctx.lineWidth = Math.max(1, Math.round(LINE * view.dpr));
    ctx.strokeStyle = tone(state.turn);
    ctx.beginPath();
    for (const cell of hinted.length ? hinted : [cursor]) {
      const [x, y] = center(cell, left, top, c);
      mark(hinted.length ? state.turn : ONE, x, y, hinted.length ? size : c);
    }
    ctx.stroke();
  };
  const advance = () => {
    if (paused || held < Infinity) return;
    if (state.over) {
      if ([ONE, TWO].some(human) || view.t - ended < rest) return;
      round++;
      deal();
      if (lock) {
        rise = view.t;
        busy = view.t + FADE;
      }
      return;
    }
    if (human(state.turn) || view.t - acted < pace) return;
    const act = pick(state, player(state.turn), view.rand);
    if (act) apply(act);
  };
  const draw = () => {
    if (view.t !== last) advance();
    last = view.t;
    if (view.t <= busy + TAIL || (lock && !view.still)) paint();
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
  const cellAt = (u: number, v: number) => {
    const { x: left, y: top, cell: c } = layout();
    const x = Math.floor((u * view.w - left) / c);
    const y = Math.floor((v * view.h - top) / c);
    return x < 0 || y < 0 || x >= side || y >= side ? -1 : index[y * side + x];
  };
  const press = () => {
    if (state.over || cursor < 0 || !human(state.turn)) return;
    shown = true;
    apply(claim(cursor));
    settle();
    paint();
  };
  const hover = (u: number, v: number) => {
    const cell = cellAt(u, v);
    if (cell < 0 || (cell === cursor && shown)) return;
    cursor = cell;
    shown = true;
    paint();
  };
  const tap = (u: number, v: number) => {
    const cell = cellAt(u, v);
    if (cell < 0) return;
    cursor = cell;
    press();
  };
  const move = (key: string) => {
    const way = STEPS[key];
    if (!way || cursor < 0) return;
    const bx = spots[cursor * 2];
    const by = spots[cursor * 2 + 1];
    let best = -1;
    let low = Infinity;
    for (let cell = 0; cell < state.board.length; cell++) {
      const dx = spots[cell * 2] - bx;
      const dy = spots[cell * 2 + 1] - by;
      const along = dx * way[0] + dy * way[1];
      if (along <= 0) continue;
      const score = along + 2 * Math.abs(dx * way[1] + dy * way[0]);
      if (score < low) {
        low = score;
        best = cell;
      }
    }
    shown = true;
    if (best >= 0) cursor = best;
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
    } else settle();
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
      settle();
    },
    playing: () => !paused,
    hold,
    windDown,
    done: () => state.over !== null,
    press,
    move,
    hover,
    tap,
    ring: () => cursor,
    state: () => state,
  };
}
