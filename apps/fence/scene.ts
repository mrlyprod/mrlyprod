import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Over, Prison, Report, State } from "./engine.ts";
import {
  BED,
  BEDS,
  deal as dealing,
  FENCE,
  GARDEN,
  GATE,
  MAP,
  MEND,
  NEXT,
  PRISONS,
  QUIET,
  ROOMS,
  report,
  ripe,
  step,
  TREES,
  tick,
  WORK,
  WORST,
  quiet as yard,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  prison: Prison;
  scarcity: number;
  bots: number;
  tint: string;
};

export type Told = Report & {
  hour: number;
  length: number;
  prison: Prison;
  scarcity: number;
  bots: number;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Fence = ReturnType<typeof make>;

export const SCARCE = ["none", "a little", "some", "much", "dire"];

export const SPEC: Spec = [
  {
    key: "prison",
    label: "After a break",
    kind: "segment",
    def: "removal",
    options: [
      ["none", "Nothing"],
      ["removal", "Removal"],
      ["repair", "Repair"],
    ],
    group: "World",
  },
  {
    key: "scarcity",
    label: "Scarcity",
    kind: "slider",
    def: 2,
    min: 0,
    max: WORST,
    step: 1,
    show: (v) => SCARCE[v] ?? String(v),
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 24, min: 12, max: 36, step: 4, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A village of bots shares one garden behind a fence. Each bot walks to the garden when it gets hungry, waits for a plant to ripen and goes home to eat. A run is two hundred and forty hours and replays exactly from the seed and the knobs.",
  "Now and then a bot breaks the rule: it knocks down a piece of the fence and grabs a plant. A calm bot almost never does; a starving one often does, and one without a home a little more often. Behind a broken piece the nearest plant stops growing until the fence stands again.",
  "The knob is what follows a break. Nothing: the village patches the fence on its own after sixty hours. Removal: the bot spends forty hours in the quiet house, and if a bot was waiting for a home it gets the empty one. Repair: the bot and two neighbours mend the fence and replant the bed together, and a neighbour with food to spare shares some.",
  "Breaks counts the fences broken, repeat the breaks by a bot that broke before, repaired the fences mended by whoever broke them, removed the hours spent in the quiet house, trust a number that falls at each break, climbs back an hour at a time, and jumps at each repair. Scarcity slows the garden and leaves bots without a home. These are this toy's rules, not findings: bots are not people, and a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `fence-${value.prison}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New village", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "village",
    title: "Quiet village",
    still: 10000,
    pick: (rand: Rng) => ({
      prison: pick(rand, PRISONS),
      scarcity: pick(rand, [0, 1, 1, 2]),
      bots: pick(rand, [20, 24, 28]),
    }),
  },
  {
    id: "lean",
    title: "Lean years",
    still: 10000,
    pick: (rand: Rng) => ({
      prison: pick(rand, PRISONS),
      scarcity: pick(rand, [3, 4]),
      bots: pick(rand, [28, 32, 36]),
    }),
  },
];

const PACE = 150;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 150;
const WIDTH = 0.28;
const LETTERS = 20;
const LEAD = 1.5;
const BOB = 1300;
const PUFF = 6;
const TUFTS = 90;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["prison", "scarcity", "bots"];

export const TITLES: Record<Prison, string> = {
  none: "Nothing follows",
  removal: "A while in the quiet house",
  repair: "Mend it with neighbours",
};

export const SIGNS: string[] = ["a bot, at home or on its way", "a hungry bot", "a broken fence", "the quiet house"];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export const hues = (accent: string, day: boolean) => ({
  bot: accent,
  wood: blend(palette.brown, day ? palette.black : palette.white, day ? 0.15 : 0.1),
  soil: palette.brown,
  leaf: palette.green,
  fruit: blend(palette.red, palette.orange, 0.3),
  quiet: palette.indigo,
});

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
      prison: state.prison,
      scarcity: state.scarcity,
      bots: state.bots,
      over: state.over,
    });
  };
  const rule = (): Prison => (lock ? PRISONS[(PRISONS.indexOf(value.prison) + round) % PRISONS.length] : value.prison);
  const deal = () => {
    state = dealing({ ...value, prison: rule() }, { seed, round });
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
    const top = pad;
    const wide = Math.max(1, view.w - pad - left);
    const tall = Math.max(1, view.h - pad - top);
    const X = (mx: number) => left + (mx / MAP) * wide;
    const Y = (my: number) => top + (my / MAP) * tall;
    const r = Math.max(1, Math.min(px * 0.75, Math.min(wide, tall) / 50));
    return { px, side, X, Y, r, wide, tall };
  };
  type Geometry = ReturnType<typeof geometry>;
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const house = (x: number, y: number, s: number, fill: string, ink: string) => {
    ctx.fillStyle = veil(palette.black, 0.16);
    ctx.beginPath();
    ctx.ellipse(x + s * 0.15, y + s * 0.95, s * 1.1, s * 0.35, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = fill;
    ctx.strokeStyle = veil(ink, 0.4);
    ctx.lineWidth = Math.max(1, s * 0.12);
    ctx.lineJoin = "round";
    ctx.beginPath();
    ctx.moveTo(x - s * 0.8, y + s * 0.8);
    ctx.lineTo(x - s * 0.8, y - s * 0.1);
    ctx.lineTo(x, y - s * 0.9);
    ctx.lineTo(x + s * 0.8, y - s * 0.1);
    ctx.lineTo(x + s * 0.8, y + s * 0.8);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = veil(ink, 0.35);
    ctx.beginPath();
    ctx.roundRect(x - s * 0.18, y + s * 0.25, s * 0.36, s * 0.55, s * 0.12);
    ctx.fill();
  };
  const tree = (x: number, y: number, s: number, leaf: string, wood: string, paper: string) => {
    ctx.fillStyle = veil(palette.black, 0.14);
    ctx.beginPath();
    ctx.ellipse(x + s * 0.2, y + s * 1.05, s * 1.1, s * 0.32, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = wood;
    ctx.fillRect(x - s * 0.12, y, s * 0.24, s);
    ctx.fillStyle = blend(leaf, paper, 0.15);
    for (const [dx, dy, k] of [
      [-0.45, -0.1, 0.6],
      [0.45, -0.1, 0.6],
      [0, -0.5, 0.75],
    ]) {
      ctx.beginPath();
      ctx.arc(x + s * dx, y + s * dy, s * k, 0, TAU);
      ctx.fill();
    }
  };
  const bot = (x: number, y: number, r: number, fill: string, ink: string, hungry: boolean) => {
    ctx.fillStyle = veil(palette.black, 0.18);
    ctx.beginPath();
    ctx.ellipse(x + r * 0.15, y + r * 0.85, r * 0.85, r * 0.3, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = fill;
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.35);
    ctx.lineWidth = Math.max(1, r * 0.12);
    ctx.stroke();
    ctx.fillStyle = veil(palette.white, 0.35);
    ctx.beginPath();
    ctx.arc(x - r * 0.35, y - r * 0.4, r * 0.28, 0, TAU);
    ctx.fill();
    ctx.fillStyle = blend(palette.black, fill, 0.25);
    const eye = r * (hungry ? 0.1 : 0.14);
    for (const side of [-1, 1]) {
      ctx.beginPath();
      ctx.arc(x + side * r * 0.32, y + (hungry ? r * 0.12 : 0), eye, 0, TAU);
      ctx.fill();
    }
  };
  const rails = (g: Geometry, seg: number, up: number, wood: string, ink: string) => {
    const [ax, ay, bx, by] = FENCE[seg];
    const [x0, y0, x1, y1] = [g.X(ax), g.Y(ay), g.X(bx), g.Y(by)];
    const post = Math.max(1, g.r * 0.32);
    ctx.lineCap = "round";
    ctx.strokeStyle = wood;
    ctx.lineWidth = Math.max(1, g.r * 0.26);
    const flat = ay === by;
    for (const lift of [0.35, 0.85]) {
      const h = g.r * lift * (flat ? 1 : 0);
      if (up >= 1) {
        ctx.beginPath();
        ctx.moveTo(x0, y0 - h);
        ctx.lineTo(x1, y1 - h);
        ctx.stroke();
        continue;
      }
      const mx = (x0 + x1) / 2;
      const my = (y0 + y1) / 2;
      const tilt = (1 - up) * (lift > 0.5 ? 0.45 : -0.3);
      const dx = (x1 - x0) / 2;
      const dy = (y1 - y0) / 2;
      const cos = Math.cos(tilt);
      const sin = Math.sin(tilt);
      const fall = (1 - up) * g.r * 1.4;
      const ex = dx * cos - dy * sin;
      const ey = dx * sin + dy * cos;
      ctx.beginPath();
      ctx.moveTo(mx - ex * 0.75, my - ey * 0.75 - h * up + fall);
      ctx.lineTo(mx + ex * 0.75, my + ey * 0.75 - h * up + fall);
      ctx.stroke();
    }
    ctx.fillStyle = blend(wood, ink, 0.15);
    for (const [px, py] of [
      [x0, y0],
      [x1, y1],
    ]) {
      ctx.beginPath();
      ctx.roundRect(px - post, py - g.r * (flat ? 1.1 : 0.4), post * 2, g.r * (flat ? 1.3 : 0.8), post * 0.6);
      ctx.fill();
    }
  };
  const plant = (x: number, y: number, s: number, k: number, shut: boolean, c: ReturnType<typeof hues>) => {
    const leaf = shut ? blend(c.leaf, c.soil, 0.55) : c.leaf;
    const tall = s * (0.25 + 0.75 * k);
    ctx.strokeStyle = leaf;
    ctx.lineCap = "round";
    ctx.lineWidth = Math.max(1, s * 0.14);
    ctx.beginPath();
    ctx.moveTo(x, y);
    ctx.lineTo(x, y - tall);
    ctx.stroke();
    ctx.fillStyle = leaf;
    for (const side of [-1, 1]) {
      ctx.beginPath();
      ctx.ellipse(x + side * tall * 0.35, y - tall * 0.55, tall * 0.32, tall * 0.16, side * 0.5, 0, TAU);
      ctx.fill();
    }
    if (k < 1) return;
    ctx.fillStyle = c.fruit;
    ctx.beginPath();
    ctx.arc(x, y - tall, s * 0.3, 0, TAU);
    ctx.fill();
    ctx.fillStyle = veil(palette.white, 0.45);
    ctx.beginPath();
    ctx.arc(x - s * 0.09, y - tall - s * 0.09, s * 0.09, 0, TAU);
    ctx.fill();
  };
  const quiet = (g: Geometry, c: ReturnType<typeof hues>, ink: string, paper: string) => {
    const x = g.X(QUIET.x + QUIET.dx);
    const y = g.Y(QUIET.y + QUIET.dy);
    const s = g.r * 2.1;
    const spots = Array.from({ length: ROOMS }, (_, i) => yard(i));
    const rx = Math.max(...spots.map(([sx]) => Math.abs(g.X(sx) - x))) + g.r * 1.6;
    const ry = Math.max(...spots.map(([, sy]) => Math.abs(g.Y(sy) - y))) + g.r * 1.6;
    ctx.fillStyle = blend(paper, c.quiet, 0.14);
    ctx.beginPath();
    ctx.ellipse(x, y, rx, ry, 0, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(c.quiet, 0.5);
    ctx.lineWidth = Math.max(1, g.r * 0.18);
    ctx.setLineDash([g.r * 0.3, g.r * 0.35]);
    ctx.stroke();
    ctx.setLineDash([]);
    house(x, y - ry, s, blend(c.quiet, paper, 0.35), ink);
  };
  const legend = (ink: string, paper: string, c: ReturnType<typeof hues>) => {
    const { px, side } = type();
    if (!side) return;
    const lines = 2 + SIGNS.length + 2 + 7 * 2;
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
    say(TITLES[state.prison], px * 1.3, ink, "600");
    y += px * LEAD * 1.1;
    say(`scarcity ${SCARCE[state.scarcity]}, ${state.bots} bots`, px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    bot(x + px * 0.6, y, px * 0.42, c.bot, ink, false);
    row(SIGNS[0]);
    bot(x + px * 0.6, y, px * 0.42, blend(c.bot, paper, 0.55), ink, true);
    row(SIGNS[1]);
    ctx.strokeStyle = c.wood;
    ctx.lineWidth = Math.max(1, px * 0.14);
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.moveTo(x, y - px * 0.3);
    ctx.lineTo(x + px * 1.2, y + px * 0.25);
    ctx.moveTo(x + px * 0.1, y + px * 0.3);
    ctx.lineTo(x + px * 1.1, y - px * 0.05);
    ctx.stroke();
    row(SIGNS[2]);
    house(x + px * 0.6, y, px * 0.45, blend(c.quiet, paper, 0.35), ink);
    row(SIGNS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${told.breaks} ${told.breaks === 1 ? "break" : "breaks"}`, "pieces of fence knocked down"],
      [`${told.repeats} repeat`, "breaks by a bot that broke before"],
      [`${told.repaired} repaired`, "mended by whoever broke them"],
      [`${told.lost} hours removed`, "spent in the quiet house"],
      [`${Math.floor(told.trust / 10)}% trust`, "falls at a break, mends slowly"],
      [`${told.desperate} desperate`, "starving or without a home now"],
      [`hour ${state.hour}`, `of ${state.length}`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.15, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.1;
    }
  };
  const ground = (g: Geometry, c: ReturnType<typeof hues>, paper: string, day: boolean) => {
    ctx.fillStyle = blend(paper, c.leaf, day ? 0.12 : 0.08);
    ctx.fillRect(g.side, 0, Math.max(0, view.w - g.side), view.h);
    ctx.strokeStyle = veil(c.leaf, day ? 0.35 : 0.3);
    ctx.lineWidth = Math.max(1, g.r * 0.12);
    ctx.lineCap = "round";
    const tuft = g.r * 0.5;
    ctx.beginPath();
    for (let i = 0; i < TUFTS; i++) {
      const x = g.side + hash(i, 1) * (view.w - g.side);
      const y = hash(i, 2) * view.h;
      ctx.moveTo(x - tuft * 0.5, y - tuft);
      ctx.lineTo(x, y);
      ctx.lineTo(x + tuft * 0.5, y - tuft);
    }
    ctx.stroke();
    const [x0, y0, x1, y1] = [g.X(GARDEN.x0), g.Y(GARDEN.y0), g.X(GARDEN.x1), g.Y(GARDEN.y1)];
    ctx.strokeStyle = veil(c.soil, day ? 0.18 : 0.22);
    ctx.lineWidth = Math.max(1, g.r * 1.6);
    ctx.beginPath();
    ctx.moveTo(g.X(GATE.x), y1);
    ctx.lineTo(g.X(GATE.x), view.h);
    ctx.stroke();
    ctx.fillStyle = blend(paper, c.soil, day ? 0.3 : 0.26);
    ctx.beginPath();
    ctx.roundRect(x0, y0, Math.max(0, x1 - x0), Math.max(0, y1 - y0), g.r * 0.6);
    ctx.fill();
    ctx.fillStyle = blend(paper, c.soil, day ? 0.42 : 0.38);
    const w = ((x1 - x0) / 6) * 0.7;
    BED.forEach(([bx, by]) => {
      ctx.beginPath();
      ctx.ellipse(g.X(bx), g.Y(by) + g.r * 0.3, Math.max(0.5, w / 2), Math.max(0.5, g.r * 0.45), 0, 0, TAU);
      ctx.fill();
    });
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    const c = hues(accent, day);
    const g = geometry();
    const k = view.still || state.over ? 1 : Math.min(1, Math.max(0, (view.t - acted) / PACE));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    ctx.fillStyle = paper;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.save();
    ctx.beginPath();
    ctx.rect(g.side, 0, Math.max(0, view.w - g.side), view.h);
    ctx.clip();
    ground(g, c, paper, day);
    ctx.restore();
    if (g.side) {
      ctx.fillStyle = veil(ink, 0.12);
      ctx.fillRect(g.side, 0, Math.max(1, g.px * 0.08), view.h);
    }
    legend(ink, paper, c);
    const r = g.r;
    const top = ripe(state.scarcity);
    const shut = new Set(FENCE.flatMap((_, seg) => (state.whole[seg] ? [] : [NEXT[seg]])));
    for (let bed = 0; bed < BEDS; bed++) {
      const grown =
        prev.grow[bed] <= state.grow[bed] ? prev.grow[bed] + (state.grow[bed] - prev.grow[bed]) * k : state.grow[bed];
      plant(g.X(BED[bed][0]), g.Y(BED[bed][1]) + r * 0.3, r * 1.6, Math.min(1, grown / top), shut.has(bed), c);
    }
    FENCE.forEach((_, seg) => {
      const up = state.whole[seg] ? (prev.whole[seg] ? 1 : ease(k)) : state.work[seg] / WORK;
      rails(g, seg, up, c.wood, ink);
    });
    for (let h = 0; h < state.hx.length; h++)
      house(g.X(state.hx[h]), g.Y(state.hy[h]), r * 1.6, blend(c.bot, paper, day ? 0.55 : 0.3), ink);
    for (let t = 0; t < TREES; t++) tree(g.X(state.rx[t]), g.Y(state.ry[t]), r * 1.4, c.leaf, c.wood, paper);
    quiet(g, c, ink, paper);
    if (k < 1) {
      state.broken.forEach((seg, n) => {
        const [ax, ay, bx, by] = FENCE[seg];
        for (let s = 0; s < PUFF; s++) {
          const a = hash(n, s) * TAU;
          const d = r * (0.8 + 2.4 * ease(k));
          ctx.fillStyle = veil(c.soil, 0.55 * (1 - k));
          ctx.beginPath();
          ctx.arc(g.X((ax + bx) / 2) + Math.cos(a) * d, g.Y((ay + by) / 2) + Math.sin(a) * d, r * 0.3, 0, TAU);
          ctx.fill();
        }
      });
      for (const seg of state.mended) {
        const [ax, ay, bx, by] = FENCE[seg];
        const x = g.X((ax + bx) / 2);
        const y = g.Y((ay + by) / 2) - r * (1 + 2.5 * ease(k));
        ctx.fillStyle = veil(c.leaf, 0.8 * (1 - k));
        ctx.beginPath();
        ctx.arc(x - r * 0.28, y, r * 0.32, 0, TAU);
        ctx.arc(x + r * 0.28, y, r * 0.32, 0, TAU);
        ctx.fill();
        ctx.beginPath();
        ctx.moveTo(x - r * 0.58, y + r * 0.1);
        ctx.lineTo(x, y + r * 0.7);
        ctx.lineTo(x + r * 0.58, y + r * 0.1);
        ctx.fill();
      }
      for (const bed of state.ate) {
        ctx.strokeStyle = veil(c.fruit, 0.7 * (1 - k));
        ctx.lineWidth = Math.max(1, r * 0.2);
        ctx.beginPath();
        ctx.arc(g.X(BED[bed][0]), g.Y(BED[bed][1]) - r * 0.6, r * (0.5 + 1.5 * ease(k)), 0, TAU);
        ctx.stroke();
      }
    }
    const order = Array.from({ length: state.bots }, (_, one) => one).sort((a, b) => state.y[a] - state.y[b] || a - b);
    for (const one of order) {
      const x = g.X(prev.x[one] + (state.x[one] - prev.x[one]) * k);
      const y = g.Y(prev.y[one] + (state.y[one] - prev.y[one]) * k);
      const still = prev.x[one] === state.x[one] && prev.y[one] === state.y[one];
      const working = state.kind[one] === MEND && still;
      const beat = working ? BOB / 4 : BOB;
      const bob =
        view.still || !still
          ? 0
          : Math.abs(Math.sin((view.t / beat + hash(one, 5)) * Math.PI)) * r * (working ? 0.35 : 0.12);
      const hungry = state.food[one] === 0;
      bot(x, y - bob, r * 0.85, hungry ? blend(c.bot, paper, 0.55) : c.bot, ink, hungry);
    }
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
