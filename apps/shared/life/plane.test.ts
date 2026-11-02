import { expect, test } from "bun:test";
import * as life from "@mrlyprod/life";
import * as math from "@mrlyprod/math";
import { rng } from "../../../net/sdk/rand.ts";
import type { Opts, View } from "../../../net/sdk/run.ts";
import type { Value } from "../../../net/sdk/spec.ts";
import type { Stat } from "./plane.ts";
import { plane } from "./plane.ts";

const BASE: Value = {
  mode: "island",
  shape: "hline",
  side: 3,
  depth: 1,
  tiling: 1,
  cap: 31,
  x: 64,
  mask: "design",
  code: "7",
  number: 3,
  level: 1,
  born: "3",
  stay: "23",
  speed: 10,
};

const BASE_KEY = "hline.3.1 x1 island mask:code7.3.1 birth:3 survive:2,3 x:64 cap:31";

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) =>
        log.push([key, ...args]),
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const open = (value: Value, { w = 640, h = 480 } = {}) => {
  const log: unknown[][] = [];
  const told: Stat[] = [];
  const start = { ...BASE, seed: 7, ...value };
  const view = {
    rand: rng(7),
    look: () => ({ paper: "#000000", accent: "#008cff", ink: "#ffffff" }),
    still: false,
    lock: false,
    w,
    h,
    dpr: 1,
    t: 0,
    wake: () => {},
  } as View;
  const canvas = Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) });
  const read = (next: Opts) => ({ ...start, ...next }) as unknown as Value;
  const scene = plane(canvas, view, { ...start, life, math, read, onStat: (stat) => told.push(stat) });
  const at = (t: number) => {
    log.length = 0;
    view.t = t;
    scene.draw();
  };
  const last = () => told[told.length - 1];
  const set = (patch: Value) => scene.set(Object.assign(start, patch));
  return { log, told, scene, view, at, last, set };
};

const rects = (log: unknown[][]) => log.filter(([verb]) => verb === "rect");

test("an island opens at generation zero with the cap drawn as a faint square and the seed centred inside it", () => {
  const run = open({});
  expect(run.last()).toMatchObject({ gen: 0, pop: 3, fate: "", recipe: BASE_KEY, budget: 8, reach: 1, flips: false });
  const cell = Math.floor((480 * 0.96) / 31);
  const x = Math.floor((640 - 31 * cell) / 2);
  const y = Math.floor((480 - 31 * cell) / 2);
  expect(run.log.find(([verb]) => verb === "strokeRect")).toEqual([
    "strokeRect",
    x + 0.5,
    y + 0.5,
    31 * cell - 1,
    31 * cell - 1,
  ]);
  expect(rects(run.log)).toEqual([["rect", x + 14 * cell, y + 15 * cell, 3 * cell, cell]]);
});

test("play steps the run at the speed and pauses at its stop, which the status tells in the lab's words", () => {
  const run = open({});
  run.scene.toggle();
  run.at(1000);
  expect([run.last().gen, run.last().fate, run.scene.playing()]).toEqual([2, "loop after 2, period 2", false]);
  run.scene.step();
  expect(run.last().gen).toBe(2);
  run.scene.toggle();
  expect([run.last().gen, run.scene.playing()]).toEqual([0, true]);
});

test("a wallpaper tiles the torus over the whole stage, and a flipping rule draws the holes on a full plane", () => {
  const run = open({ mode: "wallpaper", shape: "code7", born: "3", stay: "23" }, { w: 100, h: 60 });
  const cell = Math.floor((60 * 0.96) / 3);
  expect(run.last()).toMatchObject({
    gen: 0,
    pop: 8,
    recipe: "code7.3.1 x1 wallpaper mask:code7.3.1 birth:3 survive:2,3 x:64",
  });
  expect(run.log.filter(([verb]) => verb === "fillRect")[0]).toEqual(["fillRect", 0, 0, 100, 60]);
  const span = 3 * cell;
  const copies = (at: number, size: number) => {
    let n = 0;
    for (let t = at - Math.ceil(at / span) * span; t < size; t += span) n++;
    return n;
  };
  expect(rects(run.log).length).toBe(
    4 * copies(Math.floor((100 - span) / 2), 100) * copies(Math.floor((60 - span) / 2), 60),
  );
  const flip = open({ mode: "wallpaper", shape: "code7", born: "0", stay: "8" }, { w: 100, h: 60 });
  flip.scene.step();
  expect(flip.last()).toMatchObject({ gen: 1, background: 1, flips: true });
  expect(flip.log.filter(([key, v]) => key === "fillStyle" && v === "#000000").length).toBe(1);
});

test("a knob change rebuilds the run, a bad side is told as an error, and a switch to the board asks for a remount", () => {
  const run = open({});
  run.scene.toggle();
  run.at(500);
  expect(run.set({ tiling: 3 })).toBe(true);
  expect(run.last()).toMatchObject({
    gen: 0,
    pop: 27,
    recipe: "hline.3.1 x3 island mask:code7.3.1 birth:3 survive:2,3 x:64 cap:31",
  });
  run.set({ stay: "-" });
  expect(run.last()).toMatchObject({ recipe: "", error: "stay names no count and no source", gen: 0, pop: 0 });
  expect(run.set({ mode: "board" })).toBe(false);
});

test("run to stop replays an extract row: the x5 island of code1 under evens loops after 15 with period 3 and 16 cells", () => {
  const run = open({
    shape: "code1",
    tiling: 5,
    cap: 151,
    x: 256,
    code: "1",
    born: "evens",
    stay: "evens",
  });
  run.scene.doom();
  expect(run.last()).toMatchObject({ gen: 15, pop: 16, fate: "loop after 15, period 3" });
});
