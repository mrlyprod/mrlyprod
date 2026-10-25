import type { Value } from "../app/spec.ts";
import { startOf } from "./engine.ts";

/* TYPES */

export type Bounds = { x0: number; x1: number; y0: number; y1: number };

export const PANE = () => false;

export const SOUP = (value: Value) => startOf(value) === "noise";

const LEAST = 5;

const FILL = 0.84;

export function runs(
  ctx: CanvasRenderingContext2D,
  types: ArrayLike<number>,
  cols: number,
  rows: number,
  x: number,
  y: number,
  cell: number,
  high = cell,
): void {
  const snap = cell >= 1 ? Math.round : (n: number) => n;
  ctx.beginPath();
  for (let r = 0; r < rows; r++) {
    const top = snap(y + r * high);
    const tall = snap(y + (r + 1) * high) - top;
    for (let c = 0; c < cols; c++) {
      if (!types[r * cols + c]) continue;
      const from = c;
      while (c + 1 < cols && types[r * cols + c + 1]) c++;
      const left = snap(x + from * cell);
      ctx.rect(left, top, snap(x + (c + 1) * cell) - left, tall);
    }
  }
  ctx.fill();
}

export function reach(types: ArrayLike<number>, cols: number, rows: number): Bounds | null {
  let x0 = cols;
  let x1 = -1;
  let y0 = rows;
  let y1 = -1;
  for (let i = 0; i < types.length; i++) {
    if (!types[i]) continue;
    const x = i % cols;
    const y = (i - x) / cols;
    if (x < x0) x0 = x;
    if (x > x1) x1 = x;
    if (y < y0) y0 = y;
    if (y > y1) y1 = y;
  }
  return x1 < 0 ? null : { x0, x1, y0, y1 };
}

export function sprite(
  ctx: CanvasRenderingContext2D,
  types: ArrayLike<number>,
  cols: number,
  rows: number,
  side: number,
  ink: string,
): void {
  const box = reach(types, cols, rows);
  ctx.clearRect(0, 0, side, side);
  if (!box) return;
  const w = box.x1 - box.x0 + 1;
  const h = box.y1 - box.y0 + 1;
  const cell = Math.min((side * FILL) / Math.max(w, LEAST), (side * FILL) / Math.max(h, LEAST));
  ctx.fillStyle = ink;
  runs(ctx, types, cols, rows, (side - w * cell) / 2 - box.x0 * cell, (side - h * cell) / 2 - box.y0 * cell, cell);
}
