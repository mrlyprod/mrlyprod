import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { ink } from "#view";
import { light } from "../theme/theme.ts";
import type { Cropping } from "./engine.ts";
import {
  cap,
  circle,
  colorizer,
  cropped,
  design,
  field,
  grown,
  odd,
  plane,
  reading,
  region,
  SHAPES,
  shell,
  walls,
} from "./engine.ts";

const carpet = () => grown(math, "23", 2, 3, 3);

test("the level reaches the demo's side of 128, and 81 under a crop", () => {
  expect([2, 3, 4, 5].map((n) => cap(math, n))).toEqual([7, 4, 3, 3]);
  expect([2, 3, 4, 5].map((n) => cap(math, n, "ball"))).toEqual([6, 4, 3, 2]);
});

test("a blank code is the base's carpet and a given code is kept", () => {
  expect([
    design(math, { base: 2, code: "" }),
    design(math, { base: 3, code: "" }),
    design(math, { base: 2, code: " 126 " }),
    design(math, { base: 3, code: "100" }),
  ]).toEqual([
    { base: 2, code: "23" },
    { base: 3, code: "129928175" },
    { base: 2, code: "126" },
    { base: 3, code: "100" },
  ]);
});

test("the shapes are the crate's named 3D shapes", () => {
  expect<string[]>(SHAPES).toEqual(math.shape.shapes(3));
});

test("a crop tallies in, cut and out; inside keeps fewer cells than touching; anti keeps the outside", () => {
  const at = (over: Partial<Cropping>) =>
    cropped(math, carpet(), { crop: "ball", radius: 60, anti: 0, policy: "touching", ...over });
  expect([at({}).in, at({}).cut, at({}).out]).toEqual([2320, 1224, 4456]);
  expect([math.three.fills(at({ policy: "inside" }).cell), math.three.fills(at({}).cell)]).toEqual([2320, 3544]);
  expect([at({ anti: 1 }).in, at({ anti: 1 }).out]).toEqual([4456, 2320]);
  expect(at({}).after).toBe(Number(math.three.surface(at({}).cell)));
});

test("the exact walls of a box keep the mesh box the shape covers, in the mesh's own units", () => {
  const planes = walls(region(math, "box", 30, false))!;
  const keeps = (p: number[]) =>
    planes.every(({ normal, constant }) => normal[0] * p[0] + normal[1] * p[1] + normal[2] * p[2] + constant >= -1e-9);
  expect(planes.length).toBe(6);
  expect([keeps([0.5, 0.5, -0.5]), keeps([0.51, 0, 0]), keeps([0, 0, -0.51])]).toEqual([true, false, false]);
  expect(walls(region(math, "ball", 30, false))).toBeNull();
});

test("the circle count about the corner matches the crate's own sponge reading, clamped to the grid", () => {
  const count = circle(math, { code: "23", base: 2 }, { number: 3, count: 4, centre: "corner" });
  expect([count.side, count.top, count.mass]).toEqual([81, 80, 20]);
  expect(count.seen.slice(1, 9)).toEqual([1, 4, 13, 28, 47, 65, 95, 137]);
  expect(reading(count, 2)).toEqual({ at: 2, seen: 4, inside: 1, cut: 6 });
  expect(reading(count, 500).at).toBe(80);
  expect(circle(math, { code: "23", base: 2 }, { number: 3, count: 4, centre: "centre" }).top).toBe(40);
});

test("the field stacks the odd scales, its shell keeps the voxels at the threshold, and a plane is clear outside the cube", () => {
  const made = field(math, { code: "23", base: 2 }, { limit: 7, combine: "sum", depth: 1, size: 64 });
  expect([odd(7), made.layers, made.voxels, made.min, made.max]).toEqual([[1, 3, 5, 7], 4, 262144, 1, 4]);
  const top = shell(math, made, { threshold: 4, crop: "" });
  expect([top.count, top.faces, math.three.fills(top.cell)]).toEqual([76608, 42408, 76608]);
  const paint = { colorizer: colorizer(ink(light), "fire", 16), levels: 16, invert: 0 };
  const cut = plane(math, made, "d", 0.5, paint);
  expect([cut.pixels.shape, cut.frame.normal.map((v) => +v.toFixed(3))]).toEqual([
    [384, 384],
    [0.577, 0.577, 0.577],
  ]);
  expect([cut.pixels.colors[3], cut.pixels.colors[(192 * 384 + 192) * 4 + 3]]).toEqual([0, 255]);
  expect(plane(math, made, "d", 0.5, paint)).toBe(cut);
});

test("a colorizer bins its ramp into the levels, the ground first", () => {
  const bins = colorizer(ink(light), "heat", 4).Bins;
  expect([bins.ramp.length, bins.background, bins.ramp[0]]).toEqual([4, "#ffffff", "#ffffff"]);
});
