import type { Color, Pen } from "@mrlyprod/draw";
import { line, loop } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    round(
      [
        [5.2, 3.4],
        [13.4, 3.4],
        [18.8, 8.8],
        [18.8, 20.6],
        [5.2, 20.6],
      ],
      [2.6, 0.6, 0.6, 2.6, 2.6],
      true,
    ),
  );
  line(
    pen,
    ink,
    round(
      [
        [13.4, 3.4],
        [13.4, 8.8],
        [18.8, 8.8],
      ],
      1.4,
    ),
  );
}
