import type { Color, Pen } from "@mrlyprod/draw";
import { line } from "../own/icon.ts";
import { oval } from "../own/path.ts";
import { curve } from "../own/shape.ts";

export default function draw(pen: Pen, ink: Color) {
  pen.polygon(oval(8.9, 17.9, 3.5, 2.6, -0.35), ink);
  line(pen, ink, [
    [12.15, 17.3],
    [12.15, 4.3],
  ]);
  line(
    pen,
    ink,
    curve(
      [12.15, 4.3],
      [
        [17.6, 4.6],
        [18, 8.6],
      ],
    ),
  );
}
