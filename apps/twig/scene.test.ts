import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, solve } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { make, mirror, nearest, SAVERS, SPEC, shape, within } from "./scene.ts";

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

const at = (levels: number, fork: number): [number, number] => {
  const { xs, ys } = shape(levels, 640, 640);
  return [xs[fork] / 640, ys[fork] / 640];
};

test("leaves sit evenly above their forks, a point finds its nearest fork, and a mirror flips a node across its crown", () => {
  const { xs, ys } = shape(3, 640, 640);
  expect([xs.length, xs[1] + xs[2] === 2 * xs[0], ys[7] < ys[3], ys[3] < ys[1]]).toEqual([15, true, true, true]);
  expect([nearest(3, shape(3, 640, 640), xs[4], ys[4] + 1), nearest(3, shape(3, 640, 640), 1, 1)]).toEqual([4, -1]);
  expect([mirror(0, 3), mirror(0, 1), mirror(1, 4), mirror(2, 5), within(5, 2), within(5, 1)]).toEqual([
    6,
    2,
    3,
    6,
    true,
    false,
  ]);
});

test("smart spins the tree into bloom spin by spin in par, then deals again", () => {
  const live = open(7, { play: "smart" });
  live.run(60000, 32);
  const end = live.told.findIndex((one) => one.over === "done");
  const again = live.told.slice(end).find((one) => one.over === null)!;
  expect([end > 1, live.told[end].neat, live.told[end].taps, again.taps]).toEqual([
    true,
    live.told[end].of,
    live.told[end].par,
    0,
  ]);
});

test("one url deals one tree, also after a knob goes and comes back", () => {
  const leaves = (seed: number) => open(seed, {}).scene.state().leaves;
  expect(leaves(7)).toBe(leaves(7));
  const live = open(7, {});
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, levels: 3 });
  live.scene.set({ ...value });
  expect(live.scene.state().leaves).toBe(leaves(7));
});

test("a tap on a fork spins it, and a tap far off the tree does nothing", () => {
  const live = open(7, {}, true);
  live.scene.tap(...at(4, 2));
  const after = live.scene.state();
  expect([after.taps, live.scene.fork()]).toEqual([1, 2]);
  live.scene.tap(0.001, 0.001);
  expect(live.scene.state()).toBe(after);
});

test("the arrows walk the forks, X spins the fork in the cursor, and Enter's hint plays the spin smart would", () => {
  const live = open(7, {}, true);
  live.scene.move("ArrowUp");
  live.scene.move("ArrowRight");
  live.scene.move("ArrowRight");
  live.scene.move("ArrowUp");
  live.scene.move("ArrowDown");
  expect(live.scene.fork()).toBe(2);
  live.scene.press();
  expect(live.scene.state().taps).toBe(1);
  const first = solve(live.scene.state())[0];
  live.scene.hint();
  expect([live.scene.fork(), live.scene.state().taps]).toEqual([first, 2]);
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
  const live = open(3, { play: "smart", levels: 5 }, true);
  const taps = [0, 100, 200].map((until) => {
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
  for (const value of [{}, { levels: 2 }, { levels: 5 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed }));
});

test("the savers Grove and Canopy boot in the lock from their picks, paint, and draw no text", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["grove", "Grove"],
    ["canopy", "Canopy"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    const drawn = [...live.log];
    live.run(20000, 50);
    drawn.push(...live.log);
    expect([
      saver.id,
      live.scene.state().levels > 0,
      drawn.some(([key]) => key === "fill"),
      drawn.some(([key]) => String(key).endsWith("Text")),
    ]).toEqual([saver.id, true, true, false]);
  }
});

test("in the lock smart plays tree after tree", () => {
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
    [0, 0],
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
        look: () => ({ paper, accent: "#ff325a", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
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
