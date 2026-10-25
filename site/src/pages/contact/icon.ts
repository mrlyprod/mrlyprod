import type { Pen, Point } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const COLS = 14;
const ROWS = 10;
const CORNER = 0.07;
const TIP = 0.6;
const BEND = 6;

function corner(cx: number, cy: number, r: number, from: number, to: number): Point[] {
  return Array.from({ length: BEND + 1 }, (_, i) => {
    const a = from + ((to - from) * i) / BEND;
    return [cx + r * Math.cos(a), cy + r * Math.sin(a)];
  });
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.18);
  const w = box.w;
  const h = (w * ROWS) / COLS;
  if (h > box.h) throw new Error(`contact: an envelope ${COLS} by ${ROWS} is ${h} tall, the box ${box.h}`);
  const x = box.x;
  const y = box.y + (box.h - h) / 2;
  const r = w * CORNER;
  pen.round_rect(x, y, w, h, r, paint.white);
  const flap: Point[] = [
    ...corner(x + r, y + r, r, Math.PI, Math.PI * 1.5),
    ...corner(x + w - r, y + r, r, Math.PI * 1.5, Math.PI * 2),
    [x + w / 2, y + h * TIP],
  ];
  pen.polygon(flap, paint.deep);
}
