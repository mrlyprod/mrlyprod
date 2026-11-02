import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Mode, Over, Report, State, Wish } from "./engine.ts";
import { deal as dealing, HOURS, MILLE, MODES, report, run, step, tick, WEEK } from "./engine.ts";

/* TYPES */

export type Knobs = { decide: Mode; bots: number; eager: number; patience: number; tint: string };

export type Told = Report & { tick: number; length: number; bots: number; decide: Mode; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Decide = ReturnType<typeof make>;

type Point = [number, number];

export const SPEC: Spec = [
  {
    key: "decide",
    label: "Who decides",
    kind: "segment",
    def: "vote",
    options: [
      ["vote", "Vote"],
      ["ruler", "Ruler"],
      ["lottery", "Lottery"],
    ],
    group: "World",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 5, min: 3, max: 12, step: 1, group: "World" },
  { key: "eager", label: "Wishing", kind: "slider", def: 30, min: 10, max: 100, step: 10, unit: "%", group: "World" },
  {
    key: "patience",
    label: "Patience",
    kind: "slider",
    def: 6,
    min: 1,
    max: 24,
    step: 1,
    unit: "ticks",
    group: "World",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A village of bots, each standing on a clock face at the hour of its taste. Now and then a bot makes a wish, a small dot on the hour it wants, and the village must decide: grant it or refuse it. A bot likes a wish within two hours of its own, and always its own.",
  "Who decides is the knob. Vote: every bot casts one vote an hour, on the oldest wish it has not voted on, and a majority settles it. Ruler: one bot drawn from the seed rules on one wish an hour, oldest first. Lottery: each wish gets a judge drawn at random, and each judge rules on one wish an hour.",
  "A waiting wish drifts toward the square in the middle; when it gets there, its bot has run out of patience and takes it back. A run is a week of hours, 168 ticks, and replays exactly from the seed and the knobs.",
  "Granted is wishes granted out of wishes posted, wait the median ticks from a wish to its decision, reach the bots with at least one wish granted. Enter steps one tick, Space pauses, Reroll draws another village. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `decide-${value.decide}-${value.seed}`,
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
    title: "Village weeks",
    still: 6000,
    pick: (rand: Rng) => ({
      decide: pick(rand, MODES),
      bots: 5 + Math.floor(rand() * 3),
      eager: 30,
      patience: 6 + Math.floor(rand() * 3),
    }),
  },
  {
    id: "town",
    title: "Town weeks",
    still: 6000,
    pick: (rand: Rng) => ({
      decide: pick(rand, MODES),
      bots: 10 + Math.floor(rand() * 3),
      eager: 20,
      patience: 8 + Math.floor(rand() * 5),
    }),
  },
];

const PACE = 520;
const SPIN = 420;
const HOLD = 4000;
const TAIL = 100;
const CATCH = 40;
const RING = 0.36;
const SQUARE = 0.32;
const HUB = 0.2;
const BOT = 0.045;
const CROWD = 0.34;
const SPREAD = 2.5;
const START = 2.2;
const GLOW = 1.45;
const GARLAND = 7;
const WIDTH = 0.28;
const LETTERS = 20;
const LEAD = 1.5;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const FADE = 1400;
const LEAVE = 900;
const DEAL: (keyof Knobs)[] = ["decide", "bots", "eager", "patience"];

export const HUE = [
  palette.blue,
  palette.orange,
  palette.green,
  palette.pink,
  palette.yellow,
  palette.purple,
  palette.teal,
  palette.red,
  palette.indigo,
  palette.mint,
  palette.brown,
  palette.cyan,
];

export const SIGNS: string[] = [
  "a bot, at the hour it likes",
  "a wish, heavier as it waits",
  "a granted wish: a flower",
];

const RULES: Record<Mode, [string, string, string]> = {
  vote: ["Vote", "every bot votes, a majority wins", "a ballot cast, yes or no"],
  ruler: ["Ruler", "one crowned bot rules on all", "the ruler, who nods or shakes"],
  lottery: ["Lottery", "a wheel draws each wish a judge", "the wheel that draws a judge"],
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const lerp = (a: Point, b: Point, k: number): Point => [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k];

const hours = (n: number) => {
  const h = n / MILLE;
  const text = Number.isInteger(h) ? String(h) : h.toFixed(1);
  return `${text} ${h === 1 ? "hour" : "hours"}`;
};

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
      decide: state.decide,
      over: state.over,
    });
  };
  const rule = () => (lock ? MODES[(MODES.indexOf(value.decide) + round) % MODES.length] : value.decide);
  const deal = () => {
    state = dealing({ ...value, decide: rule() }, { seed, round });
    was = state;
    if (lock && view.still) {
      was = run(state, WEEK / 2 - 1);
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
  const geometry = () => {
    const { px, side } = type();
    const s = Math.max(0, Math.min(view.w - side, view.h) - px * 3);
    const center: Point = [side + (view.w - side) / 2, view.h / 2];
    const ring = s * RING;
    const r = Math.min(s * BOT, (s * CROWD) / state.bots);
    const angle = (hour: number) => (hour / HOURS) * TAU - Math.PI / 2;
    const on = (hour: number, radius: number): Point => [
      center[0] + Math.cos(angle(hour)) * radius,
      center[1] + Math.sin(angle(hour)) * radius,
    ];
    const spots = state.hours.map((hour, bot) => {
      const mates = state.hours.flatMap((one, i) => (one === hour ? [i] : []));
      const k = mates.indexOf(bot) - (mates.length - 1) / 2;
      return on(hour, ring + k * SPREAD * r);
    });
    const from = (wish: Wish): Point => (ring ? lerp(spots[wish.by], center, (START * r) / ring) : center);
    const to = (wish: Wish): Point => on(wish.hour, ring * SQUARE);
    const at = (wish: Wish, age: number): Point =>
      lerp(from(wish), to(wish), Math.min(1, Math.max(0, age) / state.patience));
    return { center, ring, r, spots, at, angle, on };
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const dot = (x: number, y: number, size: number) => {
    ctx.beginPath();
    ctx.arc(x, y, Math.max(0, size), 0, TAU);
  };
  const flower = (x: number, y: number, size: number, petal: string, heart: string) => {
    ctx.fillStyle = petal;
    ctx.beginPath();
    for (let i = 0; i < 5; i++) {
      const a = (i / 5) * TAU - Math.PI / 2;
      const [px, py] = [x + Math.cos(a) * size * 0.55, y + Math.sin(a) * size * 0.55];
      ctx.moveTo(px + Math.max(0, size * 0.45), py);
      ctx.arc(px, py, Math.max(0, size * 0.45), 0, TAU);
    }
    ctx.fill();
    ctx.fillStyle = heart;
    dot(x, y, size * 0.3);
    ctx.fill();
  };
  const crown = (x: number, y: number, s: number, ink: string) => {
    ctx.beginPath();
    ctx.moveTo(x - s, y);
    ctx.lineTo(x - s, y - s * 0.9);
    ctx.lineTo(x - s * 0.5, y - s * 0.4);
    ctx.lineTo(x, y - s * 1.1);
    ctx.lineTo(x + s * 0.5, y - s * 0.4);
    ctx.lineTo(x + s, y - s * 0.9);
    ctx.lineTo(x + s, y);
    ctx.closePath();
    ctx.fillStyle = palette.yellow;
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.55);
    ctx.lineWidth = Math.max(1, s * 0.14);
    ctx.stroke();
  };
  const ballot = (x: number, y: number, s: number, yes: boolean, turn: number, ink: string, accent: string) => {
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(turn);
    ctx.beginPath();
    ctx.roundRect(-s * 0.7, -s, s * 1.4, s * 2, s * 0.25);
    ctx.fillStyle = yes ? accent : palette.white;
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.5);
    ctx.lineWidth = Math.max(1, s * 0.16);
    ctx.stroke();
    ctx.strokeStyle = yes ? palette.white : veil(palette.black, 0.6);
    ctx.lineWidth = Math.max(1, s * 0.22);
    ctx.beginPath();
    if (yes) {
      ctx.moveTo(-s * 0.35, 0);
      ctx.lineTo(-s * 0.05, s * 0.35);
      ctx.lineTo(s * 0.4, -s * 0.35);
    } else {
      ctx.moveTo(-s * 0.3, -s * 0.3);
      ctx.lineTo(s * 0.3, s * 0.3);
      ctx.moveTo(s * 0.3, -s * 0.3);
      ctx.lineTo(-s * 0.3, s * 0.3);
    }
    ctx.stroke();
    ctx.restore();
  };
  const wheel = (x: number, y: number, size: number, n: number, turn: number, paper: string, ink: string) => {
    for (let i = 0; i < n; i++) {
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.arc(x, y, Math.max(0, size), turn + (i / n) * TAU, turn + ((i + 1) / n) * TAU);
      ctx.closePath();
      ctx.fillStyle = HUE[i % HUE.length];
      ctx.fill();
    }
    ctx.strokeStyle = veil(paper, 0.7);
    ctx.lineWidth = Math.max(1, size * 0.04);
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      ctx.moveTo(x, y);
      ctx.lineTo(x + Math.cos(turn + (i / n) * TAU) * size, y + Math.sin(turn + (i / n) * TAU) * size);
    }
    ctx.stroke();
    ctx.strokeStyle = veil(ink, 0.45);
    ctx.lineWidth = Math.max(1, size * 0.06);
    dot(x, y, size);
    ctx.stroke();
    ctx.fillStyle = paper;
    dot(x, y, size * 0.16);
    ctx.fill();
    ctx.stroke();
    ctx.fillStyle = ink;
    ctx.beginPath();
    ctx.moveTo(x, y - size * 0.82);
    ctx.lineTo(x - size * 0.17, y - size * 1.22);
    ctx.lineTo(x + size * 0.17, y - size * 1.22);
    ctx.closePath();
    ctx.fill();
  };
  const box = (
    x: number,
    y: number,
    s: number,
    yes: number,
    no: number,
    paper: string,
    ink: string,
    accent: string,
  ) => {
    ctx.beginPath();
    ctx.roundRect(x - s, y - s * 0.6, s * 2, s * 1.4, s * 0.16);
    ctx.fillStyle = blend(paper, accent, 0.22);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.5);
    ctx.lineWidth = Math.max(1, s * 0.07);
    ctx.stroke();
    ctx.lineWidth = Math.max(1, s * 0.1);
    ctx.beginPath();
    ctx.moveTo(x - s * 0.5, y - s * 0.32);
    ctx.lineTo(x + s * 0.5, y - s * 0.32);
    ctx.stroke();
    const marks = yes + no;
    const gap = Math.min(s * 0.16, (s * 1.6) / Math.max(1, marks));
    ctx.lineWidth = Math.max(1, gap * 0.45);
    for (let i = 0; i < marks; i++) {
      const mx = x - ((marks - 1) * gap) / 2 + i * gap;
      ctx.strokeStyle = i < yes ? accent : veil(ink, 0.4);
      ctx.beginPath();
      ctx.moveTo(mx, y + s * 0.05);
      ctx.lineTo(mx, y + s * 0.5);
      ctx.stroke();
    }
  };
  const bot = (
    x: number,
    y: number,
    size: number,
    hue: string,
    ink: string,
    day: boolean,
    joy: number,
    look: Point,
    turn: number,
  ) => {
    ctx.fillStyle = veil(palette.black, day ? 0.14 : 0.32);
    ctx.beginPath();
    ctx.ellipse(x, y + size * 0.95, Math.max(0, size * 0.8), Math.max(0, size * 0.24), 0, 0, TAU);
    ctx.fill();
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(turn);
    ctx.fillStyle = hue;
    dot(0, 0, size);
    ctx.fill();
    ctx.strokeStyle = veil(ink, day ? 0.3 : 0.5);
    ctx.lineWidth = Math.max(1, size * 0.08);
    ctx.stroke();
    ctx.fillStyle = veil(palette.white, 0.35);
    dot(-size * 0.38, -size * 0.42, size * 0.2);
    ctx.fill();
    ctx.fillStyle = veil(palette.black, 0.8);
    for (const side of [-1, 1]) {
      dot(side * size * 0.32 + look[0] * size * 0.09, -size * 0.1 + look[1] * size * 0.09, size * 0.12);
      ctx.fill();
    }
    ctx.strokeStyle = veil(palette.black, 0.7);
    ctx.lineWidth = Math.max(1, size * 0.09);
    ctx.beginPath();
    ctx.moveTo(-size * 0.3, size * 0.32);
    ctx.quadraticCurveTo(0, size * (0.32 + (joy - 0.4) * 0.7), size * 0.3, size * 0.32);
    ctx.stroke();
    ctx.restore();
  };
  const legend = (ink: string, paper: string, accent: string, day: boolean) => {
    const { px, side } = type();
    if (!side) return;
    const lines = 2 + SIGNS.length + 1 + 2 + 5 * 2;
    const x = px * 1.6;
    const room = side - x - px * 0.6;
    let y = view.h / 2 - (lines * px * LEAD) / 2;
    const fit = (text: string, size: number, weight: string, wide: number) => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      const width = ctx.measureText(text).width;
      if (!(width > wide)) return;
      ctx.font = `${weight} ${(size * wide) / width}px ${TYPE}`.trim();
    };
    const say = (text: string, size: number, color: string, weight = "") => {
      fit(text, size, weight, room);
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    const row = (text: string) => {
      fit(text, px * 0.95, "", room - px * 1.8);
      ctx.fillStyle = veil(ink, 0.8);
      ctx.fillText(text, x + px * 1.8, y);
      y += px * LEAD;
    };
    const [title, says, sign] = RULES[state.decide];
    ctx.textBaseline = "middle";
    say(title, px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(says, px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    const cx = x + px * 0.6;
    bot(cx, y, px * 0.45, HUE[0], ink, day, 0.8, [0, 0], 0);
    row(SIGNS[0]);
    ctx.fillStyle = HUE[1];
    dot(cx, y, px * 0.34);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.4);
    ctx.lineWidth = Math.max(1, px * 0.06);
    ctx.stroke();
    row(SIGNS[1]);
    flower(cx, y, px * 0.42, accent, palette.yellow);
    row(SIGNS[2]);
    if (state.decide === "vote") ballot(cx, y, px * 0.34, true, -0.2, ink, accent);
    if (state.decide === "ruler") crown(cx, y + px * 0.3, px * 0.42, ink);
    if (state.decide === "lottery") wheel(cx, y + px * 0.08, px * 0.38, 6, 0, paper, ink);
    row(sign);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${Math.round(told.granted / 10)}% granted`, `of the ${told.posted} wishes made`],
      [`${hours(told.wait)} wait`, "typical time to a yes or a no"],
      [`${told.reach} of ${state.bots} reached`, "bots with a wish granted"],
      [`${told.dropped} taken back`, "wishes that waited too long"],
      [`day ${Math.min(7, Math.floor(state.tick / 24) + 1)}`, `of 7, hour ${state.tick} of ${state.length}`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    const { center, ring, r, spots, at, angle, on } = geometry();
    const k = view.still ? 1 : ease((view.t - acted) / SPIN);
    const now = state.tick - 1 + k;
    const age = (wish: Wish) => now - wish.at;
    const before = new Map(was.wishes.map((one) => [one.id, one]));
    const fresh = state.wishes.filter(
      (one) => before.get(one.id)?.how === "open" && (one.how === "granted" || one.how === "refused"),
    );
    const hub = ring * HUB;
    const hue = (bot: number) => HUE[bot % HUE.length];
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    ctx.fillStyle = paper;
    ctx.fillRect(0, 0, view.w, view.h);
    const glow = ctx.createRadialGradient(center[0], center[1], 0, center[0], center[1], Math.max(1, ring * GLOW));
    glow.addColorStop(0, blend(paper, accent, day ? 0.13 : 0.18));
    glow.addColorStop(0.7, blend(paper, accent, day ? 0.07 : 0.09));
    glow.addColorStop(1, paper);
    ctx.fillStyle = glow;
    ctx.fillRect(0, 0, view.w, view.h);
    legend(ink, paper, accent, day);
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.strokeStyle = veil(accent, day ? 0.16 : 0.22);
    ctx.lineWidth = Math.max(1, r * 0.7);
    dot(center[0], center[1], ring);
    ctx.stroke();
    ctx.fillStyle = veil(ink, 0.28);
    for (let hour = 0; hour < HOURS; hour++) {
      const [hx, hy] = on(hour, ring * 0.82);
      dot(hx, hy, r * (hour % 3 ? 0.1 : 0.18));
      ctx.fill();
    }
    const hand = angle(now);
    ctx.strokeStyle = veil(ink, 0.14);
    ctx.lineWidth = Math.max(1, r * 0.2);
    ctx.beginPath();
    ctx.moveTo(center[0], center[1]);
    ctx.lineTo(center[0] + Math.cos(hand) * ring * 0.7, center[1] + Math.sin(hand) * ring * 0.7);
    ctx.stroke();
    ctx.fillStyle = paper;
    dot(center[0], center[1], hub * 1.3);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.12);
    ctx.lineWidth = Math.max(1, r * 0.16);
    ctx.stroke();
    const done = Math.max(0, now) / state.length;
    if (done > 0) {
      ctx.strokeStyle = accent;
      ctx.beginPath();
      ctx.arc(center[0], center[1], Math.max(0, hub * 1.3), -Math.PI / 2, -Math.PI / 2 + TAU * Math.min(1, done));
      ctx.stroke();
    }
    const cast: [number, boolean][] = [];
    if (state.decide === "vote")
      for (const wish of state.wishes) {
        const old = before.get(wish.id);
        if (old?.how !== "open") continue;
        for (let one = 0; one < state.bots; one++)
          if (wish.votes[one] !== old.votes[one]) cast.push([one, wish.votes[one] === "1"]);
      }
    if (state.decide === "vote") {
      const yes = cast.filter(([, vote]) => vote).length;
      box(center[0], center[1], hub * 0.85, yes, cast.length - yes, paper, ink, accent);
    }
    if (state.decide === "ruler") crown(center[0], center[1] + hub * 0.45, hub * 0.75, ink);
    if (state.decide === "lottery") {
      const closed = state.wishes.filter((one) => one.how === "granted" || one.how === "refused");
      const drawn = fresh.length ? fresh[fresh.length - 1] : closed[closed.length - 1];
      const n = state.bots;
      const rest = (j: number) => -Math.PI / 2 - ((j + 0.5) / n) * TAU;
      const turn = drawn ? rest(drawn.judge) - (fresh.length ? (1 - k) * TAU * 1.5 : 0) : rest(0);
      wheel(center[0], center[1], hub * 0.85, n, turn, paper, ink);
    }
    const spot = (wish: Wish) => (wish.how === "open" ? at(wish, age(wish)) : at(wish, wish.end - wish.at - 1));
    const size = (a: number) => r * (0.5 + 0.45 * Math.min(1, Math.max(0, a) / state.patience));
    ctx.lineWidth = Math.max(1, r * 0.12);
    ctx.setLineDash([r * 0.05, r * 0.35]);
    for (const wish of state.wishes) {
      if (wish.how !== "open") continue;
      ctx.strokeStyle = veil(hue(wish.by), 0.7);
      ctx.beginPath();
      ctx.moveTo(...spots[wish.by]);
      ctx.lineTo(...spot(wish));
      ctx.stroke();
    }
    ctx.setLineDash([]);
    if (state.decide !== "vote" && k < 1)
      for (const wish of fresh) {
        const judge = state.decide === "ruler" ? state.ruler : wish.judge;
        const yes = wish.how === "granted";
        const reach = lerp(spots[judge], spot(wish), ease(Math.min(1, k * 2)));
        ctx.strokeStyle = yes ? veil(accent, 0.8 * (1 - k * 0.6)) : veil(ink, 0.4 * (1 - k * 0.6));
        ctx.lineWidth = Math.max(1, r * 0.16);
        ctx.beginPath();
        ctx.moveTo(...spots[judge]);
        ctx.lineTo(...reach);
        ctx.stroke();
      }
    for (const wish of state.wishes) {
      const old = before.get(wish.id);
      if (wish.how === "open") {
        const grow = old ? 1 : k;
        const a = age(wish);
        const w = Math.min(1, Math.max(0, a) / state.patience);
        const b = size(a) * grow;
        const [x, y] = at(wish, a);
        ctx.fillStyle = veil(hue(wish.by), 0.2 + 0.25 * w);
        dot(x, y, b * 1.35);
        ctx.fill();
        ctx.fillStyle = blend(hue(wish.by), palette.black, w * 0.3);
        dot(x, y, b);
        ctx.fill();
        ctx.strokeStyle = veil(ink, 0.25 + 0.35 * w);
        ctx.lineWidth = Math.max(1, b * (0.08 + 0.14 * w));
        ctx.stroke();
        ctx.fillStyle = veil(palette.white, 0.5);
        dot(x - b * 0.35, y - b * 0.35, b * 0.22);
        ctx.fill();
        if (state.decide === "vote" && b > 0) {
          const pip = Math.min(b * 0.2, (b * 1.6 * Math.PI) / state.bots / 2.4);
          for (let one = 0; one < state.bots; one++) {
            const p = -Math.PI / 2 + (one / state.bots) * TAU;
            const vote = wish.votes[one];
            ctx.fillStyle = vote === "1" ? accent : vote === "2" ? veil(ink, 0.55) : veil(ink, 0.12);
            dot(x + Math.cos(p) * b * 1.6, y + Math.sin(p) * b * 1.6, pip);
            ctx.fill();
          }
        }
        continue;
      }
      if (old?.how !== "open" || k >= 1) continue;
      const from = spot(wish);
      const b = size(wish.end - wish.at - 1);
      if (wish.how === "granted") {
        const [x, y] = lerp(from, spots[wish.by], ease(k));
        flower(x, y, b * (1 + 0.5 * Math.sin(Math.PI * k)), accent, palette.yellow);
      } else if (wish.how === "refused") {
        ctx.strokeStyle = veil(ink, 0.5 * (1 - k));
        ctx.lineWidth = Math.max(1, b * 0.15);
        dot(from[0], from[1], b * (1 + k * 1.2));
        ctx.stroke();
      } else {
        const [x, y] = lerp(from, center, k);
        ctx.fillStyle = veil(ink, 0.35 * (1 - k));
        dot(x, y, b * (1 - k));
        ctx.fill();
      }
    }
    if (k < 1)
      for (const [one, yes] of cast) {
        const [x, y] = lerp(spots[one], center, ease(k));
        const lift = Math.sin(Math.PI * k) * r * 1.2;
        ballot(x, y - lift, r * 0.32 * (1 - k * 0.4), yes, (1 - k) * 2.4, ink, accent);
      }
    const ruled = state.decide === "ruler" && fresh.length ? fresh[0] : null;
    for (let one = 0; one < state.bots; one++) {
      const [x, y] = spots[one];
      const mine = state.wishes.filter((wish) => wish.by === one);
      const closed = mine.filter((wish) => wish.how !== "open");
      const won = mine.filter((wish) => wish.how === "granted").length;
      const joy = closed.length ? won / closed.length : 0.45;
      const open = mine.find((wish) => wish.how === "open");
      const [lx, ly] = open ? at(open, age(open)) : center;
      const far = Math.hypot(lx - x, ly - y) || 1;
      const big = state.decide === "ruler" && one === state.ruler ? r * 1.18 : r;
      const nod = ruled && one === state.ruler ? Math.sin(Math.PI * k) : 0;
      const shake = ruled?.how === "refused" ? Math.sin(TAU * k) * nod * 0.35 : 0;
      const dip = ruled?.how === "granted" ? nod * r * 0.3 : 0;
      const count = closed.length ? Math.round(joy * GARLAND) : 0;
      for (let i = 0; i < count; i++) {
        const a = Math.PI / 2 + (i - (count - 1) / 2) * 0.42;
        flower(x + Math.cos(a) * big * 1.3, y + Math.sin(a) * big * 1.3, big * 0.3, accent, palette.yellow);
      }
      bot(x, y + dip, big, hue(one), ink, day, joy, [(lx - x) / far, (ly - y) / far], shake);
      if (state.decide === "ruler" && one === state.ruler) crown(x, y + dip - big * 0.85, big * 0.55, ink);
    }
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
    busy = state.over && lock ? ended + HOLD : acted + SPIN;
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
