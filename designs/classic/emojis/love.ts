import type { Pen } from "@mrlyprod/draw";
import { FACE } from "../own/face.ts";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  "................",
  "................",
  "................",
  "................",
  "..rr.rr..rr.rr..",
  "..rrrrr..rrrrr..",
  "...rrr....rrr...",
  "....r......r....",
  "................",
  "....kkkkkkkk....",
  ".....krrrrk.....",
  "......kkkk......",
  "................",
  "................",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, FACE);
  stamp(pen, ROWS);
}
