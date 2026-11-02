import { expect, test } from "bun:test";
import type { World } from "./exo.ts";
import { globe } from "./globe.ts";
import { NAMES, world } from "./worlds.ts";

type Entry = [string, ...unknown[]];

const paint = (params: World, side = 56) => {
  const log: Entry[] = [];
  const stop = { addColorStop: (...args: unknown[]) => log.push(["stop", ...args]) };
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      String(key).startsWith("create") ? () => stop : (...args: unknown[]) => log.push([String(key), ...args]),
    set: (_, key, value) => {
      log.push([String(key), value]);
      return true;
    },
  });
  globe(ctx, side, side, params);
  return log;
};

test("every world paints a disc inside its square, and only a ringed world strokes an ellipse", () => {
  for (const name of NAMES) {
    const params = world(name, 3);
    const log = paint(params);
    const disc = log.find(([key]) => key === "arc")!;
    expect([
      name,
      Number(disc[1]) - Number(disc[3]) >= 0,
      Number(disc[1]) + Number(disc[3]) <= 56,
      log.some(([key]) => key === "ellipse"),
    ]).toEqual([name, true, true, Boolean(params.ring)]);
  }
});

test("a gas giant fills with one stop per band, and a rocky world with its sea first", () => {
  const gas = world("jupiter", 0);
  expect(paint(gas).filter(([key]) => key === "stop").length).toBe(gas.bands!.length + 3);
  const earth = world("earth", 0);
  const fills = paint(earth).filter(([key]) => key === "fillStyle");
  expect(fills[0][1]).toMatch(/^rgba\(\d+,\d+,\d+,1\)$/);
  const first = String(fills[0][1]);
  expect(Number(first.match(/\d+/g)![2])).toBeGreaterThan(Number(first.match(/\d+/g)![0]));
});
