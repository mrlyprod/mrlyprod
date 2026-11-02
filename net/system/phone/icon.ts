import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../sdk/plate.ts";

export const units = {};

const SIDE = 1024;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "green", 0.2);
  const k = box.w / (SIDE * 0.6);
  const p = (x: number, y: number): Point => [box.x + (x - SIDE * 0.2) * k, box.y + (y - SIDE * 0.2) * k];
  pen.polyline([p(352, 256), p(360, 452), p(470, 620), p(600, 700), p(724, 700)], 120 * k, paint.white);
  pen.disc(...p(372, 270), 88 * k, paint.white);
  pen.disc(...p(716, 672), 88 * k, paint.white);
}
