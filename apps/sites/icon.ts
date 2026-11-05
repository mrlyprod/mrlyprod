import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.2);
  pen.round_rect(box.x, box.y + box.h * 0.05, box.w, box.h * 0.9, box.w * 0.07, paint.white);
  pen.round_rect(box.x, box.y + box.h * 0.05, box.w, box.h * 0.18, box.w * 0.07, paint.tone);
  pen.rect(box.x, box.y + box.h * 0.14, box.w, box.h * 0.09, paint.tone);
  pen.disc(box.x + box.w * 0.12, box.y + box.h * 0.14, box.w * 0.035, paint.white);
  pen.disc(box.x + box.w * 0.23, box.y + box.h * 0.14, box.w * 0.035, paint.white);
  pen.round_rect(box.x + box.w * 0.14, box.y + box.h * 0.4, box.w * 0.72, box.h * 0.07, box.h * 0.035, paint.tone);
  pen.round_rect(box.x + box.w * 0.14, box.y + box.h * 0.6, box.w * 0.5, box.h * 0.07, box.h * 0.035, paint.tone);
}
