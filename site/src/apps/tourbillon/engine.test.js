import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import {
  angle,
  BLENDS,
  BUDGET,
  eyeOf,
  LEAST,
  label,
  live,
  MODES,
  reading,
  SCHEDULES,
  SETS,
  SWEEP,
  settle,
  shade,
  shown,
  spin,
  swatch,
  trim,
  WEIGHTS,
} from "./engine.js";

const value = (over) => ({
  limit: 21,
  sched: "degrees",
  set: "odd",
  weights: "plain",
  mode: "cells",
  blend: "mean",
  seed: 1,
  ramp: "auto",
  levels: 0,
  ...over,
});

const nans = (data) => data.reduce((sum, v) => sum + (Number.isNaN(v) ? 1 : 0), 0);

test("the sweep turns the increment at six degrees a second and wraps at a full turn", () => {
  expect([angle(1, 0), angle(1, 1000), angle(359, 1000), angle(10, -2000)]).toEqual([1, 1 + SWEEP, 5, 358]);
});

test("a landed angle keeps six places inside a full turn, and trims for the typed field", () => {
  expect([settle(8.181818181818), settle(-1), settle(400), trim(22.5), trim(1.0000000001)]).toEqual([
    8.181818,
    0,
    360,
    "22.5",
    "1",
  ]);
});

test("the live raster holds a frame to the budget of 28 layers at 384, never past the fine size or under the least", () => {
  expect([live(28, 512), live(28, 256), live(1, 1024), live(10000, 512)]).toEqual([384, 256, 1024, LEAST]);
  expect(live(50, 1024) ** 2 * 50).toBeLessThan(BUDGET * 1.1);
});

test("an eye names its quarter turn and a landed angle finds its eye", () => {
  const eyes = math.tourbillon.eyes(12);
  const eleven = eyes.find((eye) => eye.denom === 11);
  expect([label(eyes[0]), label(eyes.find((eye) => eye.angle === 22.5))]).toEqual(["0 deg", "22.5 deg = 90/4"]);
  expect(eyeOf(eyes, settle(eleven.angle))).toBe(String(eyes.indexOf(eleven)));
  expect(eyeOf(eyes, 22.4)).toBe("");
});

test("the picks are the door words: schedules, sets, weights, modes and blends", () => {
  const words = (list) => list.map(([key]) => key);
  expect(words(SCHEDULES)).toEqual(["unspun", "degrees", "golden", "primes", "random", "gaussian"]);
  expect([words(SETS).length, words(WEIGHTS), words(MODES), words(BLENDS).length]).toEqual([
    4,
    ["plain", "mobius", "harmonic"],
    ["cells", "edges", "corners"],
    6,
  ]);
  for (const [sched] of SCHEDULES) expect(math.tourbillon.layers(9, sched, 1, "odd", "plain", 1)).toHaveLength(5);
});

test("a spun field is masked to the disc and read at its own angle", () => {
  const held = spin(math, value(), 22.5, 64);
  const read = reading(math, value(), held);
  expect([held.size, held.at, Number.isNaN(held.data[0]), nans(held.data) < 4096 / 4]).toEqual([64, 22.5, true, true]);
  expect([read.layers, read.period, read.classes, read.pairs]).toEqual([11, 4, 4, 10]);
  held.field.free();
});

test("mobius weights under a weighted blend paint signed on the diverge ramp, one tone a count by default", () => {
  const held = spin(math, value({ weights: "mobius" }), 1, 32);
  const read = reading(math, value({ weights: "mobius" }), held);
  expect(shade(value({ weights: "mobius" }), read)).toEqual({ ramp: "diverge", levels: 12, signed: true });
  expect(shade(value({ weights: "mobius", blend: "parity" }), { ...read, weighted: false })).toEqual({
    ramp: "fire",
    levels: 12,
    signed: false,
  });
  expect(shade(value({ ramp: "tint", levels: 5 }), read)).toEqual({ ramp: "tint", levels: 5, signed: false });
  held.field.free();
});

test("the turns shelf holds the unit turns, all searches the names, and a swatch paints a square of the asked side", () => {
  const eyes = math.tourbillon.eyes(12);
  const unit = shown(eyes, "unit", "").map(([eye]) => label(eye));
  expect([unit.length, unit[0], unit.at(-1)]).toEqual([13, "0 deg", "90 deg"]);
  expect(shown(eyes, "all", " 22.5 ").map(([eye, at]) => [label(eye), eyes[at] === eye])).toContainEqual([
    "22.5 deg = 90/4",
    true,
  ]);
  expect(shown(eyes, "all", "").length).toBe(eyes.length);
  const out = swatch(math, value(), 22.5, 32, { paper: "#ffffff", accent: "#008cff" });
  expect([out.shape, out.colors.length]).toEqual([[32, 32], 32 * 32 * 4]);
});
