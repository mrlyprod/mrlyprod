import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, loop } from "../own/icon.ts";
import { box } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, box(2.9, 5, 18.2, 14, 2.6));
  for (const y of [8.7, 12.3]) for (const x of [6.5, 10.17, 13.83, 17.5]) dot(pen, ink, x, y);
  line(pen, ink, [
    [8.8, 15.9],
    [15.2, 15.9],
  ]);
}
