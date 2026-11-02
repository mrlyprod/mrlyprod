import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Answers, Over, Report, State } from "./engine.ts";
import {
  deal as dealing,
  IDLE,
  INVENT,
  level,
  NONE,
  report,
  step,
  TENTHS,
  TOLD,
  tick,
  UNDERSTOOD,
  UNKNOWN,
  WORLDS,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  answers: Answers;
  pupils: number;
  tint: string;
};

export type Told = Report & {
  day: number;
  length: number;
  pupils: number;
  answers: Answers;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type School = ReturnType<typeof make>;

type Spot = [number, number];

export const SPEC: Spec = [
  {
    key: "answers",
    label: "Answers",
    kind: "segment",
    def: "wiki",
    options: [
      ["off", "None"],
      ["wiki", "A wiki"],
      ["oracle", "An oracle"],
    ],
    group: "World",
  },
  { key: "pupils", label: "Pupils", kind: "slider", def: 16, min: 8, max: 32, step: 4, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A school of bot pupils grows a tree of puzzles. The root is what every pupil already knows; each branch is a puzzle that builds on the one below it. A run is a hundred and twenty days and replays exactly from the seed and the knobs.",
  "Each day a pupil works on a puzzle whose parent it knows. Working it out takes tries, more for deeper puzzles and fewer for a pupil who already understands a lot. A stuck pupil asks for help: a classmate who understands the puzzle teaches it, and the pupil understands it too. When nothing is left to learn, a pupil who understands a puzzle invents a new one on top of it, and nobody knows its answer yet.",
  "The knob is how far the answers are. None: only classmates can help. A wiki: once any pupil has worked a puzzle out, its answer is written down, and a stuck pupil reads it. An oracle: every answer, even to a puzzle invented this morning, is one question away. A pupil told an answer knows it but has not worked it out, and only what a pupil understands lets it invent; with nothing else to do it works a told answer out, at half the tries.",
  "Skill is the puzzles a pupil understands, on average; asked the questions put to the wiki or the oracle, taught the lessons between classmates, invented the new puzzles, open the puzzles nobody can answer yet. Enter steps a day, Space pauses, Reroll draws another school. Bots are not children: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `school-${value.answers}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New school", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "yard",
    title: "Schoolyard",
    still: 10000,
    pick: (rand: Rng) => ({
      answers: pick(rand, WORLDS),
      pupils: pick(rand, [12, 16, 16, 20]),
    }),
  },
  {
    id: "tree",
    title: "Tree of questions",
    still: 10000,
    pick: (rand: Rng) => ({
      answers: pick(rand, WORLDS),
      pupils: pick(rand, [24, 28, 32]),
    }),
  },
];

const PACE = 250;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 100;
const WIDTH = 0.3;
const LETTERS = 20;
const LEAD = 1.45;
const DEEP = 4;
const SPREAD = 0.12;
const GOLDEN = 2.39996;
const BOB = 900;
const SWAY = 9000;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["answers", "pupils"];

export const HUES = [
  palette.red,
  palette.orange,
  palette.yellow,
  palette.green,
  palette.mint,
  palette.teal,
  palette.cyan,
  palette.blue,
  palette.indigo,
  palette.purple,
  palette.pink,
  palette.brown,
];

export const SIGNS: string[] = [
  "a puzzle nobody can answer yet",
  "a puzzle answered, not understood",
  "a puzzle a pupil understands",
  "a puzzle a pupil invented",
  "a pupil, at work",
  "a lesson between classmates",
  "a question to the wiki or oracle",
];

const NAMES: Record<Answers, string> = { off: "No answers", wiki: "A wiki", oracle: "An oracle" };

const SAYS: Record<Answers, string> = {
  off: "only classmates can help",
  wiki: "what one pupil works out, all can read",
  oracle: "every answer is one question away",
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export const hue = (pupil: number) => HUES[(pupil * 5) % HUES.length];

export function angles(parent: number[]): number[] {
  const kids: number[][] = parent.map(() => []);
  for (let p = 1; p < parent.length; p++) kids[parent[p]].push(p);
  const weight: number[] = Array(parent.length).fill(0);
  for (let p = parent.length - 1; p >= 0; p--) {
    weight[p] = Math.max(1, weight[p]);
    if (p > 0) weight[parent[p]] += weight[p];
  }
  const out: number[] = Array(parent.length).fill(0.5);
  const lay = (p: number, from: number, to: number) => {
    out[p] = (from + to) / 2;
    let at = from;
    for (const c of kids[p]) {
      const span = ((to - from) * weight[c]) / weight[p];
      lay(c, at, at + span);
      at += span;
    }
  };
  lay(0, 0, 1);
  return out;
}

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
  let was: Spot[] = [];
  let seats: Spot[] = [];
  let before: State;
  const landing: (() => void)[] = [];
  const lock = Boolean(view.lock);
  const tell = () => {
    told = report(state);
    if (gone) return;
    opts.onTell?.({
      ...told,
      day: state.day,
      length: state.length,
      pupils: state.pupils,
      answers: state.answers,
      over: state.over,
    });
  };
  const world = () => (lock ? WORLDS[(WORLDS.indexOf(value.answers) + round) % WORLDS.length] : value.answers);
  const deal = () => {
    state = dealing({ ...value, answers: world() }, { seed, round });
    if (lock && view.still) for (let k = 0; k < STILL && !state.over; k++) state = step(state, tick);
    before = state;
    was = [];
    seats = [];
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
  const stage = () => {
    const { px, side } = type();
    const left = side + px * 2;
    const right = view.w - px * 2;
    const cx = (left + right) / 2;
    const base = view.h - px * 3.2;
    const rx = Math.max(0, (right - left) / 2);
    const ry = Math.max(0, base - px * 3);
    const node = Math.max(1, Math.min(px * 0.62, Math.min(rx, ry) * 0.035));
    return { px, side, left, right, cx, base, rx, ry, node };
  };
  type Stage = ReturnType<typeof stage>;
  const layout = (one: State, g: Stage): Spot[] => {
    const deep = Math.max(DEEP, ...one.depth);
    const sway = view.still ? 0 : Math.sin((view.t / SWAY) * TAU) * 0.012;
    return angles(one.parent).map((u, p) => {
      const f = one.depth[p] / deep;
      const a = Math.PI * (1 + SPREAD + (1 - 2 * SPREAD) * u) + sway * f;
      return [g.cx + g.rx * f * Math.cos(a), g.base + g.ry * f * Math.sin(a)];
    });
  };
  const seated = (one: State, spots: Spot[], g: Stage): Spot[] =>
    one.target.map((p, b) => {
      if (one.doing[b] === IDLE) {
        const x = g.cx + (b - (one.pupils - 1) / 2) * g.node * 2.2;
        return [x, g.base + g.px * 1.6];
      }
      const [x, y] = spots[p] ?? spots[0];
      const r = g.node * (one.doing[b] === INVENT ? 3.4 : 2.4);
      const a = b * GOLDEN;
      return [x + Math.cos(a) * r, y + Math.sin(a) * r];
    });
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const look = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    return {
      paper,
      accent,
      ink,
      day,
      lit: day ? blend(palette.orange, palette.yellow, 0.45) : blend(palette.yellow, palette.white, 0.15),
      wood: blend(palette.brown, paper, day ? 0.15 : 0.3),
      leaf: blend(palette.green, paper, day ? 0.1 : 0.25),
      told: day ? blend(accent, paper, 0.15) : blend(accent, palette.white, 0.25),
    };
  };
  type Look = ReturnType<typeof look>;
  const puzzle = (x: number, y: number, r: number, seen: number, by: number, c: Look, share: number) => {
    if (seen === UNDERSTOOD && share > 0) {
      ctx.fillStyle = veil(c.lit, 0.08 + 0.2 * share);
      ctx.beginPath();
      ctx.arc(x, y, r * (1.3 + share), 0, TAU);
      ctx.fill();
    }
    ctx.fillStyle =
      seen === UNDERSTOOD
        ? blend(c.lit, c.paper, 0.7 * (1 - share))
        : seen === TOLD
          ? blend(c.told, c.paper, 0.55)
          : c.paper;
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = seen === UNDERSTOOD ? veil(c.ink, 0.25) : seen === TOLD ? c.told : veil(c.ink, 0.4);
    ctx.lineWidth = Math.max(1, r * 0.28);
    ctx.stroke();
    if (by === NONE) return;
    ctx.fillStyle = hue(by);
    ctx.beginPath();
    ctx.arc(x, y, r * 0.42, 0, TAU);
    ctx.fill();
  };
  const body = (x: number, y: number, r: number, color: string, ink: string) => {
    ctx.fillStyle = veil(palette.black, 0.2);
    ctx.beginPath();
    ctx.ellipse(x + r * 0.2, y + r * 0.7, r * 0.9, r * 0.4, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.4);
    ctx.lineWidth = Math.max(1, r * 0.18);
    ctx.stroke();
    ctx.fillStyle = veil(palette.white, 0.7);
    ctx.beginPath();
    ctx.arc(x - r * 0.32, y - r * 0.32, r * 0.3, 0, TAU);
    ctx.fill();
  };
  const thread = (a: Spot, b: Spot, color: string, width: number) => {
    const [mx, my] = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2 - Math.hypot(b[0] - a[0], b[1] - a[1]) * 0.35];
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.moveTo(a[0], a[1]);
    ctx.quadraticCurveTo(mx, my, b[0], b[1]);
    ctx.stroke();
    return (k: number): Spot => [
      (1 - k) ** 2 * a[0] + 2 * (1 - k) * k * mx + k * k * b[0],
      (1 - k) ** 2 * a[1] + 2 * (1 - k) * k * my + k * k * b[1],
    ];
  };
  const mark = (x: number, y: number, size: number, color: string, alpha: number) => {
    ctx.font = `600 ${size}px ${TYPE}`;
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillStyle = veil(color, alpha);
    ctx.fillText("?", x, y);
    ctx.textAlign = "left";
  };
  const source = (g: Stage, c: Look, alpha: number): Spot | null => {
    if (state.answers === "off") return null;
    const r = g.px * 1.1;
    const x = g.right - r;
    const y = g.px * 1.5 + r;
    ctx.fillStyle = veil(c.told, 0.18 * alpha);
    ctx.beginPath();
    ctx.arc(x, y, r * 2, 0, TAU);
    ctx.fill();
    ctx.fillStyle = veil(c.told, alpha);
    ctx.beginPath();
    if (state.answers === "wiki") ctx.roundRect(x - r * 0.8, y - r, r * 1.6, r * 2, r * 0.25);
    else ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(c.paper, 0.8 * alpha);
    ctx.lineWidth = Math.max(1, r * 0.14);
    ctx.beginPath();
    if (state.answers === "wiki")
      for (const k of [-0.45, 0, 0.45]) {
        ctx.moveTo(x - r * 0.45, y + k * r);
        ctx.lineTo(x + r * 0.45, y + k * r);
      }
    else ctx.arc(x, y, r * 0.45, 0, TAU);
    ctx.stroke();
    return [x, y];
  };
  const legend = (c: Look) => {
    const { px, side } = type();
    if (!side) return;
    const lines = 2 + SIGNS.length + 2 + 6 * 2;
    const x = px * 1.6;
    let y = view.h / 2 - (lines * px * LEAD) / 2;
    const say = (text: string, size: number, color: string, weight = "") => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    const row = (text: string) => {
      ctx.font = `${px * 0.95}px ${TYPE}`;
      ctx.fillStyle = veil(c.ink, 0.8);
      ctx.fillText(text, x + px * 1.8, y);
      y += px * LEAD;
    };
    ctx.textBaseline = "middle";
    ctx.textAlign = "left";
    say(NAMES[state.answers], px * 1.4, c.ink, "600");
    y += px * LEAD * 1.1;
    say(SAYS[state.answers], px * 0.95, veil(c.ink, 0.65));
    y += px * LEAD * 1.4;
    const r = px * 0.36;
    puzzle(x + px * 0.6, y, r, UNKNOWN, NONE, c, 0);
    row(SIGNS[0]);
    puzzle(x + px * 0.6, y, r, TOLD, NONE, c, 0);
    row(SIGNS[1]);
    puzzle(x + px * 0.6, y, r, UNDERSTOOD, NONE, c, 1);
    row(SIGNS[2]);
    puzzle(x + px * 0.6, y, r, UNDERSTOOD, 3, c, 1);
    row(SIGNS[3]);
    body(x + px * 0.6, y, px * 0.36, hue(4), c.ink);
    row(SIGNS[4]);
    thread([x, y + px * 0.2], [x + px * 1.2, y + px * 0.2], veil(hue(7), 0.8), Math.max(1, px * 0.12));
    row(SIGNS[5]);
    mark(x + px * 0.6, y, px * 1.1, c.told, 1);
    row(SIGNS[6]);
    y += px * 0.9;
    const tenths = (n: number) => `${Math.floor(n / TENTHS)}.${n % TENTHS}`;
    const rows: [string, string][] = [
      [`${tenths(told.skill)} skill`, "puzzles each pupil understands"],
      [`${told.asked} asked`, "questions to the wiki or oracle"],
      [`${told.taught} taught`, "lessons between classmates"],
      [`${told.invented} invented`, "new puzzles the pupils made"],
      [`${told.open} open`, "puzzles nobody can answer yet"],
      [`day ${state.day}`, `of ${state.length}, ${told.puzzles} puzzles`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, c.ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(c.ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const paint = () => {
    const c = look();
    const g = stage();
    const k = view.still || state.over ? 1 : ease((view.t - acted) / PACE);
    const spots = layout(state, g);
    const from = layout(before, g);
    const at = spots.map((to, p): Spot => {
      const old = from[p] ?? from[state.parent[p]] ?? to;
      return [old[0] + (to[0] - old[0]) * k, old[1] + (to[1] - old[1]) * k];
    });
    const now = seated(state, at, g);
    const sit = now.map((to, b): Spot => {
      const old = was[b] ?? to;
      return [old[0] + (to[0] - old[0]) * k, old[1] + (to[1] - old[1]) * k];
    });
    seats = sit;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const sky = ctx.createLinearGradient(0, 0, 0, view.h);
    sky.addColorStop(0, c.day ? blend(c.paper, palette.cyan, 0.1) : blend(c.paper, palette.indigo, 0.12));
    sky.addColorStop(0.75, c.day ? blend(c.paper, palette.yellow, 0.06) : blend(c.paper, palette.blue, 0.05));
    sky.addColorStop(1, c.day ? blend(c.paper, palette.green, 0.18) : blend(c.paper, palette.green, 0.1));
    ctx.fillStyle = sky;
    ctx.fillRect(0, 0, view.w, view.h);
    ctx.fillStyle = c.day ? blend(c.paper, palette.green, 0.28) : blend(c.paper, palette.green, 0.16);
    ctx.beginPath();
    ctx.ellipse(g.cx, view.h + g.px * 0.5, Math.max(0, g.rx * 0.9), g.px * 5, 0, 0, TAU);
    ctx.fill();
    legend(c);
    const tops = level(state);
    const deep = Math.max(DEEP, ...state.depth);
    for (let p = state.parent.length - 1; p > 0; p--) {
      const [x, y] = at[p];
      const [px0, py0] = at[state.parent[p]];
      const grow = state.posed.includes(p) ? k : 1;
      const tx = px0 + (x - px0) * grow;
      const ty = py0 + (y - py0) * grow;
      const seen = tops[p];
      ctx.strokeStyle = seen === UNDERSTOOD ? c.wood : seen === TOLD ? veil(c.told, 0.7) : veil(c.ink, 0.25);
      ctx.lineWidth = Math.max(1, g.node * (seen === UNDERSTOOD ? 0.9 : 0.4) * (1.6 - state.depth[p] / deep));
      ctx.lineCap = "round";
      ctx.setLineDash(seen === UNKNOWN ? [g.node * 0.5, g.node * 0.6] : []);
      ctx.beginPath();
      ctx.moveTo(px0, py0);
      ctx.quadraticCurveTo(px0 + (tx - px0) * 0.15, ty + (py0 - ty) * 0.35, tx, ty);
      ctx.stroke();
    }
    ctx.setLineDash([]);
    for (let p = 0; p < state.parent.length; p++) {
      const grow = state.posed.includes(p) ? k : 1;
      const [x, y] = at[p];
      const [px0, py0] = at[state.parent[p]];
      let crowd = 0;
      for (let b = 0; b < state.pupils; b++) if (state.know[p * state.pupils + b] === UNDERSTOOD) crowd++;
      const r = g.node * (p === 0 ? 1.8 : 0.8 + (0.6 * crowd) / state.pupils) * grow;
      puzzle(px0 + (x - px0) * grow, py0 + (y - py0) * grow, r, tops[p], state.by[p], c, crowd / state.pupils);
    }
    for (let i = 0; i < state.solves.length; i += 2) {
      const p = state.solves[i + 1];
      if (state.first[p] !== state.day) continue;
      const [x, y] = at[p];
      ctx.strokeStyle = veil(c.lit, 0.8 * (1 - k));
      ctx.lineWidth = Math.max(1, g.node * 0.3);
      ctx.beginPath();
      ctx.arc(x, y, g.node * (1 + 3 * k), 0, TAU);
      ctx.stroke();
    }
    const spring = source(g, c, 1);
    for (let i = 0; i < state.lessons.length; i += 2) {
      const a = sit[state.lessons[i]];
      const b = sit[state.lessons[i + 1]];
      const along = thread(a, b, veil(hue(state.lessons[i]), 0.75 * (1 - k * 0.6)), Math.max(1, g.node * 0.22));
      const [x, y] = along(k);
      ctx.fillStyle = hue(state.lessons[i]);
      ctx.beginPath();
      ctx.arc(x, y, g.node * 0.4, 0, TAU);
      ctx.fill();
    }
    for (const b of state.asks) {
      const [x, y] = sit[b];
      if (spring) {
        ctx.strokeStyle = veil(c.told, 0.35 * (1 - k));
        ctx.lineWidth = Math.max(1, g.node * 0.15);
        ctx.beginPath();
        ctx.moveTo(spring[0], spring[1]);
        ctx.lineTo(x, y);
        ctx.stroke();
      }
      mark(x, y - g.node * (2.4 + 1.5 * k), g.node * 2.6, c.told, 1 - k * 0.7);
    }
    for (let b = 0; b < state.pupils; b++) {
      const [x, y] = sit[b];
      const bob = view.still ? 0 : Math.sin((view.t / BOB + hash(b, 3)) * TAU) * g.node * 0.25;
      body(x, y + bob, g.node * 0.9, hue(b), c.ink);
    }
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    const g = stage();
    was = seats.length ? seats : seated(state, layout(state, g), g);
    before = state;
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
    size: () => {
      was = [];
      paint();
    },
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
