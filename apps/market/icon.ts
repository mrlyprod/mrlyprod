import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "orange", 0.2);
  const p = (x: number, y: number): Point => [box.x + ((x - EDGE) / SIDE) * box.w, box.y + ((y - EDGE) / SIDE) * box.h];
  const s = (n: number) => (n / SIDE) * box.w;
  pen.ring(...p(512, 372), s(136), s(56), paint.white);
  pen.round_rect(...p(232, 372), s(560), s(440), s(72), paint.white);
}
