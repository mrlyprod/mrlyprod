import type { Color, Pen } from "@mrlyprod/draw";
import { dot } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  for (const x of [5.5, 12, 18.5]) dot(pen, ink, x, 12, 1.5);
}
