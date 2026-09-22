import { expect, test } from "bun:test";
import { Rng, core, gen, initSync, life, math, num } from "./index.js";

const bytes = await Bun.file(new URL("./pkg/all/mrlyjs_all_bg.wasm", import.meta.url)).arrayBuffer();
initSync({ module: bytes });

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

test("a tensor crosses both ways keeping its shape and its typed array kind", () => {
  const turned = core.tensor.rot90({ shape: [2, 3], data: Uint8Array.from([1, 2, 3, 4, 5, 6]) }, 1, [0, 1]);
  expect(turned.shape).toEqual([3, 2]);
  expect(turned.data).toBeInstanceOf(Uint8Array);
  expect(Array.from(turned.data)).toEqual([3, 6, 2, 5, 1, 4]);
  const wide = core.tensor.rot90({ shape: [1, 2], data: Uint32Array.from([70000, 1]) }, 0, [0, 1]);
  expect(wide.data).toBeInstanceOf(Uint32Array);
});

test("a seeded png crosses out as a Uint8Array", () => {
  const png = gen.background(1, 4, 4);
  expect(png).toBeInstanceOf(Uint8Array);
  expect(Array.from(png.slice(0, 8))).toEqual(Array.from(core.PNG_MAGIC()));
  expect(Array.from(png)).toEqual(Array.from(gen.background("1", 4, 4)));
});

test("u128 crosses both ways as a decimal string", () => {
  expect(num.factor.gcd("1071", "462")).toBe("21");
  expect(num.factor.gcd(1071, 462n)).toBe("21");
  expect(num.factor.gcd("340282366920938463463374607431768211454", "2")).toBe("2");
});

test("a color crosses out as a four-element array", () => {
  expect(core.colors.from_hex("#ff3d40")).toEqual([255, 61, 64, 255]);
  expect(core.colors.from_hex("008cff80")).toEqual([0, 140, 255, 128]);
  expect(core.colors.RED()).toEqual(core.colors.from_hex("#ff3d40"));
});

test("a rust error is thrown as a JS Error carrying its message", () => {
  expect(() => core.colors.from_hex("nope")).toThrow("Hex code must be in format #RRGGBB or #RRGGBBAA");
  expect(() => num.factor.gcd("x", 2)).toThrow("a u128 wants a decimal string, a whole number or a bigint.");
});

test("one seed replays one stream through the Rng class", () => {
  const draw = (rng) => [rng.unit(), rng.below(100), rng.range(1, 6), rng.boolean()];
  expect(draw(new Rng(2026))).toEqual(draw(new Rng(2026)));
  expect(draw(new Rng(2026))).not.toEqual(draw(new Rng(2027)));
  expect(draw(new Rng(2026n))).toEqual(draw(new Rng("2026")));
});

test("an Rng is passed where rust takes a mutable stream, required or optional", () => {
  const rng = new Rng(7);
  const color = core.colors.random(false, rng);
  expect(color).toHaveLength(4);
  expect(color[3]).toBe(255);
  expect(core.colors.random(false, new Rng(7))).toEqual(color);
  const mapping = { 0: [core.colors.WHITE()], 1: [core.colors.RED(), core.colors.BLUE()] };
  const painted = math.cell.paint(math.two.carpet(3, 1), mapping, "Random", rng);
  expect(painted.colors).toBeInstanceOf(Uint8Array);
  expect(painted.colors.length).toBe(36);
  expect(core.colors.random(false, rng)).not.toEqual(core.colors.random(false, new Rng(7)));
});

test("a class holds its value, reads its fields and crosses as plain data", () => {
  const tile = new gen.Tile("Fractal");
  expect(tile.group).toBe("Fractal");
  expect(gen.Tile.from(tile.toJSON()).toJSON()).toEqual(tile.toJSON());
  expect(() => tile.check()).toThrow("wrong slot count");
});

test("a class field is set, a default crosses, and the Rng chooses and shuffles as rust does", () => {
  const config = new life.Config(math.two.carpet(3, 1), life.Counts.list([3]), life.Counts.list([2, 3]));
  expect(config.boundary).toBe("Constant");
  config.boundary = "Wrap";
  expect(config.boundary).toBe("Wrap");
  expect(config.toJSON().boundary).toBe("Wrap");
  expect(() => { config.boundary = "Nope"; }).toThrow("unknown variant");
  const tile = gen.build.create_2d(gen.build.Config2d.default(), new Rng(1));
  expect([tile.group, tile.width]).toEqual(["Special", 9]);
  const items = ["a", "b", "c", "d", "e", "f", "g"];
  expect(new Rng(5).choice(items)).toBe("c");
  expect(new Rng(5).choice(items)).toBe(items[new Rng(5).below(items.length)]);
  expect(() => new Rng(1).choice([])).toThrow("a choice wants at least one item.");
  const once = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  const again = [...once];
  new Rng(9).shuffle(once);
  new Rng(9).shuffle(again);
  expect(once).toEqual([2, 8, 4, 7, 9, 0, 6, 1, 3, 5]);
  expect(again).toEqual(once);
});
