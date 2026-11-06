import type { Pen } from "@mrlyprod/draw";
import { paint } from "../own/paint.ts";

const ROWS = [
  "....g..b.",
  "......i..",
  "..y..r..y",
  "..oy...u.",
  ".rooy.b..",
  ".orooy..g",
  ".oorooy..",
  "oooo.....",
  "o........",
];

export default function draw(pen: Pen) {
  paint(pen, ROWS);
}
