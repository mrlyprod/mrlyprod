import type { Pen } from "@mrlyprod/draw";
import { FACE } from "../own/face.ts";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  "................",
  "................",
  "................",
  ".kkkkkkkkkkkkkk.",
  "..kwkkk..kwkkk..",
  "...kkk....kkk...",
  "................",
  "................",
  "....k......k....",
  ".....kkkkkk.....",
  "................",
  "................",
  "................",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, FACE);
  stamp(pen, ROWS);
}
