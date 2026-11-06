import type { Color, Pen } from "@mrlyprod/draw";
import { line, loop } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    round(
      [
        [2.9, 4.6],
        [9.6, 4.6],
        [11.8, 7.1],
        [21.1, 7.1],
        [21.1, 19.4],
        [2.9, 19.4],
      ],
      [2.2, 1, 1, 2.2, 2.6, 2.6],
      true,
    ),
  );
  line(pen, ink, [
    [2.9, 10],
    [21.1, 10],
  ]);
}
