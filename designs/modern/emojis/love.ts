import type { Pen } from "@mrlyprod/draw";
import { face, mouth } from "../own/face.ts";
import { DEEP, RED } from "../own/palette.ts";
import { heart } from "../own/shape.ts";

export default function draw(pen: Pen) {
  face(pen);
  for (const x of [10.2, 21.8]) {
    pen.polygon(heart(x, 12.2, 8.6), DEEP);
    pen.polygon(heart(x, 11.9, 8), RED);
  }
  mouth(pen, { top: 18.6, half: 7.4, depth: 7.2, sag: 1.1, teeth: 2.4 });
}
