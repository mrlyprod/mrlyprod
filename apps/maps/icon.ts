import type { Pen, Point } from "@mrlyprod/draw";
import { mix } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const hex = (cx: number, cy: number, r: number): Point[] =>
  Array.from({ length: 6 }, (_, i) => {
    const turn = (Math.PI / 3) * i - Math.PI / 2;
    return [cx + r * Math.cos(turn), cy + r * Math.sin(turn)];
  });

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.2);
  const cx = box.x + box.w / 2;
  const cy = box.y + box.h / 2;
  pen.polygon(hex(cx, cy, box.w * 0.52), mix(paint.white, paint.tone, 0.65));
  pen.polygon(hex(cx, cy, box.w * 0.34), paint.white);
  const r = box.w * 0.1;
  pen.disc(cx, cy - r * 0.4, r, paint.tone);
  pen.triangle([cx - r * 0.86, cy + r * 0.1], [cx + r * 0.86, cy + r * 0.1], [cx, cy + r * 1.7], paint.tone);
  pen.disc(cx, cy - r * 0.4, r * 0.4, paint.white);
}
