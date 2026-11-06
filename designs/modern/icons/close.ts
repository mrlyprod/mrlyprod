import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [
    [6.25, 6.25],
    [17.75, 17.75],
  ]);
  line(pen, ink, [
    [17.75, 6.25],
    [6.25, 17.75],
  ]);
}
