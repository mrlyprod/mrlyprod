import type { Pen } from "@mrlyprod/draw";
import { face, mouth, shut } from "../own/face.ts";
import { SHINE, TEAR } from "../own/palette.ts";
import { drop } from "../own/shape.ts";

export default function draw(pen: Pen) {
  face(pen);
  shut(pen, 10.6, 12.8);
  shut(pen, 21.4, 12.8);
  mouth(pen, { top: 16.8, half: 9.4, depth: 9.4, sag: 1.2 });
  for (const side of [-1, 1]) {
    pen.polygon(drop(16 + side * 11.6, 20.4, 3, 16 + side * 7.6, 13.6), TEAR);
    pen.disc(16 + side * 12.4, 21.2, 0.9, SHINE);
  }
}
