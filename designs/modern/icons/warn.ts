import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, loop } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    round(
      [
        [12, 2.1],
        [22.1, 19.6],
        [1.9, 19.6],
      ],
      2.3,
      true,
    ),
  );
  line(pen, ink, [
    [12, 9.6],
    [12, 13.4],
  ]);
  dot(pen, ink, 12, 16.3);
}
