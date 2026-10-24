import { expect, test } from "bun:test";
import * as life from "@mrlyprod/mrly/life";
import * as math from "@mrlyprod/mrly/math";
import { keymap } from "../../lib/app/page.jsx";
import { defaults, describe } from "../../lib/app/spec.js";
import { rng } from "../../lib/rand.js";
import { card, FAMOUS, make, PAGE, SPEC } from "./scene.js";

const pen = (log) =>
  new Proxy(
    {},
    {
      get:
        (_, key) =>
        (...args) =>
          log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const open = (value = {}, { w = 1028, h = 520 } = {}) => {
  const log = [];
  const told = [];
  const start = { ...defaults(SPEC), seed: 7, ...value };
  const view = {
    rand: rng(7),
    look: () => ({ paper: "#000000", accent: "#008cff" }),
    still: false,
    w,
    h,
    dpr: 1,
    t: 0,
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...start, life, math, onStat: (stat) => told.push(stat) });
  scene.draw();
  return {
    log,
    told,
    scene,
    last: () => told[told.length - 1],
    set: (patch) => scene.set(Object.assign(start, patch)),
  };
};

const lit = (tensor) => tensor.data.reduce((a, b) => a + b, 0);

test("the default diagram is rule 110 from one cell on the padded light cone, drawn whole at once", () => {
  const run = open();
  const cone = life.single_seed(110, 128);
  expect([run.last().rows, run.last().cols, run.last().live, run.last().how]).toEqual([
    129,
    257,
    lit(cone),
    "one live cell, padded by 128 and cropped to 257",
  ]);
  expect(run.log.filter(([verb]) => verb === "rect").length).toBeGreaterThan(0);
});

test("the cone is centred on the live columns whichever way the rule leans", () => {
  const ground = (run) => run.log.find(([verb]) => verb === "fillRect").slice(1);
  const [x, , w] = ground(open({ rule: 110 }));
  expect([x + w / 2, w < 1028 * 0.96]).toEqual([514, true]);
  const [x30, , w30] = ground(open({ rule: 30 }));
  expect([x30 + w30 / 2, w30 > w]).toEqual([514, true]);
});

test("a rule change redraws the whole sheet under the new rule", () => {
  const run = open();
  run.set({ rule: 90 });
  expect(run.last().live).toBe(lit(life.single_seed(90, 128)));
});

test("the generations pick sizes the cone, and wrap runs the same seed on a ring", () => {
  const run = open({ steps: 64 });
  expect([run.last().rows, run.last().cols]).toEqual([65, 129]);
  run.set({ wrap: 1 });
  const row = new Uint8Array(129);
  row[64] = 1;
  expect([run.last().live, run.last().how]).toEqual([
    lit(life.history(row, 110, 64, true)),
    "one live cell on a ring of 129",
  ]);
});

test("a soup start runs the history of a seeded soup the width of the cone", () => {
  const run = open({ from: "noise", steps: 64 });
  expect([run.last().cols, run.last().how]).toEqual([129, "a soup of 129 cells at density 0.35"]);
});

test("a tap on the top row edits the seed and reruns it, and a tap far below does nothing", () => {
  const run = open({ rule: 90, steps: 64 }, { w: 1032, h: 520 });
  const cell = Math.min((1032 * 0.96) / 129, (520 * 0.96) / 65);
  const x = (1032 - cell * 129) / 2;
  const y = (520 - cell * 65) / 2;
  const before = run.last().live;
  run.scene.touch((x + 10.5 * cell) / 1032, 200 / 520, true);
  expect(run.last().live).toBe(before);
  run.scene.touch((x + 10.5 * cell) / 1032, (y + cell / 2) / 520, true);
  expect([run.last().how, run.last().live > before]).toEqual(["a seed row edited by hand", true]);
  run.set({ rule: 30 });
  expect(run.last().how).toBe("a seed row edited by hand");
  run.set({ steps: 128 });
  expect(run.last().how).toBe("one live cell, padded by 128 and cropped to 257");
});

test("the famous rules are distinct bytes of the cube, each with a word", () => {
  expect([
    new Set(FAMOUS.map(([rule]) => rule)).size,
    FAMOUS.every(([rule, word]) => rule >= 0 && rule < 256 && word),
  ]).toEqual([FAMOUS.length, true]);
});

test("the card reads the byte as a design: rule 110 has five corners, rule 90 is affine and draws the gasket", () => {
  const r110 = card(life, 110);
  expect([
    r110.name,
    Array.from(r110.corners),
    r110.popcount,
    r110.lambda,
    r110.degree,
    r110.genus,
    r110.affine,
    r110.onto,
    r110.reversible,
    r110.bs,
    r110.mates.length,
    r110.klass,
    r110.npn,
  ]).toEqual([
    "bang dim 3, code 110",
    [0, 1, 1, 1, 0, 1, 1, 0],
    5,
    0.625,
    3,
    "comp",
    false,
    false,
    false,
    "",
    24,
    [110, 124, 137, 193],
    life.npn_class(110)[0],
  ]);
  const r90 = card(life, 90);
  expect([r90.affine, r90.onto, r90.gasket]).toEqual([true, true, "bang dim 2, code 13"]);
  expect(card(life, 150).bs).toBe("B1/S02");
  expect([card(life, 30).mixed, card(life, 110).mixed]).toEqual([true, false]);
});

test("the scene is still, so the page binds no Space", () => {
  const run = open();
  expect([run.scene.still, keymap(PAGE.keys, [], { still: run.scene.still }).some((row) => row.key === " ")]).toEqual([
    true,
    false,
  ]);
});

test("the Run group is plain spec rows, with Density only for a soup", () => {
  const groups = (from) =>
    describe(SPEC, { ...defaults(SPEC), from }).map((group) => [group.name, group.rows.map((row) => row.key)]);
  expect([groups("one"), groups("noise")]).toEqual([
    [["Run", ["from", "wrap", "steps"]]],
    [["Run", ["from", "density", "wrap", "steps"]]],
  ]);
});

test("the export is named by the rule, the start and the ring", () => {
  const value = { ...defaults(SPEC), seed: 7 };
  expect([
    PAGE.name(value),
    PAGE.name({ ...value, rule: 30, from: "noise", wrap: 1 }),
    PAGE.name({ ...value, from: "design" }),
  ]).toEqual(["wolfram-110", "wolfram-30-soup-7-ring", "wolfram-110-carpet"]);
});
