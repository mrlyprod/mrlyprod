import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Ready } from "@mrlyprod/ui/kit/hooks.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { pick } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Option, Spec, Units, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Tile } from "../shared/designs/tiles.ts";
import { DESIGNS, ink, LEVELS, NUMBERS, tile } from "../shared/designs/tiles.ts";
import type { Act, Knobs as Moved } from "./engine.ts";
import { deal, FIT, setting, staging, step, ticking } from "./engine.ts";

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

export type Sleep = ReturnType<typeof make>;

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
  acts: { next: (value) => next(value, 1), prev: (value) => next(value, -1) },
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

const veil = (d: number) => 0.25 + 0.65 * d;

const MARKS = new Map<string, Tile>();

const markOf = (design: string, number: number, level: number) => {
  const id = `${design}.${number}.${level}`;
  if (!MARKS.has(id)) MARKS.set(id, tile(design, number, level));
  return MARKS.get(id)!;
};

const moved = ({ design, number, level, count, speed, size }: Knobs): Moved => ({
  design,
  number,
  level,
  count,
  speed,
  size,
});

export function next(value: Value, by: number): Shown {
  const at = TILES.findIndex(([design, number]) => design === value.design && number === value.number);
  const [design, number] = TILES[(at + by + TILES.length) % TILES.length];
  return { design, number };
}

export function make(canvas: HTMLCanvasElement, view: View, opts: Opts) {
  const ctx = canvas.getContext("2d")!;
  const stage = () => ({ w: view.w, h: view.h, dpr: view.dpr, lock: view.lock ? 1 : 0 });
  const seeded = (seed: number | undefined) => seed ?? Math.floor(view.rand() * 4294967296);
  let seed = opts.seed;
  let state = deal(tidy(SPEC, opts) as Knobs, { ...view, seed });
  let last = view.t;
  const act = (move: Act) => {
    const now = stage();
    if (now.w !== state.w || now.h !== state.h || now.dpr !== state.dpr || now.lock !== state.lock)
      state = step(state, staging(now));
    state = step(state, move);
  };
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = view.look().accent;
    for (const one of [...state.crowd].sort((a, b) => a.d - b.d)) {
      ctx.globalAlpha = veil(one.d);
      ink(ctx, markOf(one.design, one.number, one.level), Math.round(one.x), Math.round(one.y), one.side);
    }
    ctx.globalAlpha = 1;
  };
  const draw = () => {
    act(ticking(view.t - last));
    last = view.t;
    paint();
  };
  const size = () => act(FIT);
  const set = (next: Opts) => {
    const knobs = moved(tidy(SPEC, next) as Knobs);
    if (next.seed === seed) return act(setting(knobs));
    seed = next.seed;
    act(setting({ ...knobs, seed: seeded(seed) }));
  };
  return { draw, size, theme: paint, set, state: () => state };
}
