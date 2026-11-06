import type { Color, Pen, Point } from "@mrlyprod/draw";

export const BOX = 24;

export const W = 1.8;

const DOT = 1.25;

export function line(pen: Pen, ink: Color, pts: Point[]) {
  pen.polyline(pts, W, ink);
}

export function loop(pen: Pen, ink: Color, pts: Point[]) {
  pen.polyline([...pts, pts[0]!], W, ink);
}

export function dot(pen: Pen, ink: Color, x: number, y: number, r = DOT) {
  pen.disc(x, y, r, ink);
}
