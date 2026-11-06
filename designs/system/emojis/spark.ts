import type { Pen } from "@mrlyprod/draw";
import { paint } from "../own/paint.ts";

const ROWS = [
  ".........",
  ".......o.",
  "...y..ooo",
  "...y...o.",
  "..yyy....",
  "yyyyyyy..",
  "..yyy....",
  "...y...o.",
  "...y.....",
];

export default function draw(pen: Pen) {
  paint(pen, ROWS);
}
