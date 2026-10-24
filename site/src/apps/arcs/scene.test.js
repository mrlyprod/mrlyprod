import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { defaults, describe as groups } from "../../lib/app/spec.js";
import { rng } from "../../lib/rand.js";
import { curves, fit, PAD, study, thick } from "./engine.js";
import { file, make, SPEC, shades, sketch } from "./scene.js";

const pen = (log) =>
  new Proxy(
    {},
    {
      get:
        (_, key) =>
        (...args) =>
          log.push([key, ...args.map((arg) => (arg && typeof arg === "object" ? "layer" : arg))]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const sheets = [];

let owner = sheets;

const arcs = (log) => log.filter(([name]) => name === "arc").length;

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.log = [];
    sheets.push(this);
    if (owner !== sheets) owner.push(this);
  }
  getContext() {
    return pen(this.log);
  }
};

const LOOK = { paper: "#000000", accent: "#ff8f2c" };

const INKS = shades(null, LOOK, "blue");

const START = { ...defaults(SPEC), seed: 1, base: 3, code: "495", level: 2 };

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const mine = [];
  const told = [];
  let contexts = 0;
  owner = mine;
  const view = { rand: rng(1), look: () => LOOK, still, w, h, dpr: 2, t: 0, wake: () => (view.still = false) };
  const canvas = {
    getContext: () => {
      contexts++;
      return pen(log);
    },
  };
  const scene = make(canvas, view, { ...START, math, onTell: (facts) => told.push(facts), ...value });
  const at = (t) => {
    owner = mine;
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const laid = () => mine.reduce((sum, sheet) => sum + arcs(sheet.log), 0);
  return { view, scene, log, at, mine, laid, told, contexts: () => contexts };
};

const PLAN = study(math, START);
const ARCS = 2 * PLAN.side * PLAN.side;
const LOOPED = Array.from(PLAN.lit).reduce((sum, k) => sum + PLAN.bounds[k + 1] - PLAN.bounds[k], 0);
const SPAN = PLAN.clock.span;

test("the paints are two hues, the loops in the tint and the strands in their own, the cells in the line and a faint loop for a trace", () => {
  expect(INKS).toEqual({ loop: "#ff8f2c", strand: "#008cff", dim: "rgb(115, 64, 20)", fill: "#1f1f20" });
  expect(shades(null, { paper: "#ffffff", accent: "#ff8f2c" }, "green")).toMatchObject({
    strand: "#32cc58",
    fill: "#dddddf",
  });
});

test("the scene is still: make draws nothing and tells its facts, the first draw lays every arc once with the loops lit, and a draw after is a no-op", () => {
  const still = open({});
  expect(still.log).toEqual([]);
  expect([still.scene.every, still.scene.still]).toEqual([undefined, true]);
  expect(still.told).toEqual([PLAN.facts]);
  expect(arcs(still.at(0))).toBe(0);
  expect(still.laid()).toBe(ARCS);
  const [layer] = still.mine;
  expect(layer.log).toContainEqual(["strokeStyle", INKS.strand]);
  expect(layer.log).toContainEqual(["strokeStyle", INKS.loop]);
  expect(layer.log).toContainEqual(["fillStyle", INKS.fill]);
  expect(layer.log).not.toContainEqual(["strokeStyle", INKS.dim]);
  expect(still.at(5000)).toEqual([]);
  expect(still.laid()).toBe(ARCS);
});

test("the picture is laid flipped so row 0 sits at the bottom, on the square fitted to the stage", () => {
  const still = open({});
  still.at(0);
  const box = fit(PLAN.side, 800, 600, PAD * 2);
  const log = still.mine[0].log;
  expect(log[0]).toEqual(["setTransform", 1, 0, 0, -1, 0, 600]);
  const centres = log.filter(([name]) => name === "arc").map(([, x, y, r]) => [x, y, r]);
  expect(
    centres.every(
      ([x, y, r]) =>
        r === box.px / 2 &&
        x >= box.x &&
        x <= box.x + PLAN.side * box.px &&
        y >= box.y &&
        y <= box.y + PLAN.side * box.px,
    ),
  ).toBe(true);
  expect(log).toContainEqual(["lineWidth", thick(0, box.px, 2)]);
});

test("a resize or a theme lays the picture again on the next draw", () => {
  const still = open({});
  still.at(0);
  const was = sheets.length;
  still.scene.size();
  expect(arcs(still.mine.at(-1).log)).toBe(ARCS);
  still.at(100);
  expect(sheets.length).toBe(was + 1);
  still.scene.theme();
  still.at(200);
  expect(sheets.length).toBe(was + 2);
  expect(still.at(300)).toEqual([]);
});

test("a set keeps the canvas and its context: a look knob lays again with the same plan, a design knob plans again and tells, the seed alone does nothing", () => {
  const live = open({});
  live.at(0);
  const was = sheets.length;
  live.scene.set({ ...START, seed: 9 });
  expect(live.at(100)).toEqual([]);
  live.scene.set({ ...START, width: 0.3 });
  live.at(200);
  expect(sheets.length).toBe(was + 1);
  expect(live.told).toHaveLength(1);
  live.scene.set({ ...START, width: 0.3, level: 3 });
  live.at(300);
  expect(live.told.at(-1)).toMatchObject({ level: 3, side: 27 });
  expect(arcs(live.mine.at(-1).log)).toBe(2 * 27 * 27);
  expect(live.contexts()).toBe(1);
});

test("a trace dims the loops, lights them in turn by t, then ends lit and waits", () => {
  const live = open({});
  live.at(1000);
  expect(live.scene.trace()).toBe(true);
  const [, back, layer] = live.mine;
  expect(back.log).toContainEqual(["strokeStyle", INKS.dim]);
  expect(layer.log).toContainEqual(["drawImage", "layer", 0, 0]);
  expect(arcs(live.at(1000 + SPAN / 3))).toBeGreaterThan(0);
  const done = live.at(1000 + SPAN);
  expect(arcs(done)).toBe(0);
  expect(live.mine.at(-1).log).toContainEqual(["strokeStyle", INKS.loop]);
  expect(live.at(1000 + 2 * SPAN)).toEqual([]);
});

test("a trace at a t is the same reached in one jump or by steps", () => {
  const stepped = open({});
  stepped.scene.trace();
  for (let t = 0; t <= 1200; t += 100) stepped.at(t);
  const log = [...stepped.log];
  const jumped = open({});
  jumped.scene.trace();
  const once = [...jumped.at(1200)];
  expect(jumped.laid()).toBe(stepped.laid());
  expect(once).toEqual(log);
  expect(stepped.laid()).toBeGreaterThan(ARCS);
  expect(stepped.laid()).toBeLessThan(ARCS + LOOPED);
});

test("every loop in progress is traced although a packed schedule starts the loops out of order", () => {
  const packed = study(math, { ...START, level: 4 });
  const { start, end, dur, span } = packed.clock;
  expect(Array.from(start).some((s, i) => i && s < start[i - 1])).toBe(true);
  const owed = (t) =>
    Array.from(packed.lit.keys()).reduce((sum, k) => {
      if (start[k] > t || end[k] <= t) return sum;
      const n = packed.bounds[packed.lit[k] + 1] - packed.bounds[packed.lit[k]];
      const f = (t - start[k]) / dur[k];
      const whole = Math.floor(f * n);
      return sum + whole + (f * n - whole > 0 ? 1 : 0);
    }, 0);
  const live = open({ level: 4 });
  live.scene.trace();
  for (const t of [span / 4, span / 2, (3 * span) / 4]) {
    expect(owed(t)).toBeGreaterThan(0);
    expect(arcs(live.at(t))).toBe(owed(t));
  }
});

test("under reduced motion a trace wakes the scene, and a design knob ends a trace lit", () => {
  const still = open({}, true);
  still.at(0);
  still.scene.trace();
  expect(still.view.still).toBe(false);
  still.scene.set({ ...START, code: "186" });
  still.at(SPAN / 2);
  expect(still.mine.at(-1).log).not.toContainEqual(["strokeStyle", INKS.dim]);
  expect(still.at(SPAN)).toEqual([]);
});

test("the look keeps the loops or the strands alone, strands alone cannot trace, and cells off shade nothing", () => {
  const loops = open({ look: "loops" });
  loops.at(0);
  expect(loops.laid()).toBe(LOOPED);
  const strands = open({ look: "strands" });
  strands.at(0);
  expect(strands.laid()).toBe(ARCS - LOOPED);
  expect(strands.scene.trace()).toBe(false);
  const bare = open({ cells: 0 });
  bare.at(0);
  expect(bare.mine.some(({ log }) => log.some(([name]) => name === "rect"))).toBe(false);
});

test("with no design key the scene draws the base 2 carpet at level 5, where the law meets fifty loops", () => {
  const bare = open({ base: undefined, code: undefined, level: undefined });
  expect(bare.scene.facts).toMatchObject({
    code: "7",
    name: "carpet",
    base: 2,
    level: 5,
    side: 32,
    loops: 50,
    law: { loops: 50 },
  });
});

test("a bad code clears the stage, tells the fault and has no svg", () => {
  const bad = open({ base: 2, code: "99" });
  const log = bad.at(0);
  expect(log.filter(([name]) => name === "clearRect")).toHaveLength(1);
  expect(log.filter(([name]) => name === "drawImage")).toHaveLength(0);
  expect(bad.told[0].fault).toContain("out of range");
  expect(bad.scene.svg()).toBe(null);
  expect(bad.scene.trace()).toBe(false);
});

test("the svg is written in the paints on screen", () => {
  const live = open({});
  live.at(100);
  const text = live.scene.svg();
  expect(text).toContain(`stroke="${INKS.strand}"`);
  expect(text).toContain(`stroke="${INKS.loop}"`);
  expect(text).toContain(`fill="${INKS.fill}"`);
  expect(text.match(/A5 5 0 0 /g)).toHaveLength(ARCS);
});

test("an export is named by the design, the base and the level", () => {
  expect([
    file({ base: 2, code: "", level: 5 }, { math }),
    file({ base: 3, code: "carpet", level: 2 }, { math }),
    file({ base: 2, code: "9 x", level: 1 }),
  ]).toEqual(["arcs-7-base2-level5", "arcs-495-base3-level2", "arcs-9-x-base2-level1"]);
});

test("the design keys stay out of the knobs, which open on the arcs", () => {
  const shown = groups(SPEC, START).map(({ name, rows }) => [name, rows.map((row) => row.key)]);
  expect(shown).toEqual([
    ["Arcs", ["level", "cells", "look", "width"]],
    ["Colour", ["tint", "strand"]],
  ]);
});

test("a thumb draws the shaded cells, the strands and the loops in their inks, and leaves out what the look hides", () => {
  const plan = curves(math, "495", 3, 2);
  const looped = plan.closed.reduce((sum, closed, k) => sum + (closed ? plan.bounds[k + 1] - plan.bounds[k] : 0), 0);
  const drawn = (over) => {
    const log = [];
    sketch(pen(log), 112, 112, 2, plan, { ...START, ...over }, INKS);
    return [
      log.filter(([name]) => name === "rect").length > 0,
      log.filter(([name]) => name === "strokeStyle").map(([, v]) => v),
      arcs(log),
    ];
  };
  expect(drawn({})).toEqual([true, [INKS.strand, INKS.loop], plan.order.length]);
  expect(drawn({ cells: 0, look: "loops" })).toEqual([false, [INKS.loop], looped]);
});
