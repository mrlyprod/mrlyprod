import type { Pen } from "@mrlyprod/draw";
import { paint } from "../own/paint.ts";

const ROWS = [
  "....r....",
  "...rr....",
  "...rrr.r.",
  "..rrrrrrr",
  ".rrrorrrr",
  ".rrooorrr",
  ".rooyoor.",
  ".royyyor.",
  "..rrrrr..",
];

export default function draw(pen: Pen) {
  paint(pen, ROWS);
}
