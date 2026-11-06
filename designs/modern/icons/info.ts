import type { Color, Pen } from "@mrlyprod/draw";
import { dot, line, W } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  pen.ring(12, 12, 8.6, W, ink);
  dot(pen, ink, 12, 7.9);
  line(pen, ink, [
    [12, 11],
    [12, 16.4],
  ]);
}
