import { expect, test } from "bun:test";
import { world } from "./worlds.js";

test("a world is a pure function of its name and seed", () => {
  expect(world("exo", 9)).toEqual(world("exo", 9));
  expect(world("earth", 4)).toEqual(world("earth", 4));
  expect(world("exo", 9)).not.toEqual(world("exo", 10));
});

test("exo rolls terran, desert, ice, lava and gas worlds", () => {
  const kinds = new Set(Array.from({ length: 200 }, (_, seed) => world("exo", seed).kind));
  expect([...kinds].sort()).toEqual(["desert", "gas", "ice", "lava", "terran"]);
});

test("exo gas worlds draw from three muted palettes", () => {
  const gas = Array.from({ length: 300 }, (_, seed) => world("exo", seed)).filter((w) => w.kind === "gas");
  const hsv = (rgb) => {
    const c = rgb.map((v) => v ** (1 / 2.2));
    return (Math.max(...c) - Math.min(...c)) / Math.max(...c);
  };
  expect(new Set(gas.map((w) => w.tone.join())).size).toBe(3);
  expect(Math.max(...gas.flatMap((w) => w.bands.map(([, ...rgb]) => hsv(rgb))))).toBeLessThan(0.6);
});

test("the moon has warm highlands and darker blue-grey maria", () => {
  const [high, sea] = world("moon").colors;
  expect(high[0]).toBeGreaterThan(high[2]);
  expect(sea[2]).toBeGreaterThan(sea[0]);
  expect(sea[1]).toBeLessThan(high[1] / 2);
});
