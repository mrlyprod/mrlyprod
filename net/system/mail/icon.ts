import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "cyan", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  const r = (x: number, y: number, w: number, h: number, round: number, color = paint.white) =>
    pen.round_rect(...p(x, y), w * k, h * k, round * k, color);
  r(212, 292, 600, 440, 50);
  pen.polyline([p(262, 352), p(512, 540), p(762, 352)], 44 * k, paint.tone);
}
