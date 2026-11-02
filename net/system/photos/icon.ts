import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "pink", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  const r = (x: number, y: number, w: number, h: number, round: number, color = paint.white) =>
    pen.round_rect(...p(x, y), w * k, h * k, round * k, color);
  r(212, 262, 600, 500, 60);
  pen.polygon([p(262, 712), p(432, 482), p(552, 622), p(632, 542), p(762, 712)], paint.tone);
  pen.disc(...p(652, 392), 56 * k, paint.tone);
}
