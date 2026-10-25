import type { tourbillon } from "@mrlyprod/mrly/math";
import type { Loadable, Page } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Spec, Value } from "../../lib/app/spec.ts";
import { tidy } from "../../lib/app/spec.ts";
import type { Touched } from "../../lib/app/touch.ts";
import { TINTS } from "../../lib/color.ts";
import type { Mrly } from "../../lib/designs/engine.ts";
import { colorizer, paint, RAMPS } from "../../lib/moire/engine.ts";
import { fit, shell } from "../../lib/moire/sheet.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import { wasm } from "../../lib/units/index.ts";
import type { Spun, Swatched } from "./engine.ts";
import {
  angle,
  BLENDS,
  BUDGET,
  DRAG,
  HUSH,
  LEAN,
  live,
  MODES,
  NUDGE,
  READ,
  REEL,
  reading,
  SCHEDULES,
  SETS,
  SIZES,
  SLOP,
  STAMP,
  SWEEP,
  settle,
  shade,
  spin,
  trim,
  WEIGHTS,
} from "./engine.ts";

/* TYPES */

export type Tourbillon = Swatched & { increment: number; size: number; tint: string };

export type Fault = { error: string };

export type Reading = Pick<
  tourbillon.Stats,
  "layers" | "scales" | "angles" | "mean" | "rms" | "faded" | "centre" | "peaks" | "classes" | "pairs"
> & { angle: number; playing: boolean; size: number; period: number | null; unused: boolean };

export type Facts = Fault | Reading;

export type Turn = { increment: number } | { sched: string };

export type Given = Opts & {
  math?: Mrly | null;
  onFacts?: (facts: Facts | null) => void;
  onTurn?: (patch: Turn) => void;
};

export type Player = Pick<Touched, "touch"> & {
  lift?: () => null;
  draw: () => void;
  stop: () => void;
  set?: (next: Opts) => void;
  toggle?: () => boolean;
  nudge?: (by: number) => number;
  theme?: () => void;
  size?: () => void;
  playing?: () => boolean;
  angle?: () => number;
  every?: number;
};

type Grab = { u: number; at: number; moved: boolean };

const OWN = () => false;

export const SPEC: Spec = [
  {
    key: "increment",
    label: "Increment",
    kind: "slider",
    def: 1,
    min: 0,
    max: 360,
    unit: "deg",
    group: "Spin",
    when: OWN,
  },
  { key: "sched", label: "Schedule", kind: "pick", def: "degrees", options: SCHEDULES, group: "Spin", when: OWN },
  { key: "limit", label: "Scales up to", kind: "slider", def: 55, min: 3, max: 99, step: 2, group: "Stack" },
  { key: "set", label: "Layers", kind: "pick", def: "odd", options: SETS, group: "Stack" },
  { key: "weights", label: "Weights", kind: "pick", def: "plain", options: WEIGHTS, group: "Stack" },
  { key: "size", label: "Size", kind: "segment", def: 512, options: SIZES.map((n) => [n, String(n)]), group: "Stack" },
  { key: "mode", label: "Mode", kind: "pick", def: "cells", options: MODES, group: "Draw" },
  { key: "blend", label: "Blend", kind: "pick", def: "mean", options: BLENDS, group: "Draw" },
  { key: "ramp", label: "Ramp", kind: "pick", def: "auto", options: [["auto", "Auto"], ...RAMPS], group: "Look" },
  { key: "levels", label: "Levels", kind: "slider", def: 0, min: 0, max: 64, step: 1, zero: "Auto", group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  gestures: true,
  name: (value) => `tourbillon-${trim(value.increment)}`,
  about: [
    "The odd parity carpet at the scales 1, 3, 5 and on, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.",
    "The centre is the one point every rotation fixes, so its value never moves. The carpet is unchanged by a quarter turn, so at an increment of 90a/q the layers fall into q angle classes and the spin goes dead: pick a quarter turn and watch it lock.",
    "Play sweeps the increment at 6 degrees a second and Stop lands the angle in the knobs and the link. Drag across the picture to turn it by hand, tap it to play or stop; the arrows nudge the angle by 0.05 degrees.",
    "Unspun, the stack is the one [Moire](/moire/) shows; spinning it breaks the shared grid, and the exact nodes that survive a turn are the Gaussian-integer angles. What a stack is lives in [the stack note](/research/stack/).",
  ],
  keys: [
    {
      key: "Enter",
      label: (_value: Value, scene: Pick<Player, "playing"> | null) => (scene?.playing?.() ? "Stop" : "Play"),
      act: "play",
      button: true,
      menu: "Edit",
    },
    { key: ["ArrowLeft", "ArrowRight"], label: "Nudge", act: "nudge" },
  ],
  actions: { play: (scene) => scene.toggle?.(), nudge: (scene, e) => scene.nudge?.(e.key === "ArrowLeft" ? -1 : 1) },
} satisfies Page<Loadable, Player>;

/* SAVERS */

const dial = (rand: Rng) => Math.round(rand() * 36000) / 100;

const odd = (rand: Rng, from: number, to: number) => from + 2 * Math.floor(rand() * ((to - from) / 2 + 1));

const stack = (rand: Rng, from: number, to: number, over: Partial<Tourbillon>) => ({
  increment: dial(rand),
  limit: odd(rand, from, to),
  set: "odd",
  weights: "plain",
  mode: "cells",
  blend: "mean",
  ramp: pick(rand, ["ink", "tint"]),
  levels: 0,
  size: 512,
  ...over,
});

export const SAVERS = [
  { id: "swirl", title: "Swirl", pick: (rand: Rng) => stack(rand, 25, 41, { weights: "harmonic" }) },
  {
    id: "lattices",
    title: "Lattices",
    pick: (rand: Rng) => stack(rand, 21, 41, { set: pick(rand, ["primes", "primes", "prime powers"]) }),
  },
  { id: "lace", title: "Lace", pick: (rand: Rng) => stack(rand, 21, 35, { weights: "harmonic", mode: "edges" }) },
];

export const units = wasm("math");

const read = (opts: Given) => ({ ...tidy(SPEC, opts), seed: Number(opts.seed) >>> 0 }) as Tourbillon;

/* SCENE */

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Player {
  const math = opts.math;
  if (!math) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  let value = read(opts);
  let playing = Boolean(view.lock);
  if (playing) value = { ...value, sched: "degrees" };
  let from = value.increment;
  let origin = 0;
  let held: (Spun & { key: string }) | null = null;
  let stats: tourbillon.Stats | null = null;
  let made = 0;
  let painted = "";
  let dirty = true;
  let error = "";
  let failed = "";
  let tone = 0;
  let sets = 0;
  let streak = 0;
  let stamped: number | null = null;
  let stampAt = 0;
  let readAt = -Infinity;
  let told = "";
  let grab: Grab | null = null;
  let gone = false;
  const planKey = () =>
    [value.limit, value.sched, value.set, value.weights, value.mode, value.blend, value.seed].join(" ");
  const shown = () => (playing ? angle(from, view.t - origin, view.lock ? HUSH : SWEEP) : value.increment);
  const facts = (): Facts | null => {
    if (error) return { error };
    if (!stats) return null;
    const { layers, scales, angles, mean, rms, faded, centre, weighted, peaks, period, classes, pairs } = stats;
    return {
      angle: held!.at,
      playing,
      size: held!.size,
      layers,
      scales,
      angles,
      mean,
      rms,
      faded,
      centre,
      peaks,
      period: period ?? null,
      classes,
      pairs,
      unused: !weighted && value.weights !== "plain",
    };
  };
  const tell = (now: boolean) => {
    if (gone || !opts.onFacts) return;
    if (playing && !now && view.t - readAt < READ) return;
    const next = facts();
    const key = JSON.stringify(next);
    if (key === told) return;
    told = key;
    readAt = view.t;
    opts.onFacts(next);
  };
  const turn = (patch: Turn) => {
    if ("increment" in patch) stamped = patch.increment;
    opts.onTurn?.(patch);
  };
  const compute = (at: number, size: number) => {
    held?.field.free();
    held = null;
    held = { ...spin(math, value, at, size), key: planKey() };
    stats = reading(math, value, held);
    made += 1;
  };
  const sync = () => {
    const at = shown();
    const fine = value.size;
    const quick = view.fixed || view.still ? fine : live(stats?.layers ?? 1, fine, view.lock ? LEAN : BUDGET);
    const busy = playing || streak > 1;
    const want = `${planKey()} ${at} ${busy ? quick : fine}`;
    if (want === failed) return;
    try {
      if (!held || held.key !== planKey() || held.at !== at) compute(at, busy ? quick : fine);
      else if (!busy && held.size !== fine) compute(at, fine);
      const how = shade(value, stats);
      const key = [made, how.ramp, how.levels, how.signed, value.invert, value.tint, tone].join(" ");
      if (key !== painted) {
        pad.put(
          paint(math, held!, colorizer(how.ramp, how.levels, view.look()), how.levels, value.invert, how.signed),
          held!.size,
        );
        painted = key;
        dirty = true;
      }
      error = "";
      failed = "";
    } catch (fault) {
      error = String((fault as { message?: unknown } | null | undefined)?.message ?? fault);
      failed = want;
      dirty = true;
    }
  };
  const draw = () => {
    if (view.fixed && !playing) toggle();
    streak = sets ? streak + 1 : 0;
    sets = 0;
    sync();
    if (playing && view.t - stampAt >= STAMP) {
      stampAt = view.t;
      turn({ increment: Number(shown().toFixed(2)) });
    }
    tell(false);
    if (!dirty) return;
    pad.clear();
    if (!error && held) pad.blit(held.size, fit(view.w, view.h), true);
    dirty = false;
  };
  const land = (at: number) => {
    playing = false;
    value = { ...value, increment: settle(at) };
    turn({ increment: value.increment });
    dirty = true;
    return value.increment;
  };
  const toggle = () => {
    if (playing) {
      land(shown());
      tell(true);
      return false;
    }
    playing = true;
    from = value.increment;
    origin = view.t;
    stampAt = view.t;
    if (value.sched !== "degrees") {
      value = { ...value, sched: "degrees" };
      turn({ sched: "degrees" });
    }
    view.wake?.();
    tell(true);
    return true;
  };
  const nudge = (by: number) => land(shown() + by * NUDGE);
  const set = (next: Opts) => {
    const was = value;
    value = read(next);
    sets += 1;
    if (value.increment !== was.increment && value.increment !== stamped) playing = false;
    if (value.sched !== "degrees") playing = false;
    if (!playing) from = value.increment;
    dirty = true;
  };
  const touch = (u: number, _v: number, start: boolean) => {
    if (start) {
      grab = { u, at: shown(), moved: false };
      return null;
    }
    if (!grab || (!grab.moved && Math.abs(u - grab.u) < SLOP)) return null;
    grab.moved = true;
    playing = false;
    return { increment: settle(grab.at + (u - grab.u) * DRAG) };
  };
  const lift = () => {
    const tap = grab && !grab.moved;
    grab = null;
    if (tap) toggle();
    return null;
  };
  const theme = () => {
    tone += 1;
    dirty = true;
  };
  const size = () => {
    dirty = true;
  };
  const stop = () => {
    gone = true;
    held?.field.free();
    held = null;
  };
  return {
    draw,
    set,
    toggle,
    nudge,
    touch,
    lift,
    theme,
    size,
    stop,
    playing: () => playing,
    angle: shown,
    every: view.lock ? REEL : 0,
  };
}
