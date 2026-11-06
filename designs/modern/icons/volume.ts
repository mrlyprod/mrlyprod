import type { Color, Pen } from "@mrlyprod/draw";
import { loop, W } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    round(
      [
        [4.2, 9.2],
        [8, 9.2],
        [12.2, 5],
        [12.2, 19],
        [8, 14.8],
        [4.2, 14.8],
      ],
      [1, 0.6, 1.2, 1.2, 0.6, 1],
      true,
    ),
  );
  pen.arc([12.2, 12], 4, [-0.8, 0.8], W, ink);
  pen.arc([12.2, 12], 7.7, [-0.85, 0.85], W, ink);
}
