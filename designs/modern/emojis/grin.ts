import type { Pen } from "@mrlyprod/draw";
import { eyes, face, mouth } from "../own/face.ts";

export default function draw(pen: Pen) {
  face(pen);
  eyes(pen, 11.6);
  mouth(pen, { top: 17.4, half: 8.6, depth: 8.6 });
}
