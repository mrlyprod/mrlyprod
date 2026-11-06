import type { Pen } from "@mrlyprod/draw";
import { eyes, face, LINE } from "../own/face.ts";
import { BROWN, SHINE, TEAR } from "../own/palette.ts";
import { drop } from "../own/shape.ts";

export default function draw(pen: Pen) {
  face(pen);
  pen.segment([7.4, 8.4], [12.2, 6.6], LINE, BROWN);
  pen.segment([24.6, 8.4], [19.8, 6.6], LINE, BROWN);
  eyes(pen, 13.2);
  pen.arc([16, 27.6], 6.4, [-Math.PI * 0.74, -Math.PI * 0.26], LINE, BROWN);
  pen.polygon(drop(22.6, 22.2, 2.6, 21.6, 16.6), TEAR);
  pen.disc(23.4, 22.8, 0.8, SHINE);
}
