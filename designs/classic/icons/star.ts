import type { Color, Pen } from "@mrlyprod/draw";
import { bits } from "@mrlyprod/ui/kit/glyph.ts";
import { STAR } from "../own/shapes.ts";

export default function draw(pen: Pen, ink: Color) {
  bits(pen, STAR, { "#": ink });
}
