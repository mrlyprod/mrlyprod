import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "brown", 0.2);
  const w = box.w;
  const h = box.h;
  pen.round_rect(box.x, box.y + h * 0.04, w, h * 0.26, w * 0.07, paint.white);
  pen.round_rect(box.x + w * 0.07, box.y + h * 0.37, w * 0.86, h * 0.59, w * 0.07, paint.white);
  pen.round_rect(box.x + w * 0.34, box.y + h * 0.48, w * 0.32, h * 0.09, h * 0.045, paint.tone);
}
