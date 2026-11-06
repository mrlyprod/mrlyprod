import type { Color, Pen } from "@mrlyprod/draw";
import { loop } from "../own/icon.ts";
import { round } from "../own/path.ts";

export default function draw(pen: Pen, ink: Color) {
  loop(
    pen,
    ink,
    round(
      [
        [12, 3.35],
        [20.6, 12.15],
        [16, 12.15],
        [16, 19.95],
        [8, 19.95],
        [8, 12.15],
        [3.4, 12.15],
      ],
      [1.6, 1.2, 0.6, 1.2, 1.2, 0.6, 1.2],
      true,
    ),
  );
}
