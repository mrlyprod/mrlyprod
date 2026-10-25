import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import type { Shape } from "../designs/solid.ts";
import { SHAPES } from "./engine.ts";
import type { Pose } from "./thumbs.ts";
import { BLOCK, block, glance, POSES } from "./thumbs.ts";

const faces = (shape: Shape, pose: Pose) => {
  const log: PropertyKey[] = [];
  const pen = new Proxy({} as CanvasRenderingContext2D, { get: (_, key) => () => log.push(key), set: () => true });
  glance(pen, shape, POSES[pose], 56, { paper: "#ffffff", accent: "#008cff" });
  return log.filter((key) => key === "fill").length;
};

test("a look draws one cube from its corner, its edge and its face: three faces, three and two", () => {
  expect((["corner", "edge", "face"] as const).map((pose) => faces(BLOCK, pose))).toEqual([3, 3, 2]);
});

test("every crop is a thumb of its own, a smaller solid than the whole block", () => {
  const sizes = SHAPES.map((one) => block(math, one).cells.length);
  expect(sizes.every((size) => size > 0 && size < block(math, "").cells.length)).toBe(true);
  expect(block(math, "ball")).toBe(block(math, "ball"));
});
