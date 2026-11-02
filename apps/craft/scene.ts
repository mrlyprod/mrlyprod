import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Over, Report, State, Work } from "./engine.ts";
import {
  CRAFTS,
  deal as dealing,
  ideas,
  NONE,
  OFFICE,
  REST,
  report,
  run,
  SIZE,
  step,
  TEAMS,
  tick,
  WORKS,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  work: Work;
  teams: number;
  bots: number;
  tint: string;
};

export type Told = Report & { tick: number; length: number; bots: number; teams: number; work: Work; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Craft = ReturnType<typeof make>;

type Point = [number, number];

export const SPEC: Spec = [
  {
    key: "work",
    label: "Work rule",
    kind: "segment",
    def: "earn",
    options: [
      ["earn", "Work to eat"],
      ["free", "Basics free"],
    ],
    group: "World",
  },
  {
    key: "teams",
    label: "Teams per craft",
    kind: "slider",
    def: 1,
    min: 1,
    max: TEAMS,
    step: 1,
    show: (v) => (v === 1 ? "One team" : String(v)),
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 48, min: 24, max: 96, step: 6, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "The bots share a town of six crafts around a plaza. A bakery, the builders and a clinic make the basics everyone needs each day; a studio, a music hall and a lab make ideas. Each bot loves one craft, is better at some than others, and gets better at what it practises. A run is sixty days and replays exactly from the seed and the knobs.",
  "The work rule is the first knob. Work to eat: the town hires the best hands for the basics it needs, then a few for ideas; the rest take desks in an office of busywork, and a bot with no desk does not eat that day. Basics free: everyone eats what the bakery makes, each bot works most days at the craft it loves and rests on the others, and when a basic falls short the town calls; about one bot in three answers.",
  "Teams per craft is the second: one team builds each idea together, or several race, each on its own version. A better idea builds faster, the first one finished ships, and the other teams' work on it is dropped.",
  "Fed is the bot-days with a meal, loved the work days spent at the craft a bot loves, idle the bot-days resting or without work, shipped the ideas finished. Enter steps one day, Space pauses, Reroll draws another town. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `craft-${value.work}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New town", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "street",
    title: "Workshop days",
    still: 9000,
    pick: (rand: Rng) => ({
      work: pick(rand, WORKS),
      teams: pick(rand, [1, 1, 2, 3]),
      bots: 36 + 6 * Math.floor(rand() * 4),
    }),
  },
  {
    id: "race",
    title: "Racing teams",
    still: 9000,
    pick: (rand: Rng) => ({
      work: pick(rand, WORKS),
      teams: 3 + Math.floor(rand() * 4),
      bots: 54 + 6 * Math.floor(rand() * 3),
    }),
  },
];

const PACE = 700;
const WALK = 460;
const SHOW = 1100;
const HOLD = 5000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 40;
const WIDTH = 0.28;
const LETTERS = 15;
const LEAD = 1.5;
const SHOP = 0.135;
const SQUARE = 0.025;
const ROAD = 1.3;
const LAWN = 1.6;
const FOUNT = 0.2;
const BOT = 0.11;
const CROWD = 48;
const TABLE = 0.24;
const TABLES = 0.52;
const SEAT = 2.3;
const SHELF = 24;
const BOB = 620;
const SWAY = 2400;
const RIPPLE = 2200;
const CONFETTI = 14;
const STRIPES = 8;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const GOLD = 2.399963;
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["work", "teams", "bots"];

export const HUES = [palette.orange, palette.brown, palette.green, palette.pink, palette.purple, palette.blue];
export const GRAY = palette.gray;
export const NAMES: [string, string][] = [
  ["bread", "bakery, feeds the town"],
  ["homes", "builders, a roof for all"],
  ["care", "clinic, care for all"],
  ["art", "studio, art"],
  ["music", "hall, music"],
  ["science", "lab, science"],
  ["office", "office, busywork"],
];
export const BOTS: string[] = [
  "a bot at the craft it loves",
  "a bot at work it does not love",
  "a bot with no meal today",
  "an idea shipped",
];

const RULES: Record<Work, string> = {
  earn: "a bot eats only on a day it works",
  free: "everyone eats, each bot picks its craft",
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const lerp = (a: Point, b: Point, k: number): Point => [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k];

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export const hue = (place: number) => (place < CRAFTS ? HUES[place] : GRAY);

export const angle = (place: number) => Math.PI / 2 + (place === OFFICE ? 0 : ((place + 1) * TAU) / (CRAFTS + 1));

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let was: State;
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
      tick: state.tick,
      length: state.length,
      bots: state.bots,
      teams: state.teams,
      work: state.work,
      over: state.over,
    });
  };
  const rule = () => (lock ? WORKS[(WORKS.indexOf(value.work) + round) % WORKS.length] : value.work);
  const deal = () => {
    state = dealing({ ...value, work: rule() }, { seed, round });
    was = state;
    if (lock && view.still) {
      was = run(state, STILL - 1);
      state = step(was, tick);
    }
    acted = view.t;
    ended = view.t;
    busy = view.t + WALK;
    tell();
  };
  const type = () => {
    const px = Math.max(12 * view.dpr, Math.min(16 * view.dpr, view.h * 0.019));
    const side = !lock && view.w > view.h * 1.1 ? Math.min(view.w * WIDTH, px * LETTERS) : 0;
    return { px, side };
  };
  const geometry = () => {
    const { px, side } = type();
    const wide = Math.max(1, view.w - side - px * 2);
    const tall = Math.max(1, view.h - px * 2);
    const r =
      Math.min(wide, tall) *
      (SHOP - SQUARE * (1 - Math.min(1, (Math.max(wide, tall) / Math.min(wide, tall) - 1) / 0.6)));
    const cx = side + px + wide / 2;
    const cy = px + tall / 2;
    const rx = wide / 2 - r * ROAD;
    const ry = tall / 2 - r * ROAD;
    const at = (place: number): Point => [cx + Math.cos(angle(place)) * rx, cy + Math.sin(angle(place)) * ry];
    const lawn: Point = [Math.max(r, rx - r * LAWN), Math.max(r, ry - r * LAWN)];
    const b = r * BOT * Math.min(1.1, Math.sqrt(CROWD / state.bots));
    const tables = (place: number): Point[] => {
      const [x, y] = at(place);
      if (!ideas(place) || state.teams === 1) return [[x, y]];
      return Array.from({ length: state.teams }, (_, k): Point => {
        const a = angle(place) + Math.PI + (k * TAU) / state.teams;
        return [x + Math.cos(a) * r * TABLES, y + Math.sin(a) * r * TABLES];
      });
    };
    const table = (place: number) =>
      ideas(place) && state.teams > 1 ? r * Math.min(TABLE * 0.7, Math.sin(Math.PI / state.teams) * 0.32) : r * TABLE;
    return { r, b, cx, cy, rx, ry, at, lawn, tables, table };
  };
  type Geometry = ReturnType<typeof geometry>;
  const seat = (g: Geometry, center: Point, size: number, j: number, face: number): Point => {
    let ring = 0;
    let k = j;
    for (;;) {
      const radius = size + g.b * (1.5 + ring * SEAT);
      const cap = Math.max(4, Math.floor((TAU * radius) / (g.b * SEAT)));
      if (k < cap) {
        const a = face + (k * TAU) / cap + ring * 0.4;
        return [center[0] + Math.cos(a) * radius, center[1] + Math.sin(a) * radius];
      }
      k -= cap;
      ring++;
    }
  };
  const cell = (g: Geometry) => (g.r * 1.36) / Math.ceil(Math.sqrt(Math.max(1, Math.ceil(state.bots / 3))));
  const desks = (g: Geometry) => {
    const n = Math.max(1, Math.ceil(state.bots / 3));
    const cols = Math.ceil(Math.sqrt(n));
    const rows = Math.ceil(n / cols);
    const d = cell(g);
    const [x, y] = g.at(OFFICE);
    return Array.from(
      { length: n },
      (_, j): Point => [x + ((j % cols) - (cols - 1) / 2) * d, y + (Math.floor(j / cols) - (rows - 1) / 2) * d],
    );
  };
  const plaza = (g: Geometry, bot: number, of: State): Point => {
    const f = FOUNT + 0.12;
    const u = Math.sqrt(f * f + ((bot + 0.5) / of.bots) * (0.92 * 0.92 - f * f));
    const a = bot * GOLD;
    return [g.cx + Math.cos(a) * u * g.lawn[0], g.cy + Math.sin(a) * u * g.lawn[1]];
  };
  const queue = (g: Geometry, j: number): Point => {
    const [x, y] = g.at(OFFICE);
    const gap = g.b * SEAT;
    const room = Math.max(1, Math.floor((y - g.cy - g.r * 1.1) / gap));
    const row = Math.floor(j / room);
    const k = j % room;
    const side = (row % 2 ? 1 : -1) * Math.ceil(row / 2) * gap;
    return [x + side + Math.sin(k * 1.3) * g.b * 0.3, y - g.r * 1.1 - k * gap];
  };
  const places = (g: Geometry, of: State): Point[] => {
    const out: Point[] = Array(of.bots);
    const groups = new Map<number, number[]>();
    let line = 0;
    for (let bot = 0; bot < of.bots; bot++) {
      const at = of.at[bot];
      if (at === REST) {
        out[bot] = of.work === "earn" && of.tick ? queue(g, line++) : plaza(g, bot, of);
        continue;
      }
      const key = at * (TEAMS + 1) + (of.team[bot] === NONE ? 0 : of.team[bot]);
      groups.set(key, [...(groups.get(key) ?? []), bot]);
    }
    const office = desks(g);
    for (const [key, crew] of groups) {
      const at = Math.floor(key / (TEAMS + 1));
      const team = key % (TEAMS + 1);
      if (at === OFFICE) {
        crew.forEach((bot, j) => {
          const [x, y] = office[j % office.length];
          out[bot] = [x - cell(g) * 0.2, y];
        });
        continue;
      }
      const center = g.tables(at)[team] ?? g.at(at);
      const face = angle(at) + Math.PI;
      crew.forEach((bot, j) => {
        out[bot] = seat(g, center, g.table(at), j, face);
      });
    }
    return out;
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const glyph = (place: number, x: number, y: number, r: number, color: string) => {
    ctx.fillStyle = color;
    ctx.strokeStyle = color;
    ctx.lineWidth = Math.max(1, r * 0.16);
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.beginPath();
    if (place === 0) {
      ctx.ellipse(x, y + r * 0.05, r * 0.6, r * 0.38, 0, 0, TAU);
      ctx.fill();
      ctx.strokeStyle = veil(palette.black, 0.35);
      ctx.lineWidth = Math.max(1, r * 0.1);
      ctx.beginPath();
      for (const dx of [-0.25, 0, 0.25]) {
        ctx.moveTo(x + (dx - 0.08) * r, y - r * 0.12);
        ctx.lineTo(x + (dx + 0.08) * r, y + r * 0.2);
      }
      ctx.stroke();
      return;
    }
    if (place === 1) {
      ctx.moveTo(x, y - r * 0.6);
      ctx.lineTo(x + r * 0.6, y - r * 0.05);
      ctx.lineTo(x + r * 0.42, y - r * 0.05);
      ctx.lineTo(x + r * 0.42, y + r * 0.52);
      ctx.lineTo(x - r * 0.42, y + r * 0.52);
      ctx.lineTo(x - r * 0.42, y - r * 0.05);
      ctx.lineTo(x - r * 0.6, y - r * 0.05);
      ctx.closePath();
      ctx.fill();
      return;
    }
    if (place === 2) {
      const a = r * 0.2;
      const l = r * 0.58;
      ctx.rect(x - a, y - l, a * 2, l * 2);
      ctx.rect(x - l, y - a, l * 2, a * 2);
      ctx.fill();
      return;
    }
    if (place === 3) {
      ctx.arc(x, y, r * 0.56, 0, TAU);
      ctx.arc(x + r * 0.18, y + r * 0.2, r * 0.14, 0, TAU, true);
      ctx.fill("evenodd");
      ctx.fillStyle = veil(palette.black, 0.3);
      for (const [dx, dy] of [
        [-0.25, -0.15],
        [0.05, -0.32],
        [-0.28, 0.2],
      ]) {
        ctx.beginPath();
        ctx.arc(x + dx * r, y + dy * r, r * 0.1, 0, TAU);
        ctx.fill();
      }
      return;
    }
    if (place === 4) {
      ctx.ellipse(x - r * 0.18, y + r * 0.32, r * 0.24, r * 0.18, -0.4, 0, TAU);
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(x + r * 0.04, y + r * 0.3);
      ctx.lineTo(x + r * 0.04, y - r * 0.55);
      ctx.quadraticCurveTo(x + r * 0.3, y - r * 0.4, x + r * 0.45, y - r * 0.1);
      ctx.stroke();
      return;
    }
    if (place === 5) {
      ctx.moveTo(x - r * 0.16, y - r * 0.58);
      ctx.lineTo(x + r * 0.16, y - r * 0.58);
      ctx.lineTo(x + r * 0.16, y - r * 0.14);
      ctx.lineTo(x + r * 0.55, y + r * 0.5);
      ctx.lineTo(x - r * 0.55, y + r * 0.5);
      ctx.lineTo(x - r * 0.16, y - r * 0.14);
      ctx.closePath();
      ctx.fill();
      return;
    }
    ctx.rect(x - r * 0.4, y - r * 0.52, r * 0.8, r * 1.04);
    ctx.fill();
    ctx.strokeStyle = veil(palette.black, 0.35);
    ctx.lineWidth = Math.max(1, r * 0.09);
    ctx.beginPath();
    for (const dy of [-0.25, 0, 0.25]) {
      ctx.moveTo(x - r * 0.24, y + dy * r);
      ctx.lineTo(x + r * 0.24, y + dy * r);
    }
    ctx.stroke();
  };
  const badge = (place: number, x: number, y: number, r: number) => {
    ctx.fillStyle = veil(palette.black, 0.18);
    ctx.beginPath();
    ctx.arc(x + r * 0.1, y + r * 0.14, r, 0, TAU);
    ctx.fill();
    ctx.fillStyle = hue(place);
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    glyph(place, x, y, r, palette.white);
  };
  const star = (x: number, y: number, r: number, color: string) => {
    ctx.fillStyle = color;
    ctx.beginPath();
    for (let k = 0; k < 10; k++) {
      const a = -Math.PI / 2 + (k * Math.PI) / 5;
      const l = k % 2 ? r * 0.45 : r;
      if (k) ctx.lineTo(x + Math.cos(a) * l, y + Math.sin(a) * l);
      else ctx.moveTo(x + Math.cos(a) * l, y + Math.sin(a) * l);
    }
    ctx.closePath();
    ctx.fill();
  };
  const body = (x: number, y: number, b: number, love: number, mood: number, paper: string, ink: string) => {
    const color = hue(love);
    if (mood === 2) {
      ctx.fillStyle = veil(color, 0.22);
      ctx.beginPath();
      ctx.arc(x, y, b * 1.75, 0, TAU);
      ctx.fill();
    }
    ctx.fillStyle = veil(palette.black, 0.2);
    ctx.beginPath();
    ctx.ellipse(x + b * 0.2, y + b * 0.75, b * 0.9, b * 0.4, 0, 0, TAU);
    ctx.fill();
    ctx.beginPath();
    ctx.arc(x, y, b, 0, TAU);
    if (mood === 0) {
      ctx.fillStyle = paper;
      ctx.fill();
      ctx.strokeStyle = veil(color, 0.85);
      ctx.lineWidth = Math.max(1, b * 0.36);
      ctx.stroke();
      return;
    }
    ctx.fillStyle = mood === 1 ? blend(color, blend(paper, GRAY, 0.5), 0.6) : color;
    ctx.fill();
    ctx.strokeStyle = veil(ink, mood === 1 ? 0.25 : 0.4);
    ctx.lineWidth = Math.max(1, b * 0.18);
    ctx.stroke();
    ctx.fillStyle = veil(palette.white, mood === 1 ? 0.35 : 0.7);
    ctx.beginPath();
    ctx.arc(x - b * 0.32, y - b * 0.32, b * 0.3, 0, TAU);
    ctx.fill();
  };
  const legend = (ink: string, paper: string) => {
    const { px, side } = type();
    if (!side) return;
    const pct = (n: number) => `${Math.round(n / 10)}%`;
    const lines = 2 + NAMES.length + BOTS.length + 2 + 4 * 2;
    const x = px * 1.6;
    let y = view.h / 2 - (lines * px * LEAD) / 2;
    const say = (text: string, size: number, color: string, weight = "") => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    const row = (text: string) => {
      ctx.font = `${px * 0.95}px ${TYPE}`;
      ctx.fillStyle = veil(ink, 0.8);
      ctx.fillText(text, x + px * 1.8, y);
      y += px * LEAD;
    };
    ctx.textBaseline = "middle";
    const title = SPEC[0].options as [string, string][];
    say(title.find(([key]) => key === state.work)![1], px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(RULES[state.work], px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    NAMES.forEach(([, text], place) => {
      badge(place, x + px * 0.6, y, px * 0.62);
      row(text);
    });
    y += px * 0.5;
    const b = px * 0.38;
    body(x + px * 0.6, y, b, 4, 2, paper, ink);
    row(BOTS[0]);
    body(x + px * 0.6, y, b, 4, 1, paper, ink);
    row(BOTS[1]);
    body(x + px * 0.6, y, b, 4, 0, paper, ink);
    row(BOTS[2]);
    star(x + px * 0.6, y, px * 0.5, palette.yellow);
    row(BOTS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${pct(told.fed)} fed`, "bot-days with a meal"],
      [`${pct(told.loved)} loved`, "work at the craft a bot loves"],
      [`${pct(told.idle)} idle`, "resting, or with no work"],
      [`${told.shipped} shipped`, state.teams === 1 ? "ideas, one team each" : `ideas, ${state.teams} teams racing`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const station = (g: Geometry, place: number, k: number, paper: string, day: boolean) => {
    const [x, y] = g.at(place);
    const { r } = g;
    const color = hue(place);
    const out = angle(place);
    const quiet = place === OFFICE && !state.at.includes(OFFICE) && !was.at.includes(OFFICE);
    ctx.save();
    if (quiet) ctx.globalAlpha *= 0.45;
    ctx.fillStyle = veil(palette.black, 0.16);
    ctx.beginPath();
    ctx.arc(x + r * 0.06, y + r * 0.09, r, 0, TAU);
    ctx.fill();
    ctx.fillStyle = blend(paper, color, day ? 0.16 : 0.22);
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(color, 0.6);
    ctx.lineWidth = Math.max(1, r * 0.04);
    ctx.stroke();
    for (let s = 0; s < STRIPES; s++) {
      const a0 = out - 0.95 + (s * 1.9) / STRIPES;
      ctx.fillStyle = s % 2 ? blend(color, palette.white, 0.75) : color;
      ctx.beginPath();
      ctx.arc(x, y, r * 1.2, a0, a0 + 1.9 / STRIPES);
      ctx.arc(x, y, r * 0.97, a0 + 1.9 / STRIPES, a0, true);
      ctx.closePath();
      ctx.fill();
    }
    badge(place, x + Math.cos(out) * r * 1.1, y + Math.sin(out) * r * 1.1, r * 0.22);
    if (place === OFFICE) {
      const d = cell(g);
      desks(g).forEach(([dx, dy], j) => {
        const shuffle = view.still ? 0 : Math.sin(view.t / BOB + j * GOLD) * d * 0.04;
        ctx.fillStyle = blend(paper, GRAY, 0.45);
        ctx.fillRect(dx, dy - d * 0.3, d * 0.42, d * 0.6);
        ctx.fillStyle = blend(paper, palette.white, 0.7);
        ctx.fillRect(dx + d * 0.08 + shuffle, dy - d * 0.22 - shuffle, d * 0.26, d * 0.32);
      });
      ctx.restore();
      return;
    }
    const tables = g.tables(place);
    const size = g.table(place);
    tables.forEach(([tx, ty], team) => {
      ctx.fillStyle = blend(paper, palette.white, day ? 0.6 : 0.12);
      ctx.beginPath();
      ctx.arc(tx, ty, size, 0, TAU);
      ctx.fill();
      ctx.strokeStyle = veil(color, 0.5);
      ctx.lineWidth = Math.max(1, size * 0.08);
      ctx.stroke();
      const pie = (share: number, fill: string) => {
        if (share <= 0) return;
        ctx.fillStyle = fill;
        ctx.beginPath();
        ctx.moveTo(tx, ty);
        ctx.arc(tx, ty, size * 0.78, -Math.PI / 2, -Math.PI / 2 + Math.min(1, share) * TAU);
        ctx.closePath();
        ctx.fill();
      };
      if (!ideas(place)) {
        const need = state.need;
        const now = state.tick ? state.made[place] / need : 0;
        const then = was.tick ? was.made[place] / need : 0;
        const share = Math.min(1, then + (now - then) * k);
        pie(1, veil(palette.red, state.tick && share < 1 ? 0.3 : 0));
        pie(share, color);
        return;
      }
      const i = place * state.teams + team;
      const won = shipped(place);
      if (won !== NONE) {
        const gone = 1 - k;
        if (team === won) pie(1, veil(palette.yellow, 0.4 + 0.6 * gone));
        else pie(was.progress[i] / SIZE, veil(GRAY, 0.6 * gone));
        return;
      }
      const then = was.progress[i] ?? 0;
      pie((then + (state.progress[i] - then) * k) / SIZE, color);
    });
    if (ideas(place)) {
      const count = state.shipped[place];
      const inward = out + Math.PI;
      for (let j = 0; j < Math.min(count, SHELF * 2); j++) {
        const row = Math.floor(j / SHELF);
        const slot = j % SHELF;
        const a = inward + (slot - (SHELF - 1) / 2) * (2.1 / SHELF) * (row ? -1 : 1);
        const radius = r * (1.13 + row * 0.13);
        const pop = j === count - 1 && shipped(place) !== NONE ? k : 1;
        star(x + Math.cos(a) * radius, y + Math.sin(a) * radius, r * 0.075 * pop, blend(palette.yellow, color, 0.25));
      }
    }
    ctx.restore();
  };
  const shipped = (place: number) => {
    for (let j = 0; j < state.ships.length; j += 3) if (state.ships[j] === place) return state.ships[j + 1];
    return NONE;
  };
  const confetti = (g: Geometry, accent: string) => {
    const age = view.t - acted;
    if (view.still || age < 0 || age > SHOW) return;
    const k = age / SHOW;
    for (let j = 0; j < state.ships.length; j += 3) {
      const place = state.ships[j];
      const [tx, ty] = g.tables(place)[state.ships[j + 1]] ?? g.at(place);
      for (let p = 0; p < CONFETTI; p++) {
        const a = hash(p, place + state.tick * 7) * TAU;
        const d = g.r * (0.5 + hash(p, 3) * 0.7) * ease(k);
        const x = tx + Math.cos(a) * d;
        const y = ty + Math.sin(a) * d - g.r * 0.5 * Math.sin(k * Math.PI) + g.r * 0.4 * k * k;
        const colors = [hue(place), accent, palette.yellow, palette.white];
        ctx.fillStyle = veil(colors[p % colors.length], 1 - k);
        ctx.beginPath();
        ctx.arc(x, y, g.b * (0.5 + hash(p, 9) * 0.4), 0, TAU);
        ctx.fill();
      }
    }
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const g = geometry();
    const { r, b } = g;
    const k = view.still ? 1 : ease((view.t - acted) / WALK);
    const day = bright(paper);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    const ground = ctx.createRadialGradient(g.cx, g.cy, 0, g.cx, g.cy, Math.hypot(view.w, view.h) * 0.6);
    ground.addColorStop(0, day ? blend(paper, palette.yellow, 0.1) : blend(paper, palette.indigo, 0.08));
    ground.addColorStop(1, day ? blend(paper, palette.orange, 0.16) : blend(paper, palette.blue, 0.05));
    ctx.fillStyle = ground;
    ctx.fillRect(0, 0, view.w, view.h);
    legend(ink, paper);
    const ring = (rx: number, ry: number) => {
      ctx.beginPath();
      ctx.ellipse(g.cx, g.cy, Math.max(1, rx), Math.max(1, ry), 0, 0, TAU);
    };
    ring(g.rx, g.ry);
    ctx.strokeStyle = veil(palette.black, 0.08);
    ctx.lineWidth = r * 0.62;
    ctx.stroke();
    ctx.strokeStyle = day ? blend(paper, palette.yellow, 0.3) : blend(paper, palette.white, 0.1);
    ctx.lineWidth = r * 0.5;
    ctx.stroke();
    ring(g.lawn[0], g.lawn[1]);
    ctx.fillStyle = blend(paper, palette.green, day ? 0.3 : 0.26);
    ctx.fill();
    ctx.strokeStyle = blend(paper, palette.green, day ? 0.45 : 0.4);
    ctx.lineWidth = Math.max(1, r * 0.05);
    ctx.stroke();
    const f = Math.min(...g.lawn) * FOUNT;
    ctx.fillStyle = blend(paper, GRAY, 0.35);
    ctx.beginPath();
    ctx.arc(g.cx, g.cy, f * 1.15, 0, TAU);
    ctx.fill();
    ctx.fillStyle = blend(accent, paper, day ? 0.35 : 0.3);
    ctx.beginPath();
    ctx.arc(g.cx, g.cy, f, 0, TAU);
    ctx.fill();
    for (let w = 0; w < 3; w++) {
      const ripple = (((view.t / RIPPLE + w / 3) % 1) + 1) % 1;
      ctx.strokeStyle = veil(palette.white, 0.6 * (1 - ripple));
      ctx.lineWidth = Math.max(1, f * 0.06);
      ctx.beginPath();
      ctx.arc(g.cx, g.cy, f * (0.2 + 0.75 * ripple), 0, TAU);
      ctx.stroke();
    }
    for (let place = 0; place <= OFFICE; place++) station(g, place, k, paper, day);
    const from = places(g, was);
    const to = places(g, state);
    const order = Array.from({ length: state.bots }, (_, bot) => bot).sort((a, c) => to[a][1] - to[c][1]);
    for (const bot of order) {
      const at = state.at[bot];
      const moved = was.at[bot] !== at || was.team[bot] !== state.team[bot];
      const [x0, y0] = moved ? lerp(from[bot], to[bot], k) : to[bot];
      const walking = moved && k < 1;
      const phase = hash(bot, 1);
      const loves = at === state.love[bot];
      const hop = walking ? Math.abs(Math.sin(k * Math.PI * 3)) * b * 0.9 : 0;
      const bob = view.still
        ? 0
        : at === REST
          ? Math.sin((view.t / SWAY + phase) * TAU) * b * 0.25
          : at === OFFICE
            ? Math.abs(Math.sin(view.t / (BOB / 6) + phase * TAU)) * b * 0.12
            : Math.abs(Math.sin((view.t / BOB) * Math.PI * (loves ? 1.4 : 0.8) + phase * TAU)) *
              b *
              (loves ? 0.7 : 0.25);
      const fed = state.tick === 0 || state.ate[bot] === 1;
      const mood = !fed ? 0 : at === REST ? 3 : loves ? 2 : 1;
      const size = at === OFFICE && !walking ? Math.min(b, cell(g) * 0.26) : b;
      body(x0, y0 - hop - bob, size, state.love[bot], mood, paper, ink);
      if (loves && !walking && !view.still) {
        const glint = (view.t / (BOB * 3) + phase) % 1;
        if (glint < 0.3)
          star(
            x0 + b * 0.9,
            y0 - b * 1.6 - glint * b * 2,
            b * 0.55 * Math.sin((glint / 0.3) * Math.PI),
            palette.yellow,
          );
      }
    }
    confetti(g, accent);
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    was = state;
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
    busy = state.over && lock ? ended + HOLD : acted + SHOW;
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
      busy = view.t + SHOW;
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
