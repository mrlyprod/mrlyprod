import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";

export default function draw(pen: Pen, ink: Color) {
  for (const [x, top] of [
    [4.5, 11.5],
    [9.5, 4],
    [14.5, 8.5],
    [19.5, 13.5],
  ] as const)
    line(pen, ink, [
      [x, top],
      [x, 20],
    ]);
}
