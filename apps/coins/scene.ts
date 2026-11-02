import { blend, bright, rgb, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Over, Report, State } from "./engine.ts";
import { deal as dealing, MAP, MAX, NONE, PRICE, pairs, report, step, TOWNS, tick } from "./engine.ts";

/* TYPES */

export type Knobs = {
  currencies: number;
  fee: number;
  traders: number;
  tint: string;
};

export type Told = Report & {
  day: number;
  length: number;
  currencies: number;
  fee: number;
  traders: number;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Coins = ReturnType<typeof make>;

type Corner = { x: number; y: number; by: number };

export const SPEC: Spec = [
  {
    key: "currencies",
    label: "Currencies",
    kind: "slider",
    def: 5,
    min: 1,
    max: MAX,
    step: 1,
    group: "World",
  },
  {
    key: "fee",
    label: "Exchange fee",
    kind: "slider",
    def: 3,
    min: 0,
    max: 10,
    step: 1,
    unit: "%",
    group: "World",
  },
  { key: "traders", label: "Traders", kind: "slider", def: 48, min: 16, max: 96, step: 8, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "Sixteen towns sit on a map split into regions, and each region has its own coin. Bots trade: each walks to a town, pays for its goods in that town's coin, and the town pays its takings back as wages to the bots who live there. A run is a hundred and twenty days and replays exactly from the seed and the knobs.",
  "A bot that lacks the coin it needs walks to the money changer on the border first, swaps its own coin for the other at a rate that drifts every day, and pays the fee, which is lost to the trade. A changer only holds the coins other bots left there: when one side runs dry, bots pile up at the border waiting, and after twelve days they give up and walk home.",
  "The knob is how many coins circulate, from one to five, on the same land and the same towns. With one coin there is nothing to change and no border to wait at.",
  "Traded counts goods bought, failed the trips given up at a border, lost the money the fees took out of all the money there was, stuck the bots waiting right now. This toy only counts the cost of changing money; what a region's own coin may do for it is not in it. Enter steps a day, Space pauses, Reroll draws another map. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `coins-${value.currencies}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New map", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "market",
    title: "Five coins",
    still: 10000,
    pick: (rand: Rng) => ({
      currencies: pick(rand, [1, 2, 3, 4, 5, 5]),
      fee: pick(rand, [2, 3, 5]),
      traders: pick(rand, [40, 48, 56]),
    }),
  },
  {
    id: "borders",
    title: "Busy borders",
    still: 10000,
    pick: (rand: Rng) => ({
      currencies: pick(rand, [3, 4, 5]),
      fee: pick(rand, [3, 5, 8]),
      traders: pick(rand, [80, 88, 96]),
    }),
  },
];

const PACE = 220;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 70;
const WIDTH = 0.28;
const LETTERS = 20;
const LEAD = 1.5;
const GOLD = 2.39996;
const SPARKS = 4;
const STACK = 6;
const BOB = 1100;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["currencies", "fee", "traders"];

export const FIVE = [palette.blue, palette.orange, palette.green, palette.purple, palette.yellow, palette.red];

export const SIGNS: string[] = [
  "a town, in its coin's region",
  "a trader, the colour of its coin",
  "a money changer and its coins",
  "a jam: traders waiting for coins",
];

const near = (a: string, b: string) => {
  const [p, q] = [rgb(a), rgb(b)];
  return p.reduce((sum, v, i) => sum + (v - q[i]) ** 2, 0);
};

export const coins = (accent: string) => {
  const rest = FIVE.slice();
  rest.splice(rest.indexOf(rest.reduce((a, b) => (near(b, accent) < near(a, accent) ? b : a))), 1);
  return [accent, ...rest].slice(0, MAX);
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export function clip(poly: Corner[], ux: number, uy: number, wx: number, wy: number, by: number): Corner[] {
  const mx = (ux + wx) / 2;
  const my = (uy + wy) / 2;
  const side = (p: Corner) => (p.x - mx) * (wx - ux) + (p.y - my) * (wy - uy);
  const out: Corner[] = [];
  for (let i = 0; i < poly.length; i++) {
    const p = poly[i];
    const q = poly[(i + 1) % poly.length];
    const [sp, sq] = [side(p), side(q)];
    const cut = () => {
      const k = sp / (sp - sq);
      return { x: p.x + (q.x - p.x) * k, y: p.y + (q.y - p.y) * k };
    };
    if (sp <= 0) out.push(p);
    if (sp <= 0 && sq > 0) out.push({ ...cut(), by });
    if (sp > 0 && sq <= 0) out.push({ ...cut(), by: p.by });
  }
  return out;
}

export function cells(state: Pick<State, "tx" | "ty">, x0: number, y0: number, x1: number, y1: number) {
  return state.tx.map((ux, u) => {
    let poly: Corner[] = [
      { x: x0, y: y0, by: NONE },
      { x: x1, y: y0, by: NONE },
      { x: x1, y: y1, by: NONE },
      { x: x0, y: y1, by: NONE },
    ];
    for (let w = 0; w < state.tx.length && poly.length; w++)
      if (w !== u) poly = clip(poly, ux, state.ty[u], state.tx[w], state.ty[w], w);
    return poly;
  });
}

export const slot = (j: number, r: number, room: number) => {
  const a = j * GOLD - Math.PI / 2;
  const d = room + r * 1.25 * Math.sqrt(j + 0.6);
  return [Math.cos(a) * d, Math.sin(a) * d];
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
      day: state.day,
      length: state.length,
      currencies: state.currencies,
      fee: state.fee,
      traders: state.traders,
      over: state.over,
    });
  };
  const many = () => (lock ? ((value.currencies - 1 + round) % MAX) + 1 : value.currencies);
  const deal = () => {
    state = dealing({ ...value, currencies: many() }, { seed, round });
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
    const r = Math.max(1, Math.min(px * 0.7, Math.min(wide, tall) / 52));
    const back = { x0: ((side - left) / wide) * MAP, y0: (-top / tall) * MAP };
    const end = { x1: ((view.w - left) / wide) * MAP, y1: ((view.h - top) / tall) * MAP };
    return { px, side, X, Y, r, ...back, ...end };
  };
  type Geometry = ReturnType<typeof geometry>;
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const piles = (s: State) => {
    const out = new Map<number, number>();
    const count = new Map<number, number>();
    for (let trader = 0; trader < s.traders; trader++) {
      if (s.wait[trader] <= 0) continue;
      const p = s.booth[trader];
      const j = count.get(p) ?? 0;
      count.set(p, j + 1);
      out.set(trader, j);
    }
    return out;
  };
  const place = (s: State, g: Geometry, pile: Map<number, number>, trader: number): [number, number] => {
    const j = pile.get(trader);
    if (j === undefined) return [g.X(s.x[trader]), g.Y(s.y[trader])];
    const [dx, dy] = slot(j, g.r, g.r * 2.6);
    return [g.X(s.x[trader]) + dx, g.Y(s.y[trader]) + dy];
  };
  const house = (x: number, y: number, s: number, fill: string, ink: string) => {
    ctx.fillStyle = veil(palette.black, 0.18);
    ctx.beginPath();
    ctx.ellipse(x + s * 0.15, y + s * 0.95, s * 1.1, s * 0.35, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = fill;
    ctx.strokeStyle = veil(ink, 0.45);
    ctx.lineWidth = Math.max(1, s * 0.14);
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
  };
  const coin = (x: number, y: number, r: number, color: string, ink: string, flip = 1) => {
    const rx = Math.max(0.5, r * Math.abs(flip));
    ctx.fillStyle = veil(palette.black, 0.2);
    ctx.beginPath();
    ctx.ellipse(x + r * 0.2, y + r * 0.75, rx * 0.9, r * 0.35, 0, 0, TAU);
    ctx.fill();
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.ellipse(x, y, rx, r, 0, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = veil(ink, 0.4);
    ctx.lineWidth = Math.max(1, r * 0.16);
    ctx.stroke();
    ctx.strokeStyle = veil(palette.white, 0.55);
    ctx.lineWidth = Math.max(1, r * 0.12);
    ctx.beginPath();
    ctx.ellipse(x, y, rx * 0.62, r * 0.62, 0, 0, TAU);
    ctx.stroke();
  };
  const changer = (x: number, y: number, s: number, a: string, b: string, ink: string, paper: string) => {
    ctx.save();
    ctx.beginPath();
    ctx.roundRect(x - s, y - s, s * 2, s * 2, s * 0.35);
    ctx.fillStyle = paper;
    ctx.fill();
    ctx.clip();
    ctx.fillStyle = a;
    ctx.beginPath();
    ctx.moveTo(x - s, y - s);
    ctx.lineTo(x + s, y - s);
    ctx.lineTo(x - s, y + s);
    ctx.closePath();
    ctx.fill();
    ctx.fillStyle = b;
    ctx.beginPath();
    ctx.moveTo(x + s, y - s);
    ctx.lineTo(x + s, y + s);
    ctx.lineTo(x - s, y + s);
    ctx.closePath();
    ctx.fill();
    ctx.restore();
    ctx.strokeStyle = ink;
    ctx.lineWidth = Math.max(1, s * 0.16);
    ctx.beginPath();
    ctx.roundRect(x - s, y - s, s * 2, s * 2, s * 0.35);
    ctx.stroke();
  };
  const stack = (x: number, y: number, r: number, n: number, color: string, ink: string) => {
    for (let i = 0; i < n; i++) {
      ctx.fillStyle = color;
      ctx.strokeStyle = veil(ink, 0.4);
      ctx.lineWidth = Math.max(1, r * 0.2);
      ctx.beginPath();
      ctx.ellipse(x, y - i * r * 0.55, r, r * 0.4, 0, 0, TAU);
      ctx.fill();
      ctx.stroke();
    }
  };
  const legend = (ink: string, paper: string, hues: string[], day: boolean) => {
    const { px, side } = type();
    if (!side) return;
    const pct = (n: number) => `${(n / 10).toFixed(1)}%`;
    const lines = 2 + SIGNS.length + 2 + 5 * 2;
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
    const n = state.currencies;
    say(n === 1 ? "One coin" : `${n} coins`, px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(
      n === 1
        ? "nothing to change, no border to wait at"
        : `${told.booths === 1 ? "1 changer" : `${told.booths} changers`}, ${state.fee}% fee`,
      px * 0.95,
      veil(ink, 0.65),
    );
    y += px * LEAD * 1.4;
    house(x + px * 0.6, y, px * 0.42, blend(hues[0], ink, day ? 0.15 : 0), ink);
    row(SIGNS[0]);
    coin(x + px * 0.6, y, px * 0.42, hues[Math.min(1, n - 1)], ink);
    row(SIGNS[1]);
    changer(x + px * 0.6, y, px * 0.4, hues[0], hues[Math.min(1, n - 1)], veil(ink, 0.5), paper);
    row(SIGNS[2]);
    for (let j = 0; j < 4; j++) {
      const [dx, dy] = slot(j, px * 0.16, px * 0.1);
      coin(x + px * 0.6 + dx, y + dy, px * 0.18, hues[j % n], ink);
    }
    row(SIGNS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${told.traded} traded`, "goods bought in any town"],
      [`${told.failed} failed`, "trips given up at a border"],
      [`${pct(told.lost)} lost`, "of all the money, to exchange fees"],
      [`${told.stuck} stuck`, "traders waiting at a border now"],
      [`day ${state.day}`, `of ${state.length}`],
    ];
    for (const [big, small] of rows) {
      say(big, px * 1.2, ink, "600");
      y += px * LEAD * 0.95;
      say(small, px * 0.88, veil(ink, 0.6));
      y += px * LEAD * 1.15;
    }
  };
  const map = (g: Geometry, hues: string[], paper: string, ink: string, day: boolean) => {
    const polys = cells(state, g.x0, g.y0, g.x1, g.y1);
    polys.forEach((poly, town) => {
      if (poly.length < 3) return;
      ctx.beginPath();
      for (const p of poly) ctx.lineTo(g.X(p.x), g.Y(p.y));
      ctx.closePath();
      ctx.fillStyle = blend(paper, hues[state.region[town]], day ? 0.2 + 0.05 * (town % 2) : 0.15 + 0.04 * (town % 2));
      ctx.fill();
    });
    const edges = (bold: boolean) => {
      ctx.beginPath();
      polys.forEach((poly, town) => {
        poly.forEach((p, i) => {
          if (p.by <= town || (state.region[p.by] !== state.region[town]) !== bold) return;
          const q = poly[(i + 1) % poly.length];
          ctx.moveTo(g.X(p.x), g.Y(p.y));
          ctx.lineTo(g.X(q.x), g.Y(q.y));
        });
      });
      ctx.stroke();
    };
    ctx.lineCap = "round";
    ctx.strokeStyle = veil(paper, 0.5);
    ctx.lineWidth = Math.max(1, g.px * 0.08);
    edges(false);
    ctx.strokeStyle = veil(ink, day ? 0.35 : 0.45);
    ctx.lineWidth = Math.max(1, g.px * 0.16);
    ctx.setLineDash([g.px * 0.5, g.px * 0.45]);
    edges(true);
    ctx.setLineDash([]);
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const day = bright(paper);
    const hues = coins(accent);
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
    map(g, hues, paper, ink, day);
    ctx.restore();
    if (g.side) {
      ctx.fillStyle = veil(ink, 0.12);
      ctx.fillRect(g.side, 0, Math.max(1, g.px * 0.08), view.h);
    }
    legend(ink, paper, hues, day);
    const r = g.r;
    for (let town = 0; town < TOWNS; town++) {
      const x = g.X(state.tx[town]);
      const y = g.Y(state.ty[town]);
      const hue = hues[state.region[town]];
      house(x, y, r * 1.5, blend(hue, ink, day ? 0.15 : 0), ink);
      if (state.capital.indexOf(town) < state.currencies) {
        ctx.fillStyle = veil(palette.white, 0.85);
        ctx.beginPath();
        ctx.arc(x, y + r * 0.25, r * 0.4, 0, TAU);
        ctx.fill();
      }
    }
    for (const town of state.sold) {
      const hue = hues[state.region[town]];
      ctx.strokeStyle = veil(blend(hue, ink, 0.1), 0.7 * (1 - k));
      ctx.lineWidth = Math.max(1, r * 0.25);
      ctx.beginPath();
      ctx.arc(g.X(state.tx[town]), g.Y(state.ty[town]), r * (1.6 + 2.4 * ease(k)), 0, TAU);
      ctx.stroke();
    }
    for (const p of pairs(state.currencies)) {
      const x = g.X(state.bx[p]);
      const y = g.Y(state.by[p]);
      const [a, b] = [Math.floor(p / MAX), p % MAX];
      stack(x - r * 1.7, y - r * 1.6, r * 0.55, Math.min(STACK, Math.ceil(state.till[p * 2] / PRICE)), hues[a], ink);
      stack(
        x + r * 1.7,
        y - r * 1.6,
        r * 0.55,
        Math.min(STACK, Math.ceil(state.till[p * 2 + 1] / PRICE)),
        hues[b],
        ink,
      );
      changer(x, y, r * 1.05, hues[a], hues[b], veil(ink, 0.55), paper);
    }
    if (state.fee && k < 1)
      state.changed.forEach((p, n) => {
        for (let s = 0; s < SPARKS; s++) {
          const a = hash(n, s) * TAU;
          const d = r * (1 + 3 * ease(k));
          ctx.fillStyle = veil(ink, 0.5 * (1 - k));
          ctx.beginPath();
          ctx.arc(g.X(state.bx[p]) + Math.cos(a) * d, g.Y(state.by[p]) + Math.sin(a) * d - r * 2 * k, r * 0.22, 0, TAU);
          ctx.fill();
        }
      });
    const was = piles(prev);
    const now = piles(state);
    for (let trader = 0; trader < state.traders; trader++) {
      const [x0, y0] = place(prev, g, was, trader);
      const [x1, y1] = place(state, g, now, trader);
      const moving = x0 !== x1 || y0 !== y1;
      const bob = view.still || moving ? 0 : Math.sin((view.t / BOB + hash(trader, 5)) * TAU) * r * 0.15;
      const turn = prev.coin[trader] !== state.coin[trader];
      const flip = turn ? Math.cos(Math.PI * k) : 1;
      const color = hues[turn && k < 0.5 ? prev.coin[trader] : state.coin[trader]];
      coin(x0 + (x1 - x0) * k, y0 + (y1 - y0) * k + bob, r, color, ink, flip);
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
