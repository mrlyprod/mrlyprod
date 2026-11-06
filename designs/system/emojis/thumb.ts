import type { Pen } from "@mrlyprod/draw";
import { paint } from "../own/paint.ts";

const ROWS = [
  "....yy...",
  "...yyy...",
  "...yyy...",
  "bbyyyyyy.",
  "bbyyyyyyy",
  "bbyoooooy",
  "bbyyyyyyy",
  "bbyoooooy",
  ".byyyyyy.",
];

export default function draw(pen: Pen) {
  paint(pen, ROWS);
}
