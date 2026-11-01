import { expect, test } from "bun:test";
import * as num from "@mrlyprod/mrly/num";
import { defaults } from "../../lib/app/spec.ts";
import type { FactsOf, Knobs } from "./engine.ts";
import {
  bars,
  batteryCsv,
  biasCsv,
  designOf,
  FEATURED,
  holds,
  largest,
  lay,
  lit,
  said,
  scaleOf,
  signs,
  stepped,
  study,
  title,
  turned,
  verdict,
  verdictOf,
  warm,
  widthOf,
} from "./engine.ts";
import { SPEC, tidied } from "./scene.ts";

const value = (over: Record<string, unknown> = {}): Knobs => tidied({ ...defaults(SPEC), ...over });

const thue = designOf(value());

test("a design is read off the knobs: the base and the digit mask, or Rudin-Shapiro over them", () => {
  expect(designOf({ rudin: 0, base: 6, digits: "531" })).toEqual({ base: 6, mask: 0b10101, rudin: false });
  expect(num.readings.name(designOf({ rudin: 0, base: 6, digits: "531" }))).toBe("6:135");
  expect(designOf({ rudin: 0, base: 3, digits: "27" })).toEqual({ base: 3, mask: 0b10, rudin: false });
  expect(designOf({ rudin: 1, base: 6, digits: "531" })).toEqual({ base: 2, mask: 0, rudin: true });
  expect([
    title(num, thue),
    title(num, designOf({ rudin: 1 })),
    title(num, designOf({ rudin: 0, base: 6, digits: "5" })),
  ]).toEqual(["2:1, Thue-Morse", "Rudin-Shapiro", "6:5"]);
});

test("a fitted row is the base to the power nearest the square root of N, capped at N, and a set row is kept", () => {
  expect([
    widthOf(value()),
    widthOf(value({ power: 10 })),
    widthOf(value({ base: 6 })),
    widthOf(value({ rudin: 1, base: 6 })),
    widthOf(value({ width: 3 })),
    widthOf(value({ width: 20, power: 10 })),
  ]).toEqual([64, 32, 36, 64, 8, 1024]);
});

test("a shorter stream of the same design and reading is a prefix of the one already made", () => {
  const long = signs(num, thue, "Square", 200);
  const short = signs(num, thue, "Square", 50);
  expect([short.length, short.buffer === long.buffer, Array.from(short.slice(0, 6))]).toEqual([
    50,
    true,
    [1, -1, -1, 1, -1, -1],
  ]);
});

test("the battery bars carry z = |r| sqrt(N), the exact near-heads of the squares reading z = sqrt(N) exactly", () => {
  const heads = num.readings.near_heads(thue, "Square");
  const list = bars(verdict(num, thue, "Square", 1024), heads);
  expect(list).toHaveLength(64 + 44 + 2 + 1);
  const exact = list.filter((one) => one.head);
  expect(exact.map((one) => [one.label, one.z, one.head!.exact])).toEqual([
    ["(2, 0)", 32, true],
    ["(4, 0)", 32, true],
    ["(8, 0)", 32, true],
    ["b = 2", 32, true],
  ]);
  expect(largest(list, "square")!.z).toBe(32);
  expect(lit(list)[0].z).toBe(32);
  expect(largest(list, "shift")!.z).toBeLessThan(4);
});

test("the z scale is a log from the floor to a quarter above the tallest bar, at least 10", () => {
  const list = bars(verdict(num, thue, "Square", 1024), []);
  const scale = scaleOf(list);
  expect(scale.hi).toBe(40);
  expect([scale.at(0.05), scale.at(0.1), scale.at(40), scale.at(2) < scale.at(3)]).toEqual([0, 0, 1, true]);
  expect(scaleOf(list.filter((one) => one.z < 1)).hi).toBe(10);
});

test("the plaid study lays the stream over a fitted row and counts the bias", () => {
  const plan = study(num, value());
  expect(plan.view).toBe("plaid");
  const f = plan.facts as FactsOf<"plaid">;
  expect([f.width, f.rows, f.bias, f.z, f.plus, f.count, f.formula]).toEqual([
    64,
    64,
    -260,
    -4.0625,
    1918,
    4096,
    "n^2",
  ]);
  expect(study(num, value())).toBe(plan);
  expect(study(num, value({ width: 5 }))).not.toBe(plan);
});

test("the battery study names the largest z of each test and the verdict", () => {
  const f = study(num, value({ view: "battery" })).facts as FactsOf<"battery">;
  expect([f.largest.shift!.label, f.largest.dilation!.label, f.largest.kernel!.label, f.largest.square!.z]).toEqual([
    "h = 15",
    "(2, 0)",
    "b = 2",
    64,
  ]);
  expect(f.largest.shift!.z).toBeCloseTo(3.6613, 3);
  expect(f.verdict).toBe("fails: the dilation test at (2, 0), z 64.00 lights");
  expect(f.lit[0].label).toBe("(2, 0)");
});

test("the arcs study is pending until the model is warm, then reads the lab's prediction and the residual", () => {
  const cold = study(num, value({ view: "arcs", base: 4, digits: "13" })).facts as FactsOf<"arcs">;
  expect([cold.pending, cold.model, cold.predicted, cold.why]).toEqual([true, null, null, ""]);
  warm(num, thue);
  const f = study(num, value({ view: "arcs" })).facts as FactsOf<"arcs">;
  expect([f.pending, f.exponents, f.observed]).toEqual([false, [8, 9, 10, 11, 12], [-52, -74, -116, -160, -260]]);
  expect(f.predicted).toBeCloseTo(-273.296, 2);
  expect(f.residual).toBeCloseTo(13.296, 2);
  expect(f.model!.length).toBe(13);
  expect(f.arcs!.map((one) => `${one.num}/${one.den}`)).toEqual(["1/3", "2/3"]);
  expect(f.arcs![0].eta).toBeCloseTo(0.2075, 4);
});

test("off the squares the bias stands alone and the facts say why, and Rudin-Shapiro has no arc below the cut", () => {
  const cube = study(num, value({ view: "arcs", reading: "Cube" })).facts as FactsOf<"arcs">;
  expect([cube.pending, cube.model, cube.why]).toEqual([
    false,
    null,
    "the major arcs predict the squares; along this reading the bias stands alone",
  ]);
  warm(num, designOf({ rudin: 1 }));
  const rudin = study(num, value({ view: "arcs", rudin: 1 })).facts as FactsOf<"arcs">;
  expect([rudin.arcs, rudin.predicted, rudin.why]).toEqual([[], 1, "no arc below 0.25: the model is the constant 1"]);
});

test("the near-heads are said as pairs with their exact signs, the carries apart, and a free reading as none", () => {
  expect(said(num.readings.near_heads(thue, "Square"))).toBe(
    "(2, 0), (4, 0), (8, 0), kernel 2 exact, reading +1, +1, +1, +1",
  );
  expect(said(num.readings.near_heads(designOf({ rudin: 0, base: 3, digits: "2" }), "SquarePlusN"))).toBe(
    "(3, 1), (9, 4) exact, reading -1, +1",
  );
  expect(said(num.readings.near_heads(designOf({ rudin: 1 }), "N"))).toContain(
    "(2, 1), (4, 2), (4, 3), (8, 4), (8, 5), (8, 6), (8, 7) with a carry",
  );
  expect(said(num.readings.near_heads(thue, "Primes"))).toBe("none: a free reading");
});

test("the verdict passes inside 3, sits at the edge up to 4, and fails past it", () => {
  const list = bars(verdict(num, thue, "Square", 1024), []);
  const quiet = list.filter((one) => one.z <= 3);
  const edge = { ...quiet[0], label: "(8, 7)", test: "dilation" as const, z: 3.24 };
  expect([verdictOf(quiet), verdictOf([...quiet, edge]), verdictOf(list)]).toEqual([
    "passes: every z inside 3",
    "at the edge: the dilation test at (8, 7), z 3.24, inside the lab's 4",
    "fails: the dilation test at (2, 0), z 32.00 lights",
  ]);
});

test("N steps through the powers and wraps, and the views turn in a ring", () => {
  expect([stepped({ power: 12 }, 1), stepped({ power: 20 }, 1), stepped({ power: 10 }, -1)]).toEqual([
    { power: 13 },
    { power: 10 },
    { power: 20 },
  ]);
  expect([turned({ view: "plaid" }, 1), turned({ view: "arcs" }, 1), turned({ view: "plaid" }, -1)]).toEqual([
    { view: "battery" },
    { view: "plaid" },
    { view: "arcs" },
  ]);
});

test("a raster is laid centred at a whole number of pixels a cell, or shrunk when the stage is smaller", () => {
  expect(lay(800, 600, 64, 64, 16)).toEqual({ x: 144, y: 44, w: 512, h: 512, cell: 8 });
  expect(lay(100, 100, 1024, 1024, 0)).toEqual({ x: 0, y: 0, w: 100, h: 100, cell: 100 / 1024 });
});

test("the featured designs match the knobs that spell them", () => {
  expect(FEATURED.map((one) => holds(value(), one.pick))).toEqual([
    true,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);
  expect(holds(value({ rudin: 1 }), FEATURED[1].pick)).toBe(true);
  expect(holds(value({ base: 6, digits: "531" }), FEATURED[4].pick)).toBe(true);
});

test("the battery sheet lists every bar with its near-head, the bias sheet every checkpoint with the model along the squares", () => {
  const battery = batteryCsv(num, value()).split("\n");
  expect(battery).toHaveLength(1 + 111);
  expect(battery[0]).toBe("design,reading,N,test,pair,r,z,near-head,exact");
  expect(battery.find((row) => row.includes("dilation,(2, 0)"))).toBe("2:1,n^2,4096,dilation,(2, 0),1,64,yes,yes");
  const bias = biasCsv(num, value()).split("\n");
  expect([bias.length, bias[0], bias[5].split(",").slice(0, 6)]).toEqual([
    6,
    "design,reading,e,N,S,z,model,z_model,residual",
    ["2:1", "n^2", "12", "4096", "-260", "-4.0625"],
  ]);
  expect(Number(bias[5].split(",")[6])).toBeCloseTo(-273.296, 2);
  expect(biasCsv(num, value({ reading: "Cube" })).split("\n")[1]).toBe("2:1,n^3,8,256,-2,-0.125,,,");
});
