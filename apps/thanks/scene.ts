import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Credit, Over, Report, State } from "./engine.ts";
import { alive, BUILD, CREDITS, deal as dealing, HAND, IDLE, PARTS, report, step, tick } from "./engine.ts";

/* TYPES */

export type Knobs = {
  credit: Credit;
  bots: number;
  tint: string;
};

export type Told = Report & {
  day: number;
  length: number;
  credit: Credit;
  bots: number;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Thanks = ReturnType<typeof make>;

type Spot = [number, number];

export const SPEC: Spec = [
  {
    key: "credit",
    label: "Thanks go to",
    kind: "segment",
    def: "runner",
    options: [
      ["runner", "Runner"],
      ["maker", "Maker"],
      ["all", "Everyone"],
    ],
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 32, min: 16, max: 64, step: 8, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A town of bots needs goods every day, as many as three bots in four. A bot makes one by hand at its bench; a bot with no hand work and a knack for it builds a robot, which makes two, but only while the robots fall short of the work. A finished robot goes to the bench of the bot with the most thanks, who pays its maker twelve thanks for it; if nobody else can, the maker keeps it. Robots wear out after forty-five to ninety days and the bots build new ones. A run is a hundred and twenty days and replays exactly from the seed and the knobs.",
  "The knob is who a robot's work thanks: the runner, the bot whose bench it works at; the maker, the bot who built it; or everyone, one bot after another in turn. A bot that makes a good by hand is always thanked itself.",
  "Made by robots counts today's goods, the top fifth its share of all the thanks there are, nothing to do the bots neither making nor building, robots built every robot so far. Enter steps a day, Space pauses, Reroll draws another town. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `thanks-${value.credit}-${value.seed}`,
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
    id: "workshop",
    title: "Workshop",
    still: 10000,
    pick: (rand: Rng) => ({ credit: pick(rand, CREDITS), bots: pick(rand, [24, 32, 40]) }),
  },
  {
    id: "sparks",
    title: "Thanks for all",
    still: 10000,
    pick: (rand: Rng) => ({ credit: pick(rand, ["all", "maker", "all"]), bots: pick(rand, [48, 56, 64]) }),
  },
];

const PACE = 360;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 70;
const WIDTH = 0.3;
const LETTERS = 20;
const LEAD = 1.5;
const GOLD = 2.39996;
const TRIP = 1700;
const BOB = 900;
const TRAIL = 6;
const BLAZE = 4;
const RAYS = 10;
const SPIN = 9000;
const COMET = 3;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["credit", "bots"];

export const TITLES: Record<Credit, [string, string]> = {
  runner: ["Thanks to the runner", "robots thank the bot they work for"],
  maker: ["Thanks to the maker", "robots thank the bot who built them"],
  all: ["Thanks to everyone", "robots thank every bot in turn"],
};

export const SIGNS: string[] = [
  "a workshop, aglow with thanks",
  "a robot at work, carrying goods",
  "a robot being built",
  "thanks flowing along a thread",
];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

const WARM = [palette.orange, palette.red, palette.yellow, palette.mint, palette.pink, palette.brown, palette.teal];

export const gold = (day: boolean) => blend(palette.yellow, palette.orange, day ? 0.45 : 0.1);

export function grid(n: number, w: number, h: number) {
  const cols = Math.max(1, Math.min(n, Math.round(Math.sqrt((n * Math.max(1, w)) / Math.max(1, h)))));
  const rows = Math.ceil(n / cols);
  return { cols, rows };
}

export const slot = (j: number, r: number, room: number): Spot => {
  const a = j * GOLD + Math.PI / 3;
  const d = room + r * 1.3 * Math.sqrt(j + 0.4);
  return [Math.cos(a) * d, Math.sin(a) * d];
};

const credited = (s: State, robot: number) =>
  s.credit === "runner" ? s.runner[robot] : s.credit === "maker" ? s.maker[robot] : -1;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
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
  const trail: number[][] = [];
  const lock = Boolean(view.lock);
  const tell = () => {
    told = report(state);
    if (gone) return;
    opts.onTell?.({
      ...told,
      day: state.day,
      length: state.length,
      credit: state.credit,
      bots: state.bots,
      over: state.over,
    });
  };
  const rule = (): Credit =>
    lock ? CREDITS[(Math.max(0, CREDITS.indexOf(value.credit)) + round) % CREDITS.length] : value.credit;
  const deal = () => {
    state = dealing({ ...value, credit: rule() }, { seed, round });
    trail.length = 0;
    if (lock && view.still) for (let k = 0; k < STILL && !state.over; k++) state = step(state, tick);
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
    const pad = px * 2.4;
    const left = side + pad;
    const wide = Math.max(1, view.w - pad - left);
    const tall = Math.max(1, view.h - pad * 2);
    const { cols, rows } = grid(state.bots, wide, tall);
    const cw = wide / cols;
    const ch = tall / rows;
    const r = Math.max(0.5, Math.min(cw, ch) * 0.16);
    const benches: Spot[] = [];
    for (let bot = 0; bot < state.bots; bot++) {
      const row = Math.floor(bot / cols);
      const col = bot % cols;
      const shift = (row % 2 ? 0.22 : -0.22) + (cols - Math.min(cols, state.bots - row * cols)) / 2;
      benches.push([
        left + (col + 0.5 + shift + (hash(bot, 1) - 0.5) * 0.42) * cw,
        pad + (row + 0.5 + (hash(bot, 2) - 0.5) * 0.42) * ch,
      ]);
    }
    return { px, side, pad, left, cw, ch, cols, rows, r, benches };
  };
  type Geometry = ReturnType<typeof geometry>;
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const homes = (g: Geometry) => {
    const count = new Map<number, number>();
    const out: Spot[] = [];
    for (let robot = 0; robot < state.ends.length; robot++) {
      const runner = state.runner[robot];
      const j = count.get(runner) ?? 0;
      if (alive(state, robot) || state.broke.includes(robot)) count.set(runner, j + 1);
      const [dx, dy] = slot(j, g.r, g.r * 2.4);
      const x = Math.min(view.w - g.r, Math.max(g.side + g.r, g.benches[runner][0] + dx));
      const y = Math.min(view.h - g.r, Math.max(g.r * 1.4, g.benches[runner][1] + dy));
      out.push([x, y]);
    }
    return out;
  };
  const workshop = (x: number, y: number, s: number, hue: string, paper: string, ink: string, day: boolean) => {
    const wall = blend(paper, hue, day ? 0.2 : 0.3);
    ctx.fillStyle = veil(palette.black, day ? 0.12 : 0.35);
    ctx.beginPath();
    ctx.ellipse(x + s * 0.2, y + s * 0.95, s * 1.6, s * 0.36, 0, 0, TAU);
    ctx.fill();
    ctx.lineJoin = "round";
    ctx.strokeStyle = veil(ink, day ? 0.28 : 0.22);
    ctx.lineWidth = Math.max(1, s * 0.08);
    ctx.fillStyle = wall;
    ctx.beginPath();
    ctx.rect(x - s * 1.2, y - s * 0.3, s * 2.4, s * 1.2);
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = blend(hue, ink, day ? 0.12 : 0);
    ctx.beginPath();
    ctx.moveTo(x - s * 1.32, y - s * 0.28);
    ctx.lineTo(x - s * 1.32, y - s * 1.05);
    ctx.lineTo(x, y - s * 0.28);
    ctx.lineTo(x, y - s * 1.05);
    ctx.lineTo(x + s * 1.32, y - s * 0.28);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = blend(wall, palette.black, day ? 0.35 : 0.5);
    ctx.beginPath();
    ctx.roundRect(x - s * 0.42, y + s * 0.05, s * 0.84, s * 0.85, [s * 0.3, s * 0.3, 0, 0]);
    ctx.fill();
  };
  const bot = (x: number, y: number, s: number, color: string, ink: string, lift = 0) => {
    ctx.fillStyle = color;
    ctx.strokeStyle = veil(ink, 0.45);
    ctx.lineWidth = Math.max(1, s * 0.12);
    ctx.beginPath();
    ctx.ellipse(x, y + s * 0.4, s * 0.55, s * 0.5, 0, 0, TAU);
    ctx.fill();
    ctx.stroke();
    ctx.beginPath();
    ctx.arc(x, y - s * 0.42 - lift, s * 0.4, 0, TAU);
    ctx.fill();
    ctx.stroke();
  };
  const robot = (x: number, y: number, s: number, body: string, ink: string, crate: string | null) => {
    const edge = blend(body, ink, 0.45);
    ctx.fillStyle = veil(palette.black, 0.2);
    ctx.beginPath();
    ctx.ellipse(x + s * 0.1, y + s * 0.85, s * 0.75, s * 0.22, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = edge;
    for (const side of [-1, 1]) {
      ctx.beginPath();
      ctx.arc(x + side * s * 0.38, y + s * 0.66, s * 0.2, 0, TAU);
      ctx.fill();
    }
    ctx.strokeStyle = edge;
    ctx.lineWidth = Math.max(1, s * 0.12);
    ctx.beginPath();
    ctx.moveTo(x, y - s * 0.55);
    ctx.lineTo(x, y - s * 0.95);
    ctx.stroke();
    ctx.fillStyle = crate ?? body;
    ctx.beginPath();
    ctx.arc(x, y - s * 0.98, s * 0.17, 0, TAU);
    ctx.fill();
    ctx.fillStyle = body;
    ctx.beginPath();
    ctx.roundRect(x - s * 0.62, y - s * 0.58, s * 1.24, s * 1.2, s * 0.3);
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = blend(body, palette.white, 0.6);
    ctx.beginPath();
    ctx.roundRect(x - s * 0.44, y - s * 0.4, s * 0.88, s * 0.5, s * 0.18);
    ctx.fill();
    ctx.fillStyle = edge;
    for (const side of [-1, 1]) {
      ctx.beginPath();
      ctx.arc(x + side * s * 0.2, y - s * 0.15, s * 0.1, 0, TAU);
      ctx.fill();
    }
    if (!crate) return;
    ctx.fillStyle = crate;
    ctx.strokeStyle = blend(crate, ink, 0.4);
    ctx.lineWidth = Math.max(1, s * 0.08);
    ctx.beginPath();
    ctx.roundRect(x - s * 0.4, y + s * 0.18, s * 0.8, s * 0.42, s * 0.08);
    ctx.fill();
    ctx.stroke();
  };
  const frame = (x: number, y: number, s: number, done: number, color: string, ink: string) => {
    ctx.strokeStyle = veil(ink, 0.45);
    ctx.lineWidth = Math.max(1, s * 0.1);
    ctx.setLineDash([Math.max(1, s * 0.22), Math.max(1, s * 0.16)]);
    ctx.beginPath();
    ctx.roundRect(x - s * 0.6, y - s * 0.55, s * 1.2, s * 1.15, s * 0.28);
    ctx.stroke();
    ctx.setLineDash([]);
    const fill = s * 1.15 * Math.min(1, Math.max(0, done));
    if (fill <= 0) return;
    ctx.save();
    ctx.beginPath();
    ctx.rect(x - s * 0.6, y + s * 0.6 - fill, s * 1.2, fill);
    ctx.clip();
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.roundRect(x - s * 0.6, y - s * 0.55, s * 1.2, s * 1.15, s * 0.28);
    ctx.fill();
    ctx.restore();
  };
  const spark = (x: number, y: number, s: number, color: string, alpha: number) => {
    ctx.fillStyle = veil(color, 0.22 * alpha);
    ctx.beginPath();
    ctx.arc(x, y, s * 2.6, 0, TAU);
    ctx.fill();
    ctx.fillStyle = veil(color, alpha);
    ctx.beginPath();
    ctx.arc(x, y, s, 0, TAU);
    ctx.fill();
    ctx.fillStyle = veil(palette.white, 0.7 * alpha);
    ctx.beginPath();
    ctx.arc(x, y, s * 0.45, 0, TAU);
    ctx.fill();
  };
  const glow = (x: number, y: number, size: number, color: string, alpha: number) => {
    const ray = ctx.createRadialGradient(x, y, 0, x, y, size);
    ray.addColorStop(0, veil(color, alpha));
    ray.addColorStop(0.4, veil(color, alpha * 0.45));
    ray.addColorStop(1, veil(color, 0));
    ctx.fillStyle = ray;
    ctx.beginPath();
    ctx.arc(x, y, size, 0, TAU);
    ctx.fill();
  };
  const bend = (a: Spot, b: Spot, salt: number): Spot => {
    const mx = (a[0] + b[0]) / 2;
    const my = (a[1] + b[1]) / 2;
    const dx = b[0] - a[0];
    const dy = b[1] - a[1];
    const lean = 0.18 + 0.12 * hash(salt, 7);
    return [mx - dy * lean, my + dx * lean];
  };
  const along = (a: Spot, c: Spot, b: Spot, k: number): Spot => {
    const u = 1 - k;
    return [u * u * a[0] + 2 * u * k * c[0] + k * k * b[0], u * u * a[1] + 2 * u * k * c[1] + k * k * b[1]];
  };
  const curve = (a: Spot, c: Spot, b: Spot) => {
    ctx.beginPath();
    ctx.moveTo(a[0], a[1]);
    ctx.quadraticCurveTo(c[0], c[1], b[0], b[1]);
    ctx.stroke();
  };
  const thread = (a: Spot, b: Spot, salt: number, color: string, alpha: number, width: number) => {
    const c = bend(a, b, salt);
    ctx.lineCap = "round";
    ctx.strokeStyle = veil(color, alpha * 0.22);
    ctx.lineWidth = width * 4;
    curve(a, c, b);
    ctx.strokeStyle = veil(color, alpha);
    ctx.lineWidth = width;
    curve(a, c, b);
    return c;
  };
  const legend = (ink: string, paper: string, day: boolean, hues: { accent: string; gold: string; mate: string }) => {
    const { px, side } = type();
    if (!side) return;
    const pct = (n: number) => `${(n / 10).toFixed(1)}%`;
    const lines = 2 + SIGNS.length + 2 + 5 * 2;
    const x = px * 1.6;
    let y = view.h / 2 - (lines * px * LEAD) / 2;
    const fit = (text: string, at: number, size: number, weight: string) => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      const wide = Number(ctx.measureText(text)?.width);
      const room = side - at - px * 0.8;
      if (!Number.isFinite(wide) || wide <= room || room <= 0) return;
      ctx.font = `${weight} ${Math.max(1, (size * room) / wide)}px ${TYPE}`.trim();
    };
    const say = (text: string, size: number, color: string, weight = "") => {
      fit(text, x, size, weight);
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    const row = (text: string) => {
      fit(text, x + px * 1.8, px * 0.95, "");
      ctx.fillStyle = veil(ink, 0.8);
      ctx.textBaseline = "middle";
      ctx.fillText(text, x + px * 1.8, y);
      y += px * LEAD;
    };
    ctx.textBaseline = "middle";
    const [title, sub] = TITLES[state.credit];
    say(title, px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(sub, px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    const s = px * 0.4;
    glow(x + px * 0.6, y, px * 0.9, hues.gold, 0.8);
    workshop(x + px * 0.6, y, s, WARM[0], paper, ink, day);
    bot(x + px * 0.6, y + s * 0.5, s * 0.7, hues.mate, ink);
    row(SIGNS[0]);
    robot(x + px * 0.6, y, s * 1.05, hues.accent, ink, hues.gold);
    row(SIGNS[1]);
    frame(x + px * 0.6, y, s, 0.5, hues.accent, ink);
    row(SIGNS[2]);
    thread([x, y + px * 0.3], [x + px * 1.2, y + px * 0.3], 3, hues.gold, 0.8, Math.max(1, px * 0.08));
    spark(x + px * 0.6, y - px * 0.05, px * 0.1, hues.gold, 1);
    row(SIGNS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${pct(told.share)} by robots`, "of today's goods"],
      [`${pct(told.top)} to the top fifth`, "of all the thanks there are"],
      [`${told.idle} with nothing to do`, `of ${state.bots} bots, not making or building`],
      [`${told.built} robots built`, `${told.alive} at work or waiting now`],
      [`day ${state.day}`, `of ${state.length}`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const ground = (g: Geometry, paper: string, day: boolean) => {
    const land = blend(paper, palette.orange, day ? 0.08 : 0.07);
    const x0 = g.side;
    const wide = Math.max(0, view.w - x0);
    ctx.fillStyle = land;
    ctx.fillRect(x0, 0, wide, view.h);
    const light = ctx.createRadialGradient(
      x0 + wide * 0.35,
      -view.h * 0.1,
      0,
      x0 + wide * 0.35,
      -view.h * 0.1,
      Math.max(1, Math.hypot(wide, view.h)),
    );
    light.addColorStop(0, veil(palette.yellow, day ? 0.22 : 0.1));
    light.addColorStop(1, veil(palette.yellow, 0));
    ctx.fillStyle = light;
    ctx.fillRect(x0, 0, wide, view.h);
    ctx.fillStyle = veil(palette.green, day ? 0.08 : 0.06);
    for (let i = 0; i < state.bots; i++) {
      const gx = x0 + hash(i, 11) * wide;
      const gy = hash(i, 12) * view.h;
      const size = g.r * (2 + 3 * hash(i, 13));
      ctx.beginPath();
      ctx.ellipse(gx, gy, size, size * 0.45, 0, 0, TAU);
      ctx.fill();
    }
    ctx.strokeStyle = blend(land, palette.brown, day ? 0.16 : 0.22);
    ctx.lineWidth = Math.max(1, g.r * 0.55);
    ctx.lineCap = "round";
    ctx.beginPath();
    for (let one = 0; one < state.bots; one++) {
      const a = g.benches[one];
      const near = [one % g.cols < g.cols - 1 ? one + 1 : -1, one + g.cols];
      for (const other of near) {
        if (other < 0 || other >= state.bots) continue;
        const b = g.benches[other];
        const c = bend(a, b, one * 3 + other);
        const m: Spot = [(a[0] + b[0] + c[0]) / 3, (a[1] + b[1] + c[1]) / 3];
        ctx.moveTo(a[0], a[1] + g.r * 0.9);
        ctx.quadraticCurveTo(m[0], m[1] + g.r * 0.9, b[0], b[1] + g.r * 0.9);
      }
    }
    ctx.stroke();
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    const shine = gold(day);
    const mate = blend(ink, paper, day ? 0.25 : 0.12);
    const rest = blend(ink, paper, day ? 0.55 : 0.45);
    const g = geometry();
    const k = view.still || state.over ? 1 : Math.min(1, Math.max(0, (view.t - acted) / PACE));
    const base = lock ? shade() : 1;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = base;
    ctx.fillStyle = paper;
    ctx.fillRect(0, 0, view.w, view.h);
    legend(ink, paper, day, { accent, gold: shine, mate });
    ctx.save();
    ctx.beginPath();
    ctx.rect(g.side, 0, Math.max(0, view.w - g.side), view.h);
    ctx.clip();
    ground(g, paper, day);
    const r = g.r;
    const total = Math.max(
      1,
      state.thanks.reduce((a, b) => a + b, 0),
    );
    state.thanks.forEach((n, one) => {
      if (!n) return;
      const [x, y] = g.benches[one];
      const share = Math.min(BLAZE, (n * state.bots) / total);
      glow(x, y, r * (1.4 + 1.6 * share), shine, Math.min(0.95, (day ? 0.3 : 0.25) + 0.16 * share));
      if (share < 2) return;
      ctx.strokeStyle = veil(shine, day ? 0.55 : 0.45);
      ctx.lineWidth = Math.max(1, r * 0.14);
      ctx.lineCap = "round";
      ctx.beginPath();
      const turn = view.still ? 0 : view.t / SPIN;
      for (let i = 0; i < RAYS; i++) {
        const a = (i / RAYS) * TAU + turn + hash(one, 9) * TAU;
        const long = r * (1.4 + 1.6 * share) * (0.75 + 0.2 * Math.sin(view.t / 300 + i * 2));
        ctx.moveTo(x + Math.cos(a) * r * 1.9, y + Math.sin(a) * r * 1.9);
        ctx.lineTo(x + Math.cos(a) * long, y + Math.sin(a) * long);
      }
      ctx.stroke();
    });
    const spots = homes(g);
    const at = (robot: number): Spot => {
      const home = spots[robot];
      if (!state.fresh.includes(robot)) return home;
      const from = g.benches[state.maker[robot]];
      const e = ease(k);
      return [from[0] + (home[0] - from[0]) * e, from[1] + (home[1] - from[1]) * e];
    };
    const width = Math.max(1, r * 0.13);
    for (let one = 0; one < state.ends.length; one++) {
      const to = credited(state, one);
      if (to < 0 || !alive(state, one)) continue;
      thread(at(one), g.benches[to], one, shine, day ? 0.7 : 0.6, width);
    }
    if (state.credit === "all")
      trail.forEach((flows, age) => {
        const fadeOut = 1 - (age + k) / TRAIL;
        if (fadeOut <= 0) return;
        for (let i = 0; i < flows.length; i += 3) {
          if (!alive(state, flows[i]) && !state.broke.includes(flows[i])) continue;
          thread(at(flows[i]), g.benches[flows[i + 1]], flows[i] + i, shine, 0.32 * fadeOut, width * 0.6);
        }
      });
    for (let one = 0; one < state.bots; one++) {
      const [x, y] = g.benches[one];
      const job = state.job[one];
      workshop(x, y, r, WARM[Math.floor(hash(one, 5) * WARM.length)], paper, ink, day);
      const lift =
        job === HAND && !view.still ? Math.abs(Math.sin((view.t / BOB + hash(one, 3)) * Math.PI)) * r * 0.25 : 0;
      if (job === IDLE) bot(x - r * 0.75, y + r * 0.85, r * 0.55, rest, ink);
      else bot(x, y + r * 0.45, r * 0.72, job === BUILD ? blend(mate, accent, 0.4) : mate, ink, lift);
      if (job === BUILD) frame(x + r * 1.75, y + r * 0.4, r * 0.7, state.parts[one] / PARTS, accent, ink);
      if (job === HAND && k < 1) spark(x, y - r * (0.6 + 1.6 * ease(k)), r * 0.13, shine, 1 - k);
    }
    for (let one = 0; one < state.ends.length; one++) {
      const live = alive(state, one);
      const broke = state.broke.includes(one);
      if (!live && !broke) continue;
      const [x, y] = at(one);
      const working = live && state.work[one] > 0;
      const phase = (view.t / TRIP + hash(one, 4)) % 1;
      const out = working && !view.still ? Math.sin(phase * Math.PI) * r * 0.9 : 0;
      const [bx, by] = g.benches[state.runner[one]];
      const d = Math.max(1e-6, Math.hypot(x - bx, y - by));
      const body = live ? (working ? accent : blend(accent, paper, 0.4)) : blend(rest, paper, 0.3);
      ctx.globalAlpha = broke ? base * (1 - k) : base;
      robot(
        x + ((x - bx) / d) * out,
        y + ((y - by) / d) * out,
        r * 0.9,
        body,
        ink,
        working && phase < 0.5 ? shine : null,
      );
    }
    ctx.globalAlpha = base;
    if (k < 1)
      state.flows.forEach((_, i) => {
        if (i % 3) return;
        const [one, to, n] = [state.flows[i], state.flows[i + 1], state.flows[i + 2]];
        const a = at(one);
        const b = g.benches[to];
        const c = bend(a, b, state.credit === "all" ? one + i : one);
        for (let j = 0; j < n; j++) {
          const e = ease(Math.min(1, Math.max(0, k * 1.25 - j * 0.15)));
          if (e <= 0 || e >= 1) continue;
          for (let tail = COMET; tail > 0; tail--) {
            const [tx, ty] = along(a, c, b, Math.max(0, e - tail * 0.035));
            spark(tx, ty, r * 0.16 * (1 - tail / (COMET + 1)), shine, 0.5 * (1 - tail / (COMET + 1)));
          }
          const [x, y] = along(a, c, b, e);
          spark(x, y, r * 0.26, shine, 1);
        }
      });
    ctx.restore();
    if (g.side) {
      ctx.fillStyle = veil(ink, 0.12);
      ctx.fillRect(g.side, 0, Math.max(1, g.px * 0.08), view.h);
    }
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    state = next;
    trail.unshift(state.flows);
    trail.length = Math.min(trail.length, TRAIL);
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
