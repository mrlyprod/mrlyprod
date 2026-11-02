import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, flip, mirror, solve } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { BED, FIT, make, ray, SAVERS, SPEC, shared } from "./scene.ts";

const FRAME = 16;

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).endsWith("Gradient")
        ? () => ({ addColorStop: () => {} })
        : (...args: unknown[]) => log.push([key, ...args]),
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const open = (seed: number, value: Raw, still = false, lock = false, paper = "#000000", w = 640, h = 640) => {
  const log: unknown[][] = [];
  const told: Told[] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper, accent: "#008cff", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
    still,
    lock,
    w,
    h,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...defaults(SPEC),
    seed,
    ...value,
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

const at = (size: number, p: number): [number, number] => {
  const cell = (640 * FIT) / (size + 2 * BED);
  const left = (640 - size * cell) / 2;
  return [(left + ((p % size) + 0.5) * cell) / 640, (left + (Math.floor(p / size) + 0.5) * cell) / 640];
};

test("the beam is drawn from the lamp through each turn to the wall, and a tilt keeps the light before the mirror", () => {
  const state = deal(tidy(SPEC, {}) as Knobs, { seed: 3 });
  const path = ray(state);
  expect([path.points.length, path.arcs.length, path.arcs[0], path.arcs.at(-1)]).toEqual([
    path.points.length,
    path.points.length,
    0,
    path.length,
  ]);
  const p = path.hops.find((hop) => mirror(state.board[hop.p]))!.p;
  const tilted = { ...state, board: `${state.board.slice(0, p)}${flip(state.board[p])}${state.board.slice(p + 1)}` };
  const k = path.hops.findIndex((hop) => hop.p === p);
  expect([
    shared(path.hops, ray(tilted).hops) - shared(path.hops, path.hops.slice(0, k)),
    shared(path.hops, path.hops),
  ]).toEqual([0, Infinity]);
});

test("smart plays the room tap by tap in par, lights it, then deals again", () => {
  const live = open(7, { play: "smart" });
  live.run(60000, 32);
  const end = live.told.findIndex((one) => one.over === "done");
  const again = live.told.slice(end).find((one) => one.over === null)!;
  expect([end > 1, live.told[end].lit, live.told[end].taps, again.taps]).toEqual([
    true,
    live.told[end].of,
    live.told[end].par,
    0,
  ]);
});

test("one url deals one room, also after a knob goes and comes back", () => {
  const board = (seed: number) => open(seed, {}).scene.state().board;
  expect(board(7)).toBe(board(7));
  expect(board(7)).not.toBe(board(8));
  const live = open(7, {});
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, stars: 3 });
  live.scene.set({ ...value });
  expect(live.scene.state().board).toBe(board(7));
});

test("a tap on a mirror tilts it, and a tap off the mirrors does nothing", () => {
  const live = open(7, {}, true);
  const first = live.scene.state();
  const p = [...first.board].findIndex(mirror);
  live.scene.tap(...at(9, p));
  const after = live.scene.state();
  expect([after.board[p], after.taps]).toEqual([flip(first.board[p]), 1]);
  live.scene.tap(
    ...at(
      9,
      [...after.board].findIndex((one) => !mirror(one)),
    ),
  );
  live.scene.tap(0.001, 0.001);
  expect(live.scene.state()).toBe(after);
});

test("the arrows walk the room, X tilts the mirror in the ring, and Enter's hint plays the first tilt smart would", () => {
  const live = open(7, {}, true);
  const [x, y] = live.scene.ring();
  const start = live.scene.state();
  const target = [...start.board].findIndex(mirror);
  for (let i = x; i < target % 9; i++) live.scene.move("ArrowRight");
  for (let i = x; i > target % 9; i--) live.scene.move("ArrowLeft");
  for (let i = y; i < Math.floor(target / 9); i++) live.scene.move("ArrowDown");
  for (let i = y; i > Math.floor(target / 9); i--) live.scene.move("ArrowUp");
  live.scene.press();
  expect(live.scene.state().board[target]).toBe(flip(start.board[target]));
  const now = live.scene.state();
  const first = solve(now)![0];
  live.scene.hint();
  expect([live.scene.state().board[first], live.scene.state().taps]).toEqual([flip(now.board[first]), 2]);
});

test("pause holds smart, and play lets it go on", () => {
  const live = open(7, { play: "smart" });
  live.scene.toggle();
  live.run(5000);
  expect([live.scene.playing(), live.scene.state().taps]).toEqual([false, 0]);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().taps).toBeGreaterThan(0);
});

test("under reduced motion a seek of the world clock still lets smart play, frame by frame", () => {
  const live = open(3, { play: "smart", size: 14, mirrors: 36, stars: 7 }, true);
  const taps = [0, 1500, 3000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().taps;
  });
  expect(taps[0]).toBe(0);
  expect(taps[1]).toBeGreaterThan(taps[0]);
  expect(taps[2]).toBeGreaterThan(taps[1]);
});

test("the scene deals the state deal deals from the value and the seed", () => {
  for (const value of [{}, { size: 6, mirrors: 4, stars: 1 }, { size: 14, mirrors: 40, stars: 8 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed }));
});

test("the beam is drawn in a lighter light on dark paper than on light", () => {
  const beam = (paper: string) =>
    open(5, {}, true, false, paper)
      .log.filter(([key]) => key === "strokeStyle")
      .map(([, value]) => String(value));
  const light = (color: string) => (color.match(/\d+/g) ?? []).slice(0, 3).reduce((a, b) => a + Number(b), 0);
  expect(light(beam("#000000")[1])).toBeGreaterThan(light(beam("#ffffff")[1]));
});

test("the savers Lantern and Hall boot in the lock from their picks, paint, and draw no text", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["lantern", "Lantern"],
    ["hall", "Hall"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    const drawn = [...live.log];
    live.run(20000, 50);
    drawn.push(...live.log);
    expect([
      saver.id,
      live.scene.state().stars > 0,
      drawn.some(([key]) => key === "fill"),
      drawn.some(([key]) => String(key).endsWith("Text")),
    ]).toEqual([saver.id, true, true, false]);
  }
});

test("in the lock smart plays room after room", () => {
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(150000, 50);
    expect([saver.id, live.told.filter((one) => one.over === "done").length >= 2]).toEqual([saver.id, true]);
  }
});

test("the lock's exit: hold stops smart, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(3000);
  live.scene.hold();
  const taps = live.scene.state().taps;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().taps, landed - from <= 1000, still]).toEqual([taps, true, true]);
});

test("a stage smaller than its margins never hands the pen a negative radius", () => {
  for (const [w, h] of [
    [20, 20],
    [60, 40],
  ]) {
    const live = open(3, { play: "smart" }, false, false, "#000000", w, h);
    live.run(3000);
    live.scene.size();
    const bent = live.log.filter(
      ([key, , , ...r]) =>
        (key === "arc" && Number(r[0]) < 0) ||
        (key === "ellipse" && (Number(r[0]) < 0 || Number(r[1]) < 0)) ||
        (key === "roundRect" && [r[2]].flat().some((v) => Number(v) < 0)),
    );
    expect([w, h, bent]).toEqual([w, h, []]);
  }
});

test("every colour handed to the pen is finite, at the start and after world seeks, in light, dark and the lock", () => {
  const bad: unknown[] = [];
  const check = (value: unknown) => {
    if (typeof value === "string" && /NaN|Infinity|undefined/.test(value)) bad.push(value);
  };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).endsWith("Gradient")
        ? () => ({ addColorStop: (_at: number, color: string) => check(color) })
        : () => {},
    set: (_, _key, value) => {
      check(value);
      return true;
    },
  });
  for (const paper of ["#000000", "#ffffff"])
    for (const [value, lock, still] of [
      [{}, false, false],
      [{ play: "smart" }, false, false],
      [{ play: "smart" }, false, true],
      [SAVERS[1].pick(rng(2)), true, false],
    ] as [Raw, boolean, boolean][]) {
      const view = {
        rand: rng(1),
        look: () => ({ paper, accent: "#6768fa", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
        still,
        lock,
        w: 800,
        h: 600,
        dpr: 2,
        t: 0,
        fixed: false,
        wake: () => {},
      } as unknown as View;
      const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => ctx }), view, {
        ...defaults(SPEC),
        seed: 0,
        ...value,
      });
      scene.draw();
      for (const until of [500, 3000, 12000, 30000]) {
        while (view.t < until) {
          view.t = Math.min(until, view.t + 1000 / 60);
          scene.draw();
        }
        scene.size();
      }
    }
  expect(bad).toEqual([]);
});
