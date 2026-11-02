import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults, tidy } from "../../net/sdk/spec.ts";
import { deal, sheet } from "./engine.ts";
import type { Knobs, Told } from "./scene.ts";
import { FIT, make, SAVERS, SPEC } from "./scene.ts";

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

const at = (size: number, x: number, y: number): [number, number] => {
  const cell = (640 * FIT) / size;
  const left = (640 - size * cell) / 2;
  return [(left + (x + 0.5) * cell) / 640, (left + (y + 0.5) * cell) / 640];
};

test("smart makes the lace move by move in par, opens it, then deals again", () => {
  const live = open(7, { play: "smart", size: 6, folds: 2 });
  const first = live.scene.state();
  live.run(60000);
  const end = live.told.findIndex((one) => one.over === "done");
  const again = live.told.slice(end).find((one) => one.over === null)!;
  expect([live.told[end].moves, live.told[end].holes, again.moves]).toEqual([first.par, live.told[end].of, 0]);
});

test("one url deals one sheet, also after a knob goes and comes back", () => {
  const lace = (seed: number) => open(seed, {}).scene.state().target;
  expect(lace(7)).toBe(lace(7));
  expect(lace(7)).not.toBe(lace(8));
  const live = open(7, {});
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, folds: 5 });
  live.scene.set({ ...value });
  expect(live.scene.state().target).toBe(lace(7));
});

test("a tap on a crease folds the smaller side over it, and a tap on a cell punches it", () => {
  const live = open(7, { size: 8 }, true);
  live.scene.tap(...at(8, 1.5, 3));
  const folded = live.scene.state();
  expect([folded.moves, sheet(folded).xs.lo, sheet(folded).xs.hi]).toEqual([1, 2, 8]);
  const ring = folded.target.indexOf("1");
  const fresh = open(7, { size: 8 }, true);
  fresh.scene.tap(...at(8, ring % 8, Math.floor(ring / 8)));
  expect(fresh.scene.state().holes[ring]).toBe("1");
});

test("the arrows walk the sheet, Enter punches the cell in the ring, and D folds at its right edge", () => {
  const live = open(7, { size: 8 }, true);
  const ring = live.scene.state().target.indexOf("1");
  const [x, y] = live.scene.ring();
  for (let i = x; i < ring % 8; i++) live.scene.move("ArrowRight");
  for (let i = x; i > ring % 8; i--) live.scene.move("ArrowLeft");
  for (let i = y; i < Math.floor(ring / 8); i++) live.scene.move("ArrowDown");
  for (let i = y; i > Math.floor(ring / 8); i--) live.scene.move("ArrowUp");
  live.scene.press();
  expect(live.scene.state().holes[ring]).toBe("1");
  const before = live.scene.state().moves;
  live.scene.bend("d");
  expect(live.scene.state().folds).toHaveLength(ring % 8 === 7 ? 0 : 1);
  expect(live.scene.state().moves).toBe(before + (ring % 8 === 7 ? 0 : 1));
});

test("pause holds smart, and play lets it go on", () => {
  const live = open(7, { play: "smart", size: 6, folds: 2 });
  live.scene.toggle();
  live.run(5000);
  expect([live.scene.playing(), live.scene.state().moves]).toEqual([false, 0]);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().moves).toBeGreaterThan(0);
});

test("under reduced motion a seek of the world clock still lets smart play, frame by frame", () => {
  const live = open(3, { play: "smart", size: 9, folds: 5 }, true);
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

test("the scene deals the state deal deals from the value and the seed", () => {
  for (const value of [{}, { size: 4, folds: 1 }, { size: 9, folds: 6 }] as Raw[])
    for (const seed of [1, 7, 42])
      expect(open(seed, value).scene.state()).toEqual(deal(tidy(SPEC, value) as Knobs, { seed }));
});

test("the savers Lace and Doily boot in the lock from their picks and paint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["lace", "Lace"],
    ["doily", "Doily"],
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

test("in the lock smart makes lace after lace", () => {
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(90000, 50);
    expect([saver.id, live.told.filter((one) => one.over === "done").length >= 2]).toEqual([saver.id, true]);
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
    for (const [value, lock] of [
      [{}, false],
      [{ play: "smart" }, false],
      [SAVERS[0].pick(rng(2)), true],
      [SAVERS[1].pick(rng(2)), true],
    ] as [Raw, boolean][]) {
      const view = {
        rand: rng(1),
        look: () => ({ paper, accent: "#6768fa", ink: paper === "#000000" ? "#ffffff" : "#000000" }),
        still: false,
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
      for (const until of [500, 3000, 12000]) {
        while (view.t < until) {
          view.t = Math.min(until, view.t + 1000 / 60);
          scene.draw();
        }
        scene.size();
      }
    }
  expect(bad).toEqual([]);
});
