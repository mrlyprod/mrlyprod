import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { boot, deal, run, WEEK } from "./engine.ts";
import type { Told } from "./scene.ts";
import { make, SAVERS, SPEC } from "./scene.ts";

const FRAME = 16;

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) => {
        log.push([key, ...args]);
        return { width: 0, addColorStop: () => {} };
      },
    set: (_, key, value) => {
      log.push([key, value]);
      return true;
    },
  });

const open = (seed: number, value: Raw = {}, still = false, lock = false, w = 640, h = 480) => {
  const log: unknown[][] = [];
  const told: Told[] = [];
  const view = {
    rand: rng(seed),
    look: () => ({ paper: "#000000", accent: "#008cff", ink: "#ffffff" }),
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

test("one url runs one village: the engine's own run of the url seed, and another seed another", () => {
  const at = (seed: number) => {
    const live = open(seed);
    live.run(8000);
    return JSON.stringify(live.scene.state());
  };
  const own = run(
    boot({ bots: 5, decide: "vote", eager: 30, patience: 6, length: WEEK, seed: 7 }),
    Math.floor(8000 / 520),
  );
  expect(at(7)).toBe(JSON.stringify(own));
  expect(at(7)).not.toBe(at(8));
});

test("the scene boots the state deal deals, for several seeds and rules", () => {
  for (const seed of [0, 3, 41])
    for (const decide of ["vote", "ruler", "lottery"]) {
      const value = { ...defaults(SPEC), seed, decide, bots: 4 };
      expect(open(seed, value).scene.state()).toEqual(
        deal({ bots: 4, decide, eager: 30, patience: 6 } as never, { seed }),
      );
    }
});

test("flipping who decides keeps the village and its first wishes, and a tint keeps the run going", () => {
  const live = open(7);
  const first = live.scene.state();
  live.run(3000);
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, decide: "ruler" });
  const ruled = live.scene.state();
  expect([ruled.decide, ruled.tick, ruled.hours, ruled.wishes]).toEqual(["ruler", 0, first.hours, first.wishes]);
  live.run(3000);
  const going = live.scene.state().tick;
  live.scene.set({ ...value, decide: "ruler", tint: "red" });
  expect(live.scene.state().tick).toBe(going);
});

test("Enter steps one tick and pauses, pause holds the clock, and play lets it run", () => {
  const live = open(7);
  live.scene.poke();
  expect([live.scene.state().tick, live.scene.playing()]).toEqual([1, false]);
  live.run(5000);
  expect(live.scene.state().tick).toBe(1);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().tick).toBeGreaterThan(5);
});

test("under reduced motion a seek of the world clock still runs the village, frame by frame", () => {
  const live = open(3, {}, true);
  const ticks = [0, 1000, 2000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().tick;
  });
  expect(ticks[0]).toBe(0);
  expect(ticks[1]).toBeGreaterThan(ticks[0]);
  expect(ticks[2]).toBeGreaterThan(ticks[1]);
});

test("a week ends and holds on the page, and Enter runs it again", () => {
  const page = open(5);
  page.run(WEEK * 600);
  const end = page.scene.state();
  page.run(10000);
  expect([end.over, page.scene.state()]).toEqual(["done", end]);
  page.scene.poke();
  expect(page.scene.state().tick).toBe(0);
});

test("the savers Village weeks and Town weeks boot in the lock from their picks and paint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["village", "Village weeks"],
    ["town", "Town weeks"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    expect([saver.id, live.scene.state().tick, live.log.some(([key]) => key === "arc")]).toEqual([saver.id, 0, true]);
  }
});

test("in the lock week after week brings a new village under the next rule", () => {
  const order = ["vote", "ruler", "lottery"];
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(WEEK * 1200 + 4000, 50);
    const weeks = live.told.filter((one) => one.tick === 0);
    const first = order.indexOf(String(value.decide));
    expect<(string | string[])[]>([saver.id, weeks.slice(0, 3).map((one) => one.decide)]).toEqual([
      saver.id,
      [0, 1, 2].map((k) => order[(first + k) % 3]),
    ]);
  }
});

test("the lock's exit: hold stops the clock, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
  live.scene.hold();
  const tick = live.scene.state().tick;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().tick, landed - from <= 1000, still]).toEqual([tick, true, true]);
});

test("every colour handed to the pen is finite, at the start and after world seeks, in light, dark and the lock", () => {
  const bad: unknown[] = [];
  const check = (value: unknown) => {
    if (typeof value === "string" && /NaN|Infinity|undefined/.test(value)) bad.push(value);
    if (typeof value === "number" && !Number.isFinite(value)) bad.push(value);
  };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, _key) =>
      (...args: unknown[]) => {
        args.forEach(check);
        return { width: 0, addColorStop: (at: number, color: string) => [at, color].forEach(check) };
      },
    set: (_, _key, value) => {
      check(value);
      return true;
    },
  });
  for (const paper of ["#000000", "#ffffff"])
    for (const [value, lock] of [
      [{}, false],
      [{ decide: "ruler", bots: 3, patience: 1 }, false],
      [{ decide: "lottery", bots: 12, eager: 100, patience: 24 }, false],
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
      for (const until of [500, 3000, 12000, 40000]) {
        while (view.t < until) {
          view.t = Math.min(until, view.t + 100);
          scene.draw();
        }
        scene.size();
      }
    }
  expect(bad).toEqual([]);
});

test("a stage smaller than its margins never hands the pen a negative radius", () => {
  for (const [w, h] of [
    [20, 20],
    [60, 40],
  ]) {
    const live = open(3, {}, false, false, w, h);
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
