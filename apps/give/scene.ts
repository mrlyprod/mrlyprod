import { blend, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Economy, Flow, Over, Report, State } from "./engine.ts";
import {
  CAP,
  deal as dealing,
  ECONOMIES,
  KINDS,
  NONE,
  PATIENCE,
  report,
  run,
  STORE,
  step,
  tick,
  WEEK,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  economy: Economy;
  bots: number;
  plenty: number;
  generous: number;
  purse: number;
  tint: string;
};

export type Told = Report & { tick: number; length: number; bots: number; economy: Economy; over: Over };

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Give = ReturnType<typeof make>;

type Point = [number, number];

export const SPEC: Spec = [
  {
    key: "economy",
    label: "Economy",
    kind: "segment",
    def: "money",
    options: [
      ["money", "Money"],
      ["gift", "Gift"],
      ["pool", "Pool"],
    ],
    group: "Village",
  },
  { key: "bots", label: "Bots", kind: "slider", def: 8, min: 4, max: 16, step: 1, group: "Village" },
  { key: "plenty", label: "Plenty", kind: "slider", def: 40, min: 10, max: 100, step: 10, unit: "%", group: "Village" },
  {
    key: "generous",
    label: "Generosity",
    kind: "slider",
    def: 70,
    min: 0,
    max: 100,
    step: 10,
    unit: "%",
    group: "Village",
    when: (value) => value.economy !== "money",
  },
  {
    key: "purse",
    label: "Coins each",
    kind: "slider",
    def: 3,
    min: 1,
    max: 9,
    step: 1,
    group: "Village",
    when: (value) => value.economy !== "pool",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "A village of bots stands in a ring. Each bot makes one kind of thing, its own shape and colour: orange round food, blue square tools, green three-cornered care, purple diamond songs. Each bot also needs all four, now and then; a need is a hollow shape inside the ring, drifting toward the middle and reddening while it waits. A basket holds three things; a thing made into a full basket spills.",
  "The economy is the knob, three steps from money to none. Money is honest: one coin buys one thing from any maker in the village, and a bot with no coin goes without. Gift makes the coin optional: a maker within two bots gives to the neediest, as often as Generosity says, and gets a coin of thanks when there is one. Pool forgets the coin: a maker puts what it makes in the store in the middle, as often as Generosity says, and the longest need draws first.",
  "A need not met in six ticks is missed. A run is a week of hours, 168 ticks, and replays exactly from the seed and the knobs.",
  "Fed is needs met out of needs posted, idle the things spilled out of things made, top the share of everything used that went to the best-fed fifth of the bots (fair is a fifth), hungry the bots that missed more than they got. Enter steps one tick, Space pauses, Reroll draws another village. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `give-${value.economy}-${value.seed}`,
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
    title: "Giving village",
    still: 6000,
    pick: (rand: Rng) => ({
      economy: pick(rand, ECONOMIES),
      bots: 7 + Math.floor(rand() * 3),
      plenty: 40,
      generous: 70 + 10 * Math.floor(rand() * 3),
      purse: 2 + Math.floor(rand() * 2),
    }),
  },
  {
    id: "town",
    title: "Giving town",
    still: 6000,
    pick: (rand: Rng) => ({
      economy: pick(rand, ECONOMIES),
      bots: 13 + Math.floor(rand() * 4),
      plenty: 30 + 10 * Math.floor(rand() * 2),
      generous: 80,
      purse: 2,
    }),
  },
];

const PACE = 520;
const SPIN = 440;
const HOLD = 4000;
const TAIL = 100;
const CATCH = 40;
const RING = 0.34;
const BOT = 0.03;
const CROWD = 0.3;
const HEAP = 0.4;
const SPARE = 2;
const DIAL = 1.12;
const FACE = 0.05;
const CLOCK = 0.08;
const DIALED = 0.4;
const BODY = 0.14;
const JOY = 0.76;
const FLOOR = 0.5;
const ARC = 1.42;
const SLOT = 0.4;
const SLOTS = 1.9;
const WIDE = 0.95;
const NEED = 0.44;
const NEAR = 2.3;
const FAR = 4.2;
const FAN = 0.2;
const PURSE = 3.4;
const COIN = 0.2;
const PACK = 1.15;
const COINED = 0.5;
const ITEM = 0.5;
const BENDS: Record<Economy, number> = { money: 0.85, gift: 0.12, pool: 0 };
const MARKET = 0.1;
const TRAIL = 10;
const TRAILED = 0.55;
const EMPTY = 0.1;
const MISS = 0.8;
const DROP = 1.6;
const POP = 0.6;
const PULSE = 0.8;
const THROB = 0.22;
const SHOUT = 2;
const SAD = 0.35;
const LINE = 1.5;
const GOLD = 2.399963;
const TAU = Math.PI * 2;
const FADE = 1400;
const LEAVE = 900;
const WIDTH = 0.26;
const LETTERS = 15;
const LEAD = 1.5;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const DEAL: (keyof Knobs)[] = ["economy", "bots", "plenty", "generous", "purse"];
const MOVING = new Set(["use", "give", "buy", "share", "draw"]);

export const HUES = [palette.orange, palette.blue, palette.green, palette.purple];
export const NAMES = ["food", "tools", "care", "songs"];
export const COINAGE = palette.yellow;
export const HUNGER = palette.red;

const RULES: Record<Economy, string> = {
  money: "a coin buys a thing",
  gift: "neighbours give, coins optional",
  pool: "all in the store, need first",
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const lerp = (a: Point, b: Point, k: number): Point => [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k];

const bezier = (a: Point, c: Point, b: Point, k: number): Point => lerp(lerp(a, c, k), lerp(c, b, k), k);

export function shape(ctx: CanvasRenderingContext2D, kind: number, x: number, y: number, size: number) {
  if (kind === 0) {
    ctx.moveTo(x + size, y);
    ctx.arc(x, y, size, 0, TAU);
    return;
  }
  if (kind === 1) {
    const h = size * 0.86;
    ctx.rect(x - h, y - h, h * 2, h * 2);
    return;
  }
  if (kind === 2) {
    const h = size * 1.15;
    ctx.moveTo(x, y - h);
    ctx.lineTo(x + h * 0.95, y + h * 0.62);
    ctx.lineTo(x - h * 0.95, y + h * 0.62);
    ctx.closePath();
    return;
  }
  const h = size * 1.2;
  ctx.moveTo(x, y - h);
  ctx.lineTo(x + h, y);
  ctx.lineTo(x, y + h);
  ctx.lineTo(x - h, y);
  ctx.closePath();
}

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let was: State;
  let past: Flow[][] = [];
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
    if (gone) return;
    opts.onTell?.({
      ...report(state),
      tick: state.tick,
      length: state.length,
      bots: state.bots,
      economy: state.economy,
      over: state.over,
    });
  };
  const rule = () => (lock ? ECONOMIES[(ECONOMIES.indexOf(value.economy) + round) % ECONOMIES.length] : value.economy);
  const deal = () => {
    state = dealing({ ...value, economy: rule() }, { seed, round });
    was = state;
    past = [];
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
    const px = Math.max(11 * view.dpr, Math.min(15 * view.dpr, view.h * 0.017));
    const side = !lock && view.w > view.h * 1.15 ? Math.min(view.w * WIDTH, px * LETTERS * 1.2) : 0;
    return { px, side };
  };
  const geometry = () => {
    const { side } = type();
    const s = Math.min(view.w - side, view.h);
    const center: Point = [side + (view.w - side) / 2, view.h / 2];
    const ring = s * RING;
    const n = state.bots;
    const r = Math.min(s * BOT, (s * CROWD) / n);
    const angle = (bot: number) => (bot / n) * TAU - Math.PI / 2;
    const polar = (a: number, radius: number): Point => [
      center[0] + Math.cos(a) * radius,
      center[1] + Math.sin(a) * radius,
    ];
    const spot = (bot: number) => polar(angle(bot), ring);
    const out = (bot: number, along: number, across: number): Point => {
      const a = angle(bot);
      const [x, y] = spot(bot);
      return [x + Math.cos(a) * along - Math.sin(a) * across, y + Math.sin(a) * along + Math.cos(a) * across];
    };
    const slot = (bot: number, j: number) => out(bot, r * SLOTS, (j - (CAP - 1) / 2) * r * WIDE);
    const need = (bot: number, kind: number, age: number) => {
      const k = Math.min(1, Math.max(0, age) / PATIENCE);
      return polar(angle(bot) + (kind - (KINDS - 1) / 2) * (TAU / n) * FAN, ring - r * (NEAR + (FAR - NEAR) * k));
    };
    const heap = ring * HEAP;
    const many = Math.ceil(KINDS * n * SPARE);
    const grain = heap / Math.sqrt(many);
    const wedges: Point[][] = Array.from({ length: KINDS }, () => []);
    for (let j = 0; j < many; j++) {
      const a = j * GOLD;
      const d = grain * Math.sqrt(j + 0.5);
      const turn = (((a + Math.PI / 2 + Math.PI / KINDS) % TAU) + TAU) % TAU;
      wedges[Math.floor((turn / TAU) * KINDS) % KINDS].push([center[0] + Math.cos(a) * d, center[1] + Math.sin(a) * d]);
    }
    const store = (kind: number, j: number): Point => {
      const list = wedges[kind];
      return list[Math.min(list.length - 1, Math.max(0, j))];
    };
    const item = grain * ITEM;
    const coins = state.purse * n;
    const bead = Math.min(r * COIN, (r * PURSE - r * ARC * 1.2) / (PACK * Math.sqrt(coins)));
    const pile = (bot: number) => out(bot, r * PURSE + bead * PACK * Math.sqrt(coins) * COINED, 0);
    const purse = (bot: number, at: number): Point => {
      const [x, y] = pile(bot);
      const j = Math.max(0, at);
      const d = bead * PACK * Math.sqrt(j + 0.5) * 1.4;
      return [x + Math.cos(j * GOLD) * d, y + Math.sin(j * GOLD) * d];
    };
    const bend = (a: Point, b: Point): Point => lerp(lerp(a, b, 0.5), center, BENDS[state.economy]);
    return { s, center, ring, r, spot, slot, need, heap, grain, store, item, bead, pile, purse, bend };
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const pct = (n: number) => `${Math.round(n / 10)}%`;
  const band = (ink: string) => {
    const { px } = type();
    const s = Math.min(view.w, view.h);
    const room = (view.h - s) / 2;
    if (room < px * LEAD * 4) return;
    const told = report(state);
    const y = view.h / 2 + s / 2 + room / 2 - px * LEAD * 0.6;
    const each = view.w / KINDS;
    ctx.textBaseline = "middle";
    ctx.font = `${px * 0.9}px ${TYPE}`;
    for (let kind = 0; kind < KINDS; kind++) {
      const x = each * kind + each / 2 - px * 2;
      ctx.fillStyle = HUES[kind];
      ctx.beginPath();
      shape(ctx, kind, x, y, px * 0.42);
      ctx.fill();
      ctx.fillStyle = veil(ink, 0.75);
      ctx.fillText(NAMES[kind], x + px * 0.9, y);
    }
    ctx.textAlign = "center";
    ctx.font = `600 ${px}px ${TYPE}`;
    ctx.fillStyle = ink;
    ctx.fillText(
      `${pct(told.fed)} fed   ${pct(told.idle)} idle   ${pct(told.top)} top   ${told.hungry} hungry`,
      view.w / 2,
      y + px * LEAD * 1.3,
    );
    ctx.textAlign = "left";
  };
  const legend = (ink: string) => {
    const { px, side } = type();
    if (lock) return;
    if (!side) return band(ink);
    const told = report(state);
    const coined = state.economy !== "pool";
    const lines = 2 + KINDS + (coined ? 1 : 0) + 1 + 4 * 2;
    const x = px * 1.6;
    let y = view.h / 2 - (lines * px * LEAD + px * 2) / 2;
    const say = (text: string, size: number, color: string, weight = "") => {
      ctx.font = `${weight} ${size}px ${TYPE}`.trim();
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
    };
    ctx.textBaseline = "middle";
    say(state.economy[0].toUpperCase() + state.economy.slice(1), px * 1.3, ink, "600");
    y += px * LEAD * 1.1;
    say(RULES[state.economy], px * 0.9, veil(ink, 0.6));
    y += px * LEAD * 1.4;
    const key = (draw: () => void, text: string) => {
      draw();
      ctx.font = `${px * 0.9}px ${TYPE}`;
      ctx.fillStyle = veil(ink, 0.75);
      ctx.fillText(text, x + px * 1.4, y);
      y += px * LEAD;
    };
    for (let kind = 0; kind < KINDS; kind++)
      key(() => {
        ctx.fillStyle = HUES[kind];
        ctx.beginPath();
        shape(ctx, kind, x + px * 0.45, y, px * 0.42);
        ctx.fill();
      }, NAMES[kind]);
    if (coined)
      key(() => {
        ctx.fillStyle = COINAGE;
        ctx.beginPath();
        ctx.arc(x + px * 0.45, y, px * 0.32, 0, TAU);
        ctx.fill();
      }, "coin");
    key(() => {
      ctx.strokeStyle = HUNGER;
      ctx.lineWidth = Math.max(1, view.dpr * 1.5);
      ctx.beginPath();
      shape(ctx, 0, x + px * 0.45, y, px * 0.36);
      ctx.stroke();
    }, "a need, waiting");
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${pct(told.fed)} fed`, "of needs met"],
      [`${pct(told.idle)} idle`, "of things spilled"],
      [`${pct(told.top)} top`, `to the best-fed fifth, fair ${pct(told.fair)}`],
      [`${told.hungry} hungry`, `of ${state.bots} bots`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.1, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.82, veil(ink, 0.55));
      y += px * LEAD * 1.15;
    }
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const g = geometry();
    const { center, ring, r } = g;
    const k = view.still ? 1 : ease((view.t - acted) / SPIN);
    const line = Math.max(1, LINE * view.dpr);
    const coined = state.economy !== "pool";
    const pooled = state.economy === "pool";
    const age = (since: number) => state.tick - 1 + k - since;
    const where = (bot: number, kind: number): Point => (bot === STORE ? g.store(kind, 0) : g.spot(bot));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    legend(ink);
    if (pooled) {
      ctx.beginPath();
      ctx.arc(
        center[0],
        center[1],
        Math.min(g.heap, g.grain * (Math.sqrt(KINDS * Math.max(...state.store) + 1) + 1.5)),
        0,
        TAU,
      );
      ctx.fillStyle = veil(ink, FACE);
      ctx.fill();
    }
    if (state.economy === "money") {
      ctx.lineWidth = line * 1.5;
      ctx.strokeStyle = veil(COINAGE, 0.6);
      ctx.beginPath();
      ctx.arc(center[0], center[1], ring * MARKET, 0, TAU);
      ctx.stroke();
    }
    ctx.lineWidth = line;
    ctx.strokeStyle = veil(ink, CLOCK);
    ctx.beginPath();
    ctx.arc(center[0], center[1], ring, 0, TAU);
    ctx.stroke();
    const done = (state.tick - 1 + k) / state.length;
    if (done > 0) {
      ctx.strokeStyle = veil(accent, DIALED);
      ctx.lineWidth = line * 2;
      ctx.beginPath();
      ctx.arc(center[0], center[1], g.heap * DIAL, -Math.PI / 2, -Math.PI / 2 + TAU * done);
      ctx.stroke();
    }
    ctx.lineWidth = line * 1.3;
    past.forEach((flows, i) => {
      const old = 1 - (past.length - 1 - i + k) / TRAIL;
      if (old <= 0) return;
      for (let kind = 0; kind < KINDS; kind++) {
        ctx.strokeStyle = veil(HUES[kind], TRAILED * old * old);
        ctx.beginPath();
        for (const one of flows) {
          if (one.kind !== kind || !MOVING.has(one.how) || one.from === one.to) continue;
          const a = where(one.from, kind);
          const b = where(one.to, kind);
          ctx.moveTo(...a);
          if (one.from === STORE || one.to === STORE) ctx.lineTo(...b);
          else ctx.quadraticCurveTo(...g.bend(a, b), ...b);
        }
        ctx.stroke();
      }
    });
    if (pooled)
      for (let kind = 0; kind < KINDS; kind++) {
        ctx.fillStyle = HUES[kind];
        ctx.beginPath();
        for (let j = 0; j < state.store[kind]; j++) shape(ctx, kind, ...g.store(kind, j), g.item);
        ctx.fill();
      }
    const fresh = new Map<number, number>();
    for (const one of state.flows) if (one.how === "make") fresh.set(one.from, (fresh.get(one.from) ?? 0) + 1);
    for (let bot = 0; bot < state.bots; bot++) {
      const kind = state.crafts[bot];
      const size = r * SLOT;
      ctx.fillStyle = veil(ink, EMPTY);
      ctx.beginPath();
      for (let j = state.stock[bot]; j < CAP; j++) shape(ctx, kind, ...g.slot(bot, j), size * 0.5);
      ctx.fill();
      ctx.fillStyle = HUES[kind];
      ctx.beginPath();
      const grown = state.stock[bot] - (fresh.get(bot) ?? 0);
      for (let j = 0; j < state.stock[bot]; j++) shape(ctx, kind, ...g.slot(bot, j), size * (j < grown ? 1 : k));
      ctx.fill();
      if (!coined) continue;
      if (!state.coins[bot]) {
        const [x, y] = g.pile(bot);
        ctx.setLineDash([line * 2, line * 2]);
        ctx.lineWidth = line;
        ctx.strokeStyle = veil(COINAGE, 0.7);
        ctx.beginPath();
        ctx.arc(x, y, g.bead * 2.4, 0, TAU);
        ctx.stroke();
        ctx.setLineDash([]);
        continue;
      }
      ctx.fillStyle = COINAGE;
      ctx.strokeStyle = veil(ink, 0.25);
      ctx.lineWidth = Math.max(1, view.dpr * 0.75);
      ctx.beginPath();
      for (let j = 0; j < state.coins[bot]; j++) {
        const [x, y] = g.purse(bot, j);
        ctx.moveTo(x + g.bead, y);
        ctx.arc(x, y, g.bead, 0, TAU);
      }
      ctx.fill();
      ctx.stroke();
    }
    for (let i = 0; i < state.needs.length; i++) {
      if (state.needs[i] === NONE) continue;
      const bot = Math.floor(i / KINDS);
      const kind = i % KINDS;
      const since = state.needs[i];
      const late = Math.min(1, Math.max(0, age(since)) / (PATIENCE - 1));
      const grow = since === state.tick && state.tick > was.tick ? k : 1;
      const throb = late >= 1 - 1 / PATIENCE ? 1 + THROB * Math.sin(Math.PI * k) : 1;
      ctx.lineWidth = line * (1 + (SHOUT - 1) * late);
      ctx.strokeStyle = blend(HUES[kind], HUNGER, ease(late));
      ctx.beginPath();
      shape(ctx, kind, ...g.need(bot, kind, age(since)), r * NEED * grow * throb);
      ctx.stroke();
    }
    ctx.lineWidth = line;
    const pulses = new Set<number>();
    for (const one of state.flows) {
      const size = r * SLOT;
      if (one.how === "miss") {
        const at = g.need(one.to, one.kind, PATIENCE);
        ctx.lineWidth = line * SHOUT;
        ctx.strokeStyle = veil(HUNGER, MISS * (1 - k));
        ctx.beginPath();
        shape(ctx, one.kind, ...lerp(at, g.spot(one.to), k * 0.5), r * NEED * (1 + k * 0.8));
        ctx.stroke();
        ctx.lineWidth = line;
        continue;
      }
      if (one.how === "spill") {
        const away: Point =
          one.from === STORE
            ? g.store(one.kind, state.store[one.kind])
            : lerp(g.spot(one.from), g.slot(one.from, (CAP - 1) / 2), 1 + DROP * k);
        ctx.fillStyle = veil(HUES[one.kind], 0.7 * (1 - k));
        ctx.beginPath();
        shape(ctx, one.kind, ...away, (one.from === STORE ? g.item : size) * (1 - k * 0.5));
        ctx.fill();
        continue;
      }
      if (!MOVING.has(one.how)) continue;
      const since = one.to === STORE ? NONE : was.needs[one.to * KINDS + one.kind];
      const target =
        one.to === STORE
          ? g.store(one.kind, state.store[one.kind] - 1)
          : g.need(one.to, one.kind, state.tick - 1 - since);
      const source =
        one.from === STORE
          ? g.store(one.kind, state.store[one.kind])
          : one.how === "share"
            ? g.spot(one.from)
            : g.slot(one.from, state.stock[one.from]);
      const through =
        one.from === STORE || one.to === STORE || one.from === one.to
          ? lerp(source, target, 0.5)
          : g.bend(source, target);
      const [x, y] = bezier(source, through, target, k);
      const small = one.to === STORE ? g.item / size : one.from === STORE ? g.item / size : 1;
      const grow = one.from === STORE ? small + (1 - small) * k : small + (1 - small) * (1 - k);
      ctx.fillStyle = HUES[one.kind];
      ctx.beginPath();
      shape(ctx, one.kind, x, y, size * grow * (1 + POP * Math.sin(Math.PI * k) * 0.5));
      ctx.fill();
      if (one.coin && coined) {
        const back = bezier(
          g.purse(one.to, state.coins[one.to]),
          through,
          g.purse(one.from, state.coins[one.from] - 1),
          k,
        );
        ctx.fillStyle = COINAGE;
        ctx.beginPath();
        ctx.arc(back[0], back[1], g.bead * 1.5, 0, TAU);
        ctx.fill();
      }
      if (one.to !== STORE) pulses.add(one.to);
    }
    for (let bot = 0; bot < state.bots; bot++) {
      const [x, y] = g.spot(bot);
      const hue = HUES[state.crafts[bot]];
      const got = state.fed[bot];
      const all = got + state.missed[bot];
      const joy = all ? got / all : 1;
      const glad = Math.max(0, (joy - FLOOR) / (1 - FLOOR));
      const sad = Math.min(1, Math.max(0, 1 - glad - SAD) / (1 - SAD));
      ctx.fillStyle = paper;
      ctx.beginPath();
      shape(ctx, state.crafts[bot], x, y, r);
      ctx.fill();
      ctx.fillStyle = veil(hue, BODY + JOY * glad);
      ctx.fill();
      ctx.strokeStyle = blend(hue, HUNGER, sad);
      ctx.lineWidth = line;
      ctx.stroke();
      ctx.lineWidth = line * 2;
      ctx.strokeStyle = veil(ink, CLOCK);
      ctx.beginPath();
      ctx.arc(x, y, r * ARC, 0, TAU);
      ctx.stroke();
      if (all) {
        ctx.strokeStyle = hue;
        ctx.beginPath();
        ctx.arc(x, y, r * ARC, -Math.PI / 2, -Math.PI / 2 + TAU * joy);
        ctx.stroke();
      }
      if (pulses.has(bot) && k < 1) {
        ctx.lineWidth = line;
        ctx.strokeStyle = veil(hue, PULSE * (1 - k));
        ctx.beginPath();
        ctx.arc(x, y, r * (ARC + k), 0, TAU);
        ctx.stroke();
      }
    }
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    was = state;
    state = next;
    past = [...past, state.flows].slice(-TRAIL);
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
