import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { boot, DAYS, deal, nearest, run } from "./engine.ts";
import type { Told } from "./scene.ts";
import { cells, coins, FIVE, make, SAVERS, SIGNS, SPEC } from "./scene.ts";

const FRAME = 16;
const PACE = 220;
const ACCENT = "#008cff";

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
    look: () => ({ paper: "#000000", accent: ACCENT, ink: "#ffffff" }),
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
  const own = run(boot({ currencies: 5, fee: 3, traders: 48, length: DAYS, seed: 7 }), Math.floor(6000 / PACE));
  expect(at(7)).toBe(JSON.stringify(own));
  expect(at(7)).not.toBe(at(8));
});

test("the scene boots the state deal deals, for several seeds, coin counts, fees and crowds", () => {
  for (const seed of [0, 3, 41])
    for (const currencies of [1, 3, 5])
      for (const [fee, traders] of [
        [0, 16],
        [10, 96],
      ]) {
        const value = { ...defaults(SPEC), seed, currencies, fee, traders };
        expect(open(seed, value).scene.state()).toEqual(deal({ currencies, fee, traders }, { seed }));
      }
});

test("the map's cells split the stage among the towns: each holds its town and no corner lies nearer another", () => {
  const state = deal({ currencies: 5, fee: 3, traders: 48 }, { seed: 4 });
  const polys = cells(state, -300, -100, 1400, 1100);
  polys.forEach((poly, town) => {
    const inside = poly.every((p, i) => {
      const q = poly[(i + 1) % poly.length];
      return (q.x - p.x) * (state.ty[town] - p.y) - (q.y - p.y) * (state.tx[town] - p.x) >= 0;
    });
    const fair = poly.every((p) => {
      const d = (u: number) => Math.hypot(p.x - state.tx[u], p.y - state.ty[u]);
      return d(town) <= d(nearest(state, p.x, p.y)) + 1e-6;
    });
    expect([town, poly.length >= 3, inside, fair]).toEqual([town, true, true, true]);
  });
});

test("the coins take the site tint first and drop the hue nearest it", () => {
  expect([coins(FIVE[0]), coins("#ff0000")[0], coins(FIVE[2]).includes(FIVE[2]), coins("#123456").length]).toEqual([
    [FIVE[0], ...FIVE.slice(1, 5)],
    "#ff0000",
    true,
    5,
  ]);
});

test("turning the coins starts the same towns again, and a tint keeps the run going", () => {
  const live = open(7);
  const first = live.scene.state();
  live.run(3000);
  const value = { ...defaults(SPEC), seed: 7 };
  live.scene.set({ ...value, currencies: 2 });
  const fresh = live.scene.state();
  expect([fresh.currencies, fresh.day, fresh.tx]).toEqual([2, 0, first.tx]);
  live.run(3000);
  const going = live.scene.state().day;
  live.scene.set({ ...value, currencies: 2, tint: "red" });
  expect(live.scene.state().day).toBe(going);
});

test("Enter steps one day and pauses, pause holds the clock, and play lets it run", () => {
  const live = open(7);
  live.scene.poke();
  expect([live.scene.state().day, live.scene.playing()]).toEqual([1, false]);
  live.run(3000);
  expect(live.scene.state().day).toBe(1);
  live.scene.toggle();
  live.run(3000);
  expect(live.scene.state().day).toBeGreaterThan(5);
});

test("under reduced motion a seek of the world clock still runs the days, frame by frame", () => {
  const live = open(3, {}, true);
  const seen = [0, 2000, 4000].map((until) => {
    while (live.view.t < until) {
      live.view.t = Math.min(until, live.view.t + 1000 / 60);
      live.scene.draw();
    }
    return live.scene.state().day;
  });
  expect(seen[0]).toBe(0);
  expect(seen[1]).toBeGreaterThan(seen[0]);
  expect(seen[2]).toBeGreaterThan(seen[1]);
});

test("a run ends and holds on the page, and Enter runs it again", () => {
  const page = open(5);
  page.run(DAYS * PACE + 2000, 50);
  const end = page.scene.state();
  page.run(10000, 50);
  expect([end.over, page.scene.state()]).toEqual(["done", end]);
  page.scene.poke();
  expect(page.scene.state().day).toBe(0);
});

test("the savers Five coins and Busy borders boot in the lock and paint every coin in play", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["market", "Five coins"],
    ["borders", "Busy borders"],
  ]);
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(1000);
    const painted = fills(live.log);
    const hues = coins(ACCENT).slice(0, value.currencies);
    expect([saver.id, live.scene.state().day > 0, hues.every((one) => painted.has(one))]).toEqual([
      saver.id,
      true,
      true,
    ]);
  }
});

test("in the lock run after run brings the same knobs under the next count of coins", () => {
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(DAYS * PACE * 3 + 20000, 100);
    const runs = live.told.filter((one) => one.day === 0).map((one) => one.currencies);
    expect([saver.id, runs.slice(0, 3)]).toEqual([
      saver.id,
      [0, 1, 2].map((k) => ((value.currencies - 1 + k) % 5) + 1),
    ]);
  }
});

test("the lock's exit: hold stops the clock, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
  live.scene.hold();
  const day = live.scene.state().day;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().day, landed - from <= 1000, still]).toEqual([day, true, true]);
});

test("a still lock shows a map well into its days at once", () => {
  const live = open(5, SAVERS[1].pick(rng(5)), true, true);
  expect(live.scene.state().day).toBeGreaterThan(DAYS / 2);
});

test("a wide stage carries a legend of the map and traded, failed, lost and stuck; a square one and the lock keep to the map", () => {
  const said = (w: number, h: number, lock = false) => {
    const live = open(3, {}, false, lock, w, h);
    live.run(3000);
    return live.log.filter(([key]) => key === "fillText").map(([, text]) => String(text));
  };
  const wide = said(960, 540);
  expect([
    SIGNS.every((text) => wide.includes(text)),
    ["traded", "failed", "lost", "stuck"].every((word) => wide.some((one) => one.endsWith(word))),
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
      [{ currencies: 1, fee: 0, traders: 96 }, false],
      [{ currencies: 2, fee: 10, traders: 16 }, false],
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
