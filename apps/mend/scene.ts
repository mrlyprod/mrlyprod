import { TINTS, veil } from "../../net/sdk/color.ts";
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

const FIT = 0.9;
const GROUND = 0.07;
const GHOST = 0.2;
const RING = 0.12;
const LINE = 2;
const PAD = 0.12;
const CORNER = 0.18;
const PACE = 420;
const SPIN = 220;
const HOLD = 2400;
const BLOOM = 900;
const TAIL = 100;
const GLOW = 0.16;
const SLOW = 1100;
const TURN = 640;
const REST = 5200;
const FADE = 1200;
const LEAVE = 900;
const DEAL: (keyof Knobs)[] = ["number", "level", "mess", "code"];

const ease = (k: number) => (k >= 1 ? 1 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("mend: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let laid: Laid;
  let goal: Uint8Array;
  let focus = 0;
  let cursor = -1;
  let spun: { block: number; way: number; at: number } | null = null;
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
    acted = view.t;
    ended = view.t;
    busy = view.t;
    tell();
  };
  const layout = () => board(view, laid.side, laid.side, FIT);
  const box = (block: number) => laid.boxes.subarray(block * 3, block * 3 + 3);
  const fill = (
    cells: Uint8Array,
    left: number,
    top: number,
    cell: number,
    keep: (x: number, y: number) => boolean,
  ) => {
    const side = laid.side;
    ctx.beginPath();
    for (let y = 0; y < side; y++) {
      let x = 0;
      while (x < side) {
        if (!cells[y * side + x] || !keep(x, y)) {
          x++;
          continue;
        }
        const from = x;
        while (x < side && cells[y * side + x] && keep(x, y)) x++;
        ctx.rect(left + from * cell, top + y * cell, (x - from) * cell, cell);
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
  const paint = () => {
    const { x: left, y: top, w: span, cell } = layout();
    const { accent } = view.look();
    const k = spun ? ease((view.t - spun.at) / twirl) : 1;
    if (k >= 1) spun = null;
    const turning = spun ? box(spun.block) : null;
    const inside = (x: number, y: number) =>
      turning !== null &&
      x >= turning[0] &&
      x < turning[0] + turning[2] &&
      y >= turning[1] &&
      y < turning[1] + turning[2];
    const bloom = state.over ? Math.max(0, 1 - (view.t - ended) / BLOOM) : 0;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    ctx.fillStyle = veil(accent, GROUND + GLOW * bloom);
    ctx.fillRect(left, top, span, span);
    if (!state.over) {
      ctx.fillStyle = veil(accent, GHOST);
      fill(goal, left, top, cell, () => true);
    }
    ctx.fillStyle = accent;
    fill(laid.cells, left, top, cell, (x, y) => !inside(x, y));
    if (turning && spun) {
      const half = (turning[2] * cell) / 2;
      const cx = left + turning[0] * cell + half;
      const cy = top + turning[1] * cell + half;
      ctx.save();
      ctx.translate(cx, cy);
      ctx.rotate((-(1 - k) * spun.way * Math.PI) / 2);
      ctx.translate(-cx, -cy);
      fill(laid.cells, left, top, cell, inside);
      ctx.restore();
    }
    if (state.over || cursor < 0) return;
    const [bx, by, size] = box(cursor);
    const pad = Math.max(1, Math.round(cell * Math.min(size, 3) * PAD));
    const x = left + bx * cell - pad;
    const y = top + by * cell - pad;
    const s = size * cell + 2 * pad;
    ctx.beginPath();
    ctx.roundRect(x, y, s, s, Math.min(s * CORNER, cell * 2));
    ctx.fillStyle = veil(accent, RING);
    ctx.fill();
    ctx.lineWidth = Math.max(1, Math.round(LINE * view.dpr));
    ctx.strokeStyle = accent;
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
    if (state.over) {
      ended = view.t;
      busy = view.t + (lock ? rest : BLOOM);
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
