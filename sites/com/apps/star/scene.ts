import { TINTS } from "@mrlyprod/ui/kit/color.ts";
import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Page } from "@mrlyprod/ui/kit/page.tsx";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { rng } from "@mrlyprod/ui/kit/rand.ts";
import type { Opts, Scene, View } from "@mrlyprod/ui/kit/run.ts";
import type { Spec } from "@mrlyprod/ui/kit/spec.ts";
import { tidy } from "@mrlyprod/ui/kit/spec.ts";
import type { Ramp } from "../shared/moire/engine.ts";
import { colorizer, paint, RAMPS } from "../shared/moire/engine.ts";
import { fit, shell } from "../shared/moire/sheet.ts";
import type { Law, Stack } from "./engine.ts";
import { CARPET, cutter, field, LEAST, law, MOST, more, SIZES } from "./engine.ts";

export type Star = {
  layers: number;
  size: number;
  ramp: Ramp;
  levels: number;
  invert: number;
  tint: string;
  half: number;
  seed: number;
};

export type Fault = { error: string };

export type Reading = Law & { size: number; low: number; high: number };

export type Facts = Fault | Reading;

export type Given = Opts & { mrly?: Mrly | null; onFacts?: (facts: Facts) => void };

export const SPEC: Spec = [
  { key: "layers", label: "Layers L", kind: "slider", def: 28, min: LEAST, max: MOST, step: 2, group: "Stack" },
  { key: "size", label: "Size", kind: "segment", def: 384, options: SIZES.map((n) => [n, String(n)]), group: "Stack" },
  { key: "ramp", label: "Ramp", kind: "pick", def: "fire", options: RAMPS, group: "Look" },
  { key: "levels", label: "Levels", kind: "slider", def: 24, min: 2, max: 64, step: 1, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
  { key: "half", label: "Band W", kind: "slider", def: 0, min: 0, max: 16, step: 1, unit: "cells", group: "Readings" },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  name: (value) => `star-${value.layers}`,
  reroll: (_value, seed) => drawn(rng(seed)),
} satisfies Page;

export const drawn = (rand: Rng) => ({ layers: LEAST + 2 * Math.floor(rand() * ((MOST - LEAST) / 2 + 1)) });

const read = (opts: Given) => ({ ...tidy(SPEC, opts), seed: Number(opts.seed) >>> 0 }) as Star;

export function make(canvas: HTMLCanvasElement, view: View, opts: Given = {}): Scene {
  const mrly = opts.mrly;
  if (!mrly) return { draw: () => {}, stop: () => {} };
  const pad = shell(canvas, view);
  const star = new mrly.star.Star(CARPET);
  const cuts = cutter(mrly);
  let value = read(opts);
  let held: Stack | null = null;
  let reading: Law | null = null;
  let error = "";
  let failed = "";
  let tone = 0;
  let dirty = true;
  let told = "";
  let gone = false;
  const done = { field: "", paint: "", law: "" };
  const fieldKey = () => `${value.layers} ${value.size}`;
  const paintKey = () => [fieldKey(), value.ramp, value.levels, value.invert, value.tint, tone].join(" ");
  const lawKey = () => `${value.layers} ${value.half}`;
  const tell = () => {
    if (gone || !opts.onFacts) return;
    const now = error ? { error } : { ...reading!, size: held!.size, low: held!.low, high: held!.high };
    const key = JSON.stringify(now);
    if (key === told) return;
    told = key;
    opts.onFacts(now);
  };
  const sync = () => {
    const want = `${paintKey()} ${lawKey()}`;
    if (want === failed) return;
    try {
      if (done.field !== fieldKey()) {
        held?.field.free();
        held = null;
        held = field(mrly, cuts, value.layers, value.size);
        done.field = fieldKey();
      }
      if (done.paint !== paintKey()) {
        pad.put(
          paint(mrly, held!, colorizer(value.ramp, value.levels, view.look()), value.levels, value.invert),
          held!.size,
        );
        done.paint = paintKey();
        dirty = true;
      }
      if (done.law !== lawKey()) {
        reading = law(mrly, star, value.layers, value.half);
        done.law = lawKey();
      }
      error = "";
      failed = "";
    } catch (fault) {
      error = String((fault as { message?: unknown } | null | undefined)?.message ?? fault);
      failed = want;
      dirty = true;
    }
    tell();
  };
  const draw = () => {
    sync();
    if (!dirty) return;
    pad.clear();
    if (!error && held) pad.blit(held.size, fit(view.w, view.h), true);
    dirty = false;
  };
  const set = (next: Opts) => {
    value = read(next);
    dirty = true;
  };
  const touch = (_u: number, _v: number, start: boolean) => (start ? { layers: more(value.layers) } : null);
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
    cuts.free();
    star.free();
  };
  return { draw, set, touch, theme, size, stop, still: true };
}
