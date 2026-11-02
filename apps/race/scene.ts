import type { Paints } from "../../net/sdk/color.ts";
import { blend, bright, rgb, TINTS, veil } from "../../net/sdk/color.ts";
import type { Ready } from "../../net/sdk/hooks.ts";
import type { Page } from "../../net/sdk/page.tsx";
import type { Rng } from "../../net/sdk/rand.ts";
import { pick } from "../../net/sdk/rand.ts";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Spec, Units, Value } from "../../net/sdk/spec.ts";
import { tidy } from "../../net/sdk/spec.ts";
import { num, runs, sheet } from "../../net/sdk/svg.ts";
import { light } from "../../net/sdk/theme/theme.ts";
import { turn } from "../../net/sdk/turn.ts";
import type { Mrly } from "../shared/designs/engine.ts";
import { wasm } from "../shared/units/index.ts";
import type { Laid, Over, Plan, Series, Side } from "./engine.ts";
import {
  bake,
  budget,
  cap,
  DIM,
  HOLD,
  HOT,
  heat,
  keeping,
  lay,
  NUMBERS,
  planned,
  reach,
  series,
  TURN,
  top,
} from "./engine.ts";

/* TYPES */

export type Knobs = {
  a: string;
  b: string;
  base: number;
  number: number;
  level: number;
  finish: number;
  speed: number;
  walkers: number;
  heat: number;
  tint: string;
};

export type Phase = "ready" | "running" | "held" | "done";

export type Told = Pick<Side, "code" | "name" | "fills" | "of" | "lit" | "cells" | "dimension">;

export type Stat = {
  phase: Phase;
  tick: number;
  reach: number[];
  over: Over | null;
  goal: number;
  side: number;
  walkers: number;
  sides: Told[];
};

export type Given = Opts & { math?: Mrly | null; onTell?: (stat: Stat) => void };

export type Seen = Pick<View, "w" | "h" | "dpr" | "t"> &
  Partial<Pick<View, "rand" | "still" | "lock" | "fixed" | "wake">> & { look: () => Pick<Paints, "accent"> };

export type Hands = { go: () => void; toggle: () => void; pair: () => void };

export type Race = ReturnType<typeof make>;

export type Handled = Omit<Race, keyof Hands> & Hands;

type Sheet = {
  page: OffscreenCanvas;
  pen: OffscreenCanvasRenderingContext2D;
  image: ImageData;
  hot: Float32Array;
  halo: OffscreenCanvas;
  aura: OffscreenCanvasRenderingContext2D;
  light: ImageData;
};

type Ink = { fg: string; line: string; deep: string; bg: string };

const HIDE = () => false;

export const SPEC: Spec = [
  { key: "a", label: "Code A", kind: "text", def: "127", group: "Race", when: HIDE },
  { key: "b", label: "Rival code", kind: "text", def: "239", group: "Race" },
  {
    key: "base",
    label: "Base",
    kind: "segment",
    def: 3,
    options: [
      [2, "2"],
      [3, "3"],
    ],
    group: "Board",
    when: HIDE,
  },
  { key: "number", label: "Number", kind: "pick", def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: "Board" },
  {
    key: "level",
    label: "Level",
    kind: "slider",
    def: 4,
    min: 1,
    max: (value) => cap(value.number as number),
    step: 1,
    group: "Board",
  },
  {
    key: "finish",
    label: "Finish",
    kind: "slider",
    def: 20,
    min: 1,
    max: (value) => top((value.number as number) ** (value.level as number)),
    step: 1,
    unit: "cells",
    group: "Race",
  },
  { key: "speed", label: "Speed", kind: "slider", def: 720, min: 6, max: 3000, step: 6, unit: "/s", group: "Race" },
  { key: "walkers", label: "Walkers", kind: "slider", def: 300, min: 10, max: 1000, step: 10, group: "Race" },
  {
    key: "heat",
    label: "Fade",
    kind: "slider",
    def: 0,
    min: 0,
    max: 90,
    step: 5,
    unit: "%/s",
    zero: "Off",
    group: "Look",
  },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ABOUT = [
  "Two designs that keep the same number of cells at every scale weigh the same and share one fractal dimension. Walkers start at home on both boards, the filled cell nearest the centre, and the first team to wander an average of the finish from home wins.",
  "Each tick every walker takes one blind step, and a step into a hole is a lost turn. The shape alone sets the pace. Distance grows like a power of time whose exponent the shape sets, which [the walk dimension note](/research/walks/) measures.",
  "Code A walks the left board, the top one on a tall screen, in the accent colour; code B walks the other in the opposite hue. The dashed ring is the finish and the solid ring is how far the swarm has spread.",
  "Space, Enter or a tap on the boards starts, pauses and races again on the next seed. Reroll is a new walk on the same pair, n rolls a new pair of one mass, and the codes take any two designs.",
  "The design chip picks the left board. The Library lists matchups of one mass and every rival of the left board that weighs the same.",
];

export const LABELS: Record<Phase, string> = { ready: "Start", running: "Pause", held: "Resume", done: "Race again" };

export const named = (value: Value) => `race-${value.a}-${value.b}-${value.seed}`;

export const MATCHUPS: [string, string][] = [
  ["127", "239"],
  ["495", "510"],
  ["186", "316"],
  ["486", "187"],
  ["494", "254"],
];

export const PAGE = {
  spec: SPEC,
  design: { dim: DIM, base: "base", code: "a" },
  record: true,
  about: ABOUT,
  name: named,
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Pick<Race, "phase"> | null) => LABELS[scene?.phase?.() ?? "ready"],
      act: "go",
      button: true,
      menu: "Edit",
    },
    { key: "n", label: "New pair", act: "pair", menu: "Edit" },
  ],
  actions: { go: (scene) => scene.go?.(), pair: (scene) => scene.pair?.() },
} satisfies Page<Ready & Units, Handled>;

const duel = (rand: Rng) => {
  const [a, b] = pick(rand, MATCHUPS);
  return rand() < 0.5 ? { a, b } : { a: b, b: a };
};

export const SAVERS = [
  {
    id: "footprints",
    title: "Footprints",
    pick: (rand: Rng) => ({
      ...duel(rand),
      base: 3,
      number: 3,
      level: 4,
      finish: 20,
      walkers: 300,
      speed: 180,
      heat: 0,
    }),
  },
  {
    id: "embers",
    title: "Embers",
    pick: () => ({ base: 3, number: 5, level: 3, finish: 28, walkers: 500, speed: 240, heat: 40 }),
  },
];

export const units = wasm("math");

const TAU = Math.PI * 2;
const PAD = 28;
const GAP = 40;
const TRAY = 10;
const SPOT = 0.75;
const LAMP = 0.16;
const BLOOM = 0.7;
const FINISH = 1400;
const SHADE = 0.55;
const WHITE = "#ffffff";
const BLACK = "#000000";
const TOP = 0.55;
const RING = 0.45;
const DASH = [4, 6];
const LINE = 1.5;
const WIN = 3;
const MARK = 2;
const TOLD = 100;
const BOARD: (keyof Knobs)[] = ["a", "b", "base", "number", "level"];

const glow = (h: number) => TOP * Math.min(1, h / HOT);

const won = (over: Over | null, s: number) => over?.how === "goal" && over.winner === s;

const ease = (k: number) => (k >= 1 ? 1 : k <= 0 ? 0 : 1 - (1 - k) ** 3);

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: Seen, opts: Given = {}) {
  const math = opts.math;
  if (!math) throw new Error("race: the scene wants @mrlyprod/math as opts.math");
  const ctx = canvas.getContext("2d")!;
  const hands = !view.lock;
  let value = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let laid: Plan | null = null;
  let walk = 0;
  let n = 0;
  let sheets: Sheet[] = [];
  let show: Series | null = null;
  let phase: Phase = "ready";
  let at = 0;
  let clock = { t: 0, tick: 0 };
  let seen = -1;
  let dirty = true;
  let box: Laid | null = null;
  let colours: string[] = [];
  let bytes: number[][] = [];
  let ink: Ink | null = null;
  let ground: number[][] = [];
  let told = -Infinity;
  let floor: OffscreenCanvas | null = null;
  let floored = "";
  let ended = -Infinity;
  let said = "";
  let gone = false;
  const paint = () => {
    const { accent } = view.look();
    const css = typeof getComputedStyle === "function" && canvas ? getComputedStyle(canvas) : null;
    const read = (name: string, fallback: string) => css?.getPropertyValue(name).trim() || fallback;
    colours = [accent, turn(accent, TURN)];
    bytes = colours.map(rgb);
    ink = {
      fg: read("--fg", light.fg),
      line: read("--line", light.line),
      deep: read("--deep", light.deep),
      bg: read("--bg", light.bg),
    };
    ground = [rgb(ink.deep), rgb(ink.line)];
    dirty = true;
  };
  const stat = (): Stat => {
    const { state } = show!;
    return {
      phase,
      tick: state.tick,
      reach: reach(state),
      over: state.over,
      goal: laid!.goal,
      side: n,
      walkers: laid!.walkers,
      sides: laid!.sides.map(({ code, name, fills, of, lit, cells, dimension }) => ({
        code,
        name,
        fills,
        of,
        lit,
        cells,
        dimension,
      })),
    };
  };
  const tell = (force: boolean) => {
    if (gone || !hands) return;
    if (!force && phase === said && view.t - told < TOLD) return;
    told = view.t;
    said = phase;
    opts.onTell?.(stat());
  };
  const restart = () => {
    show = series(laid!, walk, hands ? Infinity : HOLD);
    phase = hands ? "ready" : "running";
    at = 0;
    clock = { t: hands ? view.t : 0, tick: 0 };
    seen = -1;
    ended = -Infinity;
    dirty = true;
  };
  const replan = () => {
    ({ plan: laid, walk } = planned(math, hands ? value : { ...value, a: opts.a ?? "", b: opts.b ?? "" }, seed));
    if (laid.side !== n) {
      n = laid.side;
      const halos = laid.sides.map(() => new OffscreenCanvas(n, n));
      sheets = laid.sides.map((_, s) => {
        const page = new OffscreenCanvas(n, n);
        const halo = halos[s];
        return {
          page,
          pen: page.getContext("2d")!,
          image: new ImageData(n, n),
          hot: new Float32Array(n * n),
          halo,
          aura: halo.getContext("2d")!,
          light: new ImageData(n, n),
        };
      });
    }
    box = null;
    restart();
  };
  const anchor = () => {
    clock = { t: view.t, tick: at };
  };
  const play = () => {
    if (phase !== "ready" && phase !== "held") return;
    anchor();
    phase = "running";
    dirty = true;
    view.wake?.();
  };
  const go = (): Phase => {
    if (phase === "running") phase = "held";
    else play();
    dirty = true;
    tell(true);
    return phase;
  };
  const retarget = () => {
    const was = show!.state.over;
    show!.aim(laid!.goal);
    if (was && !show!.state.over && phase === "done") {
      phase = "running";
      anchor();
    }
    if (!was && show!.state.over && hands) phase = "done";
  };
  const level = (s: number) => {
    const { state, trace } = show!;
    const { filled } = laid!.sides[s];
    const trail = trace.trail[s];
    const stamp = trace.stamp[s];
    const out = sheets[s].hot;
    for (let i = 0; i < out.length; i++) out[i] = filled[i] ? heat(trail[i], stamp[i], state.tick, laid!.keep) : 0;
    return out;
  };
  const blit = (s: number) => {
    const { image, pen, light: glowing, aura } = sheets[s];
    const data = image.data;
    const lit = glowing.data;
    const { filled } = laid!.sides[s];
    const [r, g, b] = bytes[s];
    const hot = level(s);
    for (let i = 0; i < hot.length; i++) {
      const [br, bg, bb] = ground[filled[i]];
      const k = filled[i] ? glow(hot[i]) : 0;
      const o = i * 4;
      data[o] = br + (r - br) * k;
      data[o + 1] = bg + (g - bg) * k;
      data[o + 2] = bb + (b - bb) * k;
      data[o + 3] = 255;
      lit[o] = r;
      lit[o + 1] = g;
      lit[o + 2] = b;
      lit[o + 3] = 255 * k * BLOOM;
    }
    for (const i of show!.state.at[s]) {
      data.set([r, g, b, 255], i * 4);
      lit[i * 4 + 3] = 255;
    }
    pen.putImageData(image, 0, 0);
    aura.putImageData(glowing, 0, 0);
  };
  const rings = (s: number) => {
    const { state } = show!;
    const { home } = laid!.sides[s];
    if (home < 0) return;
    const far = reach(state)[s];
    const dpr = view.dpr;
    const { cell, edge } = box!;
    const { x, y } = box!.boards[s];
    const hx = x + ((home % n) + 0.5) * cell;
    const hy = y + (Math.floor(home / n) + 0.5) * cell;
    ctx.save();
    ctx.beginPath();
    ctx.rect(x, y, edge, edge);
    ctx.clip();
    ctx.strokeStyle = veil(colours[s], RING);
    ctx.lineWidth = dpr;
    ctx.setLineDash(DASH.map((d) => d * dpr));
    ctx.beginPath();
    ctx.arc(hx, hy, laid!.goal * cell, 0, TAU);
    ctx.stroke();
    ctx.setLineDash([]);
    if (far > 0) {
      ctx.strokeStyle = veil(colours[s], 0.16);
      ctx.lineWidth = (won(state.over, s) ? WIN : LINE) * dpr * 5;
      ctx.beginPath();
      ctx.arc(hx, hy, far * cell, 0, TAU);
      ctx.stroke();
    }
    ctx.strokeStyle = colours[s];
    if (far > 0) {
      ctx.lineWidth = (won(state.over, s) ? WIN : LINE) * dpr;
      ctx.beginPath();
      ctx.arc(hx, hy, far * cell, 0, TAU);
      ctx.stroke();
    }
    const m = cell + 4 * dpr;
    ctx.strokeStyle = ink!.fg;
    ctx.lineWidth = MARK * dpr;
    ctx.strokeRect(hx - m / 2, hy - m / 2, m, m);
    ctx.restore();
  };
  const stage = (pen: OffscreenCanvasRenderingContext2D, w: number, h: number, dark: boolean) => {
    const { cell, edge, boards } = box!;
    const dpr = view.dpr;
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.globalAlpha = 1;
    pen.clearRect(0, 0, w, h);
    pen.fillStyle = blend(ink!.bg, colours[0], dark ? 0.06 : 0.05);
    pen.fillRect(0, 0, w, h);
    const big = Math.max(1, Math.max(w, h) * SPOT);
    const lamp = pen.createRadialGradient(w / 2, h / 2, 0, w / 2, h / 2, big);
    lamp.addColorStop(0, veil(colours[0], dark ? LAMP : LAMP / 2));
    lamp.addColorStop(1, veil(colours[0], 0));
    pen.fillStyle = lamp;
    pen.fillRect(0, 0, w, h);
    const pad = Math.max(0, Math.min(TRAY * dpr, cell * 3));
    const base = dark ? blend(blend(ink!.bg, BLACK, 0.4), colours[0], 0.06) : blend(WHITE, colours[0], 0.04);
    boards.forEach(({ x, y }, s) => {
      pen.save();
      pen.shadowColor = `rgba(0, 0, 0, ${dark ? 0.5 : 0.16})`;
      pen.shadowBlur = Math.max(1, pad * 2);
      pen.shadowOffsetY = pad * 0.35;
      pen.beginPath();
      pen.roundRect(x - pad, y - pad, edge + 2 * pad, edge + 2 * pad, Math.max(0, pad * 0.8));
      pen.fillStyle = blend(base, colours[s], 0.05);
      pen.fill();
      pen.shadowColor = "transparent";
      pen.strokeStyle = veil(dark ? WHITE : ink!.fg, dark ? 0.07 : 0.08);
      pen.lineWidth = Math.max(0.5, dpr * 0.75);
      pen.stroke();
      pen.restore();
    });
  };
  const finale = (dark: boolean) => {
    const { over } = show!.state;
    if (over?.how !== "goal") return;
    const k = view.still ? 1 : ease((view.t - ended) / FINISH);
    const { cell, edge, boards } = box!;
    const dpr = view.dpr;
    const pad = Math.max(0, Math.min(TRAY * dpr, cell * 3));
    boards.forEach(({ x, y }, s) => {
      if (!won(over, s)) {
        ctx.fillStyle = veil(ink!.bg, SHADE * k);
        ctx.fillRect(x, y, edge, edge);
        return;
      }
      ctx.strokeStyle = veil(colours[s], dark ? 0.9 : 0.75);
      ctx.lineWidth = Math.max(1, 2.5 * dpr * k);
      ctx.beginPath();
      ctx.roundRect(x - pad / 2, y - pad / 2, edge + pad, edge + pad, Math.max(0, pad * 0.5));
      ctx.stroke();
      if (k >= 1) return;
      const { home } = laid!.sides[s];
      if (home < 0) return;
      const hx = x + ((home % n) + 0.5) * cell;
      const hy = y + (Math.floor(home / n) + 0.5) * cell;
      ctx.save();
      ctx.beginPath();
      ctx.rect(x, y, edge, edge);
      ctx.clip();
      ctx.fillStyle = veil(colours[s], 0.22 * (1 - k));
      ctx.fillRect(x, y, edge, edge);
      for (const lag of [0, 0.18]) {
        const q = Math.max(0, Math.min(1, (k - lag) / (1 - lag)));
        if (q <= 0 || q >= 1) continue;
        ctx.strokeStyle = veil(blend(colours[s], WHITE, 0.35), 0.9 * (1 - q));
        ctx.lineWidth = Math.max(1, cell * 1.6 * (1 - q));
        ctx.beginPath();
        ctx.arc(hx, hy, Math.max(0, laid!.goal * cell + q * edge * 0.8), 0, TAU);
        ctx.stroke();
      }
      ctx.restore();
    });
  };
  const draw = () => {
    if (hands && view.fixed && phase === "ready") play();
    if (view.still && !hands) {
      if (!show!.state.over) show!.finish(laid!.still);
    } else if (phase === "running") {
      at = Math.max(at, clock.tick + Math.floor(((view.t - clock.t) * value.speed) / 1000));
      show!.to(at);
      if (hands && show!.state.over) phase = "done";
    }
    if (!show!.state.over) ended = -Infinity;
    else if (ended === -Infinity) ended = view.t;
    const closing = !view.still && view.t - ended < FINISH;
    if (!dirty && seen === show!.version && !closing) return;
    seen = show!.version;
    dirty = false;
    const dpr = view.dpr;
    box ??= lay(view.w, view.h, n, GAP * dpr, PAD * dpr);
    const dark = !bright(ink!.bg);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.globalAlpha = 1;
    const w = Math.max(1, Math.ceil(view.w));
    const h = Math.max(1, Math.ceil(view.h));
    if (!floor || floor.width !== w || floor.height !== h) {
      floor = new OffscreenCanvas(w, h);
      floored = "";
    }
    const key = `${colours.join()}|${ink!.bg}|${ink!.deep}|${box.cell}|${box.boards.map((b) => `${b.x},${b.y}`).join()}`;
    if (floored !== key) {
      stage(floor.getContext("2d")!, w, h, dark);
      floored = key;
    }
    ctx.drawImage(floor, 0, 0);
    const { edge } = box;
    laid!.sides.forEach((_, s) => {
      blit(s);
      const { x, y } = box!.boards[s];
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(sheets[s].page, x, y, edge, edge);
      ctx.imageSmoothingEnabled = true;
      ctx.globalCompositeOperation = dark ? "lighter" : "source-over";
      ctx.globalAlpha = dark ? 0.85 : 0.5;
      ctx.drawImage(sheets[s].halo, x, y, edge, edge);
      ctx.globalCompositeOperation = "source-over";
      ctx.globalAlpha = 1;
      rings(s);
    });
    finale(dark);
    tell(Boolean(show!.state.over) || show!.state.tick === 0);
  };
  const set = (next: Opts) => {
    const was = value;
    value = tidy(SPEC, next) as Knobs;
    const moved = next.seed !== seed;
    seed = next.seed;
    paint();
    if (moved || BOARD.some((key) => value[key] !== was[key])) {
      replan();
      if (moved) play();
    } else {
      if (value.walkers !== was.walkers) {
        laid!.walkers = value.walkers;
        laid!.still = budget(value.walkers);
        restart();
      }
      if (value.speed !== was.speed || value.heat !== was.heat) {
        bake(show!.trace, show!.state.tick, laid!.keep);
        laid!.keep = keeping(value.heat, value.speed);
        laid!.speed = value.speed;
        anchor();
      }
      if (value.finish !== was.finish) {
        laid!.goal = value.finish;
        retarget();
      }
    }
    dirty = true;
    tell(true);
  };
  const size = () => {
    box = null;
    dirty = true;
  };
  const theme = () => paint();
  const stop = () => {
    gone = true;
  };
  const svg = () => {
    const dpr = view.dpr;
    const css = lay(view.w / dpr, view.h / dpr, n, GAP, PAD);
    const { state } = show!;
    const now = reach(state);
    const c = css.cell;
    const body = laid!.sides.map(({ filled, home }, s) => {
      const { x, y } = css.boards[s];
      const hot = level(s);
      const base = runs({ shape: [n, n], types: filled }).map(
        ([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`,
      );
      const warm: string[] = [];
      for (let i = 0; i < hot.length; i++)
        if (hot[i] > 0)
          warm.push(
            `<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1" fill-opacity="${num(glow(hot[i]), 3)}"/>`,
          );
      const dots = Array.from(state.at[s], (i) => `<rect x="${i % n}" y="${Math.floor(i / n)}" width="1" height="1"/>`);
      const cells = `<g transform="translate(${num(x)} ${num(y)}) scale(${num(c, 4)})" shape-rendering="crispEdges"><rect width="${n}" height="${n}" fill="${ink!.deep}"/><g fill="${ink!.line}">${base.join("")}</g><g fill="${colours[s]}">${warm.join("")}${dots.join("")}</g></g>`;
      if (home < 0) return cells;
      const hx = x + ((home % n) + 0.5) * c;
      const hy = y + (Math.floor(home / n) + 0.5) * c;
      const ring = (r: number, width: number, dash: boolean) =>
        `<circle cx="${num(hx)}" cy="${num(hy)}" r="${num(r)}" fill="none" stroke="${colours[s]}" stroke-width="${num(width)}"${dash ? ` stroke-dasharray="${DASH.join(" ")}" stroke-opacity="${RING}"` : ""}/>`;
      const m = c + 4;
      const mark = `<rect x="${num(hx - m / 2)}" y="${num(hy - m / 2)}" width="${num(m)}" height="${num(m)}" fill="none" stroke="${ink!.fg}" stroke-width="${MARK}"/>`;
      return `${cells}<clipPath id="race-${s}"><rect x="${num(x)}" y="${num(y)}" width="${num(css.edge)}" height="${num(css.edge)}"/></clipPath><g clip-path="url(#race-${s})">${ring(laid!.goal * c, 1, true)}${now[s] > 0 ? ring(now[s] * c, won(state.over, s) ? WIN : LINE, false) : ""}${mark}</g>`;
    });
    return sheet(view.w / dpr, view.h / dpr, body.join(""));
  };
  const csv = () => ["step,a,b", ...show!.trace.log.map(([a, b], i) => `${i},${num(a, 3)},${num(b, 3)}`)].join("\n");
  paint();
  replan();
  return {
    draw,
    size,
    theme,
    state: () => show!.state,
    stop,
    set,
    svg,
    csv,
    stat,
    go,
    toggle: go,
    playing: () => phase === "running",
    phase: () => phase,
  };
}
