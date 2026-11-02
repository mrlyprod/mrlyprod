import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "red", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  const r = (x: number, y: number, w: number, h: number, round: number, color = paint.white) =>
    pen.round_rect(...p(x, y), w * k, h * k, round * k, color);
  r(232, 252, 560, 540, 50);
  r(232, 252, 560, 140, 50, paint.tone);
  pen.rect(...p(232, 340), 560 * k, 52 * k, paint.tone);
  for (let j = 0; j < 2; j++)
    for (let i = 0; i < 3; i++) pen.disc(...p(352 + 160 * i, 504 + 150 * j), 34 * k, paint.tone);
}
