import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "pink", 0.2);
  const w = box.w;
  const h = box.h;
  const thick = w * 0.08;
  const left = box.x + w * 0.22;
  const right = box.x + w * 0.74;
  const top = box.y + h * 0.1;
  pen.polyline(
    [
      [left, box.y + h * 0.78],
      [left, top + h * 0.1],
      [right, top],
      [right, box.y + h * 0.68],
    ],
    thick,
    paint.white,
  );
  pen.disc(left - w * 0.1, box.y + h * 0.78, w * 0.13, paint.white);
  pen.disc(right - w * 0.1, box.y + h * 0.68, w * 0.13, paint.white);
}
