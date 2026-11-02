import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  pen.round_rect(box.x, box.y + box.h * 0.08, box.w, box.h * 0.6, box.w * 0.07, paint.white);
  pen.round_rect(box.x + box.w * 0.14, box.y + box.h * 0.22, box.w * 0.45, box.h * 0.08, box.h * 0.04, paint.tone);
  pen.round_rect(box.x + box.w * 0.14, box.y + box.h * 0.4, box.w * 0.72, box.h * 0.06, box.h * 0.03, paint.tone);
  pen.rect(box.x + box.w * 0.42, box.y + box.h * 0.68, box.w * 0.16, box.h * 0.2, paint.white);
}
