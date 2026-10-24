import { expect, test } from "bun:test";
import { rng } from "../../lib/rand.js";
import { bounce, make, next, TILES } from "./scene.js";

const stage = (w, h, dpr, opts) => {
  const rects = [];
  const ctx = new Proxy(
    {},
    { get: (_, key) => (key === "rect" ? (...args) => rects.push(args) : () => {}), set: () => true },
  );
  const view = { rand: rng(3), look: () => ({ accent: "#000" }), still: false, w, h, dpr, t: 0 };
  const scene = make({ getContext: () => ctx }, view, opts);
  return { rects, view, scene };
};

test("the sleeper reflects and flips at both walls", () => {
  expect(bounce(-2, -4, 100)).toEqual({ p: 2, v: 4, hit: true });
  expect(bounce(104, 4, 100)).toEqual({ p: 96, v: -4, hit: true });
  expect(bounce(50, 4, 100)).toEqual({ p: 50, v: 4, hit: false });
});

test("the crowd covers the same ground in a second at 60 Hz and at 120 Hz, on whole cells", () => {
  const corner = (hz) => {
    const { rects, view, scene } = stage(4000, 3000, 2, { speed: 240, size: 0.1, count: 3 });
    for (let n = 1; n <= hz; n++) {
      view.t = (n * 1000) / hz;
      rects.length = 0;
      scene.draw();
    }
    return { at: rects[0].slice(0, 2), whole: rects.every((one) => one.every(Number.isInteger)) };
  };
  const slow = corner(60);
  expect(corner(120)).toEqual(slow);
  expect(slow.whole).toBe(true);
});

test("at app size the crowd of tiles fills a good share of the canvas", () => {
  const { rects, scene } = stage(1200, 800, 1, { seed: 1 });
  scene.draw();
  const share = rects.reduce((sum, [, , w, h]) => sum + w * h, 0) / (1200 * 800);
  expect(share).toBeGreaterThan(0.2);
});

test("stepping walks the tiles from Mixed and wraps at both ends", () => {
  const last = TILES.length - 1;
  expect([
    next({ design: "", number: "" }, 1),
    next({ design: "", number: "" }, -1),
    next({ design: TILES[last][0], number: TILES[last][1] }, 1),
  ]).toEqual([
    { design: TILES[1][0], number: TILES[1][1] },
    { design: TILES[last][0], number: TILES[last][1] },
    { design: "", number: "" },
  ]);
});
