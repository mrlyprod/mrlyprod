import type { Frame, Pen, Point } from "./frame.ts";
import type { Color } from "./ink.ts";

/* RANGES */

function span(values: ArrayLike<number>, n: number): [number, number] {
  let lo = Number.MAX_VALUE;
  let hi = -Number.MAX_VALUE;
  for (let i = 0; i < n; i++) {
    if (values[i] < lo) lo = values[i];
    if (values[i] > hi) hi = values[i];
  }
  return hi - lo < 1e-12 ? [lo - 0.5, hi + 0.5] : [lo, hi];
}

/* MARKS */

export function bars(pen: Pen, frame: Frame, values: ArrayLike<number>, gap: number, color: Color): void {
  if (values.length === 0) return;
  let peak = 0;
  for (let i = 0; i < values.length; i++) if (Math.abs(values[i]) > peak) peak = Math.abs(values[i]);
  if (peak <= 0) return;
  const slot = frame.w / values.length;
  const pad = (gap * slot) / 2;
  for (let i = 0; i < values.length; i++) {
    const h = frame.h * (Math.abs(values[i]) / peak);
    pen.rect(frame.x + i * slot + pad, frame.y + frame.h - h, slot - 2 * pad, h, color);
  }
}

export function dots(pen: Pen, pts: Point[], r: number, color: Color): void {
  for (const [x, y] of pts) pen.disc(x, y, r, color);
}

export function rings(pen: Pen, center: Point, radii: number[], thick: number, color: Color): void {
  for (const r of radii) pen.ring(center[0], center[1], r, thick, color);
}

export function staircase(pen: Pen, frame: Frame, values: ArrayLike<number>, thick: number, color: Color): void {
  if (values.length === 0) return;
  const [lo, hi] = span(values, values.length);
  const slot = frame.w / values.length;
  const pts: Point[] = [];
  for (let i = 0; i < values.length; i++) {
    const y = frame.y + frame.h * (1 - (values[i] - lo) / (hi - lo));
    pts.push([frame.x + i * slot, y], [frame.x + (i + 1) * slot, y]);
  }
  pen.polyline(pts, thick, color);
}

export function curve(
  pen: Pen,
  frame: Frame,
  xs: ArrayLike<number>,
  ys: ArrayLike<number>,
  thick: number,
  color: Color,
): void {
  const n = Math.min(xs.length, ys.length);
  if (n < 2) return;
  const [xlo, xhi] = span(xs, n);
  const [ylo, yhi] = span(ys, n);
  const pts: Point[] = [];
  for (let i = 0; i < n; i++) {
    pts.push([
      frame.x + (frame.w * (xs[i] - xlo)) / (xhi - xlo),
      frame.y + frame.h * (1 - (ys[i] - ylo) / (yhi - ylo)),
    ]);
  }
  pen.polyline(pts, thick, color);
}

export function axis(pen: Pen, frame: Frame, color: Color): void {
  const thick = Math.max(Math.min(frame.w, frame.h) / 512, 1);
  const { x, y, w, h } = frame;
  pen.polyline(
    [
      [x, y],
      [x + w, y],
      [x + w, y + h],
      [x, y + h],
      [x, y],
    ],
    thick,
    color,
  );
}

export function dashed(pen: Pen, frame: Frame, y: number, thick: number, color: Color): void {
  const step = frame.w / 48;
  const count = frame.w > 0 ? 48 : 0;
  for (let i = 0; i < count; i++) {
    const x = frame.x + i * step;
    const to = x + step * 0.55;
    pen.segment([x, y], [to, y], thick, color);
  }
}

export function baseline(pen: Pen, frame: Frame, color: Color): void {
  const thick = Math.max(Math.min(frame.w, frame.h) / 512, 1);
  pen.segment([frame.x, frame.y + frame.h], [frame.x + frame.w, frame.y + frame.h], thick, color);
}
