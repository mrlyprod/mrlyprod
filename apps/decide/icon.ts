import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const HOURS = [0, 2, 5, 7, 10];
const RULER = 0;
const WISH = 7;
const RING = 0.4;
const BOT = 0.1;
const HALO = 1.45;
const SPOT = 0.045;
const SQUARE = 0.13;
const LINE = 0.018;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  const on = (hour: number, radius: number): Point => [
    cx + Math.cos((hour / 12) * 2 * Math.PI - Math.PI / 2) * radius,
    cy + Math.sin((hour / 12) * 2 * Math.PI - Math.PI / 2) * radius,
  ];
  const wish = on(WISH, s * SQUARE * 1.6);
  pen.disc(cx, cy, s * SQUARE, paint.light);
  for (const hour of HOURS) pen.segment(on(hour, s * (RING - BOT)), wish, s * LINE, paint.light);
  pen.disc(wish[0], wish[1], s * SPOT * 1.6, paint.white);
  for (const hour of HOURS) {
    const [x, y] = on(hour, s * RING);
    if (hour === HOURS[RULER]) pen.ring(x, y, s * BOT * HALO, s * LINE, paint.white);
    pen.disc(x, y, s * BOT, paint.white);
  }
}
