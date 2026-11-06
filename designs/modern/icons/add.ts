import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [
    [12, 4.75],
    [12, 19.25],
  ]);
  line(pen, ink, [
    [4.75, 12],
    [19.25, 12],
  ]);
}
