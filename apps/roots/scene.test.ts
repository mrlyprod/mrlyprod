import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { boot, deal, run, YEARS } from "./engine.ts";
import type { Told } from "./scene.ts";
import { HUES, hue, make, order, SAVERS, SIGNS, SPEC } from "./scene.ts";

const FRAME = 16;
const PACE = 150;

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

test("one url runs one world: the engine's own run of the url seed, and another seed another", () => {
  const at = (seed: number) => {
    const live = open(seed);
    live.run(6000);
    return JSON.stringify(live.scene.state());
  };
  const own = run(boot({ families: 16, inherit: "heir", length: YEARS, seed: 7 }), Math.floor(6000 / PACE));
  expect(at(7)).toBe(JSON.stringify(own));
  expect(at(7)).not.toBe(at(8));
});

test("the scene boots the state deal deals, for several seeds, rules and family counts", () => {
  for (const seed of [0, 3, 41])
    for (const inherit of ["heir", "commons", "none"])
      for (const families of [8, 28]) {
        const value = { ...defaults(SPEC), seed, inherit, families };
        expect(open(seed, value).scene.state()).toEqual(deal({ families, inherit } as never, { seed }));
      }
});

test("the commons sit in the middle of the river, between the families", () => {
  expect([order(4), order(5)]).toEqual([
    [0, 1, -1, 2, 3],
    [0, 1, -1, 2, 3, 4],
  ]);
});

test("turning the rule starts the same families again, and a tint keeps the run going", () => {
  const live = open(7);
  const first = live.scene.state();
  live.run(3000);
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, inherit: "none" });
  const fresh = live.scene.state();
  expect([fresh.inherit, fresh.year, fresh.span]).toEqual(["none", 0, first.span]);
  live.run(3000);
  const going = live.scene.state().year;
  live.scene.set({ ...value, inherit: "none", tint: "red" });
  expect(live.scene.state().year).toBe(going);
});

test("Enter steps one year and pauses, pause holds the clock, and play lets it run", () => {
  const live = open(7);
  live.scene.poke();
  expect([live.scene.state().year, live.scene.playing()]).toEqual([1, false]);
  live.run(3000);
  expect(live.scene.state().year).toBe(1);
  live.scene.toggle();
  live.run(3000);
  expect(live.scene.state().year).toBeGreaterThan(5);
});

test("under reduced motion a seek of the world clock still runs the years, frame by frame", () => {
  const live = open(3, {}, true);
  const seen = [0, 2000, 4000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().year;
  });
  expect(seen[0]).toBe(0);
  expect(seen[1]).toBeGreaterThan(seen[0]);
  expect(seen[2]).toBeGreaterThan(seen[1]);
});

test("a run ends and holds on the page, and Enter runs it again", () => {
  const page = open(5);
  page.run(YEARS * PACE + 2000, 50);
  const end = page.scene.state();
  page.run(10000, 50);
  expect([end.over, page.scene.state()]).toEqual(["done", end]);
  page.scene.poke();
  expect(page.scene.state().year).toBe(0);
});

test("the savers River of generations and Many families boot in the lock and paint every family", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["river", "River of generations"],
    ["delta", "Many families"],
  ]);
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(1000);
    const painted = fills(live.log);
    const families = Array.from({ length: value.families }, (_, family) => hue(family));
    expect([saver.id, live.scene.state().year > 0, families.every((one) => painted.has(one))]).toEqual([
      saver.id,
      true,
      true,
    ]);
  }
  expect(HUES.length).toBe(12);
});

test("in the lock run after run brings a new world under the next rule", () => {
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(YEARS * PACE * 3 + 20000, 100);
    const runs = live.told.filter((one) => one.year === 0).map((one) => one.inherit);
    const rules = ["heir", "commons", "none"];
    const at = rules.indexOf(value.inherit);
    expect<(string | string[])[]>([saver.id, runs.slice(0, 3)]).toEqual([
      saver.id,
      [0, 1, 2].map((k) => rules[(at + k) % 3]),
    ]);
  }
});

test("the lock's exit: hold stops the clock, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
  live.scene.hold();
  const year = live.scene.state().year;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().year, landed - from <= 1000, still]).toEqual([year, true, true]);
});

test("a still lock shows a river well into its years at once", () => {
  const live = open(5, SAVERS[1].pick(rng(5)), true, true);
  expect(live.scene.state().year).toBeGreaterThan(YEARS / 2);
});

test("a wide stage carries a legend of the river and top, mobility, made and from nothing; a square one and the lock keep to the river", () => {
  const said = (w: number, h: number, lock = false) => {
    const live = open(3, {}, false, lock, w, h);
    live.run(3000);
    return live.log.filter(([key]) => key === "fillText").map(([, text]) => String(text));
  };
  const wide = said(960, 540);
  expect([
    SIGNS.every((text) => wide.includes(text)),
    ["top", "mobility", "made", "from nothing"].every((word) => wide.some((one) => one.endsWith(word))),
    said(600, 600),
    said(960, 540, true),
  ]).toEqual([true, true, [], []]);
});

test("the background covers the whole canvas", () => {
  const live = open(3, {}, false, false, 960, 540);
  live.run(500);
  expect(live.log.some(([key, ...args]) => key === "fillRect" && Bun.deepEquals(args, [0, 0, 960, 540]))).toBe(true);
});

test("every colour handed to the pen is finite, at the start and after world seeks, in light, dark and the lock", () => {
  const bad: unknown[] = [];
  const check = (value: unknown) => {
    if (typeof value === "string" && /NaN|Infinity|undefined/.test(value)) bad.push(value);
    if (typeof value === "number" && !Number.isFinite(value)) bad.push(value);
  };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).endsWith("Gradient")
        ? (...args: unknown[]) => {
            args.forEach(check);
            return { addColorStop: (at: number, color: string) => [at, color].forEach(check) };
          }
        : (...args: unknown[]) => args.forEach(check),
    set: (_, _key, value) => {
      check(value);
      return true;
    },
  });
  for (const paper of ["#000000", "#ffffff"])
    for (const [value, lock] of [
      [{}, false],
      [{ inherit: "commons", families: 32 }, false],
      [{ inherit: "none", families: 8 }, false],
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
