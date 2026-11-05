import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "brown", 0.2);
  const fold = box.w * 0.22;
  pen.polygon(
    [
      [box.x + box.w * 0.15, box.y],
      [box.x + box.w * 0.85 - fold, box.y],
      [box.x + box.w * 0.85, box.y + fold],
      [box.x + box.w * 0.85, box.y + box.h],
      [box.x + box.w * 0.15, box.y + box.h],
    ],
    paint.white,
  );
  pen.triangle(
    [box.x + box.w * 0.85 - fold, box.y],
    [box.x + box.w * 0.85 - fold, box.y + fold],
    [box.x + box.w * 0.85, box.y + fold],
    paint.tone,
  );
  for (const [i, span] of [0.5, 0.5, 0.36].entries())
    pen.round_rect(
      box.x + box.w * 0.25,
      box.y + box.h * (0.36 + i * 0.16),
      box.w * span,
      box.h * 0.06,
      box.h * 0.03,
      paint.tone,
    );
}
