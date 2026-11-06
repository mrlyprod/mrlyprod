import type { Pen } from "@mrlyprod/draw";
import { paint } from "../own/paint.ts";

const ROWS = [
  ".rrr.rrr.",
  "rwrrrrrrr",
  "rwrrrrrrr",
  "rrrrrrrrr",
  "rrrrrrrrr",
  ".rrrrrrr.",
  "..rrrrr..",
  "...rrr...",
  "....r....",
];

export default function draw(pen: Pen) {
  paint(pen, ROWS);
}
