import type { Pen } from "@mrlyprod/draw";
import { stamp } from "../own/stamp.ts";

const ROWS = [
  "................",
  ".......kk.......",
  "......kyyk......",
  "......kyyk......",
  ".....kyyyk......",
  ".....kyyk.......",
  ".kkkkyyyykkkkk..",
  ".kbbkyyyyyyyyyk.",
  ".kbbkyyyyykkkkk.",
  ".kbbkyyyyyyyyk..",
  ".kbbkyyyyykkkk..",
  ".kbbkyyyyyyyyk..",
  ".kbbkyyyyykkk...",
  ".kbbkyyyyyyyk...",
  ".kkkkkkkkkkk....",
  "................",
];

export default function draw(pen: Pen) {
  stamp(pen, ROWS);
}
