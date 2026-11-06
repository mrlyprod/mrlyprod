import type { Pen } from "@mrlyprod/draw";
import { FACE, paint } from "../own/paint.ts";

const ROWS = [
  ".........",
  ".........",
  "......k..",
  ".kkk..k..",
  ".........",
  "..k...k..",
  "...kkk...",
  ".........",
  ".........",
];

export default function draw(pen: Pen) {
  paint(pen, FACE, ROWS);
}
