import type { Ready } from "../../lib/app/hooks.ts";
import type { Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Option, Spec, Units, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { TINTS } from "../../lib/color.ts";
import type { Tile } from "../../lib/designs/tiles.ts";
import { DESIGNS, ink, LEVELS, NUMBERS, tile } from "../../lib/designs/tiles.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";

/* TYPES */

export type Knobs = {
  design: string;
  number: number | "";
  level: number | "";
  count: number;
  speed: number;
  size: number;
  tint: string;
};

export type Shown = Pick<Knobs, "design" | "number">;

export type Bounce = { p: number; v: number; hit: boolean };

export type Sleeper = { d: number; mark: Tile; vx: number; vy: number; x: number; y: number; side: number };

export type Sleep = ReturnType<typeof make> & { step?: (by: number) => void };

const HIDE = () => false;

const named = (word: string) => word[0].toUpperCase() + word.slice(1);

const MIXED: Option = ["", "Mixed"];

export const TILES: [string, number | ""][] = [
  ["", ""],
  ...DESIGNS.flatMap((design) => NUMBERS.map((n): [string, number] => [design, n])),
];

export const SPEC: Spec = [
  {
    key: "design",
    label: "Design",
    kind: "pick",
    def: "",
    options: [MIXED, ...DESIGNS.map((design): Option => [design, named(design)])],
    group: "Shape",
    when: HIDE,
  },
  {
    key: "number",
    label: "Number",
    kind: "pick",
    def: "",
    options: [MIXED, ...NUMBERS.map((n): Option => [n, String(n)])],
    group: "Shape",
    when: HIDE,
  },
  {
    key: "level",
    label: "Level",
    kind: "pick",
    def: "",
    options: [MIXED, ...LEVELS.map((n): Option => [n, String(n)])],
    group: "Shape",
  },
  { key: "count", label: "Tiles", kind: "slider", def: 8, min: 1, max: 16, step: 1, group: "Shape" },
  {
    key: "speed",
    label: "Speed",
    kind: "slider",
    def: 180,
    min: 60,
    max: 600,
    step: 20,
    unit: "px/s",
    group: "Motion",
  },
  { key: "size", label: "Size", kind: "slider", def: 0.42, min: 0.1, max: 0.8, step: 0.02, group: "Size" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: "Enter", label: "Next tile", act: "next", button: true, menu: "View" },
    { key: ["ArrowLeft", "ArrowUp"], label: "Previous tile", act: "prev" },
    { key: ["ArrowRight", "ArrowDown"], label: "Next", act: "next" },
  ],
  actions: { next: (scene) => scene.step?.(1), prev: (scene) => scene.step?.(-1) },
} satisfies Page<Ready & Units, Sleep>;

export const SAVERS = [
  {
    id: "crowd",
    title: "Crowd",
    pick: (rand: Rng) => ({ design: "", count: 7 + Math.floor(rand() * 3), size: 0.42, speed: 80 }),
  },
  {
    id: "giants",
    title: "Giants",
    pick: (rand: Rng) => ({ design: pick(rand, DESIGNS), count: 3, size: 0.7, speed: 60 }),
  },
];

const DEEP = 0.4;

const GOLD = 0.618034;

const depth = (k: number) => DEEP + (1 - DEEP) * ((k * GOLD) % 1);

const veil = (d: number) => 0.25 + 0.65 * d;

const FINE = 25;

export function next(value: Value, by: number): Shown {
  const at = TILES.findIndex(([design, number]) => design === value.design && number === value.number);
  const [design, number] = TILES[(at + by + TILES.length) % TILES.length];
  return { design, number };
}

export function bounce(p: number, v: number, max: number): Bounce {
  if (p <= 0) return { p: Math.min(-p, max), v: Math.abs(v), hit: true };
  if (p >= max) return { p: Math.max(2 * max - p, 0), v: -Math.abs(v), hit: true };
  return { p, v, hit: false };
}

export function make(canvas: HTMLCanvasElement, view: View, opts: Opts) {
  const ctx = canvas.getContext("2d")!;
  const rand = () => view.rand();
  let knobs = tidy(SPEC, opts) as Knobs;
  let seed = opts.seed;
  let crowd: Sleeper[] = [];
  let order: Sleeper[] = [];
  let last = view.t;
  const roll = () => {
    const design = knobs.design || pick(rand, DESIGNS);
    const number = knobs.number || pick(rand, NUMBERS);
    const level = knobs.level || pick(rand, LEVELS);
    return tile(design, number, view.lock && number ** level > FINE ? 1 : level);
  };
  const fit = (one: Sleeper) => {
    const n = one.mark.size;
    const reach = Math.min(Math.sqrt(view.w * view.h) * knobs.size * one.d, view.w, view.h);
    one.side = Math.max(1, Math.floor(reach / n)) * n;
    one.x = Math.min(one.x, Math.max(0, view.w - one.side));
    one.y = Math.min(one.y, Math.max(0, view.h - one.side));
  };
  const spawn = (k: number) => {
    const one: Sleeper = {
      d: depth(k),
      mark: roll(),
      vx: (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand()),
      vy: (rand() < 0.5 ? 1 : -1) * (0.6 + 0.8 * rand()),
      x: 0,
      y: 0,
      side: 0,
    };
    fit(one);
    one.x = rand() * Math.max(0, view.w - one.side);
    one.y = rand() * Math.max(0, view.h - one.side);
    return one;
  };
  const gather = () => {
    crowd.length = Math.min(crowd.length, knobs.count);
    while (crowd.length < knobs.count) crowd.push(spawn(crowd.length));
    order = [...crowd].sort((a, b) => a.d - b.d);
  };
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = view.look().accent;
    for (const one of order) {
      ctx.globalAlpha = veil(one.d);
      ink(ctx, one.mark, Math.round(one.x), Math.round(one.y), one.side);
    }
    ctx.globalAlpha = 1;
  };
  const move = (one: Sleeper, run: number) => {
    const hx = bounce(one.x + one.vx * run * one.d, one.vx, Math.max(0, view.w - one.side));
    const hy = bounce(one.y + one.vy * run * one.d, one.vy, Math.max(0, view.h - one.side));
    one.x = hx.p;
    one.vx = hx.v;
    one.y = hy.p;
    one.vy = hy.v;
    if (!hx.hit && !hy.hit) return;
    one.mark = roll();
    fit(one);
  };
  const draw = () => {
    const run = (knobs.speed * view.dpr * (view.t - last)) / 1000;
    last = view.t;
    if (run > 0) for (const one of crowd) move(one, run);
    paint();
  };
  const size = () => crowd.forEach(fit);
  const set = (next: Opts) => {
    const was = knobs;
    knobs = tidy(SPEC, next) as Knobs;
    if (next.seed !== seed) {
      seed = next.seed;
      crowd = [];
    } else if ((["design", "number", "level"] satisfies (keyof Knobs)[]).some((key) => knobs[key] !== was[key])) {
      for (const one of crowd) one.mark = roll();
    }
    size();
    gather();
  };
  gather();
  return { draw, size, theme: paint, set };
}
