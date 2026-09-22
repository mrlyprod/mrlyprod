import { expect, test } from "bun:test";
import * as gen from "./gen.js";

const bytes = await Bun.file(new URL("./pkg/gen/mrlyjs_gen_bg.wasm", import.meta.url)).arrayBuffer();
gen.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/gen.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);
const sha256 = (bytes) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
const tileConfig = () => ({ groups: gen.Group.all(), catalog: "Classics", min_size: 3, max_size: 9, parity: "Odds" });

test("gen::name::Tile::recipe", () => {
  const r = row("gen::name::Tile::recipe");
  const name = gen.name.Tile.from_json(r.in.name);
  const tile = name.recipe();
  const recipe = {
    group: tile.group,
    factor: tile.factor,
    sources: tile.sources.map((source) => ({ code: String(source.code) })),
    numbers: Array.from(tile.numbers),
    levels: Array.from(tile.levels),
    rotations: Array.from(tile.rotations),
    invert: tile.invert,
    flip: tile.flip,
    width: tile.width,
    height: tile.height,
  };
  expect(recipe).toEqual(r.out.recipe);
  expect(name.to_json()).toBe(r.out.json);
  expect(name.to_id()).toBe(r.out.id);
});

test("gen::draw::create", () => {
  const r = row("gen::draw::create");
  const tile = gen.build.create_2d(tileConfig(), new gen.Rng(r.in.seed));
  expect(gen.name.Tile.of(tile).to_json()).toBe(r.out);
});

test("gen::build::build_2d", () => {
  const r = row("gen::build::build_2d");
  const cell = gen.build.build_2d(gen.name.Tile.from_json(r.in.tile.in.name).recipe());
  expect(cell.shape).toEqual([r.out.height, r.out.width]);
  expect(Array.from(cell.types)).toEqual(r.out.types.flat());
});

test("gen::variation::create", () => {
  const r = row("gen::variation::create");
  const config = { tile: tileConfig(), paint: {}, files: [[1, 1], [3, 3], [5, 5]] };
  const variation = gen.variation.create(config, new gen.Rng(r.in.seed));
  expect(variation.key).toBe(r.out.key);
  expect(String(variation.seed)).toBe(r.out.seed);
  expect(variation.edition).toBe(r.out.edition);
  expect(gen.name.Tile.of(variation.tile).to_json()).toBe(r.out.tile);
});

test("gen::background", () => {
  const r = row("gen::background");
  expect(sha256(gen.background(r.in.seed, r.in.width, r.in.height))).toBe(r.out);
});
