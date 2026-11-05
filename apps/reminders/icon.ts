import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  for (let i = 0; i < 3; i++) {
    pen.disc(...p(300, 312 + 200 * i), 58 * k, paint.white);
    pen.segment(p(412, 312 + 200 * i), p(784, 312 + 200 * i), 56 * k, paint.white);
  }
  pen.polyline([p(272, 312), p(294, 336), p(330, 290)], 26 * k, paint.tone);
}
