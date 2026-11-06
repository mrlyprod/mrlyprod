import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [
    [3.6, 6.2],
    [20.4, 6.2],
  ]);
  line(
    pen,
    ink,
    round(
      [
        [9, 6.2],
        [9, 3.4],
        [15, 3.4],
        [15, 6.2],
      ],
      1.4,
    ),
  );
  line(
    pen,
    ink,
    round(
      [
        [5.6, 6.2],
        [6.8, 20.6],
        [17.2, 20.6],
        [18.4, 6.2],
      ],
      2.2,
    ),
  );
  for (const x of [10, 14])
    line(pen, ink, [
      [x, 10],
      [x, 17],
    ]);
}
