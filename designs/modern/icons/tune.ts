import type { Color, Pen } from "@mrlyprod/draw";
import { line, W } from "../own/icon.ts";

const KNOB = 2.4;

export default function draw(pen: Pen, ink: Color) {
  for (const [y, x] of [
    [5.8, 15.5],
    [12, 8.5],
    [18.2, 13.5],
  ] as const) {
    line(pen, ink, [
      [3.5, y],
      [x - KNOB, y],
    ]);
    line(pen, ink, [
      [x + KNOB, y],
      [20.5, y],
    ]);
    pen.ring(x, y, KNOB, W, ink);
  }
}
