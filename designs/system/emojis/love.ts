import type { Pen } from "@mrlyprod/draw";
import { FACE, paint } from "../own/paint.ts";

const ROWS = [
  ".........",
  ".........",
  ".r.r.r.r.",
  ".rrr.rrr.",
  "..r...r..",
  ".........",
  "..k...k..",
  "...kkk...",
  ".........",
];

export default function draw(pen: Pen) {
  paint(pen, FACE, ROWS);
}
