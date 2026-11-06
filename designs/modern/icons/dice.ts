import type { Color, Pen } from "@mrlyprod/draw";
import { dot, loop } from "../own/icon.ts";
import { box } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, box(3.9, 3.9, 16.2, 16.2, 3.6));
  for (const [x, y] of [
    [8.2, 8.2],
    [15.8, 8.2],
    [12, 12],
    [8.2, 15.8],
    [15.8, 15.8],
  ] as const)
    dot(pen, ink, x, y, 1.4);
}
