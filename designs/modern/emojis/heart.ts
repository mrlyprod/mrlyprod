import type { Pen } from "@mrlyprod/draw";
import { BLUSH, DEEP, RED } from "../own/palette.ts";
import { oval } from "../own/path.ts";
import { heart } from "../own/shape.ts";

export default function draw(pen: Pen) {
  pen.polygon(heart(16, 16.4, 28), DEEP);
  pen.polygon(heart(16, 15.8, 27), RED);
  pen.polygon(oval(9.6, 9.4, 3.2, 1.7, -0.7), BLUSH);
}
