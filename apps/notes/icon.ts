import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  const r = (x: number, y: number, w: number, h: number, round: number, color = paint.white) =>
    pen.round_rect(...p(x, y), w * k, h * k, round * k, color);
  r(252, 212, 520, 600, 60);
  for (let i = 0; i < 4; i++) pen.segment(p(332, 360 + 120 * i), p(692, 360 + 120 * i), 36 * k, paint.tone);
}
