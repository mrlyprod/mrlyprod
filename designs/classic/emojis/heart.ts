import type { Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  "..kkkk....kkkk..",
  ".krrrrk..krrrrk.",
  ".krwrrk..krrrrk.",
  ".krrrrrkkrrrrrk.",
  ".krrrrrrrrrrrrk.",
  ".krrrrrrrrrrrrk.",
  ".krrrrrrrrrrrrk.",
  "..krrrrrrrrrrk..",
  "...krrrrrrrrk...",
  "....krrrrrrk....",
  ".....krrrrk.....",
  "......krrk......",
  ".......kk.......",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, ROWS);
}
