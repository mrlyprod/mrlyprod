import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "red", 0.2);
  for (const [i, h] of [0.5, 0.8, 1].entries())
    pen.round_rect(
      box.x + box.w * (0.05 + i * 0.35),
      box.y + box.h * (1 - h),
      box.w * 0.2,
      box.h * h,
      box.w * 0.05,
      paint.white,
    );
}
