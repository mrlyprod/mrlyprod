import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const DOTS = 3;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.2);
  const w = box.w;
  const h = box.h * 0.72;
  const top = box.y + box.h * 0.06;
  pen.round_rect(box.x, top, w, h, h * 0.42, paint.white);
  pen.triangle(
    [box.x + w * 0.16, top + h * 0.7],
    [box.x + w * 0.4, top + h * 0.9],
    [box.x + w * 0.08, top + h * 1.2],
    paint.white,
  );
  const r = w * 0.07;
  for (let i = 0; i < DOTS; i++) pen.disc(box.x + w * (0.3 + i * 0.2), top + h / 2, r, paint.tone);
}
