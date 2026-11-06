import type { Color, Pen } from "@mrlyprod/draw";
import { line, W } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  pen.ring(10.6, 10.6, 6.6, W, ink);
  line(pen, ink, [
    [15.3, 15.3],
    [20, 20],
  ]);
}
