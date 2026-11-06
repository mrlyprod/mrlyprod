import type { Color, Pen } from "@mrlyprod/draw";
import { STAR } from "../own/shapes.ts";
import { stamp } from "../own/stamp.ts";

export default function draw(pen: Pen, ink: Color) {
  stamp(pen, STAR, ink);
}
