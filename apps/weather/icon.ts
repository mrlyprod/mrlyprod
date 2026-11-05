import type { Pen } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "cyan", 0.2);
  const w = box.w;
  const h = box.h;
  pen.disc(box.x + w * 0.34, box.y + h * 0.34, w * 0.2, mix(paint.white, paint.tone, 0.35));
  const base = box.y + h * 0.84;
  pen.disc(box.x + w * 0.36, base - h * 0.18, w * 0.18, paint.white);
  pen.disc(box.x + w * 0.56, base - h * 0.26, w * 0.24, paint.white);
  pen.disc(box.x + w * 0.76, base - h * 0.16, w * 0.16, paint.white);
  pen.round_rect(box.x + w * 0.2, base - h * 0.2, w * 0.72, h * 0.2, h * 0.08, paint.white);
}
