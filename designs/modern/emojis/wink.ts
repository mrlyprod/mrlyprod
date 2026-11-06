import type { Pen } from "@mrlyprod/draw";
import { eye, face, shut, smile } from "../own/face.ts";

export default function draw(pen: Pen) {
  face(pen);
  eye(pen, 10.8);
  shut(pen, 21.2, 13.4);
  smile(pen, 12, 9.6, 0.22);
}
