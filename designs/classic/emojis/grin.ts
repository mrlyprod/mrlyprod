import type { Pen } from "@mrlyprod/draw";
import { FACE } from "../own/face.ts";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  "................",
  "................",
  ".....k....k.....",
  ".....k....k.....",
  ".....k....k.....",
  "................",
  "....kkkkkkkk....",
  "....kwwwwwwk....",
  ".....krrrrk.....",
  "......kkkk......",
  "................",
  "................",
  "................",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, FACE);
  stamp(pen, ROWS);
}
