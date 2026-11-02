import { TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import type { Mode, Over, Report, State, Wish } from "./engine.ts";
import { deal as dealing, HOURS, MODES, report, run, step, tick, WEEK } from "./engine.ts";

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
const RING = 0.34;
const SQUARE = 0.2;
const BOT = 0.034;
const CROWD = 0.3;
const SPREAD = 2.5;
const SPOT = 0.42;
const START = 2.2;
const CLOCK = 0.07;
const FACE = 0.04;
const DIAL = 0.28;
const BODY = 0.16;
const JOY = 0.64;
const RIM = 0.22;
const ARC = 1.32;
const HALO = 1.85;
const JEWEL = 0.17;
const CROWN = [-0.5, 0, 0.5];
const TETHER = 0.22;
const YES = 0.6;
const NO = 0.24;
const PULSE = 0.9;
const LINE = 1.5;
const TAU = Math.PI * 2;
const FADE = 1400;
const LEAVE = 900;
const DEAL: (keyof Knobs)[] = ["decide", "bots", "eager", "patience"];

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const lerp = (a: Point, b: Point, k: number): Point => [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k];

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let was: State;
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
  const geometry = () => {
    const s = Math.min(view.w, view.h);
    const center: Point = [view.w / 2, view.h / 2];
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
    const from = (wish: Wish): Point => lerp(spots[wish.by], center, (START * r) / ring);
    const to = (wish: Wish): Point => on(wish.hour, ring * SQUARE);
    const at = (wish: Wish, age: number): Point =>
      lerp(from(wish), to(wish), Math.min(1, Math.max(0, age) / state.patience));
    return { s, center, ring, r, spots, at, to };
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const paint = () => {
    const { paper, accent, ink } = view.look();
    const { center, ring, r, spots, at } = geometry();
    const k = view.still ? 1 : ease((view.t - acted) / SPIN);
    const age = (wish: Wish) => state.tick - 1 + k - wish.at;
    const line = Math.max(1, LINE * view.dpr);
    const before = new Map(was.wishes.map((one) => [one.id, one]));
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (lock) ctx.globalAlpha = shade();
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.arc(center[0], center[1], ring * SQUARE, 0, TAU);
    ctx.fillStyle = veil(ink, FACE);
    ctx.fill();
    ctx.lineWidth = line;
    ctx.strokeStyle = veil(ink, CLOCK);
    ctx.beginPath();
    ctx.arc(center[0], center[1], ring, 0, TAU);
    ctx.stroke();
    const done = (state.tick - 1 + k) / state.length;
    if (done > 0) {
      ctx.strokeStyle = veil(ink, DIAL);
      ctx.lineWidth = line * 2;
      ctx.beginPath();
      ctx.arc(center[0], center[1], ring * SQUARE + line * 3, -Math.PI / 2, -Math.PI / 2 + TAU * done);
      ctx.stroke();
    }
    ctx.lineWidth = line;
    ctx.strokeStyle = veil(accent, TETHER);
    ctx.beginPath();
    for (const wish of state.wishes) {
      if (wish.how !== "open") continue;
      ctx.moveTo(...spots[wish.by]);
      ctx.lineTo(...at(wish, age(wish)));
    }
    ctx.stroke();
    for (const wish of state.wishes) {
      const old = before.get(wish.id);
      if (!old || wish.how === "dropped") continue;
      const spot = wish.how === "open" ? at(wish, age(wish)) : at(wish, wish.end - wish.at - 1);
      for (let bot = 0; bot < state.bots; bot++) {
        const now = wish.votes[bot];
        const fresh = state.decide === "vote" ? now !== old.votes[bot] : old.how === "open" && wish.how !== "open";
        if (!fresh) continue;
        const judge = state.decide === "vote" ? bot : state.decide === "ruler" ? state.ruler : wish.judge;
        if (state.decide !== "vote" && bot !== judge) continue;
        const yes = state.decide === "vote" ? now === "1" : wish.how === "granted";
        const reach = lerp(spots[judge], spot, ease(Math.min(1, k * 2)));
        ctx.strokeStyle = veil(yes ? accent : ink, (yes ? YES : NO) * (1 - k * 0.7));
        ctx.beginPath();
        ctx.moveTo(...spots[judge]);
        ctx.lineTo(...reach);
        ctx.stroke();
      }
    }
    const pulses: number[] = [];
    for (const wish of state.wishes) {
      const old = before.get(wish.id);
      const size = r * SPOT;
      if (wish.how === "open") {
        const grow = old ? 1 : k;
        const [x, y] = at(wish, age(wish));
        ctx.beginPath();
        ctx.arc(x, y, size * grow, 0, TAU);
        ctx.fillStyle = accent;
        ctx.fill();
        continue;
      }
      if (old?.how !== "open") continue;
      const spot = at(wish, wish.end - wish.at - 1);
      if (wish.how === "granted") {
        const [x, y] = lerp(spot, spots[wish.by], k);
        ctx.beginPath();
        ctx.arc(x, y, size * (1 + 0.5 * Math.sin(Math.PI * k)), 0, TAU);
        ctx.fillStyle = veil(accent, 1 - k * 0.5);
        ctx.fill();
        pulses.push(wish.by);
      } else {
        const [x, y] = wish.how === "dropped" ? lerp(spot, center, k) : spot;
        ctx.beginPath();
        ctx.arc(x, y, size * (1 - k), 0, TAU);
        ctx.fillStyle = veil(ink, (1 - k) * RIM * 2);
        ctx.fill();
      }
    }
    const share = (bot: number) => {
      const mine = state.wishes.filter((one) => one.by === bot);
      return mine.length ? mine.filter((one) => one.how === "granted").length / mine.length : 0;
    };
    for (let bot = 0; bot < state.bots; bot++) {
      const [x, y] = spots[bot];
      const joy = share(bot);
      ctx.beginPath();
      ctx.arc(x, y, r, 0, TAU);
      ctx.fillStyle = paper;
      ctx.fill();
      ctx.fillStyle = veil(accent, BODY + JOY * joy);
      ctx.fill();
      ctx.strokeStyle = veil(ink, RIM);
      ctx.lineWidth = line;
      ctx.stroke();
      ctx.lineWidth = line * 2;
      ctx.strokeStyle = veil(ink, CLOCK);
      ctx.beginPath();
      ctx.arc(x, y, r * ARC, 0, TAU);
      ctx.stroke();
      if (joy > 0) {
        ctx.strokeStyle = accent;
        ctx.beginPath();
        ctx.arc(x, y, r * ARC, -Math.PI / 2, -Math.PI / 2 + TAU * joy);
        ctx.stroke();
      }
      if (state.decide === "ruler" && bot === state.ruler) {
        const out = Math.atan2(y - center[1], x - center[0]);
        ctx.fillStyle = accent;
        ctx.beginPath();
        for (const turn of CROWN) {
          const a = out + turn;
          const [px, py] = [x + Math.cos(a) * r * HALO, y + Math.sin(a) * r * HALO];
          ctx.moveTo(px + r * JEWEL, py);
          ctx.arc(px, py, r * JEWEL, 0, TAU);
        }
        ctx.fill();
      }
      if (pulses.includes(bot) && k < 1) {
        ctx.lineWidth = line;
        ctx.strokeStyle = veil(accent, PULSE * (1 - k));
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
