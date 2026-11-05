import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const LINES = [1, 0.7, 1, 1, 0.6];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "gray", 0.2);
  const w = box.w;
  const h = box.h;
  const pad = w * 0.14;
  pen.round_rect(box.x, box.y, w, h, w * 0.08, paint.white);
  const inner = w - 2 * pad;
  const step = h * 0.14;
  const top = box.y + h * 0.2;
  pen.round_rect(box.x + pad, top - h * 0.04, inner, h * 0.08, h * 0.04, paint.tone);
  LINES.forEach((span, i) => {
    pen.round_rect(box.x + pad, top + (i + 1) * step, inner * span, h * 0.05, h * 0.025, paint.tone);
  });
}
