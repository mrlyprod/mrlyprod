import type { Point } from "@mrlyprod/draw";
import { arc } from "./path.ts";

function fit(pts: Point[], cx: number, cy: number, w: number): Point[] {
  const xs = pts.map((p) => p[0]);
  const ys = pts.map((p) => p[1]);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const k = w / (x1 - x0);
  return pts.map(([x, y]): Point => [cx + (x - (x0 + x1) / 2) * k, cy + (y - (y0 + y1) / 2) * k]);
}

export function heart(cx: number, cy: number, w: number): Point[] {
  const pts = Array.from({ length: 160 }, (_, i): Point => {
    const t = (2 * Math.PI * i) / 160;
    return [16 * Math.sin(t) ** 3, -(13 * Math.cos(t) - 5 * Math.cos(2 * t) - 2 * Math.cos(3 * t) - Math.cos(4 * t))];
  });
  return fit(pts, cx, cy, w);
}

export function drop(cx: number, cy: number, r: number, tx: number, ty: number): Point[] {
  const d = Math.hypot(tx - cx, ty - cy);
  const to = Math.atan2(ty - cy, tx - cx);
  const a = Math.acos(r / d);
  return [[tx, ty], ...arc(cx, cy, r, to + a, to - a + 2 * Math.PI)];
}

export function curve(start: Point, ...legs: [Point, Point][]): Point[] {
  const out: Point[] = [start];
  let from = start;
  for (const [c, to] of legs) {
    for (let i = 1; i <= 16; i++) {
      const t = i / 16;
      const u = 1 - t;
      out.push([
        u * u * from[0] + 2 * u * t * c[0] + t * t * to[0],
        u * u * from[1] + 2 * u * t * c[1] + t * t * to[1],
      ]);
    }
    from = to;
  }
  return out;
}

export function sparkle(cx: number, cy: number, s: number, p = 2.6): Point[] {
  return Array.from({ length: 96 }, (_, i): Point => {
    const t = (2 * Math.PI * i) / 96;
    const c = Math.cos(t);
    const n = Math.sin(t);
    return [cx + s * Math.sign(c) * Math.abs(c) ** p, cy + s * Math.sign(n) * Math.abs(n) ** p];
  });
}

export const toward = (pts: Point[], k: number, cx = 16, cy = 30.4): Point[] =>
  pts.map(([x, y]): Point => [cx + (x - cx) * k, cy + (y - cy) * k]);

export function band(top: (x: number) => number, bottom: (x: number) => number, x0: number, x1: number): Point[] {
  const xs = Array.from({ length: 61 }, (_, i) => x0 + ((x1 - x0) * i) / 60).filter((x) => top(x) < bottom(x));
  return [...xs.map((x): Point => [x, top(x)]), ...xs.toReversed().map((x): Point => [x, bottom(x)])];
}
