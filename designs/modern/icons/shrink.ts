import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  line(pen, ink, [
    [19.5, 4.5],
    [13.5, 10.5],
  ]);
  line(pen, ink, [
    [13.5, 6],
    [13.5, 10.5],
    [18, 10.5],
  ]);
  line(pen, ink, [
    [4.5, 19.5],
    [10.5, 13.5],
  ]);
  line(pen, ink, [
    [6, 13.5],
    [10.5, 13.5],
    [10.5, 18],
  ]);
}
