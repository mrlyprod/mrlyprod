import type { Pen } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

const ARMS = 3;
const BANDS = [0.18, 0.42, 0.7, 1];
const TWIST = 1.6;
const SHIFT = [0, 0, 0.42];
const DOTS = 12;
const WIDE = 0.1;
const HUB = 0.15;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.16);
  const cx = box.x + box.w / 2;
  const cy = box.y + box.h / 2;
  const big = box.w / 2;
  for (let k = 0; k + 1 < BANDS.length; k++) {
    const r0 = BANDS[k] * big;
    const r1 = BANDS[k + 1] * big;
    for (let j = 0; j < ARMS; j++) {
      const points: [number, number][] = [];
      for (let i = 0; i <= DOTS; i++) {
        const r = r0 + ((r1 - r0) * i) / DOTS;
        const a = (2 * Math.PI * j) / ARMS + TWIST * Math.log(r / (BANDS[0] * big)) + SHIFT[k];
        points.push([cx + Math.cos(a) * r, cy + Math.sin(a) * r]);
      }
      pen.polyline(points, WIDE * big * (0.4 + BANDS[k + 1] * 0.6), paint.white);
    }
  }
  pen.disc(cx, cy, HUB * big, paint.white);
}
