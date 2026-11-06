import type { Color, Pen } from "@mrlyprod/draw";
import { line, W } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  pen.ring(12, 12, 8.6, W, ink);
  line(pen, ink, [
    [12, 7.2],
    [12, 12],
    [15.4, 14.2],
  ]);
}
