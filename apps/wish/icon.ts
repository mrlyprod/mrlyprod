import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const EDGE = 204.8;

const SIDE = 614.4;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.2);
  const p = (x: number, y: number): Point => [box.x + ((x - EDGE) / SIDE) * box.w, box.y + ((y - EDGE) / SIDE) * box.h];
  const star: [number, number][] = [
    [512, 210],
    [594, 427],
    [826, 438],
    [645, 583],
    [706, 807],
    [512, 680],
    [318, 807],
    [379, 583],
    [198, 438],
    [430, 427],
  ];
  pen.polygon(
    star.map(([x, y]) => p(x, y)),
    paint.white,
  );
}
