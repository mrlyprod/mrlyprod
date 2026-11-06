import type { Color, Pen } from "@mrlyprod/draw";
import { CHEVRON } from "../own/shapes.ts";
import { drop, stamp, turn } from "../own/stamp.ts";

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, drop(turn(CHEVRON)), ink);
}
