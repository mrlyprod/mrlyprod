import { expect, test } from "bun:test";
import * as num from "@mrlyprod/mrly/num";
import type { Raw } from "../../lib/app/spec.ts";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import { palette } from "../../lib/theme/palette.gen.js";
import type { Plan } from "./engine.ts";
import {
  BAND,
  BANDS,
  clicked,
  coords,
  FAINTLY,
  hit,
  INERT,
  inks,
  lay,
  name,
  PEEK,
  PLAIN,
  POINTS,
  parse,
  picture,
  product,
  RAMIFIED,
  REACH,
  reached,
  SKIP,
  SPAN,
  SPLIT,
  share,
  shelves,
  shuffle,
  study,
  styleOf,
  table,
  tones,
  turned,
  UNITS,
  verdict,
  WINDOWS,
  walk,
} from "./engine.ts";
import { SPEC } from "./scene.ts";

const BUDGET = 160801;

const INK = inks({ paper: "#000000", accent: "#ff00ff" });

const tally = (plan: Plan) => {
  const counts = [0, 0, 0, 0, 0, 0];
  for (let p = 0; p < plan.count; p++) counts[plan.fate[p]]++;
  return counts;
};

test("a window cut from a wider survey holds every point of the window once, in order of norm, each classed as the census counts", () => {
  for (const ring of ["gaussian", "eisenstein"] as const) {
    study(num, ring, 150);
    const plan = study(num, ring, 12);
    const { facts } = plan;
    const window = new num.gauss.Window(plan.ring, 12);
    let held = 0;
    for (let p = 0; p < plan.count; p++) held += window.holds(plan.a[p], plan.b[p]) ? 1 : 0;
    window.free();
    expect([ring, plan.count, held]).toEqual([ring, num.gauss.Ring.count(plan.ring, 12), plan.count]);
    expect(tally(plan)).toEqual([facts.composites, facts.split, facts.inert, facts.ramified, facts.units, 1]);
    expect(facts.primes).toBe(facts.split + facts.inert + facts.ramified);
    for (let p = 1; p < plan.count; p++) expect(plan.norm[p] >= plan.norm[p - 1]).toBe(true);
    expect([plan.a[0], plan.b[0], plan.norm[0], plan.fate[0]]).toEqual([0, 0, 0, 5]);
    expect(new Set(Array.from({ length: plan.count }, (_, p) => `${plan.a[p]},${plan.b[p]}`)).size).toBe(plan.count);
  }
});

test("the gaussian window is the full square and the eisenstein window leaves its corners out", () => {
  const square = study(num, "gaussian", 12);
  expect([square.word, square.facts.ring, square.count, square.top, square.facts.units]).toEqual([
    "gaussian",
    "gaussian",
    625,
    288,
    4,
  ]);
  expect(square.frame).toEqual([-12.5, -12.5, 12.5, 12.5]);
  const hex = study(num, "eisenstein", 12);
  expect([hex.word, hex.facts.ring, hex.top, hex.facts.symmetry, hex.facts.units]).toEqual([
    "eisenstein",
    "eisenstein",
    144,
    12,
    6,
  ]);
  expect([hit(num, hex, 12, -12), hit(num, hex, -12, 12), hit(num, hex, 12, 12)?.fate]).toEqual([null, null, 0]);
  expect(hex.frame[2] - hex.frame[0]).toBeCloseTo(25);
  expect(hex.frame[3] - hex.frame[1]).toBeCloseTo(12 * Math.sqrt(3) + 1);
});

test("the same window is studied once, and another ring or reach is cut afresh", () => {
  const plan = study(num, "gaussian", 12);
  expect(study(num, "gaussian", 12)).toBe(plan);
  expect(study(num, "gaussian", 13)).not.toBe(plan);
  expect(study(num, "eisenstein", 12)).not.toBe(plan);
});

test("the busiest norm counts every point of its shell, which the window holds whole", () => {
  for (const [ring, limit] of [
    ["gaussian", 12],
    ["gaussian", 1],
    ["eisenstein", 10],
    ["eisenstein", 12],
    ["eisenstein", 1],
  ] as const) {
    const { facts, norm, count } = study(num, ring, limit);
    let held = 0;
    for (let p = 0; p < count; p++) if (norm[p] === facts.busy) held++;
    expect([ring, limit, facts.most]).toEqual([ring, limit, held]);
  }
});

test("a shuffle rolls the ring, the reach and the colour from the seed, the same seed the same window", () => {
  const rolls = Array.from({ length: 60 }, (_, i) => shuffle(rng(i)));
  expect(new Set(rolls.map((one) => one.ring))).toEqual(new Set(["gaussian", "eisenstein"]));
  expect(new Set(rolls.map((one) => one.look))).toEqual(new Set(["fate", "norm", "plain"]));
  expect(rolls.every((one) => one.limit >= SPAN[0] && one.limit <= SPAN[1])).toBe(true);
  expect(new Set(rolls.map((one) => one.limit)).size).toBeGreaterThan(20);
  expect(shuffle(rng(5))).toEqual(shuffle(rng(5)));
});

test("the reach runs to the cap of its ring, which keeps the window under the point budget", () => {
  const limit = (over: Raw) => tidy(SPEC, { ...defaults(SPEC), limit: 999, ...over }).limit;
  expect([limit({ ring: "gaussian" }), limit({ ring: "eisenstein" })]).toEqual([REACH.gaussian, REACH.eisenstein]);
  expect(num.gauss.Ring.count("Gaussian", REACH.gaussian) <= BUDGET).toBe(true);
  expect(num.gauss.Ring.count("Gaussian", REACH.gaussian + 1) > BUDGET).toBe(true);
  expect(num.gauss.Ring.count("Eisenstein", REACH.eisenstein) <= BUDGET).toBe(true);
  expect(num.gauss.Ring.count("Eisenstein", REACH.eisenstein + 1) > BUDGET).toBe(true);
});

test("a hit reads the norm, the fate, the factors, the conjugate, the associates and the points of its norm, and nothing off the window", () => {
  const plan = study(num, "gaussian", 12);
  const split = hit(num, plan, 2, 3);
  expect(split).toMatchObject({ a: 2, b: 3, norm: 13, fate: 1, factors: [[13, 1]], conjugate: [2, -3], shell: 8 });
  expect(split!.associates).toEqual([
    [2, 3],
    [-3, 2],
    [-2, -3],
    [3, -2],
  ]);
  expect([plan.a[split!.index], plan.b[split!.index]]).toEqual([2, 3]);
  expect(hit(num, plan, 3, 0)).toMatchObject({ norm: 9, fate: 2, factors: [[3, 2]], shell: 4 });
  expect(hit(num, plan, 1, 1)).toMatchObject({ norm: 2, fate: 3 });
  expect(hit(num, plan, 0, 1)).toMatchObject({ norm: 1, fate: 4, factors: [] });
  expect(hit(num, plan, 0, 0)).toMatchObject({ norm: 0, fate: 5, factors: [] });
  expect(hit(num, plan, 4, 2)).toMatchObject({
    norm: 20,
    fate: 0,
    factors: [
      [2, 2],
      [5, 1],
    ],
  });
  expect(hit(num, plan, 13, 0)).toBe(null);
  const hex = study(num, "eisenstein", 12);
  expect(hit(num, hex, 2, 1)).toMatchObject({ norm: 3, fate: 3, conjugate: [1, -1] });
  expect(hit(num, hex, 2, 0)).toMatchObject({ norm: 4, fate: 2 });
});

test("a walk steps through one point of each prime class in order of norm and wraps, and a turn moves a point to its next associate", () => {
  const plan = study(num, "gaussian", 12);
  const steps: string[] = [];
  let at = 0;
  for (let k = 0; k < 4; k++) {
    const [a, b] = walk(plan, at, 1)!;
    steps.push(name("gaussian", a, b));
    at = hit(num, plan, a, b)!.index;
  }
  expect(steps).toEqual(["1 + i", "1 + 2i", "2 + i", "3"]);
  const [a, b] = walk(plan, 0, -1)!;
  expect(walk(plan, hit(num, plan, a, b)!.index, 1)).toEqual([1, 1]);
  const split = hit(num, plan, 2, 3);
  expect([turned(split!, 1), turned(split!, -1), turned(hit(num, plan, 0, 0)!, 1)]).toEqual([[-3, 2], [3, -2], null]);
});

test("a point is named with its letter and its verdict says what became of its norm", () => {
  expect([
    name("gaussian", 3, 0),
    name("gaussian", 0, 1),
    name("gaussian", 0, -2),
    name("gaussian", 2, -3),
    name("eisenstein", 1, 1),
  ]).toEqual(["3", "i", "-2i", "2 - 3i", "1 + ω"]);
  expect(
    product([
      [2, 2],
      [5, 1],
    ]),
  ).toBe("2^2 · 5");
  const plan = study(num, "gaussian", 12);
  expect(clicked("gaussian", hit(num, plan, 2, 3)!)).toBe("2 + 3i at 2, 3");
  expect(verdict("gaussian", hit(num, plan, 2, 3)!)).toBe("prime: 13 splits as (2 + 3i)(2 - 3i)");
  expect(verdict("gaussian", hit(num, plan, 3, 0)!)).toBe("prime: 3 stays prime in the plane");
  expect(verdict("gaussian", hit(num, plan, 1, 1)!)).toBe("prime: 2 ramifies, a unit times a square");
  expect(verdict("gaussian", hit(num, plan, 0, 1)!)).toBe("a unit, norm 1");
  expect(verdict("gaussian", hit(num, plan, 0, 0)!)).toBe("the origin");
  expect(verdict("gaussian", hit(num, plan, 4, 2)!)).toBe("composite, norm 20 = 2^2 · 5");
});

test("a clicked point is written a,b and read back, and junk reads as no point", () => {
  expect([parse(coords(-3, 2)), parse(" 4 , 0 "), parse(""), parse("2;3"), parse(undefined)]).toEqual([
    [-3, 2],
    [4, 0],
    null,
    null,
    null,
  ]);
});

test("a growth runs once from empty to whole, the points reached growing with the square of the share", () => {
  expect([share(-5, 8000), share(0, 8000), share(4000, 8000), share(8000, 8000), share(20000, 8000)]).toEqual([
    0, 0, 0.5, 1, 1,
  ]);
  expect([reached(0, 101), reached(0.5, 101), reached(1, 101)]).toEqual([0, 25, 100]);
});

test("the layout fits the frame inside the padding or pins a cell size, centred either way", () => {
  const fit = lay([-7, -7, 7, 7], 400, 300, 10);
  expect([fit.k, fit.ox, fit.oy]).toEqual([20, 200, 150]);
  const pinned = lay([-7, -7, 7, 7], 400, 300, 10, 3);
  expect([pinned.k, pinned.ox, pinned.oy]).toEqual([3, 200, 150]);
});

test("each fate wears its own hue, the norm look a fire ramp by the root of the norm and plain the accent", () => {
  expect([styleOf("fate", 1, 5, 0, 100), styleOf("fate", 1, 0, 4, 100), styleOf("fate", 0, 0, 4, 100)]).toEqual([
    SKIP,
    FAINTLY,
    SKIP,
  ]);
  expect([1, 2, 3, 4].map((fate) => styleOf("fate", 1, fate, 5, 100))).toEqual([SPLIT, INERT, RAMIFIED, UNITS]);
  expect([styleOf("plain", 1, 2, 9, 100), styleOf("norm", 1, 4, 1, 100)]).toEqual([PLAIN, UNITS]);
  expect([styleOf("norm", 1, 1, 0, 100), styleOf("norm", 1, 1, 25, 100), styleOf("norm", 1, 1, 100, 100)]).toEqual([
    BAND,
    BAND + BANDS / 2,
    BAND + BANDS - 1,
  ]);
  const tone = tones(INK);
  expect(tone.length).toBe(BAND + BANDS);
  expect([tone[SKIP], tone[SPLIT], tone[INERT], tone[RAMIFIED], tone[UNITS], tone[PLAIN]]).toEqual([
    null,
    palette.blue,
    palette.orange,
    palette.pink,
    palette.green,
    "#ff00ff",
  ]);
  expect([tone[FAINTLY], tone[BAND], tone[BAND + BANDS - 1]]).toEqual([
    "rgba(255, 255, 255, 0.16)",
    "rgb(255, 61, 64)",
    "rgb(255, 255, 255)",
  ]);
  expect(inks({ paper: "#ffffff", accent: "#000" }).fg).toBe("rgb(0, 0, 0)");
});

test("the svg lays the gaussian window as runs of rects per hue and the eisenstein window as circles, and the csv lists the primes", () => {
  const tone = tones(INK);
  const square = study(num, "gaussian", 12);
  const style = Uint8Array.from(square.fate, (f, p) => styleOf("fate", 1, f, square.norm[p], square.top));
  const text = picture(square, style, tone);
  expect(
    text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="250" height="250" viewBox="0 0 250 250">'),
  ).toBe(true);
  for (const hue of [palette.blue, palette.orange, palette.pink, palette.green])
    expect(text.match(new RegExp(`<g fill="${hue}">`, "g"))).toHaveLength(1);
  expect(text.match(/<rect /g)!.length).toBeGreaterThanOrEqual(square.facts.primes);
  const hex = study(num, "eisenstein", 12);
  const round = picture(
    hex,
    Uint8Array.from(hex.fate, (f, p) => styleOf("plain", 0, f, hex.norm[p], hex.top)),
    tone,
  );
  expect(round.match(/<circle /g)).toHaveLength(hex.facts.primes + hex.facts.units);
  expect(round).not.toContain("<rect");
  const rows = table(square).trim().split("\n");
  expect(rows[0]).toBe("a,b,norm,fate");
  expect(rows.length - 1).toBe(square.facts.primes);
  expect(rows).toContain("1,1,2,ramified");
  expect(rows).toContain("2,3,13,split");
});

test("every library point is the prime it says in its window, its norm and class as the survey finds them", () => {
  for (const { ring, a, b, norm, fate } of POINTS) {
    const point = hit(num, study(num, ring, PEEK), a, b);
    expect([ring, a, b, point!.norm, point!.fate]).toEqual([ring, a, b, norm, fate]);
  }
});

test("the library marks the window and the point the page holds, and picking a window lets the point go", () => {
  const [windows, primes] = shelves({ ring: "eisenstein", limit: 12, look: "fate", click: "3,1" });
  expect(windows[1].filter((one) => one.on).map((one) => one.id)).toEqual(["eisenstein-close"]);
  expect(primes[1].filter((one) => one.on).map((one) => [one.label, one.sub])).toEqual([["3 + ω", "7 splits"]]);
  expect(WINDOWS.every(({ set }) => set.click === "")).toBe(true);
  expect(primes[1].map((one) => one.sub)).toContain("3 stays prime");
});
