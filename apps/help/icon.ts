import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.2);
  pen.round_rect(box.x + box.w * 0.08, box.y, box.w * 0.84, box.h, box.w * 0.1, paint.white);
  pen.round_rect(box.x + box.w * 0.08, box.y, box.w * 0.1, box.h, box.w * 0.05, paint.tone);
  pen.disc(box.x + box.w * 0.56, box.y + box.h * 0.76, box.w * 0.06, paint.tone);
  pen.disc(box.x + box.w * 0.56, box.y + box.h * 0.36, box.w * 0.2, paint.tone);
  pen.disc(box.x + box.w * 0.56, box.y + box.h * 0.36, box.w * 0.12, paint.white);
  pen.rect(box.x + box.w * 0.3, box.y + box.h * 0.36, box.w * 0.26, box.h * 0.24, paint.white);
  pen.rect(box.x + box.w * 0.52, box.y + box.h * 0.5, box.w * 0.08, box.h * 0.14, paint.tone);
}
