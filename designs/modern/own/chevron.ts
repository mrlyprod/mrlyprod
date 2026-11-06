import type { Point } from "@mrlyprod/draw";
import { turn } from "./path.ts";

const ARM: Point[] = [
  [15.4, 4.75],
  [8.6, 12],
  [15.4, 19.25],
];

export const chevron = (quarter: number): Point[] => turn(ARM, (quarter * Math.PI) / 2);
