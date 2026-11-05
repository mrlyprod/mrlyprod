import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "purple", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  pen.polygon([p(330, 260), p(694, 260), p(812, 470), p(812, 788), p(212, 788), p(212, 470)], paint.white);
  pen.polyline([p(262, 488), p(404, 488), p(460, 570), p(564, 570), p(620, 488), p(762, 488)], 44 * k, paint.tone);
}
