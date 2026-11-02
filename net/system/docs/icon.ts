import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.2);
  pen.round_rect(box.x + box.w * 0.15, box.y, box.w * 0.7, box.h, box.w * 0.08, paint.white);
  for (const [i, span] of [0.5, 0.5, 0.3].entries())
    pen.round_rect(
      box.x + box.w * 0.25,
      box.y + box.h * (0.22 + i * 0.2),
      box.w * span,
      box.h * 0.07,
      box.h * 0.035,
      paint.tone,
    );
}
