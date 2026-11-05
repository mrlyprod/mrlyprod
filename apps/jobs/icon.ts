import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  const x = box.x + box.w * 0.2;
  const w = box.w * 0.6;
  pen.rect(x + w * 0.3, box.y + box.h * 0.2, w * 0.4, box.h * 0.2, paint.white);
  pen.rect(x + w * 0.38, box.y + box.h * 0.28, w * 0.24, box.h * 0.12, paint.tone);
  pen.rect(x, box.y + box.h * 0.36, w, box.h * 0.46, paint.white);
  pen.rect(x, box.y + box.h * 0.56, w, box.h * 0.04, paint.tone);
}
