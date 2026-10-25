import { expect, test } from "bun:test";
import type { View } from "../../lib/app/run.ts";
import type { Raw } from "../../lib/app/spec.ts";
import { defaults } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Over } from "./engine.ts";
import { UP } from "./engine.ts";
import { make, SPEC } from "./scene.ts";

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

const open = (seed: number, value: Raw, still = false) => {
  const log: unknown[][] = [];
  const told: unknown[][] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still,
    w: 640,
    h: 480,
    dpr: 1,
    t: 0,
  } as View;
  view.wake = () => {
    view.still = false;
    told.push(["wake"]);
  };
  const hooks = {
    onScore: (score: number) => told.push(["score", score]),
    onOver: (over: Over) => told.push(["over", over]),
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    ...value,
    ...hooks,
  });
  const tick = () => {
    log.length = 0;
    view.t += scene.every;
    scene.draw();
  };
  return { log, told, scene, tick };
};

test("ticks step the game and a game over restarts after the hold", () => {
  const live = open(7, { play: "silly", size: 8, wrap: 0, apples: 10 });
  expect(live.told).toEqual([
    ["score", 0],
    ["over", null],
  ]);
  let ticks = 0;
  while (!live.told.some(([name, over]) => name === "over" && over) && ticks < 500) {
    live.tick();
    ticks++;
  }
  expect(live.told.filter(([name]) => name === "score").length).toBeGreaterThan(2);
  expect(live.told.at(-1)).toEqual(["over", expect.stringMatching(/^(wall|bite)$/)]);
  live.told.length = 0;
  for (let t = live.scene.every; t < 1000; t += live.scene.every) live.tick();
  expect(live.told).toEqual([]);
  live.tick();
  expect(live.told).toEqual([
    ["score", 0],
    ["over", null],
  ]);
});

test("one seed and one set of knobs give one board after 20 ticks", () => {
  const board = (seed: number) => {
    const live = open(seed, { play: "smart", size: 12, apples: 3 });
    for (let i = 0; i < 20; i++) live.tick();
    return live.log;
  };
  expect(board(7)).toEqual(board(7));
  expect(board(7)).not.toEqual(board(8));
});

test("under reduced motion me holds the starting board until the first turn wakes the run", () => {
  const live = open(7, { play: "me", player: true }, true);
  live.tick();
  const board = [...live.log];
  live.tick();
  expect(live.log).toEqual(board);
  live.scene.turn(UP);
  live.tick();
  expect(live.told.at(-1)).toEqual(["wake"]);
  expect(live.log).not.toEqual(board);
});
