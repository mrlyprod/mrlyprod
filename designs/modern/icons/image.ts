import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, loop } from "../own/icon.ts";
import { box, round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(pen, ink, box(2.9, 4.6, 18.2, 14.8, 2.8));
  dot(pen, ink, 7.6, 9, 1.6);
  line(
    pen,
    ink,
    round(
      [
        [2.9, 16.2],
        [10.4, 11.6],
        [14.4, 15.2],
        [16.8, 13.4],
        [21.1, 16.2],
      ],
      1.2,
    ),
  );
}
