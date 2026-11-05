import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "brown", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  const r = (x: number, y: number, w: number, h: number, round: number, color = paint.white) =>
    pen.round_rect(...p(x, y), w * k, h * k, round * k, color);
  r(272, 212, 480, 600, 50);
  pen.rect(...p(332, 272), 360 * k, 120 * k, paint.tone);
  for (let j = 0; j < 3; j++)
    for (let i = 0; i < 3; i++) pen.disc(...p(382 + 130 * i, 492 + 120 * j), 40 * k, paint.tone);
}
