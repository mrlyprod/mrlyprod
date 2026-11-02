import { expect, test } from "bun:test";
import type { Pair } from "./orbit.ts";
import { prefer, score } from "./probe.ts";

const probe = (paint: (x: number, y: number) => [boolean, number, number]) => {
  const out = new Uint8Array(64 * 64 * 4);
  for (let y = 0; y < 64; y++) {
    for (let x = 0; x < 64; x++) {
      const [inside, count, per] = paint(x, y);
      const q = inside ? 0 : Math.round(count * 8);
      out.set([q >> 8, q & 255, (inside ? 128 : 0) + (per >> 8), per & 255], 4 * (y * 64 + x));
    }
  }
  return out;
};

test("score rejects an all-interior and a flat probe and ranks a boundary tile first", () => {
  expect(
    score(
      probe(() => [true, 0, 3]),
      64,
      64,
    ),
  ).toEqual([]);
  expect(
    score(
      probe(() => [false, 10, 1]),
      64,
      64,
    ),
  ).toEqual([]);
  const blob = probe((x, y) => {
    const r = Math.hypot(x - 43.5, y - 19.5);
    return r < 2 ? [true, 0, 5] : [false, 10 + 40 / (1 + (r - 2) ** 2), 5];
  });
  const [first] = score(blob, 64, 64);
  expect([first.tx, first.ty, first.kind]).toEqual([5, 2, "island"]);
});

test("the turn penalty prefers the tile straight ahead among equal tiles", () => {
  const twins = probe((x, y) => {
    const r = Math.min(Math.hypot(x - 25.5, y - 31.5), Math.hypot(x - 38.5, y - 31.5));
    return r < 1.5 ? [true, 0, 5] : [false, 10 + 40 / (1 + (r - 1.5) ** 2), 5];
  });
  const list = score(twins, 64, 64);
  const offset = (x: number, y: number): Pair => [x / 64 - 0.5, y / 64 - 0.5];
  const way = (heading: Pair) => Math.sign(prefer(twins, 64, 64, list, offset, heading, 0.3)![0]);
  expect([way([1, 0]), way([-1, 0])]).toEqual([1, -1]);
});
