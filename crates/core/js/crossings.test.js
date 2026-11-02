import { expect, test } from "bun:test";
import * as core from "./index.js";

const bytes = await Bun.file(new URL("./pkg/mrly_core_js_bg.wasm", import.meta.url)).arrayBuffer();
core.initSync({ module: bytes });

test("a tensor crosses both ways keeping its shape and its typed array kind", () => {
  const turned = core.tensor.rot90({ shape: [2, 3], data: Uint8Array.from([1, 2, 3, 4, 5, 6]) }, 1, [0, 1]);
  expect(turned.shape).toEqual([3, 2]);
  expect(turned.data).toBeInstanceOf(Uint8Array);
  expect(Array.from(turned.data)).toEqual([3, 6, 2, 5, 1, 4]);
  const wide = core.tensor.rot90({ shape: [1, 2], data: Uint32Array.from([70000, 1]) }, 0, [0, 1]);
  expect(wide.data).toBeInstanceOf(Uint32Array);
});

test("an image crosses in and back with its shape", () => {
  const image = { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 255]) };
  const grown = core.image.scale(image, 2);
  expect(grown.shape).toEqual([2, 4]);
  expect(grown.colors).toBeInstanceOf(Uint8Array);
  expect(Array.from(grown.colors.slice(0, 16))).toEqual([255, 0, 0, 255, 255, 0, 0, 255, 0, 0, 255, 255, 0, 0, 255, 255]);
});

test("an image that does not fill its shape is refused", () => {
  const scale = (image) => () => core.image.scale(image, 2);
  expect(scale({ shape: [2, 2], colors: new Uint8Array(4) })).toThrow("colors length must equal width * height.");
  expect(scale({ shape: [2, 2, 4], colors: new Uint8Array(16) })).toThrow("an image wants a shape of two lengths");
  expect(scale({ shape: [1, 1], colors: new Uint8Array(3) })).toThrow("image colors want four bytes a pixel.");
  expect(scale({ colors: new Uint8Array(4) })).toThrow("a shape wants an array of lengths.");
  expect(scale({ shape: [1, 1] })).toThrow("a Uint8Array or an array of bytes was wanted here.");
});

test("a color crosses out as a four-element array", () => {
  expect(core.colors.from_hex("#ff3d40")).toEqual([255, 61, 64, 255]);
  expect(core.colors.from_hex("008cff80")).toEqual([0, 140, 255, 128]);
  expect(core.colors.RED()).toEqual(core.colors.from_hex("#ff3d40"));
});

test("a rust error is thrown as a JS Error carrying its message", () => {
  expect(() => core.colors.from_hex("nope")).toThrow("Hex code must be in format #RRGGBB or #RRGGBBAA");
});

test("one seed replays one stream through the Rng class", () => {
  const draw = (rng) => [rng.unit(), rng.below(100), rng.range(1, 6), rng.boolean()];
  expect(draw(new core.Rng(2026))).toEqual(draw(new core.Rng(2026)));
  expect(draw(new core.Rng(2026))).not.toEqual(draw(new core.Rng(2027)));
  expect(draw(new core.Rng(2026n))).toEqual(draw(new core.Rng("2026")));
});

test("an Rng is passed where rust takes a mutable stream", () => {
  const rng = new core.Rng(7);
  const color = core.colors.random(false, rng);
  expect(color).toHaveLength(4);
  expect(color[3]).toBe(255);
  expect(core.colors.random(false, new core.Rng(7))).toEqual(color);
  expect(core.colors.random(false, rng)).not.toEqual(color);
});

test("the Rng chooses and shuffles as rust does", () => {
  const items = ["a", "b", "c", "d", "e", "f", "g"];
  expect(new core.Rng(5).choice(items)).toBe("c");
  expect(new core.Rng(5).choice(items)).toBe(items[new core.Rng(5).below(items.length)]);
  expect(() => new core.Rng(1).choice([])).toThrow("a choice wants at least one item.");
  const once = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  const again = [...once];
  new core.Rng(9).shuffle(once);
  new core.Rng(9).shuffle(again);
  expect(once).toEqual([2, 8, 4, 7, 9, 0, 6, 1, 3, 5]);
  expect(again).toEqual(once);
});
