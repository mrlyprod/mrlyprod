import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.2);
  for (const [i, span] of [0.68, 0.68, 0.5].entries()) {
    const y = box.y + box.h * (0.15 + i * 0.33);
    pen.disc(box.x + box.w * 0.1, y, box.w * 0.075, paint.white);
    pen.round_rect(box.x + box.w * 0.27, y - box.h * 0.05, box.w * span, box.h * 0.1, box.h * 0.05, paint.white);
  }
}
