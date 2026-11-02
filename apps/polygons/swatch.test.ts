import { expect, test } from "bun:test";
import { css, inks, swatch } from "./swatch.ts";

test("a swatch paints only the hues of its palette", () => {
  const one = inks({} as Element, { paper: "#101010", accent: "#008cff" });
  const fills: unknown[] = [];
  const ctx = new Proxy({} as CanvasRenderingContext2D, {
    get: () => () => {},
    set: (_, key, value: unknown) => (key === "fillStyle" ? fills.push(value) > 0 : true),
  });
  swatch(ctx, 56, 56, one, { shape: "square", palette: "gold", seed: 3, noise: 0.9 });
  expect([...new Set(fills)].sort()).toEqual([css(one.orange), css(one.yellow)].sort());
});

test("on a light paper the tints lean further to the accent and cube shadows keep its hue", () => {
  const one = inks({} as Element, { paper: "#ffffff", accent: "#ff8000" });
  expect([one.tint1, one.shadow].map(css)).toEqual(["rgb(255, 192, 128)", "rgb(102, 51, 0)"]);
});
