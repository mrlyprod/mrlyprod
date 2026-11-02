import { blend, bright, rgb, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Choose, Over, Report, State } from "./engine.ts";
import { CHOICES, deal as dealing, MAP, MOON, NONE, OPEN, report, step, tick } from "./engine.ts";

/* TYPES */

export type Knobs = {
  choose: Choose;
  bots: number;
  tint: string;
};

export type Told = Report & {
  hour: number;
  length: number;
  choose: Choose;
  bots: number;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Wish = ReturnType<typeof make>;

export const SPEC: Spec = [
  {
    key: "choose",
    label: "A granter picks",
    kind: "segment",
    def: "first",
    options: [
      ["first", "Oldest"],
      ["near", "Nearest"],
      ["most", "Most hearts"],
    ],
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 24, min: 8, max: 56, step: 4, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "Bots wander a quiet field. Now and then a bot stops and makes a wish: a kite, a cake, a song, a flower, a boat, and once in a while the moon. The wish floats above it as a bubble on a board everyone can see. A run is two hundred and forty hours and replays exactly from the seed and the knobs.",
  "Every bot can grant one kind of wish, shown by its colour. A bot that is free and not wishing picks a wish it can grant, walks over and grants it, and the bubble pops. Other bots who want the same thing give a wish hearts. A wish nobody picks up for sixty hours is let go. Nobody can grant the moon.",
  "The knob is how a granter chooses. Oldest: the wish that has waited longest. Nearest: the wish closest to it. Most hearts: the wish most others also want, oldest first on a tie.",
  "Granted counts wishes come true, waiting the bubbles on the board now, oldest wait the longest a bubble there has waited, nobody can grant the wishes no bot in the field could grant, let go the wishes given up. Enter steps an hour, Space pauses, Reroll draws another field. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `wish-${value.choose}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New field", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "board",
    title: "Wishing board",
    still: 10000,
    pick: (rand: Rng) => ({ choose: pick(rand, CHOICES), bots: pick(rand, [20, 24, 28]) }),
  },
  {
    id: "crowd",
    title: "A crowd of wishes",
    still: 10000,
    pick: (rand: Rng) => ({ choose: pick(rand, CHOICES), bots: pick(rand, [40, 48, 56]) }),
  },
];

export const RULES: Record<Choose, string> = {
  first: "oldest first",
  near: "nearest first",
  most: "most hearts first",
};

export const TITLES: Record<Choose, string> = {
  first: "Oldest wish first",
  near: "Nearest wish first",
  most: "Most hearts first",
};

export const NAMES = ["a kite", "a cake", "a song", "a flower", "a boat", "the moon"];

export const SIGNS: string[] = [
  "a bot, the colour of what it grants",
  "a wish on the board",
  "hearts: others want it too",
  "a bot on its way to grant",
];

const PACE = 200;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 90;
const WIDTH = 0.28;
const LETTERS = 20;
const LEAD = 1.5;
const BOB = 2600;
const STARS = 48;
const LIFT = 3.6;
const HEARTS = 5;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["choose", "bots"];

const FIVE = [palette.blue, palette.orange, palette.green, palette.purple, palette.yellow, palette.red];

const near = (a: string, b: string) => {
  const [p, q] = [rgb(a), rgb(b)];
  return p.reduce((sum, v, i) => sum + (v - q[i]) ** 2, 0);
};

export const hues = (accent: string) => {
  const rest = FIVE.slice();
  rest.splice(rest.indexOf(rest.reduce((a, b) => (near(b, accent) < near(a, accent) ? b : a))), 1);
  return [accent, ...rest].slice(0, MOON);
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let prev: State;
  let told: Report;
  let round = 0;
  let acted = view.t;
  let ended = view.t;
  let last = view.t;
  let paused = false;
  let busy = -Infinity;
  let gone = false;
  let rise = -Infinity;
  let held = Infinity;
  let down = Infinity;
  const landing: (() => void)[] = [];
  const lock = Boolean(view.lock);
  const tell = () => {
    told = report(state);
    if (gone) return;
    opts.onTell?.({
      ...told,
      hour: state.hour,
      length: state.length,
      choose: state.choose,
      bots: state.bots,
      over: state.over,
    });
  };
  const rule = () =>
    lock ? CHOICES[(Math.max(0, CHOICES.indexOf(value.choose)) + round) % CHOICES.length] : value.choose;
  const deal = () => {
    state = dealing({ ...value, choose: rule() }, { seed, round });
    if (lock && view.still) for (let k = 0; k < STILL && !state.over; k++) state = step(state, tick);
    prev = state;
    acted = view.t;
    ended = view.t;
    busy = view.t + PACE;
    tell();
  };
  const type = () => {
    const px = Math.max(12 * view.dpr, Math.min(16 * view.dpr, view.h * 0.019));
    const side = !lock && view.w > view.h * 1.1 ? Math.min(view.w * WIDTH, px * LETTERS) : 0;
    return { px, side };
  };
  const geometry = () => {
    const { px, side } = type();
    const pad = px * 2.2;
    const left = side + pad;
    const top = pad * 2.2;
    const wide = Math.max(1, view.w - pad - left);
    const tall = Math.max(1, view.h - pad - top);
    const X = (mx: number) => left + (mx / MAP) * wide;
    const Y = (my: number) => top + (my / MAP) * tall;
    const r = Math.max(0.5, Math.min(px * 0.75, Math.min(wide, tall) / 46));
    return { px, side, X, Y, r };
  };
  type Geometry = ReturnType<typeof geometry>;
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const disc = (x: number, y: number, r: number, color: string) => {
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(x, y, Math.max(0, r), 0, TAU);
    ctx.fill();
  };
  const glyph = (kind: number, x: number, y: number, s: number, color: string, back: string) => {
    ctx.fillStyle = color;
    ctx.strokeStyle = color;
    ctx.lineWidth = Math.max(1, s * 0.16);
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.beginPath();
    if (kind === 0) {
      ctx.moveTo(x, y - s);
      ctx.lineTo(x + s * 0.6, y - s * 0.2);
      ctx.lineTo(x, y + s * 0.5);
      ctx.lineTo(x - s * 0.6, y - s * 0.2);
      ctx.closePath();
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(x, y + s * 0.5);
      ctx.quadraticCurveTo(x - s * 0.4, y + s * 0.8, x + s * 0.1, y + s);
      ctx.stroke();
      return;
    }
    if (kind === 1) {
      ctx.rect(x - s * 0.7, y - s * 0.1, s * 1.4, s * 0.8);
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(x, y - s * 0.1);
      ctx.lineTo(x, y - s * 0.55);
      ctx.stroke();
      disc(x, y - s * 0.8, s * 0.16, color);
      return;
    }
    if (kind === 2) {
      disc(x - s * 0.25, y + s * 0.45, s * 0.32, color);
      ctx.beginPath();
      ctx.moveTo(x + s * 0.05, y + s * 0.45);
      ctx.lineTo(x + s * 0.05, y - s * 0.8);
      ctx.quadraticCurveTo(x + s * 0.6, y - s * 0.5, x + s * 0.5, y - s * 0.1);
      ctx.stroke();
      return;
    }
    if (kind === 3) {
      for (let p = 0; p < 5; p++) {
        const a = (p / 5) * TAU - Math.PI / 2;
        disc(x + Math.cos(a) * s * 0.45, y + Math.sin(a) * s * 0.45, s * 0.32, color);
      }
      disc(x, y, s * 0.24, back);
      return;
    }
    if (kind === 4) {
      ctx.arc(x, y + s * 0.15, s * 0.75, 0, Math.PI);
      ctx.closePath();
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(x, y + s * 0.15);
      ctx.lineTo(x, y - s * 0.9);
      ctx.lineTo(x + s * 0.55, y - s * 0.1);
      ctx.closePath();
      ctx.fill();
      return;
    }
    disc(x, y, s * 0.7, color);
    disc(x + s * 0.32, y - s * 0.2, s * 0.6, back);
  };
  const bot = (x: number, y: number, r: number, hue: string, ink: string, paper: string) => {
    disc(x + r * 0.15, y + r * 0.25, r, veil(palette.black, 0.12));
    disc(x, y, r, hue);
    disc(x - r * 0.32, y - r * 0.1, r * 0.13, veil(paper, 0.9));
    disc(x + r * 0.32, y - r * 0.1, r * 0.13, veil(paper, 0.9));
    ctx.strokeStyle = veil(ink, 0.25);
    ctx.lineWidth = Math.max(1, r * 0.1);
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.stroke();
  };
  const bubble = (x: number, y: number, s: number, kind: number, hue: string, ink: string, paper: string, a = 1) => {
    const fill = blend(paper, hue, 0.14);
    disc(x, y, s, veil(fill, 0.92 * a));
    ctx.strokeStyle = veil(hue, 0.55 * a);
    ctx.lineWidth = Math.max(1, s * 0.07);
    ctx.beginPath();
    ctx.arc(x, y, s, 0, TAU);
    ctx.stroke();
    disc(x - s * 0.42, y - s * 0.42, s * 0.14, veil(palette.white, 0.5 * a));
    glyph(kind, x, y + s * 0.05, s * 0.55, veil(kind === MOON ? blend(hue, ink, 0.2) : hue, a), veil(fill, a));
  };
  const color = (kind: number, set: string[], ink: string, paper: string) =>
    kind === MOON ? blend(paper, ink, 0.55) : set[kind];
  const fit = (text: string, size: number, room: number, weight = "") => {
    ctx.font = `${weight} ${size}px ${TYPE}`.trim();
    const w = Number(ctx.measureText?.(text)?.width) || 0;
    if (w <= room || !room) return size;
    const less = Math.max(1, (size * room) / w);
    ctx.font = `${weight} ${less}px ${TYPE}`.trim();
    return less;
  };
  const legend = (ink: string, paper: string, set: string[]) => {
    const { px, side } = type();
    if (!side) return;
    const lines = 2 + SIGNS.length + 2 + 6 * 2;
    const x = px * 1.6;
    const room = Math.max(1, side - x - px);
    let y = view.h / 2 - (lines * px * LEAD) / 2;
    const say = (text: string, size: number, tone: string, weight = "", at = x) => {
      fit(text, size, room - (at - x), weight);
      ctx.fillStyle = tone;
      ctx.fillText(text, at, y);
    };
    const row = (text: string) => {
      say(text, px * 0.95, veil(ink, 0.8), "", x + px * 1.8);
      y += px * LEAD;
    };
    ctx.textBaseline = "middle";
    say(TITLES[state.choose], px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(`${state.bots} bots, ${NAMES.length} kinds of wish`, px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    const cx = x + px * 0.6;
    bot(cx, y, px * 0.42, set[0], ink, paper);
    row(SIGNS[0]);
    bubble(cx, y, px * 0.5, 0, set[1], ink, paper);
    row(SIGNS[1]);
    for (let h = 0; h < 3; h++) disc(cx - px * 0.4 + h * px * 0.4, y, px * 0.13, veil(palette.red, 0.85));
    row(SIGNS[2]);
    ctx.strokeStyle = veil(set[2], 0.6);
    ctx.lineWidth = Math.max(1, px * 0.1);
    ctx.setLineDash([px * 0.2, px * 0.2]);
    ctx.beginPath();
    ctx.moveTo(cx - px * 0.6, y);
    ctx.lineTo(cx + px * 0.6, y);
    ctx.stroke();
    ctx.setLineDash([]);
    row(SIGNS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${told.granted} granted`, "wishes come true"],
      [`${told.waiting} waiting`, "bubbles on the board now"],
      [`${told.oldest} hours`, "the oldest wait on the board"],
      [`${told.nobody} nobody can grant`, "no bot here grants these"],
      [`${told.gone} let go`, "given up after sixty hours"],
      [`hour ${state.hour}`, `of ${state.length}`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const where = (s: State, g: Geometry, b: number): [number, number] => [g.X(s.x[b]), g.Y(s.y[b])];
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    const set = hues(accent);
    const g = geometry();
    const r = g.r;
    const k = view.still || state.over ? 1 : Math.min(1, Math.max(0, (view.t - acted) / PACE));
    const sway = (i: number) => (view.still ? 0 : Math.sin((view.t / BOB + hash(i, 3)) * TAU));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    ctx.fillStyle = paper;
    ctx.fillRect(0, 0, view.w, view.h);
    const field = Math.max(0, view.w - g.side);
    for (let i = 0; i < STARS; i++)
      disc(
        g.side + hash(i, 1) * field,
        hash(i, 2) * view.h,
        r * (0.1 + 0.12 * hash(i, 4)),
        veil(ink, (day ? 0.1 : 0.16) * (0.6 + 0.4 * sway(i))),
      );
    if (g.side) {
      ctx.fillStyle = veil(ink, 0.12);
      ctx.fillRect(g.side, 0, Math.max(1, g.px * 0.08), view.h);
    }
    legend(ink, paper, set);
    const at = (b: number): [number, number] => {
      const [x0, y0] = where(prev, g, b);
      const [x1, y1] = where(state, g, b);
      return [x0 + (x1 - x0) * k, y0 + (y1 - y0) * k];
    };
    const spots = Array.from({ length: state.bots }, (_, b) => at(b));
    const lift = (w: number): [number, number] => {
      const [x, y] = spots[state.by[w]];
      return [x, y - r * LIFT + sway(w) * r * 0.35];
    };
    ctx.lineWidth = Math.max(1, r * 0.14);
    ctx.setLineDash([r * 0.35, r * 0.4]);
    for (let b = 0; b < state.bots; b++) {
      const w = state.task[b];
      if (w === NONE) continue;
      const [x, y] = spots[b];
      const [tx, ty] = spots[state.by[w]];
      ctx.strokeStyle = veil(set[state.can[b]], 0.45);
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(tx, ty);
      ctx.stroke();
    }
    ctx.setLineDash([]);
    for (let b = 0; b < state.bots; b++) {
      const [x, y] = spots[b];
      const hop = state.task[b] !== NONE && !view.still ? Math.abs(Math.sin((view.t / PACE) * Math.PI)) * r * 0.3 : 0;
      bot(x, y - hop, r, blend(set[state.can[b]], ink, day ? 0.1 : 0), ink, paper);
      if (state.task[b] !== NONE) {
        const kind = state.kind[state.task[b]];
        glyph(kind, x, y - hop - r * 1.6, r * 0.5, set[kind], paper);
      }
    }
    const fresh = new Set(state.posted);
    for (let w = 0; w < state.kind.length; w++) {
      if (state.end[w] !== OPEN) continue;
      const [x, y] = lift(w);
      const [bx, by] = spots[state.by[w]];
      const grow = fresh.has(w) ? ease(k) : 1;
      const hue = color(state.kind[w], set, ink, paper);
      disc(bx + r * 0.5, by - r * 1.25, r * 0.18 * grow, veil(hue, 0.5));
      disc(bx + r * 0.3, by - r * 1.8, r * 0.26 * grow, veil(hue, 0.6));
      bubble(x, y, r * 1.45 * grow, state.kind[w], hue, ink, paper);
      const n = Math.min(HEARTS, state.hearts[w]);
      for (let h = 0; h < n; h++)
        disc(x + (h - (n - 1) / 2) * r * 0.45, y - r * 1.85 * grow, r * 0.15 * grow, veil(palette.red, 0.85));
    }
    if (k < 1) {
      for (const w of state.given) {
        const [x, y] = lift(w);
        const hue = color(state.kind[w], set, ink, paper);
        ctx.strokeStyle = veil(hue, 0.7 * (1 - k));
        ctx.lineWidth = Math.max(1, r * 0.2);
        ctx.beginPath();
        ctx.arc(x, y, r * (1.45 + 2 * ease(k)), 0, TAU);
        ctx.stroke();
        for (let s = 0; s < 6; s++) {
          const a = (s / 6) * TAU + hash(w, s);
          disc(x + Math.cos(a) * r * 2.6 * ease(k), y + Math.sin(a) * r * 2.6 * ease(k), r * 0.16, veil(hue, 1 - k));
        }
      }
      for (const w of state.dropped) {
        const [x, y] = lift(w);
        bubble(
          x,
          y - r * 3 * ease(k),
          r * 1.45,
          state.kind[w],
          color(state.kind[w], set, ink, paper),
          ink,
          paper,
          1 - k,
        );
      }
    }
    ctx.globalAlpha = 1;
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    prev = state;
    state = next;
    if (state.over) ended = view.t;
    tell();
    return true;
  };
  const advance = () => {
    if (paused || held < Infinity) return;
    if (state.over) {
      if (!lock || view.t - ended < HOLD) return;
      round++;
      deal();
      rise = view.t;
      busy = view.t + FADE;
      return;
    }
    if (view.t - acted < PACE) return;
    if (view.t - acted > PACE * CATCH) acted = view.t - PACE;
    while (!state.over && view.t - acted >= PACE) {
      acted += PACE;
      apply();
    }
    busy = state.over && lock ? ended + HOLD : acted + PACE;
  };
  const draw = () => {
    if (view.t !== last) advance();
    last = view.t;
    const moving = !paused && !state.over && held === Infinity && !view.still;
    if (view.t <= busy + TAIL || moving || (lock && !view.still)) paint();
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
  const poke = () => {
    if (state.over) {
      deal();
      paint();
      return;
    }
    paused = true;
    if (apply()) {
      acted = view.t;
      busy = view.t + PACE;
    }
    paint();
  };
  const set = (next: Given) => {
    const old = value;
    value = tidy(SPEC, next) as Knobs;
    const fresh = next.seed !== seed || DEAL.some((key) => old[key] !== value[key]);
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
      if (!paused) acted = view.t;
    },
    playing: () => !paused,
    hold,
    windDown,
    done: () => state.over !== null,
    poke,
    state: () => state,
  };
}
