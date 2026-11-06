import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, loop } from "../own/icon.ts";
import { box } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, box(3.9, 5.2, 16.2, 15.2, 2.8));
  line(pen, ink, [
    [3.9, 9.8],
    [20.1, 9.8],
  ]);
  for (const x of [8, 16])
    line(pen, ink, [
      [x, 3.4],
      [x, 6.6],
    ]);
  for (const [x, y] of [
    [8, 13.4],
    [12, 13.4],
    [16, 13.4],
    [8, 16.9],
    [12, 16.9],
  ] as const)
    dot(pen, ink, x, y);
}
