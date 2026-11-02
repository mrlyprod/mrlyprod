import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Act, Over, State } from "./engine.ts";
import { deal as dealing, depth, forks, HIGH, LOW, neat, spin, step } from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { levels: number; play: string; tint: string };

export type Told = { taps: number; par: number; neat: number; of: number; over: Over };

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

export type Twig = ReturnType<typeof make>;

export const SPEC: Spec = [
  { key: "levels", label: "Levels", kind: "slider", def: 4, min: LOW, max: HIGH, step: 1, group: "Tree" },
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
  "A tree forks again and again, and every crown has been spun around, so its blossoms sit out of order. Tap a fork to spin its whole crown around, like a mobile turning on its string.",
  "Spin the crowns until the blossoms grow from small and pale on the left to big and deep on the right. A branch glows once everything above it is in order.",
  "Every crown is a smaller copy of the whole tree, so the tree looks the same at every size, and a fork spun twice is back where it was.",
  "Tap a fork, or move with the arrows and spin with X. Enter asks for a hint. Smart spins from the trunk up, in par spins. Reroll deals another tree.",
];

const KEYS = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  gestures: true,
  name: (value: Value) => `twig-${value.levels}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : "Hint"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: "x", label: "Spin", act: "spin" },
    { key: KEYS, label: "Move", act: "move" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.hint?.()),
    spin: (scene) => scene.press?.(),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "grove",
    title: "Grove",
    pick: (rand: Rng) => ({ play: "smart", levels: 3 + Math.floor(rand() * 2) }),
  },
  {
    id: "canopy",
    title: "Canopy",
    pick: () => ({ play: "smart", levels: 5 }),
  },
];

export const WIDE = 0.9;
export const TALL = 0.84;
const RATIO = 0.8;
const TRUNK = 1.15;
const BARK = 0.055;
const THIN = 0.72;
const BLOOM = 0.36;
const SMALL = 0.55;
const REACH = 0.9;
const BLACK = "#000000";
const WHITE = "#ffffff";
const SWING = 520;
const PACE = 650;
const SLOW = 1200;
const AIM = 300;
const SIGHT = 700;
const GLOW = 1400;
const BREATH = 2800;
const WAVE = 2200;
const SWAY = 5200;
const LEAN = 0.018;
const HOLD = 3600;
const REST = 6400;
const FADE = 1200;
const LEAVE = 900;
const TAIL = 100;

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const up = (i: number) => (i - 1) >> 1;

/* GEOMETRY */

export type Shape = { xs: number[]; ys: number[]; unit: number; bloom: number; ground: number; width: number };

export function shape(levels: number, w: number, h: number): Shape {
  const size = Math.max(0, Math.min(w, h));
  const width = Math.max(0, Math.min(w * WIDE, size * 1.25));
  const tall = Math.max(0, h * TALL);
  const leaves = 2 ** levels;
  const bloom = Math.max(0, Math.min((width / leaves) * BLOOM, size * 0.05));
  const top = (h - tall) / 2 + bloom;
  const ground = (h + tall) / 2;
  const unit = Math.max(0, (ground - top) / (TRUNK + (1 - RATIO ** levels) / (1 - RATIO)));
  const xs: number[] = [];
  const ys: number[] = [];
  let y = ground - unit * TRUNK;
  for (let d = 0; d <= levels; d++) {
    const n = 2 ** d;
    for (let p = 0; p < n; p++) {
      xs.push(w / 2 - width / 2 + ((p + 0.5) * width) / n);
      ys.push(y);
    }
    y -= unit * RATIO ** d;
  }
  return { xs, ys, unit, bloom, ground, width };
}

export function nearest(levels: number, { xs, ys, unit }: Shape, x: number, y: number): number {
  let best = -1;
  let far = unit * REACH;
  for (let i = 0; i < forks(levels); i++) {
    const d = Math.hypot(xs[i] - x, ys[i] - y);
    if (d < far) {
      far = d;
      best = i;
    }
  }
  return best;
}

export function within(node: number, fork: number): boolean {
  if (fork < 0) return false;
  let at = node;
  while (depth(at) > depth(fork)) at = up(at);
  return at === fork;
}

export function mirror(fork: number, node: number): number {
  const d = depth(node) - depth(fork);
  const lo = (fork - (2 ** depth(fork) - 1)) * 2 ** d;
  const p = node - (2 ** depth(node) - 1);
  return 2 ** depth(node) - 1 + lo + (2 ** d - 1 - (p - lo));
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let off: number[] = [];
  let moved: number[] = [];
  let cursor = 0;
  let hovered = -1;
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
  const rest = lock ? REST : HOLD;
  const span = (ms: number) => (view.still ? 0 : ms);
  const after = (at: number, ms: number) => (view.still ? 1 : fade((view.t - at) / ms));
  const count = () => Array.from({ length: forks(state.levels) }, (_, i) => neat(state, i)).filter(Boolean).length;
  const tell = () => {
    if (gone) return;
    opts.onTell?.({ taps: state.taps, par: state.par, neat: count(), of: forks(state.levels), over: state.over });
  };
  const deal = () => {
    state = dealing(value, { seed, round });
    off = Array.from({ length: forks(state.levels) }, () => 0);
    moved = off.map(() => -Infinity);
    queued = null;
    ended = state.over ? view.t : Infinity;
    acted = view.t;
    busy = view.t;
    cursor = 0;
    hovered = -1;
    tell();
  };
  const lag = (i: number) => (view.still ? 0 : off[i] * (1 - ease((view.t - moved[i]) / SWING)));
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const paint = () => {
    const n = state.levels;
    const shape0 = shape(n, view.w, view.h);
    const { xs, ys, unit, bloom, ground: soil, width: crown } = shape0;
    const { accent, paper, ink } = view.look();
    const dark = !bright(paper);
    const ground = dark ? blend(paper, accent, 0.05) : blend(paper, accent, 0.04);
    const floor = dark ? blend(blend(paper, BLACK, 0.35), accent, 0.1) : blend(WHITE, accent, 0.08);
    const glow = dark ? blend(accent, WHITE, 0.2) : blend(accent, BLACK, 0.06);
    const core = dark ? blend(accent, WHITE, 0.8) : blend(accent, WHITE, 0.6);
    const stone = dark ? blend(blend(paper, WHITE, 0.32), accent, 0.08) : blend(blend(ink, WHITE, 0.62), accent, 0.1);
    const pale = dark ? blend(accent, BLACK, 0.3) : blend(accent, WHITE, 0.72);
    const deep = dark ? blend(accent, WHITE, 0.72) : blend(accent, BLACK, 0.3);
    const done = state.over === "done" ? after(ended, GLOW) : 0;
    const since = Math.max(0, view.t - ended);
    const breath = done > 0 && !view.still ? 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH) : 0;
    const sway = done > 0 && !view.still ? Math.sin((2 * Math.PI * since) / SWAY) * LEAN * done : 0;
    const wave = done > 0 && !view.still ? ((since % WAVE) / WAVE) * (n + 3) - 1 : -9;
    const total = xs.length;
    const fs = forks(n);
    const turn: number[] = [];
    const px: number[] = [];
    const py: number[] = [];
    for (let i = 0; i < total; i++) {
      const s = i < fs ? Math.cos(Math.PI * lag(i)) : 1;
      if (i === 0) {
        turn.push(s);
        px.push(xs[0]);
      } else {
        const p = up(i);
        turn.push(i < fs ? turn[p] * s : turn[p]);
        px.push(px[p] + (xs[i] - xs[p]) * turn[p]);
      }
      py.push(ys[i]);
    }
    for (let i = 0; i < total; i++) px[i] += sway * (soil - py[i]);
    const order = Array.from({ length: fs }, (_, i) => neat(state, i));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.fillStyle = ground;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const big = Math.max(view.w, view.h);
    const mid = (soil + ys[total - 1]) / 2;
    for (const [k, a] of [
      [1.2, 0.035],
      [0.85, 0.04],
      [0.6, 0.05],
    ]) {
      ctx.beginPath();
      ctx.arc(view.w / 2, mid, Math.max(0, big * 0.5 * k), 0, 2 * Math.PI);
      ctx.fillStyle = veil(accent, (dark ? a : a * 0.7) * (1 + 0.8 * done * (0.6 + 0.4 * breath)));
      ctx.fill();
    }
    ctx.beginPath();
    ctx.ellipse(view.w / 2, soil, Math.max(0, crown * 0.32), Math.max(0, unit * 0.16), 0, 0, 2 * Math.PI);
    ctx.fillStyle = floor;
    ctx.fill();
    ctx.strokeStyle = veil(dark ? WHITE : ink, 0.08);
    ctx.lineWidth = Math.max(0.5, unit * 0.012);
    ctx.stroke();
    const mark = value.play === "me" && !state.over ? (hovered >= 0 ? hovered : cursor) : -1;
    const aim = queued ? queued.args.fork : -1;
    const wide = (d: number) => Math.max(0.5, unit * BARK * THIN ** d);
    ctx.lineCap = "round";
    const limb = (x0: number, y0: number, x1: number, y1: number) => {
      const h = (y0 - y1) * 0.55;
      ctx.beginPath();
      ctx.moveTo(x0, y0);
      ctx.bezierCurveTo(x0, y0 - h, x1, y1 + h, x1, y1);
    };
    for (let i = 0; i < total; i++) {
      const d = depth(i);
      const x0 = i ? px[up(i)] : xs[0];
      const y0 = i ? py[up(i)] : soil;
      const lit = i === 0 ? order[0] : i < fs ? order[i] : order[up(i)];
      const near = within(i, mark) && i !== mark ? mark : within(i, aim) && i !== aim ? aim : -1;
      const pulse = Math.max(0, 1 - Math.abs(wave - d));
      const w = wide(d) * (i ? 1 : 1.25);
      limb(x0, y0, px[i], py[i]);
      if (lit) {
        ctx.strokeStyle = veil(glow, (dark ? 0.22 : 0.14) + 0.12 * done * breath + 0.3 * pulse * done);
        ctx.lineWidth = w * (i ? 2.4 : 1.8);
        ctx.stroke();
        ctx.strokeStyle = near >= 0 ? blend(glow, ink, 0.15) : glow;
        ctx.lineWidth = w;
        ctx.stroke();
        ctx.strokeStyle = blend(dark ? core : blend(glow, WHITE, 0.3), WHITE, 0.4 * pulse * done);
        ctx.lineWidth = Math.max(0.5, w * (dark ? 0.4 : 0.3));
        ctx.stroke();
      } else {
        ctx.strokeStyle = near >= 0 ? blend(stone, accent, 0.4) : stone;
        ctx.lineWidth = w;
        ctx.stroke();
      }
    }
    for (let i = 0; i < fs; i++) {
      const r = wide(depth(i)) * 0.95;
      if (i === mark || i === aim) {
        ctx.beginPath();
        ctx.arc(px[i], py[i], Math.max(0, r * 3.2), 0, 2 * Math.PI);
        ctx.fillStyle = veil(accent, dark ? 0.3 : 0.2);
        ctx.fill();
      }
      ctx.beginPath();
      ctx.arc(px[i], py[i], Math.max(0, r), 0, 2 * Math.PI);
      ctx.fillStyle = order[i] ? core : stone;
      ctx.fill();
    }
    const leaves = 2 ** n;
    for (let k = 0; k < leaves; k++) {
      const i = fs + k;
      const rank = Number.parseInt(state.leaves[k], 36) / Math.max(1, leaves - 1);
      const pulse = Math.max(0, 1 - Math.abs(wave - n - 0.5));
      const r = bloom * (SMALL + (1 - SMALL) * rank) * (1 + 0.12 * done * breath + 0.2 * pulse * done);
      const fill = blend(pale, deep, rank);
      ctx.beginPath();
      ctx.arc(px[i], py[i], Math.max(0, r * 1.5), 0, 2 * Math.PI);
      ctx.fillStyle = veil(fill, (dark ? 0.16 : 0.12) * (1 + done));
      ctx.fill();
      ctx.beginPath();
      ctx.arc(px[i], py[i], Math.max(0, r), 0, 2 * Math.PI);
      ctx.fillStyle = fill;
      ctx.fill();
      ctx.beginPath();
      ctx.arc(px[i] - r * 0.25, py[i] - r * 0.25, Math.max(0, r * 0.3), 0, 2 * Math.PI);
      ctx.fillStyle = veil(WHITE, dark ? 0.25 : 0.35);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return false;
    acted = view.t;
    queued = null;
    const a = act.args.fork;
    const now = off.map((_, i) => lag(i));
    for (let i = a + 1; i < off.length; i++)
      if (within(i, a)) {
        off[i] = now[mirror(a, i)];
        moved[i] = view.t;
      }
    off[a] = now[a] - 1;
    moved[a] = view.t;
    busy = view.t + span(SWING);
    if (state.over) {
      ended = view.t + span(SWING);
      busy = ended + (value.play === "me" ? GLOW : rest);
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
    if (view.t - acted < span(pace)) return;
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
  const read = (u: number, v: number) =>
    nearest(state.levels, shape(state.levels, view.w, view.h), u * view.w, v * view.h);
  const hover = (u: number, v: number) => {
    if (state.over) return;
    const k = read(u, v);
    if (k === hovered) return;
    hovered = k;
    if (k >= 0) cursor = k;
    paint();
  };
  const tap = (u: number, v: number) => {
    if (state.over) return;
    const k = read(u, v);
    if (k >= 0) {
      cursor = k;
      apply(spin(k));
    }
  };
  const press = () => {
    if (!state.over) apply(spin(cursor));
  };
  const hint = () => {
    if (state.over) return;
    const act = pick(state, "smart", view.rand);
    if (!act) return;
    cursor = act.args.fork;
    apply(act);
  };
  const move = (key: string) => {
    if (state.over) return;
    const fs = forks(state.levels);
    const d = depth(cursor);
    const k =
      key === "ArrowUp"
        ? 2 * cursor + 1
        : key === "ArrowDown"
          ? up(cursor)
          : key === "ArrowLeft"
            ? cursor - 1
            : key === "ArrowRight"
              ? cursor + 1
              : cursor;
    if (k < 0 || k >= fs || k === cursor) return;
    if ((key === "ArrowLeft" || key === "ArrowRight") && depth(k) !== d) return;
    cursor = k;
    hovered = -1;
    paint();
  };
  const set = (next: Given) => {
    const was = value;
    value = tidy(SPEC, next) as Knobs;
    const fresh = next.seed !== seed || was.levels !== value.levels;
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
    fork: () => cursor,
    state: () => state,
  };
}
