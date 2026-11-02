import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import * as num from "@mrlyprod/mrly/num";
import { keymap, labels } from "../../lib/app/page.tsx";
import type { View } from "../../lib/app/run.ts";
import type { Raw, Value } from "../../lib/app/spec.ts";
import { defaults, describe, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import { boot, cell, DRAW, deal, FRAME, HEXES, LEVELS, origin, spot, step, study } from "./engine.ts";
import type { Facts } from "./scene.ts";
import { file, hexCentre, make, PAGE, SAVERS, SPEC, sketch, tidied } from "./scene.ts";

type Call = [string | symbol, ...unknown[]];

type Sheet = { width: number; height: number; log: Call[] };

const units = { math, num };

const pen = (log: Call[]) =>
  new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) => {
      if (key === "createImageData")
        return (w: number, h: number) => ({ width: w, height: h, data: new Uint8ClampedArray(w * h * 4) });
      if (key === "canvas") return {};
      return (...args: unknown[]) =>
        log.push([
          key,
          ...args.map((arg) => (arg && typeof arg === "object" && !ArrayBuffer.isView(arg) ? "object" : arg)),
        ]);
    },
    set: (_, key, v: unknown) => {
      log.push([key, v]);
      return true;
    },
  });

const sheets: Sheet[] = [];

Object.assign(globalThis, {
  OffscreenCanvas: class {
    width: number;
    height: number;
    log: Call[] = [];
    constructor(w: number, h: number) {
      this.width = w;
      this.height = h;
      sheets.push(this);
    }
    getContext() {
      return pen(this.log);
    }
  },
});

const BASE = { ...defaults(SPEC), seed: 1, base: 2, code: "7", speed: 100, level: 2 };

const open = (value: Raw, w = 800, h = 600, still = false) => {
  const log: Call[] = [];
  const told: Facts[] = [];
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#000000", accent: "#6768fa", ink: "#ffffff" }),
    still,
    lock: false,
    w,
    h,
    dpr: 1,
    t: 0,
  };
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen(log) }), view, {
    ...BASE,
    ...units,
    ...value,
    onFacts: (facts) => told.push(facts),
  });
  const draw = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { view, scene, log, told, draw };
};

const count = (log: Call[], name: string) => log.filter(([key]) => key === name).length;

test("the scene opens playing on the gasket and rains the speed in points a frame by the clock, lighting the newest", () => {
  const was = sheets.length;
  const one = open({});
  expect(one.draw().some(([key]) => key === "arc")).toBe(false);
  expect(one.scene.facts().n).toBe(0);
  one.view.t = 10 * FRAME;
  const log = one.draw();
  expect(one.scene.facts().n).toBe(1000);
  expect([count(log, "drawImage"), count(log, "arc")]).toEqual([1, 48]);
  const cloud = sheets[was];
  expect([cloud.width, cloud.height, count(cloud.log, "fillRect")]).toEqual([1024, 1024, 1000]);
  expect(one.scene.playing()).toBe(true);
});

test("a pause stops the rain and a resume carries on from the same count", () => {
  const one = open({});
  one.view.t = FRAME;
  one.draw();
  expect(one.scene.facts().n).toBe(100);
  one.scene.toggle();
  one.view.t = 50 * FRAME;
  one.draw();
  expect([one.scene.facts().n, one.scene.playing()]).toEqual([100, false]);
  one.scene.toggle();
  one.view.t = 51 * FRAME;
  one.draw();
  expect(one.scene.facts().n).toBe(200);
});

test("the meter reads the law: every point's cell is its last k draws, the nine cells fill, and the distance falls", () => {
  const one = open({});
  one.view.t = 30 * FRAME;
  one.draw();
  const facts = one.scene.facts();
  expect([
    facts.n,
    facts.k,
    facts.counted,
    facts.checked,
    facts.agree,
    facts.painted,
    facts.total,
    facts.outside,
  ]).toEqual([3000, 2, 2999, 2999, 2999, 9, 9, 0]);
  expect(facts.distance).toBeLessThan(0.06);
  expect([facts.mode, facts.law, facts.stuck, facts.lost]).toEqual(["random", "uniform", false, false]);
  expect(one.told.at(-1)).toEqual(facts);
});

test("a new seed restarts, a new level replays the same orbit at the new level, and the grid toggle lays the level cells", () => {
  const one = open({});
  one.view.t = 5 * FRAME;
  one.draw();
  expect(one.scene.facts().n).toBe(500);
  one.scene.set({ ...BASE, ...units, seed: 2 });
  expect(one.scene.facts().n).toBe(0);
  one.view.t = 10 * FRAME;
  one.draw();
  expect(one.scene.facts().n).toBe(500);
  one.scene.set({ ...BASE, ...units, seed: 2, level: 3 });
  const deeper = one.scene.facts();
  expect([deeper.n, deeper.k, deeper.total, deeper.counted]).toEqual([500, 3, 27, 498]);
  expect(deeper.painted).toBeGreaterThan(20);
  const was = sheets.length;
  one.scene.set({ ...BASE, ...units, seed: 2, level: 3, grid: 1 });
  const sheet = sheets[was];
  expect([sheets.length, sheet.width, count(sheet.log, "putImageData")]).toEqual([was + 1, 8, 1]);
  const log = one.draw();
  expect([count(log, "drawImage"), count(log, "moveTo"), count(log, "stroke")]).toEqual([2, 18, 1]);
  const fresh = open({ grid: 1 });
  const laid = sheets.at(-1)!;
  expect([laid.width, count(laid.log, "putImageData"), count(laid.log, "fillRect")]).toEqual([4, 1, 0]);
  fresh.view.t = FRAME;
  fresh.draw();
  expect(count(laid.log, "fillRect")).toBe(fresh.scene.facts().painted);
});

test("the de Bruijn draws paint every level-k cell exactly once per lap, and the histogram sits on the law at each lap's end", () => {
  const one = open({ mode: "bruijn", speed: 10 });
  one.view.t = FRAME;
  one.draw();
  const facts = one.scene.facts();
  expect([facts.n, facts.mode, facts.lap, facts.laps, facts.perfect, facts.painted, facts.distance, facts.law]).toEqual(
    [10, "bruijn", 9, 1, true, 9, 0, "uniform"],
  );
  one.view.t = 3 * FRAME;
  one.draw();
  expect([one.scene.facts().laps, one.scene.facts().perfect]).toEqual([3, true]);
  expect(one.scene.sheet().split("\n")[1]).toBe("0,0,4,0.111111111");
});

test("a tap inside the square starts the game again from the tap, and a tap outside does nothing", () => {
  const one = open({});
  one.view.t = 5 * FRAME;
  one.draw();
  expect(one.scene.facts().n).toBe(500);
  one.scene.tap(2, 2);
  expect(one.scene.facts().n).toBe(500);
  one.scene.tap(400, 300);
  expect(one.scene.facts().n).toBe(0);
});

test("a still page bursts a whole cloud on the first draw and then rests", () => {
  const one = open({}, 800, 600, true);
  one.draw();
  expect(one.scene.facts().n).toBe(30000);
  one.draw();
  expect(one.scene.facts().n).toBe(30000);
});

test("a hexagonal base rains in its box and its grid is one square a word", () => {
  const was = sheets.length;
  const one = open({ view: "hex", grid: 1, level: 2 });
  expect([sheets.length - was, sheets[was].width, count(sheets[was].log, "fillRect")]).toEqual([1, 1024, 9]);
  one.view.t = 2 * FRAME;
  one.draw();
  const facts = one.scene.facts();
  expect([sheets.length - was, facts.n, facts.checked, facts.total]).toEqual([2, 200, 0, 9]);
  expect(facts.painted).toBeGreaterThan(5);
  expect(count(sheets[was].log, "fillRect")).toBe(9 + facts.painted);
});

test("a hex cell's square sits under the points of that cell, once the start has faded", () => {
  for (const [hex] of HEXES) {
    const source = study(units, tidied({ ...BASE, view: "hex", hex }, units));
    const { reach, norm, box } = source.hex!;
    const span = Math.hypot(box[2] - box[0], box[3] - box[1]);
    const first = Math.floor(rng(4)() * 2 ** 32);
    let state = boot(deal(source, 3, false, null, origin(source, rng(4)), first));
    for (let i = 0; i < 500; i++) {
      state = step(state, DRAW);
      if (state.n < 3) continue;
      const [x, y] = spot(state);
      const [cx, cy] = hexCentre(source, 3, cell(state));
      const off = Math.max(Math.abs(cx - x), Math.abs(cy - y));
      const half = reach * norm ** (-3 / 2) + span * norm ** (-state.n / 2) + 1e-9;
      expect([hex, state.n, off <= half]).toEqual([hex, state.n, true]);
    }
  }
});

test("a theme change recolours the cloud in place and lays the grid again", () => {
  const was = sheets.length;
  const one = open({ grid: 1 });
  one.view.t = FRAME;
  one.draw();
  const sheet = sheets[was];
  const cloud = sheets[was + 1];
  const before = count(sheet.log, "putImageData");
  one.scene.theme();
  expect(cloud.log.some(([key, v]) => key === "globalCompositeOperation" && v === "source-in")).toBe(true);
  expect(count(sheet.log, "putImageData")).toBe(before + 1);
});

test("the scene wants both units", () => {
  expect(() =>
    make(
      Object.assign({} as HTMLCanvasElement, { getContext: () => pen([]) }),
      Object.assign({} as View, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }),
      { ...defaults(SPEC), math },
    ),
  ).toThrow("chaos");
});

test("the level is capped by the source, the draws row hides on the rule view and the weights off the random draws", () => {
  const level = (over: Raw) => tidy(SPEC, { ...BASE, level: 9, ...over }, units).level;
  expect([level({}), level({ side: 5 }), level({ base: 3, code: "495" }), level({ view: "word", side: 3 })]).toEqual([
    6, 4, 6, 5,
  ]);
  expect(tidy(SPEC, { ...BASE, level: 9 }).level).toBe(LEVELS);
  const keys = (value: Value) =>
    describe(SPEC, tidied({ ...BASE, ...value }, units) as unknown as Value).flatMap(({ rows }) =>
      rows.map((row) => row.key),
    );
  expect(keys({})).toEqual(["view", "side", "mode", "weights", "speed", "level", "grid"]);
  expect(keys({ view: "rule" })).toEqual(["view", "game", "speed", "level", "grid"]);
  expect(keys({ view: "rule", game: "typed" })).toContain("rule");
  expect(keys({ mode: "bruijn" })).not.toContain("weights");
  expect(keys({ mode: "restricted" })).toEqual(["view", "side", "game", "mode", "speed", "level", "grid"]);
  expect(keys({ view: "word", word: "typed" })).toEqual([
    "view",
    "side",
    "word",
    "letters",
    "mode",
    "weights",
    "speed",
    "level",
    "grid",
  ]);
  expect(keys({ view: "slice" })).toContain("cut");
  expect(keys({ view: "hex" })).toEqual(["view", "hex", "mode", "weights", "speed", "level", "grid"]);
  expect(tidy(SPEC, { ...BASE, view: "hex", mode: "restricted" }, units).mode).toBe("random");
});

test("the design chip shows on the design and slice views, and the keys move the level, the speed and the grid", () => {
  expect(["design", "slice", "word", "hex", "rule"].map((view) => PAGE.design.when({ view }))).toEqual([
    true,
    true,
    false,
    false,
    false,
  ]);
  const map = labels(keymap(PAGE.keys), {}, null);
  expect(map.map((row) => [row.key, row.act].flat().join(":"))).toEqual([
    " :pause",
    "ArrowUp:up",
    "ArrowDown:down",
    "ArrowLeft:ArrowRight:speed",
    "g:grid",
    "r:reroll",
    "z:zen",
    "?:keys",
  ]);
});

test("an export is named by the source, the draws and the level, and a saver pick stays what it asked", () => {
  expect([
    file({ ...BASE }),
    file({ ...BASE, side: 3, mode: "bruijn", level: 4 }),
    file({ ...BASE, view: "slice", cut: "2/3" }),
    file({ ...BASE, view: "rule", game: "pair" }),
    file({ ...BASE, view: "hex", hex: "koch" }),
  ]).toEqual([
    "chaos-design-2-7-2-random-2",
    "chaos-design-2-7-3-bruijn-4",
    "chaos-slice-2-7-2-3-random-2",
    "chaos-rule-pair-restricted-2",
    "chaos-hex-koch-random-2",
  ]);
  for (const saver of SAVERS)
    for (const seed of [1, 2, 3]) {
      const picked = saver.pick(rng(seed));
      const kept = tidy(SPEC, { ...BASE, ...picked }, units);
      for (const key of Object.keys(picked))
        if (key in kept) expect([saver.id, key, kept[key]]).toEqual([saver.id, key, picked[key]!]);
    }
});

test("a thumb runs a short game and drops a dot a point inside a small edge", () => {
  const log: Call[] = [];
  sketch(pen(log), 112, 112, 2, units, { ...BASE, view: "mosaic" });
  const dots = log.filter(([key]) => key === "fillRect");
  expect(dots).toHaveLength(1500);
  expect(
    dots.every(([, x, y]) => (x as number) >= 7 && (y as number) >= 7 && (x as number) <= 105 && (y as number) <= 105),
  ).toBe(true);
});
