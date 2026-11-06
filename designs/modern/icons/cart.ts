import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line } from "../own/icon.ts";
import { round, shift } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  line(
    pen,
    ink,
    shift(
      round(
        [
          [2.6, 4],
          [4.4, 4],
          [7, 15.2],
          [18.2, 15.2],
          [20.4, 7.6],
          [5.24, 7.6],
        ],
        [0, 1, 1.5, 1.5, 1, 0],
      ),
      0.5,
      0,
    ),
  );
  dot(pen, ink, 9.1, 19.2, 1.5);
  dot(pen, ink, 17.5, 19.2, 1.5);
}
