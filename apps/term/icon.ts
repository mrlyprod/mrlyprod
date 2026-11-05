import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.2);
  pen.round_rect(box.x, box.y + box.h * 0.1, box.w, box.h * 0.8, box.w * 0.1, paint.white);
  const l = box.x + box.w * 0.18;
  const m = box.y + box.h * 0.5;
  pen.polygon(
    [
      [l, box.y + box.h * 0.32],
      [l + box.w * 0.09, box.y + box.h * 0.25],
      [l + box.w * 0.27, m],
      [l + box.w * 0.09, box.y + box.h * 0.75],
      [l, box.y + box.h * 0.68],
      [l + box.w * 0.14, m],
    ],
    paint.tone,
  );
  pen.round_rect(box.x + box.w * 0.52, box.y + box.h * 0.62, box.w * 0.3, box.h * 0.07, box.h * 0.035, paint.tone);
}
