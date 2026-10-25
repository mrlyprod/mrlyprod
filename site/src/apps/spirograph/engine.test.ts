import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { defaults, tidy } from "../../lib/app/spec.ts";
import { rng } from "../../lib/rand.ts";
import type { Shape } from "./engine.ts";
import {
  BANDS,
  bands,
  cover,
  DENSITY,
  fit,
  GLANCE,
  glance,
  HOLD,
  LEAST,
  legal,
  MOST,
  played,
  RASTER,
  ROUND,
  resolve,
  roll,
  seat,
  share,
  span,
  study,
  TRACKS,
} from "./engine.ts";
import type { Spirograph } from "./scene.ts";
import { SPEC } from "./scene.ts";

const value = (over: Partial<Spirograph>) => ({ ...defaults(SPEC), seed: 1, ...over }) as Spirograph;

const ones = (code: string) => Number(code).toString(2).split("1").length - 1;

test("a rolled code is a plane design with three fills or more in base 3 and three corners or more in base 2, the same for one seed", () => {
  const three = Array.from({ length: 50 }, (_, i) => roll(math, 3, rng(i)));
  const two = Array.from({ length: 50 }, (_, i) => roll(math, 2, rng(i)));
  expect(three.every((code) => Number(code) < 512 && ones(code) >= 3)).toBe(true);
  expect(two.every((code) => Number(code) < 16 && ones(code) >= 3)).toBe(true);
  expect(roll(math, 3, rng(9))).toBe(roll(math, 3, rng(9)));
});

test("the level is held to what the side allows", () => {
  const corners = seat(math, value({ number: 7, level: 3, mode: "corners" }), "495");
  expect([corners.level, corners.pencils.length, corners.width]).toEqual([2, 2356, 49]);
  const small = seat(math, value({ number: 3, level: 3, mode: "corners" }), "495");
  expect([small.level, small.pencils.length]).toEqual([3, 688]);
});

test("a wheel inside a ring is made smaller than it", () => {
  expect(legal("in", 7, 3, 4)).toEqual([7, 3]);
  expect(legal("in", 3, 7, 4)).toEqual([3, 2]);
  expect(legal("in", 1, 1, 4)).toEqual([2, 1]);
  expect(legal("out", 3, 7, 4)).toEqual([3, 7]);
});

test("a wheel inside a polygon is made to fit, the crate agreeing on every setting the sliders allow", () => {
  expect(legal("polyin", 7, 3, 4)).toEqual([7, 3]);
  expect(legal("polyin", 5, 3, 3)).toEqual([5, 2]);
  expect(legal("polyin", 4, 3, 4)).toEqual([4, 2]);
  expect(legal("polyin", 6, 3, 3)).toEqual([6, 2]);
  expect(legal("polyin", 1, 1, 3)).toEqual([3, 1]);
  const s = math.spirograph;
  for (let ring = 1; ring <= 24; ring++) {
    for (let wheel = 1; wheel <= 24; wheel++) {
      for (let sides = 3; sides <= 12; sides++) {
        const [r, w] = legal("polyin", ring, wheel, sides);
        expect([ring, wheel, sides, r >= ring, w >= 1 && w <= wheel, s.track("polyin", r, w, sides, 1).wheel]).toEqual([
          ring,
          wheel,
          sides,
          true,
          true,
          w,
        ]);
        if (w < wheel && r * Math.cos(Math.PI / sides) - (w + 1) < -1e-6)
          expect(() => s.track("polyin", r, w + 1, sides, 1)).toThrow();
      }
    }
  }
});

test("the carpet of 3 inside 7 over 3 seats 8 pencils on 8 curves with 1288 nodes, closing after 3 orbits 7-fold", () => {
  const plan = study(math, value({}), "495");
  expect(plan.facts).toMatchObject({
    code: "495",
    name: "carpet",
    base: 3,
    level: 1,
    pencils: 8,
    fills: 8,
    distinct: 8,
    nodes: 1288,
    ratio: [7, 3],
    orbits: 3,
    fold: 7,
    turns: 4,
    round: true,
  });
  expect([plan.beats, plan.samples, plan.trace.length]).toEqual([4, DENSITY * 4 + 1, 8 * (DENSITY * 4 + 1) * 2]);
});

test("the base-2 carpet seats the same 8 pencils", () => {
  expect(study(math, value({ base: 2 }), "7").facts).toMatchObject({
    code: "7",
    name: "carpet",
    base: 2,
    pencils: 8,
    distinct: 8,
    nodes: 1288,
  });
});

test("the samples follow the orbits of the centre when they outnumber the turns of the wheel", () => {
  const plan = study(math, value({ ring: 24, wheel: 23 }), "495");
  expect([plan.turns, plan.facts.orbits, plan.beats, plan.samples]).toEqual([1, 23, 23, DENSITY * 23 + 1]);
});

test("the cover is a 512 raster of the disc, 256 past 64 curves, and on a line or polygon only the reason there is none", () => {
  const carpet = cover(math, study(math, value({}), "495")) as Shape;
  expect([carpet.side, carpet.mask.length, carpet.covered > 0.7, carpet.wall > 0, carpet.hole > 0]).toEqual([
    RASTER[0],
    RASTER[0] ** 2,
    true,
    true,
    true,
  ]);
  expect(carpet.disc).toMatchObject({ x: 0, y: 0 });
  expect(carpet.disc.radius).toBeCloseTo(5.8);
  expect((cover(math, study(math, value({ mode: "both", level: 2 }), "495")) as Shape).side).toBe(RASTER[1]);
  expect(cover(math, study(math, value({ kind: "line" }), "495"))).toEqual({ fault: ROUND });
});

test("a line track has no nodes, and a full design seats no void pencil", () => {
  const line = study(math, value({ kind: "line" }), "495");
  expect([line.facts.nodes, line.facts.round]).toEqual([null, false]);
  const none = study(math, value({ mode: "void" }), "511");
  expect([none.facts.pencils, none.trace.length, none.facts.distinct]).toEqual([0, 0, 0]);
});

test("a polygon too tight for the wheel still lays a track", () => {
  const plan = study(math, value({ kind: "polyin", ring: 5, wheel: 3, sides: 3 }), "495");
  expect([plan.track.kind, plan.track.wheel, plan.facts.pencils]).toEqual(["polyin", 2, 8]);
});

test("the span runs at half a beat a second between three seconds and a minute", () => {
  expect([span(1), span(7), span(23), span(437)]).toEqual([LEAST, 14000, 46000, MOST]);
});

test("a play rises from its share to whole over the rest of the span and stays whole", () => {
  const length = span(7);
  expect([
    played(0, 0, length),
    played(0, length / 2, length),
    played(0.5, length / 4, length),
    played(0.5, length, length),
    played(0.2, -5, length),
  ]).toEqual([0, 0.5, 0.75, 1, 0.2]);
});

test("the lock share rises to one over the span, holds, then starts over, and is whole in a still", () => {
  const length = span(7);
  expect([
    share(0, length, false),
    share(length / 2, length, false),
    share(length + HOLD / 2, length, false),
    share(length + HOLD + length / 4, length, false),
    share(0, length, true),
  ]).toEqual([0, 0.5, 1, 0.25, 1]);
});

test("seats nearer the rim take the higher band", () => {
  const seats = bands([
    { x: 0.1, y: 0 },
    { x: 0.5, y: 0.5 },
    { x: 0, y: 1 },
  ]);
  expect(seats).toEqual([0, Math.floor(((Math.SQRT1_2 - 0.1) / 0.9) * BANDS), BANDS - 1]);
  expect(
    bands([
      { x: 0.3, y: 0 },
      { x: 0, y: 0.3 },
    ]),
  ).toEqual([BANDS - 1, BANDS - 1]);
});

test("the fit centres the frame on the stage inside the padding", () => {
  const box = fit([-7, -7, 7, 7], 400, 300, 10);
  expect(box.k).toBe(20);
  expect([box.ox, box.oy]).toEqual([200, 150]);
  expect([box.ox + 7 * box.k, box.oy - 7 * box.k]).toEqual([340, 10]);
});

test("an empty code is the home design of its base; a code kept is read as it stands", () => {
  expect([
    resolve(math, { base: 3, code: "" }),
    resolve(math, { base: 2, code: " " }),
    resolve(math, { base: 3, code: "13" }),
  ]).toEqual(["495", "7", "13"]);
});

test("every library track is legal as it stands, and a glance draws a curve for each pencil of the wheel at the thumb density", () => {
  for (const [, patch] of TRACKS) {
    const v = tidy(SPEC, { ...value({}), ...patch }, { math });
    expect(Object.keys(patch).map((key) => v[key])).toEqual(Object.values(patch));
    const one = glance(math, value({}), "495", patch);
    expect([one.count, one.trace.length, one.samples >= 2 && one.samples <= GLANCE * 8 + 1]).toEqual([
      8,
      8 * one.samples * 2,
      true,
    ]);
  }
  expect(new Set(TRACKS.map(([name]) => name)).size).toBe(TRACKS.length);
});
