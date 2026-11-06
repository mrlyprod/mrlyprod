import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "./icon.ts";
import { round } from "./path.ts";

export function tray(pen: Pen, ink: Color) {
  line(
    pen,
    ink,
    round(
      [
        [8.4, 9.8],
        [5.6, 9.8],
        [5.6, 20.6],
        [18.4, 20.6],
        [18.4, 9.8],
        [15.6, 9.8],
      ],
      2.6,
    ),
  );
}
