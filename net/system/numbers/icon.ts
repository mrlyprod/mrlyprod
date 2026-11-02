import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.2);
  pen.round_rect(box.x, box.y, box.w, box.h, box.w * 0.08, paint.white);
  for (const t of [0.33, 0.66]) {
    pen.rect(box.x, box.y + box.h * t, box.w, box.h * 0.04, paint.tone);
    pen.rect(box.x + box.w * t, box.y, box.w * 0.04, box.h, paint.tone);
  }
}
