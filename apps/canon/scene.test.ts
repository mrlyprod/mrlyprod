import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, echoes, ONE, spot, TWO } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { make, SAVERS, SPEC } from "./scene.ts";

const FRAME = 16;
const SIZE = 640;
const FIT = 0.88;

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
    w: SIZE,
    h: SIZE,
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

const where = (state: ReturnType<ReturnType<typeof make>["state"]>, cell: number): [number, number] => {
  const side = state.number ** state.level;
  const c = Math.floor((SIZE * FIT) / side);
  const left = Math.floor((SIZE - c * side) / 2);
  const [x, y] = spot(state, cell);
  return [(left + (x + 0.5) * c) / SIZE, (left + (y + 0.5) * c) / SIZE];
};

const human = (seed: number) => {
  for (let s = seed; ; s++) {
    const live = open(s, { one: "me", two: "smart", level: 2 }, true);
    if (live.scene.state().turn === ONE) return live;
  }
};

test("two bots play a game to its end, hold the result, then deal again", () => {
  const live = open(7, { one: "smart", two: "random", level: 2 });
  live.run(40000);
  const at = live.told.findIndex((one) => one.over !== null);
  const again = live.told.slice(at).find((one) => one.over === null)!;
  expect([live.told[at].over, again.moves]).toEqual(["one", 0]);
});

test("one url deals one game, also after a knob goes and comes back", () => {
  const game = (seed: number) => open(seed, {}).scene.state();
  expect(game(7)).toEqual(game(7));
  const coins = new Set(Array.from({ length: 8 }, (_, seed) => game(seed).turn));
  expect(coins).toEqual(new Set([ONE, TWO]));
  const live = open(7, {});
  const value = { ...defaults(SPEC), seed: 7, math };
  live.scene.set({ ...value, level: 2 });
  live.scene.set({ ...value, level: 3 });
  expect(live.scene.state()).toEqual(game(7));
});

test("a tap claims the cell under the pointer with its echoes, and under reduced motion smart answers at once", () => {
  const live = human(1);
  const start = live.scene.state();
  const cell = 9;
  live.scene.tap(...where(start, cell));
  const after = live.scene.state();
  expect(echoes(start, cell).every((one) => after.board[one] === "1")).toBe(true);
  expect([after.moves, after.turn, [...after.board].filter((one) => one === "2").length > 0]).toEqual([2, ONE, true]);
});

test("a tap on a taken cell or between the cells claims nothing", () => {
  const live = human(1);
  live.scene.tap(...where(live.scene.state(), 9));
  const after = live.scene.state();
  live.scene.tap(...where(after, 9));
  live.scene.tap(0.01, 0.01);
  expect(live.scene.state()).toBe(after);
});

test("the arrows walk the cells and Enter claims the one in the ring", () => {
  const live = human(1);
  const state = live.scene.state();
  const from = live.scene.ring();
  live.scene.move("ArrowDown");
  const to = live.scene.ring();
  expect(spot(state, to)[1]).toBeGreaterThan(spot(state, from)[1]);
  live.scene.press();
  expect(live.scene.state().board[to]).toBe("1");
});

test("pause holds the bots, and play lets them go on", () => {
  const live = open(7, { one: "smart", two: "smart", level: 2 });
  live.scene.toggle();
  live.run(5000);
  expect([live.scene.playing(), live.scene.state().moves]).toEqual([false, 0]);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().moves).toBeGreaterThan(0);
});

test("under reduced motion a seek of the world clock still lets two bots play, frame by frame", () => {
  const live = open(3, { one: "smart", two: "greedy" }, true);
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
  for (const value of [{}, { code: "495", level: 2 }, { number: 5, level: 1, one: "smart" }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed, units: { math } }));
});

test("the savers Duet and Fugue boot two bots in the lock from their picks and paint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["duet", "Duet"],
    ["fugue", "Fugue"],
  ]);
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    expect([saver.id, [value.one, value.two].includes("me"), live.log.some(([key]) => key === "fill")]).toEqual([
      saver.id,
      false,
      true,
    ]);
  }
});

test("in the lock two bots finish a game, then play a new one", () => {
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(170000, 50);
    const at = live.told.findIndex((one) => one.over !== null);
    const again = live.told.slice(at).find((one) => one.over === null);
    expect([saver.id, at >= 0, again?.moves]).toEqual([saver.id, true, 0]);
  }
});

test("the lock's exit: hold stops the bots, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
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
