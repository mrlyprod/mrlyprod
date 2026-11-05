import type { Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  const at = (u: number, v: number): Point => [box.x + u * box.w, box.y + v * box.h];
  const page = (left: number, right: number, color: typeof paint.white) =>
    pen.polygon([at(left, 0.1), at(right, 0.18), at(right, 0.92), at(left, 0.84)], color);
  page(0, 0.47, paint.white);
  page(1, 0.53, mix(paint.white, paint.tone, 0.2));
  pen.rect(box.x + box.w * 0.47, box.y + box.h * 0.18, box.w * 0.06, box.h * 0.74, paint.tone);
}
