import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, loop } from "../own/icon.ts";
import { arc, box } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [[8.1, 10.25], ...arc(12, 7.85, 3.9, Math.PI, 2 * Math.PI), [15.9, 10.25]]);
  loop(pen, ink, box(5, 10.25, 14, 9.8, 2.6));
  dot(pen, ink, 12, 15.15);
}
