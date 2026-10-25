import { expect, test } from "bun:test";
import { apart } from "../../lib/fractal/orbit.ts";
import type { View } from "../../lib/fractal/view.ts";
import { parse, spanOf } from "../../lib/fractal/view.ts";
import { pinch, point, scale } from "./widget.tsx";

const V: View = { centre: [parse("-0.7463")!, parse("0.1102")!], span: spanOf(8), angle: 0 };

test("a pointer reads in short sides from the centre of the canvas, up positive", () => {
  expect(point({ left: 100, top: 50, width: 800, height: 400 }, { clientX: 700, clientY: 50 })).toEqual([0.5, 0.5]);
});

test("two fingers spread twice as far about a point zoom in twice there, and the zoom stops at the cap", () => {
  const two = pinch(
    V,
    [
      [0.1, 0.2],
      [0.3, 0.2],
    ],
    [
      [0, 0.2],
      [0.4, 0.2],
    ],
    [spanOf(20), spanOf(-1.5)],
  );
  const there = scale(V, [0.2, 0.2], 2, [spanOf(20), spanOf(-1.5)]);
  expect([+(V.span / two.span).toFixed(12), Math.hypot(...apart(two.centre, there.centre)) < 1e-15]).toEqual([2, true]);
  expect(scale(V, [0, 0], 2 ** 30, [spanOf(20), spanOf(-1.5)]).span).toBe(spanOf(20));
});
