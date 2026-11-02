import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, lay, RIGHT } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { make, SAVERS, SPEC } from "./scene.ts";

const FRAME = 16;

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

const open = (seed: number, value: Raw, still = false, lock = false) => {
  const log: unknown[][] = [];
  const told: Told[] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff", ink: "#ffffff" }),
    still,
    lock,
    w: 640,
    h: 640,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    seed,
    ...value,
    math,
    onTell: (one: Told) => told.push(one),
  });
  const run = (ms: number, frame = FRAME) => {
    for (let t = 0; t < ms; t += frame) {
      log.length = 0;
      view.t += frame;
      scene.draw();
    }
  };
  return { log, told, scene, run, view };
};

test("smart mends the deal turn by turn in par, holds the mended board, then deals again", () => {
  const live = open(7, { play: "smart", level: 2, mess: 6 });
  const first = live.scene.state();
  live.run(60000);
  const at = live.told.findIndex((one) => one.over === "mended");
  const again = live.told.slice(at).find((one) => one.over === null)!;
  expect([live.told[at].moves, again.moves]).toEqual([first.par, 0]);
});

test("one url deals one board, also after a knob goes and comes back", () => {
  const board = (seed: number) => open(seed, { level: 3 }).scene.state().turns;
  expect(board(7)).toEqual(board(7));
  expect(board(7)).not.toEqual(board(8));
  const live = open(7, { level: 3 });
  const value = { ...defaults(SPEC), seed: 7, math };
  live.scene.set({ ...value, level: 2 });
  live.scene.set({ ...value, level: 3 });
  expect(live.scene.state().turns).toEqual(board(7));
});

test("a tap turns the block of the chosen level under the pointer, and Enter steps a level down", () => {
  const live = open(7, { level: 2 }, true);
  const start = live.scene.state();
  expect(live.scene.level()).toBe(2);
  live.scene.deeper();
  expect([live.scene.level(), live.told.at(-1)!.level]).toEqual([1, 1]);
  live.scene.tap(0.06, 0.94, RIGHT);
  const after = live.scene.state();
  expect(after.moves).toBe(1);
  expect(after.turns.filter((one, i) => one !== start.turns[i])).toHaveLength(1);
  expect(after.turns[0]).toBe(start.turns[0]);
  live.scene.tap(0.94, 0.06, RIGHT);
  expect(live.scene.state()).toBe(after);
});

test("the arrows walk the blocks of a level and the period turns the one in the ring", () => {
  const live = open(7, { level: 2 }, true);
  live.scene.deeper();
  const top = (block: number) => lay(live.scene.state()).boxes[block * 3 + 1];
  const from = live.scene.ring();
  live.scene.move("ArrowDown");
  const to = live.scene.ring();
  expect([to > 0, top(to) > top(from)]).toEqual([true, true]);
  const before = live.scene.state();
  live.scene.twist(RIGHT);
  const after = live.scene.state();
  expect(after.turns.flatMap((one, i) => (one === before.turns[i] ? [] : [i]))).toEqual([to]);
});

test("pause holds smart, and play lets it go on", () => {
  const live = open(7, { play: "smart", level: 2, mess: 6 });
  live.scene.toggle();
  live.run(5000);
  expect([live.scene.playing(), live.scene.state().moves]).toEqual([false, 0]);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().moves).toBeGreaterThan(0);
});

test("under reduced motion a seek of the world clock still lets smart play, frame by frame", () => {
  const live = open(3, { play: "smart", code: "491", level: 3, mess: 20 }, true);
  const moved = [0, 1000, 2000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().moves;
  });
  expect(moved[0]).toBe(0);
  expect(moved[1]).toBeGreaterThan(moved[0]);
  expect(moved[2]).toBeGreaterThan(moved[1]);
});

test("the scene boots the state deal deals from the value and the seed", () => {
  for (const value of [{}, { code: "495", level: 2, mess: 4 }, { number: 5, level: 1, mess: 1 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed, units: { math } }));
});

test("the savers Mending and Deep mend boot in the lock from their picks and paint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["mending", "Mending"],
    ["deep", "Deep mend"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    expect([saver.id, live.scene.state().over, live.log.some(([key]) => key === "fill")]).toEqual([
      saver.id,
      null,
      true,
    ]);
  }
});

test("in the lock smart mends deal after deal", () => {
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(90000, 50);
    expect([saver.id, live.told.filter((one) => one.over === "mended").length >= 2]).toEqual([saver.id, true]);
  }
});

test("the lock's exit: hold stops smart, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(3000);
  live.scene.hold();
  const moves = live.scene.state().moves;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().moves, landed - from <= 1000, still]).toEqual([moves, true, true]);
});
