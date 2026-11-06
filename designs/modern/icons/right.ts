import type { Color, Pen } from "@mrlyprod/draw";
import { chevron } from "../own/chevron.ts";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, chevron(2));
}
