import { blend, bright, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, Scene, View } from "../../net/sdk/run.ts";
import type { Spec, Units } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { palette } from "../../net/sdk/theme/palette.gen.js";
import type { Inherit, Over, Report, State } from "./engine.ts";
import { ADULT, held as common, deal as dealing, RULES, report, step, tick, wealth } from "./engine.ts";

/* TYPES */

export type Knobs = {
  inherit: Inherit;
  families: number;
  tint: string;
};

export type Told = Report & {
  year: number;
  length: number;
  families: number;
  inherit: Inherit;
  over: Over;
};

export type Given = Opts & { onTell?: (told: Told) => void };

export type Hands = { done: () => boolean; poke: () => void };

export type Held = Scene & Partial<Hands>;

export type Roots = ReturnType<typeof make>;

type Edge = { x: number; ys: number[] };

export const SPEC: Spec = [
  {
    key: "inherit",
    label: "When a bot fades",
    kind: "segment",
    def: "heir",
    options: [
      ["heir", "To the heir"],
      ["commons", "To the commons"],
      ["none", "To nobody"],
    ],
    group: "World",
  },
  { key: "families", label: "Families", kind: "slider", def: 16, min: 8, max: 32, step: 4, group: "World" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "Each family is one bot at a time. A bot is born, grows up, works, buys land, grows old and fades, and its child takes its place. Time flows left to right like a river; each family is one stream, as wide as its share of all the wealth, and a bead marks each birth. A run is two hundred years, about four generations, and replays exactly from the seed and the knobs.",
  "An adult makes coins by its skill and a little luck, spends some each year, and buys a plot of land when it can; land is scarce and pays rent every year. Every child learns from its parent, so a little skill passes down in every world.",
  "The rule is the knob: what happens to a fading bot's coins and land. To the heir: its child keeps them. To the commons: they go to everyone, and the rent of common land is shared each year. To nobody: the coins vanish and the land goes back for anyone to buy.",
  "Top is the wealth the richest fifth of grown bots holds, mobility the children who end life in another fifth than their parent, made the income earned by work rather than rent, from nothing the newborns starting with nothing. Enter steps a year, Space pauses, Reroll draws another world. Bots are not people: a toy asks a question, it proves nothing.",
];

export const PAGE = {
  spec: SPEC,
  about: ABOUT,
  name: (value) => `roots-${value.inherit}-${value.seed}`,
  keys: [
    {
      key: "Enter",
      label: (_value, scene) => (scene?.done?.() ? "Run again" : "Step"),
      act: "poke",
      button: true,
      menu: "Edit",
    },
    { key: "r", label: "New world", act: "reroll" },
  ],
  actions: { poke: (scene) => scene.poke?.() },
} satisfies Page<Ready & Units, Held>;

export const SAVERS = [
  {
    id: "river",
    title: "River of generations",
    still: 10000,
    pick: (rand: Rng) => ({
      inherit: pick(rand, RULES),
      families: pick(rand, [12, 16, 16, 20]),
    }),
  },
  {
    id: "delta",
    title: "Many families",
    still: 10000,
    pick: (rand: Rng) => ({
      inherit: pick(rand, RULES),
      families: pick(rand, [24, 28, 32]),
    }),
  },
];

const PACE = 150;
const HOLD = 6000;
const TAIL = 100;
const CATCH = 40;
const FADE = 1400;
const LEAVE = 900;
const STILL = 170;
const WIDTH = 0.28;
const LETTERS = 20;
const LEAD = 1.5;
const FLOOR = 24;
const SOURCE = 0.18;
const SPRING = 26;
const BEND = 13;
const WOBBLE = 0.035;
const SWAY = 14000;
const FLOW = 5200;
const STREAKS = 3;
const BOB = 900;
const TYPE = '"Noto Sans", system-ui, sans-serif';
const TAU = Math.PI * 2;
const DEAL: (keyof Knobs)[] = ["inherit", "families"];

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
  "a family, as wide as its wealth",
  "a birth: a child takes over",
  "the commons, everyone's",
  "a bot alive, small as a child",
];

const SAYS: Record<Inherit, string> = {
  heir: "coins and land pass to the child",
  commons: "coins and land pass to everyone",
  none: "coins vanish, land is freed",
};

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

const fade = (k: number) => ease(Math.max(0, k));

const hash = (i: number, salt: number) => {
  let x = Math.imul(i + 1, 374761393) + Math.imul(salt, 668265263);
  x = Math.imul(x ^ (x >>> 13), 1274126177);
  return ((x ^ (x >>> 16)) >>> 0) / 4294967296;
};

export const hue = (family: number) => HUES[(family * 5) % HUES.length];

export const order = (families: number) => {
  const half = Math.floor(families / 2);
  return [
    ...Array.from({ length: half }, (_, k) => k),
    -1,
    ...Array.from({ length: families - half }, (_, k) => k + half),
  ];
};

export const weights = (state: State) =>
  order(state.families).map((family) =>
    family < 0 ? (state.inherit === "commons" ? common(state) : 0) : wealth(state, family) + FLOOR,
  );

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let state: State;
  let told: Report;
  let cols: number[][] = [];
  let beads: [number, number][] = [];
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
      year: state.year,
      length: state.length,
      families: state.families,
      inherit: state.inherit,
      over: state.over,
    });
  };
  const rule = () => (lock ? RULES[(RULES.indexOf(value.inherit) + round) % RULES.length] : value.inherit);
  const record = (next: State) => {
    state = next;
    cols.push(weights(state));
    for (const family of state.faded) beads.push([state.year, family]);
  };
  const deal = () => {
    state = dealing({ ...value, inherit: rule() }, { seed, round });
    cols = [weights(state)];
    beads = [];
    if (lock && view.still) for (let k = 0; k < STILL && !state.over; k++) record(step(state, tick));
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
    const left = side + px * 1.5;
    const right = view.w - px * 2.5;
    const half = view.h / 2 - px * 2.4;
    const x = (year: number) => left + ((right - left) * year) / state.length;
    const sway = view.still ? 0 : (view.t / SWAY) * TAU;
    const mid = (year: number) => view.h / 2 + half * WOBBLE * Math.sin(year / BEND + sway);
    const thick = (year: number) => half * (SOURCE + (1 - SOURCE) * ease(year / SPRING));
    const edge = (year: number, ws: number[]): Edge => {
      const total = ws.reduce((a, b) => a + b, 0) || 1;
      const top = mid(year) - thick(year);
      const span = thick(year) * 2;
      const ys = [top];
      let sum = 0;
      for (const one of ws) {
        sum += one;
        ys.push(top + (span * sum) / total);
      }
      return { x: x(year), ys };
    };
    return { px, left, right, half, x, mid, thick, edge };
  };
  type Geometry = ReturnType<typeof geometry>;
  const edges = (g: Geometry, k: number): Edge[] => {
    const out = cols.slice(0, Math.max(1, cols.length - 1)).map((ws, year) => g.edge(year, ws));
    if (cols.length < 2) return out;
    const a = cols[cols.length - 2];
    const b = cols[cols.length - 1];
    const p = cols.length - 2 + k;
    out.push(
      g.edge(
        p,
        a.map((one, i) => one + (b[i] - one) * k),
      ),
    );
    return out;
  };
  const shade = () => {
    const t = Math.min(view.t, held);
    const enter = fade((t - rise) / FADE);
    const leave = state.over ? 1 - fade((t - ended - HOLD + FADE) / FADE) : 1;
    return Math.min(enter, leave, 1 - fade((view.t - down) / LEAVE));
  };
  const trace = (points: [number, number][], join: boolean) => {
    if (!points.length) return;
    const [x0, y0] = points[0];
    if (join) ctx.lineTo(x0, y0);
    else ctx.moveTo(x0, y0);
    for (let j = 1; j < points.length - 1; j++) {
      const [x, y] = points[j];
      const [nx, ny] = points[j + 1];
      ctx.quadraticCurveTo(x, y, (x + nx) / 2, (y + ny) / 2);
    }
    const [xl, yl] = points[points.length - 1];
    ctx.lineTo(xl, yl);
  };
  const band = (list: Edge[], b: number) => {
    ctx.beginPath();
    trace(
      list.map((one): [number, number] => [one.x, one.ys[b]]),
      false,
    );
    trace(list.map((one): [number, number] => [one.x, one.ys[b + 1]]).reverse(), true);
    ctx.closePath();
  };
  const tone = (family: number, accent: string, paper: string, day: boolean) =>
    family < 0 ? blend(accent, paper, day ? 0.15 : 0.25) : blend(hue(family), paper, day ? 0.12 : 0.22);
  const hatch = (region: () => void, gap: number) => {
    ctx.save();
    region();
    ctx.clip();
    const drift = view.still ? 0 : ((view.t / FLOW) * gap * 4) % gap;
    ctx.strokeStyle = veil(palette.white, 0.4);
    ctx.lineWidth = Math.max(1, gap * 0.16);
    ctx.beginPath();
    for (let x = -view.h - gap + drift; x < view.w; x += gap) {
      ctx.moveTo(x, view.h);
      ctx.lineTo(x + view.h, 0);
    }
    ctx.stroke();
    ctx.restore();
  };
  const bead = (x: number, y: number, r: number, color: string, paper: string) => {
    ctx.fillStyle = paper;
    ctx.beginPath();
    ctx.arc(x, y, r, 0, TAU);
    ctx.fill();
    ctx.strokeStyle = color;
    ctx.lineWidth = Math.max(1, r * 0.45);
    ctx.stroke();
  };
  const body = (x: number, y: number, r: number, color: string, ink: string) => {
    ctx.fillStyle = veil(palette.black, 0.22);
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
  const legend = (ink: string, paper: string, accent: string, day: boolean) => {
    const { px, side } = type();
    if (!side) return;
    const pct = (n: number) => `${Math.round(n / 10)}%`;
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
    const title = SPEC[0].options as [string, string][];
    say(title.find(([key]) => key === state.inherit)![1], px * 1.4, ink, "600");
    y += px * LEAD * 1.1;
    say(SAYS[state.inherit], px * 0.95, veil(ink, 0.65));
    y += px * LEAD * 1.4;
    const chip = () => {
      ctx.beginPath();
      ctx.roundRect(x, y - px * 0.35, px * 1.2, px * 0.7, px * 0.2);
    };
    const swatch = (color: string) => {
      ctx.fillStyle = color;
      chip();
      ctx.fill();
    };
    swatch(tone(2, accent, paper, day));
    row(SIGNS[0]);
    bead(x + px * 0.6, y, px * 0.3, hue(7), paper);
    row(SIGNS[1]);
    swatch(tone(-1, accent, paper, day));
    hatch(chip, px * 0.4);
    row(SIGNS[2]);
    body(x + px * 0.6, y, px * 0.42, hue(4), ink);
    row(SIGNS[3]);
    y += px * 0.9;
    const rows: [string, string][] = [
      [`${pct(told.top)} top`, "wealth held by the richest fifth"],
      [`${told.pairs ? pct(told.mobility) : "-"} mobility`, "children ending in another fifth"],
      [`${pct(told.made)} made`, "income earned by work, not rent"],
      [`${pct(told.nothing)} from nothing`, "newborns starting with nothing"],
      [`year ${state.year}`, `of ${state.length}, generation ${told.generations}`],
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
    const g = geometry();
    const k = view.still || state.over ? 1 : ease((view.t - acted) / PACE);
    const fams = order(state.families);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = lock ? shade() : 1;
    const ground = ctx.createLinearGradient(0, 0, 0, view.h);
    ground.addColorStop(0, day ? blend(paper, palette.yellow, 0.08) : blend(paper, palette.indigo, 0.1));
    ground.addColorStop(0.5, day ? blend(paper, palette.green, 0.1) : blend(paper, palette.blue, 0.05));
    ground.addColorStop(1, day ? blend(paper, palette.green, 0.2) : blend(paper, palette.green, 0.08));
    ctx.fillStyle = ground;
    ctx.fillRect(0, 0, view.w, view.h);
    legend(ink, paper, accent, day);
    const list = edges(g, k);
    const front = list[list.length - 1];
    const now = cols.length - 2 + (cols.length < 2 ? 1 : k);
    ctx.setLineDash([g.px * 0.3, g.px * 0.7]);
    ctx.strokeStyle = veil(ink, 0.18);
    ctx.lineWidth = Math.max(1, g.px * 0.12);
    ctx.beginPath();
    for (let p = Math.max(0, now); p <= state.length; p += 2) {
      if (p === Math.max(0, now)) ctx.moveTo(g.x(p), g.mid(p));
      else ctx.lineTo(g.x(p), g.mid(p));
    }
    ctx.stroke();
    ctx.setLineDash([]);
    if (list.length > 1) {
      ctx.fillStyle = veil(palette.black, day ? 0.08 : 0.2);
      ctx.save();
      ctx.translate(g.px * 0.25, g.px * 0.35);
      band(
        list.map((one) => ({ x: one.x, ys: [one.ys[0], one.ys[one.ys.length - 1]] })),
        0,
      );
      ctx.fill();
      ctx.restore();
      fams.forEach((family, b) => {
        band(list, b);
        ctx.fillStyle = tone(family, accent, paper, day);
        ctx.fill();
      });
      if (state.inherit === "commons") hatch(() => band(list, fams.indexOf(-1)), g.px * 0.8);
      ctx.strokeStyle = veil(paper, 0.55);
      ctx.lineWidth = Math.max(1, g.px * 0.08);
      for (let b = 1; b < fams.length; b++) {
        ctx.beginPath();
        trace(
          list.map((one): [number, number] => [one.x, one.ys[b]]),
          false,
        );
        ctx.stroke();
      }
      ctx.strokeStyle = blend(paper, day ? palette.green : palette.teal, day ? 0.5 : 0.35);
      ctx.lineWidth = Math.max(1, g.px * 0.25);
      for (const b of [0, fams.length]) {
        ctx.beginPath();
        trace(
          list.map((one): [number, number] => [one.x, one.ys[b]]),
          false,
        );
        ctx.stroke();
      }
      if (!view.still && front.x > g.left)
        fams.forEach((family, b) => {
          for (let s = 0; s < STREAKS; s++) {
            const p = ((view.t / FLOW + hash(b, s)) % 1) * Math.max(0, now);
            const j = Math.min(list.length - 1, Math.floor(p));
            const at = list[j];
            const wide = at.ys[b + 1] - at.ys[b];
            if (wide < g.px * 0.5) continue;
            const y = (at.ys[b] + at.ys[b + 1]) / 2 + (hash(b, s + 7) - 0.5) * wide * 0.5;
            ctx.strokeStyle = veil(
              palette.white,
              (family < 0 ? 0.5 : 0.35) * Math.sin((p / Math.max(1, now)) * Math.PI),
            );
            ctx.lineWidth = Math.max(1, Math.min(wide * 0.12, g.px * 0.14));
            ctx.lineCap = "round";
            ctx.beginPath();
            ctx.moveTo(at.x, y);
            ctx.lineTo(at.x + g.px * 2.2, y);
            ctx.stroke();
          }
        });
    }
    const r = Math.max(2, Math.min(g.px * 0.3, (g.thick(now) * 2) / fams.length / 2));
    for (const [year, family] of beads) {
      const at = list[Math.min(list.length - 1, year)];
      if (!at || year > now) continue;
      const b = fams.indexOf(family);
      bead(at.x, (at.ys[b] + at.ys[b + 1]) / 2, r, hue(family), paper);
    }
    fams.forEach((family, b) => {
      if (family < 0) return;
      const y = (front.ys[b] + front.ys[b + 1]) / 2;
      const grown = Math.min(1, state.age[family] / ADULT);
      const bob = view.still ? 0 : Math.sin((view.t / BOB + hash(family, 3)) * TAU) * r * 0.2;
      body(front.x + r * 1.6, y + bob, r * (0.7 + 0.6 * grown), hue(family), ink);
    });
  };
  const apply = () => {
    const next = step(state, tick);
    if (next === state) return false;
    record(next);
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
