import { expect, test } from "bun:test";
import * as tile from "./tile.js";

const bytes = await Bun.file(new URL("./pkg/tile/mrly_js_tile_bg.wasm", import.meta.url)).arrayBuffer();
tile.initSync({ module: bytes });
const rows = await Bun.file(new URL("../../crates/mrly/fixtures/tile.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);
const sha256 = (bytes) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
const pixels = (image) => ({ shape: image.shape, sha256: sha256(image.colors) });
const tileConfig = () => ({ groups: tile.Group.all(), catalog: "Classics", min_size: 3, max_size: 9, parity: "Odds" });

test("tile::name::Tile::recipe", () => {
  const r = row("tile::name::Tile::recipe");
  const name = tile.name.Tile.from_json(r.in.name);
  const made = name.recipe();
  const recipe = {
    group: made.group,
    factor: made.factor,
    sources: made.sources.map((source) => ({ code: String(source.code) })),
    numbers: Array.from(made.numbers),
    levels: Array.from(made.levels),
    rotations: Array.from(made.rotations),
    invert: made.invert,
    flip: made.flip,
    width: made.width,
    height: made.height,
  };
  expect(recipe).toEqual(r.out.recipe);
  expect(name.to_json()).toBe(r.out.json);
  expect(name.to_id()).toBe(r.out.id);
});

test("tile::draw::create", () => {
  const r = row("tile::draw::create");
  const made = tile.build.create_2d(tileConfig(), new tile.Rng(r.in.seed));
  expect(tile.name.Tile.of(made).to_json()).toBe(r.out);
});

test("tile::build::build_2d", () => {
  const r = row("tile::build::build_2d");
  const cell = tile.build.build_2d(tile.name.Tile.from_json(r.in.tile.in.name).recipe());
  expect(cell.shape).toEqual([r.out.height, r.out.width]);
  expect(Array.from(cell.types)).toEqual(r.out.types.flat());
});

test("tile::variation::create", () => {
  const r = row("tile::variation::create");
  const config = { tile: tileConfig(), paint: {}, files: [[1, 1], [3, 3], [5, 5]] };
  const variation = tile.variation.create(config, new tile.Rng(r.in.seed));
  expect(variation.key).toBe(r.out.key);
  expect(String(variation.seed)).toBe(r.out.seed);
  expect(variation.edition).toBe(r.out.edition);
  expect(tile.name.Tile.of(variation.tile).to_json()).toBe(r.out.tile);
});

test("tile::background", () => {
  const r = row("tile::background");
  expect(pixels(tile.background(r.in.seed, r.in.width, r.in.height))).toEqual(r.out);
});
