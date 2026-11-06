import type { Pen } from "@mrlyprod/draw";
import { FACE, paint } from "../own/paint.ts";

const ROWS = [
  ".........",
  ".........",
  "..k...k..",
  "..k...k..",
  ".c.......",
  ".c.......",
  "...kkk...",
  "..k...k..",
  ".........",
];

export default function draw(pen: Pen) {
  paint(pen, FACE, ROWS);
}
