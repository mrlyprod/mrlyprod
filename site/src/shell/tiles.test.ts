import { expect, test } from "bun:test";
import type { View } from "../lib/app/run.ts";
import { rng } from "../lib/rand.ts";
import { make, tile } from "./tiles.ts";

const mirror = (cells: number[]) => cells.map((_, i) => cells[Math.floor(i / 5) * 5 + 4 - (i % 5)]);

const flip = (cells: number[]) => cells.map((_, i) => cells[(4 - Math.floor(i / 5)) * 5 + (i % 5)]);

const turn = (cells: number[]) => cells.map((_, i) => cells[(i % 5) * 5 + 4 - Math.floor(i / 5)]);

const good = (cells: number[]) => {
  const same = (other: number[]) => other.every((on, i) => on === cells[i]);
  const filled = cells.filter(Boolean).length;
  const rows = Array.from({ length: 5 }, (_, r) => cells.slice(r * 5, r * 5 + 5).join(""));
  const cols = Array.from({ length: 5 }, (_, c) => cells.filter((_, i) => i % 5 === c).join(""));
  const edges = [rows[0], rows[4], cols[0], cols[4]].every((line) => line.includes("1"));
  const binary = cells.length === 25 && cells.every((on) => on === 0 || on === 1);
  const stripes = new Set(rows).size === 1 || new Set(cols).size === 1;
  return (
    binary &&
    (same(mirror(cells)) || same(flip(cells)) || same(turn(cells))) &&
    filled >= 9 &&
    filled <= 16 &&
    edges &&
    !stripes
  );
};

test("a seed fixes the tile and the next seed almost always rolls another", () => {
  const seeds = Array.from({ length: 50 }, (_, s) => s);
  expect(tile(rng(7))).toEqual(tile(rng(7)));
  expect(seeds.filter((s) => tile(rng(s)).join("") !== tile(rng(s + 1)).join("")).length).toBeGreaterThan(45);
});

test("every tile is symmetric, between a third and two thirds full, reaches all four edges and is no plain stripe", () => {
  const bad = Array.from({ length: 2000 }, (_, s) => s).filter((s) => !good(tile(rng(s))));
  expect(bad).toEqual([]);
});

test("the saver paints its ground, then whole-pixel cells, on a fake canvas", () => {
  const log: [PropertyKey, ...unknown[]][] = [];
  const pen = new Proxy(
    {},
    {
      get:
        (_, key) =>
        (...args: unknown[]) =>
          log.push([key, ...args]),
      set: () => true,
    },
  );
  const view = {
    rand: rng(3),
    look: () => ({ paper: "#ffffff", accent: "#5856d6", ink: "#000000" }),
    still: true,
    w: 1600,
    h: 1000,
    dpr: 2,
    t: 0,
  } as View;
  const scene = make(Object.assign({} as HTMLCanvasElement, { getContext: () => pen }), view);
  scene.draw();
  const rects = log.filter(([key]) => key === "rect").map(([, ...args]) => args);
  expect(scene.still).toBe(true);
  expect(log.find(([key]) => key === "fillRect")).toEqual(["fillRect", 0, 0, 1600, 1000]);
  expect(rects.length).toBeGreaterThan(0);
  expect(rects.flat().every((n) => Number.isInteger(n))).toBe(true);
});
