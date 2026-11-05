import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "red", 0.2);
  const w = box.w;
  const h = box.h;
  const pad = w * 0.1;
  pen.round_rect(box.x, box.y + h * 0.04, w, h * 0.92, w * 0.08, paint.white);
  const left = box.x + pad;
  const inner = w - 2 * pad;
  pen.round_rect(left, box.y + h * 0.16, inner, h * 0.12, h * 0.04, paint.tone);
  pen.round_rect(left, box.y + h * 0.36, inner * 0.42, h * 0.34, h * 0.04, paint.tone);
  for (let i = 0; i < 3; i++)
    pen.round_rect(
      left + inner * 0.5,
      box.y + h * (0.36 + i * 0.13),
      inner * (i === 2 ? 0.34 : 0.5),
      h * 0.06,
      h * 0.03,
      paint.tone,
    );
  pen.round_rect(left, box.y + h * 0.78, inner, h * 0.06, h * 0.03, paint.tone);
}
