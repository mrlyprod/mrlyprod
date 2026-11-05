import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const LINE: [number, number][] = [
  [0, 0.86],
  [0.26, 0.6],
  [0.48, 0.74],
  [0.74, 0.4],
  [1, 0.2],
];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.2);
  const at = (u: number, v: number): Point => [box.x + u * box.w, box.y + v * box.h];
  const thick = box.w * 0.1;
  pen.polyline(
    LINE.map(([u, v]) => at(u, v)),
    thick,
    paint.white,
  );
  pen.triangle(at(1, 0.2), at(1, 0.46), at(0.76, 0.2), paint.white);
}
