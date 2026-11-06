import type { Pen } from "@mrlyprod/draw";
import { eyes, face, smile } from "../own/face.ts";

export default function draw(pen: Pen) {
  face(pen);
  eyes(pen);
  smile(pen, 12.5, 8.6, 0.2);
}
