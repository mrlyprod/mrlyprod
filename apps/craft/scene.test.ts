import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { boot, DAYS, deal, run } from "./engine.ts";
import type { Told } from "./scene.ts";
import { BOTS, HUES, make, NAMES, SAVERS, SPEC } from "./scene.ts";

const FRAME = 16;
const PACE = 700;

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) => {
        log.push([key, ...args]);
        return { addColorStop: () => {} };
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

const fills = (log: unknown[][]) => new Set(log.filter(([key]) => key === "fillStyle").map(([, color]) => color));

test("one url runs one town: the engine's own run of the url seed, and another seed another", () => {
  const at = (seed: number) => {
    const live = open(seed);
    live.run(6000);
    return JSON.stringify(live.scene.state());
  };
  const own = run(boot({ bots: 48, work: "earn", teams: 1, length: DAYS, seed: 7 }), Math.floor(6000 / PACE));
  expect(at(7)).toBe(JSON.stringify(own));
  expect(at(7)).not.toBe(at(8));
});

test("the scene boots the state deal deals, for several seeds, rules and teams", () => {
  for (const seed of [0, 3, 41])
    for (const work of ["earn", "free"])
      for (const teams of [1, 5]) {
        const value = { ...defaults(SPEC), seed, work, teams, bots: 30 };
        expect(open(seed, value).scene.state()).toEqual(deal({ bots: 30, work, teams } as never, { seed }));
      }
});

test("turning the work rule keeps the bots and starts again, and a tint keeps the run going", () => {
  const live = open(7);
  const first = live.scene.state();
  live.run(3000);
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, work: "free" });
  const fresh = live.scene.state();
  expect([fresh.work, fresh.tick, fresh.love]).toEqual(["free", 0, first.love]);
  live.run(3000);
  const going = live.scene.state().tick;
  live.scene.set({ ...value, work: "free", tint: "red" });
  expect(live.scene.state().tick).toBe(going);
});

test("Enter steps one day and pauses, pause holds the clock, and play lets it run", () => {
  const live = open(7);
  live.scene.poke();
  expect([live.scene.state().tick, live.scene.playing()]).toEqual([1, false]);
  live.run(5000);
  expect(live.scene.state().tick).toBe(1);
  live.scene.toggle();
  live.run(5000);
  expect(live.scene.state().tick).toBeGreaterThan(5);
});

test("under reduced motion a seek of the world clock still runs the days, frame by frame", () => {
  const live = open(3, {}, true);
  const days = [0, 2000, 4000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().tick;
  });
  expect(days[0]).toBe(0);
  expect(days[1]).toBeGreaterThan(days[0]);
  expect(days[2]).toBeGreaterThan(days[1]);
});

test("a run ends and holds on the page, and Enter runs it again", () => {
  const page = open(5);
  page.run(DAYS * PACE + 2000, 50);
  const end = page.scene.state();
  page.run(10000, 50);
  expect([end.over, page.scene.state()]).toEqual(["done", end]);
  page.scene.poke();
  expect(page.scene.state().tick).toBe(0);
});

test("the savers Workshop days and Racing teams boot in the lock from their picks and paint every craft", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["street", "Workshop days"],
    ["race", "Racing teams"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(100);
    const painted = fills(live.log);
    expect([saver.id, live.scene.state().tick, HUES.every((hue) => painted.has(hue))]).toEqual([saver.id, 0, true]);
  }
});

test("in the lock run after run brings a new town under the other rule", () => {
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(DAYS * PACE * 3 + 20000, 100);
    const runs = live.told.filter((one) => one.tick === 0).map((one) => one.work);
    const other = value.work === "earn" ? "free" : "earn";
    expect<(string | string[])[]>([saver.id, runs.slice(0, 3)]).toEqual([saver.id, [value.work, other, value.work]]);
  }
});

test("the lock's exit: hold stops the clock, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
  live.scene.hold();
  const day = live.scene.state().tick;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().tick, landed - from <= 1000, still]).toEqual([day, true, true]);
});

test("a still lock shows a town well into its days at once", () => {
  const live = open(5, SAVERS[1].pick(rng(5)), true, true);
  expect(live.scene.state().tick).toBeGreaterThan(DAYS / 2);
});

test("a wide stage carries a legend of the crafts, the bots and fed, loved, idle and shipped; a square one and the lock keep to the town", () => {
  const said = (w: number, h: number, lock = false) => {
    const live = open(3, {}, false, lock, w, h);
    live.run(3000);
    return live.log.filter(([key]) => key === "fillText").map(([, text]) => String(text));
  };
  const wide = said(960, 540);
  expect([
    NAMES.every(([, text]) => wide.includes(text)),
    BOTS.every((text) => wide.includes(text)),
    ["fed", "loved", "idle", "shipped"].every((word) => wide.some((one) => one.endsWith(word))),
    said(600, 600),
    said(960, 540, true),
  ]).toEqual([true, true, true, [], []]);
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
      [{ work: "free", teams: 4, bots: 96 }, false],
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
