import { expect, test } from "bun:test";
import * as core from "./core.js";

const bytes = await Bun.file(new URL("./pkg/core/mrlyjs_core_bg.wasm", import.meta.url)).arrayBuffer();
core.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/core.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);
const sha256 = (bytes) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
const pixels = (image) => ({ shape: image.shape, sha256: sha256(image.colors) });

test("core::Tensor::rot90", () => {
  const r = row("core::Tensor::rot90");
  const turned = core.tensor.rot90({ shape: r.in.shape, data: Uint8Array.from(r.in.data) }, r.in.k, r.in.axes);
  expect(turned.shape).toEqual(r.out.shape);
  expect(Array.from(turned.data)).toEqual(r.out.data);
});

test("core::Color::from_hex", () => {
  const r = row("core::Color::from_hex");
  const color = core.colors.from_hex(r.in.hex);
  expect(color).toEqual(r.out.rgba);
  expect(core.colors.to_hex(color)).toBe(r.out.hex);
});

test("core::Image::scale", () => {
  const r = row("core::Image::scale");
  const corners = { shape: [r.in.height, r.in.width], colors: Uint8Array.from(r.in.colors.flat()) };
  expect(pixels(core.image.scale(corners, r.in.scale))).toEqual(r.out);
});

test("core::Colorizer::color", () => {
  const r = row("core::Colorizer::color");
  const ramp = r.in.ramp.map(core.colors.from_hex);
  const colorizer = core.Colorizer.gradient_bins(core.colors.from_hex(r.in.background), ramp, r.in.shades);
  const hexes = r.in.values.map((value) => core.colors.to_hex(core.ramp.color(colorizer, value, r.in.max)));
  expect(hexes).toEqual(r.out);
});
