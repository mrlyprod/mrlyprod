import { expect, test } from "bun:test";
import * as core from "./pkg/core/mrlyjs_core.js";

const bytes = await Bun.file(new URL("./pkg/core/mrlyjs_core_bg.wasm", import.meta.url)).arrayBuffer();
core.initSync({ module: bytes });

test("a cell crosses out as a shape and a typed array of types", () => {
  const cell = core.two_carpet(3, 1);
  expect(cell.shape).toEqual([3, 3]);
  expect(cell.types).toBeInstanceOf(Uint8Array);
  expect(Array.from(cell.types)).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(cell.colors).toBeUndefined();
  expect(cell.tags).toBeUndefined();
});

test("a cell crosses in and a serde struct crosses out as a plain object", () => {
  const read = core.two_census(core.two_carpet(3, 1));
  expect(read).not.toBeInstanceOf(Map);
  expect(read).toEqual({ fills: 8, voids: 1, perimeter: 16n, vertices: 16, edges: 24, euler: 0 });
  expect(typeof read.perimeter).toBe("bigint");
});

test("a tensor crosses both ways keeping its shape and its typed array kind", () => {
  const turned = core.tensor_rot90({ shape: [2, 3], data: Uint8Array.from([1, 2, 3, 4, 5, 6]) }, 1, 0, 1);
  expect(turned.shape).toEqual([3, 2]);
  expect(turned.data).toBeInstanceOf(Uint8Array);
  expect(Array.from(turned.data)).toEqual([3, 6, 2, 5, 1, 4]);
  const wide = core.tensor_rot90({ shape: [1, 2], data: Uint32Array.from([70000, 1]) }, 0, 0, 1);
  expect(wide.data).toBeInstanceOf(Uint32Array);
});

test("a seeded png crosses out as a Uint8Array", () => {
  const png = core.gen_background(1, 4, 4);
  expect(png).toBeInstanceOf(Uint8Array);
  expect(Array.from(png.slice(0, 8))).toEqual([137, 80, 78, 71, 13, 10, 26, 10]);
  expect(Array.from(png)).toEqual(Array.from(core.gen_background(1, 4, 4)));
});

test("u128 crosses both ways as a decimal string", () => {
  expect(core.num_factor_gcd("1071", "462")).toBe("21");
  expect(core.num_factor_gcd(1071, 462)).toBe("21");
  expect(core.num_factor_gcd("340282366920938463463374607431768211454", "2")).toBe("2");
});

test("a color crosses out as a four-element array", () => {
  expect(core.color_from_hex("#ff3d40")).toEqual([255, 61, 64, 255]);
  expect(core.color_from_hex("008cff80")).toEqual([0, 140, 255, 128]);
});

test("a rust error is thrown as a JS Error carrying its message", () => {
  expect(() => core.color_from_hex("nope")).toThrow("Hex code must be in format #RRGGBB or #RRGGBBAA");
  expect(() => core.num_factor_gcd("x", 2)).toThrow("a u128 wants a decimal string, a whole number or a bigint.");
});

test("one seed replays one stream through the Rng class", () => {
  const draw = (rng) => [rng.unit(), rng.below(100), rng.range(1, 6), rng.boolean()];
  expect(draw(new core.Rng(2026))).toEqual(draw(new core.Rng(2026)));
  expect(draw(new core.Rng(2026))).not.toEqual(draw(new core.Rng(2027)));
  expect(draw(new core.Rng(2026n))).toEqual(draw(new core.Rng("2026")));
});

test("an Rng is passed where rust takes a mutable stream", () => {
  const rng = new core.Rng(7);
  const color = core.color_random(false, rng);
  expect(color).toHaveLength(4);
  expect(color[3]).toBe(255);
  expect(core.color_random(false, new core.Rng(7))).toEqual(color);
});
