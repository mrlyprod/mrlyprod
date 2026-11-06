import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [
    [4.75, 12.75],
    [9.75, 17.5],
    [19.25, 6.5],
  ]);
}
