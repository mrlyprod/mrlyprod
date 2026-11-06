import type { Point } from "@mrlyprod/draw";
import { round } from "./path.ts";

const R = 10;
const IN = 0.5;
const CY = 12.65;

export const STAR: Point[] = round(
  Array.from({ length: 10 }, (_, k): Point => {
    const t = -Math.PI / 2 + (k * Math.PI) / 5;
    const r = k % 2 ? R * IN : R;
    return [12 + r * Math.cos(t), CY + r * Math.sin(t)];
  }),
  Array.from({ length: 10 }, (_, k) => (k % 2 ? 0.8 : 1.4)),
  true,
);
