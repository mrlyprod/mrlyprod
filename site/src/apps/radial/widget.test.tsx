import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { renderToStaticMarkup } from "react-dom/server";
import { rng } from "../../lib/rand.ts";
import type { Facts } from "./scene.ts";
import { lit, Rosettes, rolled, rows, wrap } from "./widget.tsx";

test("a reroll draws the design, the copies and a blend other than meet, on a full turn", () => {
  const all = Array.from({ length: 40 }, (_, seed) => rolled(math, { source: "flat" }, rng(seed)));
  expect(
    all.every((one) => one.copies >= 1 && one.copies <= 36 && one.blend !== "meet" && one.full === 1 && one.code),
  ).toBe(true);
  expect(new Set(all.map((one) => one.blend)).size).toBeGreaterThan(2);
});

test("a tap wraps the copies round from 36 to 1", () => {
  expect([wrap(6, 1), wrap(36, 1), wrap(1, -1)]).toEqual([7, 1, 36]);
});

test("the readouts are the demo stats and the share of order zero, then the leading orders", () => {
  const facts: Facts = {
    kind: "flat",
    code: "495",
    name: "carpet",
    side: 81,
    copies: 6,
    step: 60,
    order: 4,
    petals: 12,
    share: 0.612,
    leading: [{ order: 4, share: 0.1 }],
  };
  expect(rows(facts)).toEqual([
    ["name", "carpet 495"],
    ["side", "81"],
    ["copies", "6"],
    ["step", "60°"],
    ["rotation order", "4"],
    ["petals", "12"],
    ["order 0", "61.2%"],
    ["order 4", "10.0%"],
  ]);
});

test("the library lists the six blends and the stacks, the one in hand lit, a stack lit only by its copies and its turn", () => {
  const value = {
    source: "design",
    blend: "sum",
    copies: 6,
    full: 1,
    step: 60,
    samples: 2,
    ramp: "heat",
    levels: 64,
    invert: 0,
    tint: "",
  };
  const html = renderToStaticMarkup(<Rosettes value={value} onChange={() => {}} math={math} />);
  expect([...html.matchAll(/class="label">([^<]*)</g)].map(([, name]) => name)).toEqual([
    "Mean",
    "Sum",
    "Union",
    "Meet",
    "Parity",
    "Difference",
    "Pair",
    "Three",
    "Six",
    "Twelve",
    "Afterglow",
    "Pentagram",
    "Golden angle",
  ]);
  expect(html.match(/class="on"/g)).toHaveLength(2);
  expect(html).toContain('<span class="sub">2 x 45°</span>');
  expect([
    lit(value, { copies: 6, full: 1 }),
    lit(value, { copies: 5, full: 1 }),
    lit({ ...value, full: 0, step: 45 }, { copies: 6, full: 0, step: 45 }),
    lit({ ...value, full: 0, step: 40 }, { copies: 6, full: 0, step: 45 }),
  ]).toEqual([true, false, true, false]);
});
