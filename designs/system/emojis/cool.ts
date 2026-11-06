import type { Pen } from "@mrlyprod/draw";
import { FACE, paint } from "../own/paint.ts";

const ROWS = [
  ".........",
  ".........",
  "kkkkkkkkk",
  ".kkk.kkk.",
  ".kkk.kkk.",
  ".........",
  "..k...k..",
  "...kkk...",
  ".........",
];

export default function draw(pen: Pen) {
  paint(pen, FACE, ROWS);
}
