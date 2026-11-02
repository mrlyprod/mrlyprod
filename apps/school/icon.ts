import type { Color, Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const NODES: [number, number, number][] = [
  [0, 0.5, 0.92],
  [0, 0.5, 0.66],
  [1, 0.28, 0.48],
  [1, 0.72, 0.48],
  [2, 0.14, 0.22],
  [2, 0.36, 0.18],
  [3, 0.62, 0.18],
  [3, 0.86, 0.22],
];
const LIT = [0, 1, 2, 3, 4, 6];
const TOLD = [5];
const DOT = 0.05;
const LINE = 0.028;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.2);
  const at = (i: number): Point => [box.x + box.w * NODES[i][1], box.y + box.h * NODES[i][2]];
  const s = box.w;
  NODES.forEach(([up], i) => {
    if (!i) return;
    const [ax, ay] = at(up);
    const [bx, by] = at(i);
    const cut = LIT.includes(i) ? 0 : (s * DOT * 1.3) / Math.hypot(bx - ax, by - ay);
    pen.segment([ax, ay], [bx - (bx - ax) * cut, by - (by - ay) * cut], s * LINE * (i < 4 ? 1.4 : 1), paint.white);
  });
  const faint: Color = [255, 255, 255, 140];
  NODES.forEach((_, i) => {
    const [x, y] = at(i);
    const r = s * DOT * (i === 0 ? 1.4 : 1);
    if (LIT.includes(i)) pen.disc(x, y, r, paint.white);
    else if (TOLD.includes(i)) pen.disc(x, y, r, faint);
    else pen.ring(x, y, r, s * LINE * 0.6, paint.white);
  });
}
