import { expect, test } from "bun:test";
import * as font from "./font.js";

const bytes = await Bun.file(new URL("./pkg/font/mrlyjs_font_bg.wasm", import.meta.url)).arrayBuffer();
font.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/font.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);
const sha256 = (text) => new Bun.CryptoHasher("sha256").update(text).digest("hex");

test("font::glyph", () => {
  const r = row("font::glyph");
  expect(font.glyph(r.in.c).rows).toEqual(r.out);
});

test("font::raster::raster", () => {
  const r = row("font::raster::raster");
  expect(font.raster(r.in.text).map((line) => Array.from(line))).toEqual(r.out);
});

test("font::path", () => {
  const r = row("font::path");
  expect(font.path(r.in.c)).toEqual(r.out);
});

test("font::animate::animate", () => {
  const r = row("font::animate::animate");
  const anim = font.animate(r.in.text, r.in.pad);
  expect([anim.rows, anim.cols, anim.fps, anim.frames.length]).toEqual([r.out.rows, r.out.cols, r.out.fps, r.out.frames]);
  expect(anim.frames[anim.frames.length - 1]).toEqual(r.out.last);
});

test("font::map", () => {
  const r = row("font::map");
  const table = font.map();
  const keys = Object.keys(table).sort();
  const json = "{" + keys.map((key) => JSON.stringify(key) + ":" + JSON.stringify(table[key])).join(",") + "}";
  expect(keys.length).toBe(r.out.len);
  expect(sha256(json)).toBe(r.out.sha256);
});
