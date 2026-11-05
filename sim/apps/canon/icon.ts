import type { Pen } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const ROWS: [number, number][] = [
  [0.3, 0.28],
  [0.5, 0.34],
  [0.7, 0.24],
];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.2);
  const cx = box.x + box.w / 2;
  const cy = box.y + box.h / 2;
  pen.disc(cx, cy, box.w * 0.5, paint.white);
  for (const [v, long] of ROWS) {
    const y = box.y + box.h * v;
    pen.disc(box.x + box.w * 0.3, y, box.w * 0.045, paint.tone);
    pen.rect(box.x + box.w * 0.4, y - box.h * 0.03, box.w * long, box.h * 0.06, paint.tone);
  }
}
