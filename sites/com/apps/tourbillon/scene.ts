import type * as tourbillon from "@mrlyprod/tourbillon";
import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Crate, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { CRATES } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { pick } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, View } from "@mrlyprod/ui/kit/run.ts";
import type { Spec, Value } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Touched } from "@mrlyprod/ui/kit/touch.ts";
import type { Units as Loaded } from "@mrlyprod/ui/kit/units/index.ts";
import { wasm } from "@mrlyprod/ui/kit/units/index.ts";
import { colorizer, paint, RAMPS } from "../shared/moire/engine.ts";
import { fit, shell } from "../shared/moire/sheet.ts";
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

export type Tourbillon = Swatched & { increment: number; size: number; tint: string };

export type Fault = { error: string };

export type Reading = Pick<
  tourbillon.Stats,
  "layers" | "scales" | "angles" | "mean" | "rms" | "faded" | "centre" | "peaks" | "classes" | "pairs"
> & { angle: number; playing: boolean; size: number; period: number | null; unused: boolean };

export type Facts = Fault | Reading;

export type Turn = { increment: number } | { sched: string };

export type Given = Opts & {
  mrly?: Mrly | null;
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

export const units = wasm(...CRATES);

export const PAGE = {
  spec: SPEC,
  units,
  record: true,
  gestures: true,
  name: (value) => `tourbillon-${trim(value.increment)}`,
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
  reroll: () => ({ sched: "random" }),
} satisfies Page<Loaded<Crate>, Player>;

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

const read = (opts: Given) => ({ ...tidy(SPEC, opts), seed: Number(opts.seed) >>> 0 }) as Tourbillon;

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Player {
  const mrly = opts.mrly;
  if (!mrly) return { draw: () => {}, stop: () => {} };
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
    held = { ...spin(mrly, value, at, size), key: planKey() };
    stats = reading(mrly, value, held);
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
          paint(mrly, held!, colorizer(how.ramp, how.levels, view.look()), how.levels, value.invert, how.signed),
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
