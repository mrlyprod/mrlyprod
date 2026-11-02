import { expect, test } from "bun:test";
import * as tile from "./index.js";

const bytes = await Bun.file(new URL("./pkg/mrly_tile_js_bg.wasm", import.meta.url)).arrayBuffer();
tile.initSync({ module: bytes });

test("an image crosses out as its shape and a Uint8Array of rgba colors", () => {
  const image = tile.background(7, 2, 3);
  expect(image.shape).toEqual([27, 18]);
  expect(image.colors).toBeInstanceOf(Uint8Array);
  expect(image.colors.length).toBe(27 * 18 * 4);
  expect(image).toEqual(tile.background("7", 2, 3));
});

test("an image field is set and read back, and refused when it does not fill its shape", () => {
  const image = { shape: [1, 2], colors: Uint8Array.from([255, 0, 0, 255, 0, 0, 255, 255]) };
  const file = new tile.variation.File(1, 1);
  expect(file.image).toBeUndefined();
  file.image = image;
  expect(file.image).toEqual(image);
  expect(() => (file.image = { shape: [2, 2], colors: new Uint8Array(4) })).toThrow("colors length must equal width * height.");
});

test("a class holds its value, reads its fields and crosses as plain data", () => {
  const made = new tile.Tile("Fractal");
  expect(made.group).toBe("Fractal");
  expect(tile.Tile.from(made.toJSON()).toJSON()).toEqual(made.toJSON());
  expect(() => made.check()).toThrow("wrong slot count");
});

test("a default crosses and the unit's Rng draws as rust does", () => {
  const made = tile.build.create_2d(tile.build.Config2d.default(), new tile.Rng(1));
  expect([made.group, made.width]).toEqual(["Special", 9]);
});
