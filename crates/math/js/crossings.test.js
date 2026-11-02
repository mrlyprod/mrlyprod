import { expect, test } from "bun:test";
import * as math from "./index.js";

const bytes = await Bun.file(new URL("./pkg/mrly_math_js_bg.wasm", import.meta.url)).arrayBuffer();
math.initSync({ module: bytes });

test("a cell crosses out as a shape and a typed array of types", () => {
  const cell = math.two.carpet(3, 1);
  expect(cell.shape).toEqual([3, 3]);
  expect(cell.types).toBeInstanceOf(Uint8Array);
  expect(Array.from(cell.types)).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(cell.colors).toBeUndefined();
  expect(cell.tags).toBeUndefined();
});

test("a cell crosses in and a serde struct crosses out as a plain object", () => {
  const read = math.two.census(math.two.carpet(3, 1));
  expect(read).not.toBeInstanceOf(Map);
  expect(read).toEqual({ fills: 8, voids: 1, perimeter: 16n, vertices: 16, edges: 24, euler: 0 });
});

test("an optional Rng is passed where rust takes a mutable stream", () => {
  const mapping = { 0: [[255, 255, 255, 255]], 1: [[255, 61, 64, 255], [0, 140, 255, 255]] };
  const painted = math.cell.paint(math.two.carpet(3, 1), mapping, "Random", new math.Rng(7));
  expect(painted.colors).toBeInstanceOf(Uint8Array);
  expect(painted.colors.length).toBe(36);
  expect(math.cell.paint(math.two.carpet(3, 1), mapping, "Random", new math.Rng(7))).toEqual(painted);
});
