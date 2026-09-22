import { expect, test } from "bun:test";
import * as life from "./life.js";

const bytes = await Bun.file(new URL("./pkg/life/mrlyjs_life_bg.wasm", import.meta.url)).arrayBuffer();
life.initSync({ module: bytes });
const rows = await Bun.file(new URL("../mrlyrs/fixtures/life.json", import.meta.url)).json();
const row = (fn) => rows.find((r) => r.fn === fn);
const grid = (seed) => ({ shape: seed.shape, types: Uint8Array.from(seed.data) });
const carpet = grid({ shape: [4, 4], data: [1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0] });
const fates = { Dead: "dead", Alive: "alive", Loop: "loop", Timeout: "timeout" };

function run(r) {
  const config = new life.Config(life.moore(), life.Counts.list(r.in.config.birth), life.Counts.list(r.in.config.survive));
  expect(config.boundary).toBe(r.in.config.boundary);
  expect(config.max_generations).toBe(r.in.config.max_generations);
  expect(config.grid_size).toBe(r.in.config.grid_size);
  expect(config.padding).toBe(r.in.config.padding);
  return life.animate(grid(r.in.seed), config);
}

test("life::history", () => {
  const r = row("life::history");
  const diagram = life.history(r.in.row, r.in.rule, r.in.steps, r.in.wrap);
  expect(diagram.shape).toEqual(r.out.shape);
  expect(Array.from(diagram.data)).toEqual(r.out.data);
});

test("life::animate::animate", () => {
  const r = row("life::animate::animate");
  const played = run(r);
  expect(fates[played.fate]).toBe(r.out.fate);
  expect(played.count).toBe(r.out.count);
  expect(played.loop_length).toBe(r.out.loop_length);
  const last = played.last();
  expect(last.shape).toEqual([r.out.last.height, r.out.last.width]);
  expect(Array.from(last.types)).toEqual(r.out.last.types.flat());
});

test("life::entropy", () => {
  const r = row("life::entropy");
  expect(Number(life.entropy(carpet))).toBe(r.out);
});

test("life::churn", () => {
  const r = row("life::churn");
  const zeros = grid({ shape: r.in.grids[0].zeros.shape, data: new Array(16).fill(0) });
  expect(life.churn([zeros, carpet])).toBeCloseTo(r.out, 12);
});

test("life::counts", () => {
  const r = row("life::counts");
  const counts = life.counts(life.Source.from(r.in.seq), r.in.max_neighbors, r.in.include_zeros, r.in.include_ones);
  expect(Array.from(counts)).toEqual(r.out);
});

test("life::heatmap", () => {
  const r = row("life::heatmap");
  const frames = life.heatmap(run(r.in.grids).grids, r.in.scale);
  expect(frames.length).toBe(r.out.length);
  frames.forEach((frame, i) => expect(Array.from(frame)).toEqual(r.out[i]));
});
