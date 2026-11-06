import type { Color, Pen, Point } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { flip } from "../own/path.ts";

const ARM: Point[] = [
  [8, 7.2],
  [3.2, 12],
  [8, 16.8],
];

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, ARM);
  line(pen, ink, flip(ARM));
  line(pen, ink, [
    [13.7, 5.4],
    [10.3, 18.6],
  ]);
}
