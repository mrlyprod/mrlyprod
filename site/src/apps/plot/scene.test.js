import { expect, test } from "bun:test";
import * as num from "@mrlyprod/mrly/num";
import { pen, said } from "../../../test/pen.js";
import { defaults, describe, tidy } from "../../lib/app/spec.js";
import { rng } from "../../lib/rand.js";
import { rule } from "./engine.js";
import { filed, lines, make, PAGE, SPEC } from "./scene.js";

const ROW = {
  key: "2-2-7-surface-level",
  dim: 2,
  base: 2,
  code: "7",
  label: "carpet 7",
  measure: "surface",
  axis: "level",
  cells: [1, 1, 1, 1, 0, 1, 1, 1, 1],
  hit: null,
  terms: ["16", "80", "496", "3536", "26992", "212048", "1684720", "13442768"],
};

const MIX = {
  key: "mix",
  title: "differences of carpet 7 surface",
  start: 0,
  say: "the first sequence, index from 0",
  hit: null,
  terms: ["64", "416", "3040", "23456", "185056", "1472672", "11758048"],
};

const open = (panels, value = {}) => {
  const log = [];
  const live = { current: { panels, version: 1 } };
  const view = {
    rand: rng(1),
    look: () => ({ paper: "#ffffff", accent: "#008cff" }),
    still: true,
    w: 900,
    h: 600,
    dpr: 1,
    t: 0,
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), ...value, live });
  const frame = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { scene, frame };
};

test("the rule spells two lines under the chart: the recurrence, then the polynomial and the growth", () => {
  expect(lines(rule(num, ROW.terms))).toEqual([
    "a(n) = 11 a(n-1) - 24 a(n-2)",
    "x^2 - 11x + 24 · growth 8.000000, 0.903090 decades a term · read from 8 terms",
  ]);
  expect(lines(rule(num, ["1", "2", "4", "7", "12", "19", "30"]))).toEqual([
    "no linear rule fits these terms",
    "read from 7 terms",
  ]);
  const one = said(open([{ row: ROW, rule: rule(num, ROW.terms), note: "" }]).frame());
  expect(one).toContain("a(n) = 11 a(n-1) - 24 a(n-2)");
  expect(one).toContain("level L");
});

test("with the mix on the stage draws a second panel, its own terms and its own rule, indexed from 0", () => {
  const two = said(
    open([
      { row: ROW, rule: rule(num, ROW.terms), note: "" },
      { row: MIX, rule: rule(num, MIX.terms), note: "the mix stops where 128 bits do" },
    ]).frame(),
  );
  expect(two).toContain("differences of carpet 7 surface");
  expect(two).toContain("the first sequence, index from 0");
  expect(two).toContain("the mix stops where 128 bits do");
  expect(two.filter((text) => text.startsWith("a(n) = 11 a(n-1)"))).toHaveLength(2);
});

test("set takes a new chart in place, and a tap on a pin picks that term and a second tap lets it go", () => {
  const { scene, frame } = open([{ row: ROW, rule: rule(num, ROW.terms), note: "" }]);
  frame();
  scene.set({ chart: "ratios", digits: 2 });
  expect(said(frame())).toContain("each term over the one before");
  scene.set({ chart: "pins", digits: 2 });
  frame();
  const got = scene.tap(500, 300);
  expect(got.panel === 0 && got.term >= 0).toBe(true);
  expect(scene.tap(500, 300)).toEqual({ panel: 0, term: -1 });
  expect(scene.tap(5, 5)).toBe(null);
});

test("the measures follow the space, the mix rows show only with the mix on, and the argument only for the operations that take one", () => {
  const keys = (value) =>
    describe(SPEC, { ...defaults(SPEC), dim: 2, base: 2, ...value }).flatMap(({ rows }) => rows.map((row) => row.key));
  expect(keys({}).filter((key) => ["op", "scode", "smeasure", "arg"].includes(key))).toEqual([]);
  expect(keys({ mix: 1, op: "hadamard" })).toEqual(expect.arrayContaining(["op", "scode", "smeasure"]));
  expect(keys({ mix: 1, op: "hadamard" })).not.toContain("arg");
  expect(keys({ mix: 1, op: "shift" })).toContain("arg");
  expect(keys({ mix: 1, op: "shift" })).not.toContain("scode");
  expect(["shift", "decimate", "scale"].map((op) => tidy(SPEC, { mix: 1, op, arg: -5 }).arg)).toEqual([1, 2, -5]);
  expect([
    tidy(SPEC, { dim: 2, base: 2, measure: "faces" }).measure,
    tidy(SPEC, { dim: 3, base: 2, measure: "faces" }).measure,
  ]).toEqual(["surface", "faces"]);
  expect(PAGE.keys.find((row) => row.button).act).toBe("mix");
});

test("the design is a slot on dim, base and code, and the measure and axis ride the library, hidden but kept in the url", () => {
  expect(PAGE.design).toEqual({
    dim: "dim",
    base: "base",
    code: "code",
    spaces: [
      [2, 2],
      [2, 3],
      [3, 2],
    ],
  });
  const keys = describe(SPEC, { ...defaults(SPEC), dim: 2, base: 2 }).flatMap(({ rows }) => rows.map((row) => row.key));
  expect(keys).not.toContain("measure");
  expect(keys).not.toContain("axis");
  expect(tidy(SPEC, { dim: 2, base: 2, measure: "fills", axis: "side" })).toMatchObject({
    measure: "fills",
    axis: "side",
  });
});

test("an export is named by the row, the operation while the mix is on, and the chart", () => {
  expect(PAGE.name).toBe(filed);
  const value = { ...defaults(SPEC), dim: 2, base: 2, code: "7" };
  expect([
    filed(value),
    filed({ ...value, dim: 3, base: 3, code: "23", measure: "fills", mix: 1, op: "convolve", chart: "log" }),
  ]).toEqual(["plot-2-2-7-surface-level-pins", "plot-3-2-23-fills-level-convolve-log"]);
});
