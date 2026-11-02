import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "../../net/sdk/plate.ts";

export const units = {};

const KINDS = [0, 1, 2, 3, 0, 1];
const RING = 0.4;
const BOT = 0.09;
const HEAP = 0.15;
const SPOKE = 0.18;
const LINE = 0.018;
const GOLD = 2.399963;
const GRAINS = 9;
const GRAIN = 0.022;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.16);
  const [cx, cy] = box.center();
  const s = box.w;
  const on = (i: number, radius: number): Point => [
    cx + Math.cos((i / KINDS.length) * 2 * Math.PI - Math.PI / 2) * radius,
    cy + Math.sin((i / KINDS.length) * 2 * Math.PI - Math.PI / 2) * radius,
  ];
  pen.disc(cx, cy, s * HEAP, paint.light);
  for (let j = 0; j < GRAINS; j++) {
    const d = s * GRAIN * 1.6 * Math.sqrt(j + 0.5);
    pen.disc(cx + Math.cos(j * GOLD) * d, cy + Math.sin(j * GOLD) * d, s * GRAIN, paint.white);
  }
  KINDS.forEach((kind, i) => {
    pen.segment(on(i, s * (RING - BOT * 1.6)), on(i, s * SPOKE), s * LINE, paint.light);
    const [x, y] = on(i, s * RING);
    const r = s * BOT;
    if (kind === 0) pen.disc(x, y, r, paint.white);
    else if (kind === 1) pen.rect(x - r * 0.86, y - r * 0.86, r * 1.72, r * 1.72, paint.white);
    else if (kind === 2)
      pen.triangle([x, y - r * 1.15], [x + r * 1.09, y + r * 0.71], [x - r * 1.09, y + r * 0.71], paint.white);
    else
      pen.polygon(
        [
          [x, y - r * 1.2],
          [x + r * 1.2, y],
          [x, y + r * 1.2],
          [x - r * 1.2, y],
        ],
        paint.white,
      );
  });
}
