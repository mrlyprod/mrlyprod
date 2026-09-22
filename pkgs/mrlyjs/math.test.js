import { expect, test } from "bun:test";
import * as math from "./math.js";

const bytes = await Bun.file(new URL("./pkg/math/mrlyjs_math_bg.wasm", import.meta.url)).arrayBuffer();
math.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/math.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);

test("math::atoms::carpet_2d", () => {
  const r = row("math::atoms::carpet_2d");
  const seed = math.atoms.carpet_2d(r.in.n);
  expect(seed.shape).toEqual(r.out.shape);
  expect(Array.from(seed.data)).toEqual(r.out.data);
});

test("math::two::carpet", () => {
  const r = row("math::two::carpet");
  const cell = math.two.carpet(r.in.number, r.in.level);
  expect(JSON.parse(math.two.to_json(cell))).toEqual(r.out);
});

test("math::two::census::census", () => {
  const r = row("math::two::census::census");
  const census = math.two.census(math.two.carpet(r.in.cell.in.number, r.in.cell.in.level));
  expect({ ...census, perimeter: Number(census.perimeter) }).toEqual(r.out);
});

test("math::counts::fill", () => {
  const r = row("math::counts::fill");
  expect(math.counts.fill(r.in.code, r.in.number, r.in.dimension, r.in.level, r.in.base)).toBe(r.out);
});

test("math::bang::bang", () => {
  const r = row("math::bang::bang");
  expect(math.bang.bang(r.in.dimension).distinct()).toBe(r.out);
});

test("math::three::census::census", () => {
  const r = row("math::three::census::census");
  const census = math.three.census(math.three.carpet(r.in.cell.in.number, r.in.cell.in.level));
  expect(String(census.surface)).toBe(r.out);
});

test("math::spectrum::laplacian_spectrum", () => {
  const r = row("math::spectrum::laplacian_spectrum");
  const network = new math.graph.Network(r.in.network.dim);
  for (const position of r.in.network.nodes) network.add_node(position);
  for (const [parent, child, radius] of r.in.network.branches) network.add_branch(parent, child, radius);
  const spectrum = Array.from(math.spectrum.laplacian_spectrum(network, r.in.normalised));
  expect(spectrum.length).toBe(r.out.length);
  spectrum.forEach((value, i) => expect(value).toBeCloseTo(r.out[i], 12));
});
