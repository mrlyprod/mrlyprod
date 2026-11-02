import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Fired } from "../../net/sdk/keys.tsx";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Act, Over, State } from "./engine.ts";
import { deal as dealing, FEW, FULL, home, MANY, period, step, THICK, THIN, turn } from "./engine.ts";
import { pick } from "./policy.ts";

/* TYPES */

export type Knobs = { rings: number; arms: number; play: string; tint: string };

export type Told = { taps: number; par: number; home: number; of: number; over: Over };

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

export type Swirl = ReturnType<typeof make>;

export const SPEC: Spec = [
  { key: "rings", label: "Rings", kind: "slider", def: 5, min: FEW, max: MANY, step: 1, group: "Whorl" },
  { key: "arms", label: "Arms", kind: "slider", def: 3, min: THIN, max: THICK, step: 1, group: "Whorl" },
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
  "A whorl is cut into rings, and each ring has been turned out of place, so its arms break at every seam. Tap a ring to turn it one notch.",
  "A ring drags the ring just outside it along, one notch too. Turn the rings until every arm runs whole from the middle to the rim; a ring glows once it is home.",
  "Each ring is wider than the one inside it by the same factor, and each arm turns by the same angle across every ring, so the whole whorl looks the same at every size: a fractal spiral.",
  "Tap a ring, or move with the arrows and turn with X. Enter asks for a hint. Smart sets the rings from the middle out, in par turns. Reroll deals another whorl.",
];

const OUT = ["ArrowUp", "ArrowRight"];
const IN = ["ArrowDown", "ArrowLeft"];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  gestures: true,
  name: (value: Value) => `swirl-${value.rings}-${value.arms}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Held | null) => (scene?.done?.() ? "New deal" : "Hint"),
      act: "enter",
      button: true,
      menu: "Edit",
    },
    { key: "x", label: "Turn", act: "turn" },
    { key: [...OUT, ...IN], label: "Move", act: "move" },
  ],
  actions: {
    enter: (scene) => (scene.done?.() ? scene.again?.() : scene.hint?.()),
    turn: (scene) => scene.press?.(),
    move: (scene, e: Fired) => scene.move?.(e.key ?? ""),
  },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "shell",
    title: "Shell",
    pick: (rand: Rng) => ({ play: "smart", rings: 5 + Math.floor(rand() * 2), arms: 3 }),
  },
  {
    id: "galaxy",
    title: "Galaxy",
    pick: (rand: Rng) => ({ play: "smart", rings: 7, arms: rand() < 0.5 ? 2 : 4 }),
  },
];

export const FIT = 0.86;
export const HUB = 0.15;
const TWIST = 2.2;
const POINTS = 14;
const WIDE = 0.15;
const CORE = 0.42;
const BEAD = 0.075;
const GROOVE = 0.012;
const BLACK = "#000000";
const WHITE = "#ffffff";
const SWING = 420;
const PACE = 650;
const SLOW = 1200;
const AIM = 300;
const SIGHT = 700;
const BLOOM = 1400;
const BREATH = 2800;
const WAVE = 2400;
const DRIFT = 0.00012;
const HOLD = 3600;
const REST = 6400;
const FADE = 1200;
const LEAVE = 900;
const TAIL = 100;
const DEAL: (keyof Knobs)[] = ["rings", "arms"];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const spring = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 + 2.6 * (k - 1) ** 3 + 1.6 * (k - 1) ** 2);

const fade = (k: number) => ease(Math.max(0, k));

/* GEOMETRY */

export function bands(rings: number, outer: number): number[] {
  const inner = Math.max(0, outer * HUB);
  const grow = (1 / HUB) ** (1 / rings);
  return Array.from({ length: rings + 1 }, (_, k) => inner * grow ** k);
}

export function which(radii: number[], d: number): number {
  for (let k = 0; k + 1 < radii.length; k++) if (d >= radii[k] && d < radii[k + 1]) return k;
  return -1;
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let wound: number[] = [];
  let from: number[] = [];
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
  const tell = () => {
    if (gone) return;
    opts.onTell?.({ taps: state.taps, par: state.par, home: home(state), of: state.rings, over: state.over });
  };
  const deal = () => {
    state = dealing(value, { seed, round });
    wound = [...state.turns].map(Number);
    from = [...wound];
    moved = wound.map(() => -Infinity);
    queued = null;
    ended = state.over ? view.t : Infinity;
    acted = view.t;
    busy = view.t;
    cursor = 0;
    tell();
  };
  const layout = () => {
    const outer = Math.max(0, (Math.min(view.w, view.h) * FIT) / 2);
    return { cx: view.w / 2, cy: view.h / 2, outer, radii: bands(state.rings, outer) };
  };
  const shown = (k: number) =>
    view.still ? wound[k] : from[k] + (wound[k] - from[k]) * spring((view.t - moved[k]) / SWING);
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - rest + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const paint = () => {
    const { cx, cy, outer, radii } = layout();
    const { accent, paper, ink } = view.look();
    const dark = !bright(paper);
    const n = state.rings;
    const arms = state.arms;
    const q = period(arms);
    const ground = dark ? blend(paper, accent, 0.05) : blend(paper, accent, 0.04);
    const floor = dark ? blend(blend(paper, BLACK, 0.35), accent, 0.08) : blend(WHITE, accent, 0.06);
    const glow = dark ? blend(accent, WHITE, 0.2) : blend(accent, BLACK, 0.06);
    const core = dark ? blend(accent, WHITE, 0.8) : blend(accent, WHITE, 0.6);
    const stone = dark ? blend(blend(paper, WHITE, 0.32), accent, 0.08) : blend(blend(ink, WHITE, 0.62), accent, 0.1);
    const done = state.over === "done" ? after(ended, BLOOM) : 0;
    const since = Math.max(0, view.t - ended);
    const breath = done > 0 && !view.still ? 0.5 - 0.5 * Math.cos((2 * Math.PI * since) / BREATH) : 0;
    const drift = done > 0 && !view.still ? since * DRIFT * done : 0;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    ctx.fillStyle = ground;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const big = Math.max(view.w, view.h);
    for (const [k, a] of [
      [1.2, 0.035],
      [0.85, 0.04],
      [0.6, 0.05],
    ]) {
      ctx.beginPath();
      ctx.arc(cx, cy, Math.max(0, big * 0.5 * k), 0, 2 * Math.PI);
      ctx.fillStyle = veil(accent, (dark ? a : a * 0.7) * (1 + 0.8 * done * (0.6 + 0.4 * breath)));
      ctx.fill();
    }
    const rim = radii[n] + outer * 0.04;
    ctx.beginPath();
    ctx.arc(cx, cy + outer * 0.025, Math.max(0, rim + outer * 0.01), 0, 2 * Math.PI);
    ctx.fillStyle = `rgba(0, 0, 0, ${dark ? 0.32 : 0.07})`;
    ctx.fill();
    ctx.beginPath();
    ctx.arc(cx, cy, Math.max(0, rim), 0, 2 * Math.PI);
    ctx.fillStyle = floor;
    ctx.fill();
    ctx.strokeStyle = veil(dark ? WHITE : ink, dark ? 0.08 : 0.08);
    ctx.lineWidth = Math.max(0.5, outer * 0.006);
    ctx.stroke();
    const mark = value.play === "me" && !state.over ? (hovered >= 0 ? hovered : cursor) : -1;
    const aim = queued ? queued.args.ring : -1;
    for (let k = 0; k < n; k++) {
      if (k % 2 === 0 && k !== mark && k !== aim) continue;
      ctx.beginPath();
      ctx.arc(cx, cy, Math.max(0, radii[k + 1]), 0, 2 * Math.PI);
      ctx.arc(cx, cy, Math.max(0, radii[k]), 0, 2 * Math.PI, true);
      ctx.fillStyle =
        k === mark || k === aim ? veil(accent, dark ? 0.16 : 0.1) : veil(dark ? WHITE : ink, dark ? 0.025 : 0.022);
      ctx.fill();
    }
    ctx.strokeStyle = veil(dark ? WHITE : ink, dark ? 0.1 : 0.09);
    ctx.lineWidth = Math.max(0.5, outer * GROOVE * 0.4);
    ctx.beginPath();
    for (let k = 1; k <= n; k++) {
      ctx.moveTo(cx + Math.max(0, radii[k]), cy);
      ctx.arc(cx, cy, Math.max(0, radii[k]), 0, 2 * Math.PI);
    }
    ctx.stroke();
    const base = radii[0] || 1;
    const angle = (j: number, r: number, spin: number) =>
      (2 * Math.PI * j) / arms + TWIST * Math.log(Math.max(r, 1e-6) / base) + spin;
    const wave = done > 0 && !view.still ? ((since % WAVE) / WAVE) * (n + 2) - 1 : -9;
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    for (let k = 0; k < n; k++) {
      const r0 = radii[k];
      const r1 = radii[k + 1];
      const mid = (r0 + r1) / 2;
      const at = shown(k);
      const set = wound[k] % q === 0 && Math.abs(at - wound[k]) < 0.02;
      const spin = (at * 2 * Math.PI) / FULL + drift;
      const pulse = Math.max(0, 1 - Math.abs(wave - k - 0.5));
      const width = Math.max(0.5, (r1 - r0) * WIDE * 1.6);
      for (let j = 0; j < arms; j++) {
        ctx.beginPath();
        for (let i = 0; i <= POINTS; i++) {
          const r = r0 + ((r1 - r0) * i) / POINTS;
          const a = angle(j, r, spin);
          const x = cx + Math.cos(a) * r;
          const y = cy + Math.sin(a) * r;
          if (i) ctx.lineTo(x, y);
          else ctx.moveTo(x, y);
        }
        if (set) {
          ctx.strokeStyle = veil(glow, (dark ? 0.22 : 0.12) + 0.12 * done * breath + 0.25 * pulse * done);
          ctx.lineWidth = width * 2.4;
          ctx.stroke();
          ctx.strokeStyle = glow;
          ctx.lineWidth = width;
          ctx.stroke();
          ctx.strokeStyle = blend(core, WHITE, 0.4 * pulse * done);
          ctx.lineWidth = Math.max(0.5, width * CORE);
          ctx.stroke();
        } else {
          ctx.strokeStyle = stone;
          ctx.lineWidth = width;
          ctx.stroke();
        }
        const a = angle(j, mid, spin) + Math.PI / arms;
        ctx.beginPath();
        ctx.arc(
          cx + Math.cos(a) * mid,
          cy + Math.sin(a) * mid,
          Math.max(0, mid * BEAD * (set ? 0.6 + 0.25 * done * breath + 0.3 * pulse * done : 0.45)),
          0,
          2 * Math.PI,
        );
        ctx.fillStyle = set ? veil(core, dark ? 0.85 : 0.95) : veil(stone, 0.6);
        ctx.fill();
      }
    }
    const hub = Math.max(0, radii[0] * 0.92);
    ctx.beginPath();
    ctx.arc(cx, cy, hub, 0, 2 * Math.PI);
    ctx.fillStyle = blend(glow, WHITE, dark ? 0.15 : 0.1);
    ctx.fill();
    ctx.beginPath();
    ctx.arc(cx, cy, Math.max(0, hub * (0.45 + 0.15 * done * breath)), 0, 2 * Math.PI);
    ctx.fillStyle = core;
    ctx.fill();
    ctx.globalAlpha = 1;
  };
  const apply = (act: Act) => {
    const was = state;
    state = step(state, act);
    if (state === was) return false;
    acted = view.t;
    queued = null;
    const k = act.args.ring;
    for (const m of [k, k + 1])
      if (m < state.rings) {
        from[m] = shown(m);
        wound[m]++;
        moved[m] = view.t;
      }
    busy = view.t + span(SWING);
    if (state.over) {
      ended = view.t + span(SWING);
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
  const read = (u: number, v: number) => {
    const { cx, cy, radii } = layout();
    return which(radii, Math.hypot(u * view.w - cx, v * view.h - cy));
  };
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
    if (k >= 0) apply(turn(k));
  };
  const press = () => {
    if (!state.over) apply(turn(cursor));
  };
  const hint = () => {
    if (state.over) return;
    const act = pick(state, "smart", view.rand);
    if (!act) return;
    cursor = act.args.ring;
    apply(act);
  };
  const move = (key: string) => {
    if (state.over) return;
    const k = Math.min(state.rings - 1, Math.max(0, cursor + (OUT.includes(key) ? 1 : IN.includes(key) ? -1 : 0)));
    if (k === cursor) return;
    cursor = k;
    hovered = -1;
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
