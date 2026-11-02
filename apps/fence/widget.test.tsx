import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show breaks, repeats, repaired, hours removed, trust in percent, the desperate, the hour and the world in words", () => {
  const told = {
    breaks: 12,
    repeats: 3,
    repaired: 0,
    patched: 8,
    lost: 320,
    trust: 487,
    desperate: 7,
    hungry: 1,
    homeless: 6,
    hour: 120,
    length: 240,
    prison: "removal" as const,
    scarcity: 2,
    bots: 24,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["breaks", "12"],
    ["repeat", "3"],
    ["repaired", "0"],
    ["removed", "320 hours"],
    ["trust", "48%"],
    ["desperate", "7"],
    ["hour", "120 of 240"],
    [
      "world",
      "a break means the quiet house; scarcity some, 24 bots, 1 starving, 6 without a home, 8 patched by the village",
    ],
  ]);
  expect([
    rows({ ...told, over: "done" })[6][0],
    String(rows({ ...told, prison: "repair" })[7][1]).startsWith("a break is mended"),
    rows(null),
  ]).toEqual(["ran", true, []]);
});
