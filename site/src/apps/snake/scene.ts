import type { Ready } from "../../lib/app/hooks.ts";
import type { Fired } from "../../lib/app/keys.tsx";
import type { Handle, Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import { board } from "../../lib/app/run.ts";
import type { Row, Spec, Units } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import { TINTS, veil } from "../../lib/color.ts";
import type { Tile } from "../../lib/designs/tiles.ts";
import { DESIGNS, ink, NUMBERS, tile } from "../../lib/designs/tiles.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import type { Over, State } from "./engine.ts";
import { DOWN, LEFT, RIGHT, rounds, start, starve, step, turn, UP } from "./engine.ts";
import { pick as choose, STARVED } from "./policy.ts";

/* TYPES */

export type Part = "head" | "body" | "food";

export type Knobs = Record<Part, string> & {
  size: number;
  wrap: number;
  apples: number;
  tick: number;
  play: string;
  tint: string;
};

export type Told = { onScore: number; onOver: Over };

export type Hooks = { [K in keyof Told]?: (arg: Told[K]) => void };

export type Given = Opts & Hooks & { player?: boolean };

export type Snake = ReturnType<typeof make>;

export const PARTS: Part[] = ["head", "body", "food"];

const HIDE = () => false;

export const named = (word: string) => word[0].toUpperCase() + word.slice(1);

export const PIECES: [string, string][] = [
  ["", "Random"],
  ...DESIGNS.map((design): [string, string] => [design, named(design)]),
];

export const SPEC: Spec = [
  { key: "size", label: "Size", kind: "slider", def: 16, min: 8, max: 32, step: 2, group: "Board" },
  { key: "wrap", label: "Wrap", kind: "toggle", def: 1, group: "Board" },
  { key: "apples", label: "Apples", kind: "slider", def: 1, min: 1, max: 10, step: 1, group: "Board" },
  { key: "tick", label: "Tick", kind: "slider", def: 150, min: 50, max: 500, step: 25, unit: "ms", group: "Pace" },
  {
    key: "play",
    label: "Player",
    kind: "segment",
    def: "me",
    options: [
      ["me", "Me"],
      ["smart", "Smart"],
      ["silly", "Silly"],
    ],
    group: "Play",
  },
  ...PARTS.map(
    (part): Row => ({
      key: part,
      label: named(part),
      kind: "pick",
      def: "",
      options: PIECES,
      group: "Pieces",
      when: HIDE,
    }),
  ),
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

const ARROWS: Record<string, number> = { ArrowUp: UP, ArrowDown: DOWN, ArrowLeft: LEFT, ArrowRight: RIGHT };

const WASD: Record<string, number> = { w: UP, a: LEFT, s: DOWN, d: RIGHT };

const steer = (scene: Handle<Snake>, e: Fired) =>
  scene.turn({ ...ARROWS, ...WASD }[e.key!.length === 1 ? e.key!.toLowerCase() : e.key!]);

export const PAGE = {
  spec: SPEC,
  gestures: true,
  keys: [
    { key: Object.keys(ARROWS), label: "Turn", act: "arrows" },
    { key: "r", label: "New game", act: "reroll" },
    { key: "?", label: "Keys", act: "keys" },
    { key: Object.keys(WASD), label: "Turn", act: "wasd" },
  ],
  actions: { arrows: steer, wasd: steer },
} satisfies Page<Ready & Units, Snake>;

export const SAVERS = [
  {
    id: "solo",
    title: "One apple",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 12 + 2 * Math.floor(rand() * 5),
      wrap: pick(rand, [0, 1]),
      apples: 1,
      tick: 200,
    }),
  },
  {
    id: "feast",
    title: "Many apples",
    pick: (rand: Rng) => ({
      play: "smart",
      size: 20 + 2 * Math.floor(rand() * 4),
      wrap: pick(rand, [0, 1]),
      apples: 5 + Math.floor(rand() * 5),
      tick: 175,
    }),
  },
];

const TAU = Math.PI * 2;
const HOLD = 1000;
const FIT = 0.92;
const GROUND = 0.07;
const DIM = 0.35;
const PREVIEW = 160;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}) {
  const ctx = canvas.getContext("2d")!;
  const rand = view.rand;
  const value = tidy(SPEC, opts) as Knobs;
  const play = value.play;
  const auto = play !== "me" || !opts.player;
  let state: State;
  let marks: Record<string, Tile>;
  let ended = 0;
  let last = view.t;
  let gone = false;
  const tell = <K extends keyof Told>(name: K, arg: Told[K]) => !gone && (opts as Hooks)[name]?.(arg);
  const begin = () => {
    state = start(value, rand);
    marks = Object.fromEntries(
      PARTS.map((part) => {
        const design = pick(rand, DESIGNS);
        return [part, tile(value[part] || design, pick(rand, NUMBERS))];
      }),
    );
    ended = 0;
    tell("onScore", state.score);
    tell("onOver", state.over);
  };
  const advance = () => {
    if (state.over) {
      if (auto && view.t - ended >= HOLD) begin();
      return;
    }
    const score = state.score;
    if (auto && !state.queue.length) {
      const dir = choose(state, play, rand);
      state = dir === STARVED ? starve(state) : turn(state, dir);
    }
    state = step(state, rand);
    if (state.score !== score) tell("onScore", state.score);
    if (!state.over) return;
    ended = view.t;
    tell("onOver", state.over);
  };
  const paint = () => {
    const n = state.size;
    const { x: left, y: top, w: side, cell } = board(view, n, n, FIT);
    const { accent } = view.look();
    const piece = (mark: Tile, at: number, corners: number[] | null) => {
      const x = left + (at % n) * cell;
      const y = top + Math.floor(at / n) * cell;
      ctx.save();
      ctx.beginPath();
      if (corners)
        ctx.roundRect(
          x,
          y,
          cell,
          cell,
          corners.map((one) => (one * cell) / 2),
        );
      else ctx.arc(x + cell / 2, y + cell / 2, cell / 2, 0, TAU);
      ctx.clip();
      ink(ctx, mark, x, y, cell);
      ctx.restore();
    };
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.globalAlpha = 1;
    ctx.fillStyle = veil(accent, GROUND);
    ctx.fillRect(left, top, side, side);
    ctx.globalAlpha = state.over ? DIM : 1;
    ctx.fillStyle = accent;
    for (const at of state.foods) piece(marks.food, at, null);
    const ends = rounds(state);
    for (const [i, at] of state.snake.entries()) piece(i ? marks.body : marks.head, at, ends[i]);
  };
  const draw = () => {
    if (!view.still && view.t !== last) advance();
    last = view.t;
    paint();
  };
  const restart = () => {
    begin();
    paint();
  };
  const stop = () => {
    gone = true;
  };
  const move = (dir: number) => {
    state = turn(state, dir);
    if (!auto) view.wake?.();
  };
  begin();
  if (view.lock && view.still) for (let i = 0; i < PREVIEW && !state!.over; i++) advance();
  paint();
  return { draw, every: value.tick, size: paint, theme: paint, turn: move, restart, stop };
}
