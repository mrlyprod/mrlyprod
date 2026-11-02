import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { View } from "../../net/sdk/run.ts";
import type { Raw } from "../../net/sdk/spec.ts";
import { defaults } from "../../net/sdk/spec.ts";
import { boot, CHOICES, deal, HOURS, run } from "./engine.ts";
import type { Told } from "./scene.ts";
import { make, SAVERS, SIGNS, SPEC, TITLES } from "./scene.ts";

const FRAME = 16;
const PACE = 200;
const ACCENT = "#008cff";

const pen = (log: unknown[][]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get:
      (_, key) =>
      (...args: unknown[]) => {
        log.push([key, ...args]);
        return { width: 10 };
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

test("one url runs one field: the engine's own run of the url seed, and another seed another", () => {
  const at = (seed: number) => {
    const live = open(seed);
    live.run(6000);
    return JSON.stringify(live.scene.state());
  };
  const own = run(boot({ choose: "first", bots: 24, length: HOURS, seed: 7 }), Math.floor(6000 / PACE));
  expect(at(7)).toBe(JSON.stringify(own));
  expect(at(7)).not.toBe(at(8));
});

test("the scene boots the state deal deals, for several seeds, rules and crowds", () => {
  for (const seed of [0, 3, 41])
    for (const choose of CHOICES)
      for (const bots of [8, 56]) {
        const value = { ...defaults(SPEC), seed, choose, bots };
        expect(open(seed, value).scene.state()).toEqual(deal({ choose, bots }, { seed }));
      }
});

test("turning the rule starts the same field again, and a tint keeps the run going", () => {
  const live = open(5);
  live.run(3000);
  const hour = live.scene.state().hour;
  live.scene.set({ ...defaults(SPEC), seed: 5, tint: "green" });
  const kept = live.scene.state().hour;
  live.scene.set({ ...defaults(SPEC), seed: 5, choose: "most" });
  expect([hour > 0, kept, live.scene.state()]).toEqual([true, hour, deal({ choose: "most", bots: 24 }, { seed: 5 })]);
});

test("Enter steps one hour and pauses, and a run that ended runs again", () => {
  const live = open(5);
  live.scene.poke();
  const one = live.scene.state().hour;
  live.run(2000);
  const paused = live.scene.state().hour;
  live.scene.toggle();
  live.run(HOURS * PACE + 2000, 50);
  const end = live.scene.state();
  live.run(10000, 50);
  const kept = live.scene.state();
  live.scene.poke();
  expect([one, paused, end.over, kept, live.scene.state().hour]).toEqual([1, 1, "done", end, 0]);
});

test("the savers Wishing board and A crowd of wishes boot in the lock and paint their bots in the site tint", () => {
  expect(SAVERS.map((one) => [one.id, one.title])).toEqual([
    ["board", "Wishing board"],
    ["crowd", "A crowd of wishes"],
  ]);
  for (const saver of SAVERS) {
    const live = open(5, saver.pick(rng(5)), false, true);
    live.run(1000);
    expect([saver.id, live.scene.state().hour > 0, fills(live.log).has(ACCENT)]).toEqual([saver.id, true, true]);
  }
});

test("in the lock run after run brings the next rule after a break", () => {
  for (const saver of SAVERS) {
    const value = saver.pick(rng(5));
    const live = open(5, value, false, true);
    live.run(HOURS * PACE * 3 + 20000, 100);
    const runs = live.told.filter((one) => one.hour === 0).map((one) => one.choose);
    const from = CHOICES.indexOf(value.choose);
    expect([saver.id, runs.slice(0, 3)]).toEqual([saver.id, [0, 1, 2].map((k) => CHOICES[(from + k) % 3])]);
  }
});

test("the lock's exit: hold stops the clock, windDown lands within a second, and a still lock lands at once", () => {
  const value = SAVERS[0].pick(rng(5));
  const live = open(5, value, false, true);
  live.run(4000);
  live.scene.hold();
  const hour = live.scene.state().hour;
  live.run(5000);
  const from = live.view.t;
  let landed = Infinity;
  live.scene.windDown(() => (landed = live.view.t));
  live.run(1500);
  let still = false;
  open(5, value, true, true).scene.windDown(() => (still = true));
  expect([live.scene.state().hour, landed - from <= 1000, still]).toEqual([hour, true, true]);
});

test("a still lock shows a field well into its hours, wishes on the board, at once", () => {
  const live = open(5, SAVERS[1].pick(rng(5)), true, true);
  expect([live.scene.state().hour > HOURS / 3, live.scene.state().end.includes(0)]).toEqual([true, true]);
});

test("a wide stage carries a legend of the rule, signs and readouts; a square one and the lock keep to the field", () => {
  const said = (w: number, h: number, lock = false) => {
    const live = open(3, {}, false, lock, w, h);
    live.run(3000);
    return live.log.filter(([key]) => key === "fillText").map(([, text]) => String(text));
  };
  const wide = said(960, 540);
  expect([
    SIGNS.every((text) => wide.includes(text)),
    wide.includes(TITLES.first),
    ["granted", "waiting", "nobody can grant", "let go"].every((word) => wide.some((one) => one.endsWith(word))),
    said(600, 600),
    said(960, 540, true),
  ]).toEqual([true, true, true, [], []]);
});

test("a legend line wider than its column shrinks its type to fit", () => {
  let size = 0;
  const over: string[] = [];
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) => (text: string, x: number) => {
      const width = String(text).length * size * 0.6;
      if (key === "fillText" && x + width > 240.5) over.push(text);
      return { width };
    },
    set: (_, key, value) => {
      if (key === "font") size = Number(String(value).match(/([\d.]+)px/)?.[1]);
      return true;
    },
  });
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: ACCENT, ink: "#ffffff" }),
    still: false,
    lock: false,
    w: 960,
    h: 540,
    dpr: 1,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => ctx }), view, {
    ...defaults(SPEC),
    seed: 3,
  });
  for (let t = 0; t < 3000; t += 100) {
    view.t = t;
    scene.draw();
  }
  expect(over).toEqual([]);
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
      [{ choose: "near", bots: 56 }, false],
      [{ choose: "most", bots: 8 }, false],
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
    const bent = live.log.filter(([key, , , ...r]) => key === "arc" && Number(r[0]) < 0);
    expect([w, h, bent]).toEqual([w, h, []]);
  }
});
