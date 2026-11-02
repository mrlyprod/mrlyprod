import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Borders, Over, Own, Report, State } from "./engine.ts";
import {
  BORDERS,
  deal as dealing,
  EMPTY,
  FARM,
  H,
  HOME,
  NONE,
  OWNS,
  PARK,
  report,
  run,
  SCHOOL,
  SEA,
  SHOP,
  step,
  TENT,
  tick,
  W,
  WOOD,
  xy,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  own: Own;
  city: number;
  borders: Borders;
  bots: number;
  tint: string;
};

export type Told = Report & { tick: number; length: number; bots: number; own: Own; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Settle = ReturnType<typeof make>;

type Point = [number, number];

export const ONE = 128;

export const SPEC: Spec = [
  {
    key: "own",
    label: "Who owns the land",
    kind: "segment",
    def: "private",
    options: [
      ["private", "First come"],
      ["shared", "Everyone"],
      ["none", "Nobody"],
    ],
    group: "World",
  },
  {
    key: "city",
    label: "Bots per town",
    kind: "slider",
    def: 16,
    min: 4,
    max: ONE,
    step: 4,
    show: (v) => (v >= ONE ? "One city" : String(v)),
    group: "World",
  },
  {
    key: "borders",
    label: "Borders",
    kind: "segment",
    def: "open",
    options: [
      ["open", "Open"],
      ["closed", "Fenced"],
    ],
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 64, min: 16, max: ONE, step: 8, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "The Rainbow Submarine surfaces beside an empty island: grass, woods, a sandy shore. The bots come ashore one by one over sixty days and build a world from nothing: a home each, then farms, workshops, schools and parks, as many as their town needs. A run is a hundred days and replays exactly from the seed and the knobs.",
  "Who owns the land is the first knob. First come: each bot fences a plot of six tiles on arrival and builds its farm and workshop there; a school or a park needs land nobody holds; a bot who finds no land rents a floor in a home, and failing that camps on the beach. Everyone: the town plans together, homes stack into towers near the middle, fields ring the town, woods are kept while other land is free. Nobody: no fences and no plan; a bot builds wherever it finds room near home.",
  "Bots per town is the second: a town full, the next bot founds a new one as far from the others as it can; at the top it is one city. Borders is the third: fenced, the island splits in three lands and a bot stays in the one its passport names, even when the next land is empty.",
  "Housed is the bots with a home out of the bots ashore, fed the bots their town's farms can feed (two a field), green the land open to all and unbuilt, parks included, reach the bots with a school and a park within four tiles. Enter steps one day, Space pauses, Reroll draws another island. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `settle-${value.own}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New island", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "island",
    title: "Island days",
    still: 9000,
    pick: (rand: Rng) => ({
      own: pick(rand, OWNS),
      city: pick(rand, [16, 32, ONE]),
      borders: pick(rand, BORDERS),
      bots: 56 + 8 * Math.floor(rand() * 4),
    }),
  },
  {
    id: "villages",
    title: "Many villages",
    still: 9000,
    pick: (rand: Rng) => ({
      own: pick(rand, OWNS),
      city: 4 + 4 * Math.floor(rand() * 2),
      borders: pick(rand, BORDERS),
      bots: 72 + 8 * Math.floor(rand() * 3),
    }),
  },
];

const PACE = 360;
const SPIN = 320;
const HOLD = 5000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 80;
const WIDTH = 0.28;
const LETTERS = 15;
const LEAD = 1.5;
const PAD = 1.2;
const COAST = 0.74;
const BEACH = 0.3;
const SURF = 0.16;
const TREE = 0.3;
const SLAB = 0.16;
const WALL = 0.1;
const SHADOW = 0.09;
const WALK = 0.15;
const WALKING = 1400;
const HOPS = 5;
const HULL = 2.3;
const BELLY = 0.6;
const PORTS = 4;
const BOB = 0.06;
const WAVE = 2600;
const SWELL = 1700;
const CRESTS = 90;
const DRIFT = 0.00012;
const SMOKE = 2200;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const GOLD = 2.399963;
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["own", "city", "borders", "bots"];

export const HUES = [
  palette.red,
  palette.orange,
  palette.purple,
  palette.blue,
  palette.pink,
  palette.indigo,
  palette.teal,
  palette.cyan,
];
export const RAINBOW = [palette.red, palette.orange, palette.yellow, palette.green, palette.blue, palette.purple];
export const FIELD = palette.yellow;
export const LEAF = palette.green;
export const SHOPS = palette.brown;
export const NAMES: [string, string][] = [
  ["home", "a home, a floor for each bot"],
  ["farm", "a field, feeds two"],
  ["workshop", "a workshop"],
  ["school", "a school"],
  ["park", "a park"],
  ["tent", "a bot with nowhere to build"],
];

const RULES: Record<Own, string> = {
  private: "first come fences a plot",
  shared: "everyone plans the town",
  none: "nobody owns, anyone builds",
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const lerp = (a: Point, b: Point, k: number): Point => [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k];

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export function roads(state: State): [number, number][] {
  const out: [number, number][] = [];
  const stops = [state.shore, ...state.towns];
  const side = (i: number) => (state.borders === "closed" ? state.region[i] : 0);
  const linked = [0];
  const rest = stops.map((_, i) => i).slice(1);
  while (rest.length) {
    let best: [number, number, number] = [NONE, NONE, Infinity];
    for (const a of linked)
      for (const b of rest) {
        if (side(stops[a]) !== side(stops[b]) && linked.some((one) => side(stops[one]) === side(stops[b]))) continue;
        const [ax, ay] = xy(stops[a]);
        const [bx, by] = xy(stops[b]);
        const d = (ax - bx) ** 2 + (ay - by) ** 2;
        if (d < best[2]) best = [a, b, d];
      }
    if (best[0] === NONE) break;
    const [a, b] = best;
    if (side(stops[a]) === side(stops[b])) out.push([stops[a], stops[b]]);
    linked.push(b);
    rest.splice(rest.indexOf(b), 1);
  }
  return out;
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let was: State;
  let told: Report;
  let walkers: [number, number][] = [];
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
      tick: state.tick,
      length: state.length,
      bots: state.bots,
      own: state.own,
      over: state.over,
    });
  };
  const rule = () => (lock ? OWNS[(OWNS.indexOf(value.own) + round) % OWNS.length] : value.own);
  const deal = () => {
    state = dealing({ ...value, own: rule() }, { seed, round });
    was = state;
    walkers = [];
    if (lock && view.still) {
      was = run(state, STILL - 1);
      state = step(was, tick);
    }
    acted = view.t;
    ended = view.t;
    busy = view.t + SPIN;
    tell();
  };
  const type = () => {
    const px = Math.max(12 * view.dpr, Math.min(16 * view.dpr, view.h * 0.019));
    const side = !lock && view.w > view.h * 1.1 ? Math.min(view.w * WIDTH, px * LETTERS) : 0;
    return { px, side };
  };
  const bounds = () => {
    let [x0, y0, x1, y1] = [W, H, 0, 0];
    const take = (x: number, y: number) => {
      x0 = Math.min(x0, x);
      y0 = Math.min(y0, y);
      x1 = Math.max(x1, x + 1);
      y1 = Math.max(y1, y + 1);
    };
    state.ground.forEach((one, i) => {
      if (one !== SEA) take(...xy(i));
    });
    const [dx, dy] = xy(state.dock);
    const [sx, sy] = xy(state.shore);
    take(dx + (dx - sx) * 2 - 3, dy + (dy - sy) * 2);
    take(dx + (dx - sx) * 2 + 3, dy + (dy - sy) * 2);
    return [x0 - PAD, y0 - PAD, x1 + PAD, y1 + PAD];
  };
  const geometry = () => {
    const { px, side } = type();
    const [bx0, by0, bx1, by1] = bounds();
    const room = px;
    const wide = Math.max(1, view.w - side - room * 2);
    const tall = Math.max(1, view.h - room * 2);
    const s = Math.min(wide / (bx1 - bx0), tall / (by1 - by0));
    const x0 = side + room + (wide - (bx1 - bx0) * s) / 2 - bx0 * s;
    const y0 = room + (tall - (by1 - by0) * s) / 2 - by0 * s;
    const corner = (i: number): Point => {
      const [x, y] = xy(i);
      return [x0 + x * s, y0 + y * s];
    };
    const mid = (i: number): Point => {
      const [x, y] = corner(i);
      return [x + s / 2, y + s / 2];
    };
    const tile = (x: number, y: number) => {
      const tx = Math.floor((x - x0) / s);
      const ty = Math.floor((y - y0) / s);
      return tx < 0 || ty < 0 || tx >= W || ty >= H ? NONE : ty * W + tx;
    };
    const center: Point = [x0 + ((bx0 + bx1) / 2) * s, y0 + ((by0 + by1) / 2) * s];
    const [dx, dy] = mid(state.dock);
    const [sx, sy] = mid(state.shore);
    const bob = Math.sin((view.t / WAVE) * TAU) * s * BOB;
    const sub: Point = [dx + (dx - sx) * 0.95, dy + (dy - sy) * 0.95 + bob];
    const hatch: Point = [sub[0] - s * HULL * 0.14, sub[1] - s * BELLY * 2];
    return { s, corner, mid, tile, center, sub, hatch };
  };
  type Geometry = ReturnType<typeof geometry>;
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const block = (x: number, y: number, a: number, deep: number, hue: string) => {
    ctx.fillStyle = blend(hue, palette.black, 0.38);
    ctx.beginPath();
    ctx.roundRect(x - a / 2, y - a / 2, a, a + deep, a * 0.12);
    ctx.fill();
    const top = y - a / 2;
    const left = x - a / 2;
    const roof = (color: string, points: Point[]) => {
      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.moveTo(...points[0]);
      for (const one of points.slice(1)) ctx.lineTo(...one);
      ctx.closePath();
      ctx.fill();
    };
    const peak: Point = [x, y];
    roof(blend(hue, palette.white, 0.22), [[left, top], [left + a, top], peak]);
    roof(blend(hue, palette.white, 0.08), [[left, top], [left, top + a], peak]);
    roof(hue, [[left + a, top], [left + a, top + a], peak]);
    roof(blend(hue, palette.black, 0.14), [[left, top + a], [left + a, top + a], peak]);
  };
  const glyph = (use: number, x: number, y: number, s: number, hue: string, ink: string, paper: string) => {
    const r = s / 2;
    const shadow = (w: number, h: number, round: number) => {
      ctx.fillStyle = veil(palette.black, 0.18);
      ctx.beginPath();
      ctx.roundRect(x - w / 2 + s * SHADOW, y - h / 2 + s * SHADOW, w, h, round);
      ctx.fill();
    };
    if (use === HOME) {
      shadow(r * 1.3, r * 1.3, r * 0.2);
      block(x, y, r * 1.3, s * WALL, hue);
      return;
    }
    if (use === FARM) {
      ctx.fillStyle = blend(FIELD, paper, 0.25);
      ctx.beginPath();
      ctx.roundRect(x - r * 0.88, y - r * 0.88, r * 1.76, r * 1.76, r * 0.14);
      ctx.fill();
      ctx.strokeStyle = blend(FIELD, palette.black, 0.3);
      ctx.lineWidth = Math.max(1, s * 0.06);
      ctx.beginPath();
      for (let k = -1.5; k <= 1.5; k++) {
        ctx.moveTo(x - r * 0.62, y + k * r * 0.42);
        ctx.lineTo(x + r * 0.62, y + k * r * 0.42);
      }
      ctx.stroke();
      return;
    }
    if (use === SHOP) {
      shadow(r * 1.4, r * 1.2, r * 0.12);
      ctx.fillStyle = blend(SHOPS, palette.black, 0.35);
      ctx.beginPath();
      ctx.rect(x + r * 0.15, y - r * 0.95, r * 0.32, r * 0.6);
      ctx.fill();
      block(x, y + r * 0.05, r * 1.3, s * WALL, SHOPS);
      const puff = (((view.t / SMOKE) % 1) + 1) % 1;
      ctx.fillStyle = veil(paper, 0.55 * (1 - puff));
      ctx.beginPath();
      ctx.arc(x + r * 0.31 + puff * r * 0.3, y - r * 1.05 - puff * r * 0.7, r * (0.14 + puff * 0.2), 0, TAU);
      ctx.fill();
      return;
    }
    if (use === SCHOOL) {
      shadow(r * 1.4, r * 1.4, r * 0.7);
      ctx.fillStyle = paper;
      ctx.strokeStyle = ink;
      ctx.lineWidth = Math.max(1, s * 0.08);
      ctx.beginPath();
      ctx.arc(x, y, r * 0.66, 0, TAU);
      ctx.fill();
      ctx.stroke();
      ctx.fillStyle = hue;
      ctx.beginPath();
      ctx.arc(x, y, r * 0.28, 0, TAU);
      ctx.fill();
      return;
    }
    if (use === PARK) {
      ctx.fillStyle = blend(LEAF, paper, 0.45);
      ctx.beginPath();
      ctx.arc(x, y, r * 0.9, 0, TAU);
      ctx.fill();
      for (const [dx, dy] of [
        [-0.32, -0.2],
        [0.3, -0.12],
        [0, 0.34],
      ]) {
        ctx.fillStyle = blend(LEAF, palette.black, 0.12);
        ctx.beginPath();
        ctx.arc(x + dx * r, y + dy * r, r * 0.3, 0, TAU);
        ctx.fill();
        ctx.fillStyle = blend(LEAF, palette.white, 0.3);
        ctx.beginPath();
        ctx.arc(x + (dx - 0.08) * r, y + (dy - 0.08) * r, r * 0.12, 0, TAU);
        ctx.fill();
      }
      return;
    }
    if (use === TENT) {
      ctx.fillStyle = veil(palette.black, 0.18);
      ctx.beginPath();
      ctx.moveTo(x + s * SHADOW, y - r * 0.6 + s * SHADOW);
      ctx.lineTo(x + r * 0.7 + s * SHADOW, y + r * 0.55 + s * SHADOW);
      ctx.lineTo(x - r * 0.7 + s * SHADOW, y + r * 0.55 + s * SHADOW);
      ctx.fill();
      ctx.fillStyle = palette.white;
      ctx.strokeStyle = palette.red;
      ctx.lineWidth = Math.max(1, s * 0.08);
      ctx.beginPath();
      ctx.moveTo(x, y - r * 0.6);
      ctx.lineTo(x + r * 0.7, y + r * 0.55);
      ctx.lineTo(x - r * 0.7, y + r * 0.55);
      ctx.closePath();
      ctx.fill();
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(x, y - r * 0.6);
      ctx.lineTo(x, y + r * 0.55);
      ctx.stroke();
    }
  };
  const legend = (ink: string, paper: string) => {
    const { px, side } = type();
    if (!side) return;
    const pct = (n: number) => `${Math.round(n / 10)}%`;
    const lines = 2 + NAMES.length + 1 + 4 * 2;
    const x = px * 1.6;
    let y = view.h / 2 - (lines * px * LEAD + px * 2) / 2;
    const say = (text: string, size: number, color: string, weight = "") => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    ctx.textBaseline = "middle";
    const title = SPEC[0].options as [string, string][];
    say(title.find(([key]) => key === state.own)![1], px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(RULES[state.own], px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    const uses = [HOME, FARM, SHOP, SCHOOL, PARK, TENT];
    uses.forEach((use, k) => {
      glyph(use, x + px * 0.55, y, px * 1.3, HUES[0], ink, paper);
      ctx.font = `${px * 0.95}px ${TYPE}`;
      ctx.fillStyle = veil(ink, 0.8);
      ctx.fillText(NAMES[k][1], x + px * 1.8, y);
      y += px * LEAD;
    });
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${pct(told.housed)} housed`, `of ${told.ashore} bots ashore`],
      [`${pct(told.fed)} fed`, "by their town's fields"],
      [`${pct(told.green)} green`, "land open to all, parks too"],
      [`${pct(told.reach)} reach`, "a school and a park near home"],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const submarine = (g: Geometry, ink: string, accent: string, sea: string, foam: string) => {
    const { s } = g;
    const rise = Math.min(1, (state.tick + (view.still ? 1 : ease((view.t - acted) / SPIN))) / 2);
    const [x, y0] = g.sub;
    const y = y0 + (1 - rise) * s * 0.7;
    const len = s * HULL;
    const tall = s * BELLY;
    ctx.save();
    ctx.globalAlpha *= 0.5 + 0.5 * rise;
    const swell = (view.t / SWELL) % 1;
    ctx.strokeStyle = veil(foam, 0.6 * (1 - swell));
    ctx.lineWidth = Math.max(1, s * 0.06);
    ctx.beginPath();
    ctx.ellipse(x, y + tall * 0.5, len * (1.05 + swell * 0.35), tall * (0.45 + swell * 0.3), 0, 0, TAU);
    ctx.stroke();
    const tower = (fill: string) => {
      ctx.fillStyle = fill;
      ctx.beginPath();
      ctx.roundRect(x - len * 0.32, y - tall * 2, len * 0.36, tall * 1.3, [tall * 0.4, tall * 0.4, 0, 0]);
      ctx.fill();
    };
    tower(accent);
    ctx.strokeStyle = blend(ink, sea, 0.3);
    ctx.lineWidth = Math.max(1, s * 0.05);
    ctx.beginPath();
    ctx.moveTo(x - len * 0.06, y - tall * 2);
    ctx.lineTo(x - len * 0.06, y - tall * 2.7);
    ctx.lineTo(x + len * 0.06, y - tall * 2.7);
    ctx.stroke();
    ctx.beginPath();
    ctx.roundRect(x - len, y - tall, len * 2, tall * 2, tall);
    ctx.save();
    ctx.clip();
    RAINBOW.forEach((hue, k) => {
      ctx.fillStyle = hue;
      ctx.fillRect(x - len, y - tall + (k * tall * 2) / RAINBOW.length, len * 2, (tall * 2) / RAINBOW.length + 1);
    });
    ctx.fillStyle = veil(palette.white, 0.25);
    ctx.fillRect(x - len, y - tall, len * 2, tall * 0.35);
    ctx.fillStyle = veil(sea, 0.85);
    ctx.fillRect(x - len, y + tall * 0.5, len * 2, tall);
    ctx.restore();
    ctx.strokeStyle = veil(foam, 0.8);
    ctx.lineWidth = Math.max(1, s * 0.06);
    ctx.beginPath();
    ctx.moveTo(x - len * 0.94, y + tall * 0.5);
    ctx.lineTo(x + len * 0.94, y + tall * 0.5);
    ctx.stroke();
    for (let k = 0; k < PORTS; k++) {
      const px = x - len * 0.55 + (k * len * 1.1) / (PORTS - 1);
      ctx.fillStyle = blend(palette.yellow, palette.white, 0.55);
      ctx.strokeStyle = blend(ink, sea, 0.2);
      ctx.lineWidth = Math.max(1, s * 0.05);
      ctx.beginPath();
      ctx.arc(px, y - tall * 0.1, tall * 0.24, 0, TAU);
      ctx.fill();
      ctx.stroke();
    }
    const waiting = Math.min(12, state.home.filter((one) => one === NONE).length);
    ctx.fillStyle = ink;
    ctx.strokeStyle = veil(foam, 0.9);
    ctx.lineWidth = Math.max(1, s * 0.03);
    for (let k = 0; k < waiting; k++) {
      const px = x + len * 0.12 + (k % 6) * len * 0.13;
      const py = y - tall * 1.15 - Math.floor(k / 6) * s * 0.06;
      ctx.beginPath();
      ctx.arc(px, py, s * 0.07, 0, TAU);
      ctx.fill();
      ctx.stroke();
    }
    ctx.restore();
  };
  const waves = (g: Geometry, foam: string) => {
    const { s } = g;
    const { side } = type();
    const near = (x: number, y: number) => {
      for (const [dx, dy] of [
        [0, 0],
        [s, 0],
        [-s, 0],
        [0, s],
        [0, -s],
      ]) {
        const i = g.tile(x + dx, y + dy);
        if (i !== NONE && state.ground[i] !== SEA) return true;
      }
      return false;
    };
    ctx.lineWidth = Math.max(1, s * 0.05);
    for (let j = 0; j < CRESTS; j++) {
      const span = view.w + s * 4;
      const x = ((hash(j, 1) * span + view.t * DRIFT * s * 60) % span) - s * 2;
      const y = hash(j, 2) * view.h;
      if (x < side || near(x, y)) continue;
      const glow = 0.5 + 0.5 * Math.sin((view.t / SWELL) * TAU * (0.6 + hash(j, 3) * 0.6) + hash(j, 4) * TAU);
      const l = s * (0.25 + hash(j, 5) * 0.25);
      ctx.strokeStyle = veil(foam, 0.12 + 0.4 * glow);
      ctx.beginPath();
      ctx.moveTo(x - l, y);
      ctx.quadraticCurveTo(x, y - l * 0.55, x + l, y);
      ctx.stroke();
    }
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const g = geometry();
    const { s } = g;
    const k = view.still ? 1 : ease((view.t - acted) / SPIN);
    const fresh = new Set(state.fresh);
    const day = bright(paper);
    const sea = blend(paper, palette.blue, day ? 0.3 : 0.32);
    const deep = blend(paper, palette.blue, day ? 0.12 : 0.14);
    const foam = day ? palette.white : blend(palette.cyan, palette.white, 0.55);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    const water = ctx.createRadialGradient(...g.center, 0, ...g.center, Math.hypot(view.w, view.h) * 0.62);
    water.addColorStop(0, sea);
    water.addColorStop(0.55, blend(sea, deep, 0.5));
    water.addColorStop(1, deep);
    ctx.fillStyle = water;
    ctx.fillRect(0, 0, view.w, view.h);
    waves(g, foam);
    legend(ink, paper);
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    const isSea = (i: number) => state.ground[i] === SEA;
    const land = state.ground.map((_, i) => i).filter((i) => !isSea(i));
    const cells = (list: number[]) => {
      ctx.beginPath();
      for (const i of list) {
        const [x, y] = g.corner(i);
        ctx.rect(x, y, s, s);
      }
    };
    const blob = (list: number[], grow: number) => {
      ctx.beginPath();
      for (const i of list) {
        const [x, y] = g.mid(i);
        const r = s * (COAST + grow);
        ctx.moveTo(x + r, y);
        ctx.arc(x, y, r, 0, TAU);
      }
    };
    const breath = 0.5 + 0.5 * Math.sin((view.t / WAVE) * TAU);
    blob(land, BEACH + SURF * 2 + 0.12 * breath);
    ctx.fillStyle = veil(foam, 0.22);
    ctx.fill();
    blob(land, BEACH + SURF * (0.6 + 0.6 * breath));
    ctx.fillStyle = veil(foam, 0.6);
    ctx.fill();
    blob(land, BEACH);
    ctx.fillStyle = blend(paper, palette.yellow, day ? 0.34 : 0.55);
    ctx.fill();
    const [cx, cy] = g.center;
    const lit = ctx.createLinearGradient(cx - s * 10, cy - s * 10, cx + s * 10, cy + s * 10);
    lit.addColorStop(0, blend(paper, LEAF, day ? 0.24 : 0.3));
    lit.addColorStop(1, blend(paper, LEAF, day ? 0.36 : 0.42));
    blob(land, 0);
    ctx.fillStyle = lit;
    ctx.fill();
    const owned = land.filter((i) => state.owner[i] !== NONE);
    for (let hue = 0; hue < HUES.length; hue++) {
      const mine = owned.filter((i) => state.owner[i] % HUES.length === hue);
      if (!mine.length) continue;
      cells(mine);
      ctx.fillStyle = veil(HUES[hue], 0.12);
      ctx.fill();
    }
    const roadway = roads(state);
    ctx.strokeStyle = veil(palette.black, 0.12);
    ctx.lineWidth = s * 0.34;
    ctx.beginPath();
    for (const [a, b] of roadway) {
      ctx.moveTo(...g.mid(a));
      ctx.lineTo(...g.mid(b));
    }
    ctx.stroke();
    ctx.strokeStyle = blend(paper, palette.yellow, day ? 0.22 : 0.3);
    ctx.lineWidth = s * 0.24;
    ctx.stroke();
    ctx.lineWidth = Math.max(1, s * 0.055);
    for (let hue = 0; hue < HUES.length; hue++) {
      ctx.strokeStyle = HUES[hue];
      ctx.beginPath();
      for (const i of owned) {
        if (state.owner[i] % HUES.length !== hue) continue;
        const [x, y] = g.corner(i);
        const [tx, ty] = xy(i);
        const edge = (nx: number, ny: number) =>
          nx < 0 || ny < 0 || nx >= W || ny >= H || state.owner[ny * W + nx] !== state.owner[i];
        const inset = s * 0.07;
        if (edge(tx, ty - 1)) {
          ctx.moveTo(x + inset, y + inset);
          ctx.lineTo(x + s - inset, y + inset);
        }
        if (edge(tx + 1, ty)) {
          ctx.moveTo(x + s - inset, y + inset);
          ctx.lineTo(x + s - inset, y + s - inset);
        }
        if (edge(tx, ty + 1)) {
          ctx.moveTo(x + inset, y + s - inset);
          ctx.lineTo(x + s - inset, y + s - inset);
        }
        if (edge(tx - 1, ty)) {
          ctx.moveTo(x + inset, y + inset);
          ctx.lineTo(x + inset, y + s - inset);
        }
      }
      ctx.stroke();
    }
    const closed = state.borders === "closed";
    ctx.beginPath();
    for (const i of land) {
      const [tx, ty] = xy(i);
      const [x, y] = g.corner(i);
      const right = tx + 1 < W ? i + 1 : NONE;
      const below = ty + 1 < H ? i + W : NONE;
      if (right !== NONE && !isSea(right) && state.region[right] !== state.region[i]) {
        ctx.moveTo(x + s, y);
        ctx.lineTo(x + s, y + s);
      }
      if (below !== NONE && !isSea(below) && state.region[below] !== state.region[i]) {
        ctx.moveTo(x, y + s);
        ctx.lineTo(x + s, y + s);
      }
    }
    if (closed) {
      ctx.strokeStyle = veil(palette.black, 0.2);
      ctx.lineWidth = s * 0.26;
      ctx.save();
      ctx.translate(s * SHADOW, s * SHADOW);
      ctx.stroke();
      ctx.restore();
      ctx.strokeStyle = blend(ink, paper, 0.3);
      ctx.lineWidth = s * 0.2;
      ctx.stroke();
      ctx.strokeStyle = blend(paper, ink, 0.25);
      ctx.lineWidth = s * 0.06;
      ctx.stroke();
    } else {
      ctx.strokeStyle = veil(ink, 0.22);
      ctx.lineWidth = Math.max(1, s * 0.05);
      ctx.setLineDash([s * 0.12, s * 0.22]);
      ctx.stroke();
      ctx.setLineDash([]);
    }
    const woods = land.filter((i) => state.ground[i] === WOOD && state.use[i] === EMPTY);
    const crowns = (dx: number, dy: number, grow: number, color: string) => {
      ctx.fillStyle = color;
      ctx.beginPath();
      for (const i of woods) {
        const [x, y] = g.corner(i);
        for (let t = 0; t < 3; t++) {
          const a = hash(i, t) * TAU + t * GOLD;
          const r = s * TREE * (0.75 + 0.35 * hash(i, t + 3));
          const px = x + s / 2 + Math.cos(a) * s * 0.22 + dx * r;
          const py = y + s / 2 + Math.sin(a) * s * 0.22 + dy * r;
          ctx.moveTo(px + r * grow, py);
          ctx.arc(px, py, r * grow, 0, TAU);
        }
      }
      ctx.fill();
    };
    crowns(0.3, 0.35, 1, veil(palette.black, 0.16));
    crowns(0, 0, 1, blend(LEAF, palette.black, 0.34));
    crowns(-0.25, -0.25, 0.45, blend(LEAF, palette.black, 0.12));
    const order = land.filter((i) => state.use[i] !== EMPTY).sort((a, b) => a - b);
    for (const i of order) {
      const use = state.use[i];
      const [x, y] = g.mid(i);
      const hue = HUES[Math.max(0, state.of[i]) % HUES.length];
      const grow = fresh.has(i) && (use !== HOME || state.level[i] === 1 || was.use[i] !== HOME) ? k : 1;
      if (grow <= 0) continue;
      if (use !== HOME || state.level[i] === 1) {
        glyph(use, x, y, s * grow, hue, ink, paper);
        continue;
      }
      const floors = fresh.has(i) && was.use[i] === HOME ? state.level[i] - 1 + k : state.level[i];
      const lift = (floors - 1) * s * SLAB;
      const a = s * 0.66;
      ctx.fillStyle = veil(palette.black, 0.18);
      ctx.beginPath();
      ctx.roundRect(x - a / 2 + s * SHADOW, y - a / 2 + s * SHADOW, a + lift * 0.5, a, a * 0.12);
      ctx.fill();
      block(x, y - lift, a, lift + s * WALL, hue);
      ctx.fillStyle = veil(blend(palette.yellow, palette.white, 0.5), 0.9);
      ctx.beginPath();
      for (let f = 0; f < Math.floor(floors) - 1; f++) {
        const wy = y + a / 2 - lift + f * s * SLAB + s * SLAB * 0.5;
        for (const wx of [-0.3, 0, 0.3]) ctx.rect(x + wx * a - s * 0.035, wy - s * 0.03, s * 0.07, s * 0.06);
      }
      ctx.fill();
    }
    submarine(g, ink, accent, sea, foam);
    ctx.strokeStyle = palette.white;
    ctx.lineWidth = Math.max(1, s * 0.04);
    for (const [bot, from] of walkers) {
      const home = state.home[bot];
      const age = (view.t - from) / WALKING;
      if (home === NONE || age < 0 || age >= 1 || view.still) continue;
      const shore = g.mid(state.shore);
      const [x, y] = age < 0.3 ? lerp(g.hatch, shore, age / 0.3) : lerp(shore, g.mid(home), ease((age - 0.3) / 0.7));
      const hop = Math.abs(Math.sin(age * Math.PI * HOPS)) * s * 0.12;
      ctx.fillStyle = veil(palette.black, 0.2);
      ctx.beginPath();
      ctx.ellipse(x, y + s * 0.1, s * WALK, s * WALK * 0.45, 0, 0, TAU);
      ctx.fill();
      ctx.fillStyle = state.member[bot] === NONE ? ink : HUES[state.member[bot] % HUES.length];
      ctx.beginPath();
      ctx.arc(x, y - hop, s * WALK, 0, TAU);
      ctx.fill();
      ctx.stroke();
    }
  };
  const apply = (at = view.t) => {
    const next = step(state, tick);
    if (next === state) return false;
    was = state;
    state = next;
    walkers = [
      ...walkers.filter(([, from]) => at - from < WALKING),
      ...state.landed.map((bot): [number, number] => [bot, at]),
    ];
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
      apply(acted);
    }
    busy = state.over && lock ? ended + HOLD : acted + (walkers.length ? WALKING : SPIN);
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
  const poke = () => {
    if (state.over) {
      deal();
      paint();
      return;
    }
    paused = true;
    if (apply()) {
      acted = view.t;
      busy = view.t + SPIN;
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
